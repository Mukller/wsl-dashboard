//! wsl-dashboard — менеджер WSL через браузер.
//!
//! cargo run --release  →  http://localhost:7070

use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::process::Command;
use tokio::net::TcpListener;

// ──────────────────────────────────────────────
// Типы
// ──────────────────────────────────────────────

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Distro {
    name:    String,
    version: String,   // "WSL1" | "WSL2"
    running: bool,
    cpu:     Option<f32>,
    ram_mb:  Option<u32>,
}

// ──────────────────────────────────────────────
// WSL-команды
// ──────────────────────────────────────────────

fn wsl_list() -> Vec<Distro> {
    let out = Command::new("wsl")
        .args(["--list", "--verbose"])
        .output();

    let Ok(out) = out else { return vec![] };
    let text = String::from_utf8_lossy(&out.stdout);

    text.lines()
        .skip(1) // заголовок
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            // формат: "* Ubuntu     Running   2"
            let line = line.trim_start_matches('*').trim();
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                return Distro { name: line.to_owned(), version: "?".into(),
                                running: false, cpu: None, ram_mb: None };
            }
            Distro {
                name:    parts[0].to_owned(),
                version: format!("WSL{}", parts[parts.len()-1]),
                running: parts[1].eq_ignore_ascii_case("running"),
                cpu:     None,
                ram_mb:  None,
            }
        })
        .collect()
}

fn wsl_stop(name: &str) -> bool {
    Command::new("wsl")
        .args(["--terminate", name])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn wsl_start(name: &str) -> bool {
    // запускаем в фоне, просто чтобы загрузить дистрибутив
    Command::new("wsl")
        .args(["-d", name, "--", "true"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

// ──────────────────────────────────────────────
// Хендлеры
// ──────────────────────────────────────────────

async fn api_list() -> Json<Vec<Distro>> {
    Json(wsl_list())
}

async fn api_stop(Path(name): Path<String>) -> impl IntoResponse {
    if wsl_stop(&name) {
        (StatusCode::OK, Json(serde_json::json!({"ok": true})))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"ok": false})))
    }
}

async fn api_start(Path(name): Path<String>) -> impl IntoResponse {
    if wsl_start(&name) {
        (StatusCode::OK, Json(serde_json::json!({"ok": true})))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"ok": false})))
    }
}

async fn index() -> Html<&'static str> {
    Html(HTML)
}

// ──────────────────────────────────────────────
// main
// ──────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/",                  get(index))
        .route("/api/distros",       get(api_list))
        .route("/api/stop/:name",    post(api_stop))
        .route("/api/start/:name",   post(api_start));

    let addr = "127.0.0.1:7070";
    eprintln!("wsl-dashboard → http://{addr}");

    // открываем браузер автоматически (Windows)
    #[cfg(target_os = "windows")]
    let _ = Command::new("cmd").args(["/c", "start", &format!("http://{addr}")]).spawn();

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ──────────────────────────────────────────────
// Встроенный HTML/JS интерфейс
// ──────────────────────────────────────────────

const HTML: &str = r#"<!DOCTYPE html>
<html lang="ru">
<head>
<meta charset="utf-8">
<title>WSL Dashboard</title>
<style>
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body { background: #0d1117; color: #c9d1d9; font: 14px/1.5 'Segoe UI', monospace; padding: 24px; }
  h1 { color: #58a6ff; margin-bottom: 20px; font-size: 20px; }
  table { width: 100%; border-collapse: collapse; }
  th { text-align: left; color: #8b949e; padding: 8px 12px; border-bottom: 1px solid #21262d; }
  td { padding: 10px 12px; border-bottom: 1px solid #161b22; }
  tr:hover td { background: #161b22; }
  .running { color: #3fb950; }
  .stopped { color: #6e7681; }
  button { border: none; border-radius: 4px; padding: 4px 12px; cursor: pointer; font-size: 13px; }
  .btn-stop  { background: #da3633; color: #fff; }
  .btn-start { background: #238636; color: #fff; }
  button:disabled { opacity: 0.4; cursor: default; }
  .refresh { color: #8b949e; font-size: 12px; margin-top: 16px; }
</style>
</head>
<body>
<h1>⬡ WSL Dashboard</h1>
<table>
  <thead>
    <tr><th>Дистрибутив</th><th>Версия</th><th>Статус</th><th>CPU</th><th>RAM</th><th></th></tr>
  </thead>
  <tbody id="body"></tbody>
</table>
<p class="refresh">Обновление каждые 3 сек</p>

<script>
async function load() {
  const r = await fetch('/api/distros');
  const distros = await r.json();
  const tbody = document.getElementById('body');
  tbody.innerHTML = distros.map(d => `
    <tr>
      <td>${d.name}</td>
      <td>${d.version}</td>
      <td class="${d.running ? 'running' : 'stopped'}">
        ${d.running ? '● Запущен' : '○ Остановлен'}
      </td>
      <td>${d.cpu != null ? d.cpu.toFixed(1) + '%' : '—'}</td>
      <td>${d.ram_mb != null ? d.ram_mb + ' MB' : '—'}</td>
      <td>
        ${d.running
          ? `<button class="btn-stop" onclick="action('stop','${d.name}')">Стоп</button>`
          : `<button class="btn-start" onclick="action('start','${d.name}')">Запуск</button>`}
      </td>
    </tr>
  `).join('');
}

async function action(cmd, name) {
  await fetch('/api/' + cmd + '/' + name, { method: 'POST' });
  await load();
}

load();
setInterval(load, 3000);
</script>
</body>
</html>
"#;

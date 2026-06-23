# Contributing

## Как помочь

Принимаю PR с:
- Новыми метриками (дисковое пространство, сетевой трафик)
- Улучшением UI (темы, компоновка)
- Поддержкой WSL1
- Исправлением работы на нестандартных конфигурациях

## Как делать

```bash
git clone https://github.com/Mukller/wsl-dashboard
cd wsl-dashboard
cargo build
cargo run
```

Требуется Windows с WSL2.

## Стиль

- `cargo fmt` и `cargo clippy`
- HTML/CSS — встроен в `main.rs`, минимальный и без зависимостей
- Без JavaScript-фреймворков

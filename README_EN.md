<div align="center">

[Русский](README.md) • **English**

</div>

# wsl-dashboard

Manage WSL distributions from the browser. Start, stop, view stats — all in one place.

~10 MB RAM footprint, runs as a local server.

## Run

```bash
cargo run --release
# → http://localhost:7070
```

Or:
```bash
cargo install --path .
wsl-dashboard
```

## Features

- List all WSL distributions with version and status
- Start / stop with one click
- Live CPU and RAM per distribution
- Open a shell directly from the UI
- Export a distribution to `.tar` with one button
- Status auto-refresh every 2 seconds

## Requirements

- Windows 10/11 with WSL2
- Rust 1.75+

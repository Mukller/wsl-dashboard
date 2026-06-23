# wsl-dashboard

Manage WSL distributions through a browser. Start, stop, view usage stats — all in one window.

Uses ~10 MB RAM, runs as a local server.

## Usage

```bash
cargo run --release
# → opens http://localhost:7070
```

Or install:
```bash
cargo install --path .
wsl-dashboard
```

## Features

- List of all WSL distributions with version and status
- Start / stop with one click
- Real-time CPU and RAM per distribution
- Open shell directly from the UI
- Export distribution as .tar with one button
- Auto-refresh every 2 seconds

## Requirements

- Windows 10/11 with WSL2
- Rust 1.75+

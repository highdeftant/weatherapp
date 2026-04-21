# Weather App

A high-density weather dashboard built with Rust and Ratatui.

## Project

- **Status**: Active Development
- **Started**: Learning Rust project
- **Purpose**: High-density weather dashboard with local WMATA train times
- **Features**: OPM status, hourly data, future crypto/portfolio tracking

## Tech Stack

- **Language**: Rust 1.94.1+
- **UI**: Ratatui (TUI dashboard)
- **Async Runtime**: Tokio
- **HTTP Client**: reqwest
- **Time Handling**: chrono

## Architecture

```
src/
├── main.rs          # Single async event loop
├── api/             # API integrations
│   ├── weatherapi.rs # Weather data
│   ├── wmatapi.rs   # WMATA train times
│   └── opmapi.rs    # OPM status
├── ui/              # UI components
│   └── app_widget.rs # Main dashboard widget
└── weatherconv.rs   # Weather conversions
```

## Key Patterns

- **Single async loop** - Main loop owns state, background tasks feed via channels
- **No unwrap** - Safe error handling with match/guard patterns
- **UI rendering guards** - Check for empty data before rendering
- **Loading states** - Explicit loading state in UI

## WIP

- [ ] Add hourly data chart
- [ ] Add crypto portfolio PnL tracking
- [ ] Add high/low system
- [ ] Auto-updates every x seconds
- [ ] Replace OPM status in off-season

## Setup

```bash
# Clone
git clone https://github.com/yourusername/weatherapp.git

# Install dependencies
cargo install

# Run
cargo run
```

## Notes

See `~/Nextcloud/Main/Second Brain/Rust/` for troubleshooting patterns and development notes.

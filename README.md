# Weather App

A high-density weather dashboard built with Rust and Ratatui, featuring multi-station WMATA train arrivals.

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

## Configuration

### Environment Variables

#### Required
- `WMATA_API_KEY`: Your WMATA API key for train predictions ([Get one here](https://api.wmata.com/))

#### Optional
- `WMATA_STATION_CODE`: Comma-separated list of station codes (e.g., `A01,C02,D04`). Defaults to `A01` (Metro Center).
- `WMATA_MAX_ROWS`: Maximum arrival lines per station (1-20). Defaults to `6`.

### Station Codes

WMATA station codes are 3-character identifiers (e.g., `A01` = Metro Center, `D04` = Foggy Bottom). See the [WMATA Station List](https://api.wmata.com/stations.htm) for the complete list.

Examples:
```bash
# Single station (default)
WMATA_STATION_CODE=A01 cargo run

# Multiple stations
WMATA_STATION_CODE=A01,C02,D04 cargo run

# Custom max rows
WMATA_MAX_ROWS=10 cargo run
```

## Key Patterns

- **Single async loop** - Main loop owns state, background tasks feed via channels
- **No unwrap** - Safe error handling with match/guard patterns
- **UI rendering guards** - Check for empty data before rendering
- **Loading states** - Explicit loading state in UI

## Multi-Station Support

The app now supports fetching arrivals from multiple stations simultaneously. Each station's arrivals are displayed in a grouped section with the station name as a header.

```
WMATA Arrivals
Station: Metro Center (A01)
Shady Gro… RD BRD
Silver Sp… RD ARR
Glenmont   RD   3

Station: Foggy Bottom (D04)
Farragut   RD  5
Rosslyn    BL  8
```

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
cd weatherapp

# Install dependencies
cargo install --path .

# Run with configuration
WMATA_API_KEY=your_key_here cargo run
```

## Notes

See `~/Nextcloud/Main/Second Brain/Rust/` for troubleshooting patterns and development notes.

# hkgbox-rs Wiki

Welcome to the hkgbox-rs project documentation wiki.

## About This Project

**hkgbox-rs** is a terminal-based TUI client for HKGolden (香港高登), a popular Hong Kong forum. Written in Rust, it provides a fast, keyboard-driven interface for browsing and interacting with forum content.

## Quick Links

- [Project Modernization (2026-04-26)](Project-Modernization-2026.md)
- [API Migration Analysis](API-Migration-Analysis.md)
- [Development Roadmap](Roadmap.md)

## Project Status

✅ **Build Status**: Successfully compiles with Rust 1.95+
✅ **Edition**: Migrated to Rust 2021
✅ **Dependencies**: All updated to modern versions

## Architecture Overview

```
src/
├── main.rs           # Application entry point
├── lib.rs            # Library exports
├── model.rs          # Data models
├── builders/         # UI builders (index, show)
├── resources/        # API/HTTP resources
├── screen/           # TUI screen components
├── control/          # Keyboard input handling
├── caches/           # File caching
├── utility/          # Helper functions
└── web.rs            # HTTP client manager
```

## Key Technologies

- **TUI**: termion 4.0 (terminal manipulation)
- **HTTP**: reqwest 0.12 (blocking HTTP client)
- **HTML Parsing**: kuchiki 0.8 (for scraping old API)
- **Serialization**: serde/serde_json 1.0
- **Logging**: log4rs 1.3

## Current API Integration

The project currently uses HTML scraping to fetch data from:
- `http://archive.hkgolden.com/topics.aspx?type=BW`

**Note**: HKGolden has a new JSON API available. See [API Migration Analysis](API-Migration-Analysis.md) for details.

## Usage

### Building

```bash
cargo build --release
```

### Running

```bash
./target/debug/hkg
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| ↑↓ | Move cursor / Scroll |
| ←→ | Previous/Next page |
| Enter | View topic |
| PageUp/Down | Scroll full page |
| Backspace | Back to list |
| R | Refresh screen |
| Q | Quit |

## System Requirements

- macOS 10.7+ (primary target)
- iTerm2 v3+ with Shell Integration
- Rust 1.95+

## Contributing

This project was originally created by **u59u75u65** and was dormant for several years until being modernized in 2026.

## License

See [LICENSE](../LICENSE) file in the repository root.

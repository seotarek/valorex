# Contributing to Valorex (فالوركس)

Thank you for your interest in contributing to Valorex, the native desktop browser engineered in Rust.

## Development Setup

### System Prerequisites (Linux / GTK3 / WebKitGTK)

Valorex links natively against GTK3 and WebKitGTK 4.1 development libraries:

**Fedora / RHEL:**
```bash
sudo dnf install -y gcc pkg-config gtk3-devel webkit2gtk4.1-devel openssl-devel sqlite-devel
```

**Ubuntu / Debian:**
```bash
sudo apt-get install -y build-essential pkg-config libgtk-3-dev libwebkit2gtk-4.1-dev libssl-dev libsqlite3-dev
```

**Arch Linux:**
```bash
sudo pacman -S --needed base-devel pkgconf gtk3 webkit2gtk-4.1 openssl sqlite
```

### Building and Testing

```bash
# Check compilation and clippy lints
cargo check
cargo clippy -- -D warnings

# Run all 44 automated unit tests
cargo test

# Launch debug binary
cargo run
```

## Architectural Guidelines

1. **Memory Safety**: No `unsafe` blocks without formal safety documentation.
2. **BiDi Parity**: All new UI components must use CSS logical properties (`margin-inline-start`, `inset-inline`, etc.) and test both Arabic (RTL) and English (LTR).
3. **Shield Performance**: Adblock rule matching in `src/shield/` must remain zero-allocation on common paths.

## Pull Request Workflow

1. Fork and create a branch (`git checkout -b feat/adblock-cosmetics`).
2. Verify all tests pass (`cargo test`).
3. Open a pull request with a clear description and benchmark numbers if affecting startup time or memory footprint.

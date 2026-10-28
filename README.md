# Valorex (فالوريكس)

<p align="center">
  <b>Valorex</b> — Ultra-fast, lightweight, memory-safe, privacy-respecting native desktop web browser engineered in <b>Rust</b>.
</p>

<p align="center">
  <a href="https://github.com/seotarek/valorex"><img alt="GitHub Repo" src="https://img.shields.io/badge/GitHub-seotarek%2Fvalorex-10b981"></a>
  <img alt="built on" src="https://img.shields.io/badge/built%20with-Rust%202021-f97316"></a>
  <img alt="engine" src="https://img.shields.io/badge/engine-WebKitGTK%20%2B%20Brave%20Adblock-6366f1"></a>
  <img alt="license" src="https://img.shields.io/badge/license-GPL--3.0--or--later-64748b">
</p>

---

## ⚡ Overview

**Valorex** is designed as a streamlined, lightning-fast alternative to bloated Electron apps and heavy Chromium forks. Powered by Rust's zero-cost abstractions, native WebKitGTK rendering, and the official Brave `adblock-rust` rule-matching engine, Valorex starts in milliseconds, uses 10x less RAM than Electron, and blocks aggressive trackers before they hit your CPU.

### Key Pillars
- **Zero-Bloat Performance:** Pure compiled native binary (~10MB) with minimal RAM overhead (20–40MB on idle).
- **Valorex Shield:** Native integration with Brave's `adblock-rust` engine supporting EasyList, EasyPrivacy, and custom cosmetic/network rules.
- **Memory Safety:** 100% memory-safe architecture powered by Rust's ownership model, preventing memory corruption and use-after-free vulnerabilities.
- **Hardware Acceleration:** Native Wayland & X11 GPU compositing via WebKitGTK.

---

## 📁 Architecture

```
valorex/
├── Cargo.toml            # Project manifest & optimization profiles
├── src/
│   ├── main.rs           # Application event loop & lifecycle
│   ├── lib.rs            # Library exports
│   ├── config/           # User preferences & configuration
│   │   └── mod.rs
│   ├── shield/           # Valorex Shield (adblock-rust engine)
│   │   └── mod.rs
│   └── browser/          # Window and Webview controller
│       └── mod.rs
└── tests/                # Integration test suites
```

---

## 🚀 Building & Running

### Prerequisites (Fedora / RHEL / CentOS)
```bash
sudo dnf install -y rust cargo gtk3-devel webkit2gtk4.1-devel
```

### Development Run
```bash
# Run with default homepage
cargo run

# Or pass a target URL directly
cargo run -- https://duckduckgo.com
```

### Run Unit Tests
```bash
cargo test
```

### Optimized Production Build
```bash
cargo build --release
# Binary will be placed in target/release/valorex
```

---

## 📄 License
Licensed under the [GNU General Public License v3.0 or later](LICENSE).

# Rust Zero-Copy Event Stream

[![Rust CI](https://github.com/FelipeMadson/rust-zero-copy-event-stream/actions/workflows/ci.yml/badge.svg)](https://github.com/FelipeMadson/rust-zero-copy-event-stream/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Release](https://img.shields.io/badge/release-v1.0.0-green.svg)](https://github.com/FelipeMadson/rust-zero-copy-event-stream/releases)

High-throughput, deterministic circular ring buffer event streamer in Rust. Designed for predictable memory consumption and microsecond-level latency in event-driven systems.

## Key Architecture

- **Zero Allocation in Critical Path**: Pre-allocated contiguous buffers avoiding heap fragmentation.
- **Strict Error Boundary**: Custom `thiserror` enums without panic paths.
- **Batch Drain Mechanics**: High-speed chunk flushing with native `Vec::drain` ownership transfers.

## Build & Test

```bash
cargo build --release
cargo test
```

## Author

Developed by **[Felipe Madison](https://github.com/FelipeMadson)**.

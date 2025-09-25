# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an embedded Rust project template for the RP2350 microcontroller with Wi-Fi capabilities using the Embassy async runtime. It's designed for the Raspberry Pi Pico 2W and similar RP2350-based boards with CYW43 wireless chips.

## Architecture

### Core Components
- **Embassy Runtime**: Async/await embedded framework with executor and networking stack
- **CYW43 Wi-Fi Driver**: Wireless networking implementation using PIO-based SPI communication
- **Resource Management**: Hardware peripherals assigned using the `assign-resources` crate
- **Dual-target Support**: Compiles for both ARM Cortex-M (thumbv8m.main-none-eabihf) and host testing (aarch64-apple-darwin)

### Module Structure
- `src/bin/main.rs`: Main application entry point with async executor
- `src/networking/`: Wi-Fi driver implementation and networking stack initialization
- `src/resources.rs`: Hardware peripheral assignment and interrupt bindings
- `src/logging.rs`: Logging infrastructure (defmt for embedded, env_logger for tests)
- `src/macros.rs`: Utility macros including `measure_time!` for performance profiling

### Memory Layout
- Flash: 2MB at 0x10000000 (RP2350 external/internal flash)
- RAM: 512KB at 0x20000000 (striped SRAM0-SRAM7 banks)
- Direct RAM: SRAM4/5 for dedicated use cases
- CYW43 Firmware: Baked at 0x10100000 (firmware) and 0x10140000 (CLM) when using `baked-cyw43` feature

## Development Commands

### Building and Running
```bash
# Cross-compile for RP2350 (default target)
cargo build

# Build for host testing
cargo build --target aarch64-apple-darwin

# Run tests (requires host target)
cargo test --target aarch64-apple-darwin

# Flash and run on device (requires probe-rs)
cargo run
```

### Linting and Code Quality
```bash
# Run all linters (format, sort, clippy, machete)
./scripts/lint.sh

# Check mode (CI/pre-commit)
./scripts/lint.sh --check

# Individual tools
cargo fmt
cargo sort --grouped
cargo clippy
cargo machete
```

### Wi-Fi Firmware Management
```bash
# Bake CYW43 firmware into flash (alternative to bundled assets)
./scripts/bake-cyw43.sh
```

## Environment Configuration

### Required Environment Variables
- `WIFI_SSID`: Wi-Fi network name
- `WIFI_PASSWORD`: Wi-Fi network password

### Feature Flags
- `bin`: Enable binary compilation with embassy-rp
- `wifi`: Enable Wi-Fi networking (requires `bin`)
- `baked-cyw43`: Use firmware baked in flash instead of bundled assets
- `log`: Use log crate instead of defmt for host testing

## Development Patterns

### Resource Assignment
Hardware peripherals are statically assigned using `assign_resources!` macro in `src/resources.rs`. The `split_resources!` macro in main.rs distributes these to different subsystems.

### Async Task Management
Embassy tasks are spawned for:
- CYW43 driver runner (handles Wi-Fi chip communication)
- Network stack runner (handles TCP/IP stack)
- Main application loop with watchdog feeding

### Logging Strategy
- Embedded builds use `defmt` with RTT transport
- Host test builds use `env_logger`
- Performance measurement via `measure_time!` macro

### Error Handling
Wi-Fi connection failures are retried with LED feedback (GPIO 0 toggles on failure, stays on when connected).

## Hardware Configuration

### Pin Assignments (RP2350W)
- PIN_23: CYW43 power control
- PIN_24: CYW43 SPI data I/O
- PIN_25: CYW43 SPI chip select
- PIN_29: CYW43 SPI clock
- PIO0: Used for bit-banged SPI communication with CYW43
- DMA_CH0: DMA channel for SPI transfers

### Memory Constraints
- Optimized for size with LTO and high optimization levels
- Uses `static_cell` for zero-allocation static initialization
- Watchdog configured with 10-second timeout
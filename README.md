# RP2350W Embassy Template

A Rust embedded template for the RP2350 microcontroller with Wi-Fi capabilities, built on the [Embassy](https://embassy.dev/) async runtime framework.

## Features

- 🚀 **Embassy Async Runtime**: Modern async/await embedded programming
- 📶 **Wi-Fi Connectivity**: CYW43 wireless chip support with automatic connection
- 🔧 **Resource Management**: Clean hardware peripheral assignment
- 🧪 **Dual Target Support**: Cross-compilation for embedded and host testing
- ⚡ **Performance Optimized**: LTO and size optimization for embedded targets
- 📋 **Development Tools**: Integrated linting, formatting, and testing

## Hardware Requirements

- Raspberry Pi Pico 2W or compatible RP2350-based board with CYW43 wireless chip
- USB cable for programming and debugging
- [probe-rs](https://probe.rs/) compatible debug probe (optional, for advanced debugging)

## Quick Start

### Prerequisites

**Option 1: Manual installation**
```bash
# Install Rust with the required target
rustup target add thumbv8m.main-none-eabihf

# Install required tools
cargo install probe-rs-tools cargo-sort cargo-machete

# Optional: Install direnv for automatic environment loading
```

**Option 2: Nix flake (recommended for reproducible builds)**
```bash
nix develop  # Automatically installs all dependencies
```

### Setup

1. Configure Wi-Fi credentials using the provided template. You have several options:
   - **Using direnv** (recommended): Install [direnv](https://direnv.net/) and copy `.env.template` to `.env`
   - **Environment variables**: Export `WIFI_SSID` and `WIFI_PASSWORD` in your shell
   - **Nix flake**: Use `nix develop` to enter the development environment

2. Build and flash to your device:
```bash
cargo run
```

## Development

### Building

```bash
# Build for RP2350 (default)
cargo build

# Build optimized release version
cargo build --release

# Build for host testing
cargo build --target aarch64-apple-darwin
```

### Testing

```bash
# Run tests (host target required)
cargo test --target aarch64-apple-darwin
```

### Code Quality

```bash
# Run all linters and formatters
./scripts/lint.sh

# Check mode (for CI/pre-commit hooks)
./scripts/lint.sh --check
```

### Wi-Fi Configuration

Configure your Wi-Fi credentials using one of these methods:

**Option 1: direnv (recommended)**
```bash
cp .env.template .env
# Edit .env with your credentials
# direnv will automatically load them when you enter the directory
```

**Option 2: Environment variables**
```bash
export WIFI_SSID="your-network-name"
export WIFI_PASSWORD="your-network-password"
```

**Option 3: Nix flake**
```bash
nix develop
# Then set up .env as in Option 1
```

The credentials are compiled into the binary at build time.

## Features

### Feature Flags

- `bin` (default): Enable binary compilation with embassy-rp
- `wifi` (default): Enable Wi-Fi networking capabilities  
- `defmt` (default): Use defmt logging for embedded builds
- `baked-cyw43`: Use firmware baked in flash instead of bundled assets
- `log`: Use log crate instead of defmt for host builds

### Firmware Options

**Bundled Assets (Default):**
Firmware files are included in the binary from `assets/cyw43-firmware/`.

**Baked Firmware:**
For production or when flash space is critical, use the `baked-cyw43` feature:

```bash
# Flash firmware to specific addresses
./scripts/bake-cyw43.sh

# Build with baked firmware
cargo build --features baked-cyw43 --no-default-features
```

## Hardware Configuration

### Pin Assignments (RP2350W)

| Function | Pin | Usage |
|----------|-----|-------|
| CYW43 Power | PIN_23 | Power control for wireless chip |
| CYW43 Data | PIN_24 | SPI data I/O |
| CYW43 CS | PIN_25 | SPI chip select |
| CYW43 Clock | PIN_29 | SPI clock |
| LED | GPIO_0 | Status indicator (via CYW43) |

### Memory Layout

- **Flash**: 2MB at 0x10000000 (external/internal)
- **RAM**: 512KB at 0x20000000 (striped SRAM banks)
- **Direct RAM**: SRAM4/5 for dedicated use cases

## What It Does

This template creates a basic Wi-Fi enabled application that:

1. Initializes the RP2350 with Embassy runtime
2. Sets up the CYW43 wireless driver using PIO-based SPI
3. Connects to your specified Wi-Fi network
4. Obtains an IP address via DHCP
5. Runs a main loop with watchdog feeding and status logging
6. Provides LED feedback during Wi-Fi connection process

The LED on your Pico 2W will:
- Blink during Wi-Fi connection attempts
- Stay solid once successfully connected

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

# Technical Context

## Technologies
- **Rust** (edition 2021): Main language, compiled to a single static binary
- **Tokio**: Async runtime for the control loop and signal handling
- **Docker**: Deployment via multi-stage Dockerfile
- **ipmitool**: Runtime dependency for IPMI communication with Dell iDRAC
- **GitHub Actions**: CI/CD for automated Docker image builds

## Rust Dependencies (Cargo.toml)
| Crate | Purpose |
|-------|---------|
| `tokio` (full) | Async runtime, timers, signal handling |
| `anyhow` | Error handling with context |
| `thiserror` | Derive macros for custom error types |
| `serde` (derive) | Serialization (config handling) |
| `env_logger` | Logging initialization from `RUST_LOG` env var |
| `log` | Logging macros (info!, error!, warn!) |
| `chrono` | Timestamp formatting for log output |

## Release Profile
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```
Produces a small (~2MB), optimized binary. LTO + single codegen unit for maximum optimization at the cost of compile time.

## Development Setup

### Prerequisites
- Docker Desktop with buildx support
- No local Rust toolchain needed (builds inside Docker)
- Git for version control

### Dev Machine
- macOS on Apple Silicon (arm64)
- Docker Desktop 4.52+ with buildx and QEMU for cross-platform builds

### Target Machine
- Dell PowerEdge R720xd
- Unraid OS
- AMD64 (Intel Xeon)
- iDRAC 7 (firmware < 3.30.30.30)

## Build Commands

### Local build (for testing)
```bash
./build-local.sh                      # Builds :latest locally
./build-local.sh --tag test           # Builds :test locally
```

### Build and push to Docker Hub
```bash
./build-local.sh --tag stable --push  # Build + push :stable
./build-local.sh --push               # Build + push :latest
```

### Critical build flags (must be consistent everywhere)
```
--platform linux/amd64    # Cross-compile for target architecture
--provenance=false        # Avoid multi-manifest issues on single-platform
no cache                  # Clean builds to prevent corruption
```
These flags are identical in `build-local.sh`, `docker-beta.yml`, and `docker-main.yml`.

## Docker Image Details
- **Builder stage**: `rust:1.85-slim` (Debian Bookworm based)
- **Runtime stage**: `debian:bookworm-slim`
- **Runtime packages**: `ipmitool`, `ca-certificates`, `procps` (for pgrep healthcheck)
- **Final size**: ~100MB
- **Architecture**: linux/amd64 only

## Repository Structure
```
rust-version/
├── src/                    # Rust source code (4 modules)
├── examples/               # curve_demo.rs visualization tool
├── _memory_bank/           # AI context persistence (this)
├── unraid/                 # Unraid container template XML
├── .github/workflows/      # CI/CD (docker-beta.yml, docker-main.yml)
├── AGENTS.md               # Agent instructions (cross-tool: OpenCode, Copilot, Windsurf, Cline)
├── CLAUDE.md               # Claude Code instructions (redirects to AGENTS.md)
├── Cargo.toml              # Rust dependencies and build config
├── Dockerfile              # Multi-stage Docker build
├── docker-compose.yml      # Deployment template
├── build-local.sh          # Local build script (cross-compiles to AMD64)
├── test-local.sh           # Interactive test script
└── README.md               # Main documentation
```

## Technical Constraints
- Must produce AMD64 images (Dell PowerEdge servers are x86_64)
- Development machine is Apple Silicon (arm64) -- requires cross-compilation
- ipmitool is a runtime dependency, not a build dependency
- `Cargo.lock` should be committed (binary project, not a library)
- No Rust toolchain installed locally -- all compilation happens inside Docker

## Docker Hub
- **Registry**: `docker.io/maanstr/idrac-fan-controller-rust`
- **Tags**: `:latest`, `:main`, `:stable`, `:beta`, `:test`
- **Secrets required**: `DOCKERHUB_USERNAME`, `DOCKERHUB_TOKEN` in GitHub repo settings

## Known Technical Debt
- `Cargo.lock` not yet committed (removed from `.gitignore` but never generated -- no local Rust toolchain; gets generated inside Docker during build)
- Compiler warnings: unused import `warn` in `main.rs:6`, unused variable `server_info` in `ipmi.rs:103`, dead code `extract_temperature` in `ipmi.rs:206`

## IPMI Raw Commands Used
| Command | Purpose |
|---------|---------|
| `0x30 0x30 0x01 0x00` | Enable manual fan control |
| `0x30 0x30 0x01 0x01` | Restore Dell default fan control |
| `0x30 0x30 0x02 0xff <hex>` | Set all fans to speed (hex percentage) |
| `sdr type temperature` | Read temperature sensors |
| `fru` | Get server manufacturer/model info |

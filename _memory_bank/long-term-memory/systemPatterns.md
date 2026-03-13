# System Patterns

## Architecture

### Multi-Module Rust Binary
```
src/
├── main.rs        # Controller struct, control loop, signal handling, shutdown
├── config.rs      # Config struct loaded from environment variables
├── ipmi.rs        # IpmiClient wrapping ipmitool CLI, temperature parsing
└── fan_curve.rs   # Exponential curve calculation, unit tests
```

The binary is a single async process (`tokio` runtime) that loops on a timer, reads temps, calculates fan speed, and sends IPMI commands.

### Docker Multi-Stage Build
- **Stage 1 (builder)**: `rust:1.85-slim` -- compiles and strips the binary
- **Stage 2 (runtime)**: `debian:bookworm-slim` -- only `ipmitool`, `ca-certificates`, `procps`
- Final image ~100MB, binary ~2MB

## Key Technical Decisions

### Exponential curve over linear
Linear curves respond too slowly at low temps and too aggressively at high temps. Exponential provides a slow initial ramp that accelerates -- matching the urgency of rising temperatures.

### ipmitool CLI wrapper over native IPMI
Using `ipmitool` as a subprocess rather than a native Rust IPMI library because:
- Proven, battle-tested tool for Dell IPMI
- Handles all the raw command formatting
- No unsafe Rust code needed for low-level IPMI
- Tradeoff: requires ipmitool in the container image

### Hysteresis for stability
Fan speed only changes if the new calculated speed differs from current by more than the hysteresis threshold (default 5%). Prevents rapid oscillation around transition points.

### Temperature smoothing
Rolling window average (`TEMP_SMOOTHING_WINDOW`, default 3 readings) dampens transient spikes. Rate limiting (`MIN_CHANGE_INTERVAL`, default 60s) prevents changes faster than the hardware can stabilize.

### Signal handling for safety
Catches SIGTERM, SIGINT, SIGQUIT to restore Dell default fan control before exit. This is critical -- if the container is killed without restoring, fans stay at the last manual speed which could be dangerously low.

## Design Patterns

### Configuration from Environment
All config is loaded from env vars with sensible defaults (`Config::from_env()`). No config files, no CLI args. This is the Docker-native pattern -- compose files and Unraid templates set env vars.

### Controller Pattern
`Controller` struct owns all state: config, IPMI client, server info, last fan speed. Single responsibility methods: `new()` for init, `control_loop()` for the main loop, `shutdown()` for cleanup.

### Graceful Degradation
- If one temperature sensor is missing, uses the other
- If IPMI commands fail, logs error but continues loop
- On fatal init errors (not a Dell server, no IPMI access), exits cleanly

## Build & Deploy Patterns

### Local Build (Apple Silicon -> AMD64)
`build-local.sh` uses `docker buildx build --platform linux/amd64 --provenance=false` to cross-compile from arm64 dev machine to amd64 production target. This was a critical fix -- plain `docker build` produces arm64 images that silently fail on the Dell server.

### CI/CD (GitHub Actions)
Two separate workflows matching local build flags exactly:
- `docker-beta.yml`: PR opened/updated -> builds and pushes `:beta`
- `docker-main.yml`: Merge to main -> builds and pushes `:main` and `:latest`
- `:stable` is manual-only (promoted via `build-local.sh --tag stable --push` after testing)

### Tag Strategy
| Tag | Updated | Purpose |
|-----|---------|---------|
| `:beta` | On PR | Test on hardware before merging |
| `:main` | On merge | Tracks main branch |
| `:latest` | On merge | Default pull tag |
| `:stable` | Manual only | Known-good fallback, promoted after testing |

## Component Relationships
```
Config (env vars) --> Controller --> IpmiClient (ipmitool subprocess)
                         |
                         +--> fan_curve::calculate_fan_speed()
                         |
                         +--> tokio signal handlers --> shutdown()
```

## Critical Implementation Paths
1. **Startup**: Load config -> Init IPMI client -> Detect Dell server -> Set manual fan control -> (optional) Disable third-party PCIe cooling
2. **Control loop**: Read temps -> Smooth -> Calculate curve -> Apply hysteresis -> Set fan speed -> Sleep
3. **Shutdown**: Restore Dell default fan control -> Re-enable third-party PCIe cooling -> Exit

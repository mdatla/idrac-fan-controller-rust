# Product Context

## Why This Project Exists
The original [Dell iDRAC fan controller](https://github.com/tigerblue77/Dell_iDRAC_fan_controller_Docker) uses a binary on/off threshold approach: if CPU temp exceeds a single threshold, it reverts to Dell's aggressive default fan profile (40-60%); below it, a static user-defined speed (e.g., 5%). This causes abrupt fan speed swings, oscillation around the threshold, and unnecessary noise.

This Rust rewrite replaces binary control with a smooth **exponential fan curve** that continuously adjusts fan speed based on temperature, eliminating abrupt transitions.

## Problems It Solves
1. **Fan noise from binary control**: The original's on/off threshold causes fans to swing between near-silent and jet-engine repeatedly
2. **Lack of progressive response**: No middle ground between "user static speed" and "Dell default"
3. **Code quality**: Bash scripts with limited error handling, no type safety, no tests
4. **Deployment reliability**: The original had no CI/CD for automated Docker builds

## How It Works
1. Reads CPU temperatures via `ipmitool` (local `/dev/ipmi0` or remote via LAN)
2. Applies an exponential curve: gradual increase from `MIN_FAN_SPEED` at `BASE_TEMP` to `MAX_FAN_SPEED` at `CRITICAL_TEMP`
3. Sets fan speed via IPMI raw commands with hysteresis to prevent oscillation
4. Runs in a loop on `CHECK_INTERVAL` (default 60s)
5. On shutdown (SIGTERM/SIGINT/container stop), restores Dell default fan control

## Exponential Curve Formula
```
normalized = (T - T_base) / (T_critical - T_base)
exp_factor = (1 - e^(-k * normalized * 10)) / (1 - e^(-k * 10))
fan_speed  = min_speed + (max_speed - min_speed) * exp_factor
```
Where `k` = curve steepness (default 0.15). This produces slow initial ramp that accelerates as temperatures rise.

## Target Users
- **Homelab operators** running Dell PowerEdge servers (especially R720/R720xd) who want quiet operation
- **Unraid users** who need a Docker container for fan control
- Anyone with Dell iDRAC 7/8/9 (firmware < 3.30.30.30) who wants better-than-binary fan management

## User Experience Goals
- Deploy via single `docker run` or docker-compose with environment variables
- Quiet at idle, aggressive when needed, no manual tuning required for most setups
- Safe defaults that never let hardware overheat
- Automatic restore of Dell fan control if the container stops for any reason

## Future Enhancement Ideas
- Multiple curve types (linear, quadratic, custom)
- Per-CPU fan control
- GPU temperature monitoring
- Web UI for configuration
- Prometheus metrics export
- Email/webhook alerts
- Native Rust IPMI library (eliminate ipmitool dependency)
- ARM64 builds (for Raspberry Pi or ARM servers)

## Compatibility
- Dell PowerEdge servers with IPMI support (Gen 11-14+)
- iDRAC 7/8/9 with firmware < 3.30.30.30
- Both local (`/dev/ipmi0`) and remote (LAN) iDRAC access
- Primary deployment target: Unraid on Dell R720xd (AMD64)

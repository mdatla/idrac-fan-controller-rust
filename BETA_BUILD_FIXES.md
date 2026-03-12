# Beta Build Fixes

## Issues Found and Fixed

### Issue 1: Missing Environment Variables in Dockerfile
**Problem:** The new smoothing configuration variables were not defined in the Dockerfile's ENV section.

**Impact:** While the code has defaults, Docker best practice is to document all environment variables in the Dockerfile.

**Fix:** Added to Dockerfile (commit 6961051):
```dockerfile
TEMP_SMOOTHING_WINDOW=3 \
MIN_CHANGE_INTERVAL=60 \
EMERGENCY_TEMP_DELTA=10 \
HYSTERESIS_PERCENT=5 \
```

### Issue 2: Emergency Spike Calculation Bug
**Problem:** The emergency spike message was using `self.last_temp` after it had already been updated, resulting in incorrect spike values in log messages.

**Impact:** Emergency detection still worked, but the log message would show 0°C spike.

**Fix:** Moved `self.last_temp` update to after the emergency check, and stored spike value separately (commit 06a7cd0).

### Issue 3: Insufficient Startup Logging
**Problem:** Container startup failures were hard to diagnose without more verbose logging.

**Fix:** Added logging statements during initialization (commit 6961051):
- "Initializing IPMI client..."
- "Getting server information..."

## Testing the :beta Build

After the PR updates and GitHub Actions completes the build:

```bash
# Remove any cached images
docker rmi maanstr/idrac-fan-controller-rust:beta

# Pull fresh beta build
docker pull maanstr/idrac-fan-controller-rust:beta

# Verify it's AMD64
docker image inspect maanstr/idrac-fan-controller-rust:beta \
  --format '{{.Architecture}} {{.Os}}'
# Should show: amd64 linux

# Test run with your configuration
docker run -d \
  --name idrac-fan-beta \
  -e IDRAC_HOST=your_idrac_ip \
  -e IDRAC_USERNAME=root \
  -e IDRAC_PASSWORD=yourpassword \
  -e CHECK_INTERVAL=10 \
  -e TEMP_SMOOTHING_WINDOW=3 \
  -e MIN_CHANGE_INTERVAL=60 \
  -e HYSTERESIS_PERCENT=5 \
  maanstr/idrac-fan-controller-rust:beta

# Watch logs
docker logs -f idrac-fan-beta
```

## Expected Behavior

With the default smoothing settings:
- Container should start successfully
- Logs should show initialization messages
- Temperature readings every 10 seconds
- Fan speed changes at most every 60 seconds (unless emergency)
- Output includes "Smoothed" temperature column

Example output:
```
Dell iDRAC Fan Controller (Rust Edition with Exponential Curve)
================================================================

Initializing IPMI client...
Getting server information...
Server model: DELL PowerEdge R720
...
Smoothing and rate limiting:
  Temperature smoothing window: 3 readings
  Minimum change interval: 60s
  Emergency temp delta: 10°C
  Hysteresis: ±5%

    Date & time          Inlet  CPU 1  CPU 2  Exhaust  Smoothed  Fan Speed  Comment
11-03-2026 14:23:45   28°C   45°C   47°C     52°C     45.0°C      13%  Fan speed adjusted
11-03-2026 14:23:55   28°C   46°C   48°C     53°C     45.5°C      13%  -
11-03-2026 14:24:05   28°C   47°C   49°C     54°C     46.0°C      13%  -
...
```

## Platform Verification

The build is configured for AMD64 only to avoid previous cross-compilation issues:

**GitHub Actions workflow:**
- Platform: `linux/amd64`
- Cache: Disabled
- Buildx: Used but single-platform only

**Why AMD64 only?**
- Previous multi-platform builds (amd64+arm64) produced non-functional binaries
- GitHub Actions cache was corrupting Rust builds
- Most Dell PowerEdge servers and Unraid systems are AMD64

See `_docs/DOCKER_IMAGES.md` for more details on the platform decision.

# Safe Testing Guide

## Overview

This guide helps you safely test the fan controller without risking hardware damage.

## Safety Features Built-In

The controller has multiple safety mechanisms:

1. **Automatic restoration on exit** - Dell default fan control is restored when:
   - You press Ctrl+C
   - Container stops
   - Process crashes
   - Any fatal error occurs

2. **Signal handlers** - Gracefully handles SIGTERM, SIGINT, SIGQUIT

3. **Dell default fallback** - Dell's thermal management takes over if controller stops

## Testing Levels (Recommended Order)

### Level 1: Dry Run Testing (NO hardware risk)

Test the code logic without affecting your server:

```bash
cd rust-version

# 1. Run the curve visualization
cargo run --example curve_demo

# 2. Try with different parameters
MIN_FAN_SPEED=10 MAX_FAN_SPEED=80 BASE_TEMP=35 CRITICAL_TEMP=65 \
  cargo run --example curve_demo

# 3. Run unit tests
cargo test

# 4. Check if tests pass
cargo test -- --nocapture
```

**Expected output**: Tables showing temperature → fan speed mappings, all tests passing.

### Level 2: IPMI Read-Only Testing (NO fan changes)

Verify IPMI communication works without changing fan speeds:

```bash
# Test IPMI connection manually first
ipmitool -I lanplus -H <IDRAC_IP> -U <USER> -P <PASS> sdr type temperature

# Or for local:
ipmitool -I open sdr type temperature

# Should show temperature readings like:
# Inlet Temp       | 04h | ok  |  7.1 | 22 degrees C
# Temp             | 0Eh | ok  |  3.1 | 42 degrees C
```

**If this fails**, don't proceed - fix IPMI connectivity first.

### Level 3: Short Duration Test (5 minutes)

First actual test with fan control:

```bash
cd rust-version

# Set conservative parameters
export IDRAC_HOST=<your-idrac-ip>
export IDRAC_USERNAME=root
export IDRAC_PASSWORD=<your-password>
export MIN_FAN_SPEED=20        # Higher minimum for safety
export MAX_FAN_SPEED=100
export BASE_TEMP=35            # Lower base for earlier response
export CRITICAL_TEMP=65        # Lower critical for safety
export CHECK_INTERVAL=30       # More frequent checks
export RUST_LOG=debug          # Detailed logging

# Build
cargo build --release

# Run in foreground (keep terminal open!)
./target/release/idrac_fan_controller
```

**During this test**:
1. Keep the terminal visible
2. Watch temperature readings
3. Listen to fan behavior
4. Have iDRAC web interface open in browser
5. Monitor for 5 minutes
6. Press **Ctrl+C** to stop

**Verify**:
- Fans return to Dell default (should hear them ramp up briefly)
- Temperatures stayed in safe range
- No errors in logs

### Level 4: Load Test (15 minutes)

Test under actual server load:

```bash
# In one terminal, start the controller (same as Level 3)
./target/release/idrac_fan_controller

# In another terminal, create some CPU load
stress --cpu 4 --timeout 300s  # 5 minutes of stress

# Or use a real workload if available
```

**Monitor**:
1. Watch temperatures rise
2. Watch fan speeds increase exponentially
3. Verify curve behavior matches expectations
4. Let it run for 10 more minutes after stress ends
5. Verify fan speeds decrease smoothly

**Expected behavior**:
```
Initial:  45°C → 8% fan
Load:     55°C → 28% fan
Load:     60°C → 48% fan
Cooldown: 55°C → 28% fan
Cooldown: 45°C → 8% fan
```

### Level 5: Extended Test (2-4 hours)

Run during normal workload:

```bash
# Use docker-compose for easier management
cd rust-version

# Edit docker-compose.yml with your settings
# Start in detached mode
docker-compose up -d

# Watch logs
docker-compose logs -f

# After 2-4 hours of normal operation:
docker-compose down
```

**Check for**:
- Stable operation
- No unexpected errors
- Reasonable fan speeds
- Acceptable noise levels
- Temperatures within limits

### Level 6: Production Deployment

After successful testing:

```bash
# Set up as systemd service or permanent docker container
docker-compose up -d --restart unless-stopped
```

## Safety Checklist

Before each test level:

- [ ] iDRAC web interface accessible
- [ ] Can manually control fans via iDRAC if needed
- [ ] Know your CPU's maximum safe temperature (Tjunction/Tcase)
- [ ] Server is not running critical workloads
- [ ] You can physically access the server if needed
- [ ] You have time to monitor the test
- [ ] Backup controller (original script) is available

## Emergency Procedures

### If temperatures get too high:

1. **Immediate**: Press Ctrl+C or `docker-compose down`
   - Dell default control restores automatically

2. **If controller won't stop**:
   ```bash
   # Find and kill process
   pkill -9 idrac_fan_controller
   
   # Or stop container
   docker stop idrac_fan_controller_rust
   ```

3. **Manual IPMI restoration** (if all else fails):
   ```bash
   ipmitool -I lanplus -H <IP> -U <USER> -P <PASS> raw 0x30 0x30 0x01 0x01
   ```

### If fans are too loud:

Adjust parameters (don't stop the controller):

1. Note current temperatures
2. Stop controller (Ctrl+C)
3. Increase `BASE_TEMP` by 5°C
4. Decrease `MAX_FAN_SPEED` to 80%
5. Decrease `CURVE_STEEPNESS` to 0.1
6. Restart and monitor

### If fans are too quiet:

1. Check current temperatures
2. If temps > 60°C, stop and adjust:
   - Decrease `BASE_TEMP` by 5°C
   - Increase `MIN_FAN_SPEED` to 15%
   - Increase `CURVE_STEEPNESS` to 0.25

## Comparison Testing (Original vs Rust)

To safely compare behavior:

```bash
# Day 1-2: Run original bash version
docker run -d --name original_controller \
  -e IDRAC_HOST=<IP> \
  -e IDRAC_USERNAME=root \
  -e IDRAC_PASSWORD=<pass> \
  -e FAN_SPEED=10 \
  -e CPU_TEMPERATURE_THRESHOLD=60 \
  tigerblue77/dell_idrac_fan_controller:latest

# Monitor and log temperatures
docker logs -f original_controller > original.log

# Day 3-4: Stop original, run Rust version
docker stop original_controller
cd rust-version
docker-compose up -d
docker-compose logs -f > rust.log

# Compare logs
# - Count fan speed changes
# - Note temperature ranges
# - Observe noise levels
```

## Docker-Specific Testing

### Build the image:
```bash
cd rust-version
docker build -t idrac_fan_controller_rust .
```

**Note**: First build takes 5-10 minutes (compiles Rust code), subsequent builds are faster due to layer caching.

### Test the built image:
```bash
# Test without starting daemon
docker run --rm \
  -e IDRAC_HOST=<IP> \
  -e IDRAC_USERNAME=root \
  -e IDRAC_PASSWORD=<pass> \
  -e RUST_LOG=debug \
  idrac_fan_controller_rust

# Watch output for 5 minutes, then Ctrl+C
```

### Verify binary size:
```bash
docker run --rm idrac_fan_controller_rust ls -lh /usr/local/bin/idrac_fan_controller

# Should show ~2-3 MB
```

### Verify no Rust in final image:
```bash
docker run --rm idrac_fan_controller_rust which rustc

# Should show: (nothing - rustc not found)
```

## Monitoring During Tests

### Terminal 1: Controller logs
```bash
docker-compose logs -f
```

### Terminal 2: iDRAC web interface
Open in browser, monitor:
- Temperature readings
- Fan speeds
- System event log

### Terminal 3: System monitoring
```bash
# CPU temperatures via IPMI
watch -n 5 'ipmitool -I lanplus -H <IP> -U <USER> -P <PASS> sdr type temperature'
```

## Parameter Tuning Guide

Start with conservative (safe) values and adjust:

### Very Conservative (Testing)
```bash
MIN_FAN_SPEED=20       # High minimum
MAX_FAN_SPEED=100
BASE_TEMP=30           # Low activation point
CRITICAL_TEMP=60       # Low critical point
CURVE_STEEPNESS=0.25   # Aggressive response
```

### Balanced (Recommended)
```bash
MIN_FAN_SPEED=10
MAX_FAN_SPEED=80
BASE_TEMP=40
CRITICAL_TEMP=70
CURVE_STEEPNESS=0.15
```

### Aggressive Cooling (Hot environment)
```bash
MIN_FAN_SPEED=15
MAX_FAN_SPEED=100
BASE_TEMP=35
CRITICAL_TEMP=65
CURVE_STEEPNESS=0.2
```

### Quiet (Cool environment)
```bash
MIN_FAN_SPEED=5
MAX_FAN_SPEED=60
BASE_TEMP=45
CRITICAL_TEMP=75
CURVE_STEEPNESS=0.1
```

## Know Your Hardware Limits

Check your CPU specifications:

1. Find your CPU model: `ipmitool fru | grep "Product Name"`

2. Look up on Intel Ark or AMD specs:
   - **Tjunction**: Maximum junction temperature (usually 90-105°C)
   - **Tcase**: Maximum case temperature (usually 60-85°C)

3. Set your `CRITICAL_TEMP` at least **10-15°C below Tcase**

Example:
- Intel Xeon E5-2630L v2: Tcase = 63°C
- Safe CRITICAL_TEMP = 50-55°C

## Success Criteria

After Level 5 testing, you should observe:

✅ **Stability**:
- No crashes or restarts
- Consistent behavior
- Clean shutdown/startup

✅ **Temperature Control**:
- CPU temps stay below your critical threshold
- No thermal throttling (check system logs)
- Reasonable temperature ranges

✅ **Fan Behavior**:
- Smooth speed transitions
- No rapid oscillations
- Noise is acceptable
- Speeds correlate with temps

✅ **Performance**:
- Low CPU usage (<1%)
- Low memory usage (<10 MB)
- No IPMI errors

## Red Flags - Stop Testing If You See:

🚩 **CPU temp > 70°C** (adjust threshold down)
🚩 **Rapid fan speed oscillations** (increase hysteresis or check interval)
🚩 **IPMI command failures** (connectivity issue)
🚩 **Controller crashes repeatedly** (check logs, report bug)
🚩 **Fans at minimum while temp rising** (curve misconfigured)

## Rollback Plan

If Rust version doesn't work for you:

```bash
# Stop Rust version
docker-compose down

# Return to original bash version
docker run -d --name dell_fan_controller \
  --restart=unless-stopped \
  -e IDRAC_HOST=<IP> \
  -e IDRAC_USERNAME=root \
  -e IDRAC_PASSWORD=<pass> \
  -e FAN_SPEED=10 \
  -e CPU_TEMPERATURE_THRESHOLD=60 \
  tigerblue77/dell_idrac_fan_controller:latest
```

Original version is battle-tested and works reliably!

## Final Notes

- **Don't rush**: Complete each testing level successfully before proceeding
- **Document your settings**: Note what works for your environment
- **Monitor initially**: Check logs daily for the first week
- **Seasonal adjustment**: You may need different settings in summer vs winter
- **Hardware changes**: Retest if you add/remove components or change workloads

The exponential curve provides smoother control, but the original's binary approach is simpler and proven. Choose what works best for your needs!

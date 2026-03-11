# Quick Start Guide

## Prerequisites

1. **ipmitool** must be installed:
   ```bash
   # Debian/Ubuntu
   sudo apt install ipmitool
   
   # RHEL/CentOS
   sudo yum install ipmitool
   
   # Alpine
   apk add ipmitool
   ```

2. **Rust toolchain** (for building from source):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Dell PowerEdge server** with iDRAC firmware < 3.30.30.30

## Option 1: Docker (Recommended)

### Remote iDRAC
```bash
cd rust-version

# Edit docker-compose.yml and set:
# - IDRAC_HOST=<your iDRAC IP>
# - IDRAC_USERNAME=<your username>
# - IDRAC_PASSWORD=<your password>

docker-compose up -d
docker-compose logs -f
```

### Local iDRAC
```bash
cd rust-version

# Ensure docker-compose.yml has:
# - IDRAC_HOST=local
# - devices: /dev/ipmi0:/dev/ipmi0:rw

docker-compose up -d
docker-compose logs -f
```

## Option 2: Build from Source

```bash
cd rust-version

# Build
cargo build --release

# Set environment variables
export IDRAC_HOST=192.168.1.100
export IDRAC_USERNAME=root
export IDRAC_PASSWORD=calvin
export MIN_FAN_SPEED=10
export MAX_FAN_SPEED=80
export BASE_TEMP=40
export CRITICAL_TEMP=70
export CURVE_STEEPNESS=0.15

# Run
./target/release/idrac_fan_controller
```

## Option 3: Pre-built Binary (if available)

```bash
# Download the binary for your platform
wget https://github.com/.../idrac_fan_controller-linux-amd64

# Make executable
chmod +x idrac_fan_controller-linux-amd64

# Run with environment variables
IDRAC_HOST=192.168.1.100 \
IDRAC_USERNAME=root \
IDRAC_PASSWORD=calvin \
./idrac_fan_controller-linux-amd64
```

## Visualize the Fan Curve

Before deploying, see how the exponential curve will behave:

```bash
cd rust-version

# With default settings
cargo run --example curve_demo

# With custom settings
MIN_FAN_SPEED=10 \
MAX_FAN_SPEED=80 \
BASE_TEMP=35 \
CRITICAL_TEMP=65 \
CURVE_STEEPNESS=0.2 \
cargo run --example curve_demo
```

## Configuration Examples

### Conservative (Quieter, Higher Temps)
```bash
MIN_FAN_SPEED=5
MAX_FAN_SPEED=60
BASE_TEMP=45
CRITICAL_TEMP=75
CURVE_STEEPNESS=0.1
```

### Balanced (Recommended)
```bash
MIN_FAN_SPEED=10
MAX_FAN_SPEED=80
BASE_TEMP=40
CRITICAL_TEMP=70
CURVE_STEEPNESS=0.15
```

### Aggressive (Cooler, Noisier)
```bash
MIN_FAN_SPEED=15
MAX_FAN_SPEED=100
BASE_TEMP=35
CRITICAL_TEMP=65
CURVE_STEEPNESS=0.25
```

## Testing Your Configuration

1. **Start the controller** with your desired settings

2. **Monitor the output** for a few check intervals:
   ```
   Date & time          Inlet  CPU 1  CPU 2  Exhaust  Fan Speed  Comment
   10-03-2026 15:30:45   22°C   42°C   40°C     28°C      10%  Fan speed adjusted
   10-03-2026 15:31:45   23°C   45°C   43°C     29°C      15%  Fan speed adjusted
   10-03-2026 15:32:45   23°C   44°C   42°C     28°C      15%  -
   ```

3. **Put load on the server** (optional):
   ```bash
   # Install stress tool
   sudo apt install stress
   
   # Run CPU stress test
   stress --cpu 8 --timeout 300s
   ```

4. **Observe fan behavior**:
   - Fan speed should increase gradually with temperature
   - No sudden jumps or oscillations
   - Smooth transitions up and down

5. **Verify safety**:
   - Press Ctrl+C to stop the controller
   - Check that Dell default fan control is restored
   - Fans should ramp up briefly when controller stops

## Troubleshooting

### "Could not open device at /dev/ipmi0"
- For local mode, ensure `/dev/ipmi0` exists
- In Docker, verify device is mounted: `--device=/dev/ipmi0:/dev/ipmi0:rw`
- Check permissions: `ls -l /dev/ipmi0`

### "ipmitool command failed"
- For remote mode, verify iDRAC IP is reachable: `ping <IDRAC_HOST>`
- Check IPMI over LAN is enabled in iDRAC settings
- Verify credentials are correct
- Test manually: `ipmitool -I lanplus -H <IP> -U <user> -P <pass> sdr type temperature`

### Fans not responding
- Check that IPMI commands are supported on your iDRAC version
- Ensure iDRAC firmware is < 3.30.30.30
- Some newer iDRAC versions block manual fan control

### Fan speed oscillating
- Increase hysteresis (currently fixed at ±2% in code)
- Decrease `CURVE_STEEPNESS` for gentler response
- Increase `CHECK_INTERVAL` for less frequent adjustments

### Temperatures too high
- Decrease `BASE_TEMP` and `CRITICAL_TEMP`
- Increase `MIN_FAN_SPEED` and `MAX_FAN_SPEED`
- Increase `CURVE_STEEPNESS` for more aggressive cooling
- Check server airflow and dust buildup

### Fans too loud
- Increase `BASE_TEMP` and `CRITICAL_TEMP`
- Decrease `MAX_FAN_SPEED`
- Decrease `CURVE_STEEPNESS` for gentler response

## Monitoring

### View logs
```bash
# Docker
docker-compose logs -f

# Systemd (if setup as service)
journalctl -u idrac_fan_controller -f

# Direct run
# Logs go to stdout
```

### Check if running
```bash
# Docker
docker ps | grep idrac_fan_controller

# Process
ps aux | grep idrac_fan_controller
```

### Stop gracefully
```bash
# Docker
docker-compose down

# Process
kill -SIGTERM <pid>
# or
Ctrl+C
```

## Next Steps

- Read [README.md](README.md) for detailed documentation
- See [COMPARISON.md](COMPARISON.md) for differences from original
- Adjust configuration based on your temperature observations
- Set up as a systemd service for auto-start
- Monitor for a few days to ensure stability

## Support

For issues or questions:
1. Check the troubleshooting section above
2. Review the original project: https://github.com/tigerblue77/Dell_iDRAC_fan_controller_Docker
3. Check IPMI compatibility for your iDRAC version
4. Verify hardware compatibility (Dell PowerEdge servers only)

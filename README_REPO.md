# Dell iDRAC Fan Controller - Rust Edition

**Smooth, progressive fan control for Dell PowerEdge servers using an exponential curve.**

[![Docker Hub](https://img.shields.io/docker/v/maanstr/idrac-fan-controller-rust?label=Docker%20Hub)](https://hub.docker.com/r/maanstr/idrac-fan-controller-rust)
[![Docker Image Size](https://img.shields.io/docker/image-size/maanstr/idrac-fan-controller-rust/latest)](https://hub.docker.com/r/maanstr/idrac-fan-controller-rust)

This is a Rust rewrite of the [Dell iDRAC Fan Controller](https://github.com/tigerblue77/Dell_iDRAC_fan_controller_Docker) with a major enhancement: **exponential fan curve control** instead of simple on/off switching.

## 🎯 Quick Start

```bash
docker run -d \
  --name idrac-fan-controller \
  -e IDRAC_HOST=192.168.1.100 \
  -e IDRAC_USERNAME=root \
  -e IDRAC_PASSWORD=yourpassword \
  -e MIN_FAN_SPEED=10 \
  -e MAX_FAN_SPEED=80 \
  -e BASE_TEMP=40 \
  -e CRITICAL_TEMP=70 \
  maanstr/idrac-fan-controller-rust:latest
```

## ✨ Key Features

### Exponential Fan Curve
Unlike the original which uses binary on/off (either static 5% or Dell default ~60%), this uses a **smooth exponential curve**:

- 40°C → 10% fans (quiet)
- 55°C → 28% fans (moderate)
- 70°C → 80% fans (aggressive)

**Result:** Smooth, progressive cooling with no jarring transitions.

### Additional Benefits
- ⚡ **Low resource usage** (~0.1% CPU, ~5MB RAM)
- 🔒 **Auto-restore** Dell default on exit/crash
- 🎚️ **Hysteresis** prevents oscillation (±2%)
- 🛡️ **Type safe** Rust implementation
- 📊 **Predictable** behavior vs Dell's dynamic mode

## 📦 Installation

### Docker (Recommended)

**Quick run:**
```bash
docker run -d \
  --name idrac-fan-controller \
  -e IDRAC_HOST=192.168.1.100 \
  -e IDRAC_USERNAME=root \
  -e IDRAC_PASSWORD=yourpassword \
  maanstr/idrac-fan-controller-rust:latest
```

**Docker Compose:**
```yaml
version: '3.8'
services:
  idrac-fan-controller:
    image: maanstr/idrac-fan-controller-rust:latest
    container_name: idrac-fan-controller
    restart: unless-stopped
    environment:
      - IDRAC_HOST=192.168.1.100
      - IDRAC_USERNAME=root
      - IDRAC_PASSWORD=yourpassword
      - MIN_FAN_SPEED=10
      - MAX_FAN_SPEED=80
      - BASE_TEMP=40
      - CRITICAL_TEMP=70
      - CURVE_STEEPNESS=0.15
```

### Unraid

See [UNRAID_SETUP.md](UNRAID_SETUP.md) for detailed instructions.

## ⚙️ Configuration

### Required
- `IDRAC_HOST` - iDRAC IP address or `local`
- `IDRAC_USERNAME` - iDRAC username (default: `root`)
- `IDRAC_PASSWORD` - iDRAC password (default: `calvin`)

### Fan Curve Settings
- `MIN_FAN_SPEED` - Minimum fan % (default: `5`)
- `MAX_FAN_SPEED` - Maximum fan % (default: `100`)
- `BASE_TEMP` - Temp for min speed in °C (default: `40`)
- `CRITICAL_TEMP` - Temp for max speed in °C (default: `70`)
- `CURVE_STEEPNESS` - Curve aggressiveness (default: `0.15`, range: 0.1-0.3)

### Other
- `CHECK_INTERVAL` - Seconds between checks (default: `60`)
- `RUST_LOG` - Log level: `debug`, `info`, `warn`, `error` (default: `info`)

## 🎚️ Tuning Your Curve

### Too Loud?
```bash
BASE_TEMP=45
MAX_FAN_SPEED=70
CURVE_STEEPNESS=0.1
```

### Temps Too High?
```bash
BASE_TEMP=35
MIN_FAN_SPEED=15
CURVE_STEEPNESS=0.25
```

### Balanced (Recommended)
```bash
MIN_FAN_SPEED=10
MAX_FAN_SPEED=80
BASE_TEMP=40
CRITICAL_TEMP=70
CURVE_STEEPNESS=0.15
```

## 📊 How It Works

**Exponential Curve Formula:**
```
For T between BASE_TEMP and CRITICAL_TEMP:
  normalized = (T - BASE_TEMP) / (CRITICAL_TEMP - BASE_TEMP)
  exp_factor = (1 - e^(-k·normalized·10)) / (1 - e^(-k·10))
  fan_speed = MIN + (MAX - MIN) · exp_factor
```

**Example behavior** (40-70°C range):

| Temp | Fan Speed | Change |
|------|-----------|--------|
| 40°C | 10%       | Base   |
| 45°C | 13%       | +3%    |
| 50°C | 18%       | +5%    |
| 55°C | 28%       | +10%   |
| 60°C | 43%       | +15%   |
| 65°C | 62%       | +19%   |
| 70°C | 80%       | +18%   |

Notice how the increase accelerates as temperature rises - this is the exponential curve in action!

## 🆚 Comparison with Original

| Feature | Original (Bash) | This (Rust) |
|---------|----------------|-------------|
| Fan Control | Binary (5% OR Dell default) | Exponential curve (5-100%) |
| Transitions | Abrupt jumps | Smooth progression |
| Noise | Can be choppy | Progressive |
| Predictability | Unpredictable (Dell mode) | Fully predictable |
| Resource Use | ~1-2% CPU | ~0.1% CPU |
| Image Size | N/A | ~50 MB |

See [COMPARISON.md](COMPARISON.md) for detailed comparison.

## 🔧 Building from Source

### Prerequisites
- Docker
- Rust 1.70+ (optional, Docker handles it)

### Build
```bash
# Build for Docker Hub
./build-local.sh YOUR_DOCKERHUB_USERNAME

# Or build locally
docker build -t idrac-fan-controller-rust .
```

See [MANUAL_WORKFLOW.md](MANUAL_WORKFLOW.md) for complete instructions.

## 🧪 Testing

⚠️ **Read [SAFE_TESTING.md](SAFE_TESTING.md) before first use!**

Quick test:
```bash
./test-local.sh
```

## 🛡️ Safety

The controller automatically restores Dell default fan control when:
- Container stops
- Process crashes  
- SIGTERM/SIGINT/SIGQUIT received
- Any fatal error occurs

**Your server will never be left with inadequate cooling.**

## 📋 Requirements

- Dell PowerEdge server (R610, R620, R710, R720, R730, etc.)
- iDRAC firmware < 3.30.30.30 (IPMI support required)
- IPMI over LAN enabled (for remote mode)
- `ipmitool` (included in Docker image)

## 📖 Documentation

- [START_HERE.md](START_HERE.md) - Quick 8-step guide
- [MANUAL_WORKFLOW.md](MANUAL_WORKFLOW.md) - Build → Docker Hub → Unraid
- [SAFE_TESTING.md](SAFE_TESTING.md) - 6-level testing strategy
- [UNRAID_SETUP.md](UNRAID_SETUP.md) - Unraid-specific guide
- [COMPARISON.md](COMPARISON.md) - Detailed comparison with original
- [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) - Technical overview

## 🐛 Troubleshooting

**Container won't start?**
- Check iDRAC is reachable: `ping YOUR_IDRAC_IP`
- Verify IPMI over LAN is enabled in iDRAC settings
- Check credentials are correct

**No temperature readings?**
- Check logs: `docker logs idrac-fan-controller`
- Verify iDRAC firmware version (must be < 3.30.30.30)

**Fans too loud/quiet?**
- Adjust `BASE_TEMP`, `CRITICAL_TEMP`, `CURVE_STEEPNESS`
- See tuning guide above

See [SAFE_TESTING.md](SAFE_TESTING.md) for more troubleshooting.

## 🤝 Contributing

Contributions welcome! Please:
1. Test thoroughly (see [SAFE_TESTING.md](SAFE_TESTING.md))
2. Add unit tests for new features
3. Update documentation
4. Ensure backwards compatibility

## 📜 License

Based on the original [Dell_iDRAC_fan_controller_Docker](https://github.com/tigerblue77/Dell_iDRAC_fan_controller_Docker) by tigerblue77.

This Rust rewrite maintains compatibility with the original project's Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License.

## 🙏 Acknowledgments

- Original project by [tigerblue77](https://github.com/tigerblue77)
- Inspired by the need for smoother fan control
- Built with Rust for performance and reliability

## 📦 Docker Hub

Available at: https://hub.docker.com/r/maanstr/idrac-fan-controller-rust

```bash
docker pull maanstr/idrac-fan-controller-rust:latest
```

## 🔗 Links

- [Docker Hub](https://hub.docker.com/r/maanstr/idrac-fan-controller-rust)
- [Original Project](https://github.com/tigerblue77/Dell_iDRAC_fan_controller_Docker)
- [Dell iDRAC Documentation](https://www.dell.com/support/kbdoc/en-us/000134243/how-to-use-the-integrated-dell-remote-access-controller-idrac)

---

**Made with ❤️ and Rust**

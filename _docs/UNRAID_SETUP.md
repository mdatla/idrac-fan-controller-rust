# Unraid Setup Guide

## Overview

This guide covers installing the Dell iDRAC Fan Controller on Unraid in two ways:
1. **Local testing** - Load a manually built image
2. **Production** - Use a published Docker Hub image

## Prerequisites

- Unraid 6.8 or newer
- Dell PowerEdge server with iDRAC firmware < 3.30.30.30
- Network access to your iDRAC (or local IPMI device passthrough)

## Method 1: Local Testing (Build and Load Manually)

Use this method to test before publishing to Docker Hub.

### Step 1: Build the Image

On your development machine (not Unraid):

```bash
cd oss-repos/Dell_iDRAC_fan_controller_Docker/rust-version

# Build the image
./build-local.sh

# This creates: idrac-fan-controller-rust:latest
```

### Step 2: Save the Image to a File

```bash
# Export the image to a tar file
docker save idrac-fan-controller-rust:latest -o idrac-fan-controller-rust.tar

# Check the file size (should be ~50-100 MB compressed)
ls -lh idrac-fan-controller-rust.tar
```

### Step 3: Transfer to Unraid

Transfer the .tar file to your Unraid server:

```bash
# Option A: Using SCP
scp idrac-fan-controller-rust.tar root@unraid-ip:/tmp/

# Option B: Using Unraid shares
# Copy to: \\unraid-server\appdata\idrac-controller\
```

### Step 4: Load the Image on Unraid

SSH into your Unraid server and load the image:

```bash
ssh root@unraid-ip

# Load the image
docker load -i /tmp/idrac-fan-controller-rust.tar

# Verify it loaded
docker images | grep idrac

# You should see:
# idrac-fan-controller-rust   latest   <id>   <time>   ~50MB
```

### Step 5: Run Manually for Testing

Test the container manually first:

```bash
# Quick test (replace with your iDRAC details)
docker run --rm \
  -e IDRAC_HOST=192.168.1.100 \
  -e IDRAC_USERNAME=root \
  -e IDRAC_PASSWORD=yourpassword \
  -e MIN_FAN_SPEED=20 \
  -e BASE_TEMP=35 \
  -e CRITICAL_TEMP=60 \
  -e RUST_LOG=info \
  idrac-fan-controller-rust:latest

# Press Ctrl+C after a few minutes to stop
# Verify Dell default fan control is restored
```

### Step 6: Add to Unraid Docker (Manual Template)

1. In Unraid WebUI, go to **Docker** tab
2. Click **Add Container**
3. Fill in the following:

**Basic Settings:**
- Name: `iDRAC-Fan-Controller`
- Repository: `idrac-fan-controller-rust:latest`
- Icon URL: (leave empty or use custom)
- Network Type: `bridge`

**Environment Variables:**

Click "Add another Path, Port, Variable, Label or Device" for each:

| Name | Key | Value | Description |
|------|-----|-------|-------------|
| iDRAC Host | IDRAC_HOST | `192.168.1.100` | Your iDRAC IP |
| iDRAC Username | IDRAC_USERNAME | `root` | iDRAC username |
| iDRAC Password | IDRAC_PASSWORD | `yourpassword` | iDRAC password |
| Min Fan Speed | MIN_FAN_SPEED | `10` | Minimum % |
| Max Fan Speed | MAX_FAN_SPEED | `80` | Maximum % |
| Base Temp | BASE_TEMP | `40` | Temp for min speed (°C) |
| Critical Temp | CRITICAL_TEMP | `70` | Temp for max speed (°C) |
| Curve Steepness | CURVE_STEEPNESS | `0.15` | 0.1-0.3 range |
| Check Interval | CHECK_INTERVAL | `60` | Seconds |
| Log Level | RUST_LOG | `info` | debug/info/warn/error |

4. Click **Apply**
5. Watch the logs to verify it's working

### Step 7: Monitor and Adjust

```bash
# View logs
docker logs -f iDRAC-Fan-Controller

# Should show:
# Dell iDRAC Fan Controller (Rust Edition with Exponential Curve)
# Server model: DELL PowerEdge R720
# ...temperature readings...
```

Watch for a few hours and adjust temperatures/speeds as needed.

## Method 2: Production Setup (Docker Hub)

Use this after you've tested locally and published to Docker Hub.

### Step 1: Install from Docker Hub

1. In Unraid WebUI, go to **Docker** tab
2. Click **Add Container**
3. At the top, click "Template repositories" (or use Community Applications)

**Manual Docker Hub Setup:**

- Repository: `YOUR_USERNAME/idrac-fan-controller-rust:latest`
- (Fill in environment variables same as Method 1)

### Step 2: Using Unraid Template (Recommended)

After publishing your template XML file:

1. Go to **Docker** → **Add Container**
2. Template: Select **iDRAC-Fan-Controller-Rust** from dropdown
3. Fill in your iDRAC details
4. Adjust fan curve settings
5. Click **Apply**

## Unraid-Specific Configuration

### For Local iDRAC (Passthrough)

If your Unraid server IS the Dell server:

1. Find IPMI device:
   ```bash
   ls -l /dev/ipmi*
   # Usually /dev/ipmi0
   ```

2. Add device mapping in container:
   - Click "Add another Path, Port, Variable, Label or Device"
   - Config Type: **Device**
   - Name: `IPMI Device`
   - Container Path: `/dev/ipmi0`
   - Host Path: `/dev/ipmi0`
   - Access Mode: `rw`

3. Set environment variable:
   - IDRAC_HOST: `local`

### Recommended Settings for Unraid

**Conservative (Quiet):**
```bash
MIN_FAN_SPEED=10
MAX_FAN_SPEED=70
BASE_TEMP=42
CRITICAL_TEMP=72
CURVE_STEEPNESS=0.12
```

**Balanced:**
```bash
MIN_FAN_SPEED=15
MAX_FAN_SPEED=85
BASE_TEMP=40
CRITICAL_TEMP=70
CURVE_STEEPNESS=0.15
```

**Aggressive (24/7 heavy workload):**
```bash
MIN_FAN_SPEED=20
MAX_FAN_SPEED=100
BASE_TEMP=35
CRITICAL_TEMP=65
CURVE_STEEPNESS=0.20
```

### Auto-Start on Boot

1. In container settings, set:
   - **Autostart**: `Yes`
   - **Network Type**: `bridge` (or `host` if having issues)

2. If using local IPMI, ensure device is available at boot

## Monitoring in Unraid

### View Logs

**Method 1: WebUI**
1. Go to **Docker** tab
2. Click container icon → **Logs**

**Method 2: Terminal**
```bash
docker logs -f iDRAC-Fan-Controller

# Or via Unraid terminal:
docker logs --tail 50 iDRAC-Fan-Controller
```

### Expected Output

```
Dell iDRAC Fan Controller (Rust Edition with Exponential Curve)
================================================================

Server model: DELL PowerEdge R720
iDRAC/IPMI host: 192.168.1.100
...

    Date & time          Inlet  CPU 1  CPU 2  Exhaust  Fan Speed  Comment
10-03-2026 15:30:45   22°C   42°C   40°C     28°C      10%  Fan speed adjusted
10-03-2026 15:31:45   23°C   45°C   43°C     29°C      15%  Fan speed adjusted
...
```

### Check Container Status

```bash
docker ps | grep idrac

# Should show:
# <id>  idrac-fan-controller-rust  Up 2 hours  (healthy)
```

## Troubleshooting on Unraid

### Container Won't Start

Check logs:
```bash
docker logs iDRAC-Fan-Controller
```

Common issues:
- **IPMI connection failed**: Verify iDRAC IP is reachable from Unraid
- **Device not found**: For local mode, check `/dev/ipmi0` exists
- **Permission denied**: Container needs access to IPMI device

### Fans Not Responding

1. Test IPMI manually from Unraid terminal:
```bash
ipmitool -I lanplus -H 192.168.1.100 -U root -P yourpass sdr type temperature
```

2. If that works, check container environment variables are correct

### Container Keeps Restarting

1. Check logs for errors
2. Verify iDRAC credentials
3. Ensure iDRAC firmware is < 3.30.30.30
4. Test with conservative settings first

### High CPU Usage

Should be <1%. If higher:
- Check `CHECK_INTERVAL` (should be ≥30 seconds)
- Review logs for errors/retries
- Verify network connectivity is stable

## Updating the Container

### Method 1: Local Image Update

1. Build new image on dev machine
2. Save and transfer: `docker save ... > new-image.tar`
3. On Unraid:
   ```bash
   docker stop iDRAC-Fan-Controller
   docker rm iDRAC-Fan-Controller
   docker load -i /tmp/new-image.tar
   # Recreate container in WebUI
   ```

### Method 2: Docker Hub Update

1. Push new image to Docker Hub
2. In Unraid WebUI:
   - Docker tab → Container → **Force Update**
   - Or: `docker pull YOUR_USERNAME/idrac-fan-controller-rust:latest`

## Backup Configuration

Save your working configuration:

```bash
# Export container config
docker inspect iDRAC-Fan-Controller > /boot/config/idrac-controller-backup.json

# Or use Unraid's built-in Docker backup
# Settings → Docker → Backup
```

## Fan Curve Tuning for Unraid

Monitor temps during typical workload:

```bash
# Watch live temperature updates
docker logs -f iDRAC-Fan-Controller | grep "Date & time"
```

Adjust based on your needs:

**Too loud?**
- Increase `BASE_TEMP` by 2-5°C
- Decrease `MAX_FAN_SPEED` to 70-80%
- Decrease `CURVE_STEEPNESS` to 0.10-0.12

**Temps too high?**
- Decrease `BASE_TEMP` by 2-5°C
- Increase `MIN_FAN_SPEED` by 5-10%
- Increase `CURVE_STEEPNESS` to 0.18-0.25

## Safety Notes

⚠️ **Important**: The container will restore Dell default fan control when:
- Container stops
- Unraid reboots
- Container crashes
- You manually stop it

This ensures your server is never left with inadequate cooling.

## Community Template Submission

After testing, share with Unraid community:

1. Fork the Community Applications repository
2. Add your template XML to the repository
3. Submit a pull request
4. Users can install via CA GUI

Template location: `unraid/idrac-fan-controller-rust.xml`

## Next Steps

1. ✅ Test locally with `./test-local.sh` first
2. ✅ Load manually on Unraid and monitor for 24 hours
3. ✅ Publish to Docker Hub when stable
4. ✅ Create template XML with your Docker Hub URL
5. ✅ Share with community

## Support

For Unraid-specific issues:
- Check Unraid forums
- Review Docker logs
- Ensure iDRAC/IPMI access works

For controller issues:
- See SAFE_TESTING.md
- Review README.md
- Check GitHub issues

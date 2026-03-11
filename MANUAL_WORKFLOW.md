# Manual Build → Docker Hub → Unraid Testing Workflow

Complete workflow to build, publish, and test on Unraid manually. Once everything works, we can automate with GitHub Actions.

## Phase 1: Build and Publish to Docker Hub

### Step 1: Get Your Docker Hub Username

1. Go to https://hub.docker.com
2. Sign up or log in
3. Note your username (e.g., `johndoe`)

### Step 2: Create Repository on Docker Hub

1. Click **Repositories** → **Create Repository**
2. Name: `idrac-fan-controller-rust`
3. Description: "Dell iDRAC Fan Controller with Exponential Curve - Rust Edition"
4. Visibility: **Public** (or Private if you prefer)
5. Click **Create**

Your image will be: `YOUR_USERNAME/idrac-fan-controller-rust`

### Step 3: Login to Docker Hub

```bash
docker login

# Enter your username
# Enter password (or access token - recommended)
```

**Pro tip**: Use an access token instead of password:
- Go to https://hub.docker.com/settings/security
- New Access Token → Name: "manual-publish" → Generate
- Use token as password when running `docker login`

### Step 4: Build the Image

```bash
cd oss-repos/Dell_iDRAC_fan_controller_Docker/rust-version

# Build for your Docker Hub username
# Replace YOUR_USERNAME with your actual username!
./build-local.sh YOUR_USERNAME

# Example:
# ./build-local.sh johndoe
```

This will build: `YOUR_USERNAME/idrac-fan-controller-rust:latest`

**Build time**: 5-10 minutes on first build (compiling Rust)

### Step 5: Push to Docker Hub

```bash
# Push the image
docker push YOUR_USERNAME/idrac-fan-controller-rust:latest

# Example:
# docker push johndoe/idrac-fan-controller-rust:latest
```

**Upload time**: 1-3 minutes depending on connection (~50 MB)

### Step 6: Verify on Docker Hub

1. Go to https://hub.docker.com/r/YOUR_USERNAME/idrac-fan-controller-rust
2. Check the image appears
3. Verify the **Tags** tab shows `latest`
4. Check size is ~50 MB

✅ **You now have a published Docker image!**

## Phase 2: Test on Unraid

### Step 1: Open Unraid Docker Settings

1. Log into Unraid WebUI
2. Go to **Docker** tab
3. Click **Add Container** at the bottom

### Step 2: Configure Basic Settings

In the "Add Container" form:

**Basic:**
- Name: `iDRAC-Fan-Controller`
- Repository: `YOUR_USERNAME/idrac-fan-controller-rust:latest`
- Docker Hub URL: `https://hub.docker.com/r/YOUR_USERNAME/idrac-fan-controller-rust`
- Icon URL: (leave empty for now)
- Network Type: `bridge`
- Console shell command: `sh`

### Step 3: Add Environment Variables

Click "Add another Path, Port, Variable, Label or Device" for each variable:

**Required:**

| Name | Key | Value | Default |
|------|-----|-------|---------|
| iDRAC Host | `IDRAC_HOST` | `192.168.1.100` | (your iDRAC IP) |
| iDRAC Username | `IDRAC_USERNAME` | `root` | root |
| iDRAC Password | `IDRAC_PASSWORD` | `yourpassword` | calvin |

**Fan Curve Settings:**

| Name | Key | Value | Default |
|------|-----|-------|---------|
| Min Fan Speed | `MIN_FAN_SPEED` | `10` | 5 |
| Max Fan Speed | `MAX_FAN_SPEED` | `80` | 100 |
| Base Temp | `BASE_TEMP` | `40` | 40 |
| Critical Temp | `CRITICAL_TEMP` | `70` | 70 |
| Curve Steepness | `CURVE_STEEPNESS` | `0.15` | 0.15 |

**Other:**

| Name | Key | Value | Default |
|------|-----|-------|---------|
| Check Interval | `CHECK_INTERVAL` | `60` | 60 |
| Log Level | `RUST_LOG` | `info` | info |

### Step 4: Apply and Start

1. Click **Apply** at the bottom
2. Unraid will pull the image from Docker Hub
3. Container will start automatically

### Step 5: Check the Logs

1. Click the container icon → **Logs**
2. Or in terminal: `docker logs -f iDRAC-Fan-Controller`

**Expected output:**
```
Dell iDRAC Fan Controller (Rust Edition with Exponential Curve)
================================================================

Server model: DELL PowerEdge R720
iDRAC/IPMI host: 192.168.1.100
Generation 14 or newer: false

Fan curve configuration:
  Min fan speed: 10%
  Max fan speed: 80%
  Base temperature: 40°C
  Critical temperature: 70°C
  Curve steepness: 0.15
  Check interval: 60s

    Date & time          Inlet  CPU 1  CPU 2  Exhaust  Fan Speed  Comment
10-03-2026 15:30:45   22°C   42°C   40°C     28°C      10%  Fan speed adjusted
```

### Step 6: Monitor for Issues

Watch the logs for 10-15 minutes. Check for:

✅ Temperature readings appear
✅ Fan speeds are adjusting
✅ No errors in logs
✅ Fans sound reasonable (not too loud/quiet)

### Step 7: Stop and Verify Safety

1. Stop the container: Click icon → **Stop**
2. Listen to fans - they should ramp up briefly (Dell default restored)
3. Check in logs: Should say "Dell default dynamic fan control profile applied for safety"

✅ **If all looks good, enable auto-start!**

### Step 8: Enable Auto-Start

1. Click container icon → **Edit**
2. Scroll to **Autostart:** → Set to `Yes`
3. Click **Apply**

Container will now start automatically on Unraid boot.

## Phase 3: Fine-Tuning

### Monitor Under Load

Run your typical workload and watch temperatures:

```bash
docker logs -f iDRAC-Fan-Controller
```

### Adjust Fan Curve

Based on observations, adjust settings:

**Too loud?**
1. Edit container
2. Increase `BASE_TEMP` to 45
3. Decrease `MAX_FAN_SPEED` to 70
4. Apply and monitor

**Temps too high?**
1. Edit container
2. Decrease `BASE_TEMP` to 35
3. Increase `MIN_FAN_SPEED` to 15
4. Increase `CURVE_STEEPNESS` to 0.20
5. Apply and monitor

### Save Working Configuration

Once you have settings you like, document them!

## Phase 4: (Optional) Multi-Architecture Build

If you want to support both AMD64 and ARM64:

```bash
# Set up buildx
docker buildx create --name multiarch --use
docker buildx inspect --bootstrap

# Build and push for both architectures
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t YOUR_USERNAME/idrac-fan-controller-rust:latest \
  --push \
  .
```

Most Dell servers use AMD64, so this is optional.

## Phase 5: Version Tagging (Recommended)

Tag releases for better version control:

```bash
# Tag as v1.0.0
docker tag YOUR_USERNAME/idrac-fan-controller-rust:latest \
  YOUR_USERNAME/idrac-fan-controller-rust:v1.0.0

docker push YOUR_USERNAME/idrac-fan-controller-rust:v1.0.0

# Now users can use either:
# - YOUR_USERNAME/idrac-fan-controller-rust:latest (auto-updates)
# - YOUR_USERNAME/idrac-fan-controller-rust:v1.0.0 (pinned version)
```

## Troubleshooting

### Build fails with "no space left on device"

```bash
# Clean up Docker
docker system prune -a

# Check disk space
df -h
```

### Push fails with "unauthorized"

```bash
# Login again
docker login

# Verify username is correct
docker info | grep Username
```

### Can't pull on Unraid

Check:
- Repository name is correct (case-sensitive)
- Image is public (or you're logged into Docker Hub on Unraid)
- Network connectivity from Unraid

### Container won't start on Unraid

Check logs:
```bash
docker logs iDRAC-Fan-Controller
```

Common issues:
- iDRAC IP not reachable from Unraid
- Wrong credentials
- iDRAC firmware too new (≥ 3.30.30.30)

## Complete Example Workflow

Here's a complete example with actual commands:

```bash
# 1. Build and push (on your dev machine)
cd oss-repos/Dell_iDRAC_fan_controller_Docker/rust-version
docker login
./build-local.sh johndoe
docker push johndoe/idrac-fan-controller-rust:latest

# 2. Tag version
docker tag johndoe/idrac-fan-controller-rust:latest \
  johndoe/idrac-fan-controller-rust:v1.0.0
docker push johndoe/idrac-fan-controller-rust:v1.0.0

# 3. On Unraid WebUI:
# - Add Container
# - Repository: johndoe/idrac-fan-controller-rust:latest
# - Add environment variables for your iDRAC
# - Apply

# 4. Monitor
docker logs -f iDRAC-Fan-Controller

# 5. If all good, enable autostart in container settings
```

## Next Steps: GitHub Actions Automation

Once you've verified everything works:

1. ✅ Image builds successfully
2. ✅ Pushes to Docker Hub
3. ✅ Works on Unraid
4. ✅ Fan control behaves as expected
5. ✅ Temperatures are safe

**Then** we can set up GitHub Actions to automate the build/push process!

The automation will:
- Build on every push to main branch
- Tag releases automatically
- Build for multiple architectures
- Push to Docker Hub without manual intervention

But first, let's make sure it all works manually!

## Quick Reference

**Build:**
```bash
./build-local.sh YOUR_USERNAME
```

**Push:**
```bash
docker push YOUR_USERNAME/idrac-fan-controller-rust:latest
```

**Test locally before Unraid:**
```bash
./test-local.sh
```

**View logs on Unraid:**
```bash
docker logs -f iDRAC-Fan-Controller
```

**Update image on Unraid:**
```bash
docker pull YOUR_USERNAME/idrac-fan-controller-rust:latest
# Then restart container in WebUI
```

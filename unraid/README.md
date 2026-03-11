# Unraid Template

This directory contains the Unraid Community Applications template for the Dell iDRAC Fan Controller.

## Files

- **idrac-fan-controller-rust.xml** - Unraid template with all default values pre-configured
- **icon.png** - Icon for Unraid (to be added)

## Using This Template

### Method 1: Community Applications (Recommended - Future)

Once this template is added to Community Applications:
1. Open Unraid WebUI
2. Go to **Apps** tab
3. Search for "iDRAC Fan Controller"
4. Click **Install**
5. Configure your iDRAC IP and credentials
6. Click **Apply**

### Method 2: Manual Template URL (Available Now)

1. Open Unraid WebUI
2. Go to **Docker** tab
3. Click **Add Container**
4. At the top, change dropdown from "Select a template" to **"Template repositories"**
5. Add this URL: `https://raw.githubusercontent.com/mdatla/idrac-fan-controller-rust/main/unraid/idrac-fan-controller-rust.xml`
6. Or use the "Template URL" field when adding a container

### Method 3: Manual Configuration (If template doesn't work)

See [UNRAID_SETUP.md](../_docs/UNRAID_SETUP.md) for manual docker run instructions.

## Pre-Configured Defaults

The template comes with these recommended defaults:

| Variable | Default | Description |
|----------|---------|-------------|
| `IDRAC_HOST` | `192.168.1.100` | **Change this!** Your iDRAC IP |
| `IDRAC_USERNAME` | `root` | Usually correct |
| `IDRAC_PASSWORD` | `calvin` | **Change this!** Default Dell password |
| `MIN_FAN_SPEED` | `10` | 10% minimum (quiet) |
| `MAX_FAN_SPEED` | `80` | 80% maximum (not ear-splitting) |
| `BASE_TEMP` | `40` | 40°C - temp for min fan speed |
| `CRITICAL_TEMP` | `70` | 70°C - temp for max fan speed |
| `CURVE_STEEPNESS` | `0.15` | Balanced exponential curve |
| `CHECK_INTERVAL` | `60` | Check every 60 seconds |
| `RUST_LOG` | `info` | Normal logging |

**All variables are visible by default** - no need to click "Add another Variable"!

## Icon

To add a custom icon:
1. Create or find a 256x256 PNG image
2. Upload to `/unraid/icon.png`
3. Commit and push

For now, Unraid will use a default Docker icon.

## Community Applications Submission

To submit this template to Unraid Community Applications:

1. Fork the Community Applications repository: https://github.com/Squidly271/docker-templates
2. Add your template XML to the appropriate directory
3. Add your icon
4. Submit a pull request
5. Wait for approval

**Requirements:**
- Must be publicly available Docker image ✅ (on Docker Hub)
- Must have icon.png ⚠️ (needs to be added)
- Must have tested and working template ✅
- Template must follow CA guidelines ✅

## Testing the Template

Before submitting to Community Applications, test it:

```bash
# On Unraid, manually add container
# Docker → Add Container
# Template URL: https://raw.githubusercontent.com/mdatla/idrac-fan-controller-rust/main/unraid/idrac-fan-controller-rust.xml
```

Verify:
- All variables show up with correct defaults
- Container starts successfully
- Logs show temperature readings
- Icon displays (once added)

## Support

For Unraid-specific issues:
- Main docs: [UNRAID_SETUP.md](../_docs/UNRAID_SETUP.md)
- GitHub Issues: https://github.com/mdatla/idrac-fan-controller-rust/issues
- Unraid Forums: (link to forum thread if created)

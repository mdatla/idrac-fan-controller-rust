# Publishing to Docker Hub - Manual Guide

## Prerequisites

1. **Docker Hub account**
   - Create account at https://hub.docker.com
   - Note your username (you'll need it below)

2. **Docker installed locally**
   ```bash
   docker --version
   # Should show: Docker version 20.x or newer
   ```

3. **Docker Buildx** (for multi-architecture builds)
   ```bash
   docker buildx version
   # If not installed, Docker Desktop includes it by default
   ```

## Step 1: Create Docker Hub Repository

1. Go to https://hub.docker.com
2. Click "Create Repository"
3. Repository name: `idrac-fan-controller-rust`
4. Description: "Dell iDRAC Fan Controller with Exponential Curve - Rust Edition"
5. Visibility: Public (or Private if you prefer)
6. Click "Create"

Your repository will be at: `docker.io/YOUR_USERNAME/idrac-fan-controller-rust`

## Step 2: Login to Docker Hub

```bash
docker login

# Enter your Docker Hub username
# Enter your Docker Hub password (or access token - recommended)
```

**Security Note**: Use an access token instead of your password:
1. Go to https://hub.docker.com/settings/security
2. Click "New Access Token"
3. Name: "idrac-controller-publish"
4. Permissions: Read, Write, Delete
5. Generate and copy the token
6. Use this token as password when running `docker login`

## Step 3: Build the Image

Navigate to the rust-version directory:

```bash
cd oss-repos/Dell_iDRAC_fan_controller_Docker/rust-version
```

### Option A: Build for Single Architecture (Fast - 5-10 minutes)

Build for your current platform only (amd64 or arm64):

```bash
# Replace YOUR_USERNAME with your Docker Hub username
docker build -t YOUR_USERNAME/idrac-fan-controller-rust:latest .

# Example:
# docker build -t johndoe/idrac-fan-controller-rust:latest .
```

### Option B: Build for Multiple Architectures (Recommended - 10-20 minutes)

Build for both amd64 and arm64 (most Dell servers use amd64):

```bash
# Create and use buildx builder
docker buildx create --name multiarch --use
docker buildx inspect --bootstrap

# Build and push for both architectures
# Replace YOUR_USERNAME with your Docker Hub username
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t YOUR_USERNAME/idrac-fan-controller-rust:latest \
  --push \
  .

# Example:
# docker buildx build \
#   --platform linux/amd64,linux/arm64 \
#   -t johndoe/idrac-fan-controller-rust:latest \
#   --push \
#   .
```

**Note**: The `--push` flag automatically pushes to Docker Hub after building.

## Step 4: Push to Docker Hub (if using Option A)

If you used Option A (single architecture), push the image:

```bash
docker push YOUR_USERNAME/idrac-fan-controller-rust:latest
```

## Step 5: Add Version Tags (Optional but Recommended)

Tag with a version number for better release management:

```bash
# Tag as v1.0.0
docker tag YOUR_USERNAME/idrac-fan-controller-rust:latest \
  YOUR_USERNAME/idrac-fan-controller-rust:v1.0.0

# Push version tag
docker push YOUR_USERNAME/idrac-fan-controller-rust:v1.0.0

# Tag as v1.0 (minor version)
docker tag YOUR_USERNAME/idrac-fan-controller-rust:latest \
  YOUR_USERNAME/idrac-fan-controller-rust:v1.0

docker push YOUR_USERNAME/idrac-fan-controller-rust:v1.0

# Tag as v1 (major version)
docker tag YOUR_USERNAME/idrac-fan-controller-rust:latest \
  YOUR_USERNAME/idrac-fan-controller-rust:v1

docker push YOUR_USERNAME/idrac-fan-controller-rust:v1
```

This allows users to pin to specific versions:
- `:latest` - Always the newest build
- `:v1` - Latest v1.x.x
- `:v1.0` - Latest v1.0.x
- `:v1.0.0` - Specific version

## Step 6: Verify the Upload

1. Go to https://hub.docker.com/r/YOUR_USERNAME/idrac-fan-controller-rust
2. Check that the image appears
3. Verify the tags (should show `latest` and any version tags)
4. Check the image size (~50 MB)

## Step 7: Test Pull the Image

From any machine with Docker:

```bash
docker pull YOUR_USERNAME/idrac-fan-controller-rust:latest

# Verify it works
docker run --rm \
  -e IDRAC_HOST=192.168.1.100 \
  -e IDRAC_USERNAME=root \
  -e IDRAC_PASSWORD=yourpassword \
  YOUR_USERNAME/idrac-fan-controller-rust:latest
```

## Step 8: Update Unraid Template

Edit `unraid/idrac-fan-controller-rust.xml` and replace `yourusername` with your actual Docker Hub username:

```xml
<Repository>YOUR_USERNAME/idrac-fan-controller-rust:latest</Repository>
<Registry>https://hub.docker.com/r/YOUR_USERNAME/idrac-fan-controller-rust</Registry>
```

## Complete Build Script

Here's a complete script that does everything:

```bash
#!/bin/bash
set -e

# Configuration
DOCKERHUB_USERNAME="YOUR_USERNAME"  # CHANGE THIS!
IMAGE_NAME="idrac-fan-controller-rust"
VERSION="1.0.0"

# Navigate to project directory
cd "$(dirname "$0")"

echo "Building Docker image..."

# Build for multiple architectures
docker buildx create --name multiarch --use 2>/dev/null || docker buildx use multiarch
docker buildx inspect --bootstrap

echo "Building and pushing ${DOCKERHUB_USERNAME}/${IMAGE_NAME}..."

docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:latest \
  -t ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:v${VERSION} \
  -t ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:v$(echo $VERSION | cut -d. -f1).$(echo $VERSION | cut -d. -f2) \
  -t ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:v$(echo $VERSION | cut -d. -f1) \
  --push \
  .

echo "✅ Successfully built and pushed:"
echo "  - ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:latest"
echo "  - ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:v${VERSION}"
echo ""
echo "Next steps:"
echo "1. Verify at https://hub.docker.com/r/${DOCKERHUB_USERNAME}/${IMAGE_NAME}"
echo "2. Update unraid/idrac-fan-controller-rust.xml with your username"
echo "3. Test: docker pull ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:latest"
```

Save as `build-and-push.sh`, make executable, and run:

```bash
chmod +x build-and-push.sh
./build-and-push.sh
```

## Updating an Existing Image

When you make changes and want to publish a new version:

1. **Update version number** in build script
2. **Build and push** with new tags:
   ```bash
   docker buildx build \
     --platform linux/amd64,linux/arm64 \
     -t YOUR_USERNAME/idrac-fan-controller-rust:latest \
     -t YOUR_USERNAME/idrac-fan-controller-rust:v1.1.0 \
     --push \
     .
   ```

3. **Update Unraid template** if configuration options changed

## Troubleshooting

### "permission denied" when pushing
- Run `docker login` again
- Verify your Docker Hub username is correct
- Check if using access token instead of password

### "manifest unknown" error
- Make sure repository exists on Docker Hub
- Check repository name matches exactly (case-sensitive)

### Build fails with "no space left on device"
- Clean up Docker images: `docker system prune -a`
- Check available disk space: `df -h`

### Multi-arch build not working
- Enable Docker BuildKit: `export DOCKER_BUILDKIT=1`
- Update Docker Desktop to latest version
- Try removing and recreating buildx: `docker buildx rm multiarch`

### Build is very slow
- First build takes 10-20 minutes (compiling Rust)
- Subsequent builds are faster due to layer caching
- Consider using Option A (single arch) for faster testing

## Best Practices

1. **Always test locally first** before pushing to Docker Hub
2. **Use semantic versioning** (v1.0.0, v1.1.0, v2.0.0)
3. **Keep `latest` tag** for users who want auto-updates
4. **Document breaking changes** in version tags
5. **Test pull and run** after publishing
6. **Update README** with correct Docker Hub path

## Docker Hub Repository Description

Copy this for your Docker Hub repository description:

```
Dell iDRAC Fan Controller with Exponential Curve - Rust Edition

Smooth, progressive fan speed control for Dell PowerEdge servers using an exponential curve instead of binary on/off switching.

Features:
✨ Exponential fan curve for smooth transitions
📊 Configurable temperature thresholds and curve steepness  
🔄 Hysteresis to prevent oscillation
🛡️ Auto-restore Dell default fan control on exit
⚡ Low resource usage (~0.1% CPU, ~5MB RAM)

Compatible with Dell PowerEdge servers with iDRAC firmware < 3.30.30.30

Documentation: https://github.com/YOUR_USERNAME/Dell_iDRAC_fan_controller_Docker/tree/main/rust-version

⚠️ Read SAFE_TESTING.md before first use!
```

## Next Steps

After publishing to Docker Hub:
1. See [UNRAID_SETUP.md](UNRAID_SETUP.md) for Unraid installation
2. Update template XML with your Docker Hub username
3. Share your template repository URL with the community

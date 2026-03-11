# Docker Image Tags and Versions

This document explains the available Docker image tags and how to use them.

## Available Tags

### `:latest` (Recommended)
- **Description**: Latest stable build from GitHub Actions
- **Updates**: Automatically built on every push to `main` branch
- **Use case**: Normal production use
- **Architecture**: AMD64 (linux/amd64)

```bash
docker pull maanstr/idrac-fan-controller-rust:latest
```

### `:stable` (Fallback)
- **Description**: Known-good fallback version
- **Updates**: Manually updated only when a version is confirmed working
- **Use case**: Revert if `:latest` has issues
- **Architecture**: AMD64 (linux/amd64)
- **Digest**: `sha256:f87df539325cf58df31ae421b103474cfc73cd8c74d1c7e5b3ec81cf49cb280e`

```bash
docker pull maanstr/idrac-fan-controller-rust:stable
```

### `:main`
- **Description**: Tracks the main branch (same as `:latest`)
- **Updates**: Automatically built with `:latest`
- **Use case**: If you want to explicitly track main branch builds

```bash
docker pull maanstr/idrac-fan-controller-rust:main
```

## When to Use Each Tag

### Normal Use → `:latest`
For day-to-day production use, always use `:latest`. This gets the newest features and fixes.

```yaml
services:
  idrac-fan-controller:
    image: maanstr/idrac-fan-controller-rust:latest
    restart: unless-stopped
```

### Troubleshooting → `:stable`
If `:latest` stops working after an update, revert to `:stable`:

```bash
# Stop current container
docker stop idrac-fan-controller
docker rm idrac-fan-controller

# Switch to stable
docker run -d \
  --name idrac-fan-controller \
  -e IDRAC_HOST=192.168.1.100 \
  maanstr/idrac-fan-controller-rust:stable
```

### Updating → Pull Fresh
Always pull the latest image before recreating containers:

```bash
docker pull maanstr/idrac-fan-controller-rust:latest
docker stop idrac-fan-controller
docker rm idrac-fan-controller
docker run -d --name idrac-fan-controller ...
```

## Build Process

### GitHub Actions (Automated)
Every push to `main` branch triggers an automated build:

1. **Checkout** code from GitHub
2. **Build** Docker image for AMD64
3. **Push** to Docker Hub as `:latest` and `:main`
4. **Total time**: ~2-3 minutes

**Build configuration:**
- Platform: `linux/amd64` only
- Cache: Disabled (ensures clean builds)
- Base image: `rust:1.75-slim` → `debian:bookworm-slim`
- Final size: ~150-160 MB

### Why AMD64 Only?

The GitHub Actions workflow currently builds **AMD64 only** (not multi-arch) because:

1. **Reliability**: Multi-platform builds with buildx were producing broken binaries
2. **Simplicity**: Single-platform builds are faster and more reliable
3. **Target audience**: Most Dell PowerEdge servers run AMD64 processors
4. **Unraid compatibility**: Unraid typically runs on AMD64 hardware

ARM64 support may be added in the future once cross-compilation issues are resolved.

## Image History

### Known Issues (Resolved)
**March 11, 2026** - Multi-platform build produced broken images
- **Problem**: Images built with `platforms: linux/amd64,linux/arm64` would not execute
- **Symptom**: Container exits immediately with code 0, no logs
- **Root cause**: Cross-compilation with Docker buildx + GitHub Actions cache corruption
- **Fix**: Simplified to AMD64 only, disabled cache
- **Status**: ✅ Resolved

## Troubleshooting

### Container Exits Immediately (No Logs)

**Symptom:**
```bash
$ docker logs idrac-fan-controller
(no output)
$ docker ps -a | grep idrac
Exited (0) 2 seconds ago
```

**Solution:**
Pull a fresh image and verify it's the correct build:

```bash
# Remove old image
docker rmi maanstr/idrac-fan-controller-rust:latest

# Pull fresh
docker pull maanstr/idrac-fan-controller-rust:latest

# Check digest
docker image inspect maanstr/idrac-fan-controller-rust:latest \
  --format '{{.RepoDigests}}'

# Should NOT be sha256:49571b064ddf... (that was the broken build)
# If you see the old broken digest, Docker might be caching

# Force remove ALL versions
docker rmi -f $(docker images -q maanstr/idrac-fan-controller-rust)

# Pull again
docker pull maanstr/idrac-fan-controller-rust:latest
```

### Reverting to Stable

If `:latest` has issues after an update:

```bash
# Unraid
docker rm -f IDRAC-Fan-Controler
docker run -d \
  --name='IDRAC-Fan-Controler' \
  --net='bridge' \
  -e 'IDRAC_HOST'='192.168.1.23' \
  -e 'IDRAC_USERNAME'='root' \
  -e 'IDRAC_PASSWORD'='yourpassword' \
  maanstr/idrac-fan-controller-rust:stable

# Docker Compose
# Edit docker-compose.yml:
# image: maanstr/idrac-fan-controller-rust:stable
docker-compose up -d
```

### Checking Image Version

Get detailed information about your current image:

```bash
# Check digest
docker image inspect maanstr/idrac-fan-controller-rust:latest \
  --format '{{.RepoDigests}}'

# Check build date
docker image inspect maanstr/idrac-fan-controller-rust:latest \
  --format '{{.Created}}'

# Check architecture
docker image inspect maanstr/idrac-fan-controller-rust:latest \
  --format '{{.Architecture}} {{.Os}}'

# Should show: amd64 linux
```

## Pinning to Specific Digest

For maximum stability, you can pin to a specific image digest:

```bash
# Get current digest
docker image inspect maanstr/idrac-fan-controller-rust:latest \
  --format '{{index .RepoDigests 0}}'

# Use digest in docker run
docker run -d \
  --name idrac-fan-controller \
  maanstr/idrac-fan-controller-rust@sha256:f87df539325cf58df31ae421b103474cfc73cd8c74d1c7e5b3ec81cf49cb280e
```

This ensures the exact image version never changes, even if `:latest` is updated.

## Future Plans

### Potential Improvements
1. **Re-enable build cache** (carefully tested)
2. **Add ARM64 support** (for Raspberry Pi, etc.)
3. **Versioned releases** (v1.0.0, v1.1.0, etc.)
4. **Multi-stage cache optimization**

### Contributing
If you have experience with GitHub Actions and multi-platform Docker builds, contributions are welcome! See the main README for contribution guidelines.

---

**Summary:**
- Use `:latest` for normal production
- Use `:stable` as a fallback if issues occur
- GitHub Actions builds are now reliable (AMD64 only)
- Always pull fresh images before updating containers

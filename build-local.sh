#!/bin/bash
set -e

DOCKERHUB_USER="maanstr"
IMAGE_NAME="idrac-fan-controller-rust"

echo "================================================"
echo "Dell iDRAC Fan Controller - Local Build Script"
echo "================================================"
echo ""

# Parse arguments
TAG="latest"
PUSH=false

usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --tag TAG     Docker image tag (default: latest)"
    echo "  --push        Push to Docker Hub after building"
    echo "  --help        Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                     # Build :latest locally"
    echo "  $0 --tag stable        # Build :stable locally"
    echo "  $0 --tag stable --push # Build :stable and push to Docker Hub"
    echo ""
}

while [[ $# -gt 0 ]]; do
    case $1 in
        --tag)
            TAG="$2"
            shift 2
            ;;
        --push)
            PUSH=true
            shift
            ;;
        --help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

IMAGE_TAG="${DOCKERHUB_USER}/${IMAGE_NAME}:${TAG}"

echo "Target image : ${IMAGE_TAG}"
echo "Platform     : linux/amd64"
echo "Push         : ${PUSH}"
echo ""
echo "Building Docker image..."
echo "(This will take 5-10 minutes on first build)"
echo ""

# Build AMD64 image using buildx.
# This is required because the development machine (Apple Silicon / arm64) differs
# from the deployment target (Dell PowerEdge 720xd / amd64 running Unraid).
#
# --platform linux/amd64  : cross-compile for the target architecture
# --load                  : load the image into the local Docker daemon
# --provenance=false      : avoid multi-manifest issues when pushing single-platform images
if [ "$PUSH" = true ]; then
    # When pushing, we can build and push in one step
    docker buildx build \
        --platform linux/amd64 \
        --provenance=false \
        --tag "${IMAGE_TAG}" \
        --push \
        .
else
    # For local-only builds, load into Docker daemon
    docker buildx build \
        --platform linux/amd64 \
        --provenance=false \
        --tag "${IMAGE_TAG}" \
        --load \
        .
fi

echo ""
echo "Build complete!"
echo ""
echo "Image    : ${IMAGE_TAG}"
echo "Size     : $(docker images "${IMAGE_TAG}" --format "{{.Size}}" 2>/dev/null || echo "N/A (pushed directly)")"
echo "Platform : $(docker image inspect "${IMAGE_TAG}" --format '{{.Os}}/{{.Architecture}}' 2>/dev/null || echo "linux/amd64")"
echo ""

if [ "$PUSH" = true ]; then
    echo "Image pushed to Docker Hub: ${IMAGE_TAG}"
    echo ""
    echo "Verify on your Unraid box:"
    echo "  docker pull ${IMAGE_TAG}"
    echo "  docker image inspect ${IMAGE_TAG} --format '{{.Os}}/{{.Architecture}}'"
else
    echo "To push to Docker Hub:"
    echo "  $0 --tag ${TAG} --push"
    echo ""
    echo "To test locally (note: ipmitool won't work on macOS, use --rm to auto-clean):"
    echo "  docker run --rm -e RUST_LOG=debug ${IMAGE_TAG}"
fi
echo ""

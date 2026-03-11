#!/bin/bash
set -e

echo "================================================"
echo "Dell iDRAC Fan Controller - Local Build Script"
echo "================================================"
echo ""

# Get Docker Hub username if building for push
if [ -n "$1" ]; then
    DOCKERHUB_USERNAME="$1"
    IMAGE_TAG="${DOCKERHUB_USERNAME}/idrac-fan-controller-rust:latest"
    echo "Building for Docker Hub: ${IMAGE_TAG}"
else
    IMAGE_TAG="idrac-fan-controller-rust:latest"
    echo "Building locally as: ${IMAGE_TAG}"
fi

echo ""
echo "📦 Building Docker image..."
echo "   (This will take 5-10 minutes on first build)"
echo ""

# Build the image
docker build -t "${IMAGE_TAG}" .

echo ""
echo "✅ Build complete!"
echo ""
echo "Image: ${IMAGE_TAG}"
echo "Size: $(docker images ${IMAGE_TAG} --format "{{.Size}}")"
echo ""

if [ -n "$1" ]; then
    echo "To push to Docker Hub, run:"
    echo "  docker login"
    echo "  docker push ${IMAGE_TAG}"
    echo ""
else
    echo "To test locally, run:"
    echo "  docker run --rm -e IDRAC_HOST=192.168.1.100 -e IDRAC_USERNAME=root -e IDRAC_PASSWORD=yourpass ${IMAGE_TAG}"
    echo ""
    echo "To push to Docker Hub later, rebuild with:"
    echo "  ./build-local.sh YOUR_DOCKERHUB_USERNAME"
fi

echo ""
echo "Next steps:"
echo "  1. Test with: ./test-local.sh"
echo "  2. See UNRAID_SETUP.md for Unraid installation"
echo ""

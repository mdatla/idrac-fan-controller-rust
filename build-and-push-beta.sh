#!/bin/bash
# Build and push :beta tag manually (AMD64 only)
# This is useful for testing before creating a PR

set -e

echo "================================================"
echo "Building and Pushing :beta tag (AMD64)"
echo "================================================"
echo ""
echo "⚠️  This will build for AMD64 and push to Docker Hub"
echo ""

read -p "Continue? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Cancelled."
    exit 1
fi

IMAGE_TAG="maanstr/idrac-fan-controller-rust:beta"

echo ""
echo "📦 Building Docker image for AMD64..."
echo ""

# Build for AMD64 (matches production Dell servers)
docker buildx build \
    --platform linux/amd64 \
    -t "${IMAGE_TAG}" \
    --push \
    .

echo ""
echo "✅ Build and push complete!"
echo ""
echo "Image: ${IMAGE_TAG}"
echo "Platform: linux/amd64"
echo ""
echo "To test on your server:"
echo "  docker pull ${IMAGE_TAG}"
echo "  docker image inspect ${IMAGE_TAG} --format '{{.Architecture}}'"
echo "  # Should show: amd64"
echo ""

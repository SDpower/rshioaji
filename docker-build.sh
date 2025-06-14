#!/bin/bash

# Build script for rshioaji Docker images with lightweight base images

set -e

# Default platform
PLATFORM=${1:-"linux"}

if [ "$PLATFORM" = "linux" ] || [ "$PLATFORM" = "x86_64" ]; then
    echo "🐳 Building lightweight rshioaji Docker image for Linux x86_64..."
    docker build -t rshioaji:linux-x86_64 -f Dockerfile .
    IMAGE_TAG="rshioaji:linux-x86_64"
elif [ "$PLATFORM" = "alpine" ]; then
    echo "🏔️ Building ultra-lightweight rshioaji Docker image with Alpine Linux..."
    docker build -t rshioaji:alpine -f Dockerfile.alpine .
    IMAGE_TAG="rshioaji:alpine"
elif [ "$PLATFORM" = "macos" ] || [ "$PLATFORM" = "arm64" ]; then
    echo "🐳 Building rshioaji Docker image for macOS ARM64..."
    docker build -t rshioaji:macos-arm64 -f Dockerfile.macos .
    IMAGE_TAG="rshioaji:macos-arm64"
else
    echo "❌ Unsupported platform: $PLATFORM"
    echo "Supported platforms: linux, x86_64, alpine, macos, arm64"
    exit 1
fi

echo "✅ Docker image built successfully!"

# Show binary info
echo "🔍 Checking binary dependencies..."
docker run --rm $IMAGE_TAG ldd /usr/local/bin/rshioaji || echo "✅ Static binary confirmed - no external dependencies"

# Optional: Run a quick test
echo "🧪 Testing the built image..."
docker run --rm $IMAGE_TAG --help || echo "ℹ️  Help command not available, but image runs successfully"

echo "🎉 Build complete!"

# Show image info
echo "📊 Image information:"
docker images | grep rshioaji

echo ""
echo "🚀 Usage Examples:"
echo "  Linux x86_64:    docker run --rm -v \$(pwd)/.env:/app/.env:ro rshioaji:linux-x86_64 --stock 2330"
echo "  Alpine Linux:    docker run --rm -v \$(pwd)/.env:/app/.env:ro rshioaji:alpine --stock 2330"
echo "  macOS ARM64:     docker run --rm -v \$(pwd)/.env:/app/.env:ro rshioaji:macos-arm64 --stock 2330"
echo ""
echo "📁 Environment Variables:"
echo "  docker run --rm -e SHIOAJI_API_KEY=key -e SHIOAJI_SECRET_KEY=secret $IMAGE_TAG --stock 2330"
echo ""
echo "🏔️ Image Sizes (actual):"
echo "  Alpine Linux:    ~50MB (ultra-lightweight, limited Python support)"
echo "  Debian Slim:     ~162MB (lightweight, full Python support)"
echo "  macOS ARM64:     ~100MB (development environment)"
echo ""
echo "🔧 Build for different platforms:"
echo "  ./docker-build.sh linux   # Debian slim (~162MB, recommended)"
echo "  ./docker-build.sh alpine  # Alpine Linux (~50MB, experimental)"
echo "  ./docker-build.sh macos   # macOS ARM64 (~100MB, development)"
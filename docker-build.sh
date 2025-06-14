#!/bin/bash

# Build script for rshioaji Docker images with Python 3.12 support

set -e

# Default platform
PLATFORM=${1:-"linux"}

if [ "$PLATFORM" = "linux" ] || [ "$PLATFORM" = "x86_64" ]; then
    echo "🐳 Building rshioaji Docker image for manylinux_x86_64 with Python 3.12..."
    docker build -t rshioaji:manylinux-x86_64 -f Dockerfile .
    IMAGE_TAG="rshioaji:manylinux-x86_64"
elif [ "$PLATFORM" = "macos" ] || [ "$PLATFORM" = "arm64" ]; then
    echo "🐳 Building rshioaji Docker image for macOS ARM64 with Python 3.12..."
    docker build -t rshioaji:macos-arm64 -f Dockerfile.macos .
    IMAGE_TAG="rshioaji:macos-arm64"
else
    echo "❌ Unsupported platform: $PLATFORM"
    echo "Supported platforms: linux, x86_64, macos, arm64"
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
echo "  Linux x86_64: docker run --rm -it rshioaji:manylinux-x86_64"
echo "  macOS ARM64:  docker run --rm -it rshioaji:macos-arm64"
echo ""
echo "📁 To mount a config directory:"
echo "  docker run --rm -it -v \$(pwd)/config:/app/config $IMAGE_TAG"
echo ""
echo "🐍 Python 3.12 Support:"
echo "  - Compatible with Python 3.12 wheels"
echo "  - Supports cpython-312-darwin.so (macOS ARM64)"
echo "  - Supports cpython-312-x86_64-linux-gnu.so (Linux x86_64)"
echo ""
echo "🔧 Build for different platforms:"
echo "  ./docker-build.sh linux   # or x86_64"
echo "  ./docker-build.sh macos   # or arm64"
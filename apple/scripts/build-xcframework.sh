#!/bin/bash
# Build FractaFFI.xcframework for Apple platforms
#
# Prerequisites:
#   - Rust with targets: rustup target add aarch64-apple-ios aarch64-apple-ios-sim aarch64-apple-darwin
#   - Xcode with iOS/macOS SDK installed
#
# Usage:
#   ./apple/scripts/build-xcframework.sh          # Build all platforms (iOS + macOS)
#   ./apple/scripts/build-xcframework.sh ios      # Build iOS only
#   ./apple/scripts/build-xcframework.sh macos    # Build macOS only

set -e

# Load Rust environment (cargo may not be in PATH when run from Xcode/Finder)
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# Verify cargo is available
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo not found. Please install Rust: https://rustup.rs"
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
APPLE_DIR="$PROJECT_ROOT/apple"

# Parse argument
BUILD_TARGET="${1:-all}"

# Find Xcode
if [ -d "/Applications/Xcode.app" ]; then
    XCODE_PATH="/Applications/Xcode.app"
elif [ -d "/Applications/Xcode-beta.app" ]; then
    XCODE_PATH="/Applications/Xcode-beta.app"
else
    echo "Error: Xcode not found in /Applications"
    exit 1
fi

DEVELOPER_DIR="$XCODE_PATH/Contents/Developer"
XCODEBUILD="$DEVELOPER_DIR/usr/bin/xcodebuild"

echo "Using Xcode: $XCODE_PATH"
echo "Project root: $PROJECT_ROOT"
echo "Build target: $BUILD_TARGET"

# Set up SDK paths
IOS_SDK="$DEVELOPER_DIR/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk"
SIM_SDK="$DEVELOPER_DIR/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk"
MACOS_SDK="$DEVELOPER_DIR/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk"

# Create output directory
FRAMEWORKS_DIR="$APPLE_DIR/Frameworks"
mkdir -p "$FRAMEWORKS_DIR"

# Headers directory
HEADERS_DIR="$APPLE_DIR/Fracta.swiftpm/Sources/Fracta/Generated"

# Track which libraries to include
XCFRAMEWORK_ARGS=()

# Build for macOS (ARM64 - Apple Silicon)
if [ "$BUILD_TARGET" = "all" ] || [ "$BUILD_TARGET" = "macos" ]; then
    echo ""
    echo "=== Building for macOS (aarch64-apple-darwin) ==="
    SDKROOT="$MACOS_SDK" \
    MACOSX_DEPLOYMENT_TARGET=15.0 \
    cargo build -p fracta-ffi --release --target aarch64-apple-darwin

    # Create macOS directory structure
    MACOS_OUT="$FRAMEWORKS_DIR/macos-arm64"
    mkdir -p "$MACOS_OUT"
    cp "$PROJECT_ROOT/target/aarch64-apple-darwin/release/libfracta_ffi.a" "$MACOS_OUT/"

    XCFRAMEWORK_ARGS+=(-library "$PROJECT_ROOT/target/aarch64-apple-darwin/release/libfracta_ffi.a" -headers "$HEADERS_DIR")
fi

# Build for iOS device (ARM64)
if [ "$BUILD_TARGET" = "all" ] || [ "$BUILD_TARGET" = "ios" ]; then
    echo ""
    echo "=== Building for iOS device (aarch64-apple-ios) ==="
    DEVELOPER_DIR="$DEVELOPER_DIR" \
    SDKROOT="$IOS_SDK" \
    IPHONEOS_DEPLOYMENT_TARGET=18.0 \
    cargo build -p fracta-ffi --release --target aarch64-apple-ios

    XCFRAMEWORK_ARGS+=(-library "$PROJECT_ROOT/target/aarch64-apple-ios/release/libfracta_ffi.a" -headers "$HEADERS_DIR")

    echo ""
    echo "=== Building for iOS Simulator (aarch64-apple-ios-sim) ==="
    DEVELOPER_DIR="$DEVELOPER_DIR" \
    SDKROOT="$SIM_SDK" \
    IPHONEOS_DEPLOYMENT_TARGET=18.0 \
    cargo build -p fracta-ffi --release --target aarch64-apple-ios-sim

    XCFRAMEWORK_ARGS+=(-library "$PROJECT_ROOT/target/aarch64-apple-ios-sim/release/libfracta_ffi.a" -headers "$HEADERS_DIR")
fi

# Create XCFramework
echo ""
echo "=== Creating XCFramework ==="
rm -rf "$FRAMEWORKS_DIR/FractaFFI.xcframework"
"$XCODEBUILD" -create-xcframework \
    "${XCFRAMEWORK_ARGS[@]}" \
    -output "$FRAMEWORKS_DIR/FractaFFI.xcframework"

echo ""
echo "=== Done! ==="
echo "XCFramework created at: $FRAMEWORKS_DIR/FractaFFI.xcframework"
ls -la "$FRAMEWORKS_DIR/FractaFFI.xcframework"

#!/bin/bash
# Build FractaFFI.xcframework for iOS
#
# Prerequisites:
#   - Rust with iOS targets: rustup target add aarch64-apple-ios aarch64-apple-ios-sim
#   - Xcode with iOS SDK installed
#
# Usage:
#   ./apple/scripts/build-xcframework.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
APPLE_DIR="$PROJECT_ROOT/apple"

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

# Set up iOS SDK paths
IOS_SDK="$DEVELOPER_DIR/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk"
SIM_SDK="$DEVELOPER_DIR/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk"

# Build for iOS device (ARM64)
echo ""
echo "=== Building for iOS device (aarch64-apple-ios) ==="
DEVELOPER_DIR="$DEVELOPER_DIR" \
SDKROOT="$IOS_SDK" \
IPHONEOS_DEPLOYMENT_TARGET=18.0 \
cargo build -p fracta-ffi --release --target aarch64-apple-ios

# Build for iOS Simulator (ARM64 - Apple Silicon)
echo ""
echo "=== Building for iOS Simulator (aarch64-apple-ios-sim) ==="
DEVELOPER_DIR="$DEVELOPER_DIR" \
SDKROOT="$SIM_SDK" \
IPHONEOS_DEPLOYMENT_TARGET=18.0 \
cargo build -p fracta-ffi --release --target aarch64-apple-ios-sim

# Create output directory
FRAMEWORKS_DIR="$APPLE_DIR/Frameworks"
rm -rf "$FRAMEWORKS_DIR/FractaFFI.xcframework"
mkdir -p "$FRAMEWORKS_DIR"

# Headers directory
HEADERS_DIR="$APPLE_DIR/Fracta.swiftpm/Sources/Fracta/Generated"

# Create XCFramework
echo ""
echo "=== Creating XCFramework ==="
"$XCODEBUILD" -create-xcframework \
    -library "$PROJECT_ROOT/target/aarch64-apple-ios/release/libfracta_ffi.a" \
    -headers "$HEADERS_DIR" \
    -library "$PROJECT_ROOT/target/aarch64-apple-ios-sim/release/libfracta_ffi.a" \
    -headers "$HEADERS_DIR" \
    -output "$FRAMEWORKS_DIR/FractaFFI.xcframework"

echo ""
echo "=== Done! ==="
echo "XCFramework created at: $FRAMEWORKS_DIR/FractaFFI.xcframework"
ls -la "$FRAMEWORKS_DIR/FractaFFI.xcframework"

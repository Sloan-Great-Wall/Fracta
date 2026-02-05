# Fracta Apple Platforms

SwiftUI application for macOS, iOS, iPadOS, and visionOS.

## Architecture

```
Fracta.swiftpm/
├── Package.swift          # Swift Package definition
└── Sources/Fracta/
    ├── App/
    │   └── FractaApp.swift       # App entry point, scenes
    ├── Core/
    │   ├── Models.swift          # Data models (FileItem, DocumentContent, etc.)
    │   └── FractaBridge.swift    # Rust FFI bridge wrapper
    ├── Views/
    │   ├── ContentView.swift     # Main navigation structure
    │   ├── DocumentPreviewView.swift  # Markdown preview
    │   └── PlatformSpecific.swift     # Platform adaptations
    ├── Components/
    │   └── FileRowView.swift     # File list row component
    ├── Styles/
    │   └── GlassStyle.swift      # Liquid Glass design system
    └── Extensions/
        └── GameControllerSupport.swift  # MFi/Xbox/DualShock support
```

## Design System

### Liquid Glass
Inspired by iOS 26/macOS 26's new design language:
- Translucent materials (`.ultraThinMaterial`)
- Soft shadows with depth
- Smooth spring animations
- Focus-friendly large touch targets

### Game Controller Support
Full D-pad/joystick navigation:
- Arrow keys / D-pad: Navigate between items
- Enter / A button: Select
- Escape / B button: Back
- Shoulder buttons: Switch sections

## Building

### Prerequisites
- Xcode 16+ with Swift 6
- macOS 15+ for development
- iOS 18+ / iPadOS 18+ / visionOS 2+ for deployment
- Rust with iOS targets (for iOS builds)

### Generate FFI Bindings
Before building with real Rust backend:

```bash
# From project root
cargo build -p fracta-ffi

# Generate Swift bindings
cargo run -p fracta-ffi --bin uniffi-bindgen generate \
  --library target/debug/libfracta_ffi.dylib \
  --language swift \
  --out-dir apple/Fracta.swiftpm/Sources/Fracta/Generated
```

### Build iOS XCFramework

The Rust core needs to be compiled as a static library for iOS. The build script handles this automatically.

**1. Install iOS Rust targets:**

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
```

**2. Run the build script:**

```bash
./apple/scripts/build-xcframework.sh
```

This will:
- Build `libfracta_ffi.a` for iOS device (ARM64)
- Build `libfracta_ffi.a` for iOS Simulator (ARM64 Apple Silicon)
- Create `FractaFFI.xcframework` in `apple/Frameworks/`

**3. XCFramework structure:**

```
apple/Frameworks/FractaFFI.xcframework/
├── ios-arm64/                    # Device (iPhone/iPad)
│   ├── libfracta_ffi.a
│   └── Headers/
│       └── fracta_ffiFFI.h
├── ios-arm64-simulator/          # Simulator (Apple Silicon Mac)
│   ├── libfracta_ffi.a
│   └── Headers/
│       └── fracta_ffiFFI.h
└── Info.plist
```

> **Note:** The `apple/Frameworks/` directory is in `.gitignore` because the XCFramework is ~120MB.
> Always regenerate it locally using the build script.

### Open in Xcode
1. Open `Fracta.swiftpm` in Xcode
2. Select target platform (My Mac, iPhone, iPad, Apple Vision Pro)
3. Build and run (⌘R)

### Swift Playgrounds
The `.swiftpm` format is compatible with Swift Playgrounds on iPad.

## Platform Differences

| Feature | macOS | iOS | iPadOS | visionOS |
|---------|-------|-----|--------|----------|
| Window management | Native windows | - | Split View | Volumes |
| Navigation | 3-column | Stack | Sidebar + Detail | Spatial |
| Menu bar | Yes | - | - | - |
| Game controller | Yes | Yes | Yes | Yes |
| Touch | - | Yes | Yes | - |
| Eye tracking | - | - | - | Yes |

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| ⌘F | Search |
| ⌘N | New file |
| ⌘O | Open location |
| ⌘, | Settings |
| ⌘↑/↓ | Navigate files |
| ⌘Enter | Open selected |
| Esc | Cancel / Back |

## Development Notes

### Mock Mode
In DEBUG builds, the app uses mock data and a mock FFI bridge.
This allows UI development without the Rust backend.

### Adding Real FFI
1. Generate bindings (see above)
2. Add generated files to `Sources/Fracta/Generated/`
3. Remove `#if DEBUG` guards in `FractaBridge.swift`
4. Link against `libfracta_ffi.a` (iOS) or `.dylib` (macOS)

### Testing Controllers
Use the iOS Simulator's "Hardware > Game Controller" menu to simulate MFi controllers.

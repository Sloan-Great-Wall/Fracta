// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "Fracta",
    platforms: [
        .iOS(.v18),
        .macOS(.v15),
        .visionOS(.v2)
    ],
    products: [
        .executable(name: "Fracta", targets: ["Fracta"])
    ],
    targets: [
        // C headers for FFI (module map for Clang importer)
        .target(
            name: "fracta_ffiFFI",
            path: "Sources/FractaFFIHeaders",
            publicHeadersPath: "."
        ),
        // Main application target
        .executableTarget(
            name: "Fracta",
            dependencies: ["fracta_ffiFFI"],
            path: "Sources/Fracta",
            exclude: ["Generated/fracta_ffiFFI.h", "Generated/fracta_ffiFFI.modulemap"],
            linkerSettings: [
                .linkedLibrary("fracta_ffi"),
                .linkedLibrary("sqlite3"),
                .linkedLibrary("c++"),
                // iOS device
                .unsafeFlags(["-L../../Frameworks/FractaFFI.xcframework/ios-arm64"], .when(platforms: [.iOS])),
                // iOS simulator
                .unsafeFlags(["-L../../Frameworks/FractaFFI.xcframework/ios-arm64-simulator"], .when(platforms: [.iOS])),
                // macOS (need to build macOS framework separately)
                .unsafeFlags(["-L../../Frameworks/FractaFFI.xcframework/macos-arm64"], .when(platforms: [.macOS]))
            ]
        )
    ]
)

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
        .library(name: "Fracta", targets: ["Fracta"])
    ],
    targets: [
        // C headers for FFI (module map for Clang importer)
        .target(
            name: "fracta_ffiFFI",
            path: "Sources/FractaFFIHeaders",
            publicHeadersPath: "."
        ),
        // Main application target
        .target(
            name: "Fracta",
            dependencies: ["fracta_ffiFFI"],
            path: "Sources/Fracta",
            exclude: ["Generated/fracta_ffiFFI.h", "Generated/fracta_ffiFFI.modulemap"],
            linkerSettings: [
                .linkedLibrary("fracta_ffi"),
                .linkedLibrary("sqlite3"),
                .linkedLibrary("c++"),
                .unsafeFlags([
                    "-L../../Frameworks/FractaFFI.xcframework/ios-arm64",
                    "-L../../Frameworks/FractaFFI.xcframework/ios-arm64-simulator"
                ])
            ]
        )
    ]
)

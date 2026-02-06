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
        // Binary target for the Rust FFI library (inside the package)
        .binaryTarget(
            name: "FractaFFI",
            path: "Frameworks/FractaFFI.xcframework"
        ),
        // C headers for FFI (module map for Clang importer)
        .target(
            name: "fracta_ffiFFI",
            path: "Sources/FractaFFIHeaders",
            publicHeadersPath: "."
        ),
        // Main application target
        .executableTarget(
            name: "Fracta",
            dependencies: [
                "FractaFFI",
                "fracta_ffiFFI"
            ],
            path: "Sources/Fracta",
            exclude: ["Generated/fracta_ffiFFI.h", "Generated/fracta_ffiFFI.modulemap"],
            linkerSettings: [
                .linkedLibrary("sqlite3"),
                .linkedLibrary("c++")
            ]
        )
    ]
)

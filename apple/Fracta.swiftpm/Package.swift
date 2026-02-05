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
        .target(
            name: "Fracta",
            path: "Sources/Fracta"
        )
    ]
)

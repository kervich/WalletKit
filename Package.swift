// swift-tools-version:5.5

import PackageDescription;

let package = Package(
    name: "WalletKit",
    platforms: [
        .iOS(.v13),
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "WalletKit",
            targets: ["WalletKit"]
        )
    ],
    dependencies: [ ],
    targets: [
        .binaryTarget(name: "RustFramework", path: "./WalletKit/RustFramework.xcframework"),
        .target(
            name: "WalletKit",
            dependencies: [
                .target(name: "RustFramework")
            ],
            path: "./WalletKit/Sources"
        ),
    ]
)

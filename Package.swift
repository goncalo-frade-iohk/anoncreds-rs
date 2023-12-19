// swift-tools-version:5.7
import PackageDescription

let package = Package(
    name: "AnoncredsSwift",
    platforms: [
        .iOS(.v13),
        .macOS(.v12)
    ],
    products: [
        .library(
            name: "AnoncredsSwift",
            type: .dynamic,
            targets: ["AnoncredsSwift"]
        ),
    ],
    targets: [
        .target(
            name: "AnoncredsSwift",
            dependencies: ["anoncreds_wrapperFFI"],
            path: "uniffi/output-frameworks/anoncreds-swift/AnoncredsSwift/Sources/Swift"
        ),
        .target(
            name: "anoncreds_wrapperFFI",
            dependencies: ["libanoncreds"],
            path: "uniffi/output-frameworks/anoncreds-swift/AnoncredsSwift/Sources/C"),
        // LOCAL
//        .binaryTarget(
//            name: "libanoncreds",
//            path: "./uniffi/output-frameworks/anoncreds-swift/libanoncreds.xcframework.zip"
//        )
        // RELEASE
        .binaryTarget(
            name: "libanoncreds",
            url: "https://github.com/input-output-hk/anoncreds-rs/releases/download/0.4.1/libanoncreds.xcframework.zip",
            checksum: "85a67c56a4c6480975c631c122091381c3bfe74d3b3102f77da2cb0d71841873"
        )
    ]
)

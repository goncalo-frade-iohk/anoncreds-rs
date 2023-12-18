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
            url: "https://github.com/input-output-hk/anoncreds-rs/releases/download/0.4.0/libanoncreds.xcframework.zip",
            checksum: "2cbb147f58dd4d59990c1fc01302e63aafd63673b64310946420893f639b1f67"
        )
    ]
)

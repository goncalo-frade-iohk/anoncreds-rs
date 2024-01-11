# AnonCred-KMP

[![Kotlin](https://img.shields.io/badge/kotlin-1.9.22-blue.svg?logo=kotlin)](http://kotlinlang.org)
![badge-license]
![badge-latest-release]
[![semantic-release-kotlin]](https://github.com/semantic-release/semantic-release)

![badge-platform-android]
![badge-platform-jvm]

## Introduction

This is a Kotlin MultiPlatform (KMP) wrapper of a rust library and reference implementation of the [Anoncreds V1.0
specification](https://hyperledger.github.io/anoncreds-spec/).


The AnonCreds (Anonymous Credentials) specification is based on the open source verifiable credential implementation of AnonCreds that has been in use since 2017, initially as part of the Hyperledger Indy open source project and now in the Hyperledger AnonCreds project. The extensive use of AnonCreds around the world has made it a de facto standard for ZKP-based verifiable credentials, and this specification is the formalization of that implementation.

## Library

AnonCred-KMP exposes three main parts: [`issuer`](./anoncred-wrapper-rust/src/issuer/mod.rs),
[`prover`](./anoncred-wrapper-rust/src/prover/mod.rs) and
[`verifier`](./anoncred-wrapper-rust/src/verifier/mod.rs).

The library provides wrapper for the following operations

### Issuer

- Create a [schema](https://hyperledger.github.io/anoncreds-spec/#schema-publisher-publish-schema-object)
- Create a [credential definition](https://hyperledger.github.io/anoncreds-spec/#issuer-create-and-publish-credential-definition-object)
- Create a [revocation registry definition](https://hyperledger.github.io/anoncreds-spec/#issuer-create-and-publish-revocation-registry-objects)
- Create a [revocation status list](https://hyperledger.github.io/anoncreds-spec/#publishing-the-initial-initial-revocation-status-list-object)
- Update a [revocation status list](https://hyperledger.github.io/anoncreds-spec/#publishing-the-initial-initial-revocation-status-list-object)
- Update a [revocation status list](https://hyperledger.github.io/anoncreds-spec/#publishing-the-initial-initial-revocation-status-list-object)'s timestamp
- Create a [credential offer](https://hyperledger.github.io/anoncreds-spec/#credential-offer)
- Create a [credential](https://hyperledger.github.io/anoncreds-spec/#issue-credential)

### Prover / Holder

- Create a [credential request](https://hyperledger.github.io/anoncreds-spec/#credential-request)
- Process an incoming [credential](https://hyperledger.github.io/anoncreds-spec/#receiving-a-credential)
- Create a [presentation](https://hyperledger.github.io/anoncreds-spec/#generate-presentation)
- Create, and update, a revocation state
- Create, and update, a revocation state with a witness

### Verifier

- [Verify a presentation](https://hyperledger.github.io/anoncreds-spec/#verify-presentation)
- generate a nonce

## Requirement

- Rust v1.72.0
- KMP v1.9.22

## Integrating the lib into an existing project

### How to use for JVM/Android app

In `build.gradle.kts` files include the dependency
```kotlin
repositories {
    mavenCentral()
}
```
For dependencies
```kotlin
dependencies {
    implementation("io.iohk.atala.prism.anoncredskmp:anoncreds-kmp:<latest version>")
}
```

### How to use for another KMP (Kotlin Multiplatform) project

#### Using Groovy

In the project `build.gradle`
```groovy
allprojects {
    repositories {
        // along with all the other current existing repos add the following
        mavenCentral()
    }
}
```
In the module `build.gradle`
```groovy
kotlin {
    sourceSets {
        commonMain {
            dependencies {
                // This following is just an example you can import it as per you needs
                implementation 'io.iohk.atala.prism.anoncredskmp:anoncreds-kmp:<latest version>'
            }
        }
    }
}
```

#### Using Kotlin DSL

In the project `build.gradle.kts`
```kotlin
allprojects {
    repositories {
        // along with all the other current existing repos add the following
        mavenCentral()
    }
}
```
```kotlin
kotlin {
    sourceSets {
        val commonMain by getting {
            dependencies {
                // This following is just an example you can import it as per you needs
                implementation("io.iohk.atala.prism.anoncredskmp:anoncreds-kmp:<latest version>")
            }
        }
    }
}
```

### How to use for Scala project

```scala
libraryDependencies += "io.iohk.atala.prism.anoncredskmp" % "anoncreds-kmp-jvm" % "<latest version>"
```

<!-- TAG_VERSION -->
[badge-latest-release]: https://img.shields.io/badge/latest--release-0.4.3-blue.svg?style=flat
[badge-license]: https://img.shields.io/badge/license-Apache%20License%202.0-blue.svg?style=flat
[semantic-release-kotlin]: https://img.shields.io/badge/semantic--release-kotlin-blue?logo=semantic-release

<!-- TAG_PLATFORMS -->
[badge-platform-android]: http://img.shields.io/badge/-android-6EDB8D.svg?style=flat
[badge-platform-ios]: http://img.shields.io/badge/-ios-CDCDCD.svg?style=flat
[badge-platform-jvm]: http://img.shields.io/badge/-jvm-DB413D.svg?style=flat
[badge-platform-js]: http://img.shields.io/badge/-js-F8DB5D.svg?style=flat
[badge-platform-js-node]: https://img.shields.io/badge/-nodejs-68a063.svg?style=flat
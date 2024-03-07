rootProject.name = "anoncred-kmm"
include(":uniffi-kmm")
include(":anoncred-wrapper-rust")
include(":anoncreds-kmp")
include(":testapp")

pluginManagement {
    repositories {
        mavenLocal()
        gradlePluginPortal()
        google()
        mavenCentral()
    }
}

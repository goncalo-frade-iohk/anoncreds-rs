plugins {
    id("org.jlleitschuh.gradle.ktlint") version "11.6.0"
    kotlin("jvm") version "1.9.22"
    id("com.android.library") version "8.1.4" apply false
    id("org.jetbrains.dokka") version "1.9.20"
    id("io.github.gradle-nexus.publish-plugin") version "2.0.0-rc-1"
}

buildscript {
    repositories {
        mavenLocal()
        gradlePluginPortal()
        google()
        mavenCentral()
    }
    dependencies {
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:1.9.22")
        classpath("org.jetbrains.kotlinx:atomicfu-gradle-plugin:0.21.0")
    }
}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

allprojects {
    repositories {
        mavenLocal()
        mavenCentral()
        gradlePluginPortal()
        google()
        maven { url = uri("https://jitpack.io") }
    }
}

subprojects {
    apply(plugin = "org.jlleitschuh.gradle.ktlint")
    ktlint {
        verbose.set(true)
        outputToConsole.set(true)
        filter {
            val generatedCodePath = rootDir
                .resolve("anoncreds-kmp")
                .resolve("build")
                .resolve("generated")

            exclude(
                "$generatedCodePath/*/*",
                "$generatedCodePath/*",
                "$generatedCodePath/**",
                "$generatedCodePath/**/**"
            )
            exclude("**/generated/**")
            exclude { projectDir.toURI().relativize(it.file.toURI()).path.contains("/generated/") }
            exclude { element -> element.file.path.contains("generated/") }
            exclude("${project.layout.buildDirectory.asFile.get()}/generated/")
            exclude { it.file.path.contains(layout.buildDirectory.dir("generated").get().toString()) }
        }
    }
}

nexusPublishing {
    repositories {
        sonatype {
            nexusUrl.set(uri("https://oss.sonatype.org/service/local/"))
            snapshotRepositoryUrl.set(uri("https://oss.sonatype.org/content/repositories/snapshots/"))
            username.set(System.getenv("SONATYPE_USERNAME"))
            password.set(System.getenv("SONATYPE_PASSWORD"))
        }
    }
}

import java.util.Base64

val publishedMavenId: String = "io.iohk.atala.prism.anoncredskmp"

plugins {
    id("org.jlleitschuh.gradle.ktlint") version "11.6.0"
    kotlin("jvm") version "1.9.22"
    id("com.android.library") version "8.1.4" apply false
    id("org.jetbrains.dokka") version "1.9.20"
    id("io.github.gradle-nexus.publish-plugin") version "2.0.0-rc-1"
    id("maven-publish")
    id("signing")
}

group = publishedMavenId
version = "0.4.4"

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
        classpath("org.jetbrains.dokka:dokka-base:1.9.20")
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

    if (this.name == "anoncreds-kmp") {
        apply(plugin = "org.gradle.maven-publish")
        apply(plugin = "org.gradle.signing")
        publishing {
            publications {
                withType<MavenPublication> {
                    groupId = publishedMavenId
                    artifactId = project.name
                    version = project.version.toString()
                    pom {
                        name.set("AnonCred KMP Wrapper")
                        description.set("The AnonCreds (Anonymous Credentials) specification is based on the open source verifiable credential implementation of AnonCreds that has been in use since 2017, initially as part of the Hyperledger Indy open source project and now in the Hyperledger AnonCreds project. The extensive use of AnonCreds around the world has made it a de facto standard for ZKP-based verifiable credentials, and this specification is the formalization of that implementation.")
                        url.set("https://docs.atalaprism.io/")
                        organization {
                            name.set("IOG")
                            url.set("https://iog.io/")
                        }
                        issueManagement {
                            system.set("Github")
                            url.set("https://github.com/input-output-hk/anoncreds-rs")
                        }
                        licenses {
                            license {
                                name.set("The Apache License, Version 2.0")
                                url.set("https://www.apache.org/licenses/LICENSE-2.0.txt")
                            }
                        }
                        developers {
                            developer {
                                id.set("hamada147")
                                name.set("Ahmed Moussa")
                                email.set("ahmed.moussa@iohk.io")
                                organization.set("IOG")
                                roles.add("developer")
                                url.set("https://github.com/hamada147")
                            }
                        }
                        scm {
                            connection.set("scm:git:git://input-output-hk/anoncreds-rs.git")
                            developerConnection.set("scm:git:ssh://input-output-hk/anoncreds-rs.git")
                            url.set("https://github.com/input-output-hk/anoncreds-rs")
                        }
                    }
                    if (System.getenv("BASE64_ARMORED_GPG_SIGNING_KEY_MAVEN") != null) {
                        if (System.getenv("BASE64_ARMORED_GPG_SIGNING_KEY_MAVEN").isNotBlank()) {
                            signing {
                                val base64EncodedAsciiArmoredSigningKey: String =
                                    System.getenv("BASE64_ARMORED_GPG_SIGNING_KEY_MAVEN") ?: ""
                                val signingKeyPassword: String = System.getenv("SIGNING_KEY_PASSWORD") ?: ""
                                useInMemoryPgpKeys(
                                    String(
                                        Base64.getDecoder().decode(base64EncodedAsciiArmoredSigningKey.toByteArray())
                                    ),
                                    signingKeyPassword
                                )
                                sign(this@withType)
                            }
                        }
                    }
                }
            }
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

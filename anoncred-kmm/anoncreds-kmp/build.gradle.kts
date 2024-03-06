import org.gradle.internal.os.OperatingSystem
import org.jetbrains.kotlin.gradle.plugin.mpp.KotlinNativeCompilation
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import java.util.Base64

val os: OperatingSystem = OperatingSystem.current()
val publishedMavenId: String = "io.iohk.atala.prism.anoncredskmp"

plugins {
    id("com.android.library")
    kotlin("multiplatform")
    id("org.jetbrains.dokka")
    id("maven-publish")
    id("signing")
}

apply(plugin = "kotlinx-atomicfu")
version = "0.4.4"
group = publishedMavenId

fun KotlinNativeCompilation.anoncredsCinterops(type: String) {
    cinterops {
        val anoncreds_wrapper by creating {
            val crate = this.name
            packageName("$crate.cinterop")
            header(
                project.layout.buildDirectory.asFile.get()
                    .resolve("nativeInterop")
                    .resolve("cinterop")
                    .resolve("headers")
                    .resolve(crate)
                    .resolve("$crate.h")
            )
            tasks.named(interopProcessingTaskName) {
                dependsOn(":anoncred-wrapper-rust:buildRust")
            }
            when (type) {
                "macosX64" -> {
                    extraOpts(
                        "-libraryPath",
                        rootDir
                            .resolve("anoncred-wrapper-rust")
                            .resolve("target")
                            .resolve("x86_64-apple-darwin")
                            .resolve("release")
                            .absolutePath
                    )
                }

                "macosArm64" -> {
                    extraOpts(
                        "-libraryPath",
                        rootDir
                            .resolve("anoncred-wrapper-rust")
                            .resolve("target")
                            .resolve("aarch64-apple-darwin")
                            .resolve("release")
                            .absolutePath
                    )
                }

                "ios" -> {
                    extraOpts(
                        "-libraryPath",
                        rootDir
                            .resolve("anoncred-wrapper-rust")
                            .resolve("target")
                            .resolve("ios-universal")
                            .resolve("release")
                            .absolutePath
                    )
                }

                else -> {
                    throw GradleException("Unsupported linking")
                }
            }
        }
    }
}

/**
 * The `javadocJar` variable is used to register a `Jar` task to generate a Javadoc JAR file.
 * The Javadoc JAR file is created with the classifier "javadoc" and it includes the HTML documentation generated
 * by the `dokkaHtml` task.
 */
val javadocJar by tasks.registering(Jar::class) {
    archiveClassifier.set("javadoc")
    from(tasks.dokkaHtml)
}

kotlin {
    jvm {
        compilations.all {
            kotlinOptions.jvmTarget = "17"
        }
        testRuns["test"].executionTask.configure {
            useJUnitPlatform()
        }
        withSourcesJar()
        publishing {
            publications {
                withType<MavenPublication> {
                    artifact(javadocJar)
                }
            }
        }
    }
    androidTarget {
        publishAllLibraryVariants()
        compilations.all {
            kotlinOptions.jvmTarget = "17"
        }
    }

    sourceSets {
        val commonMain by getting {
            val generatedDir = project.layout.buildDirectory.asFile.get()
                .resolve("generated")
                .resolve("commonMain")
                .resolve("kotlin")
            kotlin.srcDir(generatedDir)
            dependencies {
                implementation("com.squareup.okio:okio:3.7.0")
                implementation("org.jetbrains.kotlinx:kotlinx-datetime:0.5.0")
            }
        }
        val commonTest by getting {
            dependencies {
                implementation(kotlin("test"))
                implementation("org.jetbrains.kotlinx:kotlinx-coroutines-test:1.8.0")
            }
        }
        val jvmMain by getting {
            val generatedDir = project.layout.buildDirectory.asFile.get()
                .resolve("generated")
                .resolve("jvmMain")
                .resolve("kotlin")
            kotlin.srcDir(generatedDir)
            val generatedResources = project.layout.buildDirectory.asFile.get()
                .resolve("generatedResources")
                .resolve("jvm")
                .resolve("main")
            resources.srcDir(generatedResources)
            dependencies {
                implementation("net.java.dev.jna:jna:5.13.0")
            }
        }
        val jvmTest by getting
        val androidMain by getting {
            val generatedDir = project.layout.buildDirectory.asFile.get()
                .resolve("generated")
                .resolve("androidMain")
                .resolve("kotlin")
            kotlin.srcDir(generatedDir)
            val generatedResources = project.layout.buildDirectory.asFile.get()
                .resolve("generatedResources")
                .resolve("android")
                .resolve("main")
                .resolve("jniLibs")
            resources.srcDir(generatedResources)
            dependencies {
                implementation("net.java.dev.jna:jna:5.13.0@aar")
            }
        }
        val androidUnitTest by getting {
            dependencies {
                implementation("junit:junit:4.13.2")
            }
        }
        all {
            languageSettings {
                optIn("kotlin.RequiresOptIn")
                optIn("kotlinx.cinterop.ExperimentalForeignApi")
            }
        }
    }
}

/**
 * Delete the generated `Target` folder that is being generated by Rust Cargo
 */
val rustClean by tasks.register("rustClean") {
    group = "rust"
    delete(projectDir.resolve("target"))
    dependsOn("clean")
}

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
                        val base64EncodedAsciiArmoredSigningKey: String = System.getenv("BASE64_ARMORED_GPG_SIGNING_KEY_MAVEN") ?: ""
                        val signingKeyPassword: String = System.getenv("SIGNING_KEY_PASSWORD") ?: ""
                        useInMemoryPgpKeys(String(Base64.getDecoder().decode(base64EncodedAsciiArmoredSigningKey.toByteArray())), signingKeyPassword)
                        sign(this@withType)
                    }
                }
            }
        }
    }
    repositories {
        maven {
            this.name = "GitHubPackages"
            this.url = uri("https://maven.pkg.github.com/input-output-hk/anoncreds-rs/")
            credentials {
                this.username = System.getenv("ATALA_GITHUB_ACTOR")
                this.password = System.getenv("ATALA_GITHUB_TOKEN")
            }
        }
    }
}

android {
    ndkVersion = "26.0.10792818"
    compileSdk = 34
    namespace = "io.iohk.atala.prism.anoncredskmp"
    sourceSets["main"].manifest.srcFile("src/androidMain/AndroidManifest.xml")

    sourceSets["main"].jniLibs {
        setSrcDirs(
            listOf(
                project.layout.buildDirectory.asFile.get()
                    .resolve("generatedResources")
                    .resolve("android")
                    .resolve("main")
                    .resolve("jniLibs")
            )
        )
    }
    defaultConfig {
        minSdk = 21
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    /**
     * Because Software Components will not be created automatically for Maven publishing from
     * Android Gradle Plugin 8.0. To opt-in to the future behavior, set the Gradle property android.
     * disableAutomaticComponentCreation=true in the `gradle.properties` file or use the new
     * publishing DSL.
     */
    publishing {
        multipleVariants {
            withSourcesJar()
            withJavadocJar()
            allVariants()
        }
    }
}

afterEvaluate {
    tasks.withType<KotlinCompile> {
        dependsOn(":anoncred-wrapper-rust:buildRust")
    }
    tasks.withType<ProcessResources> {
        dependsOn(":anoncred-wrapper-rust:buildRust")
    }
    tasks.named("packageDebugResources") {
        dependsOn(":anoncred-wrapper-rust:copyBindings")
    }
    tasks.named("packageReleaseResources") {
        dependsOn(":anoncred-wrapper-rust:copyBindings")
    }
}

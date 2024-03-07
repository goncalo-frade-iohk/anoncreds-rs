import com.android.build.gradle.tasks.ProcessAndroidResources
import com.android.build.gradle.tasks.SourceJarTask
import org.gradle.internal.os.OperatingSystem
import org.jetbrains.dokka.gradle.DokkaTask
import org.jetbrains.kotlin.gradle.plugin.mpp.KotlinNativeCompilation
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import java.net.URL

val os: OperatingSystem = OperatingSystem.current()

plugins {
    id("com.android.library")
    kotlin("multiplatform")
    id("org.jetbrains.dokka")
    id("maven-publish")
}

apply(plugin = "kotlinx-atomicfu")
version = rootProject.version
group = rootProject.group

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
    applyDefaultHierarchyTemplate()

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

android {
    ndkVersion = "26.0.10792818"
    compileSdk = 34
    namespace = rootProject.group.toString()
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

// Dokka implementation
tasks.withType<DokkaTask>().configureEach {
    moduleName.set("AnonCred KMP")
    moduleVersion.set(rootProject.version.toString())
    description = "This is a Kotlin Multiplatform Library Wrapper for AnonCred Rust"
    dokkaSourceSets {
        configureEach {
            jdkVersion.set(17)
            languageVersion.set("1.9.22")
            apiVersion.set("2.0")
            sourceLink {
                localDirectory.set(projectDir.resolve("src"))
                remoteUrl.set(URL("https://github.com/input-output-hk/anoncreds-rs/tree/main/src"))
                remoteLineSuffix.set("#L")
            }
            externalDocumentationLink {
                url.set(URL("https://kotlinlang.org/api/latest/jvm/stdlib/"))
            }
            externalDocumentationLink {
                url.set(URL("https://kotlinlang.org/api/kotlinx.serialization/"))
            }
            externalDocumentationLink {
                url.set(URL("https://api.ktor.io/"))
            }
            externalDocumentationLink {
                url.set(URL("https://kotlinlang.org/api/kotlinx-datetime/"))
                packageListUrl.set(URL("https://kotlinlang.org/api/kotlinx-datetime/"))
            }
            externalDocumentationLink {
                url.set(URL("https://kotlinlang.org/api/kotlinx.coroutines/"))
            }
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
    tasks.withType<PublishToMavenRepository> {
        dependsOn(tasks.withType<Sign>(), ":anoncred-wrapper-rust:copyBindings")
    }
    tasks.withType<ProcessAndroidResources> {
        dependsOn(":anoncred-wrapper-rust:copyBindings")
    }
    tasks.withType<SourceJarTask> {
        dependsOn(":anoncred-wrapper-rust:copyBindings")
    }
    tasks.withType<org.gradle.jvm.tasks.Jar> {
        dependsOn(":anoncred-wrapper-rust:copyBindings")
    }
    tasks.named("packageDebugResources") {
        dependsOn(":anoncred-wrapper-rust:copyBindings")
    }
    tasks.named("packageReleaseResources") {
        dependsOn(":anoncred-wrapper-rust:copyBindings")
    }
    tasks.named("extractDeepLinksForAarDebug") {
        dependsOn(":anoncred-wrapper-rust:copyBindings")
    }
    tasks.named("extractDeepLinksForAarRelease") {
        dependsOn(":anoncred-wrapper-rust:copyBindings")
    }
    tasks.named("androidReleaseSourcesJar") {
        dependsOn(":anoncred-wrapper-rust:copyBindings")
    }
    tasks.named("androidDebugSourcesJar") {
        dependsOn(":anoncred-wrapper-rust:copyBindings")
    }
    tasks.named("mergeDebugJniLibFolders") {
        dependsOn(":anoncred-wrapper-rust:copyGeneratedBinaryForAndroid")
    }
    tasks.named("mergeReleaseJniLibFolders") {
        dependsOn(":anoncred-wrapper-rust:copyGeneratedBinaryForAndroid")
    }
    tasks.named("jvmSourcesJar") {
        dependsOn(":anoncred-wrapper-rust:copyBindings")
    }
    tasks.named("sourcesJar") {
        dependsOn(":anoncred-wrapper-rust:copyBindings")
    }
}

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "com.umaai.ramen"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.umaai.ramen"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"
    }

    // cargo-ndk writes native libraries into this directory.
    sourceSets["main"].jniLibs.srcDir("src/main/jniLibs")
    packaging { jniLibs { useLegacyPackaging = false } }
}

kotlin { jvmToolchain(17) }
androidResources { noCompress += "json"; noCompress += "toml" }

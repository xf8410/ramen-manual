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
    sourceSets["main"].jniLibs.srcDir("src/main/jniLibs")
    packaging { jniLibs { useLegacyPackaging = false } }
    androidResources { noCompress += "json"; noCompress += "toml" }
    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }
    }
}

kotlin { jvmToolchain(17) }

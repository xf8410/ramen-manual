# Android 构建流程

Debug：

```bash
rustup target add aarch64-linux-android
cargo install cargo-ndk
cd mobile
./scripts/build-native.sh
gradle assembleDebug
```

Release：

```bash
./scripts/build-native.sh
gradle assembleRelease
```

产物分别位于：

```text
app/build/outputs/apk/debug/app-debug.apk
app/build/outputs/apk/release/app-release.apk
```

Release 开启 R8/资源压缩，但 JNI 入口类通过 `proguard-rules.pro` 保留。当前仍需在真实 Android SDK/NDK 环境执行构建，不能仅凭 Gradle 配置宣称 APK 已完成。

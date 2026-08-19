# Android 构建流程

```bash
rustup target add aarch64-linux-android
cargo install cargo-ndk
cd mobile
./scripts/copy-assets.sh
cargo ndk -t arm64-v8a -o app/src/main/jniLibs build --release --manifest-path core/Cargo.toml
./gradlew assembleDebug
```

若本机没有 Gradle wrapper，需要先用 Android Studio 或 `gradle wrapper` 生成 wrapper；本仓库暂不提交二进制 SDK/NDK。

运行时：

- Kotlin 将 `assets/` 解压到 app 私有目录；
- JNI 设置 Rust 当前工作目录；
- Rust 通过原始 `RamenGame` 加载数据；
- UI 只展示 `UiState` 并提交索引。

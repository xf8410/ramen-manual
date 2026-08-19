# Android 构建流程

```bash
rustup target add aarch64-linux-android
cargo install cargo-ndk
cd mobile
./scripts/build-native.sh
# build-native.sh 会复制拉面杯 assets，并输出 app/src/main/jniLibs/arm64-v8a/libramen_mobile_core.so
./gradlew assembleDebug
```

当前 Android app 只声明并打包 ARM64；没有 native `.so` 时应用会在启动时报告 native library 加载失败，不能把纯 Kotlin APK 当作完成品。

运行时：

- `AssetInstaller` 解压完整拉面杯数据；
- JNI 设置 Rust 当前工作目录；
- Rust 通过 `load_game_config()` + `init_global_with_config()` 初始化；
- Rust 创建原始 `RamenGame`；
- UI 只展示 `UiState` 并提交候选索引。

注意：仓库根目录的 `default_config.toml` 是温泉配置，构建脚本使用 `mobile/assets/game_config.toml`，不能直接复制根目录配置为手机版 `game_config.toml`。

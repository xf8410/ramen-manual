# Android 构建流程

本分支提供 `.github/workflows/mobile-android.yml`。可在 GitHub Actions 中手动触发 `mobile-android`，或修改 mobile 文件后由 Pull Request 自动触发。

本地构建：

```bash
rustup target add aarch64-linux-android
cargo install cargo-ndk
gradle --version
cd mobile
./scripts/build-native.sh
gradle assembleDebug
```

`build-native.sh` 会复制拉面杯 assets，并输出 `app/src/main/jniLibs/arm64-v8a/libramen_mobile_core.so`。`gradle assembleDebug` 会将它打包进 APK。

运行时：`AssetInstaller` 解压完整拉面杯数据；JNI 设置 Rust 当前工作目录；Rust 使用 `load_game_config()` + `init_global_with_config()` 初始化；UI 只展示 `UiState` 并提交候选索引。

注意：仓库根目录的 `default_config.toml` 是温泉配置，构建脚本使用 `mobile/assets/game_config.toml`，不能直接复制根目录配置为手机版 `game_config.toml`。

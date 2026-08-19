# 拉面杯手机版

这是拉面杯手动模拟器的 Android 触屏前端工作目录。

## 当前链路

```text
Android assets
  → AssetInstaller
  → nativeSetDataRoot
  → RamenGameAdapter
  → RamenGameDriver
  → TouchSession / UiState JSON
  → Android buttons
```

- 只支持拉面杯剧本；
- 直接复用 `xulai1001/umaai-rs` 中的 `RamenGame`；
- 触屏按钮替代 Windows `ramen_manual.exe` 的 `ManualTrainer/inquire`；
- 不使用 EXE、Wine 或 Winlator；
- 不包含温泉剧本；
- Native bridge 已接入，但 Rust Android target 和 APK 构建仍需验证。

## 构建前准备

1. 在仓库根目录执行 `mobile/scripts/copy-assets.sh`；
2. 安装 Android SDK/NDK、Rust Android targets 和 cargo-ndk；
3. 编译 `ramen-mobile-core` 为 `aarch64-linux-android`；
4. 将 `libramen_mobile_core.so` 放入 `mobile/app/src/main/jniLibs/arm64-v8a/`；
5. 在 `mobile/` 执行 Gradle assemble。

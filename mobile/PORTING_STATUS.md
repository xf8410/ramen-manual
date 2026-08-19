# 手机版改造状态

本轮继续完成：

- Gradle 配置 `src/main/jniLibs`，可以打包 cargo-ndk 生成的 `.so`；
- 增加 `mobile/scripts/build-native.sh`；
- Android assets 安装后逐项验证文件存在且非空；
- 构建文档明确没有 `.so` 时不能称为 APK 完成；
- 继续使用拉面专用 `game_config.toml`，避免根目录温泉配置误用。

未完成：

- 在真实 Android SDK/NDK 环境执行 `cargo-ndk` 编译；
- 解决上游依赖在 Android target 上的实际编译兼容性；
- 生成并安装 APK 做真机测试；
- 固定种子回归对照。
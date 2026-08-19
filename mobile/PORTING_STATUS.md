# 手机版改造状态

本轮继续完成：

- 添加 GitHub Actions Android 构建工作流；
- 工作流会安装 Android SDK/NDK、Rust Android target 和 cargo-ndk；
- 自动执行 assets 复制、Rust ARM64 native 编译和 Debug APK 构建；
- 上传 Debug APK artifact；
- 忽略本地生成的 target、Gradle 和 JNI 二进制目录。

仍未完成：

- 实际触发并观察 CI 结果；
- 根据真实编译日志修复上游依赖兼容问题；
- 安装 APK 做真机验证；
- 与 Windows 固定种子回归对照。

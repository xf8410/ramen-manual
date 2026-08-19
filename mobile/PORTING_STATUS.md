# 手机版改造状态

本轮继续完成：

- Rust crate 增加 `cdylib` 输出和 `jni`/`serde_json` 依赖；
- JNI `nativeSetDataRoot`、`nativeStart`、`nativeSubmit`、`nativeReset`；
- Android Kotlin 页面已经调用 JNI，而不是 FakeDriver；
- Android 启动时解压拉面杯 assets，并设置 Rust 工作目录；
- 添加 ARM64 构建说明和 Rust target 配置；
- 修复真实适配器的候选保存、阶段推进、PC 配置初始化和结算 PT 映射。

仍未完成：

- 实际安装 NDK/SDK 后的编译验证；
- cargo/Gradle wrapper 与 JNI 动态库产物；
- 真机测试；
- 固定种子与 Windows 版的回归差异检查。

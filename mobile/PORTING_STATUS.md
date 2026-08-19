# 手机版改造状态

本轮继续完成：

- JNI native session 使用全局 Mutex，支持 Android 回调线程切换；
- RamenGameAdapter 的候选和事件状态使用 Arc/Mutex；
- 增加 release R8/资源压缩配置并保留 JNI 类；
- Android Manifest 增加明确的备份和 RTL 配置；
- Debug/Release APK 输出路径和构建命令已记录。

当前真实状态：

- Rust/Android 代码已写到 JNI 和 Gradle 接入层；
- 尚无 GitHub Actions run 结果；
- 尚未在实际 SDK/NDK 环境编译；
- 尚未安装 APK 真机测试；
- 尚未完成固定种子 PC 回归。

不能把配置文件存在等同于 APK 构建成功，后续继续以真实编译日志为准。
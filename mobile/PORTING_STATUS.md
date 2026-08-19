# 手机版改造状态

本轮继续修复：

- JNI 会话由线程局部存储改为 `OnceLock<Mutex<...>>`，避免 Android UI 回调跨线程后丢失游戏会话；
- JNI 数据目录同样使用互斥保护；
- 事件暂停时保留完整 `EventData` 和候选组，提交索引后调用上游 `RamenGame::apply_event`；
- 触屏 UI 仍只提交索引，不复制事件或拉面规则。

仍未完成：

- 真实 CI/Android 编译验证；
- 上游 crate 的 Android 依赖裁剪；
- APK 安装与完整 77 回合触屏流程验证；
- PC 固定种子回归对照。
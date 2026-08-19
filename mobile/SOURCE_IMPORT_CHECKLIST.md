# 源码导入检查表

- [x] 添加上游 `umasim` Git 依赖
- [x] 迁移 PC 入口的拉面杯配置和 `RamenGame::newgame`
- [x] 新增真实 `RamenGameAdapter`
- [x] 新增非阻塞 `PendingDecision` 状态
- [x] 新增 `advance_until_decision()`
- [x] 新增 `submit(index)` 校验
- [x] 移除 Android 路径对 `inquire` 的依赖
- [ ] 将上游数据文件打包到 Android assets 并设置工作目录
- [ ] 添加 Android ARM64 构建配置
- [ ] 加入触屏 UI
- [ ] 构建并测试 APK
- [ ] 用 Windows 版结果做回归对照

当前核心已不再是 FakeDriver，而是接到上游真实 `RamenGame` 的适配器。
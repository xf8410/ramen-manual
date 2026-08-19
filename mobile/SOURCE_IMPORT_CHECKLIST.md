# 源码导入检查表

- [ ] 导入上游 `Cargo.toml` 和 `crates/umasim` 必要源码
- [ ] 导入拉面杯 `game/ramen` 全部模块
- [ ] 导入必要的 `gamedata` 和基础游戏模块
- [ ] 将 Windows/终端二进制入口与 Android 入口分离
- [ ] 新增非阻塞 `PendingDecision` 状态
- [ ] 新增 `advance_until_decision()`
- [ ] 新增 `submit(index)` 校验
- [ ] 移除 Android 路径对 `inquire` 的依赖
- [ ] 添加 Android ARM64 构建配置
- [ ] 加入触屏 UI
- [ ] 构建并测试 APK
- [ ] 用 Windows 版结果做回归对照

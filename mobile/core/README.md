# ramen-mobile-core

这是 Android UI 与上游 `umasim::RamenGame` 之间的适配边界。

当前已实现：

- 可序列化的待决策状态；
- 拉面、隐藏风味、训练和事件四类决策；
- 触屏提交索引校验；
- `advance_until_decision` / `submit` 生命周期；
- 不依赖 `inquire`、stdin、stdout 或 Windows。

下一步是在上游源码导入后实现 `RamenDriver`，由它驱动真正的 `RamenGame`。规则仍然只来自上游 `game/ramen`，本 crate 不复制规则。

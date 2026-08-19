# Android 接入进行中

当前不是把 PC 程序重写成另一套规则，而是把 `ramen_manual.rs` 的交互层替换掉：

```text
RamenGame::newgame(UMA_ID, DECK, INHERIT)
        ↓
RamenGameAdapter
        ↓
RamenGamePort::pump / choose
        ↓
TouchSession
        ↓
UiState(JSON)
        ↓
Android 触屏
```

## 接下来必须完成的真实代码

1. 将 PC 入口中的 `UMA_ID / DECK / INHERIT / SEED` 提取为手机版配置；
2. 在 Rust 侧创建真实 `RamenGame` 和 `StdRng`；
3. 驱动 `RamenGame::run_stage` 与 `Game::next`，但在 `Trainer` 请求选择时暂停；
4. 将 `RamenAction`、地区选择和 `EventChoice` 转换为 `PendingDecision`；
5. 将 Android 提交的索引应用回原始候选数组；
6. 游戏结束后返回 `RamenGame` 的原始结算字段；
7. 再接 Android JNI/UniFFI/Flutter 其中一个前端，不提前锁死 UI 技术。

`OnsenGame`、温泉数据和温泉规则不进入此链路。

# 手机版改造状态

## 已开始的实际改造

当前分支已经把 PC 拉面杯程序的交互替换点写成独立 Rust 端口：

- `mobile/core/src/ramen_driver.rs`
  - `RamenGamePort`：真实 `RamenGame` 的最小适配接口；
  - `RamenGameDriver`：将拉面杯模拟器分段推进到触屏决策；
  - `action_options()`：直接使用 `RamenAction` 的显示文本生成 UI 候选项；
  - 不复制拉面杯规则。
- `mobile/core/src/lib.rs`
  - 将端口导出给 Android 前端；
  - 保持 `TouchSession` 的触屏状态机。

## 尚未完成

`RamenGamePort` 还需要在导入 PC 源码后由真实 `RamenGame` 包装器实现。下一步要把 `ramen_manual.rs` 的初始化配置和 `RamenGame::newgame()` 接入该包装器，并把 `RamenStage`、`RamenAction`、`EventData` 转换为 `PendingDecision`。

在这一步之前不能声称已经生成 APK 或已经完成真实游戏流程。

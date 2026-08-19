# 手机版改造状态

本分支仍在持续改造中，未视为完成。

## 已完成

- `RamenGamePort` / `RamenGameDriver` 交互替换点；
- `TouchSession` 触屏暂停、提交、继续状态机；
- `UiState`、`UiMessage`、`SubmitMessage` JSON 协议；
- 拉面杯专属决策类型已包含：拉面、隐藏风味、训练、地区、超级拉面、事件。

## 当前工作

正在把 PC 入口 `ramen_manual.rs` 的真实初始化和 `RamenGame` 逐阶段驱动接入 `RamenGamePort`。在真实适配器完成前，不能声称已经生成可运行 APK。

## 原则

只改 PC 拉面杯程序的交互/平台层；保留 `RamenGame` 和 `game/ramen` 机制；不接入 `OnsenGame`。

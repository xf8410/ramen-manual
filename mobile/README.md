# 拉面杯手机版

这是拉面杯手动模拟器的 Android 触屏前端工作目录。

## 目标

- 只支持拉面杯剧本；
- 直接复用 `xulai1001/umaai-rs` 的 Rust 模拟器核心；
- 用触屏按钮替代 `inquire` 终端菜单；
- 不使用 EXE、Wine 或 Winlator；
- 不包含温泉剧本；
- 不连接真实游戏。

## 源码对应关系

- 手动入口：`crates/umasim/src/bin/ramen_manual.rs`
- 拉面杯逻辑：`crates/umasim/src/game/ramen/`
- 数据：`gamedata/scenario_ramen.json` 及相关 JSON

## 当前状态

Android 工程和 Rust-Android 桥接尚未加入。本目录先固定手机版边界和资源布局；下一步是在不复制/重写拉面杯规则的前提下，把 `ManualTrainer` 的同步终端选择改成可暂停、可恢复的触屏决策接口。

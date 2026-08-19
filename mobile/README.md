# 拉面杯手机版

这是拉面杯手动模拟器的 Android 触屏前端工作目录。

## 目标

- 只支持拉面杯剧本；
- 直接复用 `xulai1001/umaai-rs` 中 `RamenGame` 的 Rust 模拟器核心；
- 用触屏按钮替代 Windows 版 `ramen_manual.exe` 使用的 `ManualTrainer`/`inquire` 终端菜单；
- 不使用 EXE、Wine 或 Winlator；
- 不包含温泉剧本；
- 不连接真实游戏。

## Windows 版本的准确定位

仓库中的 `ramen_manual.exe` **不是温泉杯程序**。它是一个 Windows 命令行前端，创建并运行：

```rust
let mut game = RamenGame::newgame(...)?;
game.run_full_game(&trainer, &mut rng)?;
```

它复用的是通用模拟器框架（`Game`、`Trainer`、基础训练/事件设施等），但实际剧本规则来自 `RamenGame`，也就是拉面杯专属实现：

```text
crates/umasim/src/game/ramen/
```

温泉杯是另一套独立类型和规则：

```text
crates/umasim/src/game/onsen/
OnsenGame
```

因此手机版应当复用 `RamenGame`，而不是 `OnsenGame`。现有 EXE 和手机版的关系是：

```text
Windows: ramen_manual.exe → ManualTrainer/inquire → RamenGame
Android: 触屏 UI → RamenDriver → RamenGame
```

## 源码对应关系

- Windows 手动入口：`crates/umasim/src/bin/ramen_manual.rs`
- 手机版复用的剧本核心：`crates/umasim/src/game/ramen/`
- 通用模拟器接口：`crates/umasim/src/game/traits.rs`
- 温泉杯代码不接入手机版：`crates/umasim/src/game/onsen/`
- 数据：`scenario_ramen.json` 及相关 JSON

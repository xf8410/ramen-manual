# 上游源码导入范围

上游仓库：`xulai1001/umaai-rs`

## 必须导入/复用

- `crates/umasim/src/game/ramen/` 全部拉面杯模块；
- 拉面杯所依赖的通用基础游戏、训练、事件和数据加载模块；
- `crates/umasim/src/game/traits.rs` 中的通用接口；
- `ramen_manual.rs` 中的配置和 `RamenGame::newgame` 初始化思路，但不直接使用其终端入口。

## 不作为手机版剧本核心

- `crates/umasim/src/game/onsen/`；
- `OnsenGame`；
- 温泉数据和温泉选择逻辑。

## 关键结论

现有 `ramen_manual.exe` 的正确调用链是：

```text
ramen_manual.exe
  → ManualTrainer
  → RamenGame
  → RamenGame::run_full_game
```

所以手机版不是“从温泉杯模拟器改成拉面杯”，而是“复用已经正确运行拉面杯的 `RamenGame`，把 Windows 终端输入改成 Android 触屏输入”。

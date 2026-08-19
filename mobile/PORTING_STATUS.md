# 手机版改造状态

本轮继续修复真实适配器：

- 待决策候选在 `pump()` 返回后保持到 `choose()`，不会提前丢失；
- 动作提交后推进一次上游 `Game::next()`，避免重复运行当前阶段；
- 结算 PT 使用原 PC 入口的 `game.uma.total_pt()`，不是尚未结算/已清零的 `ramen.scenario_pt`；
- 初始化改为加载 `game_config.toml` 并调用 `init_global_with_config`，与 PC `ramen_manual.rs` 一致；
- `RamenMobileConfig` 改为可 Clone，避免所有权错误。

仍在继续：事件暂停恢复的阶段级验证、Android assets 解压、ARM64 工程、编译和回归测试。
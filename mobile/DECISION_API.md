# 触屏决策接口

这是源码导入时必须实现的边界。核心模拟器不能直接调用终端输入，也不能在 Android UI 线程中阻塞。

## 状态模型

```rust
pub enum PendingDecision {
    Action {
        turn: u32,
        stage: String,
        options: Vec<DecisionOption>,
    },
    EventChoice {
        turn: u32,
        options: Vec<DecisionOption>,
    },
    Finished {
        summary: GameSummary,
    },
}

pub struct DecisionOption {
    pub index: usize,
    pub title: String,
    pub detail: String,
}
```

## 生命周期

```text
new_game()
  → advance_until_decision()
  → PendingDecision
  → submit(index)
  → advance_until_decision()
  → ...
  → Finished
```

`advance_until_decision()` 负责自动执行不需要玩家输入的阶段；遇到拉面选择、隐藏风味、训练/比赛或事件选项时立即返回。`submit(index)` 只接受当前阶段的合法索引。

## 重要约束

- 不在 UI 线程运行长时间同步循环；
- 不把终端输出解析成按钮；
- 不修改拉面杯数值规则来适配 UI；
- 所有候选项由 Rust 核心生成，UI 只负责展示和提交索引；
- 状态和候选项应支持 JSON 序列化，便于 Android 前端接入。

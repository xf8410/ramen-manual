# 手机版技术边界

## 剧本身份

`ramen_manual.exe` 是拉面杯的 Windows 手动前端，不是温泉杯。它复用通用模拟器框架，但通过 `RamenGame` 执行拉面杯机制。手机版应保持这一结构：只替换输入/展示层，不替换剧本核心。

```text
通用 Game/Trainer 抽象
        ↓
RamenGame + game/ramen/*  ← 手机版必须接入这一层
        ↓
RamenDriver
        ↓
Android 触屏 UI
```

`OnsenGame + game/onsen/*` 是另一套剧本实现，不能因为两者共用基础模拟器设施，就把温泉逻辑混入手机版。

## 保留

`RamenGame` 中的拉面杯状态、行动计算、事件处理、随机数和结算逻辑全部保留，包括：

- `RamenStage::RamenSelect`；
- `RamenStage::SpecialSelect`；
- `RamenStage::Train`；
- 地区选择；
- RMJ 结算；
- 超级拉面阶段；
- 拉面库存、诀窍和隐藏风味。

## 替换

当前 `ManualTrainer` 在 `select_action` / `select_choice` 中调用 `inquire::Select`，手机版不能阻塞等待终端输入。手机版适配层应提供：

1. 返回当前 `RamenGame` 待决策状态和候选项；
2. 暂停模拟；
3. 接收触屏返回的候选索引；
4. 继续执行原有 `RamenGame` 流程。

规则代码不复制到 UI，也不改成温泉杯规则。

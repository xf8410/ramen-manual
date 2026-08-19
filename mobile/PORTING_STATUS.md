# 手机版改造状态

## 已完成

- 使用 PC `ramen_manual.rs` 的真实配置创建 `RamenGame`；
- `RamenGameAdapter` 已接入上游 `RamenGame`；
- `PauseTrainer` 在拉面、隐藏风味、训练、地区、超级拉面和事件选择处暂停；
- Android 通过候选索引继续执行原始动作/事件；
- 保留上游 `game/ramen` 规则，不复制温泉杯规则。

## 尚未完成

- Android assets 数据目录接入；
- ARM64/JNI 或其他 Android 前端；
- 编译和真机验证；
- 与 PC 版固定种子的回归结果比对。

因此当前是“真实 RamenGame 核心已接入，Android 外壳和构建仍在继续”。

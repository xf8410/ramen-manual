# 手机版改造状态

本分支仍在持续改造中。

## 已完成

- 使用 PC `ramen_manual.rs` 的真实配置创建上游 `RamenGame`；
- `RamenGameAdapter` 已实现非阻塞暂停与触屏索引提交；
- 拉面、隐藏风味、训练、地区、超级拉面和事件选择统一转换为 UI 状态；
- Android JSON 状态协议；
- Android 运行数据目录校验；
- 明确只携带 `scenario_ramen.json`，不接入温泉剧本。

## 当前继续处理

- Android 宿主工程和 ARM64 构建；
- assets 解压与工作目录设置；
- 依赖编译验证；
- 固定种子下与 PC `ramen_manual` 的回归对照。

当前还不能称为 APK 完成，但已经从接口层进入真实 `RamenGame` 适配和运行时数据接入阶段。

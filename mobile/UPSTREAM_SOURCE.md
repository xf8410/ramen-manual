# 上游源码导入计划

上游源码：`https://github.com/xulai1001/umaai-rs`

目标分支：`workbench/mobile-ramen-apk`

## 导入原则

不能把 `ramen_manual.exe` 当作手机版运行时，也不能把终端版 `ManualTrainer` 原样作为 Android 入口。导入源码时应当同时做以下适配：

1. 导入 `crates/umasim` 的拉面杯核心模块和必要的数据加载模块；
2. 保留 `game/ramen/` 的规则、状态、事件、行动和结算逻辑；
3. 将 `inquire::Select` 从手机版执行路径移除；
4. 把同步的 `Trainer::select_action` / `select_choice` 改造成可暂停的决策边界；
5. 让 UI 能读取候选动作、候选事件选项和当前状态；
6. UI 点击后提交候选索引，模拟器继续执行；
7. Windows 命令行入口继续保留，不与手机版入口混用。

## 不导入到手机版

- `ramen_manual.exe`；
- Windows 专用运行文件；
- `inquire` 终端菜单作为 Android UI；
- 温泉剧本入口；
- 真实游戏连接功能。

## 数据

手机版只打包拉面杯运行所需数据，优先使用：

- `scenario_ramen.json`
- `constants.json`
- `events.json`
- `cardDB.json`
- `umaDB.json`
- `text_data_dict.json`
- `default_config.toml`

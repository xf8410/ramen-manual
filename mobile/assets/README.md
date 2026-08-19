# Android 运行数据

手机版必须把上游拉面杯运行所需数据放到应用私有工作目录，保持 `umasim` 的现有数据加载方式，不修改拉面杯规则。

至少需要：

```text
assets/gamedata/cardDB.json
assets/gamedata/constants.json
assets/gamedata/default_config.toml
assets/gamedata/events.json
assets/gamedata/scenario_ramen.json
assets/gamedata/text_data_dict.json
assets/gamedata/umaDB.json
assets/game_config.toml
```

启动时由 Android 宿主将这些 assets 解压到可写目录，并把当前工作目录设置到该目录，然后调用 `create_ramen_game()`。`scenario_onsen.json` 不属于拉面杯手机版运行链路。

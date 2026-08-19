# 手机版改造状态

本轮继续处理运行时问题：

- 修正 Android 配置不能直接复制仓库根目录 `default_config.toml` 的问题；根目录默认配置是 `scenario=onsen`，会导致手机版拒绝启动；
- 增加专用 `mobile/assets/game_config.toml`，明确 `scenario=ramen`、拉面杯卡组和继承配置；
- `copy-assets.sh` 现在固定生成拉面杯配置，不再把温泉配置伪装成 `game_config.toml`；
- 同步更新 `mobile/app/src/main/assets/game_config.toml` 模板。

仍在继续：native library 的 Gradle/NDK 产物接入、编译验证和真机运行。
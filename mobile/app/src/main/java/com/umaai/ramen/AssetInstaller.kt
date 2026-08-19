package com.umaai.ramen

import android.content.Context
import java.io.File

object AssetInstaller {
    private val dataFiles = listOf(
        "gamedata/cardDB.json", "gamedata/constants.json", "gamedata/default_config.toml",
        "gamedata/events.json", "gamedata/scenario_ramen.json", "gamedata/text_data_dict.json",
        "gamedata/umaDB.json"
    )

    fun install(context: Context): File {
        val root = File(context.filesDir, "ramen-data")
        root.mkdirs()

        for (relative in dataFiles) {
            installIfMissing(context, relative, File(root, relative))
        }

        // umasim's load_game_config() resolves the conventional default_config.toml
        // from the current working directory. Keep the APK asset named game_config.toml
        // so it cannot be confused with the repository's Onsen default_config.toml,
        // but always refresh the extracted loader path on every launch.
        installAlways(context, "game_config.toml", File(root, "default_config.toml"))
        installAlways(context, "game_config.toml", File(root, "game_config.toml"))

        for (relative in dataFiles) {
            check(File(root, relative).isFile && File(root, relative).length() > 0) {
                "运行数据无效: $relative"
            }
        }
        check(File(root, "default_config.toml").isFile && File(root, "default_config.toml").length() > 0) {
            "手机版配置安装失败"
        }
        return root
    }

    private fun installIfMissing(context: Context, relative: String, target: File) {
        if (target.isFile && target.length() > 0L) return
        copyAsset(context, relative, target)
    }

    private fun installAlways(context: Context, relative: String, target: File) {
        copyAsset(context, relative, target)
    }

    private fun copyAsset(context: Context, relative: String, target: File) {
        target.parentFile?.mkdirs()
        val temp = File(target.parentFile, ".${target.name}.tmp")
        context.assets.open(relative).use { input ->
            temp.outputStream().use { output -> input.copyTo(output) }
        }
        check(temp.length() > 0L) { "运行数据为空: $relative" }
        check(temp.renameTo(target) || (temp.copyTo(target, overwrite = true).let { temp.delete(); true })) {
            "安装运行数据失败: $relative"
        }
    }
}

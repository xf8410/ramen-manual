package com.umaai.ramen

import android.content.Context
import java.io.File

object AssetInstaller {
    private val files = listOf(
        "gamedata/cardDB.json", "gamedata/constants.json", "gamedata/default_config.toml",
        "gamedata/events.json", "gamedata/scenario_ramen.json", "gamedata/text_data_dict.json",
        "gamedata/umaDB.json", "game_config.toml"
    )

    fun install(context: Context): File {
        val root = File(context.filesDir, "ramen-data")
        for (relative in files) {
            val target = File(root, relative)
            if (!target.isFile) {
                target.parentFile?.mkdirs()
                context.assets.open(relative).use { input -> target.outputStream().use { input.copyTo(it) } }
            }
            check(target.isFile && target.length() > 0) { "运行数据为空: $relative" }
        }
        return root
    }
}

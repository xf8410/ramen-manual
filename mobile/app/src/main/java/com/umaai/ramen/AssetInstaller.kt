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
            if (!target.isFile || target.length() == 0L) {
                target.parentFile?.mkdirs()
                val temp = File(target.parentFile, ".${target.name}.tmp")
                context.assets.open(relative).use { input -> temp.outputStream().use { input.copyTo(it) } }
                check(temp.length() > 0) { "运行数据为空: $relative" }
                check(temp.renameTo(target)) { "安装运行数据失败: $relative" }
            }
            check(target.isFile && target.length() > 0) { "运行数据无效: $relative" }
        }
        return root
    }
}

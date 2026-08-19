package com.umaai.ramen

import android.app.Activity
import android.os.Bundle
import android.view.View
import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import org.json.JSONObject

class MainActivity : Activity() {
    private lateinit var title: TextView
    private lateinit var options: LinearLayout

    companion object {
        init { System.loadLibrary("ramen_mobile_core") }
        @JvmStatic private external fun nativeSetDataRoot(path: String): Boolean
        @JvmStatic private external fun nativeStart(): String
        @JvmStatic private external fun nativeSubmit(index: Int): String
        @JvmStatic private external fun nativeReset()
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val root = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(32, 32, 32, 32)
        }
        title = TextView(this).apply { textSize = 22f }
        options = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL }
        root.addView(title); root.addView(options); setContentView(root)

        try {
            val dataRoot = AssetInstaller.install(this)
            if (!nativeSetDataRoot(dataRoot.absolutePath)) {
                showError("运行数据目录设置失败")
            } else {
                renderState(nativeStart())
            }
        } catch (error: Exception) {
            showError("运行数据安装失败：${error.message ?: "未知错误"}")
        }
    }

    override fun onDestroy() { nativeReset(); super.onDestroy() }

    private fun showError(message: String) {
        title.text = message
        options.removeAllViews()
    }

    private fun renderState(json: String) {
        val state = JSONObject(json)
        if (state.optString("status") == "error") {
            showError(state.optString("message", "发生错误")); return
        }
        when (state.optString("status")) {
            "decision" -> {
                val decision = state.getJSONObject("decision")
                title.text = "第${state.optInt("turn")}回合\n${decision.optString("title")}"
                options.removeAllViews()
                val list = decision.getJSONArray("options")
                for (i in 0 until list.length()) {
                    val option = list.getJSONObject(i)
                    options.addView(Button(this).apply {
                        text = option.optString("title")
                        setOnClickListener {
                            options.setButtonsEnabled(false)
                            renderState(nativeSubmit(option.optInt("index", i)))
                        }
                    })
                }
            }
            "finished" -> {
                val summary = state.optJSONObject("summary")
                title.text = if (summary == null) "育成结束" else {
                    "育成结束\n回合：${summary.optInt("final_turn")}\n评分：${summary.optInt("score")}\nPT：${summary.optInt("pt")}"
                }
                options.removeAllViews()
            }
            else -> { title.text = "拉面杯"; options.removeAllViews() }
        }
    }

    private fun LinearLayout.setButtonsEnabled(enabled: Boolean) {
        for (i in 0 until childCount) getChildAt(i).isEnabled = enabled
    }
}

package com.umaai.ramen

import android.app.Activity
import android.os.Bundle
import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import org.json.JSONObject

class MainActivity : Activity() {
    private lateinit var title: TextView
    private lateinit var options: LinearLayout

    companion object {
        init { System.loadLibrary("ramen_mobile_core") }
        @JvmStatic private external fun nativeStart(): String
        @JvmStatic private external fun nativeSubmit(index: Int): String
        @JvmStatic private external fun nativeReset()
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val root = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL; setPadding(32, 32, 32, 32) }
        title = TextView(this).apply { textSize = 22f }
        options = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL }
        root.addView(title); root.addView(options); setContentView(root)
        renderState(nativeStart())
    }

    override fun onDestroy() { nativeReset(); super.onDestroy() }

    private fun renderState(json: String) {
        val state = JSONObject(json)
        if (state.optString("status") == "error") { title.text = state.optString("message", "发生错误"); options.removeAllViews(); return }
        when (state.optString("status")) {
            "decision" -> {
                val decision = state.getJSONObject("decision")
                title.text = "第${state.optInt("turn")}回合\n${decision.optString("title")}"
                options.removeAllViews()
                val list = decision.getJSONArray("options")
                for (i in 0 until list.length()) {
                    val option = list.getJSONObject(i)
                    options.addView(Button(this).apply { text = option.optString("title"); setOnClickListener { renderState(nativeSubmit(option.optInt("index", i))) } })
                }
            }
            "finished" -> { title.text = "育成结束"; options.removeAllViews() }
            else -> { title.text = "拉面杯"; options.removeAllViews() }
        }
    }
}

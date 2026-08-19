package com.umaai.ramen

import android.app.Activity
import android.os.Bundle
import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView

/**
 * 触屏外壳：按钮只提交候选索引，不重建 RamenGame 规则。
 * Native bridge 接入后，renderDecision 将由 Rust UiState 驱动。
 */
class MainActivity : Activity() {
    private lateinit var title: TextView
    private lateinit var options: LinearLayout

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val root = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL; setPadding(32, 32, 32, 32) }
        title = TextView(this).apply { textSize = 22f }
        options = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL }
        root.addView(title); root.addView(options)
        setContentView(root)
        title.text = "拉面杯\n正在准备模拟器…"
    }

    fun renderDecision(decisionTitle: String, labels: List<String>, onSelected: (Int) -> Unit) {
        title.text = decisionTitle
        options.removeAllViews()
        labels.forEachIndexed { index, label ->
            options.addView(Button(this).apply { text = label; setOnClickListener { onSelected(index) } })
        }
    }
}

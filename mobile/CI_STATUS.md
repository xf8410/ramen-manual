# CI 状态

Android workflow 已加入：

```text
.github/workflows/mobile-android.yml
```

它会构建：

```text
Rust ARM64 cdylib
→ Android JNI library
→ Debug APK
→ GitHub Actions artifact
```

当前分支写入 workflow 后，若 GitHub 的 `workflow_dispatch` 尚未在默认分支登记，API 可能拒绝从工作分支直接 dispatch。这不是 CI 正在占用任务；需要先创建 PR/合并 workflow 到默认分支，或在 GitHub Actions 页面手动选择该分支运行。

目前没有伪造 CI 成功结果。实际编译状态必须以 Actions 日志为准。

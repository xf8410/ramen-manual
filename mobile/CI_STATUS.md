# CI 状态

工作流：`.github/workflows/mobile-android.yml`

预期步骤：

```text
Android SDK/NDK
→ Rust aarch64 target
→ cargo-ndk
→ libramen_mobile_core.so
→ assembleDebug
→ APK artifact
```

当前 API 查询没有返回 workflow run，因此没有成功或失败可以报告。不能把 workflow 文件提交视为编译验证。

如果工作流不能从工作分支 dispatch，先创建 PR 让 workflow 出现在默认分支，或在 GitHub Actions 页面选择 `workbench/mobile-ramen-apk` 手动运行。
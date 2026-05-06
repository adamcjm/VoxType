# Development Progress

> Last updated: 2026-05-06

## Summary

| Phase | Status | Tests | Highlights |
|-------|--------|-------|------------|
| S0: Scaffold | ✅ | - | Tauri v2 + React 19 + Rust 骨架 |
| S1: Audio + Hotkey | ✅ | 14 | cpal 录音、VAD、降噪、Fn 键 |
| S2: STT Integration | ✅ | 17 | Groq/OpenAI/Deepgram/Local/Custom API |
| S3: Keyboard Output | ✅ | 20 | enigo 模拟、剪贴板粘贴、IME 绕过 |
| S4: Capsule UI | ✅ | 20 | 悬浮胶囊、波形动画、状态指示 |
| S5: LLM Polish | ✅ | 25 | 多场景 Prompt、重试、token 追踪 |
| S6: Settings + History | ✅ | 28 | JSON 持久化、SQLite 历史 |
| S7: Local Whisper + Release | ✅ | 28 | whisper.cpp、模型下载、打包配置 |

## Post-Phase Enhancements

| Enhancement | Description |
|-------------|-------------|
| CGEventTap Fn Key | macOS CGEventTap 拦截 Fn 键，阻止 emoji 弹窗，toggle 录音 |
| Typeless UI Redesign | 圆润 pill 胶囊、侧边栏导航 Settings、品牌色重构 |
| i18n Multi-language | zh-CN / en-US，系统语言自动检测，Settings 手动切换 |
| Model Dropdown | 填入 API Key 自动 GET /models 拉取模型列表，下拉选择 |
| Unified ~/.VoxType/ | 所有数据统一到一个跨平台路径 |
| Dual-window Architecture | Capsule 窗口 + Main 窗口独立入口 |
| .env Support | 开发者用 .env 跳过 Settings |
| Onboarding Flow | 首次启动自动弹出 Settings 引导配置 |

## Final State

| Metric | Value |
|--------|-------|
| Rust | 0 errors, 0 warnings |
| Frontend | Build OK |
| Tests | 28/28 passed |
| Commits | 18 |
| GitHub | [adamcjm/VoxType](https://github.com/adamcjm/VoxType) |
| Docs | 12 files |

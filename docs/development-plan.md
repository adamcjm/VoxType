# VoxType Development Plan

## All Phases Complete ✅

| Phase | Content | Status | Tests |
|-------|---------|--------|-------|
| S0 | Tauri v2 + React 19 + Rust 骨架、全模块目录、文档体系 | ✅ | - |
| S1 | cpal 音频采集、VAD、降噪、Fn 热键基础 | ✅ | 14 |
| S2 | Groq/OpenAI/Deepgram/Local/Custom STT API 实现 | ✅ | 17 |
| S3 | enigo 键盘模拟、剪贴板粘贴、IME 绕过 | ✅ | 20 |
| S4 | 悬浮胶囊、波形动画、状态指示、窗口分离 | ✅ | 20 |
| S5 | 多场景 Prompt、重试逻辑、token 追踪、Settings 集成 | ✅ | 25 |
| S6 | JSON 持久化、SQLite 历史、Onboarding 引导 | ✅ | 28 |
| S7 | whisper.cpp 本地推理、模型下载、打包配置 | ✅ | 28 |

## Post-Phase Enhancements

| Enhancement | Description |
|-------------|-------------|
| CGEventTap | macOS Fn 键全局拦截（阻止 emoji 弹窗） |
| Typeless UI | 圆润 pill 胶囊、侧边栏导航、品牌色 |
| i18n | zh-CN / en-US 多语言支持 |
| Model Dropdown | API GET /models 自动拉取模型列表 |
| ~/.VoxType/ | 统一跨平台数据目录 |
| .env | 开发者快速配置跳过 UI |

## Future Roadmap

- [ ] Tauri 全局热键 (Windows RegisterHotKey / Linux XGrabKey)
- [ ] Bundled whisper.cpp binary in app resources
- [ ] Homebrew cask / WinGet packages
- [ ] Mobile companion app
- [ ] Plugin system for custom STT/LLM integrations
- [ ] Voice commands ("delete last sentence")

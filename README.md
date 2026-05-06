# VoxType

> **说出来，不用打字。** — 开源 AI 语音输入工具，Typeless 替代方案。

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri)](https://v2.tauri.app)
[![Rust](https://img.shields.io/badge/Rust-edition2021-000000?logo=rust)](https://www.rust-lang.org)
[![React](https://img.shields.io/badge/React-19-61DAFB?logo=react)](https://react.dev)

VoxType 是你的桌面 AI 语音输入助手。按一下 Fn 键，说出你想说的话，AI 自动转写成流畅的文字，粘贴到你正在使用的任何应用中。

```
🎙 按 Fn → 说话 → 再按 Fn → 转录 → AI 润色 → 粘贴到光标
```

---

## Quick Start

```bash
# 1. 克隆项目
git clone https://github.com/adamcjm/VoxType.git
cd VoxType

# 2. 安装依赖
pnpm install

# 3. 启动开发模式
pnpm tauri dev
```

首次启动会**自动弹出 Settings 面板** — 选择你的语音识别和 AI 润色提供商，填入 API Key，点 Save。

下次启动会自动加载已保存的配置，无需重复设置。

### 开发者快速配置 (可选)

不想用 UI？创建 `.env` 文件跳过 Settings：

```bash
cp .env.example .env
# 编辑 .env 填入 API Key
pnpm tauri dev
```

---

## 功能

| 功能 | 说明 |
|------|------|
| 🎙 **全局热键** | Fn (macOS) / Right Alt (Win/Linux)，按一下开始，再按一下结束 |
| 🗣 **语音转文字** | Groq / Deepgram / OpenAI Whisper / 本地 Whisper / 自定义端点 |
| ✨ **AI 润色** | 去口头禅、修同音错字、加标点、结构化 — DeepSeek / OpenAI / Gemini / Ollama |
| 🌐 **翻译模式** | 说中文出英文，说英文出中文 |
| 💊 **悬浮胶囊** | Typeless 风格圆润 pill，毛玻璃效果，录音波形动画 |
| 📋 **剪贴板输出** | 绕过输入法，直接粘贴到光标位置 |
| 📖 **历史记录** | SQLite 全文搜索，最近转录回顾 |
| 🎨 **多语言** | 中文 / English，系统语言自动检测 |
| 🔄 **模型下拉** | 填入 API Key 自动拉取可用模型列表 |
| 🖥 **跨平台** | macOS / Windows / Linux |

---

## STT 提供商

在 Settings → Speech Recognition 中配置。

| 提供商 | 费用 | 模型 | 注册 |
|--------|------|------|------|
| **Groq** | 免费 | whisper-large-v3-turbo | [console.groq.com](https://console.groq.com/keys) |
| **Deepgram** | $200 免费额度 | nova-2, nova-3 | [console.deepgram.com](https://console.deepgram.com) |
| **OpenAI** | 按量付费 | whisper-1 | [platform.openai.com](https://platform.openai.com) |
| **本地 Whisper** | 免费 | ggml-small.bin (466MB) | 不需要 API Key |
| **自定义** | 按你的服务 | OpenAI 兼容格式 | 任意端点 |

填入 API Key 后，模型字段会自动从 API 拉取可选模型列表（或使用刷新按钮🔄手动拉取）。

## LLM 提供商

在 Settings → AI Polish 中配置。

| 提供商 | 费用 | 模型 |
|--------|------|------|
| **DeepSeek** | ~¥1/M tokens | deepseek-chat, deepseek-reasoner |
| **OpenAI** | $0.15/M tokens | gpt-4o-mini, gpt-4o |
| **Groq** | 免费 | llama-3.3-70b-versatile |
| **Gemini** | 免费 | gemini-2.0-flash |
| **Ollama** | 免费（本地） | llama3, 任意本地模型 |
| **自定义** | 按你的服务 | OpenAI 兼容格式 |

---

## 配置

### 数据目录

所有 VoxType 数据统一存放在 `~/.VoxType/`（跨平台一致）：

```
~/.VoxType/
├── config.json     # 用户设置（STT/LLM providers、模型、API Key）
├── history.db      # SQLite 转录历史
└── models/          # Whisper 模型文件（本地模式）
    └── ggml-small.bin
```

### 配置路径

| 方式 | 路径 | 优先级 |
|------|------|--------|
| `.env` 文件 | 项目根目录 `.env` | 最高（开发者） |
| Settings 保存 | `~/.VoxType/config.json` | 用户持久化 |
| 代码默认值 | 内置 | 出厂设置 |

---

## 开发

### 环境要求

| 工具 | 版本 | 检查 |
|------|------|------|
| Rust | 1.80+ | `rustc --version` |
| Node.js | 20+ | `node --version` |
| pnpm | 10+ | `pnpm --version` |

### 常用命令

```bash
pnpm install            # 安装前端依赖
pnpm tauri dev          # 启动开发模式（热重载）
pnpm tauri build        # 生产构建

cargo build             # 仅编译 Rust
pnpm build              # 仅编译前端

# 测试
cargo test --manifest-path src-tauri/Cargo.toml    # Rust 28 个测试
cargo test --manifest-path src-tauri/Cargo.toml -- --nocapture  # 显示输出
```

### 项目结构

```
voxtype/
├── src/                      # React 19 + TypeScript 前端
│   ├── components/
│   │   ├── capsule/           #   Capsule.tsx 悬浮胶囊（Typeless 风格）
│   │   ├── settings/          #   Settings.tsx 侧边栏导航设置面板
│   │   └── history/           #   History.tsx 转录历史列表
│   ├── i18n/                  #   多语言 (zh-CN, en-US)
│   ├── stores/                #   Zustand 状态管理
│   └── styles/                #   Tailwind CSS 4
├── src-tauri/                 # Rust 后端
│   ├── src/
│   │   ├── audio/             #   cpal 音频采集 + VAD + 降噪
│   │   ├── stt/               #   STT Provider (Groq/OpenAI/Deepgram/Local/Custom)
│   │   ├── llm/               #   LLM Provider + 多场景 Prompt + 重试
│   │   ├── output/            #   键盘模拟 + 剪贴板粘贴 + IME 绕过
│   │   ├── pipeline/          #   全流程编排 (STT→LLM→Output)
│   │   ├── hotkey_macos.rs    #   macOS CGEventTap Fn 键拦截
│   │   ├── model_manager/     #   Whisper 模型下载管理
│   │   ├── config/            #   配置加载/保存 (.env + JSON)
│   │   ├── history/           #   SQLite 转录历史
│   │   └── commands/          #   Tauri 命令处理器
│   └── Cargo.toml
├── docs/                      # 完整文档体系
├── capsule.html               # 悬浮胶囊窗口入口
├── index.html                 # 主窗口入口
└── .env.example               # 开发者配置模板
```

### 架构

```
Fn 键 → CGEventTap 拦截 → toggle_recording
  → AudioCapture (cpal, 16kHz PCM) → VAD → 降噪
  → STT API (Deepgram/Groq/OpenAI/Local) → 原始文本
  → LLM API (DeepSeek/Gemini/Ollama) → 润色后文本
  → 剪贴板 Cmd+V 粘贴 → 恢复原剪贴板 → 存历史 SQLite
```

---

## 测试

```
28 passed, 0 failed, 0 ignored
```

| 模块 | 测试数 | 内容 |
|------|--------|------|
| audio | 12 | WAV 编码、VAD 检测、降噪处理 |
| stt | 3 | OpenAI 兼容 API 调用 (mock HTTP) |
| llm | 5 | Polish、认证、翻译、空文本 |
| output | 3 | 键盘、剪贴板、IME 检测 |
| history | 3 | CRUD、全文搜索 |
| hotkey | 2 | 平台键码 |

---

## 生产构建

```bash
pnpm tauri build
```

产物：

| 平台 | 格式 | 路径 |
|------|------|------|
| macOS | `.dmg` | `src-tauri/target/release/bundle/dmg/` |
| Windows | `.msi` | `src-tauri/target/release/bundle/msi/` |
| Linux | `.AppImage` `.deb` | `src-tauri/target/release/bundle/appimage/` |

---

## 故障排除

| 问题 | 解决 |
|------|------|
| Fn 键弹出 emoji | System Settings → Keyboard → Press 🌐 to → **Do Nothing** |
| Fn 键无反应 | System Settings → Privacy → Accessibility → 授权你的终端 |
| 麦克风不工作 | System Settings → Privacy → Microphone → 授权 |
| 界面黑屏 | 重启 `pnpm tauri dev` |
| 编译失败 | 确保 `xcode-select --install` 和 `rustup update` |

---

## 许可

MIT

## 参考

- [Typeless](https://typeless.com) — UX 参考
- [tover0314-w/opentypeless](https://github.com/tover0314-w/opentypeless) — Tauri 架构参考
- [kuleka/OpenTypeless](https://github.com/kuleka/OpenTypeless) — macOS 原生参考

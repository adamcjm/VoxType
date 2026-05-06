const zh = {
  app: {
    title: "VoxType",
    tagline: "说出来，不用打字。",
    hint: "开始语音输入",
    settings: "设置",
  },
  capsule: {
    recording: "录音中",
    transcribing: "转录中…",
    polishing: "润色中…",
    pasted: "已粘贴",
    error: "错误",
  },
  settings: {
    speech: "语音识别",
    polish: "AI 润色",
    translate: "翻译",
    save: "保存",
    saved: "✓ 已保存",
    close: "关闭",
    provider: "提供商",
    apiKey: "API 密钥",
    model: "模型",
    language: "语言",
    temperature: "温度",
    customPrompt: "自定义提示词（可选）",
    customPromptPlaceholder: "覆盖默认的润色指令…",
    apiKeyPlaceholder: "粘贴你的 API 密钥…",
    source: "源语言",
    target: "目标语言",
    sttProviders: {
      groq: "Groq（免费）",
      deepgram: "Deepgram",
      openai: "OpenAI Whisper",
      local: "本地 Whisper（离线）",
      custom: "自定义",
    },
    llmProviders: {
      deepseek: "DeepSeek",
      openai: "OpenAI",
      groq: "Groq",
      gemini: "Google Gemini",
      ollama: "Ollama（本地）",
      custom: "自定义",
    },
    languages: {
      zh: "中文",
      en: "English",
      ja: "日本語",
      ko: "한국어",
      auto: "自动检测",
      de: "Deutsch",
      fr: "Français",
      es: "Español",
    },
  },
  history: {
    recent: "最近",
  },
};

export default zh;
export type Translations = typeof zh;

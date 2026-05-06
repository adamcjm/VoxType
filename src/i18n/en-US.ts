import type { Translations } from "./zh-CN";

const en: Translations = {
  app: {
    title: "VoxType",
    tagline: "Speak, don't type.",
    hint: "to start dictating",
    settings: "Settings",
  },
  capsule: {
    recording: "Recording",
    transcribing: "Transcribing…",
    polishing: "Polishing…",
    pasted: "Pasted",
    error: "Error",
  },
  settings: {
    speech: "Speech Recognition",
    polish: "AI Polish",
    translate: "Translate",
    save: "Save",
    saved: "✓ Saved",
    close: "Close",
    provider: "Provider",
    apiKey: "API Key",
    model: "Model",
    language: "Language",
    temperature: "Temperature",
    customPrompt: "Custom Prompt (optional)",
    customPromptPlaceholder: "Override the default polish instructions…",
    apiKeyPlaceholder: "Paste your API key…",
    source: "Source",
    target: "Target",
    sttProviders: {
      groq: "Groq (free)",
      deepgram: "Deepgram",
      openai: "OpenAI Whisper",
      local: "Local Whisper (offline)",
      custom: "Custom",
    },
    llmProviders: {
      deepseek: "DeepSeek",
      openai: "OpenAI",
      groq: "Groq",
      gemini: "Google Gemini",
      ollama: "Ollama (local)",
      custom: "Custom",
    },
    languages: {
      zh: "中文",
      en: "English",
      ja: "日本語",
      ko: "한국어",
      auto: "Auto",
      de: "Deutsch",
      fr: "Français",
      es: "Español",
    },
  },
  history: {
    recent: "Recent",
  },
};

export default en;

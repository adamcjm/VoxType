import { create } from "zustand";

export interface SttConfig {
  provider: "groq" | "openai" | "deepgram" | "local" | "custom";
  baseUrl: string;
  apiKey: string;
  model: string;
  language: string;
}

export interface LlmConfig {
  provider: "openai" | "deepseek" | "anthropic" | "gemini" | "groq" | "ollama" | "custom";
  baseUrl: string;
  apiKey: string;
  model: string;
  temperature: number;
  maxTokens: number;
  customPrompt: string;
}

export interface HotkeyConfig {
  macos: string;
  other: string;
}

export interface TranslateConfig {
  enabled: boolean;
  sourceLang: string;
  targetLang: string;
}

export interface SettingsState {
  open: boolean;
  stt: SttConfig;
  llm: LlmConfig;
  hotkey: HotkeyConfig;
  translate: TranslateConfig;
  theme: "light" | "dark" | "system";

  setOpen: (open: boolean) => void;
  setStt: (stt: Partial<SttConfig>) => void;
  setLlm: (llm: Partial<LlmConfig>) => void;
  setHotkey: (hotkey: Partial<HotkeyConfig>) => void;
  setTranslate: (translate: Partial<TranslateConfig>) => void;
  setTheme: (theme: "light" | "dark" | "system") => void;
}

// Provider-specific defaults
const STT_DEFAULTS: Record<SttConfig["provider"], Pick<SttConfig, "baseUrl" | "model">> = {
  groq: {
    baseUrl: "https://api.groq.com/openai/v1",
    model: "whisper-large-v3-turbo",
  },
  openai: {
    baseUrl: "https://api.openai.com/v1",
    model: "whisper-1",
  },
  deepgram: {
    baseUrl: "https://api.deepgram.com/v1",
    model: "nova-2",
  },
  local: {
    baseUrl: "http://localhost:8080",
    model: "ggml-small.bin",
  },
  custom: {
    baseUrl: "",
    model: "whisper-1",
  },
};

const LLM_DEFAULTS: Record<LlmConfig["provider"], Pick<LlmConfig, "baseUrl" | "model">> = {
  openai: {
    baseUrl: "https://api.openai.com/v1",
    model: "gpt-4o-mini",
  },
  deepseek: {
    baseUrl: "https://api.deepseek.com/v1",
    model: "deepseek-chat",
  },
  anthropic: {
    baseUrl: "https://api.anthropic.com/v1",
    model: "claude-3-haiku-20240307",
  },
  gemini: {
    baseUrl: "https://generativelanguage.googleapis.com/v1beta",
    model: "gemini-2.0-flash",
  },
  groq: {
    baseUrl: "https://api.groq.com/openai/v1",
    model: "llama-3.3-70b-versatile",
  },
  ollama: {
    baseUrl: "http://localhost:11434/v1",
    model: "llama3",
  },
  custom: {
    baseUrl: "",
    model: "",
  },
};

const defaultStt: SttConfig = {
  provider: "groq",
  baseUrl: STT_DEFAULTS.groq.baseUrl,
  apiKey: "",
  model: STT_DEFAULTS.groq.model,
  language: "zh",
};

const defaultLlm: LlmConfig = {
  provider: "deepseek",
  baseUrl: LLM_DEFAULTS.deepseek.baseUrl,
  apiKey: "",
  model: LLM_DEFAULTS.deepseek.model,
  temperature: 0.3,
  maxTokens: 4096,
  customPrompt: "",
};

const defaultHotkey: HotkeyConfig = {
  macos: "Fn",
  other: "RightAlt",
};

const defaultTranslate: TranslateConfig = {
  enabled: false,
  sourceLang: "auto",
  targetLang: "en",
};

export const useSettingsStore = create<SettingsState>((set) => ({
  open: false,
  stt: defaultStt,
  llm: defaultLlm,
  hotkey: defaultHotkey,
  translate: defaultTranslate,
  theme: "system",

  setOpen: (open) => set({ open }),

  setStt: (stt) =>
    set((s) => {
      const merged = { ...s.stt, ...stt };
      // Auto-fill defaults when provider changes
      if (stt.provider) {
        const defaults = STT_DEFAULTS[stt.provider];
        if (!stt.baseUrl) merged.baseUrl = defaults.baseUrl;
        if (!stt.model) merged.model = defaults.model;
      }
      return { stt: merged };
    }),

  setLlm: (llm) =>
    set((s) => {
      const merged = { ...s.llm, ...llm };
      if (llm.provider) {
        const defaults = LLM_DEFAULTS[llm.provider];
        if (!llm.baseUrl) merged.baseUrl = defaults.baseUrl;
        if (!llm.model) merged.model = defaults.model;
      }
      return { llm: merged };
    }),

  setHotkey: (hotkey) =>
    set((s) => ({ hotkey: { ...s.hotkey, ...hotkey } })),

  setTranslate: (translate) =>
    set((s) => ({ translate: { ...s.translate, ...translate } })),

  setTheme: (theme) => set({ theme }),
}));

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
  /** macOS key (Fn by default) */
  macos: string;
  /** Windows/Linux key (Right Alt by default) */
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

const defaultStt: SttConfig = {
  provider: "groq",
  baseUrl: "https://api.groq.com/openai/v1",
  apiKey: "",
  model: "whisper-large-v3-turbo",
  language: "zh",
};

const defaultLlm: LlmConfig = {
  provider: "deepseek",
  baseUrl: "https://api.deepseek.com/v1",
  apiKey: "",
  model: "deepseek-chat",
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

  setStt: (stt) => set((s) => ({ stt: { ...s.stt, ...stt } })),

  setLlm: (llm) => set((s) => ({ llm: { ...s.llm, ...llm } })),

  setHotkey: (hotkey) =>
    set((s) => ({ hotkey: { ...s.hotkey, ...hotkey } })),

  setTranslate: (translate) =>
    set((s) => ({ translate: { ...s.translate, ...translate } })),

  setTheme: (theme) => set({ theme }),
}));

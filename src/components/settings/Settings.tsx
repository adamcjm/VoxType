import { useEffect, useCallback } from "react";
import { useSettingsStore } from "../../stores/settingsStore";
import { X, Settings as SettingsIcon, Save } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import type { SettingsState } from "../../stores/settingsStore";

export default function Settings() {
  const open = useSettingsStore((s) => s.open);
  const setOpen = useSettingsStore((s) => s.setOpen);
  const stt = useSettingsStore((s) => s.stt);
  const llm = useSettingsStore((s) => s.llm);
  const hotkey = useSettingsStore((s) => s.hotkey);
  const translate = useSettingsStore((s) => s.translate);
  const setStt = useSettingsStore((s) => s.setStt);
  const setLlm = useSettingsStore((s) => s.setLlm);
  const setHotkey = useSettingsStore((s) => s.setHotkey);
  const setTranslate = useSettingsStore((s) => s.setTranslate);

  // Load settings from Rust backend on mount
  useEffect(() => {
    invoke("get_settings")
      .then((config) => {
        const c = config as SettingsState;
        setStt(c.stt);
        setLlm(c.llm);
        setHotkey(c.hotkey);
        setTranslate(c.translate);
      })
      .catch(console.error);
  }, []);

  // Save to Rust backend
  const saveSettings = useCallback(async () => {
    const config = { stt, llm, hotkey, translate, theme: "system" };
    try {
      await invoke("save_settings", { config });
      setOpen(false);
    } catch (e) {
      console.error("Failed to save settings:", e);
    }
  }, [stt, llm, hotkey, translate]);

  if (!open) return null;

  return (
    <div className="fixed inset-0 z-[9998] bg-black/40 flex items-center justify-center animate-fade-in">
      <div className="bg-white dark:bg-neutral-900 rounded-2xl shadow-2xl w-[640px] max-h-[85vh] overflow-hidden border border-neutral-200 dark:border-neutral-800">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-neutral-100 dark:border-neutral-800">
          <div className="flex items-center gap-2">
            <SettingsIcon className="w-5 h-5 text-brand-500" />
            <h2 className="text-lg font-semibold text-neutral-900 dark:text-neutral-100">
              VoxType Settings
            </h2>
          </div>
          <button
            onClick={() => setOpen(false)}
            className="p-1.5 rounded-lg hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors cursor-pointer"
          >
            <X className="w-4 h-4 text-neutral-500" />
          </button>
        </div>

        {/* Body */}
        <div className="p-6 space-y-8 overflow-y-auto max-h-[calc(85vh-120px)]">
          {/* Speech Recognition */}
          <section>
            <h3 className="text-sm font-semibold text-neutral-900 dark:text-neutral-100 mb-3">
               Speech Recognition
            </h3>
            <div className="space-y-3">
              <div>
                <label className="block text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider mb-1">
                  Provider
                </label>
                <select
                  className="w-full px-3 py-2 rounded-xl text-sm bg-neutral-50 dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 text-neutral-900 dark:text-neutral-100 focus:outline-none focus:ring-2 focus:ring-brand-400/40 focus:border-brand-400"
                  value={stt.provider}
                  onChange={(e) => setStt({ provider: e.target.value as any })}
                >
                  <option value="groq">Groq (free tier)</option>
                  <option value="openai">OpenAI Whisper</option>
                  <option value="deepgram">Deepgram</option>
                  <option value="local">Local Whisper (offline)</option>
                  <option value="custom">Custom Endpoint</option>
                </select>
              </div>
              <div>
                <label className="block text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider mb-1">
                  API Key
                </label>
                <input
                  type="password"
                  className="w-full px-3 py-2 rounded-xl text-sm bg-neutral-50 dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 text-neutral-900 dark:text-neutral-100 placeholder:text-neutral-400 focus:outline-none focus:ring-2 focus:ring-brand-400/40 focus:border-brand-400"
                  placeholder="gsk_..."
                  value={stt.apiKey}
                  onChange={(e) => setStt({ apiKey: e.target.value })}
                />
              </div>
              <div>
                <label className="block text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider mb-1">
                  Model
                </label>
                <input
                  type="text"
                  className="w-full px-3 py-2 rounded-xl text-sm bg-neutral-50 dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 text-neutral-900 dark:text-neutral-100 focus:outline-none focus:ring-2 focus:ring-brand-400/40 focus:border-brand-400"
                  value={stt.model}
                  onChange={(e) => setStt({ model: e.target.value })}
                />
              </div>
            </div>
          </section>

          {/* AI Polish */}
          <section>
            <h3 className="text-sm font-semibold text-neutral-900 dark:text-neutral-100 mb-3">
               AI Polish
            </h3>
            <div className="space-y-3">
              <div>
                <label className="block text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider mb-1">
                  Provider
                </label>
                <select
                  className="w-full px-3 py-2 rounded-xl text-sm bg-neutral-50 dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 text-neutral-900 dark:text-neutral-100 focus:outline-none focus:ring-2 focus:ring-brand-400/40 focus:border-brand-400"
                  value={llm.provider}
                  onChange={(e) => setLlm({ provider: e.target.value as any })}
                >
                  <option value="deepseek">DeepSeek</option>
                  <option value="openai">OpenAI (GPT-4o-mini)</option>
                  <option value="groq">Groq</option>
                  <option value="gemini">Google Gemini</option>
                  <option value="ollama">Ollama (local)</option>
                  <option value="custom">Custom Endpoint</option>
                </select>
              </div>
              <div>
                <label className="block text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider mb-1">
                  API Key
                </label>
                <input
                  type="password"
                  className="w-full px-3 py-2 rounded-xl text-sm bg-neutral-50 dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 text-neutral-900 dark:text-neutral-100 placeholder:text-neutral-400 focus:outline-none focus:ring-2 focus:ring-brand-400/40 focus:border-brand-400"
                  placeholder="sk-..."
                  value={llm.apiKey}
                  onChange={(e) => setLlm({ apiKey: e.target.value })}
                />
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <label className="block text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider mb-1">
                    Model
                  </label>
                  <input
                    type="text"
                    className="w-full px-3 py-2 rounded-xl text-sm bg-neutral-50 dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 text-neutral-900 dark:text-neutral-100 focus:outline-none focus:ring-2 focus:ring-brand-400/40 focus:border-brand-400"
                    value={llm.model}
                    onChange={(e) => setLlm({ model: e.target.value })}
                  />
                </div>
                <div>
                  <label className="block text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider mb-1">
                    Temperature
                  </label>
                  <input
                    type="number"
                    className="w-full px-3 py-2 rounded-xl text-sm bg-neutral-50 dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 text-neutral-900 dark:text-neutral-100 focus:outline-none focus:ring-2 focus:ring-brand-400/40 focus:border-brand-400"
                    min="0"
                    max="2"
                    step="0.1"
                    value={llm.temperature}
                    onChange={(e) => setLlm({ temperature: parseFloat(e.target.value) || 0 })}
                  />
                </div>
              </div>
              <div>
                <label className="block text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider mb-1">
                  Custom Prompt (optional)
                </label>
                <textarea
                  className="w-full px-3 py-2 rounded-xl text-sm bg-neutral-50 dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 text-neutral-900 dark:text-neutral-100 placeholder:text-neutral-400 focus:outline-none focus:ring-2 focus:ring-brand-400/40 focus:border-brand-400 h-20 resize-none"
                  placeholder="Override the default polish instructions..."
                  value={llm.customPrompt}
                  onChange={(e) => setLlm({ customPrompt: e.target.value })}
                />
              </div>
            </div>
          </section>

          {/* Translation */}
          <section>
            <h3 className="text-sm font-semibold text-neutral-900 dark:text-neutral-100 mb-3">
               Translation
            </h3>
            <div className="flex items-center gap-3">
              <div className="flex-1">
                <label className="block text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider mb-1">
                  Source
                </label>
                <select
                  className="w-full px-3 py-2 rounded-xl text-sm bg-neutral-50 dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 text-neutral-900 dark:text-neutral-100"
                  value={translate.sourceLang}
                  onChange={(e) => setTranslate({ sourceLang: e.target.value })}
                >
                  <option value="auto">Auto Detect</option>
                  <option value="zh">中文</option>
                  <option value="en">English</option>
                  <option value="ja">日本語</option>
                  <option value="ko">한국어</option>
                </select>
              </div>
              <div className="flex-1">
                <label className="block text-xs font-medium text-neutral-500 dark:text-neutral-400 uppercase tracking-wider mb-1">
                  Target
                </label>
                <select
                  className="w-full px-3 py-2 rounded-xl text-sm bg-neutral-50 dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 text-neutral-900 dark:text-neutral-100"
                  value={translate.targetLang}
                  onChange={(e) => setTranslate({ targetLang: e.target.value })}
                >
                  <option value="en">English</option>
                  <option value="zh">中文</option>
                  <option value="ja">日本語</option>
                  <option value="ko">한국어</option>
                  <option value="de">Deutsch</option>
                  <option value="fr">Français</option>
                  <option value="es">Español</option>
                </select>
              </div>
            </div>
          </section>
        </div>

        {/* Footer */}
        <div className="px-6 py-4 border-t border-neutral-100 dark:border-neutral-800 flex justify-between">
          <button
            onClick={() => setOpen(false)}
            className="px-4 py-2 rounded-xl text-sm font-medium text-neutral-600 dark:text-neutral-400 hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors cursor-pointer"
          >
            Cancel
          </button>
          <button
            onClick={saveSettings}
            className="px-5 py-2 rounded-xl text-sm font-medium bg-brand-500 text-white hover:bg-brand-600 transition-colors flex items-center gap-2 cursor-pointer"
          >
            <Save className="w-4 h-4" />
            Save
          </button>
        </div>
      </div>
    </div>
  );
}

import { useEffect, useState, useCallback } from "react";
import { useSettingsStore } from "../../stores/settingsStore";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { SettingsState } from "../../stores/settingsStore";

export default function SettingsPanel() {
  const stt = useSettingsStore((s) => s.stt);
  const llm = useSettingsStore((s) => s.llm);
  const translate = useSettingsStore((s) => s.translate);
  const setStt = useSettingsStore((s) => s.setStt);
  const setLlm = useSettingsStore((s) => s.setLlm);
  const setTranslate = useSettingsStore((s) => s.setTranslate);

  const [activeTab, setActiveTab] = useState<"stt" | "llm" | "translate">("stt");
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    invoke("get_settings").then((c) => {
      const cfg = c as SettingsState;
      setStt(cfg.stt); setLlm(cfg.llm); setTranslate(cfg.translate);
    }).catch(() => {});
  }, []);

  const save = useCallback(async () => {
    await invoke("save_settings", { config: { stt, llm, translate, hotkey: { macos:"Fn", other:"RightAlt" }, theme:"system" } });
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  }, [stt, llm, translate]);

  const close = () => getCurrentWindow().hide();

  const inputClass = "w-full px-3 py-2 bg-[#F8FAFC] dark:bg-neutral-800 border border-[#E2E8F0] dark:border-neutral-700 rounded-xl text-sm text-[#0F172A] dark:text-neutral-100 placeholder:text-[#94A3B8] focus:outline-none focus:ring-2 focus:ring-brand-400/30 focus:border-brand-400 transition-all";
  const selectClass = inputClass;
  const labelClass = "block text-[11px] font-semibold text-[#64748B] dark:text-neutral-400 uppercase tracking-wider mb-1.5";

  return (
    <div className="flex h-screen bg-[#FFFFFF] dark:bg-neutral-950">
      {/* Sidebar */}
      <div className="w-48 border-r border-[#F1F5F9] dark:border-neutral-800 flex flex-col shrink-0">
        <div className="px-5 py-4 border-b border-[#F1F5F9] dark:border-neutral-800">
          <span className="text-lg font-bold text-brand-500 tracking-tight">VoxType</span>
        </div>
        <nav className="flex-1 px-3 py-4 space-y-1">
          {[
            { id: "stt" as const, label: "Speech", icon: "🎤" },
            { id: "llm" as const, label: "AI Polish", icon: "✨" },
            { id: "translate" as const, label: "Translate", icon: "🌐" },
          ].map(tab => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`w-full flex items-center gap-3 px-3 py-2 rounded-xl text-sm font-medium transition-all cursor-pointer
                ${activeTab === tab.id
                  ? "bg-brand-50 dark:bg-brand-900/20 text-brand-600 dark:text-brand-400"
                  : "text-[#64748B] dark:text-neutral-400 hover:bg-[#F8FAFC] dark:hover:bg-neutral-800"}`}
            >
              <span>{tab.icon}</span>
              <span>{tab.label}</span>
            </button>
          ))}
        </nav>
        <div className="px-3 py-3 border-t border-[#F1F5F9] dark:border-neutral-800 space-y-2">
          <button onClick={save} className="w-full px-4 py-2 bg-brand-500 hover:bg-brand-600 text-white text-sm font-medium rounded-xl transition-all cursor-pointer">
            {saved ? "✓ Saved" : "Save"}
          </button>
          <button onClick={close} className="w-full px-4 py-2 text-sm text-[#94A3B8] dark:text-neutral-500 hover:text-[#64748B] dark:hover:text-neutral-400 transition-colors cursor-pointer">
            Close
          </button>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto">
        <div className="max-w-lg mx-auto px-8 py-8">
          {activeTab === "stt" && (
            <div className="space-y-5 animate-fade-in">
              <div>
                <h2 className="text-base font-semibold text-[#0F172A] dark:text-neutral-100 mb-4">Speech Recognition</h2>
                <label className={labelClass}>Provider</label>
                <select className={selectClass} value={stt.provider} onChange={e => setStt({ provider: e.target.value as any })}>
                  <option value="groq">Groq (free)</option>
                  <option value="deepgram">Deepgram</option>
                  <option value="openai">OpenAI Whisper</option>
                  <option value="local">Local Whisper (offline)</option>
                  <option value="custom">Custom</option>
                </select>
              </div>
              <div>
                <label className={labelClass}>API Key</label>
                <input type="password" className={inputClass} placeholder="Paste your API key…" value={stt.apiKey} onChange={e => setStt({ apiKey: e.target.value })} />
              </div>
              <div>
                <label className={labelClass}>Model</label>
                <input type="text" className={inputClass} value={stt.model} onChange={e => setStt({ model: e.target.value })} />
              </div>
              <div>
                <label className={labelClass}>Language</label>
                <select className={selectClass} value={stt.language} onChange={e => setStt({ language: e.target.value })}>
                  <option value="zh">中文</option>
                  <option value="en">English</option>
                  <option value="ja">日本語</option>
                  <option value="ko">한국어</option>
                  <option value="auto">Auto</option>
                </select>
              </div>
            </div>
          )}

          {activeTab === "llm" && (
            <div className="space-y-5 animate-fade-in">
              <div>
                <h2 className="text-base font-semibold text-[#0F172A] dark:text-neutral-100 mb-4">AI Polish</h2>
                <label className={labelClass}>Provider</label>
                <select className={selectClass} value={llm.provider} onChange={e => setLlm({ provider: e.target.value as any })}>
                  <option value="deepseek">DeepSeek</option>
                  <option value="openai">OpenAI</option>
                  <option value="groq">Groq</option>
                  <option value="gemini">Google Gemini</option>
                  <option value="ollama">Ollama (local)</option>
                  <option value="custom">Custom</option>
                </select>
              </div>
              <div>
                <label className={labelClass}>API Key</label>
                <input type="password" className={inputClass} placeholder="Paste your API key…" value={llm.apiKey} onChange={e => setLlm({ apiKey: e.target.value })} />
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <label className={labelClass}>Model</label>
                  <input type="text" className={inputClass} value={llm.model} onChange={e => setLlm({ model: e.target.value })} />
                </div>
                <div>
                  <label className={labelClass}>Temperature</label>
                  <input type="number" className={inputClass} min="0" max="2" step="0.1" value={llm.temperature} onChange={e => setLlm({ temperature: parseFloat(e.target.value) || 0 })} />
                </div>
              </div>
              <div>
                <label className={labelClass}>Custom Prompt (optional)</label>
                <textarea className={`${inputClass} h-20 resize-none`} placeholder="Override the default polish instructions…" value={llm.customPrompt} onChange={e => setLlm({ customPrompt: e.target.value })} />
              </div>
            </div>
          )}

          {activeTab === "translate" && (
            <div className="space-y-5 animate-fade-in">
              <h2 className="text-base font-semibold text-[#0F172A] dark:text-neutral-100 mb-4">Translation</h2>
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <label className={labelClass}>Source</label>
                  <select className={selectClass} value={translate.sourceLang} onChange={e => setTranslate({ sourceLang: e.target.value })}>
                    <option value="auto">Auto Detect</option>
                    <option value="zh">中文</option>
                    <option value="en">English</option>
                    <option value="ja">日本語</option>
                    <option value="ko">한국어</option>
                  </select>
                </div>
                <div>
                  <label className={labelClass}>Target</label>
                  <select className={selectClass} value={translate.targetLang} onChange={e => setTranslate({ targetLang: e.target.value })}>
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
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

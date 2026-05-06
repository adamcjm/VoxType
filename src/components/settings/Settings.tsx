import { useSettingsStore } from "../../stores/settingsStore";
import { X, Settings as SettingsIcon } from "lucide-react";

export default function Settings() {
  const setOpen = useSettingsStore((s) => s.setOpen);
  const stt = useSettingsStore((s) => s.stt);
  const llm = useSettingsStore((s) => s.llm);
  const setStt = useSettingsStore((s) => s.setStt);
  const setLlm = useSettingsStore((s) => s.setLlm);

  return (
    <div className="fixed inset-0 z-[9998] bg-black/40 flex items-center justify-center animate-fade-in">
      <div className="bg-white dark:bg-neutral-900 rounded-2xl shadow-2xl w-[620px] max-h-[80vh] overflow-hidden border border-neutral-200 dark:border-neutral-800">
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

        <div className="p-6 space-y-6 overflow-y-auto max-h-[calc(80vh-64px)]">
          {/* Speech Recognition */}
          <section>
            <h3 className="text-sm font-semibold text-neutral-900 dark:text-neutral-100 mb-3">
               Speech Recognition
            </h3>
            <div className="space-y-3">
              <div>
                <label className="label">Provider</label>
                <select
                  className="input"
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
                <label className="label">API Key</label>
                <input
                  type="password"
                  className="input"
                  placeholder="sk-..."
                  value={stt.apiKey}
                  onChange={(e) => setStt({ apiKey: e.target.value })}
                />
              </div>
              <div>
                <label className="label">Model</label>
                <input
                  type="text"
                  className="input"
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
                <label className="label">Provider</label>
                <select
                  className="input"
                  value={llm.provider}
                  onChange={(e) => setLlm({ provider: e.target.value as any })}
                >
                  <option value="deepseek">DeepSeek</option>
                  <option value="openai">OpenAI</option>
                  <option value="anthropic">Anthropic (Claude)</option>
                  <option value="gemini">Google Gemini</option>
                  <option value="groq">Groq</option>
                  <option value="ollama">Ollama (local)</option>
                  <option value="custom">Custom Endpoint</option>
                </select>
              </div>
              <div>
                <label className="label">API Key</label>
                <input
                  type="password"
                  className="input"
                  placeholder="sk-..."
                  value={llm.apiKey}
                  onChange={(e) => setLlm({ apiKey: e.target.value })}
                />
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <label className="label">Model</label>
                  <input
                    type="text"
                    className="input"
                    value={llm.model}
                    onChange={(e) => setLlm({ model: e.target.value })}
                  />
                </div>
                <div>
                  <label className="label">Temperature</label>
                  <input
                    type="number"
                    className="input"
                    min="0"
                    max="2"
                    step="0.1"
                    value={llm.temperature}
                    onChange={(e) =>
                      setLlm({ temperature: parseFloat(e.target.value) })
                    }
                  />
                </div>
              </div>
            </div>
          </section>
        </div>
      </div>
    </div>
  );
}

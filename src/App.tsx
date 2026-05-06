import { useEffect } from "react";
import { Settings } from "lucide-react";
import SettingsPanel from "./components/settings/Settings";
import History from "./components/history/History";
import { useSettingsStore } from "./stores/settingsStore";
import { useHistoryStore } from "./stores/historyStore";
import { invoke } from "@tauri-apps/api/core";

export default function App() {
  const settingsOpen = useSettingsStore((s) => s.open);
  const setOpen = useSettingsStore((s) => s.setOpen);
  const setItems = useHistoryStore((s) => s.setItems);

  // Check onboarding on mount
  useEffect(() => {
    invoke("needs_onboarding")
      .then((needs) => { if (needs) setOpen(true); })
      .catch(() => {});

    // Load history
    invoke("get_history", { limit: 20 })
      .then((items) => setItems(items as any[]))
      .catch(() => {});
  }, []);

  return (
    <div className="w-screen h-screen bg-neutral-50 dark:bg-neutral-950 flex items-center justify-center">
      {settingsOpen ? (
        <SettingsPanel />
      ) : (
        /* Idle state: minimal launcher */
        <div className="text-center select-none">
          <div className="w-16 h-16 mx-auto mb-4 rounded-2xl bg-brand-500 flex items-center justify-center shadow-lg shadow-brand-500/20">
            <span className="text-white text-2xl font-bold">V</span>
          </div>
          <h1 className="text-xl font-semibold text-neutral-800 dark:text-neutral-200 mb-1">
            VoxType
          </h1>
          <p className="text-sm text-neutral-500 dark:text-neutral-400 mb-6">
            Press Fn to start dictating
          </p>
          <div className="flex items-center justify-center gap-3">
            <button
              onClick={() => setOpen(true)}
              className="px-4 py-2 rounded-xl text-sm font-medium bg-neutral-100 dark:bg-neutral-800 hover:bg-neutral-200 dark:hover:bg-neutral-700 text-neutral-700 dark:text-neutral-300 transition-colors cursor-pointer flex items-center gap-2"
            >
              <Settings className="w-4 h-4" />
              Settings
            </button>
          </div>
          <History />
        </div>
      )}
    </div>
  );
}

import { useEffect } from "react";
import SettingsPanel from "./components/settings/Settings";
import History from "./components/history/History";
import { useSettingsStore } from "./stores/settingsStore";
import { useRecordingStore } from "./stores/recordingStore";
import { useHistoryStore } from "./stores/historyStore";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export default function App() {
  const settingsOpen = useSettingsStore((s) => s.open);
  const setOpen = useSettingsStore((s) => s.setOpen);
  const setItems = useHistoryStore((s) => s.setItems);

  const isRecording = useRecordingStore((s) => s.isRecording);
  const startRecording = useRecordingStore((s) => s.startRecording);
  const setState = useRecordingStore((s) => s.setState);
  const setFinalText = useRecordingStore((s) => s.setFinalText);
  const setTranscript = useRecordingStore((s) => s.setTranscript);
  const setError = useRecordingStore((s) => s.setError);

  // Check onboarding + load history
  useEffect(() => {
    invoke("needs_onboarding").then((n) => { if (n) setOpen(true); }).catch(() => {});
    invoke("get_history", { limit: 20 }).then(items => setItems(items as any[])).catch(() => {});
  }, []);

  // Listen for Fn key toggle
  useEffect(() => {
    const unlisten = listen("hotkey:toggle", async () => {
      if (isRecording) {
        // Stop recording
        useRecordingStore.getState().stopRecording();
        setState("transcribing");
        try {
          const text: string = await invoke("stop_recording");
          setFinalText(text);
          setTranscript(text);
          setState("done");
        } catch (e) {
          setError(String(e));
        }
      } else {
        // Start recording
        startRecording();
        try {
          await invoke("start_recording");
        } catch (e) {
          setError(String(e));
        }
      }
    });
    return () => { unlisten.then(fn => fn()); };
  }, [isRecording]);

  // Show Settings
  if (settingsOpen) return <SettingsPanel />;

  // Idle state
  return (
    <div className="h-screen bg-[#FFFFFF] dark:bg-neutral-950 flex flex-col items-center justify-center select-none">
      <div className="flex flex-col items-center gap-6 pb-20">
        {/* Logo */}
        <div className="w-20 h-20 rounded-2xl bg-gradient-to-br from-brand-500 to-brand-600 shadow-lg shadow-brand-500/20 flex items-center justify-center">
          <span className="text-white text-3xl font-bold tracking-tight">V</span>
        </div>

        {/* Title */}
        <div className="text-center">
          <h1 className="text-2xl font-bold text-[#0F172A] dark:text-neutral-100 tracking-tight">
            VoxType
          </h1>
          <p className="mt-1 text-sm text-[#94A3B8] dark:text-neutral-500">
            Speak, don't type.
          </p>
        </div>

        {/* Hint */}
        <div className="flex items-center gap-2 px-4 py-2 bg-[#F8FAFC] dark:bg-neutral-900 rounded-full border border-[#F1F5F9] dark:border-neutral-800">
          <kbd className="px-2 py-0.5 text-[11px] font-semibold bg-white dark:bg-neutral-800 border border-[#E2E8F0] dark:border-neutral-700 rounded-md text-[#64748B] dark:text-neutral-400">
            Fn
          </kbd>
          <span className="text-[13px] text-[#94A3B8] dark:text-neutral-500">
            to start dictating
          </span>
        </div>

        {/* Settings button */}
        <button
          onClick={() => setOpen(true)}
          className="px-5 py-2.5 text-sm font-medium text-[#64748B] dark:text-neutral-400 hover:text-[#0F172A] dark:hover:text-neutral-200 hover:bg-[#F8FAFC] dark:hover:bg-neutral-800 rounded-xl transition-all cursor-pointer"
        >
          Settings
        </button>
      </div>

      {/* History at bottom */}
      <div className="absolute bottom-6 left-6 right-6">
        <History />
      </div>
    </div>
  );
}

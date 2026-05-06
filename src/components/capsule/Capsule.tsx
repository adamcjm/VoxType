import { motion, AnimatePresence } from "framer-motion";
import { useRecordingStore } from "../../stores/recordingStore";
import { useEffect } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import { useT } from "../../i18n";

export default function Capsule() {
  const t = useT();
  const isRecording = useRecordingStore((s) => s.isRecording);
  const state = useRecordingStore((s) => s.state);
  const transcript = useRecordingStore((s) => s.transcript);
  const error = useRecordingStore((s) => s.error);
  const duration = useRecordingStore((s) => s.duration);
  const setDuration = useRecordingStore((s) => s.setDuration);
  const reset = useRecordingStore((s) => s.reset);

  const visible = isRecording || state === "transcribing" || state === "polishing" || state === "done" || state === "error";

  // Listen for Fn toggle
  useEffect(() => {
    const unlisten = listen("hotkey:toggle", () => {});
    return () => { unlisten.then(fn => fn()); };
  }, []);

  // Show/hide window
  useEffect(() => {
    const win = getCurrentWindow();
    if (visible) {
      win.show();
      win.setAlwaysOnTop(true);
    } else {
      if ((state as string) === "done") {
        const t = setTimeout(() => { reset(); win.hide(); }, 2000);
        return () => clearTimeout(t);
      }
      win.hide();
    }
  }, [visible, state]);

  // Duration timer
  useEffect(() => {
    let timer: any;
    if (isRecording) {
      const start = Date.now();
      timer = setInterval(() => setDuration(Date.now() - start), 100);
    }
    return () => clearInterval(timer);
  }, [isRecording]);

  const fmt = (ms: number) => {
    const m = Math.floor(ms / 60000);
    const s = Math.floor((ms % 60000) / 1000);
    return `${m}:${String(s).padStart(2, "0")}`;
  };

  const statusText = {
    recording: t.capsule.recording,
    transcribing: t.capsule.transcribing,
    polishing: t.capsule.polishing,
    done: t.capsule.pasted,
    error: t.capsule.error,
    idle: "",
  };

  return (
    <AnimatePresence>
      {visible && (
        <motion.div
          initial={{ opacity: 0, y: -16, scale: 0.97 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          exit={{ opacity: 0, scale: 0.96 }}
          transition={{ type: "spring", stiffness: 400, damping: 30 }}
          className="absolute top-[10%] left-1/2 -translate-x-1/2"
          data-tauri-drag-region
        >
          <motion.div
            layout
            className="flex items-center gap-3 px-5 py-3 bg-white/85 backdrop-blur-2xl border border-black/[0.06] rounded-full shadow-[0_0_0_0.5px_rgba(0,0,0,0.05),0_4px_20px_rgba(0,0,0,0.06)] dark:bg-neutral-900/85 dark:border-white/[0.06] dark:shadow-[0_0_0_0.5px_rgba(255,255,255,0.03),0_4px_20px_rgba(0,0,0,0.3)]"
          >
            {isRecording && (
              <>
                <span className="relative flex h-2.5 w-2.5 shrink-0">
                  <span className="animate-pulse-dot absolute inline-flex h-full w-full rounded-full bg-red-500" />
                </span>
                <div className="flex items-end gap-px h-4">
                  {[0,1,2,3,4].map(i => (
                    <motion.div
                      key={i}
                      className="w-0.5 rounded-full bg-red-400/80"
                      animate={{ height: [6,18,10,16,8,18,6][i] ?? 8 }}
                      transition={{ duration: 0.6 + i * 0.05, repeat: Infinity, ease: "easeInOut", delay: i * 0.1 }}
                    />
                  ))}
                </div>
              </>
            )}
            {state === "transcribing" && (
              <div className="w-4 h-4 border-2 border-amber-400 border-t-transparent rounded-full animate-spin shrink-0" />
            )}
            {state === "polishing" && (
              <div className="flex gap-1 shrink-0">
                {[0,1,2].map(i => (
                  <motion.div key={i} className="w-1.5 h-1.5 rounded-full bg-brand-400"
                    animate={{ opacity: [0.3,1,0.3] }} transition={{ duration: 0.8, repeat: Infinity, delay: i*0.2 }} />
                ))}
              </div>
            )}
            {state === "done" && (
              <svg className="w-4 h-4 text-green-500 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3">
                <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
              </svg>
            )}
            {state === "error" && (
              <span className="text-sm font-medium text-red-500 shrink-0">!</span>
            )}
            <span className="text-[13px] font-medium text-[#334155] dark:text-neutral-300 whitespace-nowrap">
              {statusText[state]}
            </span>
            {isRecording && (
              <span className="text-[13px] font-mono tabular-nums text-[#94A3B8] dark:text-neutral-500 ml-auto">
                {fmt(duration)}
              </span>
            )}
          </motion.div>

          {transcript && state === "recording" && (
            <motion.div initial={{ opacity: 0, y: -4 }} animate={{ opacity: 1, y: 0 }}
              className="mt-2 px-5 py-3 bg-white/80 backdrop-blur-xl border border-black/[0.05] rounded-2xl shadow-sm max-h-28 overflow-y-auto mx-4">
              <p className="text-[13px] leading-relaxed text-[#334155] select-text">{transcript}</p>
            </motion.div>
          )}

          {error && (
            <motion.div initial={{ opacity: 0, y: -4 }} animate={{ opacity: 1, y: 0 }}
              className="mt-2 px-4 py-2.5 bg-red-50 border border-red-200 rounded-xl mx-4 max-w-[360px]">
              <p className="text-[12px] leading-relaxed text-red-600">{error}</p>
            </motion.div>
          )}
        </motion.div>
      )}
    </AnimatePresence>
  );
}

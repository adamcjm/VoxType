import { motion, AnimatePresence } from "framer-motion";
import { useRecordingStore } from "../../stores/recordingStore";
import { useEffect, useCallback } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import Waveform from "./Waveform";
import StatusBadge from "./StatusBadge";

export default function Capsule() {
  const isRecording = useRecordingStore((s) => s.isRecording);
  const state = useRecordingStore((s) => s.state);
  const transcript = useRecordingStore((s) => s.transcript);
  const error = useRecordingStore((s) => s.error);
  const duration = useRecordingStore((s) => s.duration);

  const visible = isRecording || state !== "idle";

  // Show/hide the capsule window based on state
  useEffect(() => {
    const appWindow = getCurrentWindow();
    if (visible) {
      appWindow.show();
      appWindow.setAlwaysOnTop(true);
    } else {
      // Auto-hide after 2s on completion
      if ((state as string) === "done") {
        const timer = setTimeout(() => appWindow.hide(), 2000);
        return () => clearTimeout(timer);
      }
      appWindow.hide();
    }
  }, [visible, state]);

  // Handle click: dismiss capsule on done/error
  const handleClick = useCallback(() => {
    if (state === "done" || state === "error") {
      getCurrentWindow().hide();
    }
  }, [state]);

  const formatDuration = (ms: number) => {
    const secs = Math.floor(ms / 1000);
    const mins = Math.floor(secs / 60);
    const remain = secs % 60;
    return `${mins}:${remain.toString().padStart(2, "0")}`;
  };

  return (
    <AnimatePresence>
      {visible && (
        <motion.div
          initial={{ opacity: 0, y: -12, scale: 0.97 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          exit={{ opacity: 0, y: -8, scale: 0.97 }}
          transition={{ duration: 0.25, ease: [0.22, 1, 0.36, 1] }}
          onClick={handleClick}
          className="absolute top-[10%] left-1/2 -translate-x-1/2 cursor-default"
          data-tauri-drag-region
        >
          <div
            className={`
              backdrop-blur-2xl rounded-2xl border shadow-2xl
              min-w-[340px] max-w-[520px] px-5 py-4
              transition-colors duration-300
              ${state === "recording"
                ? "bg-white/90 dark:bg-neutral-900/90 border-red-400/40 shadow-red-200/20"
                : state === "transcribing" || state === "polishing"
                  ? "bg-white/90 dark:bg-neutral-900/90 border-amber-400/40"
                  : state === "error"
                    ? "bg-white/90 dark:bg-neutral-900/90 border-red-500/60"
                    : "bg-white/85 dark:bg-neutral-900/85 border-neutral-200/40 dark:border-neutral-700/40"
              }`}
          >
            {/* Header row: status + duration/waveform */}
            <div className="flex items-center justify-between mb-1">
              <div className="flex items-center gap-2.5">
                <StatusBadge state={state} />
                <span className="text-xs font-medium text-neutral-500 dark:text-neutral-400 select-none">
                  {state === "recording" && "Recording"}
                  {state === "transcribing" && "Transcribing..."}
                  {state === "polishing" && "Polishing text..."}
                  {state === "done" && "Done"}
                  {state === "error" && "Error"}
                </span>
              </div>
              <div className="flex items-center gap-3">
                {state === "recording" && (
                  <>
                    <Waveform active />
                    <span className="text-xs font-mono tabular-nums text-neutral-400 dark:text-neutral-500 select-none min-w-[40px] text-right">
                      {formatDuration(duration)}
                    </span>
                  </>
                )}
                {state === "transcribing" && (
                  <div className="w-4 h-4 border-2 border-amber-400 border-t-transparent rounded-full animate-spin" />
                )}
                {state === "polishing" && (
                  <div className="flex gap-1">
                    {[0, 1, 2].map((i) => (
                      <motion.div
                        key={i}
                        className="w-1.5 h-1.5 rounded-full bg-brand-400"
                        animate={{ opacity: [0.3, 1, 0.3] }}
                        transition={{ duration: 0.8, repeat: Infinity, delay: i * 0.2 }}
                      />
                    ))}
                  </div>
                )}
              </div>
            </div>

            {/* Transcript text */}
            {transcript && (
              <motion.p
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: "auto" }}
                className="text-[13px] leading-relaxed text-neutral-700 dark:text-neutral-300 max-h-36 overflow-y-auto select-text"
              >
                {transcript}
              </motion.p>
            )}

            {/* Error message */}
            {error && (
              <motion.div
                initial={{ opacity: 0, y: 4 }}
                animate={{ opacity: 1, y: 0 }}
                className="mt-2 flex items-start gap-2 p-2.5 rounded-xl bg-red-50 dark:bg-red-950/30 border border-red-200 dark:border-red-900/50"
              >
                <span className="text-xs leading-relaxed text-red-700 dark:text-red-400">
                  {error}
                </span>
              </motion.div>
            )}

            {/* Done: brief summary */}
            {state === "done" && (
              <p className="text-xs text-neutral-400 dark:text-neutral-500 select-none mt-1">
                Text pasted at cursor — click to dismiss
              </p>
            )}
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}

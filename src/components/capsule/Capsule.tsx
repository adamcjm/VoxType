import { motion, AnimatePresence } from "framer-motion";
import { useRecordingStore } from "../../stores/recordingStore";
import { useEffect, useCallback } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import Waveform from "./Waveform";

export default function Capsule() {
  const isRecording = useRecordingStore((s) => s.isRecording);
  const state = useRecordingStore((s) => s.state);
  const transcript = useRecordingStore((s) => s.transcript);
  const error = useRecordingStore((s) => s.error);
  const duration = useRecordingStore((s) => s.duration);

  const setState = useRecordingStore((s) => s.setState);
  const setTranscript = useRecordingStore((s) => s.setTranscript);
  const setFinalText = useRecordingStore((s) => s.setFinalText);
  const setError = useRecordingStore((s) => s.setError);
  const setDuration = useRecordingStore((s) => s.setDuration);

  const visible = isRecording || state === "transcribing" || state === "done" || state === "error";

  // Listen for Fn key toggle from Rust backend
  useEffect(() => {
    const unlisten = listen("hotkey:toggle", () => {
      handleToggle();
    });
    return () => { unlisten.then((fn) => fn()); };
  }, [state, isRecording]);

  // Toggle recording: start or stop
  const handleToggle = useCallback(async () => {
    if (state === "idle") {
      // Start recording
      useRecordingStore.getState().startRecording();
      try {
        await invoke("start_recording");
        // Start duration timer
        const startTime = Date.now();
        const timer = setInterval(() => {
          setDuration(Date.now() - startTime);
        }, 100);
        (window as any).__voxTimer = timer;
      } catch (e) {
        setError(String(e));
      }
    } else if (isRecording) {
      // Stop recording
      useRecordingStore.getState().stopRecording();
      clearInterval((window as any).__voxTimer);
      try {
        setState("transcribing");
        const text: string = await invoke("stop_recording");
        setFinalText(text);
        setTranscript(text);
        setState("done");
      } catch (e) {
        setError(String(e));
      }
    }
  }, [state, isRecording]);

  // Show/hide window
  useEffect(() => {
    const appWindow = getCurrentWindow();
    if (visible) {
      appWindow.show();
      appWindow.setAlwaysOnTop(true);
    } else {
      if ((state as string) === "done") {
        const timer = setTimeout(() => {
          useRecordingStore.getState().reset();
          appWindow.hide();
        }, 2000);
        return () => clearTimeout(timer);
      }
      appWindow.hide();
    }
  }, [visible, state]);

  const formatTime = (ms: number) => {
    const m = Math.floor(ms / 60000);
    const s = Math.floor((ms % 60000) / 1000);
    return `${m}:${s.toString().padStart(2, "0")}`;
  };

  return (
    <AnimatePresence>
      {visible && (
        <motion.div
          initial={{ opacity: 0, y: -16, scale: 0.98 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          exit={{ opacity: 0, y: -8, scale: 0.98 }}
          transition={{ duration: 0.3, ease: [0.22, 1, 0.36, 1] }}
          className="absolute top-[12%] left-1/2 -translate-x-1/2"
          data-tauri-drag-region
        >
          <div className={`
            backdrop-blur-3xl rounded-full border shadow-lg
            min-w-[280px] max-w-[480px] px-5 py-3
            transition-all duration-300
            ${isRecording
              ? "bg-white/92 dark:bg-neutral-900/92 border-red-400/30 shadow-red-500/10"
              : state === "transcribing"
                ? "bg-white/92 dark:bg-neutral-900/92 border-amber-400/30"
                : state === "error"
                  ? "bg-white/92 dark:bg-neutral-900/92 border-red-500/40"
                  : "bg-white/88 dark:bg-neutral-900/88 border-neutral-200/30 dark:border-neutral-700/30"
            }`}
          >
            {/* Status row */}
            <div className="flex items-center gap-3">
              {/* Red dot + waveform when recording */}
              {isRecording && (
                <>
                  <span className="relative flex h-3 w-3">
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-red-400 opacity-75" />
                    <span className="relative inline-flex rounded-full h-3 w-3 bg-red-500" />
                  </span>
                  <Waveform active />
                  <span className="text-sm font-mono tabular-nums text-neutral-500 dark:text-neutral-400 ml-auto">
                    {formatTime(duration)}
                  </span>
                </>
              )}

              {/* Transcribing spinner */}
              {state === "transcribing" && (
                <>
                  <div className="w-4 h-4 border-2 border-amber-400 border-t-transparent rounded-full animate-spin" />
                  <span className="text-sm text-neutral-500 dark:text-neutral-400">
                    Transcribing...
                  </span>
                </>
              )}

              {/* Done: green check */}
              {state === "done" && (
                <>
                  <svg className="w-4 h-4 text-green-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={3}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
                  </svg>
                  <span className="text-sm text-neutral-500 dark:text-neutral-400">
                    Done — text pasted
                  </span>
                </>
              )}

              {/* Error */}
              {state === "error" && (
                <>
                  <span className="text-sm font-medium text-red-500">Error</span>
                  <span className="text-sm text-red-400 truncate max-w-[300px]">{error}</span>
                </>
              )}
            </div>

            {/* Transcript */}
            {(transcript && (isRecording || state === "done")) && (
              <motion.p
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: "auto" }}
                className="mt-2 text-[13px] leading-relaxed text-neutral-700 dark:text-neutral-300 max-h-32 overflow-y-auto"
              >
                {transcript}
              </motion.p>
            )}
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}

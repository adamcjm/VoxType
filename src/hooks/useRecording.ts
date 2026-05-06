import { useEffect } from "react";
import { useRecordingStore } from "../stores/recordingStore";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export function useRecording() {
  const startRecording = useRecordingStore((s) => s.startRecording);
  const setState = useRecordingStore((s) => s.setState);
  const setTranscript = useRecordingStore((s) => s.setTranscript);
  const setFinalText = useRecordingStore((s) => s.setFinalText);
  const setError = useRecordingStore((s) => s.setError);
  const reset = useRecordingStore((s) => s.reset);

  /** Start recording via Tauri backend */
  const start = async () => {
    try {
      startRecording();
      await invoke("start_recording");
    } catch (e) {
      setError(String(e));
    }
  };

  /** Stop recording and run pipeline */
  const stop = async () => {
    try {
      setState("transcribing");
      const text: string = await invoke("stop_recording");
      setFinalText(text);
      setTranscript(text);
      setState("done");
    } catch (e) {
      setError(String(e));
    }
  };

  /** Listen for backend events */
  useEffect(() => {
    // Listen for state updates from Rust (future: real-time transcript streaming)
    const unlistenState = listen("recording:state", (event) => {
      // Handle async state updates
      console.log("State event:", event.payload);
    });

    const unlistenTranscript = listen("recording:transcript", (event) => {
      const payload = event.payload as { text: string };
      setTranscript(payload.text);
    });

    return () => {
      unlistenState.then((fn) => fn());
      unlistenTranscript.then((fn) => fn());
    };
  }, []);

  return { start, stop, reset };
}

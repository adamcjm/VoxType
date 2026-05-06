import { create } from "zustand";

export type RecordingState =
  | "idle"
  | "recording"
  | "transcribing"
  | "polishing"
  | "done"
  | "error";

interface RecordingStore {
  isRecording: boolean;
  state: RecordingState;
  transcript: string;
  finalText: string;
  duration: number;
  error: string | null;

  startRecording: () => void;
  stopRecording: () => void;
  setState: (state: RecordingState) => void;
  setTranscript: (text: string) => void;
  setFinalText: (text: string) => void;
  setDuration: (ms: number) => void;
  setError: (msg: string) => void;
  reset: () => void;
}

export const useRecordingStore = create<RecordingStore>((set) => ({
  isRecording: false,
  state: "idle",
  transcript: "",
  finalText: "",
  duration: 0,
  error: null,

  startRecording: () =>
    set({
      isRecording: true,
      state: "recording",
      transcript: "",
      finalText: "",
      error: null,
    }),

  stopRecording: () =>
    set({ isRecording: false, state: "transcribing" }),

  setState: (state) => set({ state }),

  setTranscript: (transcript) => set({ transcript }),

  setFinalText: (finalText) => set({ finalText }),

  setDuration: (duration) => set({ duration }),

  setError: (error) => set({ state: "error", error }),

  reset: () =>
    set({
      isRecording: false,
      state: "idle",
      transcript: "",
      finalText: "",
      duration: 0,
      error: null,
    }),
}));

import { useRecordingStore } from "../../stores/recordingStore";

interface CapsuleProps {
  visible: boolean;
}

export default function Capsule({ visible }: CapsuleProps) {
  const state = useRecordingStore((s) => s.state);
  const transcript = useRecordingStore((s) => s.transcript);
  const error = useRecordingStore((s) => s.error);

  if (!visible) return null;

  return (
    <div className="fixed top-4 left-1/2 -translate-x-1/2 z-[9999] animate-slide-up">
      <div className="capsule-recording px-5 py-3 min-w-[320px] max-w-[480px]">
        <div className="flex items-center gap-3">
          <div className="relative w-3 h-3">
            <span
              className={`absolute inset-0 rounded-full ${
                state === "recording"
                  ? "bg-red-500 animate-pulse-glow"
                  : state === "transcribing" || state === "polishing"
                    ? "bg-amber-500 animate-pulse"
                    : state === "done"
                      ? "bg-green-500"
                      : state === "error"
                        ? "bg-red-500"
                        : "bg-neutral-400"
              }`}
            />
          </div>
          <span className="text-sm font-medium text-neutral-700 dark:text-neutral-200">
            {state === "recording" && "Recording..."}
            {state === "transcribing" && "Transcribing..."}
            {state === "polishing" && "Polishing..."}
            {state === "done" && "Done"}
            {state === "error" && "Error"}
            {state === "idle" && "Ready"}
          </span>
        </div>
        {transcript && (
          <p className="mt-2 text-sm text-neutral-600 dark:text-neutral-400 leading-relaxed max-h-32 overflow-y-auto">
            {transcript}
          </p>
        )}
        {error && (
          <p className="mt-2 text-xs text-red-500">{error}</p>
        )}
      </div>
    </div>
  );
}

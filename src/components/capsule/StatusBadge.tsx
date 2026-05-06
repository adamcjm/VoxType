import type { RecordingState } from "../../stores/recordingStore";

interface StatusBadgeProps {
  state: RecordingState;
}

export default function StatusBadge({ state }: StatusBadgeProps) {
  const colors: Record<RecordingState, string> = {
    recording: "bg-red-500 shadow-red-500/40 animate-pulse-glow",
    transcribing: "bg-amber-500 shadow-amber-500/40 animate-pulse",
    polishing: "bg-brand-500 shadow-brand-500/40 animate-pulse",
    done: "bg-green-500 shadow-green-500/40",
    error: "bg-red-500 shadow-red-500/40",
    idle: "bg-neutral-400 shadow-neutral-400/20",
  };

  return (
    <span
      className={`inline-block w-2.5 h-2.5 rounded-full shadow-[0_0_8px_var(--tw-shadow-color)] ${colors[state]}`}
    />
  );
}

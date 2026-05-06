import { useHistoryStore } from "../../stores/historyStore";

export default function History() {
  const items = useHistoryStore((s) => s.items);

  if (items.length === 0) return null;

  return (
    <div className="fixed bottom-4 right-4 z-[9997] max-w-sm">
      <div className="bg-white/90 dark:bg-neutral-900/90 backdrop-blur-xl rounded-2xl border border-neutral-200/50 dark:border-neutral-700/50 shadow-lg p-4">
        <h3 className="text-xs font-semibold text-neutral-500 dark:text-neutral-400 uppercase tracking-wider mb-2">
          Recent
        </h3>
        <div className="space-y-2 max-h-48 overflow-y-auto">
          {items.slice(0, 5).map((item) => (
            <div
              key={item.id}
              className="text-sm text-neutral-700 dark:text-neutral-300 leading-relaxed truncate"
            >
              {item.finalText || item.rawText}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

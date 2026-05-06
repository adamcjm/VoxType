import { useHistoryStore } from "../../stores/historyStore";

export default function History() {
  const items = useHistoryStore((s) => s.items);

  if (items.length === 0) return null;

  return (
    <div>
      <h3 className="text-[11px] font-semibold text-[#94A3B8] dark:text-neutral-500 uppercase tracking-wider mb-2">
        Recent
      </h3>
      <div className="space-y-2 max-h-40 overflow-y-auto">
        {items.slice(0, 5).map((item) => (
          <div
            key={item.id}
            className="px-3 py-2 rounded-xl bg-[#F8FAFC] dark:bg-neutral-900 border border-[#F1F5F9] dark:border-neutral-800"
          >
            <p className="text-[13px] leading-relaxed text-[#475569] dark:text-neutral-400 truncate">
              {item.finalText || item.rawText}
            </p>
            <p className="text-[11px] text-[#94A3B8] dark:text-neutral-600 mt-0.5">
              {item.sttProvider} · {item.mode}
            </p>
          </div>
        ))}
      </div>
    </div>
  );
}

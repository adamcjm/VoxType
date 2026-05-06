import { create } from "zustand";

export interface HistoryItem {
  id: string;
  rawText: string;
  finalText: string;
  sttProvider: string;
  llmProvider: string;
  appName: string;
  durationMs: number;
  mode: "cleanup" | "translate" | "format";
  createdAt: string;
}

interface HistoryStore {
  items: HistoryItem[];
  selectedId: string | null;
  searchQuery: string;

  addItem: (item: HistoryItem) => void;
  removeItem: (id: string) => void;
  selectItem: (id: string | null) => void;
  setSearchQuery: (query: string) => void;
  setItems: (items: HistoryItem[]) => void;
}

export const useHistoryStore = create<HistoryStore>((set) => ({
  items: [],
  selectedId: null,
  searchQuery: "",

  addItem: (item) =>
    set((s) => ({ items: [item, ...s.items].slice(0, 1000) })),

  removeItem: (id) =>
    set((s) => ({
      items: s.items.filter((i) => i.id !== id),
      selectedId: s.selectedId === id ? null : s.selectedId,
    })),

  selectItem: (selectedId) => set({ selectedId }),

  setSearchQuery: (searchQuery) => set({ searchQuery }),

  setItems: (items) => set({ items }),
}));

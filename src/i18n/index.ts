import { create } from "zustand";
import zh from "./zh-CN";
import en from "./en-US";
import type { Translations } from "./zh-CN";

export type Locale = "zh-CN" | "en-US";

const translations: Record<Locale, Translations> = {
  "zh-CN": zh,
  "en-US": en,
};

function detectLocale(): Locale {
  // Check stored preference
  try {
    const stored = localStorage.getItem("voxtype-locale");
    if (stored === "en-US" || stored === "zh-CN") return stored;
  } catch {}

  // Detect from system
  const langs = navigator.language || "en";
  if (langs.startsWith("zh")) return "zh-CN";
  return "en-US";
}

interface I18nStore {
  locale: Locale;
  setLocale: (l: Locale) => void;
}

export const useI18nStore = create<I18nStore>((set) => ({
  locale: detectLocale(),
  setLocale: (locale) => {
    try { localStorage.setItem("voxtype-locale", locale); } catch {}
    set({ locale });
  },
}));

/** Hook: use t() to get translations */
export function useT() {
  const locale = useI18nStore((s) => s.locale);
  return translations[locale];
}

/** Get t directly (for outside React) */
export function getT(): Translations {
  const locale = useI18nStore.getState().locale;
  return translations[locale];
}

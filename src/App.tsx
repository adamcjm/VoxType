import Settings from "./components/settings/Settings";
import History from "./components/history/History";
import { useSettingsStore } from "./stores/settingsStore";

export default function App() {
  const settingsOpen = useSettingsStore((s) => s.open);

  return (
    <div className="relative w-screen h-screen bg-neutral-50 dark:bg-neutral-950">
      {settingsOpen && <Settings />}
      <div className="p-6">
        <History />
      </div>
    </div>
  );
}

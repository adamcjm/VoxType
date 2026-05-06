import Capsule from "./components/capsule/Capsule";
import Settings from "./components/settings/Settings";
import History from "./components/history/History";
import { useSettingsStore } from "./stores/settingsStore";

export default function App() {
  const settingsOpen = useSettingsStore((s) => s.open);

  return (
    <div className="relative w-screen h-screen bg-transparent overflow-hidden">
      <Capsule />
      {settingsOpen && <Settings />}
      <History />
    </div>
  );
}

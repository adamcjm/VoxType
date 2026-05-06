import { useRecordingStore } from "./stores/recordingStore";
import { useSettingsStore } from "./stores/settingsStore";
import Capsule from "./components/capsule/Capsule";
import Settings from "./components/settings/Settings";
import History from "./components/history/History";

export default function App() {
  const isRecording = useRecordingStore((s) => s.isRecording);
  const settingsOpen = useSettingsStore((s) => s.open);

  return (
    <div className="relative w-screen h-screen bg-transparent">
      <Capsule visible={isRecording} />
      {settingsOpen && <Settings />}
      <History />
    </div>
  );
}

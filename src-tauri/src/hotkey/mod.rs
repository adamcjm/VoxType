// Global hotkey management
//
// macOS: Fn key (kVK_Function)
// Windows: Right Alt (VK_RMENU)
// Linux: Right Alt
//
// TODO: Implement with platform-specific hotkey registration

pub fn register_hotkey() {
    #[cfg(target_os = "macos")]
    {
        // TODO: Register Fn key via CGEvent or Carbon Event
    }

    #[cfg(target_os = "windows")]
    {
        // TODO: Register Right Alt via RegisterHotKey or SetWindowsHookEx
    }
}

pub fn unregister_hotkey() {
    // TODO
}

/// macOS global Fn key interception via CGEventTap.
///
/// Captures the Fn/globe key (keycode 63) before macOS handles it,
/// preventing the emoji picker. Toggles recording on each press.
///
/// Requires Accessibility permission (System Settings → Privacy → Accessibility).

use std::sync::atomic::{AtomicBool, Ordering};
use core_graphics::event::{
    CGEvent, CGEventTap, CGEventTapLocation, CGEventTapPlacement,
    CGEventTapOptions, CGEventType,
};
use tracing;

/// Toggle flag — set to true when Fn key is pressed, polled by main loop
static RECORDING_TOGGLE: AtomicBool = AtomicBool::new(false);

/// Check if the user pressed Fn (clears the flag)
pub fn poll_toggle() -> bool {
    RECORDING_TOGGLE.swap(false, Ordering::SeqCst)
}

/// Register CGEventTap for the Fn key.
/// Must be called from a thread with a CFRunLoop (e.g., Tauri setup thread).
pub fn register_fn_hotkey() {
    // Run this in a background thread that gets its own CFRunLoop
    std::thread::spawn(move || {
        let events = vec![CGEventType::KeyDown];

        let tap = match CGEventTap::new(
            CGEventTapLocation::HID,
            CGEventTapPlacement::HeadInsertEventTap,
            CGEventTapOptions::Default,
            events,
            move |_proxy, _event_type, event| -> Option<CGEvent> {
                let keycode = event.get_integer_value_field(
                    core_graphics::event::EventField::KEYBOARD_EVENT_KEYCODE,
                );

                // Fn key = keycode 63
                if keycode == 63 {
                    tracing::info!("Fn key pressed — toggling recording");
                    RECORDING_TOGGLE.store(true, Ordering::SeqCst);
                    // Return None to consume the event (block emoji picker)
                    return None;
                }

                // Pass through all other events
                Some(event.clone())
            },
        ) {
            Ok(t) => t,
            Err(()) => {
                tracing::error!(
                    "CGEventTap failed. Grant Accessibility permission: \
                     System Settings → Privacy & Security → Accessibility"
                );
                return;
            }
        };

        // Enable and add to run loop
        let run_loop_source = tap
            .mach_port
            .create_runloop_source(0)
            .expect("Failed to create runloop source");

        let run_loop = core_foundation::runloop::CFRunLoop::get_current();
        run_loop.add_source(
            &run_loop_source,
            unsafe { core_foundation::runloop::kCFRunLoopCommonModes },
        );

        tap.enable();

        tracing::info!("VoxType Fn key hotkey active");

        // Run the event loop — blocks until tap is invalidated
        core_foundation::runloop::CFRunLoop::run_current();

        tracing::info!("CGEventTap event loop exited");
    });
}

use objc2_app_kit::{NSWindow, NSWindowButton};
use objc2_foundation::NSPoint;
use tauri::Manager;

const CONTROLS: [NSWindowButton; 3] = [
    NSWindowButton::CloseButton,
    NSWindowButton::MiniaturizeButton,
    NSWindowButton::ZoomButton,
];

/// macOS centres the controls 16pt from the top of the title bar strip, sized
/// for a plain window. The sidebar's control row centres at 22pt, so the lights
/// are nudged down to share its line. Only the vertical position is touched: the
/// horizontal margins are the platform's, and moving them would crowd our own
/// toggle, which clears the lights by design.
const CONTROL_CENTRE_FROM_TOP: f64 = 22.0;

/// Hides or shows the native window controls, to match the sidebar's collapsed
/// state: with the sidebar out of the way the lights have nothing to sit beside.
pub fn set_visible(window: &tauri::WebviewWindow, visible: bool) {
    let Some(ns_window) = window_handle(window) else {
        return;
    };
    for control in CONTROLS {
        if let Some(button) = ns_window.standardWindowButton(control) {
            button.setHidden(!visible);
        }
    }
}

/// Moves the controls onto the sidebar's control row. AppKit lays these buttons
/// out itself, so this is re-applied on resize rather than set once — and it is
/// skipped in fullscreen, where the system owns their placement.
pub fn align(window: &tauri::WebviewWindow) {
    if window.is_fullscreen().unwrap_or(false) {
        return;
    }
    let Some(ns_window) = window_handle(window) else {
        return;
    };

    for control in CONTROLS {
        let Some(button) = ns_window.standardWindowButton(control) else {
            continue;
        };
        // The buttons live in the title bar strip, which is far shorter than the
        // window — measuring against the window would fling them off-screen.
        let Some(strip) = (unsafe { button.superview() }) else {
            continue;
        };
        let strip_height = strip.frame().size.height;
        if strip_height < 20.0 {
            // Not laid out yet; a later resize will bring us back here.
            continue;
        }

        let current = button.frame();
        // The strip's origin is bottom-left, so a distance from the top has to be
        // converted before it can be applied.
        button.setFrameOrigin(NSPoint {
            x: current.origin.x,
            y: strip_height - CONTROL_CENTRE_FROM_TOP - current.size.height / 2.0,
        });
    }
}

/// AppKit lays the title bar out on the pass *after* whatever changed it, so
/// aligning immediately can be undone a frame later. This clears that layout
/// first, then does the work back on the main thread, where AppKit requires it.
const REALIGN_DELAY: std::time::Duration = std::time::Duration::from_millis(60);

pub fn align_soon(window: &tauri::WebviewWindow) {
    let handle = window.app_handle().clone();
    let app = window.app_handle().clone();
    std::thread::spawn(move || {
        std::thread::sleep(REALIGN_DELAY);
        let _ = handle.run_on_main_thread(move || {
            if let Some(window) = app.get_webview_window("main") {
                align(&window);
            }
        });
    });
}

fn window_handle(window: &tauri::WebviewWindow) -> Option<&NSWindow> {
    let handle = window.ns_window().ok()?;
    // SAFETY: `ns_window` hands back this app's own window, which outlives the
    // call, and AppKit requires it be touched on the main thread — which is
    // where Tauri runs synchronous commands and window events.
    Some(unsafe { &*handle.cast() })
}

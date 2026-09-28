use tauri::{App, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

pub const LABEL: &str = "capture";

/// Cmd/Ctrl-based rather than Option/Alt, for the same dead-key and IME reason
/// the rest of the app's shortcuts avoid Option.
const DEFAULT_SHORTCUT: &str = "CmdOrCtrl+Shift+K";

/// Room for one field and its hint line, and no more — this is a capture box,
/// not a second app.
const WIDTH: f64 = 560.0;
const HEIGHT: f64 = 104.0;

/// Builds the capture window hidden and binds the global shortcut that reveals
/// it. The window is a real route rather than a hash or query, which is what
/// makes it behave the same in a dev server and a packaged build.
pub fn install(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let window = WebviewWindowBuilder::new(app, LABEL, WebviewUrl::App("capture".into()))
        .title("Quick capture")
        .decorations(false)
        .resizable(false)
        .always_on_top(true)
        .visible(false)
        .inner_size(WIDTH, HEIGHT)
        .build()?;

    // Clicking anywhere else dismisses it, which is what makes a two-keystroke
    // capture feel like it is not there at all.
    let dismiss = window.clone();
    window.on_window_event(move |event| {
        if matches!(event, tauri::WindowEvent::Focused(false)) {
            let _ = dismiss.hide();
        }
    });

    app.global_shortcut()
        .on_shortcut(DEFAULT_SHORTCUT, |app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                reveal(app);
            }
        })?;

    Ok(())
}

pub fn reveal(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.center();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

pub fn hide(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.hide();
    }
}

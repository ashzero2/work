use tauri::{App, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub const LABEL: &str = "settings";

/// Built once and hidden, then shown on demand: ⌘, is the way in, and Apple's
/// convention is that there is only ever one settings window.
///
/// The minimize and zoom buttons are disabled the way the HIG asks, precisely
/// because ⌘, makes the window easy to bring back, so there is nothing to gain
/// from shrinking it — and it keeps the window from joining the window cycle.
pub fn install(app: &App) -> tauri::Result<()> {
    let window = WebviewWindowBuilder::new(app, LABEL, WebviewUrl::App("settings".into()))
        .title("Work Dashboard Settings")
        .inner_size(680.0, 560.0)
        .min_inner_size(560.0, 420.0)
        .minimizable(false)
        .maximizable(false)
        .visible(false)
        .center()
        .build()?;

    // Closing hides it, so the next ⌘, is instant and the pane the user was last
    // on is still there.
    let hide_on_close = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            let _ = hide_on_close.hide();
        }
    });

    Ok(())
}

/// Brings the window forward. Showing before focusing, or the app can end up
/// active with nothing on screen.
pub fn show(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

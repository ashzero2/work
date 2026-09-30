use std::sync::Mutex;

use rusqlite::Connection;
use serde::Serialize;
use tauri::{App, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent, ShortcutState};

use crate::error::AppError;
use crate::storage::settings;

pub const LABEL: &str = "capture";

/// Where the user's choice lives. The default is what applies before they make
/// one, and the fallback when their choice can't be bound.
const SHORTCUT_KEY: &str = "shortcut.quick_capture";
pub const DEFAULT_SHORTCUT: &str = "CmdOrCtrl+Shift+K";

/// Room for one field and its hint line, and no more — this is a capture box,
/// not a second app.
const WIDTH: f64 = 560.0;
const HEIGHT: f64 = 230.0;

/// What is bound right now, so a change can release it first: the plugin errors
/// on registering a combination it already holds rather than replacing it.
static BOUND: Mutex<Option<String>> = Mutex::new(None);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutStatus {
    /// What is actually bound, or `None` when nothing could be.
    pub shortcut: Option<String>,
    /// What the user asked for, which is what the recorder should show even when
    /// it could not be bound this time.
    pub preferred: String,
    /// What resetting returns to.
    pub default: String,
}

/// Builds the capture window hidden and binds the global shortcut that reveals
/// it. The window is a real route rather than a hash or query, which is what
/// makes it behave the same in a dev server and a packaged build.
pub fn install(app: &App) -> std::result::Result<(), Box<dyn std::error::Error>> {
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

    // The stored choice is tried first, then the default. The preference is left
    // alone either way: a conflict with another app today doesn't mean the same
    // combination should be discarded for good.
    let handle = app.handle().clone();
    let preferred = preference(&handle);
    if let Err(error) = rebind(&handle, &preferred) {
        eprintln!("quick capture shortcut {preferred} unavailable: {error}");
        if let Err(error) = rebind(&handle, DEFAULT_SHORTCUT) {
            eprintln!("quick capture has no global shortcut: {error}");
        }
    }

    Ok(())
}

/// Binds `shortcut`, releasing whatever was bound before. The new combination is
/// bound before the old is released, so a refusal leaves the previous key working
/// rather than leaving the user with nothing.
pub fn rebind(app: &AppHandle, shortcut: &str) -> std::result::Result<(), String> {
    let mut bound = BOUND.lock().unwrap();

    if bound.as_deref() != Some(shortcut) {
        app.global_shortcut()
            .on_shortcut(shortcut, pressed)
            .map_err(|error| error.to_string())?;
    }

    if let Some(previous) = bound.as_deref().filter(|previous| *previous != shortcut) {
        let _ = app.global_shortcut().unregister(previous);
    }

    *bound = Some(shortcut.to_string());
    Ok(())
}

/// The one thing a bound combination does. Shared so binding and rebinding can't
/// drift apart.
fn pressed(app: &AppHandle, _shortcut: &Shortcut, event: ShortcutEvent) {
    if event.state() == ShortcutState::Pressed {
        reveal(app);
    }
}

/// The user's stored choice, or the default before they make one.
pub fn preference(app: &AppHandle) -> String {
    let state = app.state::<Mutex<Connection>>();
    let Ok(conn) = state.lock() else {
        return DEFAULT_SHORTCUT.to_string();
    };

    settings::get(&conn, SHORTCUT_KEY)
        .ok()
        .flatten()
        .unwrap_or_else(|| DEFAULT_SHORTCUT.to_string())
}

pub fn status(app: &AppHandle) -> ShortcutStatus {
    ShortcutStatus {
        shortcut: BOUND.lock().unwrap().clone(),
        preferred: preference(app),
        default: DEFAULT_SHORTCUT.to_string(),
    }
}

/// Rebinds and remembers. A refused combination is reported and *not* stored, so
/// the app is never left claiming a shortcut it doesn't have.
pub fn set_preference(
    app: &AppHandle,
    conn: &Connection,
    shortcut: &str,
) -> crate::error::Result<ShortcutStatus> {
    let shortcut = shortcut.trim();
    if shortcut.is_empty() {
        return Err(AppError::InvalidInput("a shortcut can't be empty".into()));
    }

    rebind(app, shortcut).map_err(|error| {
        AppError::InvalidInput(format!("{shortcut} couldn't be registered — {error}"))
    })?;

    settings::set(conn, SHORTCUT_KEY, shortcut)?;
    Ok(status(app))
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

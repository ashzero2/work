use std::sync::OnceLock;

#[cfg(target_os = "macos")]
mod accent;
#[cfg(target_os = "macos")]
mod notifications;
#[cfg(target_os = "macos")]
mod traffic_lights;
#[cfg(target_os = "macos")]
mod vibrancy;

/// The action identifier and the request it came from, reported when the user
/// presses a notification's button.
pub type NotificationActionHandler = Box<dyn Fn(&str, &str) + Send + Sync>;

/// Notification action identifiers. Defined here rather than next to the macOS
/// code so the handler that maps an action back onto reminder state can name
/// them on every platform.
pub const ACTION_SNOOZE: &str = "tittle.snooze";
pub const ACTION_DISMISS: &str = "tittle.dismiss";

/// What the user answered when asked for permission. `None` until they have been
/// asked, which is what lets the UI say "we asked and you said no" rather than
/// quietly doing nothing.
static AUTHORIZED: OnceLock<bool> = OnceLock::new();

/// The macOS accent colour as `#RRGGBB`, or `None` where the platform has no
/// such notion or the system value can't be converted.
pub fn accent_color() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        accent::accent_color()
    }
    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

/// Applies the platform's translucent window material. A no-op off macOS, and
/// deliberately applied to the whole window rather than a region: only the
/// sidebar is ever painted transparent, so it can never show behind content.
pub fn apply_window_material(window: &tauri::WebviewWindow) {
    #[cfg(target_os = "macos")]
    vibrancy::apply(window);
    #[cfg(not(target_os = "macos"))]
    let _ = window;
}

/// Applies the popover material behind the quick-capture window. A no-op off
/// macOS. The window is transparent, so this is what actually paints it; the page
/// keeps a translucent surface on top so the app theme still leads the colour.
pub fn apply_capture_material(window: &tauri::WebviewWindow) {
    #[cfg(target_os = "macos")]
    vibrancy::apply_popover(window);
    #[cfg(not(target_os = "macos"))]
    let _ = window;
}

/// Shows or hides the native window controls. A no-op off macOS, where the
/// window keeps its own decorations.
pub fn set_traffic_lights_visible(window: &tauri::WebviewWindow, visible: bool) {
    #[cfg(target_os = "macos")]
    traffic_lights::set_visible(window, visible);
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (window, visible);
    }
}

/// Nudges the native window controls onto the sidebar's control row so the two
/// share a centre line, deferred past AppKit's own title-bar layout. Required
/// after anything that makes it re-lay the bar out — showing the controls again
/// after a collapse, or a window resize — since aligning first is undone. A
/// no-op off macOS.
pub fn align_traffic_lights_soon(window: &tauri::WebviewWindow) {
    #[cfg(target_os = "macos")]
    traffic_lights::align_soon(window);
    #[cfg(not(target_os = "macos"))]
    let _ = window;
}

/// True where the OS can actually deliver notifications. False off macOS, and
/// false whenever the process runs without a bundle identifier (`tauri dev`),
/// where the framework raises rather than failing quietly.
pub fn notifications_available() -> bool {
    #[cfg(target_os = "macos")]
    {
        notifications::available()
    }
    #[cfg(not(target_os = "macos"))]
    {
        false
    }
}

/// Installs the notification delegate and asks for permission. A no-op
/// wherever notifications aren't available.
pub fn start_notifications(handler: NotificationActionHandler) {
    #[cfg(target_os = "macos")]
    {
        notifications::install(handler);
        notifications::request_authorization(Box::new(|granted| {
            let _ = AUTHORIZED.set(granted);
        }));
    }
    #[cfg(not(target_os = "macos"))]
    let _ = handler;
}

/// The answer to the permission prompt: `None` while it is still open or was
/// never asked, which is a different state from a refusal.
pub fn notifications_authorized() -> Option<bool> {
    AUTHORIZED.get().copied()
}

/// Opens the OS notification settings pane. Reaching permission here is the
/// recovery path when a reminder would otherwise never appear.
pub fn open_notification_settings() {
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.notifications")
            .spawn();
    }
}

/// Posts a notification carrying the Snooze/Dismiss buttons.
pub fn post_notification(identifier: &str, title: &str, body: &str, after_seconds: f64) {
    #[cfg(target_os = "macos")]
    notifications::post(identifier, title, body, after_seconds);
    #[cfg(not(target_os = "macos"))]
    let _ = (identifier, title, body, after_seconds);
}

/// Drops pending notifications that are no longer wanted.
pub fn cancel_notifications(identifiers: &[String]) {
    #[cfg(target_os = "macos")]
    notifications::cancel(identifiers);
    #[cfg(not(target_os = "macos"))]
    let _ = identifiers;
}

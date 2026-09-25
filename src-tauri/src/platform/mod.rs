#[cfg(target_os = "macos")]
mod accent;
#[cfg(target_os = "macos")]
mod traffic_lights;
#[cfg(target_os = "macos")]
mod vibrancy;

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

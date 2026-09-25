use tauri::WebviewWindow;

/// The user's system accent colour, so the platform theme can mirror it. Kept
/// deliberately narrow: the frontend detects the platform itself, so a failed
/// colour read can never masquerade as "not macOS".
#[tauri::command]
pub fn system_accent() -> Option<String> {
    crate::platform::accent_color()
}

/// Matches the window controls to the sidebar being collapsed. Showing them
/// again makes AppKit re-lay the title bar out, so the alignment is re-applied
/// afterwards rather than being left at the stock position.
#[tauri::command]
pub fn set_traffic_lights_visible(window: WebviewWindow, visible: bool) {
    crate::platform::set_traffic_lights_visible(&window, visible);
    crate::platform::align_traffic_lights_soon(&window);
}

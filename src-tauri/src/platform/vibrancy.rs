use window_vibrancy::{NSVisualEffectMaterial, NSVisualEffectState, apply_vibrancy};

/// The system's sidebar material, following the window's active state so it
/// dims like a native window when the app loses focus. Without a transparent
/// window this fails, and the sidebar would show the page background instead of
/// the material, so a failure is reported rather than swallowed.
pub fn apply(window: &tauri::WebviewWindow) {
    let result = apply_vibrancy(
        window,
        NSVisualEffectMaterial::Sidebar,
        Some(NSVisualEffectState::FollowsWindowActiveState),
        None,
    );
    if let Err(error) = result {
        eprintln!("window material unavailable: {error}");
    }
}

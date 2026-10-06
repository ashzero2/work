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

/// The transient-popover material, for the quick-capture window. This is the one
/// place a material is semantically right rather than decorative — it is a
/// popover — and it follows the system appearance the way a native one does.
/// `Active` rather than following focus, since the window only exists while it is
/// being used and dimming it on a stray click would look broken.
pub fn apply_popover(window: &tauri::WebviewWindow) {
    let result = apply_vibrancy(
        window,
        NSVisualEffectMaterial::Popover,
        Some(NSVisualEffectState::Active),
        None,
    );
    if let Err(error) = result {
        eprintln!("capture material unavailable: {error}");
    }
}

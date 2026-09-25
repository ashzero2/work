use gpui_kit::component::{Theme, ThemeMode};
use gpui_kit::*;

/// Adopts the operating system's light/dark appearance for the first frame.
pub fn adopt_system_appearance(cx: &mut App) {
    Theme::sync_system_appearance(None, cx);
}

/// Flips the application between its light and dark themes.
pub fn toggle_mode(window: &mut Window, cx: &mut App) {
    let next = if Theme::global(cx).is_dark() {
        ThemeMode::Light
    } else {
        ThemeMode::Dark
    };
    Theme::change(next, Some(window), cx);
}

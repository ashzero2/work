use tauri::menu::{Menu, MenuItem};
use tauri::{AppHandle, Runtime};

/// The id the Settings item is registered under, shared by the builder and the
/// handler so they cannot drift apart.
pub const SETTINGS_ID: &str = "settings";

/// The app menu, with a Settings item where macOS expects it: directly under
/// About, above Services.
///
/// Tauri's default menu is **extended rather than rebuilt**, because supplying a
/// menu replaces the default one outright — and a hand-rolled menu that forgot
/// the Edit items would quietly take ⌘C, ⌘V and ⌘Z away from every text field.
pub fn build<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let menu = Menu::default(app)?;
    let settings = MenuItem::with_id(app, SETTINGS_ID, "Settings…", true, Some("CmdOrCtrl+,"))?;

    if let Some(submenu) = menu.items()?.first().and_then(|item| item.as_submenu()) {
        submenu.insert(&settings, 1)?;
    }

    Ok(menu)
}

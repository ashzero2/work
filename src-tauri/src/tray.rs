use std::sync::Mutex;
use std::time::Duration;

use chrono::{DateTime, Utc};
use rusqlite::Connection;
use tauri::image::Image;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{App, AppHandle, Manager};

use crate::domain::PomodoroSession;
use crate::storage::pomodoro;

pub const MAIN_LABEL: &str = "main";

const TRAY_ID: &str = "main";
const OPEN_ID: &str = "open";
const QUIT_ID: &str = "quit";
const TICK: Duration = Duration::from_secs(1);

/// Adds the menu-bar item: a template icon with the focus countdown beside it,
/// and a menu carrying the two actions the tray needs. Behaviour hangs off the
/// menu rather than the icon's click event, which macOS has a history of not
/// delivering.
pub fn install(app: &App) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, OPEN_ID, "Open Tittle", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, QUIT_ID, "Quit Tittle", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &PredefinedMenuItem::separator(app)?, &quit])?;

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(Image::from_bytes(include_bytes!("../icons/tray.png"))?)
        .icon_as_template(true)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id().as_ref() {
            OPEN_ID => show_main(app),
            QUIT_ID => app.exit(0),
            _ => {}
        })
        .build(app)?;

    Ok(())
}

/// Brings the window back and the app forward with it. `set_focus` alone does not
/// raise an app macOS has hidden, so the app is shown first.
pub fn show_main(app: &AppHandle) {
    #[cfg(target_os = "macos")]
    let _ = app.show();

    if let Some(window) = app.get_webview_window(MAIN_LABEL) {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

/// Closing the window hides it rather than ending the app: the tray is the way
/// back and Cmd+Q is the way out. Because the window is only hidden and never
/// destroyed, the "all windows closed" exit path never runs — which is also why
/// Quit keeps working, since on macOS that path is not reliably reached from
/// Cmd+Q at all.
pub fn hide_main(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(MAIN_LABEL) {
        let _ = window.hide();
    }
    #[cfg(target_os = "macos")]
    let _ = app.hide();
}

/// Keeps the menu-bar countdown in step with the open session. The remaining time
/// is derived from the session's start rather than counted down, so a throttled
/// timer or a sleeping machine cannot make the display drift, and the tray is
/// only touched when the text actually changes.
pub fn spawn_countdown(app: AppHandle) {
    std::thread::spawn(move || {
        let mut shown: Option<Option<String>> = None;
        loop {
            std::thread::sleep(TICK);

            let next = title_for_open_session(&app, Utc::now());
            if shown.as_ref() == Some(&next) {
                continue;
            }
            if let Some(tray) = app.tray_by_id(TRAY_ID) {
                let _ = tray.set_title(next.as_deref());
            }
            shown = Some(next);
        }
    });
}

fn title_for_open_session(app: &AppHandle, now: DateTime<Utc>) -> Option<String> {
    let state = app.state::<Mutex<Connection>>();
    let conn = state.lock().ok()?;
    let session = pomodoro::open(&conn).ok().flatten();
    countdown_title(session.as_ref(), now)
}

/// `MM:SS` while a session runs, nothing while idle.
pub fn countdown_title(session: Option<&PomodoroSession>, now: DateTime<Utc>) -> Option<String> {
    let session = session?;
    let remaining = (session.planned_seconds - (now - session.started_at).num_seconds()).max(0);
    Some(format!("{:02}:{:02}", remaining / 60, remaining % 60))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::SessionKind;

    fn started_ago(seconds: i64, planned_seconds: i64) -> PomodoroSession {
        PomodoroSession {
            id: 1,
            task_id: None,
            kind: SessionKind::Work,
            planned_seconds,
            started_at: Utc::now() - chrono::Duration::seconds(seconds),
            ended_at: None,
            completed: false,
        }
    }

    #[test]
    fn nothing_is_shown_while_idle() {
        assert!(countdown_title(None, Utc::now()).is_none());
    }

    #[test]
    fn a_running_session_counts_down() {
        let now = Utc::now();
        let session = PomodoroSession {
            started_at: now - chrono::Duration::seconds(29),
            ..started_ago(0, 1500)
        };

        assert_eq!(countdown_title(Some(&session), now).unwrap(), "24:31");
    }

    #[test]
    fn minutes_and_seconds_are_always_two_digits() {
        let now = Utc::now();
        let session = PomodoroSession {
            started_at: now,
            ..started_ago(0, 300)
        };

        assert_eq!(countdown_title(Some(&session), now).unwrap(), "05:00");
    }

    #[test]
    fn an_expired_session_rests_at_zero_instead_of_going_negative() {
        let now = Utc::now();
        let session = PomodoroSession {
            started_at: now - chrono::Duration::seconds(2000),
            ..started_ago(0, 1500)
        };

        assert_eq!(countdown_title(Some(&session), now).unwrap(), "00:00");
    }
}

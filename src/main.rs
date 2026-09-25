use gpui_kit::component::Root;
use gpui_kit::*;
use work_dashboard::storage::{columns, db};
use work_dashboard::ui::workspace::Workspace;

fn main() {
    let conn = db::open().expect("failed to open database");
    columns::ensure_default(&conn).expect("failed to ensure a default column");

    gpui_kit::application()
        .with_assets(gpui_kit::assets::AllAssets)
        .run(move |cx| {
            gpui_kit::init(cx);

            cx.spawn(async move |cx| {
                cx.open_window(WindowOptions::default(), |window, cx| {
                    let view = cx.new(|cx| Workspace::new(conn, window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                })
                .expect("failed to open window");
            })
            .detach();
        });
}

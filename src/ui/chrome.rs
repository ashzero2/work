use gpui_kit::assets::IconName;
use gpui_kit::component::button::*;
use gpui_kit::component::input::Input;
use gpui_kit::component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_kit::component::separator::Separator;
use gpui_kit::component::tab::{Tab, TabBar};
use gpui_kit::component::{ActiveTheme as _, Icon, Sizable as _, *};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use crate::ui::theme;
use crate::ui::workspace::{ViewMode, Workspace};

/// The window's title row: app identity, a task summary, the view switch and
/// the light/dark toggle.
pub fn header(workspace: &Workspace, cx: &mut Context<Workspace>) -> impl IntoElement {
    let (border, muted, primary, primary_foreground) = {
        let t = cx.theme();
        (
            t.border,
            t.muted_foreground,
            t.primary,
            t.primary_foreground,
        )
    };
    let is_dark = cx.theme().is_dark();

    let open = workspace
        .tasks()
        .iter()
        .filter(|t| t.completed_at.is_none() && t.parent_task_id.is_none())
        .count();
    let done = workspace
        .tasks()
        .iter()
        .filter(|t| t.completed_at.is_some())
        .count();
    let summary = if open == 0 && done == 0 {
        "No tasks yet".to_string()
    } else {
        format!("{open} open · {done} done")
    };

    h_flex()
        .h(px(56.))
        .flex_shrink_0()
        .items_center()
        .justify_between()
        .gap_4()
        .px_6()
        .border_b_1()
        .border_color(border)
        .child(
            h_flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .size_8()
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_lg()
                        .bg(primary)
                        .text_color(primary_foreground)
                        .child(Icon::new(IconName::Kanban).small()),
                )
                .child(
                    v_flex()
                        .gap_0p5()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .child("Tasks"),
                        )
                        .child(div().text_xs().text_color(muted).child(summary)),
                ),
        )
        .child(
            h_flex()
                .items_center()
                .gap_3()
                .child(view_tabs(workspace, cx))
                .child(Separator::vertical().h_5())
                .child(
                    Button::new("theme-toggle")
                        .ghost()
                        .icon(if is_dark {
                            IconName::Sun
                        } else {
                            IconName::Moon
                        })
                        .tooltip(if is_dark {
                            "Use light mode"
                        } else {
                            "Use dark mode"
                        })
                        .on_click(cx.listener(|_, _, window, cx| theme::toggle_mode(window, cx))),
                ),
        )
}

/// The composer row: the quick-add input, the column it targets, the add
/// action and the board-only column action.
pub fn toolbar(workspace: &Workspace, cx: &mut Context<Workspace>) -> impl IntoElement {
    let border = cx.theme().border;
    let is_board = workspace.view_mode() == ViewMode::Board;

    h_flex()
        .flex_shrink_0()
        .items_center()
        .gap_2()
        .px_6()
        .py_3()
        .border_b_1()
        .border_color(border)
        .child(
            div()
                .flex_1()
                .min_w_0()
                .child(Input::new(workspace.new_task_input()).cleanable(true)),
        )
        .child(target_column_picker(workspace, cx))
        .child(
            Button::new("add-task")
                .primary()
                .icon(IconName::Plus)
                .label("Add task")
                .on_click(cx.listener(|this, _, window, cx| this.add_task(window, cx))),
        )
        .when(is_board, |el| {
            el.child(
                Button::new("add-column")
                    .ghost()
                    .icon(IconName::Plus)
                    .label("Add column")
                    .on_click(
                        cx.listener(|this, _, window, cx| this.open_add_column_dialog(window, cx)),
                    ),
            )
        })
}

fn view_tabs(workspace: &Workspace, cx: &mut Context<Workspace>) -> impl IntoElement {
    let selected = match workspace.view_mode() {
        ViewMode::Board => 0,
        ViewMode::List => 1,
    };

    TabBar::new("view-tabs")
        .segmented()
        .selected_index(selected)
        .on_click(cx.listener(|this, index: &usize, _, cx| {
            let mode = if *index == 0 {
                ViewMode::Board
            } else {
                ViewMode::List
            };
            this.set_view_mode(mode, cx);
        }))
        .children([
            Tab::new().label("Board").icon(IconName::Columns3),
            Tab::new().label("List").icon(IconName::List),
        ])
}

fn target_column_picker(workspace: &Workspace, cx: &mut Context<Workspace>) -> impl IntoElement {
    let weak = cx.entity().downgrade();
    let columns: Vec<(i64, String)> = workspace
        .columns()
        .iter()
        .map(|column| (column.id, column.name.clone()))
        .collect();
    let current = workspace
        .target_column_name()
        .unwrap_or_else(|| "No column".to_string());
    let has_columns = !columns.is_empty();

    Button::new("target-column")
        .outline()
        .dropdown_caret(true)
        .label(current)
        .tooltip("New tasks are added to this column")
        .disabled(!has_columns)
        .dropdown_menu(move |mut menu, _, _| {
            for (id, name) in &columns {
                let id = *id;
                let name = name.clone();
                let weak = weak.clone();
                menu = menu.item(PopupMenuItem::new(name).on_click(move |_, _, cx| {
                    weak.update(cx, |this, cx| this.set_target_column(id, cx))
                        .ok();
                }));
            }
            menu
        })
}

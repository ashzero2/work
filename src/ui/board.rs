use gpui_kit::assets::IconName;
use gpui_kit::component::button::*;
use gpui_kit::component::empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use gpui_kit::component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::component::{ActiveTheme as _, Icon, Sizable as _, *};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use crate::domain::{Column, Task};
use crate::ui::task_meta;
use crate::ui::workspace::{TaskDrag, Workspace};

/// Renders every column as a kanban lane over the same task data the list
/// view uses — one data model, two renderers.
pub fn render(workspace: &Workspace, cx: &mut Context<Workspace>) -> impl IntoElement {
    let columns = workspace.columns().to_vec();
    let tasks = workspace.tasks().to_vec();
    let weak = cx.entity().downgrade();

    let mut lanes: Vec<AnyElement> = Vec::new();
    for column in &columns {
        lanes.push(render_column(column, &tasks, weak.clone(), cx).into_any_element());
    }
    if lanes.is_empty() {
        lanes.push(empty_state(cx).into_any_element());
    }

    div()
        .size_full()
        .overflow_scrollbar()
        .p_6()
        .child(h_flex().items_start().gap_4().children(lanes))
}

fn render_column(
    column: &Column,
    all_tasks: &[Task],
    weak: WeakEntity<Workspace>,
    cx: &mut Context<Workspace>,
) -> impl IntoElement {
    let column_id = column.id;
    let column_tasks: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| {
            t.column_id == column_id && t.completed_at.is_none() && t.parent_task_id.is_none()
        })
        .collect();

    let (lane, card_bg, muted, danger) = {
        let t = cx.theme();
        (t.muted, t.background, t.muted_foreground, t.danger)
    };
    let over_wip = column
        .wip_limit
        .map(|limit| column_tasks.len() as i64 > limit)
        .unwrap_or(false);

    let count_label = match column.wip_limit {
        Some(limit) => format!("{}/{}", column_tasks.len(), limit),
        None => column_tasks.len().to_string(),
    };

    let mut cards: Vec<AnyElement> = Vec::new();
    for task in column_tasks.iter().copied() {
        let (done, total) = subtask_progress(task.id, all_tasks);
        cards.push(render_card(task, column_id, done, total, cx).into_any_element());
    }
    let has_cards = !cards.is_empty();

    v_flex()
        .id(("column", column_id as u64))
        .w(px(300.))
        .flex_shrink_0()
        .gap_2()
        .p_2()
        .rounded_lg()
        .bg(lane)
        .child(
            h_flex()
                .items_center()
                .justify_between()
                .gap_2()
                .px_1()
                .pt_1()
                .child(
                    h_flex()
                        .flex_1()
                        .min_w_0()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .overflow_hidden()
                                .whitespace_nowrap()
                                .text_ellipsis()
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .child(column.name.clone()),
                        )
                        .child(
                            div()
                                .flex_shrink_0()
                                .px_1p5()
                                .rounded_full()
                                .text_xs()
                                .when(over_wip, |el| {
                                    el.bg(danger.opacity(0.15)).text_color(danger)
                                })
                                .when(!over_wip, |el| el.bg(card_bg).text_color(muted))
                                .child(count_label),
                        ),
                )
                .child(column_menu(column_id, weak, cx)),
        )
        .child(v_flex().gap_2().children(cards).when(!has_cards, |el| {
            el.child(
                div()
                    .flex()
                    .justify_center()
                    .py_5()
                    .text_xs()
                    .text_color(muted)
                    .child("No tasks"),
            )
        }))
        .child(
            // Empty, borderless spacer — dropping a card anywhere in here
            // appends it to the end of this column.
            div()
                .id(("column-drop-end", column_id as u64))
                .h(px(28.))
                .on_drop(cx.listener(move |this, drag: &TaskDrag, _, cx| {
                    this.move_task_to_end(drag.task_id, column_id, cx);
                })),
        )
}

fn column_menu(
    column_id: i64,
    weak: WeakEntity<Workspace>,
    _cx: &mut Context<Workspace>,
) -> impl IntoElement {
    Button::new(("column-menu", column_id as u64))
        .icon(IconName::Ellipsis)
        .ghost()
        .xsmall()
        .tooltip("Column actions")
        .dropdown_menu(move |menu, _, _| {
            let rename = weak.clone();
            let delete = weak.clone();
            menu.item(
                PopupMenuItem::new("Rename")
                    .icon(IconName::Pencil)
                    .on_click(move |_, window, cx| {
                        rename
                            .update(cx, |this, cx| {
                                this.open_rename_column_dialog(column_id, window, cx)
                            })
                            .ok();
                    }),
            )
            .separator()
            .item(
                PopupMenuItem::element(|_, cx| div().text_color(cx.theme().danger).child("Delete"))
                    .icon(IconName::Delete)
                    .on_click(move |_, window, cx| {
                        delete
                            .update(cx, |this, cx| this.delete_column(column_id, window, cx))
                            .ok();
                    }),
            )
        })
}

fn render_card(
    task: &Task,
    column_id: i64,
    subtasks_done: usize,
    subtasks_total: usize,
    cx: &mut Context<Workspace>,
) -> impl IntoElement {
    let id = task.id;
    let position = task.position;
    let title = task.title.clone();
    let group: SharedString = format!("card-{id}").into();

    let mut meta: Vec<AnyElement> = Vec::new();
    if let Some(tag) = task_meta::priority_tag(task.priority) {
        meta.push(tag);
    }
    if let Some(due) = task.due_at {
        meta.push(task_meta::due_chip(due, cx));
    }
    if let Some(rule) = task.repeat_rule {
        meta.push(task_meta::repeat_chip(rule, cx));
    }
    if let Some(chip) = task_meta::subtask_chip(subtasks_done, subtasks_total, cx) {
        meta.push(chip);
    }
    let has_meta = !meta.is_empty();

    let (card_bg, border, hover_border) = {
        let t = cx.theme();
        (t.background, t.border, t.primary.opacity(0.35))
    };

    div()
        .id(("card", id as u64))
        .group(group.clone())
        .v_flex()
        .gap_2()
        .p_3()
        .rounded_lg()
        .bg(card_bg)
        .border_1()
        .border_color(border)
        .shadow_xs()
        .cursor_move()
        .hover(move |style| style.border_color(hover_border))
        .on_drag(TaskDrag::new(id, title.clone()), |drag, position, _, cx| {
            cx.new(|_| drag.clone().at(position))
        })
        .on_drop(cx.listener(move |this, drag: &TaskDrag, _, cx| {
            this.move_task_before(drag.task_id, column_id, position, cx);
        }))
        .child(
            h_flex()
                .items_start()
                .gap_2()
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .text_sm()
                        .line_height(relative(1.35))
                        .child(title),
                )
                .child(
                    h_flex()
                        .flex_shrink_0()
                        .items_center()
                        .gap_0p5()
                        .opacity(0.)
                        .group_hover(group.clone(), |style| style.opacity(1.))
                        .child(
                            Button::new(("complete", id as u64))
                                .icon(IconName::CircleCheck)
                                .ghost()
                                .xsmall()
                                .tooltip("Complete")
                                .on_click(
                                    cx.listener(move |this, _, _, cx| this.complete_task(id, cx)),
                                ),
                        )
                        .child(
                            Button::new(("delete", id as u64))
                                .icon(IconName::Delete)
                                .ghost()
                                .xsmall()
                                .tooltip("Delete")
                                .on_click(
                                    cx.listener(move |this, _, _, cx| this.delete_task(id, cx)),
                                ),
                        ),
                ),
        )
        .when(has_meta, |el| {
            el.child(h_flex().flex_wrap().items_center().gap_2().children(meta))
        })
}

fn subtask_progress(parent_id: i64, all_tasks: &[Task]) -> (usize, usize) {
    all_tasks
        .iter()
        .filter(|t| t.parent_task_id == Some(parent_id))
        .fold((0, 0), |(done, total), child| {
            (done + usize::from(child.completed_at.is_some()), total + 1)
        })
}

fn empty_state(cx: &mut Context<Workspace>) -> impl IntoElement {
    div().pt(px(64.)).child(
        Empty::new()
            .header(
                EmptyHeader::new()
                    .media(
                        EmptyMedia::new()
                            .with_variant(EmptyMediaVariant::Icon)
                            .child(Icon::new(IconName::Kanban)),
                    )
                    .title(EmptyTitle::new().child("No columns yet"))
                    .description(
                        EmptyDescription::new()
                            .child("Add a column to start organising your tasks."),
                    ),
            )
            .content(
                EmptyContent::new().child(
                    Button::new("empty-add-column")
                        .primary()
                        .icon(IconName::Plus)
                        .label("Add column")
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.open_add_column_dialog(window, cx)
                        })),
                ),
            ),
    )
}

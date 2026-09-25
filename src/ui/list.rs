use gpui_kit::assets::IconName;
use gpui_kit::component::button::*;
use gpui_kit::component::checkbox::Checkbox;
use gpui_kit::component::empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::component::{ActiveTheme as _, Icon, Sizable as _, *};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use crate::domain::Task;
use crate::ui::task_meta;
use crate::ui::workspace::Workspace;

/// Renders open tasks grouped by column, with subtasks nested under their
/// parent and a trailing section of finished work.
pub fn render(workspace: &Workspace, cx: &mut Context<Workspace>) -> impl IntoElement {
    let columns = workspace.columns().to_vec();
    let tasks = workspace.tasks().to_vec();

    let mut sections: Vec<AnyElement> = Vec::new();
    for column in &columns {
        let column_tasks: Vec<&Task> = tasks
            .iter()
            .filter(|t| {
                t.column_id == column.id && t.completed_at.is_none() && t.parent_task_id.is_none()
            })
            .collect();
        if column_tasks.is_empty() {
            continue;
        }
        let rows = task_rows(&column_tasks, &tasks, cx);
        sections.push(section(&column.name, column_tasks.len(), rows, cx).into_any_element());
    }

    let completed: Vec<&Task> = tasks.iter().filter(|t| t.completed_at.is_some()).collect();
    if !completed.is_empty() {
        let rows: Vec<AnyElement> = completed
            .iter()
            .copied()
            .map(|task| finished_row(task, cx).into_any_element())
            .collect();
        sections.push(section("Completed", completed.len(), rows, cx).into_any_element());
    }

    if sections.is_empty() {
        sections.push(empty_state().into_any_element());
    }

    div()
        .size_full()
        .overflow_y_scrollbar()
        .p_6()
        .child(v_flex().gap_6().children(sections))
}

fn task_rows(parents: &[&Task], all: &[Task], cx: &mut Context<Workspace>) -> Vec<AnyElement> {
    let mut rows: Vec<AnyElement> = Vec::new();
    for parent in parents.iter().copied() {
        rows.push(open_row(parent, false, cx).into_any_element());
        for child in all
            .iter()
            .filter(|t| t.parent_task_id == Some(parent.id) && t.completed_at.is_none())
        {
            rows.push(open_row(child, true, cx).into_any_element());
        }
    }
    rows
}

fn section(
    name: &str,
    count: usize,
    rows: Vec<AnyElement>,
    cx: &mut Context<Workspace>,
) -> impl IntoElement {
    let muted = cx.theme().muted_foreground;

    v_flex()
        .gap_1()
        .child(
            h_flex()
                .items_center()
                .gap_2()
                .px_3()
                .pb_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(muted)
                        .child(name.to_uppercase()),
                )
                .child(div().text_xs().text_color(muted).child(count.to_string())),
        )
        .children(rows)
}

fn open_row(task: &Task, indented: bool, cx: &mut Context<Workspace>) -> impl IntoElement {
    let id = task.id;
    let group: SharedString = format!("row-{id}").into();
    let (accent, foreground) = {
        let t = cx.theme();
        (t.accent, t.foreground)
    };

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

    h_flex()
        .id(("list-row", id as u64))
        .group(group.clone())
        .items_center()
        .gap_3()
        .px_3()
        .py_2()
        .rounded_lg()
        .when(indented, |el| el.ml(px(28.)))
        .hover(move |style| style.bg(accent))
        .child(
            Checkbox::new(("done", id as u64))
                .checked(false)
                .on_click(cx.listener(move |this, checked: &bool, _, cx| {
                    if *checked {
                        this.complete_task(id, cx);
                    }
                })),
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .text_sm()
                .text_color(foreground)
                .child(task.title.clone()),
        )
        .children(meta)
        .child(
            Button::new(("list-delete", id as u64))
                .icon(IconName::Delete)
                .ghost()
                .xsmall()
                .opacity(0.)
                .tooltip("Delete")
                .group_hover(group, |style| style.opacity(1.))
                .on_click(cx.listener(move |this, _, _, cx| this.delete_task(id, cx))),
        )
}

fn finished_row(task: &Task, cx: &mut Context<Workspace>) -> impl IntoElement {
    let id = task.id;
    let group: SharedString = format!("finished-{id}").into();
    let (accent, muted) = {
        let t = cx.theme();
        (t.accent, t.muted_foreground)
    };

    h_flex()
        .id(("finished-row", id as u64))
        .group(group.clone())
        .items_center()
        .gap_3()
        .px_3()
        .py_1p5()
        .rounded_lg()
        .hover(move |style| style.bg(accent))
        .child(
            Checkbox::new(("finished-done", id as u64))
                .checked(true)
                .disabled(true),
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .text_sm()
                .text_color(muted)
                .line_through()
                .child(task.title.clone()),
        )
        .child(
            Button::new(("finished-delete", id as u64))
                .icon(IconName::Delete)
                .ghost()
                .xsmall()
                .opacity(0.)
                .tooltip("Delete")
                .group_hover(group, |style| style.opacity(1.))
                .on_click(cx.listener(move |this, _, _, cx| this.delete_task(id, cx))),
        )
}

fn empty_state() -> impl IntoElement {
    div().pt(px(64.)).child(
        Empty::new()
            .header(
                EmptyHeader::new()
                    .media(
                        EmptyMedia::new()
                            .with_variant(EmptyMediaVariant::Icon)
                            .child(Icon::new(IconName::List)),
                    )
                    .title(EmptyTitle::new().child("Nothing here yet"))
                    .description(
                        EmptyDescription::new()
                            .child("Add a task above and it will show up in this list."),
                    ),
            )
            .content(EmptyContent::new()),
    )
}

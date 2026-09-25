use gpui_kit::component::button::*;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{IconName, Root, *};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rusqlite::Connection;

use crate::domain::{NewTask, Priority, Task};
use crate::storage::tasks;

/// Phase 1: a single flat task list (no kanban yet) — proves the domain
/// model works before a second renderer (the board) is added in Phase 2.
pub struct TaskListView {
    conn: Connection,
    column_id: i64,
    tasks: Vec<Task>,
    new_task_input: Entity<InputState>,
}

impl TaskListView {
    pub fn new(
        conn: Connection,
        column_id: i64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let tasks = tasks::list_by_column(&conn, column_id).unwrap_or_default();
        let new_task_input = cx.new(|cx| InputState::new(window, cx));
        Self {
            conn,
            column_id,
            tasks,
            new_task_input,
        }
    }

    fn refresh(&mut self) {
        self.tasks = tasks::list_by_column(&self.conn, self.column_id).unwrap_or_default();
    }

    fn add_task(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let title = self.new_task_input.read(cx).value().trim().to_string();
        if title.is_empty() {
            return;
        }
        let _ = tasks::create(
            &self.conn,
            NewTask {
                title,
                description: None,
                column_id: self.column_id,
                priority: Priority::None,
                due_at: None,
                repeat_rule: None,
                parent_task_id: None,
            },
        );
        self.new_task_input
            .update(cx, |input, cx| input.set_value("", window, cx));
        self.refresh();
        cx.notify();
    }

    fn complete_task(&mut self, id: i64, cx: &mut Context<Self>) {
        let _ = tasks::complete_and_recur(&self.conn, id);
        self.refresh();
        cx.notify();
    }

    fn delete_task(&mut self, id: i64, cx: &mut Context<Self>) {
        let _ = tasks::delete(&self.conn, id);
        self.refresh();
        cx.notify();
    }
}

fn task_row(task: &Task, indented: bool, cx: &mut Context<TaskListView>) -> impl IntoElement {
    let id = task.id;
    h_flex()
        .id(("task-row", id as u64))
        .gap_2()
        .items_center()
        .when(indented, |el| el.pl_6())
        .child(
            Button::new(("complete", id as u64))
                .icon(IconName::Check)
                .outline()
                .on_click(cx.listener(move |this, _, _, cx| this.complete_task(id, cx))),
        )
        .child(div().flex_1().child(task.title.clone()))
        .child(
            Button::new(("delete", id as u64))
                .icon(IconName::Delete)
                .outline()
                .on_click(cx.listener(move |this, _, _, cx| this.delete_task(id, cx))),
        )
}

impl Render for TaskListView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dialogs = Root::render_dialog_layer(window, cx);

        let active_tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|t| t.completed_at.is_none())
            .collect();
        let root_tasks: Vec<&Task> = active_tasks
            .iter()
            .filter(|t| t.parent_task_id.is_none())
            .copied()
            .collect();

        let mut rows: Vec<AnyElement> = Vec::new();
        for task in &root_tasks {
            rows.push(task_row(task, false, cx).into_any_element());
            for subtask in active_tasks
                .iter()
                .filter(|t| t.parent_task_id == Some(task.id))
            {
                rows.push(task_row(subtask, true, cx).into_any_element());
            }
        }

        div()
            .size_full()
            .bg(gpui_kit::white())
            .v_flex()
            .gap_4()
            .p_8()
            .child(div().text_xl().child("Tasks"))
            .child(
                h_flex()
                    .gap_2()
                    .child(Input::new(&self.new_task_input).id("new-task"))
                    .child(
                        Button::new("add-task")
                            .primary()
                            .icon(IconName::Plus)
                            .label("Add")
                            .on_click(cx.listener(Self::add_task)),
                    ),
            )
            .child(div().v_flex().gap_2().children(rows))
            .children(dialogs)
    }
}

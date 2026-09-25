use gpui_kit::component::dialog::DialogButtonProps;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::{ActiveTheme as _, Root, WindowExt, *};
use gpui_kit::*;
use rusqlite::Connection;

use crate::domain::{Column, NewColumn, NewTask, Priority, Task};
use crate::error::AppError;
use crate::storage::{columns, tasks};
use crate::ui::{board, chrome, list, theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Board,
    List,
}

/// Which column prompt the shared dialog input is currently serving.
#[derive(Clone, Copy)]
enum ColumnPrompt {
    Create,
    Rename(i64),
}

/// The dragged payload while a task card is being moved — also doubles as
/// its own drag-preview element (gpui renders whatever this returns at the
/// cursor while dragging).
#[derive(Clone)]
pub struct TaskDrag {
    pub task_id: i64,
    pub title: String,
    position: Point<Pixels>,
}

impl TaskDrag {
    pub fn new(task_id: i64, title: String) -> Self {
        Self {
            task_id,
            title,
            position: Point::default(),
        }
    }

    pub fn at(mut self, position: Point<Pixels>) -> Self {
        self.position = position;
        self
    }
}

impl Render for TaskDrag {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().pl(self.position.x).pt(self.position.y).child(
            div()
                .px_3()
                .py_1p5()
                .rounded_lg()
                .bg(cx.theme().primary)
                .text_color(cx.theme().primary_foreground)
                .text_sm()
                .shadow_md()
                .child(self.title.clone()),
        )
    }
}

/// Owns the task/column data and every mutation. `board` and `list` are
/// pure render functions over the same data — this is what makes the
/// Board/List toggle show identical data either way.
pub struct Workspace {
    conn: Connection,
    columns: Vec<Column>,
    tasks: Vec<Task>,
    view_mode: ViewMode,
    target_column_id: Option<i64>,
    new_task_input: Entity<InputState>,
    prompt_input: Entity<InputState>,
}

impl Workspace {
    pub fn new(conn: Connection, window: &mut Window, cx: &mut Context<Self>) -> Self {
        theme::adopt_system_appearance(cx);

        let columns = columns::list(&conn).unwrap_or_default();
        let tasks = tasks::list_all(&conn).unwrap_or_default();
        let new_task_input = cx.new(|cx| InputState::new(window, cx).placeholder("Add a task…"));
        let prompt_input = cx.new(|cx| InputState::new(window, cx));
        let target_column_id = columns.first().map(|column| column.id);

        Self {
            conn,
            columns,
            tasks,
            view_mode: ViewMode::Board,
            target_column_id,
            new_task_input,
            prompt_input,
        }
    }

    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn view_mode(&self) -> ViewMode {
        self.view_mode
    }

    pub fn new_task_input(&self) -> &Entity<InputState> {
        &self.new_task_input
    }

    pub fn target_column_name(&self) -> Option<String> {
        self.target_column_id.and_then(|id| {
            self.columns
                .iter()
                .find(|column| column.id == id)
                .map(|column| column.name.clone())
        })
    }

    fn refresh(&mut self) {
        self.columns = columns::list(&self.conn).unwrap_or_default();
        self.tasks = tasks::list_all(&self.conn).unwrap_or_default();

        // Keep the composer pointed at a column that still exists.
        if !self
            .columns
            .iter()
            .any(|column| Some(column.id) == self.target_column_id)
        {
            self.target_column_id = self.columns.first().map(|column| column.id);
        }
    }

    pub fn set_view_mode(&mut self, mode: ViewMode, cx: &mut Context<Self>) {
        self.view_mode = mode;
        cx.notify();
    }

    pub fn set_target_column(&mut self, id: i64, cx: &mut Context<Self>) {
        self.target_column_id = Some(id);
        cx.notify();
    }

    pub fn add_task(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let title = self.new_task_input.read(cx).value().trim().to_string();
        if title.is_empty() {
            return;
        }
        let Some(column_id) = self
            .target_column_id
            .or_else(|| self.columns.first().map(|column| column.id))
        else {
            return;
        };

        let _ = tasks::create(
            &self.conn,
            NewTask {
                title,
                description: None,
                column_id,
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

    pub fn delete_column(&mut self, id: i64, window: &mut Window, cx: &mut Context<Self>) {
        match columns::delete(&self.conn, id) {
            Ok(()) => {
                self.refresh();
                cx.notify();
            }
            Err(AppError::ColumnNotEmpty) => {
                window.push_notification(
                    Notification::new().message("Move tasks out before deleting this column"),
                    cx,
                );
            }
            Err(_) => {}
        }
    }

    pub fn open_add_column_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_column_prompt(ColumnPrompt::Create, window, cx);
    }

    pub fn open_rename_column_dialog(
        &mut self,
        id: i64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_column_prompt(ColumnPrompt::Rename(id), window, cx);
    }

    fn show_column_prompt(
        &mut self,
        prompt: ColumnPrompt,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (title, ok_text) = match prompt {
            ColumnPrompt::Create => ("New column", "Create column"),
            ColumnPrompt::Rename(_) => ("Rename column", "Save"),
        };
        let input = self.prompt_input.clone();
        input.update(cx, |state, cx| {
            state.set_value("", window, cx);
            state.set_placeholder("Column name", window, cx);
        });
        let workspace = cx.entity().downgrade();

        window.open_dialog(cx, move |dialog, _window, _cx| {
            let input = input.clone();
            let workspace = workspace.clone();
            dialog
                .title(title)
                .child(v_flex().w_full().child(Input::new(&input)))
                .button_props(
                    DialogButtonProps::default()
                        .ok_text(ok_text)
                        .cancel_text("Cancel")
                        .show_cancel(true),
                )
                .on_ok(move |_, _, cx| {
                    let name = input.read(cx).value().trim().to_string();
                    if name.is_empty() {
                        return false;
                    }
                    workspace
                        .update(cx, |this, cx| this.commit_column_prompt(prompt, name, cx))
                        .is_ok()
                })
        });
    }

    fn commit_column_prompt(&mut self, prompt: ColumnPrompt, name: String, cx: &mut Context<Self>) {
        match prompt {
            ColumnPrompt::Create => {
                let _ = columns::create(
                    &self.conn,
                    NewColumn {
                        name,
                        color: None,
                        wip_limit: None,
                    },
                );
            }
            ColumnPrompt::Rename(id) => {
                let _ = columns::rename(&self.conn, id, &name);
            }
        }
        self.refresh();
        cx.notify();
    }

    pub fn complete_task(&mut self, id: i64, cx: &mut Context<Self>) {
        let _ = tasks::complete_and_recur(&self.conn, id);
        self.refresh();
        cx.notify();
    }

    pub fn delete_task(&mut self, id: i64, cx: &mut Context<Self>) {
        let _ = tasks::delete(&self.conn, id);
        self.refresh();
        cx.notify();
    }

    /// Drops a dragged task directly before `before_position` in `column_id`.
    pub fn move_task_before(
        &mut self,
        task_id: i64,
        column_id: i64,
        before_position: f64,
        cx: &mut Context<Self>,
    ) {
        let _ = tasks::move_before(&self.conn, task_id, column_id, before_position);
        self.refresh();
        cx.notify();
    }

    /// Drops a dragged task at the end of `column_id`.
    pub fn move_task_to_end(&mut self, task_id: i64, column_id: i64, cx: &mut Context<Self>) {
        let _ = tasks::move_to_end(&self.conn, task_id, column_id);
        self.refresh();
        cx.notify();
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dialogs = Root::render_dialog_layer(window, cx);
        let sheets = Root::render_sheet_layer(window, cx);
        let notifications = Root::render_notification_layer(window, cx);
        let (background, foreground) = {
            let theme = cx.theme();
            (theme.background, theme.foreground)
        };

        div()
            .size_full()
            .v_flex()
            .bg(background)
            .text_color(foreground)
            .child(chrome::header(self, cx))
            .child(chrome::toolbar(self, cx))
            .child(
                // Main content: fills all remaining height.
                div().flex_1().min_h_0().child(match self.view_mode {
                    ViewMode::Board => board::render(self, cx).into_any_element(),
                    ViewMode::List => list::render(self, cx).into_any_element(),
                }),
            )
            .children(dialogs)
            .children(sheets)
            .children(notifications)
    }
}

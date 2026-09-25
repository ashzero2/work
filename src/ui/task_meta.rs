use chrono::{DateTime, Utc};
use gpui_kit::assets::IconName;
use gpui_kit::component::tag::Tag;
use gpui_kit::component::{ActiveTheme as _, Icon, Sizable as _, *};
use gpui_kit::*;

use crate::domain::{Priority, RepeatRule};

/// Renders a coloured chip naming a task's priority, or nothing when it
/// carries none.
pub fn priority_tag(priority: Priority) -> Option<AnyElement> {
    let tag = match priority {
        Priority::High => Tag::danger(),
        Priority::Medium => Tag::warning(),
        Priority::Low => Tag::info(),
        Priority::None => return None,
    };
    Some(
        tag.small()
            .child(priority_label(priority))
            .into_any_element(),
    )
}

/// Renders a calendar chip with a task's due date, marked overdue once the
/// date has passed.
pub fn due_chip(due_at: DateTime<Utc>, cx: &App) -> AnyElement {
    let days = (due_at.date_naive() - Utc::now().date_naive()).num_days();
    let (label, color) = match days {
        0 => ("Today".to_string(), cx.theme().warning),
        1 => ("Tomorrow".to_string(), cx.theme().muted_foreground),
        overdue if overdue < 0 => (format!("{}d overdue", -overdue), cx.theme().danger),
        _ => (
            due_at.format("%b %-d").to_string(),
            cx.theme().muted_foreground,
        ),
    };

    meta_chip(IconName::CalendarDays, label, color)
}

/// Renders a chip naming a task's repeat cadence.
pub fn repeat_chip(rule: RepeatRule, cx: &App) -> AnyElement {
    let label = match rule {
        RepeatRule::Daily => "Daily",
        RepeatRule::Weekly => "Weekly",
        RepeatRule::Monthly => "Monthly",
    };
    meta_chip(
        IconName::RotateCw,
        label.to_string(),
        cx.theme().muted_foreground,
    )
}

/// Renders a chip counting finished subtasks against the total, or nothing
/// when a task has none.
pub fn subtask_chip(done: usize, total: usize, cx: &App) -> Option<AnyElement> {
    if total == 0 {
        return None;
    }
    Some(meta_chip(
        IconName::ListChecks,
        format!("{done}/{total}"),
        cx.theme().muted_foreground,
    ))
}

fn priority_label(priority: Priority) -> &'static str {
    match priority {
        Priority::High => "High",
        Priority::Medium => "Medium",
        Priority::Low => "Low",
        Priority::None => "",
    }
}

fn meta_chip(icon: IconName, label: String, color: Hsla) -> AnyElement {
    h_flex()
        .flex_shrink_0()
        .items_center()
        .gap_1()
        .text_xs()
        .text_color(color)
        .child(Icon::new(icon).xsmall())
        .child(label)
        .into_any_element()
}

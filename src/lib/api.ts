import { invoke } from '@tauri-apps/api/core';

import type {
  Column,
  NewColumn,
  NewTask,
  NoteCard,
  NotificationState,
  PomodoroSession,
  PomodoroSettings,
  Reminder,
  ReminderKind,
  SessionKind,
  TagSummary,
  Task
} from './types';

export function listTasks(): Promise<Task[]> {
  return invoke<Task[]>('list_tasks');
}

export function createTask(newTask: NewTask): Promise<Task> {
  return invoke<Task>('create_task', { newTask });
}

export function completeTask(id: number): Promise<Task | null> {
  return invoke<Task | null>('complete_task', { id });
}

export function deleteTask(id: number): Promise<void> {
  return invoke<void>('delete_task', { id });
}

export function moveTaskBefore(
  taskId: number,
  columnId: number,
  beforePosition: number
): Promise<void> {
  return invoke<void>('move_task_before', { taskId, columnId, beforePosition });
}

export function moveTaskToEnd(taskId: number, columnId: number): Promise<void> {
  return invoke<void>('move_task_to_end', { taskId, columnId });
}

export function listColumns(): Promise<Column[]> {
  return invoke<Column[]>('list_columns');
}

export function createColumn(newColumn: NewColumn): Promise<Column> {
  return invoke<Column>('create_column', { newColumn });
}

export function renameColumn(id: number, name: string): Promise<void> {
  return invoke<void>('rename_column', { id, name });
}

export function deleteColumn(id: number): Promise<void> {
  return invoke<void>('delete_column', { id });
}

export function listNotes(): Promise<NoteCard[]> {
  return invoke<NoteCard[]>('list_notes');
}

export function createNote(title: string): Promise<NoteCard> {
  return invoke<NoteCard>('create_note', { title });
}

export function noteBody(id: number): Promise<string> {
  return invoke<string>('note_body', { id });
}

export function saveNote(
  id: number,
  title: string,
  body: string,
  tags: string[]
): Promise<NoteCard> {
  return invoke<NoteCard>('save_note', { id, title, body, tags });
}

export function moveNote(id: number, x: number, y: number): Promise<void> {
  return invoke<void>('move_note', { id, x, y });
}

export function deleteNote(id: number): Promise<void> {
  return invoke<void>('delete_note', { id });
}

export function searchNotes(query: string): Promise<number[]> {
  return invoke<number[]>('search_notes', { query });
}

export function listTags(): Promise<TagSummary[]> {
  return invoke<TagSummary[]>('list_tags');
}

export function deleteTag(id: number): Promise<void> {
  return invoke<void>('delete_tag', { id });
}

export function systemAccent(): Promise<string | null> {
  return invoke<string | null>('system_accent');
}

export function getSetting(key: string): Promise<string | null> {
  return invoke<string | null>('get_setting', { key });
}

export function setSetting(key: string, value: string): Promise<void> {
  return invoke<void>('set_setting', { key, value });
}

export function setTrafficLightsVisible(visible: boolean): Promise<void> {
  return invoke<void>('set_traffic_lights_visible', { visible });
}

export function startSession(
  kind: SessionKind,
  taskId: number | null,
  plannedSeconds: number
): Promise<PomodoroSession> {
  return invoke<PomodoroSession>('start_session', { kind, taskId, plannedSeconds });
}

export function finishSession(id: number, completed: boolean): Promise<PomodoroSession> {
  return invoke<PomodoroSession>('finish_session', { id, completed });
}

export function reconcileSession(): Promise<PomodoroSession | null> {
  return invoke<PomodoroSession | null>('reconcile_session');
}

export function nextPhase(): Promise<SessionKind> {
  return invoke<SessionKind>('next_phase');
}

export function cyclePosition(): Promise<number> {
  return invoke<number>('cycle_position');
}

export function recentSessions(limit: number): Promise<PomodoroSession[]> {
  return invoke<PomodoroSession[]>('recent_sessions', { limit });
}

export function getPomodoroSettings(): Promise<PomodoroSettings> {
  return invoke<PomodoroSettings>('get_pomodoro_settings');
}

export function setPomodoroSettings(settings: PomodoroSettings): Promise<PomodoroSettings> {
  return invoke<PomodoroSettings>('set_pomodoro_settings', { settings });
}

export function listReminders(): Promise<Reminder[]> {
  return invoke<Reminder[]>('list_reminders');
}

export function createReminder(
  kind: ReminderKind,
  taskId: number | null,
  triggerAt: string
): Promise<Reminder> {
  return invoke<Reminder>('create_reminder', { kind, taskId, triggerAt });
}

export function rescheduleReminder(id: number, triggerAt: string): Promise<Reminder> {
  return invoke<Reminder>('reschedule_reminder', { id, triggerAt });
}

export function snoozeReminder(id: number): Promise<Reminder> {
  return invoke<Reminder>('snooze_reminder', { id });
}

export function dismissReminder(id: number): Promise<Reminder> {
  return invoke<Reminder>('dismiss_reminder', { id });
}

export function deleteReminder(id: number): Promise<void> {
  return invoke<void>('delete_reminder', { id });
}

export function notificationState(): Promise<NotificationState> {
  return invoke<NotificationState>('notification_state');
}

export function setKindEnabled(kind: ReminderKind, enabled: boolean): Promise<void> {
  return invoke<void>('set_kind_enabled', { kind, enabled });
}

export function openNotificationSettings(): Promise<void> {
  return invoke<void>('open_notification_settings');
}

export function captureTask(title: string): Promise<void> {
  return invoke<void>('capture_task', { title });
}

export function hideCaptureWindow(): Promise<void> {
  return invoke<void>('hide_capture_window');
}

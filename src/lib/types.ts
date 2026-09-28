export type Priority = 'none' | 'low' | 'medium' | 'high';

export type RepeatRule = 'daily' | 'weekly' | 'monthly';

export type ViewMode = 'board' | 'list';

export type Section = 'tasks' | 'notes' | 'focus' | 'reminders' | 'settings';

export type ExportFormat = 'json' | 'csv';

export type SessionKind = 'work' | 'break' | 'long_break';

export type ReminderKind = 'task_due' | 'priority_alert' | 'pomodoro_break' | 'custom';

export type ReminderStatus = 'pending' | 'fired' | 'snoozed' | 'dismissed';

export interface Task {
  id: number;
  title: string;
  description: string | null;
  columnId: number;
  position: number;
  priority: Priority;
  dueAt: string | null;
  completedAt: string | null;
  repeatRule: RepeatRule | null;
  parentTaskId: number | null;
  createdAt: string;
  updatedAt: string;
}

export interface Column {
  id: number;
  name: string;
  orderIndex: number;
  color: string | null;
  wipLimit: number | null;
}

export interface NewTask {
  title: string;
  description: string | null;
  columnId: number;
  priority: Priority;
  dueAt: string | null;
  repeatRule: RepeatRule | null;
  parentTaskId: number | null;
}

export interface NewColumn {
  name: string;
  color: string | null;
  wipLimit: number | null;
}

export interface Note {
  id: number;
  title: string;
  filePath: string;
  linkedTaskId: number | null;
  color: string;
  rotationDeg: number;
  posX: number | null;
  posY: number | null;
  createdAt: string;
  updatedAt: string;
}

export interface NoteCard extends Note {
  tags: string[];
}

export interface TagSummary {
  id: number;
  name: string;
  color: string | null;
  noteCount: number;
}

export interface PomodoroSession {
  id: number;
  taskId: number | null;
  kind: SessionKind;
  plannedSeconds: number;
  startedAt: string;
  endedAt: string | null;
  completed: boolean;
}

export interface PomodoroSettings {
  workSeconds: number;
  breakSeconds: number;
  longBreakSeconds: number;
  sessionsPerLongBreak: number;
}

export interface Reminder {
  id: number;
  kind: ReminderKind;
  taskId: number | null;
  sessionId: number | null;
  triggerAt: string;
  status: ReminderStatus;
  snoozedUntil: string | null;
  systemNotificationTag: string | null;
}

export interface KindToggle {
  kind: ReminderKind;
  enabled: boolean;
}

export interface NotificationState {
  available: boolean;
  /// null while the permission prompt has not been answered.
  authorized: boolean | null;
  enabled: KindToggle[];
}

export interface ShortcutState {
  /// What is bound now, or null when nothing could be registered.
  shortcut: string | null;
  /// What was asked for, which is what the recorder shows even when the binding
  /// itself was refused.
  preferred: string;
  default: string;
}

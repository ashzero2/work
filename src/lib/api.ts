import { invoke } from '@tauri-apps/api/core';

import type { Column, NewColumn, NewTask, NoteCard, TagSummary, Task } from './types';

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

export function setTrafficLightsVisible(visible: boolean): Promise<void> {
  return invoke<void>('set_traffic_lights_visible', { visible });
}

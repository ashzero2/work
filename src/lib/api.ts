import { invoke } from '@tauri-apps/api/core';

import type { Column, NewColumn, NewTask, Task } from './types';

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

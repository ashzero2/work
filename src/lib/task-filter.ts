import type { Priority, Task } from './types';

export type TaskStatusFilter = 'all' | 'open' | 'completed';
export type TaskDueFilter = 'all' | 'today' | 'overdue' | 'undated';
export interface TaskFilters {
  status: TaskStatusFilter;
  priority: Priority | 'all';
  due: TaskDueFilter;
}

export function matchesTask(task: Task, filters: TaskFilters, now = new Date()): boolean {
  if (filters.status === 'open' && task.completedAt !== null) return false;
  if (filters.status === 'completed' && task.completedAt === null) return false;
  if (filters.priority !== 'all' && task.priority !== filters.priority) return false;
  if (filters.due === 'all') return true;
  if (filters.due === 'undated') return task.dueAt === null;
  if (task.dueAt === null || task.completedAt !== null) return false;
  const due = new Date(task.dueAt);
  if (filters.due === 'today') return due.toDateString() === now.toDateString();
  return due.getTime() < now.getTime();
}

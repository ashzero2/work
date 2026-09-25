import * as api from '$lib/api';
import { toast } from '$lib/stores/toast.svelte';
import type { Column, Task, ViewMode } from '$lib/types';

function errorMessage(error: unknown): string {
  return typeof error === 'string' ? error : String(error);
}

/// Owns the task/column data and every mutation — the single source the board
/// and the list view both read from, so the two renderers never diverge.
class WorkspaceStore {
  columns = $state<Column[]>([]);
  tasks = $state<Task[]>([]);
  viewMode = $state<ViewMode>('board');
  targetColumnId = $state<number | null>(null);
  loading = $state(true);

  async load(): Promise<void> {
    try {
      await this.refresh();
    } finally {
      this.loading = false;
    }
  }

  async refresh(): Promise<void> {
    try {
      const [columns, tasks] = await Promise.all([api.listColumns(), api.listTasks()]);
      this.columns = columns;
      this.tasks = tasks;
      this.repairTargetColumn();
    } catch (error) {
      toast.show(errorMessage(error));
    }
  }

  get targetColumnName(): string | null {
    return this.columns.find((column) => column.id === this.targetColumnId)?.name ?? null;
  }

  get openCount(): number {
    return this.tasks.filter((task) => task.completedAt === null && task.parentTaskId === null)
      .length;
  }

  get doneCount(): number {
    return this.tasks.filter((task) => task.completedAt !== null).length;
  }

  setViewMode(mode: ViewMode): void {
    this.viewMode = mode;
  }

  setTargetColumn(id: number): void {
    this.targetColumnId = id;
  }

  subtaskProgress(parentId: number): { done: number; total: number } {
    let done = 0;
    let total = 0;
    for (const task of this.tasks) {
      if (task.parentTaskId !== parentId) continue;
      total += 1;
      if (task.completedAt !== null) done += 1;
    }
    return { done, total };
  }

  async addTask(title: string): Promise<void> {
    const trimmed = title.trim();
    const columnId = this.targetColumnId ?? this.columns[0]?.id ?? null;
    if (!trimmed || columnId === null) return;

    await api.createTask({
      title: trimmed,
      description: null,
      columnId,
      priority: 'none',
      dueAt: null,
      repeatRule: null,
      parentTaskId: null
    });
    await this.refresh();
  }

  async completeTask(id: number): Promise<void> {
    this.tasks = this.tasks.map((task) =>
      task.id === id ? { ...task, completedAt: task.completedAt ?? new Date().toISOString() } : task
    );
    await this.persist(() => api.completeTask(id));
  }

  async deleteTask(id: number): Promise<void> {
    const previous = this.tasks;
    this.tasks = previous.filter((task) => task.id !== id);
    try {
      await api.deleteTask(id);
    } catch (error) {
      this.tasks = previous;
      toast.show(errorMessage(error));
    }
    await this.refresh();
  }

  async moveTaskBefore(taskId: number, columnId: number, beforePosition: number): Promise<void> {
    this.tasks = this.tasks.map((task) =>
      task.id === taskId ? { ...task, columnId, position: beforePosition - 0.5 } : task
    );
    await this.persist(() => api.moveTaskBefore(taskId, columnId, beforePosition));
  }

  async moveTaskToEnd(taskId: number, columnId: number): Promise<void> {
    const siblings = this.tasks.filter((task) => task.columnId === columnId);
    const max = siblings.reduce((highest, task) => Math.max(highest, task.position), 0);
    this.tasks = this.tasks.map((task) =>
      task.id === taskId ? { ...task, columnId, position: max + 1 } : task
    );
    await this.persist(() => api.moveTaskToEnd(taskId, columnId));
  }

  async createColumn(name: string): Promise<void> {
    const trimmed = name.trim();
    if (!trimmed) return;
    await api.createColumn({ name: trimmed, color: null, wipLimit: null });
    await this.refresh();
  }

  async renameColumn(id: number, name: string): Promise<void> {
    const trimmed = name.trim();
    if (!trimmed) return;
    await api.renameColumn(id, trimmed);
    await this.refresh();
  }

  async deleteColumn(id: number): Promise<void> {
    try {
      await api.deleteColumn(id);
    } catch (error) {
      toast.show(errorMessage(error));
    }
    await this.refresh();
  }

  private async persist(action: () => Promise<unknown>): Promise<void> {
    try {
      await action();
    } catch (error) {
      toast.show(errorMessage(error));
    }
    await this.refresh();
  }

  private repairTargetColumn(): void {
    if (!this.columns.some((column) => column.id === this.targetColumnId)) {
      this.targetColumnId = this.columns[0]?.id ?? null;
    }
  }
}

export const workspace = new WorkspaceStore();

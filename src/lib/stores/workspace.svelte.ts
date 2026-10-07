import * as api from '$lib/api';
import { toast } from '$lib/stores/toast.svelte';
import type { Column, Task, TaskEdit, ViewMode } from '$lib/types';

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
      // The board/list choice is a preference like any other, so it survives a
      // restart rather than resetting every launch.
      const stored = await api.getSetting('viewMode');
      if (stored === 'board' || stored === 'list') this.viewMode = stored;
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
    return this.tasks.filter((task) => task.completedAt !== null && task.parentTaskId === null)
      .length;
  }

  async setViewMode(mode: ViewMode): Promise<void> {
    this.viewMode = mode;
    try {
      await api.setSetting('viewMode', mode);
    } catch (error) {
      toast.show(errorMessage(error));
    }
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

  async addSubtask(parentId: number, title: string): Promise<boolean> {
    const parent = this.tasks.find((task) => task.id === parentId);
    const trimmed = title.trim();
    if (!parent || parent.parentTaskId !== null || !trimmed) return false;
    try {
      await api.createTask({
        title: trimmed,
        description: null,
        columnId: parent.columnId,
        priority: 'none',
        dueAt: null,
        repeatRule: null,
        parentTaskId: parent.id
      });
      await this.refresh();
      return true;
    } catch (error) {
      toast.show(errorMessage(error));
      return false;
    }
  }

  /// Replaces the task with the version the backend returns, so both the board
  /// and the list see the change without a full reload.
  async updateTask(id: number, edit: TaskEdit): Promise<boolean> {
    try {
      const updated = await api.updateTask(id, edit);
      this.tasks = this.tasks.map((task) => (task.id === id ? updated : task));
      return true;
    } catch (error) {
      toast.show(errorMessage(error));
      return false;
    }
  }

  async toggleTaskCompletion(id: number): Promise<void> {
    const task = this.tasks.find((candidate) => candidate.id === id);
    if (!task || (task.completedAt !== null && task.repeatRule !== null)) return;

    const reopening = task.completedAt !== null;
    this.tasks = this.tasks.map((candidate) =>
      candidate.id === id
        ? { ...candidate, completedAt: reopening ? null : new Date().toISOString() }
        : candidate
    );
    await this.persist(() => reopening ? api.reopenTask(id) : api.completeTask(id));
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

  /// Both return whether the write landed, so the dialog can stay open on
  /// failure instead of silently discarding what was typed.
  async createColumn(name: string): Promise<boolean> {
    const trimmed = name.trim();
    if (!trimmed) return false;
    try {
      await api.createColumn({ name: trimmed, color: null, wipLimit: null });
      await this.refresh();
      return true;
    } catch (error) {
      toast.show(errorMessage(error));
      return false;
    }
  }

  async renameColumn(id: number, name: string): Promise<boolean> {
    const trimmed = name.trim();
    if (!trimmed) return false;
    try {
      await api.renameColumn(id, trimmed);
      await this.refresh();
      return true;
    } catch (error) {
      toast.show(errorMessage(error));
      return false;
    }
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

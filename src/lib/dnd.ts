import type { DragDropState } from '@thisux/sveltednd';

import { workspace } from '$lib/stores/workspace.svelte';
import type { Task } from '$lib/types';

export const cardContainer = (id: number): string => `card-${id}`;
export const columnContainer = (id: number): string => `column-${id}`;

const byPosition = (a: Task, b: Task): number => a.position - b.position;

const openTasksInColumn = (columnId: number): Task[] =>
  workspace.tasks
    .filter(
      (task) => task.columnId === columnId && task.completedAt === null && task.parentTaskId === null
    )
    .sort(byPosition);

/// Translates a drop onto a card or a column lane into the fractional-rank
/// insert the storage layer expects: drop before a card -> that card's
/// position; drop after it -> the next sibling's position (or the end).
export function applyDrop(state: DragDropState<Task>): void {
  const target = state.targetContainer;
  if (!target) return;

  if (target.startsWith('column-')) {
    const columnId = Number(target.slice('column-'.length));
    void workspace.moveTaskToEnd(state.draggedItem.id, columnId);
    return;
  }

  if (!target.startsWith('card-')) return;

  const before = workspace.tasks.find((task) => task.id === Number(target.slice('card-'.length)));
  if (!before || before.id === state.draggedItem.id) return;

  if (state.dropPosition !== 'after') {
    void workspace.moveTaskBefore(state.draggedItem.id, before.columnId, before.position);
    return;
  }

  const next = openTasksInColumn(before.columnId).find((task) => task.position > before.position);
  if (next) {
    void workspace.moveTaskBefore(state.draggedItem.id, before.columnId, next.position);
  } else {
    void workspace.moveTaskToEnd(state.draggedItem.id, before.columnId);
  }
}

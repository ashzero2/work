<script lang="ts">
  import { CircleCheck, Trash2 } from '@lucide/svelte';
  import { draggable, droppable, type DragDropState } from '@thisux/sveltednd';

  import { applyDrop, cardContainer, columnContainer } from '$lib/dnd';
  import type { Task } from '$lib/types';
  import TaskMeta from './TaskMeta.svelte';

  interface Props {
    task: Task;
    subtasksDone: number;
    subtasksTotal: number;
    oncomplete: (id: number) => void;
    ondelete: (id: number) => void;
  }

  let { task, subtasksDone, subtasksTotal, oncomplete, ondelete }: Props = $props();

  function onDrop(state: DragDropState<Task>): void {
    applyDrop(state);
  }
</script>

<article
  class="card"
  data-task-id={task.id}
  use:draggable={{ container: columnContainer(task.columnId), dragData: task }}
  use:droppable={{ container: cardContainer(task.id), callbacks: { onDrop } }}
>
  <div class="row">
    <span class="card-title">{task.title}</span>
    <div class="actions">
      <button class="icon-btn" title="Complete" aria-label="Complete" onclick={() => oncomplete(task.id)}>
        <CircleCheck size={15} />
      </button>
      <button class="icon-btn" title="Delete" aria-label="Delete" onclick={() => ondelete(task.id)}>
        <Trash2 size={15} />
      </button>
    </div>
  </div>
  <TaskMeta {task} {subtasksDone} {subtasksTotal} />
</article>

<style>
  .card {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 14px 0;
    border: 0;
    border-bottom: 1px solid color-mix(in srgb, var(--border) 82%, transparent);
    border-radius: 0;
    background: transparent;
    cursor: grab;
  }

  .card:hover {
    border-color: var(--accent);
    background: var(--hover);
  }

  .row {
    display: flex;
    align-items: flex-start;
    gap: 8px;
  }

  .card-title {
    flex: 1;
    min-width: 0;
    font-size: 14px;
    line-height: 1.35;
  }

  .actions {
    display: flex;
    flex-shrink: 0;
    gap: 2px;
    opacity: 0;
  }

  .card:hover .actions,
  .card:focus-within .actions {
    opacity: 1;
  }
</style>

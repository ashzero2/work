<script lang="ts">
  import { CircleCheck, Pencil, Trash2 } from '@lucide/svelte';
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
    onedit: (id: number) => void;
  }

  let { task, subtasksDone, subtasksTotal, oncomplete, ondelete, onedit }: Props = $props();

  function onDrop(state: DragDropState<Task>): void {
    applyDrop(state);
  }
</script>

<article
  class="card"
  class:completed={task.completedAt !== null}
  data-task-id={task.id}
  use:draggable={{ container: columnContainer(task.columnId), dragData: task }}
  use:droppable={{ container: cardContainer(task.id), callbacks: { onDrop } }}
>
  <div class="row">
    <span class="card-title">{task.title}</span>
    <div class="actions">
      <button class="icon-btn" title="Edit task" aria-label={`Edit ${task.title}`} onclick={() => onedit(task.id)}>
        <Pencil size={15} />
      </button>
      <button
        class="icon-btn"
        title={task.completedAt ? (task.repeatRule ? 'Repeating tasks can’t be reopened' : 'Reopen task') : 'Complete task'}
        aria-label={task.completedAt ? `Reopen ${task.title}` : `Complete ${task.title}`}
        disabled={task.completedAt !== null && task.repeatRule !== null}
        onclick={() => oncomplete(task.id)}
      >
        <CircleCheck size={15} />
      </button>
      <button class="icon-btn" title="Delete" aria-label="Delete" onclick={() => ondelete(task.id)}>
        <Trash2 size={15} />
      </button>
    </div>
  </div>
  {#if task.description}<p class="description">{task.description}</p>{/if}
  <TaskMeta {task} {subtasksDone} {subtasksTotal} />
</article>

<style>
  .completed .card-title { text-decoration: line-through; color: var(--muted-fg); }
  .description { margin: 0; font-size: var(--text-base); line-height: var(--leading-normal); display: -webkit-box; -webkit-line-clamp: 3; line-clamp: 3; -webkit-box-orient: vertical; overflow: hidden; }
  .card {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--panel);
    cursor: grab;
  }

  .card:hover {
    border-color: color-mix(in srgb, var(--accent) 45%, var(--border));
  }

  .row {
    display: flex;
    align-items: flex-start;
    gap: 8px;
  }

  .card-title {
    flex: 1;
    min-width: 0;
    font-size: var(--text-md);
    line-height: var(--leading-snug);
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

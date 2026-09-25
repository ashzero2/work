<script lang="ts">
  import { Check, Trash2 } from '@lucide/svelte';

  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Task } from '$lib/types';
  import EmptyState from './EmptyState.svelte';
  import TaskMeta from './TaskMeta.svelte';

  const byPosition = (a: Task, b: Task): number => a.position - b.position;

  function openTopLevel(columnId: number): Task[] {
    return workspace.tasks
      .filter((task) => task.columnId === columnId && task.completedAt === null && task.parentTaskId === null)
      .sort(byPosition);
  }

  function openChildren(parentId: number): Task[] {
    return workspace.tasks
      .filter((task) => task.parentTaskId === parentId && task.completedAt === null)
      .sort(byPosition);
  }

  const completed = $derived(
    workspace.tasks.filter((task) => task.completedAt !== null).sort(byPosition)
  );
  const hasOpen = $derived(workspace.columns.some((column) => openTopLevel(column.id).length > 0));
</script>

{#snippet row(task: Task, indented: boolean, finished: boolean)}
  {@const progress = workspace.subtaskProgress(task.id)}
  <div class="row" class:indented>
    <button
      class="checkbox"
      class:checked={finished}
      aria-label={finished ? 'Completed' : 'Complete task'}
      disabled={finished}
      onclick={() => void workspace.completeTask(task.id)}
    >
      {#if finished}<Check size={12} />{/if}
    </button>
    <span class="title" class:done={finished}>{task.title}</span>
    {#if !finished}
      <TaskMeta {task} subtasksDone={progress.done} subtasksTotal={progress.total} />
    {/if}
    <button class="icon-btn delete" aria-label="Delete task" onclick={() => void workspace.deleteTask(task.id)}>
      <Trash2 size={15} />
    </button>
  </div>
{/snippet}

<div class="list">
  {#each workspace.columns as column (column.id)}
    {@const open = openTopLevel(column.id)}
    {#if open.length > 0}
      <section class="group">
        <header class="group-head">
          <span class="group-name">{column.name.toUpperCase()}</span>
          <span class="group-count">{open.length}</span>
        </header>
        {#each open as task (task.id)}
          {@render row(task, false, false)}
          {#each openChildren(task.id) as child (child.id)}
            {@render row(child, true, false)}
          {/each}
        {/each}
      </section>
    {/if}
  {/each}

  {#if completed.length > 0}
    <section class="group">
      <header class="group-head">
        <span class="group-name">COMPLETED</span>
        <span class="group-count">{completed.length}</span>
      </header>
      {#each completed as task (task.id)}
        {@render row(task, false, true)}
      {/each}
    </section>
  {/if}

  {#if !hasOpen && completed.length === 0}
    <div class="center">
      <EmptyState
        variant="list"
        title="Nothing here yet"
        description="Add a task above and it will show up in this list."
      />
    </div>
  {/if}
</div>

<style>
  .list {
    display: flex;
    flex-direction: column;
    gap: 22px;
    height: 100%;
    padding: 20px;
    overflow-y: auto;
  }

  .group {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .group-head {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 10px 4px;
    color: var(--muted-fg);
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.03em;
  }

  .group-count {
    font-weight: 500;
  }

  .row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 7px 10px;
    border-radius: var(--radius-sm);
  }

  .row:hover {
    background: var(--lane);
  }

  .row.indented {
    margin-left: 26px;
  }

  .checkbox {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    padding: 0;
    border: 1px solid var(--border);
    border-radius: 5px;
    background: var(--panel);
    color: var(--accent-fg);
    cursor: pointer;
  }

  .checkbox:hover {
    border-color: var(--accent);
  }

  .checkbox.checked {
    border-color: var(--accent);
    background: var(--accent);
  }

  .title {
    flex: 1;
    min-width: 0;
    font-size: 13px;
  }

  .title.done {
    color: var(--muted-fg);
    text-decoration: line-through;
  }

  .delete {
    opacity: 0;
  }

  .row:hover .delete,
  .row:focus-within .delete {
    opacity: 1;
  }

  .center {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
    min-height: 240px;
  }
</style>

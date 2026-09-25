<script lang="ts">
  import { Ellipsis, Pencil, Trash2 } from '@lucide/svelte';
  import { droppable, type DragDropState } from '@thisux/sveltednd';

  import { applyDrop, columnContainer } from '$lib/dnd';
  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Column as ColumnModel, Task } from '$lib/types';
  import DropdownMenu from './DropdownMenu.svelte';
  import TaskCard from './TaskCard.svelte';

  interface Props {
    column: ColumnModel;
    tasks: Task[];
    onrename: (column: ColumnModel) => void;
  }

  let { column, tasks, onrename }: Props = $props();

  const count = $derived(tasks.length);
  const overWip = $derived(column.wipLimit !== null && count > column.wipLimit);
  const countLabel = $derived(column.wipLimit !== null ? `${count}/${column.wipLimit}` : `${count}`);

  function onDrop(state: DragDropState<Task>): void {
    applyDrop(state);
  }
</script>

<section
  class="lane"
  use:droppable={{ container: columnContainer(column.id), callbacks: { onDrop } }}
>
  <header class="lane-head">
    <span class="lane-name" title={column.name}>{column.name}</span>
    <span class="lane-count" class:over={overWip}>{countLabel}</span>
    <DropdownMenu align="end">
      {#snippet trigger({ toggle })}
        <button class="icon-btn" onclick={toggle} title="Column actions" aria-label="Column actions">
          <Ellipsis size={15} />
        </button>
      {/snippet}
      {#snippet content({ close })}
        <button
          class="menu-item"
          onclick={() => {
            onrename(column);
            close();
          }}
        >
          <Pencil size={14} /> Rename
        </button>
        <div class="menu-sep"></div>
        <button
          class="menu-item danger"
          onclick={() => {
            void workspace.deleteColumn(column.id);
            close();
          }}
        >
          <Trash2 size={14} /> Delete
        </button>
      {/snippet}
    </DropdownMenu>
  </header>

  <div class="lane-cards">
    {#each tasks as task (task.id)}
      {@const progress = workspace.subtaskProgress(task.id)}
      <TaskCard
        {task}
        subtasksDone={progress.done}
        subtasksTotal={progress.total}
        oncomplete={(id) => void workspace.completeTask(id)}
        ondelete={(id) => void workspace.deleteTask(id)}
      />
    {/each}
    {#if tasks.length === 0}
      <div class="lane-empty">No tasks</div>
    {/if}
  </div>
</section>

<style>
  .lane {
    display: flex;
    flex-direction: column;
    gap: 8px;
    width: 300px;
    max-height: 100%;
    flex-shrink: 0;
    padding: 10px;
    border-radius: var(--radius);
    background: var(--lane);
  }

  .lane-head {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 2px;
  }

  .lane-name {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    font-size: 13px;
    font-weight: 600;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .lane-count {
    flex-shrink: 0;
    padding: 2px 7px;
    border-radius: 999px;
    background: var(--panel);
    color: var(--muted-fg);
    font-size: 11px;
  }

  .lane-count.over {
    background: color-mix(in srgb, var(--danger) 16%, transparent);
    color: var(--danger);
  }

  .lane-cards {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-height: 44px;
    overflow-y: auto;
  }

  .lane-empty {
    display: flex;
    justify-content: center;
    padding: 18px 0;
    color: var(--muted-fg);
    font-size: 12px;
  }
</style>

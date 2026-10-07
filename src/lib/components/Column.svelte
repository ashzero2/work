<script lang="ts">
  import { Ellipsis, Pencil, Plus, Trash2 } from '@lucide/svelte';
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
    onadd: () => void;
    onaddcolumn: () => void;
    onedit: (id: number) => void;
  }

  let { column, tasks, onrename, onadd, onaddcolumn, onedit }: Props = $props();

  const count = $derived(tasks.length);
  const overWip = $derived(column.wipLimit !== null && count > column.wipLimit);
  const countLabel = $derived(column.wipLimit !== null ? `${count}/${column.wipLimit}` : `${count}`);

  function onDrop(state: DragDropState<Task>): void {
    applyDrop(state);
  }
</script>

<section
  class="lane"
  role="group"
  aria-label={`${column.name} column`}
  data-column-id={column.id}
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
        <button class="menu-item" onclick={() => { close(); onaddcolumn(); }}><Plus size={14} /> Add column</button>
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
        oncomplete={(id) => void workspace.toggleTaskCompletion(id)}
        ondelete={(id) => void workspace.deleteTask(id)}
        {onedit}
      />
    {/each}
    {#if tasks.length === 0}
      <div class="lane-empty">No tasks</div>
    {/if}
  </div>
  <button class="btn add-task" onclick={onadd}><Plus size={14} /> Add task</button>
</section>

<style>
  .add-task { width: 100%; flex-shrink: 0; }
  .lane {
    display: flex;
    flex-direction: column;
    gap: 10px;
    width: max(240px, calc((100% - 28px) / 3));
    min-height: 475px;
    max-height: 100%;
    flex-shrink: 0;
    padding: 12px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--lane);
  }

  .lane-head {
    display: flex;
    align-items: center;
    gap: 10px;
    min-height: 40px;
    padding: 0 0 10px;
    border-bottom: 1px solid var(--divider);
  }

  .lane-name {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    font-size: var(--text-base);
    font-weight: var(--weight-semibold);
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .lane-count {
    flex-shrink: 0;
    min-width: 20px;
    padding: 2px 0;
    border-radius: 0;
    background: transparent;
    color: var(--muted-fg);
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    font-variant-numeric: tabular-nums;
  }

  .lane-count.over {
    background: color-mix(in srgb, var(--danger) 16%, transparent);
    color: var(--danger);
  }

  .lane-cards {
    display: flex;
    flex-direction: column;
    gap: 10px;
    min-height: 44px;
    overflow-y: auto;
  }

  .lane-empty {
    display: flex;
    justify-content: center;
    padding: 18px 0;
    border-bottom: 1px dashed var(--divider);
    color: var(--muted-fg);
    font-size: var(--text-base);
  }
</style>

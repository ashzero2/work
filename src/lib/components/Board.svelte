<script lang="ts">
  import { Plus } from '@lucide/svelte';

  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Column as ColumnModel, Task } from '$lib/types';
  import Column from './Column.svelte';
  import EmptyState from './EmptyState.svelte';

  interface Props {
    onrename: (column: ColumnModel) => void;
    onaddcolumn: () => void;
  }

  let { onrename, onaddcolumn }: Props = $props();

  const openByColumn = $derived.by(() => {
    const grouped = new Map<number, Task[]>();
    for (const column of workspace.columns) grouped.set(column.id, []);
    for (const task of workspace.tasks) {
      if (task.completedAt !== null || task.parentTaskId !== null) continue;
      grouped.get(task.columnId)?.push(task);
    }
    for (const list of grouped.values()) list.sort((a, b) => a.position - b.position);
    return grouped;
  });
</script>

{#if workspace.columns.length === 0}
  <div class="center">
    <EmptyState
      variant="board"
      title="No columns yet"
      description="Add a column to start organising your tasks."
    >
      {#snippet action()}
        <button class="btn btn-primary" onclick={onaddcolumn}><Plus size={15} /> Add column</button>
      {/snippet}
    </EmptyState>
  </div>
{:else}
  <div class="board">
    {#each workspace.columns as column (column.id)}
      <Column {column} tasks={openByColumn.get(column.id) ?? []} {onrename} />
    {/each}
  </div>
{/if}

<style>
  .board {
    display: flex;
    align-items: flex-start;
    gap: 16px;
    height: 100%;
    padding: 20px;
    overflow: auto;
  }

  .center {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
  }
</style>

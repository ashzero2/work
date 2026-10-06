<script lang="ts">
  import { Plus } from '@lucide/svelte';

  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Column as ColumnModel, Task } from '$lib/types';
  import Column from './Column.svelte';
  import EmptyState from './EmptyState.svelte';
  import LoadingState from './LoadingState.svelte';

  interface Props {
    onrename: (column: ColumnModel) => void;
    onaddcolumn: () => void;
    onaddtask: (id: number) => void;
    onedit: (id: number) => void;
    tasks: Task[];
  }

  let { onrename, onaddcolumn, onaddtask, onedit, tasks }: Props = $props();

  const openByColumn = $derived.by(() => {
    const grouped = new Map<number, Task[]>();
    for (const column of workspace.columns) grouped.set(column.id, []);
    for (const task of tasks) {
      if (task.parentTaskId !== null) continue;
      grouped.get(task.columnId)?.push(task);
    }
    for (const list of grouped.values()) list.sort((a, b) => a.position - b.position);
    return grouped;
  });

  // A column cut off at the right edge reads as a rendering bug unless something
  // says the board scrolls sideways — so the fade appears only while there is
  // more to see, and disappears at the end.
  let board = $state<HTMLDivElement | null>(null);
  let moreRight = $state(false);

  function measure(): void {
    if (board === null) return;
    moreRight = board.scrollWidth - board.clientWidth - board.scrollLeft > 4;
  }

  $effect(() => {
    // Re-measure when the columns or tasks change, not only on scroll.
    void workspace.columns.length;
    void workspace.tasks.length;
    measure();
  });
</script>

<svelte:window onresize={measure} />

{#if workspace.loading}
  <LoadingState label="Loading tasks…" />
{:else if workspace.columns.length === 0}
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
  <div class="board-wrap">
    <div class="board" bind:this={board} onscroll={measure}>
      {#each workspace.columns as column (column.id)}
        <Column {column} tasks={openByColumn.get(column.id) ?? []} {onrename} {onaddcolumn} {onedit} onadd={() => onaddtask(column.id)} />
      {/each}
    </div>
    {#if moreRight}
      <div class="scroll-fade" aria-hidden="true"></div>
    {/if}
  </div>
{/if}

<style>
  .board-wrap {
    position: relative;
    height: 100%;
  }

  .board {
    display: flex;
    align-items: flex-start;
    gap: 14px;
    height: 100%;
    padding: 0;
    overflow: auto;
  }

  .scroll-fade {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    width: 40px;
    pointer-events: none;
    background: linear-gradient(to right, transparent, var(--bg));
  }

  .center {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
  }
</style>

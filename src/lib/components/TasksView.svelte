<script lang="ts">
  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Column } from '$lib/types';
  import Board from './Board.svelte';
  import TaskList from './TaskList.svelte';
  import Toolbar from './Toolbar.svelte';
  import ViewToggle from './ViewToggle.svelte';

  interface Props {
    onaddcolumn: () => void;
    onrename: (column: Column) => void;
  }

  let { onaddcolumn, onrename }: Props = $props();

  const summary = $derived(
    workspace.openCount === 0 && workspace.doneCount === 0
      ? 'No tasks yet'
      : `${workspace.openCount} open · ${workspace.doneCount} done`
  );
</script>

<section class="view">
  <header class="view-head" data-tauri-drag-region>
    <div class="titles">
      <span class="title">Tasks</span>
      <span class="summary">{summary}</span>
    </div>
    <ViewToggle />
  </header>

  <Toolbar {onaddcolumn} />

  <div class="body">
    {#if workspace.viewMode === 'board'}
      <Board {onrename} {onaddcolumn} />
    {:else}
      <TaskList />
    {/if}
  </div>
</section>

<style>
  .view {
    display: flex;
    flex-direction: column;
    height: 100%;
    min-height: 0;
  }

  .view-head {
    display: flex;
    flex-shrink: 0;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    min-height: 68px;
    padding: 12px 28px;
    border-bottom: 1px solid var(--border);
    background: color-mix(in srgb, var(--bg) 92%, var(--panel));
  }

  .titles {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .title {
    font-size: 20px;
    font-weight: 700;
    letter-spacing: -0.03em;
  }

  .summary {
    color: var(--muted-fg);
    font-size: 12px;
  }

  .body {
    flex: 1;
    min-height: 0;
  }
</style>

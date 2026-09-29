<script lang="ts">
  import { Columns3, List } from '@lucide/svelte';

  import { workspace } from '$lib/stores/workspace.svelte';
</script>

<div class="toggle" role="tablist" aria-label="View">
  <button
    class="segment"
    class:active={workspace.viewMode === 'board'}
    role="tab"
    aria-selected={workspace.viewMode === 'board'}
    onclick={() => workspace.setViewMode('board')}
  >
    <Columns3 size={14} /> Board
  </button>
  <button
    class="segment"
    class:active={workspace.viewMode === 'list'}
    role="tab"
    aria-selected={workspace.viewMode === 'list'}
    onclick={() => workspace.setViewMode('list')}
  >
    <List size={14} /> List
  </button>
</div>

<style>
  .toggle {
    display: inline-flex;
    gap: 2px;
    padding: 3px;
    border: 1px solid color-mix(in srgb, var(--border) 76%, transparent);
    border-radius: var(--radius-sm);
    background: var(--lane);
  }

  .segment {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    height: 30px;
    padding: 0 10px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--muted-fg);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
  }

  .segment:hover {
    color: var(--fg);
  }

  .segment.active {
    background: color-mix(in srgb, var(--accent) 14%, var(--panel));
    color: var(--fg);
    box-shadow: var(--shadow-sm);
  }
</style>

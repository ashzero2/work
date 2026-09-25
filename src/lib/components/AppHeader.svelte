<script lang="ts">
  import { Moon, SquareKanban, Sun } from '@lucide/svelte';

  import { workspace } from '$lib/stores/workspace.svelte';
  import { toggleMode, type ThemeMode } from '$lib/theme';
  import ViewToggle from './ViewToggle.svelte';

  interface Props {
    mode: ThemeMode;
    onmode: (mode: ThemeMode) => void;
  }

  let { mode, onmode }: Props = $props();

  const summary = $derived(
    workspace.openCount === 0 && workspace.doneCount === 0
      ? 'No tasks yet'
      : `${workspace.openCount} open · ${workspace.doneCount} done`
  );
</script>

<header class="header">
  <div class="identity">
    <div class="mark"><SquareKanban size={17} /></div>
    <div class="titles">
      <span class="title">Tasks</span>
      <span class="summary">{summary}</span>
    </div>
  </div>

  <div class="actions">
    <ViewToggle />
    <div class="divider"></div>
    <button
      class="icon-btn"
      title={mode === 'dark' ? 'Use light mode' : 'Use dark mode'}
      aria-label="Toggle theme"
      onclick={() => onmode(toggleMode(mode))}
    >
      {#if mode === 'dark'}<Sun size={16} />{:else}<Moon size={16} />{/if}
    </button>
  </div>
</header>

<style>
  .header {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    height: 56px;
    padding: 0 20px;
    border-bottom: 1px solid var(--border);
  }

  .identity {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .mark {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    border-radius: 8px;
    background: var(--accent);
    color: var(--accent-fg);
  }

  .titles {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .title {
    font-size: 13px;
    font-weight: 600;
  }

  .summary {
    color: var(--muted-fg);
    font-size: 11px;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .divider {
    width: 1px;
    height: 20px;
    background: var(--border);
  }
</style>

<script lang="ts">
  import { Plus } from '@lucide/svelte';

  import { workspace } from '$lib/stores/workspace.svelte';
  import TargetColumnPicker from './TargetColumnPicker.svelte';

  interface Props {
    onaddcolumn: () => void;
  }

  let { onaddcolumn }: Props = $props();

  let title = $state('');

  function submit(): void {
    const value = title.trim();
    if (!value) return;
    void workspace.addTask(value);
    title = '';
  }

  function onKeydown(event: KeyboardEvent): void {
    if (event.key === 'Enter') submit();
  }
</script>

<div class="toolbar">
  <input class="add-input" bind:value={title} onkeydown={onKeydown} placeholder="Add a task…" />
  <TargetColumnPicker />
  <button class="btn btn-primary" onclick={submit} disabled={!title.trim()}>
    <Plus size={15} /> Add task
  </button>
  {#if workspace.viewMode === 'board'}
    <button class="btn btn-ghost" onclick={onaddcolumn}><Plus size={15} /> Add column</button>
  {/if}
</div>

<style>
  .toolbar {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    gap: 8px;
    padding: 12px 20px;
    border-bottom: 1px solid var(--border);
  }

  .add-input {
    flex: 1;
    min-width: 0;
    height: 32px;
    padding: 0 11px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--panel);
    color: var(--fg);
    font-size: 13px;
  }

  .add-input::placeholder {
    color: var(--muted-fg);
  }

  .add-input:focus {
    border-color: var(--accent);
    outline: none;
  }
</style>

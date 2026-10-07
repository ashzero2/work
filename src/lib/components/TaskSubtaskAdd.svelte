<script lang="ts">
  import { Check, X } from '@lucide/svelte';

  import { workspace } from '$lib/stores/workspace.svelte';

  let { parentId, onclose }: { parentId: number; onclose: () => void } = $props();
  let title = $state('');
  let saving = $state(false);
  let input = $state<HTMLInputElement | null>(null);

  $effect(() => {
    requestAnimationFrame(() => input?.focus());
  });

  async function add(): Promise<void> {
    if (saving || !title.trim()) return;
    saving = true;
    const added = await workspace.addSubtask(parentId, title);
    saving = false;
    if (added) onclose();
  }
</script>

<div class="add-row">
  <input
    bind:this={input}
    bind:value={title}
    aria-label="New subtask"
    placeholder="Subtask title"
    autocomplete="off"
    onkeydown={(event) => {
      if (event.key === 'Enter') { event.preventDefault(); void add(); }
      if (event.key === 'Escape') onclose();
    }}
  />
  <button class="icon-btn save" aria-label="Add subtask" title="Add subtask" disabled={saving || !title.trim()} onclick={() => void add()}><Check size={14} /></button>
  <button class="icon-btn" aria-label="Cancel adding subtask" title="Cancel" onclick={onclose}><X size={14} /></button>
</div>

<style>
  .add-row { display: flex; align-items: center; gap: 3px; width: 100%; max-width: 360px; }
  .add-row input { flex: 1; width: 100%; min-width: 60px; padding: 5px 7px; border: 1px solid var(--field-border); border-radius: var(--radius-sm); background: var(--bg); color: var(--fg); font: inherit; font-size: var(--text-sm); }
  .save { color: var(--accent); }
</style>

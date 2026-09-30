<script lang="ts">
  import { onMount } from 'svelte';
  import { createTask } from '$lib/api';
  import { workspace } from '$lib/stores/workspace.svelte';
  import { toast } from '$lib/stores/toast.svelte';
  import type { Priority } from '$lib/types';
  import DateTimeField from './DateTimeField.svelte';

  let { columnId, onclose }: { columnId: number | null; onclose: () => void } = $props();
  let title = $state('');
  let description = $state('');
  let priority = $state<Priority>('none');
  let due = $state('');
  let selectedColumn = $state<number | null>(null);
  let saving = $state(false);
  let dialog: HTMLDialogElement;
  onMount(() => {
    selectedColumn = columnId ?? workspace.targetColumnId ?? workspace.columns[0]?.id ?? null;
    dialog.showModal();
  });
  async function save(): Promise<void> {
    if (!title.trim() || selectedColumn === null || saving) return;
    saving = true;
    try {
      await createTask({ title: title.trim(), description: description.trim() || null,
        columnId: selectedColumn, priority, dueAt: due ? new Date(due).toISOString() : null,
        repeatRule: null, parentTaskId: null });
      await workspace.refresh();
      onclose();
    } catch (error) { toast.show(String(error)); }
    finally { saving = false; }
  }
</script>

<dialog bind:this={dialog} onclose={onclose} oncancel={onclose}>
  <form onsubmit={(event) => { event.preventDefault(); void save(); }}>
    <h2>New task</h2>
    <label>Title<input bind:value={title} placeholder="Give it a clear name" required /></label>
    <label>Details<textarea bind:value={description} rows="3" placeholder="Add a short description"></textarea></label>
    <div class="fields">
      <label>Column<select bind:value={selectedColumn}>{#each workspace.columns as column}<option value={column.id}>{column.name}</option>{/each}</select></label>
      <label>Priority<select bind:value={priority}><option value="none">None</option><option value="low">Low</option><option value="medium">Medium</option><option value="high">High</option></select></label>
    </div>
    <DateTimeField bind:value={due} />
    <footer><button type="button" class="btn" onclick={onclose}>Cancel</button><button class="btn btn-primary" disabled={saving || !title.trim() || selectedColumn === null}>{saving ? 'Creating…' : 'Create'}</button></footer>
  </form>
</dialog>

<style>
  dialog { width: min(500px, calc(100vw - 40px)); max-height: calc(100vh - 40px); padding: 24px; border: 1px solid var(--border); border-radius: var(--radius-lg); background: var(--panel); color: var(--fg); box-shadow: var(--shadow-md); }
  dialog::backdrop { background: #0005; }
  form, label { display: flex; flex-direction: column; gap: 8px; }
  form { gap: 16px; } h2 { margin: 0; font-size: 20px; }
  label { font-size: 13px; font-weight: 600; }
  input, textarea, select { width: 100%; min-width: 0; padding: 9px 10px; border: 1px solid var(--field-border); border-radius: var(--radius-sm); background: var(--bg); color: var(--fg); font: inherit; font-weight: 400; }
  select { appearance: none; padding-right: 32px; background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath d='m3 5 3 3 3-3' fill='none' stroke='%23888' stroke-width='1.5'/%3E%3C/svg%3E"); background-repeat: no-repeat; background-position: right 10px center; }
  textarea { resize: vertical; } .fields { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
  footer { display: flex; justify-content: flex-end; gap: 8px; }
  @media (max-width: 520px) { dialog { padding: 18px; } .fields { grid-template-columns: 1fr; } }
</style>

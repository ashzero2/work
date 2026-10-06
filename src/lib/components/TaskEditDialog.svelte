<script lang="ts">
  import { onMount } from 'svelte';

  import { toDatetimeLocal } from '$lib/format';
  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Priority, RepeatRule, Task } from '$lib/types';
  import DateTimeField from './DateTimeField.svelte';

  let { task, onclose }: { task: Task; onclose: () => void } = $props();

  // svelte-ignore state_referenced_locally — the dialog is mounted per task, so these seed once
  let title = $state(task.title);
  // svelte-ignore state_referenced_locally
  let description = $state(task.description ?? '');
  // svelte-ignore state_referenced_locally
  let priority = $state<Priority>(task.priority);
  // svelte-ignore state_referenced_locally
  let due = $state(task.dueAt ? toDatetimeLocal(task.dueAt) : '');
  // svelte-ignore state_referenced_locally
  let repeat = $state<RepeatRule | 'none'>(task.repeatRule ?? 'none');
  let saving = $state(false);
  let dialog: HTMLDialogElement;

  onMount(() => dialog.showModal());

  async function save(): Promise<void> {
    const trimmed = title.trim();
    if (!trimmed || saving) return;
    saving = true;
    const saved = await workspace.updateTask(task.id, {
      title: trimmed,
      description: description.trim() || null,
      priority,
      dueAt: due ? new Date(due).toISOString() : null,
      repeatRule: repeat === 'none' ? null : repeat,
      parentTaskId: task.parentTaskId
    });
    saving = false;
    if (saved) onclose();
  }
</script>

<dialog bind:this={dialog} onclose={onclose} oncancel={onclose}>
  <form onsubmit={(event) => { event.preventDefault(); void save(); }}>
    <h2>Edit task</h2>

    <label>
      Title
      <input bind:value={title} required autocomplete="off" />
    </label>

    <label>Details<textarea bind:value={description} rows="3" placeholder="Add a short description"></textarea></label>

    <div class="fields">
      <label>
        Priority
        <select bind:value={priority}>
          <option value="none">None</option>
          <option value="low">Low</option>
          <option value="medium">Medium</option>
          <option value="high">High</option>
        </select>
      </label>
      <label>
        Repeats
        <select bind:value={repeat}>
          <option value="none">Never</option>
          <option value="daily">Daily</option>
          <option value="weekly">Weekly</option>
          <option value="monthly">Monthly</option>
        </select>
      </label>
    </div>

    <DateTimeField bind:value={due} />

    <footer>
      <button type="button" class="btn" onclick={onclose}>Cancel</button>
      <button class="btn btn-primary" disabled={saving || !title.trim()}>
        {saving ? 'Saving…' : 'Save'}
      </button>
    </footer>
  </form>
</dialog>

<style>
  dialog { width: min(500px, calc(100vw - 40px)); max-height: calc(100vh - 40px); padding: 24px; border: 1px solid var(--border); border-radius: var(--radius-lg); background: var(--panel); color: var(--fg); box-shadow: var(--shadow-md); }
  dialog::backdrop { background: #0005; }
  form, label { display: flex; flex-direction: column; gap: 8px; }
  form { gap: 16px; } h2 { margin: 0; font-size: var(--text-xl); letter-spacing: var(--tracking-tight); }
  label { font-size: var(--text-base); font-weight: var(--weight-semibold); }
  input, textarea, select { width: 100%; min-width: 0; padding: 9px 10px; border: 1px solid var(--field-border); border-radius: var(--radius-sm); background: var(--bg); color: var(--fg); font: inherit; font-weight: var(--weight-normal); }
  select { appearance: none; padding-right: 32px; background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath d='m3 5 3 3 3-3' fill='none' stroke='%23888' stroke-width='1.5'/%3E%3C/svg%3E"); background-repeat: no-repeat; background-position: right 10px center; }
  textarea { resize: vertical; } .fields { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
  footer { display: flex; justify-content: flex-end; gap: 8px; }
  @media (max-width: 520px) { dialog { padding: 18px; } .fields { grid-template-columns: 1fr; } }
</style>

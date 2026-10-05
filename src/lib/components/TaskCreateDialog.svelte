<script lang="ts">
  import { onMount } from 'svelte';
  import { CalendarDays, X } from '@lucide/svelte';

  import { createTask } from '$lib/api';
  import { dueChip, priorityLabel, priorityTone } from '$lib/format';
  import { workspace } from '$lib/stores/workspace.svelte';
  import { toast } from '$lib/stores/toast.svelte';
  import { parseTaskInput, stripFragments } from '$lib/task-parse';
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

  // Rejection is held as the fragment text rather than a flag, so typing a *different* date revives
  // parsing while re-typing the rejected phrase stays rejected.
  let rejectedDate = $state<string | null>(null);
  let rejectedPriority = $state<string | null>(null);

  const parsed = $derived(parseTaskInput(title));
  const dateActive = $derived(parsed.dateText !== null && parsed.dateText !== rejectedDate);
  const priorityActive = $derived(
    parsed.priorityText !== null && parsed.priorityText !== rejectedPriority
  );

  // A rejected fragment stays in the title; only the accepted ones are stripped out of it.
  const effectiveTitle = $derived(
    stripFragments(title, [
      dateActive ? parsed.dateText : null,
      priorityActive ? parsed.priorityText : null
    ])
  );

  const dateLabel = $derived.by(() => {
    if (!parsed.due) return '';
    const day = dueChip(new Date(parsed.due).toISOString()).label;
    if (parsed.allDay) return day;
    const time = new Date(parsed.due).toLocaleTimeString(undefined, {
      hour: 'numeric',
      minute: '2-digit'
    });
    return `${day}, ${time}`;
  });

  $effect(() => {
    if (dateActive) due = parsed.due;
    else if (parsed.dateText === null && rejectedDate === null) due = '';
  });

  $effect(() => {
    if (priorityActive) priority = parsed.priority;
    else if (parsed.priorityText === null && rejectedPriority === null) priority = 'none';
  });

  function rejectDate(): void {
    rejectedDate = parsed.dateText;
    due = '';
  }

  function rejectPriority(): void {
    rejectedPriority = parsed.priorityText;
    priority = 'none';
  }

  onMount(() => {
    selectedColumn = columnId ?? workspace.targetColumnId ?? workspace.columns[0]?.id ?? null;
    dialog.showModal();
  });

  async function save(): Promise<void> {
    if (!effectiveTitle || selectedColumn === null || saving) return;
    saving = true;
    try {
      await createTask({ title: effectiveTitle, description: description.trim() || null,
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
    <label>
      Title
      <input
        bind:value={title}
        placeholder="Call the dentist tomorrow at 3pm !!"
        required
        autocomplete="off"
      />
    </label>

    {#if dateActive || priorityActive}
      <div class="detected">
        <span class="detected-label">Detected</span>
        {#if dateActive}
          <button
            type="button"
            class="chip chip-info"
            title="Stop reading this as a due date"
            aria-label={`Remove due date ${dateLabel}`}
            onclick={rejectDate}
          >
            <CalendarDays size={11} />{dateLabel}<X size={11} />
          </button>
        {/if}
        {#if priorityActive}
          <button
            type="button"
            class="chip chip-{priorityTone[parsed.priority] ?? 'muted'}"
            title="Stop reading this as a priority"
            aria-label={`Remove priority ${priorityLabel[parsed.priority]}`}
            onclick={rejectPriority}
          >
            {priorityLabel[parsed.priority]}<X size={11} />
          </button>
        {/if}
      </div>
    {/if}

    <label>Details<textarea bind:value={description} rows="3" placeholder="Add a short description"></textarea></label>
    <div class="fields">
      <label>Column<select bind:value={selectedColumn}>{#each workspace.columns as column}<option value={column.id}>{column.name}</option>{/each}</select></label>
      <label>Priority<select bind:value={priority} onchange={() => (rejectedPriority = parsed.priorityText)}><option value="none">None</option><option value="low">Low</option><option value="medium">Medium</option><option value="high">High</option></select></label>
    </div>
    <DateTimeField bind:value={due} />
    <footer><button type="button" class="btn" onclick={onclose}>Cancel</button><button class="btn btn-primary" disabled={saving || !effectiveTitle || selectedColumn === null}>{saving ? 'Creating…' : 'Create'}</button></footer>
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
  .detected { display: flex; flex-wrap: wrap; align-items: center; gap: 6px; margin-top: -10px; }
  .detected-label { color: var(--muted-fg); font-size: 12px; font-weight: 500; }
  .chip { border: 0; background: color-mix(in srgb, var(--tone) 16%, transparent); color: var(--tone); font: inherit; font-size: 11px; font-weight: 600; cursor: pointer; }
  .chip:hover { background: color-mix(in srgb, var(--tone) 26%, transparent); }
  @media (max-width: 520px) { dialog { padding: 18px; } .fields { grid-template-columns: 1fr; } }
</style>

<script lang="ts">
  import { BellOff, Clock, Plus, Settings, Trash2 } from '@lucide/svelte';

  import { formatMoment, fromDatetimeLocal, toDatetimeLocal } from '$lib/format';
  import { kindHint, kindLabel, reminders, statusLabel } from '$lib/stores/reminders.svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Reminder } from '$lib/types';
  import ReminderDialog from './ReminderDialog.svelte';

  let creating = $state(false);
  let editing = $state<number | null>(null);
  let editedWhen = $state('');

  const taskTitle = (taskId: number | null): string | null =>
    taskId === null ? null : (workspace.tasks.find((task) => task.id === taskId)?.title ?? null);

  function startEditing(reminder: Reminder): void {
    editing = reminder.id;
    editedWhen = toDatetimeLocal(reminder.triggerAt);
  }

  async function saveEdit(id: number): Promise<void> {
    await reminders.reschedule(id, fromDatetimeLocal(editedWhen));
    editing = null;
  }
</script>

<section class="view">
  <header class="view-head" data-tauri-drag-region>
    <div class="titles">
      <span class="title">Reminders</span>
      <span class="summary">
        {#if reminders.outstanding.length === 0}
          Nothing scheduled
        {:else}
          {reminders.outstanding.length} upcoming
        {/if}
      </span>
    </div>
    <button class="btn btn-primary" onclick={() => (creating = true)}>
      <Plus size={15} /> New reminder
    </button>
  </header>

  <div class="body">
    {#if !reminders.available}
      <div class="banner">
        <BellOff size={16} />
        <div class="banner-text">
          <strong>Notifications aren’t available here.</strong>
          <p>
            macOS delivers reminders, and it needs the packaged app rather than a dev server. Build
            the app and reminders will work.
          </p>
        </div>
      </div>
    {:else if reminders.authorized === false}
      <div class="banner">
        <BellOff size={16} />
        <div class="banner-text">
          <strong>Notifications are turned off for Work Dashboard.</strong>
          <p>Reminders are saved, but macOS won’t show them until notifications are allowed.</p>
        </div>
        <button class="btn" onclick={() => void reminders.openSettings()}>
          <Settings size={14} /> Settings
        </button>
      </div>
    {/if}

    <div class="block">
      <span class="block-label">Notify me about</span>
      <div class="toggles">
        {#each reminders.toggles as toggle (toggle.kind)}
          <label class="toggle">
            <input
              type="checkbox"
              checked={toggle.enabled}
              onchange={(event) =>
                void reminders.setEnabled(toggle.kind, event.currentTarget.checked)}
            />
            <span class="toggle-text">
              <span class="toggle-label">{kindLabel[toggle.kind]}</span>
              <span class="toggle-hint">{kindHint[toggle.kind]}</span>
            </span>
          </label>
        {/each}
      </div>
    </div>

    <div class="block">
      <span class="block-label">Scheduled</span>
      {#if reminders.outstanding.length === 0}
        <p class="hint">Nothing scheduled. Use New reminder to add one.</p>
      {:else}
        {#each reminders.outstanding as reminder (reminder.id)}
          <div class="row">
            <Clock size={14} />
            <span class="row-main">
              <span class="row-kind">{kindLabel[reminder.kind]}</span>
              {#if taskTitle(reminder.taskId)}
                <span class="row-task">{taskTitle(reminder.taskId)}</span>
              {/if}
            </span>
            {#if reminder.status === 'snoozed'}
              <span class="chip">{statusLabel.snoozed}</span>
            {/if}
            {#if editing === reminder.id}
              <input type="datetime-local" bind:value={editedWhen} />
              <button class="btn tiny" onclick={() => (editing = null)}>Cancel</button>
              <button class="btn tiny btn-primary" onclick={() => void saveEdit(reminder.id)}>
                Save
              </button>
            {:else}
              <span class="row-when">{formatMoment(reminder.triggerAt)}</span>
              <button class="btn tiny" onclick={() => startEditing(reminder)}>Change</button>
              <button class="btn tiny" onclick={() => void reminders.snooze(reminder.id)}>
                Snooze
              </button>
              <button class="btn tiny" onclick={() => void reminders.dismiss(reminder.id)}>
                Dismiss
              </button>
            {/if}
            <button
              class="icon-btn"
              aria-label="Delete reminder"
              title="Delete reminder"
              onclick={() => void reminders.remove(reminder.id)}
            >
              <Trash2 size={13} />
            </button>
          </div>
        {/each}
      {/if}
    </div>

    {#if reminders.settled.length > 0}
      <div class="block">
        <span class="block-label">Earlier</span>
        {#each reminders.settled as reminder (reminder.id)}
          <div class="row settled">
            <Clock size={14} />
            <span class="row-main">
              <span class="row-kind">{kindLabel[reminder.kind]}</span>
              {#if taskTitle(reminder.taskId)}
                <span class="row-task">{taskTitle(reminder.taskId)}</span>
              {/if}
            </span>
            <span class="row-when">{formatMoment(reminder.triggerAt)}</span>
            <span class="chip">{statusLabel[reminder.status]}</span>
            <button
              class="icon-btn"
              aria-label="Delete reminder"
              title="Delete reminder"
              onclick={() => void reminders.remove(reminder.id)}
            >
              <Trash2 size={13} />
            </button>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</section>

{#if creating}
  <ReminderDialog onclose={() => (creating = false)} />
{/if}

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
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    min-height: 56px;
    padding: 8px 20px;
    border-bottom: 1px solid var(--border);
  }

  .titles {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .title {
    font-size: 15px;
    font-weight: 600;
  }

  .summary {
    color: var(--muted-fg);
    font-size: 11px;
  }

  .body {
    display: flex;
    flex: 1;
    min-height: 0;
    flex-direction: column;
    gap: 24px;
    width: 100%;
    max-width: 620px;
    margin: 0 auto;
    padding: 28px 20px;
    overflow-y: auto;
  }

  .banner {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    padding: 12px 14px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--lane);
    color: var(--fg);
  }

  .banner-text {
    flex: 1;
    min-width: 0;
  }

  .banner strong {
    font-size: 13px;
  }

  .banner p {
    margin: 3px 0 0;
    color: var(--muted-fg);
    font-size: 12px;
    line-height: 1.45;
  }

  .block {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .block-label {
    color: var(--muted-fg);
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.03em;
    text-transform: uppercase;
  }

  .toggles {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .toggle {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    padding: 7px 9px;
    border-radius: var(--radius-sm);
    cursor: pointer;
  }

  .toggle:hover {
    background: var(--lane);
  }

  .toggle input {
    margin-top: 2px;
    accent-color: var(--accent);
  }

  .toggle-text {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .toggle-label {
    font-size: 13px;
  }

  .toggle-hint {
    color: var(--muted-fg);
    font-size: 11px;
  }

  .hint {
    margin: 0;
    color: var(--muted-fg);
    font-size: 12px;
  }

  .row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 7px 9px;
    border-radius: var(--radius-sm);
    font-size: 12px;
  }

  .row:hover {
    background: var(--lane);
  }

  .row.settled {
    color: var(--muted-fg);
  }

  .row-main {
    display: flex;
    flex: 1;
    min-width: 0;
    flex-direction: column;
    gap: 1px;
  }

  .row-kind {
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .row-task {
    color: var(--muted-fg);
    font-size: 11px;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .row-when {
    color: var(--muted-fg);
    font-variant-numeric: tabular-nums;
  }

  .chip {
    padding: 1px 7px;
    border-radius: 999px;
    background: var(--selection-bg);
    color: var(--selection-fg);
    font-size: 11px;
  }

  input[type='datetime-local'] {
    height: 28px;
    padding: 0 8px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg);
    color: var(--fg);
    font-size: 12px;
    font-variant-numeric: tabular-nums;
  }
</style>

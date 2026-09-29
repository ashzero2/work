<script lang="ts">
  import { BellOff, Clock, Plus, Settings, Trash2 } from '@lucide/svelte';

  import { formatMoment, fromDatetimeLocal, toDatetimeLocal } from '$lib/format';
  import { kindLabel, reminders, statusLabel } from '$lib/stores/reminders.svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Reminder } from '$lib/types';
  import NotificationKindToggles from './NotificationKindToggles.svelte';
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
      <NotificationKindToggles />
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
              <span class="chip chip-muted">{statusLabel.snoozed}</span>
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
            <span class="chip chip-muted">{statusLabel[reminder.status]}</span>
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
    display: flex;
    flex: 1;
    min-height: 0;
    flex-direction: column;
    gap: 32px;
    width: 100%;
    max-width: 820px;
    margin: 0 auto;
    padding: 40px 28px 56px;
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
    font-size: 13px;
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

  .hint {
    margin: 0;
    color: var(--muted-fg);
    font-size: 13px;
  }

  .row {
    display: flex;
    align-items: center;
    gap: 8px;
    min-height: 48px;
    padding: 9px 12px;
    border-radius: var(--radius-sm);
    font-size: 13px;
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

  /* Four controls per row is heavier than the information in it, so the actions
     appear on hover — and on focus, which is how a keyboard reaches them. */
  .row .btn,
  .row .icon-btn {
    opacity: 0;
    transition: opacity var(--motion-fast) var(--ease);
  }

  .row:hover .btn,
  .row:hover .icon-btn,
  .row:focus-within .btn,
  .row:focus-within .icon-btn {
    opacity: 1;
  }

  input[type='datetime-local'] {
    height: 28px;
    padding: 0 8px;
    border: 1px solid var(--field-border);
    border-radius: var(--radius-sm);
    background: var(--bg);
    color: var(--fg);
    font-size: 13px;
    font-variant-numeric: tabular-nums;
  }
</style>

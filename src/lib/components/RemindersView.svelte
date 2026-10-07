<script lang="ts">
  import { BellOff, Clock, Ellipsis, Plus, Settings, Trash2 } from '@lucide/svelte';

  import { showSettingsWindow } from '$lib/api';
  import { formatMoment, fromDatetimeLocal, toDatetimeLocal } from '$lib/format';
  import { kindLabel, reminders, statusLabel } from '$lib/stores/reminders.svelte';
  import { toast } from '$lib/stores/toast.svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Reminder } from '$lib/types';
  import NotificationKindToggles from './NotificationKindToggles.svelte';
  import ReminderDialog from './ReminderDialog.svelte';
  import DropdownMenu from './DropdownMenu.svelte';

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
    const when = fromDatetimeLocal(editedWhen);
    if (when === null) {
      toast.show('Choose a valid date and time before saving.');
      return;
    }
    await reminders.reschedule(id, when);
    editing = null;
  }
</script>

<section class="workspace-view">
  <header class="workspace-head" data-tauri-drag-region>
    <div class="titles">
      <h1>Reminders</h1>
      <p>
        {#if reminders.outstanding.length === 0}
          Nothing scheduled
        {:else}
          {reminders.outstanding.length} upcoming
        {/if}
      </p>
    </div>
    <button class="btn btn-primary" onclick={() => (creating = true)}>
      <Plus size={15} /> New reminder
    </button>
  </header>

  <div class="workspace-body body">
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
          <strong>Notifications are turned off for Tittle.</strong>
          <p>Reminders are saved, but macOS won’t show them until notifications are allowed.</p>
        </div>
        <button class="btn" onclick={() => void reminders.openSettings()}>
          <Settings size={14} /> Settings
        </button>
      </div>
    {:else}
      <div class="banner"><div class="banner-text"><strong>Notifications on</strong><p>Reminders appear through the operating system.</p></div><button class="btn" onclick={() => void showSettingsWindow('notifications')}>Notification settings</button></div>
    {/if}

    <details><summary>Notify me about</summary><NotificationKindToggles /></details>

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
            {/if}
            <DropdownMenu align="end">
              {#snippet trigger({ toggle })}<button class="icon-btn" aria-label="Reminder actions" onclick={toggle}><Ellipsis size={15} /></button>{/snippet}
              {#snippet content({ close })}
                <button class="menu-item" onclick={() => { close(); void reminders.snooze(reminder.id); }}>Snooze</button>
                <button class="menu-item" onclick={() => { close(); void reminders.dismiss(reminder.id); }}>Dismiss</button>
                <button class="menu-item danger" onclick={() => { close(); void reminders.remove(reminder.id); }}>Delete</button>
              {/snippet}
            </DropdownMenu>
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
  .body { display: flex; flex-direction: column; gap: 18px; }
  details { font-size: var(--text-base); } summary { cursor: pointer; margin-bottom: 8px; color: var(--muted-fg); }
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
    font-size: var(--text-base);
  }

  .banner p {
    margin: 3px 0 0;
    color: var(--muted-fg);
    font-size: var(--text-base);
    line-height: var(--leading-normal);
  }

  .block {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .block-label {
    color: var(--muted-fg);
    font-size: var(--text-2xs);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-label);
    text-transform: uppercase;
  }

  .hint {
    margin: 0;
    color: var(--muted-fg);
    font-size: var(--text-base);
  }

  .row {
    display: flex;
    align-items: center;
    gap: 8px;
    min-height: 44px;
    padding: 9px 12px;
    border-bottom: 1px solid var(--divider);
    background: var(--panel);
    font-size: var(--text-base);
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
    font-size: var(--text-xs);
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .row-when {
    color: var(--muted-fg);
    font-variant-numeric: tabular-nums;
  }

  .row .btn,
  .row .icon-btn {
    opacity: 1;
    transition: opacity var(--motion-fast) var(--ease);
  }

  .row:hover .btn,
  .row:hover .icon-btn,
  .row:focus-within .btn,
  .row:focus-within .icon-btn {
    opacity: 1;
  }

  input[type='datetime-local'] {
    height: 30px;
    padding: 0 8px;
    border: 1px solid var(--field-border);
    border-radius: var(--radius-sm);
    background: var(--bg);
    color: var(--fg);
    font-size: var(--text-base);
    font-variant-numeric: tabular-nums;
  }
</style>

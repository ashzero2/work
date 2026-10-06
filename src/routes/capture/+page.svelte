<script lang="ts">
  import { onMount } from 'svelte';

  import { CalendarDays, X, Zap } from '@lucide/svelte';

  import { captureTask, hideCaptureWindow } from '$lib/api';
  import { dueChip, priorityLabel, priorityTone } from '$lib/format';
  import { parseTaskInput, stripFragments } from '$lib/task-parse';
  import { initTheme } from '$lib/theme';

  let title = $state('');
  let input: HTMLInputElement | null = null;
  let saving = $state(false);
  let error = $state('');

  // Rejection is held as the fragment text rather than a flag, so typing a
  // *different* date revives parsing while re-typing the rejected phrase stays
  // rejected — the same rule the new-task dialog uses.
  let rejectedDate = $state<string | null>(null);
  let rejectedPriority = $state<string | null>(null);

  const parsed = $derived(parseTaskInput(title));
  const dateActive = $derived(parsed.dateText !== null && parsed.dateText !== rejectedDate);
  const priorityActive = $derived(
    parsed.priorityText !== null && parsed.priorityText !== rejectedPriority
  );

  // Only the accepted fragments are stripped, so a rejected one stays in the title.
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

  onMount(async () => {
    // This window has its own document, so it applies the theme itself —
    // otherwise it would be the one surface that ignores dark mode.
    await initTheme();
    input?.focus();
  });

  async function save(): Promise<void> {
    const text = effectiveTitle.trim();
    if (text.length === 0 || saving) return;
    saving = true;
    error = '';
    try {
      await captureTask(
        text,
        priorityActive ? parsed.priority : 'none',
        dateActive && parsed.due ? new Date(parsed.due).toISOString() : null
      );
      title = '';
      rejectedDate = null;
      rejectedPriority = null;
    } catch (cause) {
      error = String(cause);
    } finally {
      saving = false;
    }
  }

  function rejectDate(): void {
    rejectedDate = parsed.dateText;
  }

  function rejectPriority(): void {
    rejectedPriority = parsed.priorityText;
  }

  function onKeydown(event: KeyboardEvent): void {
    if (event.key === 'Escape') void hideCaptureWindow();
  }
</script>

<svelte:window onkeydown={onKeydown} />

<main class="capture">
  <header class="drag" data-tauri-drag-region>
    <span class="mark" aria-hidden="true"><Zap size={13} /></span>
    <span class="title">Quick capture</span>
  </header>

  <input
    bind:this={input}
    bind:value={title}
    aria-label="Task title"
    placeholder="Call the dentist tomorrow at 3pm !!"
    spellcheck="false"
    autocomplete="off"
    onkeydown={(event) => {
      if (event.key === 'Enter') void save();
    }}
  />

  {#if dateActive || priorityActive}
    <div class="detected">
      <span class="detected-label">Detected</span>
      {#if dateActive}
        <button
          type="button"
          class="chip chip-info"
          title="Keep this in the title instead"
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
          title="Keep this in the title instead"
          aria-label={`Remove priority ${priorityLabel[parsed.priority]}`}
          onclick={rejectPriority}
        >
          {priorityLabel[parsed.priority]}<X size={11} />
        </button>
      {/if}
    </div>
  {/if}

  {#if error}<p class="error" role="alert">{error}</p>{/if}

  <div class="foot">
    <p class="hint">Enter adds it to your first column · Esc closes</p>
    <button
      class="btn btn-primary"
      disabled={!effectiveTitle.trim() || saving}
      onclick={() => void save()}
    >
      {saving ? 'Adding…' : 'Add task'}
    </button>
  </div>
</main>

<style>
  .capture {
    display: flex;
    flex-direction: column;
    height: 100vh;
    padding: 0 16px 14px;
    background: var(--panel);
    color: var(--fg);
  }

  .drag {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    gap: 8px;
    height: 40px;
    border-bottom: 1px solid var(--divider);
  }

  .mark {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    border-radius: var(--radius-xs);
    background: color-mix(in srgb, var(--accent) 14%, transparent);
    color: var(--accent);
  }

  .title {
    font-size: var(--text-base);
    font-weight: var(--weight-semibold);
  }

  input {
    height: 44px;
    margin-top: 14px;
    padding: 0 13px;
    border: 1px solid var(--field-border);
    border-radius: var(--radius);
    background: var(--bg);
    color: var(--fg);
    font-size: var(--text-lg);
  }

  input::placeholder {
    color: var(--muted-fg);
  }

  input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px var(--ring-soft);
    outline: none;
  }

  .detected {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 6px;
    margin-top: 10px;
  }

  .detected-label {
    color: var(--muted-fg);
    font-size: var(--text-xs);
  }

  /* The chip look comes from app.css; these are buttons, so the user-agent
     border and cursor have to be cleared. */
  .chip {
    border: 0;
    cursor: pointer;
  }

  .error {
    margin: 10px 0 0;
    color: var(--danger);
    font-size: var(--text-xs);
  }

  .foot {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-top: auto;
    padding-top: 12px;
  }

  .hint {
    flex: 1;
    min-width: 0;
    margin: 0;
    color: var(--muted-fg);
    font-size: var(--text-xs);
  }
</style>

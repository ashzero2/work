<script lang="ts">
  import { onMount } from 'svelte';

  import { captureTask, hideCaptureWindow } from '$lib/api';
  import { initTheme } from '$lib/theme';

  let title = $state('');
  let input: HTMLInputElement | null = null;
  let saving = $state(false);
  let error = $state('');

  onMount(async () => {
    // This window has its own document, so it applies the theme itself —
    // otherwise it would be the one surface that ignores dark mode.
    await initTheme();
    input?.focus();
  });

  async function save(): Promise<void> {
    const trimmed = title.trim();
    if (trimmed.length === 0 || saving) return;
    saving = true;
    error = '';
    try {
      await captureTask(trimmed);
      title = '';
    } catch (cause) {
      error = String(cause);
    } finally {
      saving = false;
    }
  }

  function onKeydown(event: KeyboardEvent): void {
    if (event.key === 'Escape') void hideCaptureWindow();
  }
</script>

<svelte:window onkeydown={onKeydown} />

<main class="capture">
  <div class="drag" data-tauri-drag-region>Quick capture / New task</div>
  <label for="capture-title">Task title</label>
  <input
    id="capture-title"
    bind:this={input}
    bind:value={title}
    aria-label="Task title"
    placeholder="Add a task…"
    spellcheck="false"
    onkeydown={(event) => {
      if (event.key === 'Enter') void save();
    }}
  />
  <p class="hint">Enter adds it to your first column · Esc closes</p>
  {#if error}<p class="hint" role="alert">{error}</p>{/if}
  <button class="btn btn-primary" disabled={!title.trim() || saving} onclick={() => void save()}>{saving ? 'Adding…' : 'Add task'}</button>
</main>

<style>
  .capture {
    display: flex;
    flex-direction: column;
    gap: 8px;
    height: 100vh;
    padding: 0 16px 14px;
    background: var(--panel);
    color: var(--fg);
  }

  .drag {
    flex-shrink: 0;
    padding: 12px 0;
    border-bottom: 1px solid var(--border);
    font-weight: var(--weight-semibold);
    font-size: var(--text-base);
  }

  label { font-size: var(--text-sm); font-weight: var(--weight-semibold); }
  .btn { align-self: flex-start; min-height: 32px; }

  input {
    height: 38px;
    padding: 0 12px;
    border: 1px solid var(--field-border);
    border-radius: var(--radius-sm);
    background: var(--bg);
    color: var(--fg);
    font-size: var(--text-base);
  }

  .hint {
    margin: 0;
    color: var(--muted-fg);
    font-size: var(--text-xs);
  }
</style>

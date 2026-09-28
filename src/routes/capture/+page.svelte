<script lang="ts">
  import { onMount } from 'svelte';

  import { captureTask, hideCaptureWindow } from '$lib/api';
  import { initTheme } from '$lib/theme';

  let title = $state('');
  let input: HTMLInputElement | null = null;
  let saving = $state(false);

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
    try {
      await captureTask(trimmed);
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
  <div class="drag" data-tauri-drag-region></div>
  <input
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
    height: 12px;
  }

  input {
    height: 38px;
    padding: 0 12px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg);
    color: var(--fg);
    font-size: 14px;
  }

  input:focus {
    border-color: var(--accent);
  }

  .hint {
    margin: 0;
    color: var(--muted-fg);
    font-size: 11px;
  }
</style>

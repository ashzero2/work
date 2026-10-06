<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';

  import CommandPalette from '$lib/components/CommandPalette.svelte';
  import FocusView from '$lib/components/FocusView.svelte';
  import NotesView from '$lib/components/NotesView.svelte';
  import PromptDialog from '$lib/components/PromptDialog.svelte';
  import RemindersView from '$lib/components/RemindersView.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import TasksView from '$lib/components/TasksView.svelte';
  import Toast from '$lib/components/Toast.svelte';
  import { navigation } from '$lib/stores/navigation.svelte';
  import { notes } from '$lib/stores/notes.svelte';
  import { pomodoro } from '$lib/stores/pomodoro.svelte';
  import { reminders } from '$lib/stores/reminders.svelte';
  import { theme } from '$lib/stores/theme.svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Column } from '$lib/types';

  type ColumnPrompt = { mode: 'create' } | { mode: 'rename'; column: Column };

  let prompt = $state<ColumnPrompt | null>(null);
  let paletteOpen = $state(false);

  onMount(async () => {
    await theme.load();
    await Promise.all([workspace.load(), notes.load(), pomodoro.load(), reminders.load()]);
  });

  // Backend changes this window didn't start: a notification's Snooze or
  // Dismiss button, a quick capture filed from the global shortcut, or a
  // preference changed in the settings window.
  $effect(() => {
    const pending = Promise.all([
      listen('reminders:changed', () => void reminders.load()),
      listen('tasks:changed', () => void workspace.load()),
      listen('settings:changed', () => {
        void theme.load();
        void pomodoro.load();
        void reminders.load();
      })
    ]);
    return () => {
      void pending.then((unlisten) => unlisten.forEach((off) => off()));
    };
  });

  function onWindowKeydown(event: KeyboardEvent): void {
    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === 'k') {
      event.preventDefault();
      paletteOpen = !paletteOpen;
    }
  }

  function openCreateColumn(): void {
    prompt = { mode: 'create' };
  }

  function openRenameColumn(column: Column): void {
    prompt = { mode: 'rename', column };
  }

  async function submitPrompt(value: string): Promise<void> {
    const current = prompt;
    if (current === null) return;

    const saved =
      current.mode === 'rename'
        ? await workspace.renameColumn(current.column.id, value)
        : await workspace.createColumn(value);

    // Keep the dialog open when the write failed, so the name isn't lost.
    if (saved) prompt = null;
  }
</script>

<svelte:window onkeydown={onWindowKeydown} />

<div class="app">
  <div class="shell">
    <Sidebar oncommand={() => paletteOpen = true} />
    <main class="content">
      {#if navigation.section === 'tasks'}
        <TasksView onaddcolumn={openCreateColumn} onrename={openRenameColumn} oncommand={() => paletteOpen = true} />
      {:else if navigation.section === 'notes'}
        <NotesView />
      {:else if navigation.section === 'focus'}
        <FocusView />
      {:else}
        <RemindersView />
      {/if}
    </main>
  </div>

  {#if paletteOpen}
    <CommandPalette onclose={() => (paletteOpen = false)} />
  {/if}

  <Toast />

  {#if prompt}
    <PromptDialog
      title={prompt.mode === 'create' ? 'New column' : 'Rename column'}
      confirmLabel={prompt.mode === 'create' ? 'Create column' : 'Save'}
      placeholder="Column name"
      initial={prompt.mode === 'rename' ? prompt.column.name : ''}
      onsubmit={submitPrompt}
      oncancel={() => (prompt = null)}
    />
  {/if}
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  .shell {
    display: flex;
    flex: 1;
    min-height: 0;
  }

  .content {
    flex: 1;
    min-width: 0;
    min-height: 0;
    overflow: hidden;
    background: var(--bg);
  }
</style>

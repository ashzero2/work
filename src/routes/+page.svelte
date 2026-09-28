<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';

  import FocusView from '$lib/components/FocusView.svelte';
  import NoteEditorDialog from '$lib/components/NoteEditorDialog.svelte';
  import NotesView from '$lib/components/NotesView.svelte';
  import PromptDialog from '$lib/components/PromptDialog.svelte';
  import RemindersView from '$lib/components/RemindersView.svelte';
  import SettingsView from '$lib/components/SettingsView.svelte';
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

  onMount(async () => {
    await theme.load();
    await Promise.all([workspace.load(), notes.load(), pomodoro.load(), reminders.load()]);
  });

  // Backend changes this window didn't start: a notification's Snooze or
  // Dismiss button, or a quick capture filed from the global shortcut.
  $effect(() => {
    const pending = Promise.all([
      listen('reminders:changed', () => void reminders.load()),
      listen('tasks:changed', () => void workspace.load())
    ]);
    return () => {
      void pending.then((unlisten) => unlisten.forEach((off) => off()));
    };
  });

  function openCreateColumn(): void {
    prompt = { mode: 'create' };
  }

  function openRenameColumn(column: Column): void {
    prompt = { mode: 'rename', column };
  }

  async function submitPrompt(value: string): Promise<void> {
    if (prompt?.mode === 'rename') {
      await workspace.renameColumn(prompt.column.id, value);
    } else {
      await workspace.createColumn(value);
    }
    prompt = null;
  }
</script>

<div class="app">
  <div class="shell">
    <Sidebar />
    <main class="content">
      {#if navigation.section === 'tasks'}
        <TasksView onaddcolumn={openCreateColumn} onrename={openRenameColumn} />
      {:else if navigation.section === 'notes'}
        <NotesView />
      {:else if navigation.section === 'focus'}
        <FocusView />
      {:else if navigation.section === 'reminders'}
        <RemindersView />
      {:else}
        <SettingsView />
      {/if}
    </main>
  </div>

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

  {#key notes.selectedId}
    {@const selectedNote = notes.selected}
    {#if selectedNote}
      <NoteEditorDialog note={selectedNote} onclose={() => notes.closeEditor()} />
    {/if}
  {/key}
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
  }
</style>

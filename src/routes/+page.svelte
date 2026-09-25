<script lang="ts">
  import { onMount } from 'svelte';

  import NoteEditorDialog from '$lib/components/NoteEditorDialog.svelte';
  import NotesView from '$lib/components/NotesView.svelte';
  import PromptDialog from '$lib/components/PromptDialog.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import TasksView from '$lib/components/TasksView.svelte';
  import Toast from '$lib/components/Toast.svelte';
  import { navigation } from '$lib/stores/navigation.svelte';
  import { notes } from '$lib/stores/notes.svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import { initTheme, setTheme, type ThemeMode, type ThemeState } from '$lib/theme';
  import type { Column } from '$lib/types';

  type ColumnPrompt = { mode: 'create' } | { mode: 'rename'; column: Column };

  let theme = $state<ThemeState>({ mode: 'light', appearance: 'light', macos: false });
  let prompt = $state<ColumnPrompt | null>(null);

  onMount(async () => {
    theme = await initTheme();
    await Promise.all([workspace.load(), notes.load()]);
  });

  async function ontheme(mode: ThemeMode): Promise<void> {
    theme = await setTheme(mode);
  }

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
    <Sidebar {theme} {ontheme} />
    <main class="content">
      {#if navigation.section === 'tasks'}
        <TasksView onaddcolumn={openCreateColumn} onrename={openRenameColumn} />
      {:else}
        <NotesView />
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

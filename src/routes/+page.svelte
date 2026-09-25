<script lang="ts">
  import { onMount } from 'svelte';

  import AppHeader from '$lib/components/AppHeader.svelte';
  import Board from '$lib/components/Board.svelte';
  import PromptDialog from '$lib/components/PromptDialog.svelte';
  import TaskList from '$lib/components/TaskList.svelte';
  import Toast from '$lib/components/Toast.svelte';
  import Toolbar from '$lib/components/Toolbar.svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import { initTheme, type ThemeMode } from '$lib/theme';
  import type { Column } from '$lib/types';

  type ColumnPrompt = { mode: 'create' } | { mode: 'rename'; column: Column };

  let themeMode = $state<ThemeMode>('light');
  let prompt = $state<ColumnPrompt | null>(null);

  onMount(async () => {
    themeMode = initTheme();
    await workspace.load();
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
  <AppHeader mode={themeMode} onmode={(mode) => (themeMode = mode)} />
  <Toolbar onaddcolumn={openCreateColumn} />
  <main class="content">
    {#if workspace.viewMode === 'board'}
      <Board onrename={openRenameColumn} onaddcolumn={openCreateColumn} />
    {:else}
      <TaskList />
    {/if}
  </main>

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

  .content {
    flex: 1;
    min-height: 0;
  }
</style>

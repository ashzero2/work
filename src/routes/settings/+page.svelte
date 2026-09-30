<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';

  import { getSetting, setSettingsPane } from '$lib/api';
  import AppearancePane from '$lib/components/settings/AppearancePane.svelte';
  import DataPane from '$lib/components/settings/DataPane.svelte';
  import FocusPane from '$lib/components/settings/FocusPane.svelte';
  import NotificationsPane from '$lib/components/settings/NotificationsPane.svelte';
  import ShortcutsPane from '$lib/components/settings/ShortcutsPane.svelte';
  import { pomodoro } from '$lib/stores/pomodoro.svelte';
  import { reminders } from '$lib/stores/reminders.svelte';
  import { theme } from '$lib/stores/theme.svelte';

  type Pane = 'appearance' | 'notifications' | 'focus' | 'shortcuts' | 'data';

  const panes: { id: Pane; label: string }[] = [
    { id: 'appearance', label: 'Appearance' },
    { id: 'notifications', label: 'Notifications' },
    { id: 'focus', label: 'Focus' },
    { id: 'shortcuts', label: 'Shortcuts' },
    { id: 'data', label: 'Data' }
  ];

  let pane = $state<Pane>('appearance');

  const labelFor = (id: Pane): string =>
    panes.find((entry) => entry.id === id)?.label ?? 'Settings';

  onMount(async () => {
    // This window is its own document: it applies the theme and loads its own
    // copy of the stores, exactly as the capture window does.
    await theme.load();

    const stored = await getSetting('settings.pane');
    const remembered = stored?.toLowerCase();
    if (remembered && panes.some((entry) => entry.id === remembered)) {
      pane = remembered as Pane;
    }
    await setSettingsPane(labelFor(pane));

    await Promise.all([pomodoro.load(), reminders.load()]);
  });

  // Both windows edit the same preferences, so whichever one changes something,
  // this one re-reads.
  $effect(() => {
    const pending = listen('settings:changed', () => {
      void theme.load();
      void pomodoro.load();
      void reminders.load();
    });
    return () => {
      void pending.then((off) => off());
    };
  });

  $effect(() => {
    const pending = listen<string>('settings:navigate', ({ payload }) => {
      if (panes.some(entry => entry.id === payload)) void choose(payload as Pane);
    });
    return () => { void pending.then(off => off()); };
  });

  async function choose(next: Pane): Promise<void> {
    pane = next;
    await setSettingsPane(labelFor(next));
  }
</script>

<div class="settings">
  <header><h1>Settings</h1><p>Preferences for this local workspace</p></header>
  <div class="settings-layout">
  <nav class="panes" aria-label="Settings panes">
    {#each panes as entry (entry.id)}
      <button
        class="pane-button"
        class:active={pane === entry.id}
        aria-current={pane === entry.id}
        onclick={() => void choose(entry.id)}
      >
        {entry.label}
      </button>
    {/each}
  </nav>

  <div class="pane">
    {#if pane === 'appearance'}
      <AppearancePane />
    {:else if pane === 'notifications'}
      <NotificationsPane />
    {:else if pane === 'focus'}
      <FocusPane />
    {:else if pane === 'shortcuts'}
      <ShortcutsPane />
    {:else}
      <DataPane />
    {/if}
  </div>
  </div>
</div>

<style>
  header { margin: 22px 24px 18px; padding-bottom: 14px; border-bottom: 1px solid var(--border); }
  h1 { margin: 0; font-size: 21px; }
  header p { margin: 4px 0 0; color: var(--muted-fg); font-size: 13px; }
  .settings-layout { display: grid; grid-template-columns: 150px minmax(0, 1fr); gap: 22px; padding: 0 24px; flex: 1; min-height: 0; }
  .settings {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--bg);
    color: var(--fg);
  }

  /* A stable toolbar that always shows which pane is active, per the HIG. */
  .panes {
    display: flex;
    flex-shrink: 0;
    flex-direction: column;
    gap: 4px;
    padding-right: 14px;
    border-right: 1px solid var(--border);
  }

  .pane-button {
    min-height: 36px;
    text-align: left;
    padding: 0 12px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--muted-fg);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
  }

  .pane-button:hover {
    background: var(--hover);
    color: var(--fg);
  }

  .pane-button.active {
    background: var(--accent);
    color: var(--accent-fg);
  }

  .pane {
    flex: 1;
    min-height: 0;
    padding: 0 0 24px;
    overflow-y: auto;
  }
</style>

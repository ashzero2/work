<script lang="ts">
  import { Check, Download, Settings as SettingsIcon } from '@lucide/svelte';

  import { exportTasks } from '$lib/api';
  import { theme } from '$lib/stores/theme.svelte';
  import { reminders } from '$lib/stores/reminders.svelte';
  import { toast } from '$lib/stores/toast.svelte';
  import { availableModes, themeModeLabel } from '$lib/theme';
  import type { ExportFormat } from '$lib/types';
  import DropdownMenu from './DropdownMenu.svelte';
  import NotificationKindToggles from './NotificationKindToggles.svelte';
  import PomodoroDurations from './PomodoroDurations.svelte';
  import ShortcutRecorder from './ShortcutRecorder.svelte';

  const modes = $derived(availableModes(theme.state.macos));
  let exporting = $state<ExportFormat | null>(null);

  async function exportAs(format: ExportFormat): Promise<void> {
    exporting = format;
    try {
      const path = await exportTasks(format);
      // A cancelled dialog is not a failure, and says nothing.
      if (path !== null) toast.show(`Exported to ${path}`);
    } catch (error) {
      toast.show(String(error));
    } finally {
      exporting = null;
    }
  }
</script>

<section class="view">
  <header class="view-head" data-tauri-drag-region>
    <div class="titles">
      <h1 class="title">Settings</h1>
      <span class="summary">Appearance, notifications, focus, shortcuts and your data</span>
    </div>
  </header>

  <div class="body">
    <section class="group">
      <h2>Appearance</h2>
      <div class="row">
        <span class="row-main">
          <span class="row-label">Theme</span>
          <span class="row-hint">System mirrors macOS, including its accent colour.</span>
        </span>
        <DropdownMenu align="end">
          {#snippet trigger({ toggle })}
            <button class="btn" onclick={toggle}>{themeModeLabel[theme.state.mode]}</button>
          {/snippet}
          {#snippet content({ close })}
            {#each modes as mode (mode)}
              <button
                class="menu-item"
                onclick={() => {
                  void theme.set(mode);
                  close();
                }}
              >
                <span class="mode-icon">
                  {#if mode === theme.state.mode}<Check size={14} />{/if}
                </span>
                {themeModeLabel[mode]}
              </button>
            {/each}
          {/snippet}
        </DropdownMenu>
      </div>
    </section>

    <section class="group">
      <h2>Notifications</h2>

      {#if !reminders.available}
        <p class="note">
          Reminders are delivered by macOS, which needs the packaged app rather than a dev server.
          Nothing can be shown here until then.
        </p>
      {:else if reminders.authorized === false}
        <div class="note-row">
          <p class="note">
            macOS is not currently allowing notifications from Work Dashboard, so reminders are
            saved but never shown.
          </p>
          <button class="btn" onclick={() => void reminders.openSettings()}>
            <SettingsIcon size={14} /> Open settings
          </button>
        </div>
      {/if}

      <NotificationKindToggles />
    </section>

    <section class="group">
      <h2>Focus</h2>
      <div class="row">
        <span class="row-main">
          <span class="row-label">Durations, in minutes</span>
          <span class="row-hint">The same values the Focus screen shows.</span>
        </span>
      </div>
      <PomodoroDurations />
    </section>

    <section class="group">
      <h2>Shortcuts</h2>
      <div class="row">
        <span class="row-main">
          <span class="row-label">Quick capture</span>
          <span class="row-hint">Opens the capture field from anywhere, even unfocused.</span>
        </span>
        <ShortcutRecorder />
      </div>
    </section>

    <section class="group">
      <h2>Data</h2>
      <div class="row">
        <span class="row-main">
          <span class="row-label">Export tasks</span>
          <span class="row-hint">Every task with its column, due date and history.</span>
        </span>
        <div class="actions">
          <button class="btn" disabled={exporting !== null} onclick={() => void exportAs('json')}>
            <Download size={14} /> JSON
          </button>
          <button class="btn" disabled={exporting !== null} onclick={() => void exportAs('csv')}>
            <Download size={14} /> CSV
          </button>
        </div>
      </div>
    </section>
  </div>
</section>

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
    min-height: 56px;
    padding: 8px 20px;
    border-bottom: 1px solid var(--border);
  }

  .titles {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .title {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
  }

  .summary {
    color: var(--muted-fg);
    font-size: 11px;
  }

  .body {
    display: flex;
    flex: 1;
    min-height: 0;
    flex-direction: column;
    gap: 28px;
    width: 100%;
    max-width: 640px;
    margin: 0 auto;
    padding: 28px 20px 40px;
    overflow-y: auto;
  }

  .group {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  h2 {
    margin: 0;
    color: var(--muted-fg);
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.03em;
    text-transform: uppercase;
  }

  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .row-main {
    display: flex;
    min-width: 0;
    flex-direction: column;
    gap: 2px;
  }

  .row-label {
    font-size: 13px;
  }

  .row-hint {
    color: var(--muted-fg);
    font-size: 11px;
  }

  .actions {
    display: flex;
    flex-shrink: 0;
    gap: 8px;
  }

  .note-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 10px 12px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--lane);
  }

  .note {
    margin: 0;
    color: var(--muted-fg);
    font-size: 12px;
    line-height: 1.45;
  }

  .mode-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 14px;
    color: var(--accent);
  }
</style>

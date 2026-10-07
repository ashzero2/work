<script lang="ts">
  import {
    Bell,
    Command,
    ListTodo,
    PanelLeft,
    Settings2,
    StickyNote,
    Tags,
    Timer,
    Trash2
  } from '@lucide/svelte';

  import { setTrafficLightsVisible, showSettingsWindow } from '$lib/api';
  import { navigation } from '$lib/stores/navigation.svelte';
  import { notes } from '$lib/stores/notes.svelte';
  import { theme } from '$lib/stores/theme.svelte';
  import FocusIndicator from './FocusIndicator.svelte';

  let { oncommand }: { oncommand: () => void } = $props();

  const collapsed = $derived(navigation.sidebarCollapsed);
  const chromeless = $derived(theme.state.macos);
  const navIcon = $derived(collapsed ? 18 : 15);

  $effect(() => {
    if (!theme.state.macos) return;
    // Collapsed, the lights have nothing to sit beside — and the rail is too
    // narrow to hold both them and the toggle — so they are hidden until the
    // sidebar is expanded again.
    void setTrafficLightsVisible(!collapsed);
  });
</script>

<aside class="sidebar" class:collapsed class:chromeless>
  <div class="control-row" data-tauri-drag-region={chromeless ? '' : undefined}>
    <button
      class="icon-btn toggle"
      aria-label={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
      title={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
      onclick={() => navigation.toggleSidebar()}
    >
      <PanelLeft size={18} />
    </button>
  </div>

  <div class="brand-row">
    <span class="brand-mark" aria-hidden="true">
      <svg viewBox="0 0 44 44" focusable="false">
        <circle cx="22" cy="10.5" r="5.4" />
        <rect x="18.8" y="22" width="6.4" height="17" rx="2.6" />
      </svg>
    </span>
    {#if !collapsed}
      <span class="brand-copy">
        <strong>tittle</strong>
      </span>
    {/if}
  </div>

  <nav class="sections">
    {#if !collapsed}<div class="section-label">Workspace</div>{/if}
    <button
      class="nav-item"
      class:active={navigation.section === 'tasks'}
      aria-current={navigation.section === 'tasks'}
      aria-label="Tasks"
      title={collapsed ? 'Tasks' : undefined}
      onclick={() => navigation.setSection('tasks')}
    >
      <ListTodo size={navIcon} />
      {#if !collapsed}<span>Tasks</span>{/if}
    </button>
    <button
      class="nav-item"
      class:active={navigation.section === 'notes'}
      aria-current={navigation.section === 'notes'}
      aria-label="Notes"
      title={collapsed ? 'Notes' : undefined}
      onclick={() => navigation.setSection('notes')}
    >
      <StickyNote size={navIcon} />
      {#if !collapsed}<span>Notes</span>{/if}
    </button>
    <button
      class="nav-item"
      class:active={navigation.section === 'focus'}
      aria-current={navigation.section === 'focus'}
      aria-label="Focus"
      title={collapsed ? 'Focus' : undefined}
      onclick={() => navigation.setSection('focus')}
    >
      <Timer size={navIcon} />
      {#if !collapsed}<span>Focus</span>{/if}
    </button>
    <button
      class="nav-item"
      class:active={navigation.section === 'reminders'}
      aria-current={navigation.section === 'reminders'}
      aria-label="Reminders"
      title={collapsed ? 'Reminders' : undefined}
      onclick={() => navigation.setSection('reminders')}
    >
      <Bell size={navIcon} />
      {#if !collapsed}<span>Reminders</span>{/if}
    </button>
  </nav>

  {#if navigation.section === 'notes' && !collapsed}
    <div class="tags">
      <div class="tags-head"><Tags size={13} /> Tags</div>
      <div class="tag-row">
        <button
          class="tag"
          class:active={notes.activeTag === null}
          onclick={() => notes.setActiveTag(null)}
        >
          <span class="tag-name">All notes</span>
          <span class="count">{notes.notes.length}</span>
        </button>
      </div>
      {#each notes.tags as tag (tag.id)}
        <div class="tag-row">
          <button
            class="tag"
            class:active={notes.activeTag === tag.name}
            onclick={() => notes.setActiveTag(tag.name)}
          >
            <span class="tag-name">{tag.name}</span>
            <span class="count">{tag.noteCount}</span>
          </button>
          <button
            class="icon-btn tiny"
            aria-label={`Delete tag ${tag.name}`}
            title="Delete tag"
            onclick={() => void notes.deleteTag(tag.id)}
          >
            <Trash2 size={12} />
          </button>
        </div>
      {/each}
      {#if notes.tags.length === 0}
        <p class="hint">Tags appear here once a note uses one.</p>
      {/if}
    </div>
  {/if}

  <div class="foot">
    {#if !collapsed}<div class="section-label">Preferences</div>{/if}
    {#if !collapsed}
      <FocusIndicator />
    {/if}
    <button
      class="utility"
      class:collapsed={collapsed}
      aria-label="Settings"
      title="Settings"
      onclick={() => void showSettingsWindow()}
    >
      <Settings2 size={collapsed ? 17 : 15} />
      {#if !collapsed}<span>Settings</span>{/if}
    </button>
    <button class="utility" class:collapsed aria-label="Command menu" onclick={oncommand}><Command size={navIcon} />{#if !collapsed}<span>Command menu</span>{/if}</button>
  </div>
</aside>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    gap: 18px;
    width: var(--sidebar-width);
    padding: 12px 12px 14px;
    border-right: 1px solid var(--border);
    background: color-mix(in srgb, var(--sidebar-bg) 90%, var(--bg));
  }

  .sidebar.collapsed {
    width: 52px;
    gap: 14px;
    padding: 6px 8px 12px;
    align-items: center;
  }

  /* A little top inset so the toggle's hover fill never touches the window
     edge, while staying close enough to read as the same row as the lights. */
  .sidebar.chromeless {
    padding-top: 6px;
  }

  .control-row {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    justify-content: flex-end;
    gap: 6px;
    height: 32px;
  }

  .sidebar.collapsed .control-row {
    justify-content: center;
    width: 32px;
  }

  .toggle {
    width: 32px;
    height: 32px;
  }

  .brand-row {
    display: flex;
    align-items: center;
    gap: 10px;
    min-height: 34px;
    padding: 0 8px;
  }

  .sidebar.collapsed .brand-row {
    justify-content: center;
    padding: 0;
  }

  .brand-mark {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    flex: none;
    color: var(--accent);
  }

  .brand-mark svg {
    width: 100%;
    height: 100%;
    fill: currentColor;
    overflow: visible;
  }

  .brand-copy {
    display: flex;
    min-width: 0;
    flex-direction: column;
    gap: 0;
  }

  .brand-copy strong {
    font-size: var(--text-md);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-tight);
  }

  .section-label {
    padding: 0 10px 5px;
    color: var(--muted-fg);
    font-size: var(--text-2xs);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-label);
    text-transform: uppercase;
  }

  .sections {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 9px;
    min-height: 38px;
    padding: 8px 10px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--muted-fg);
    font-size: var(--text-md);
    font-weight: var(--weight-medium);
    text-align: left;
    cursor: pointer;
  }

  .sidebar.collapsed .nav-item {
    justify-content: center;
    width: 36px;
    height: 34px;
    padding: 0;
  }

  .nav-item:hover {
    background: var(--hover);
    color: var(--fg);
  }

  .nav-item.active {
    background: color-mix(in srgb, var(--accent) 9%, transparent);
    color: var(--fg);
    box-shadow: inset 2px 0 0 var(--accent);
  }

  .tags {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
    min-height: 0;
    overflow-y: auto;
  }

  .tags-head {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 9px 6px;
    color: var(--muted-fg);
    font-size: var(--text-2xs);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-label);
    text-transform: uppercase;
  }

  .tag-row {
    display: flex;
    align-items: center;
  }

  .tag {
    display: flex;
    flex: 1;
    min-width: 0;
    align-items: center;
    gap: 8px;
    padding: 6px 9px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--muted-fg);
    font-size: var(--text-base);
    text-align: left;
    cursor: pointer;
  }

  .tag:hover {
    background: var(--hover);
    color: var(--fg);
  }

  .tag.active {
    background: var(--selection-bg);
    color: var(--selection-fg);
    box-shadow: var(--shadow-sm);
  }

  .tag-name {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .count {
    flex-shrink: 0;
    color: var(--muted-fg);
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    font-variant-numeric: tabular-nums;
  }

  .hint {
    margin: 2px 9px;
    color: var(--muted-fg);
    font-size: var(--text-xs);
    line-height: 1.45;
  }

  .foot {
    display: flex;
    align-items: center;
    flex-direction: column;
    gap: 8px;
    justify-content: flex-start;
    margin-top: auto;
    padding: 0 2px;
  }

  .sidebar.collapsed .foot {
    justify-content: center;
  }

  .utility {
    display: flex;
    align-items: center;
    gap: 9px;
    width: 100%;
    min-height: 34px;
    padding: 0 9px;
    border: 0;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--muted-fg);
    font-size: var(--text-base);
    font-weight: var(--weight-medium);
    cursor: pointer;
  }

  .utility:hover {
    background: var(--hover);
    color: var(--fg);
  }

  .utility.collapsed {
    justify-content: center;
    width: 36px;
    padding: 0;
  }
</style>

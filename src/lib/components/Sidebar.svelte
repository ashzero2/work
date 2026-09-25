<script lang="ts">
  import {
    Check,
    ListTodo,
    Monitor,
    Moon,
    PanelLeft,
    StickyNote,
    Sun,
    Tags,
    Trash2
  } from '@lucide/svelte';

  import { setTrafficLightsVisible } from '$lib/api';
  import { navigation } from '$lib/stores/navigation.svelte';
  import { notes } from '$lib/stores/notes.svelte';
  import { availableModes, themeModeLabel, type ThemeMode, type ThemeState } from '$lib/theme';
  import DropdownMenu from './DropdownMenu.svelte';

  interface Props {
    theme: ThemeState;
    ontheme: (mode: ThemeMode) => void;
  }

  let { theme, ontheme }: Props = $props();

  const collapsed = $derived(navigation.sidebarCollapsed);
  const chromeless = $derived(theme.macos);
  const modes = $derived(availableModes(theme.macos));
  const navIcon = $derived(collapsed ? 18 : 15);

  $effect(() => {
    if (!theme.macos) return;
    // Collapsed, the lights have nothing to sit beside — and the rail is too
    // narrow to hold both them and the toggle — so they are hidden until the
    // sidebar is expanded again.
    void setTrafficLightsVisible(!collapsed);
  });
</script>

{#snippet modeIcon(mode: ThemeMode)}
  {#if mode === 'dark'}
    <Moon size={16} />
  {:else if mode === 'macos'}
    <Monitor size={16} />
  {:else}
    <Sun size={16} />
  {/if}
{/snippet}

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

  <nav class="sections">
    <button
      class="nav-item"
      class:active={navigation.section === 'tasks'}
      aria-current={navigation.section === 'tasks'}
      onclick={() => navigation.setSection('tasks')}
    >
      <ListTodo size={navIcon} />
      {#if !collapsed}<span>Tasks</span>{/if}
    </button>
    <button
      class="nav-item"
      class:active={navigation.section === 'notes'}
      aria-current={navigation.section === 'notes'}
      onclick={() => navigation.setSection('notes')}
    >
      <StickyNote size={navIcon} />
      {#if !collapsed}<span>Notes</span>{/if}
    </button>
  </nav>

  {#if navigation.section === 'notes' && !collapsed}
    <div class="tags">
      <div class="tags-head"><Tags size={13} /> Tags</div>
      <button
        class="tag"
        class:active={notes.activeTag === null}
        onclick={() => notes.setActiveTag(null)}
      >
        <span class="tag-name">All notes</span>
        <span class="count">{notes.notes.length}</span>
      </button>
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
    <DropdownMenu align="start" placement="above">
      {#snippet trigger({ toggle })}
        <button
          class="icon-btn"
          aria-label={`Theme: ${themeModeLabel[theme.mode]}`}
          title={`Theme: ${themeModeLabel[theme.mode]}`}
          onclick={toggle}
        >
          {@render modeIcon(theme.mode)}
        </button>
      {/snippet}
      {#snippet content({ close })}
        {#each modes as mode (mode)}
          <button
            class="menu-item"
            onclick={() => {
              ontheme(mode);
              close();
            }}
          >
            <span class="mode-icon">
              {#if mode === theme.mode}<Check size={14} />{/if}
            </span>
            {themeModeLabel[mode]}
          </button>
        {/each}
      {/snippet}
    </DropdownMenu>
  </div>
</aside>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    gap: 16px;
    width: 220px;
    padding: 12px 10px;
    border-right: 1px solid var(--border);
    background: var(--sidebar-bg);
  }

  .sidebar.collapsed {
    width: 52px;
    gap: 10px;
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
    width: 30px;
    height: 30px;
  }

  .sections {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 9px;
    padding: 7px 9px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--muted-fg);
    font-size: 13px;
    font-weight: 500;
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
    background: var(--selection-bg);
    color: var(--selection-fg);
    box-shadow: var(--shadow-sm);
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
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.03em;
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
    font-size: 12px;
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
    font-size: 11px;
  }

  .hint {
    margin: 2px 9px;
    color: var(--muted-fg);
    font-size: 11px;
    line-height: 1.45;
  }

  .foot {
    display: flex;
    justify-content: flex-start;
    margin-top: auto;
    padding: 0 2px;
  }

  .sidebar.collapsed .foot {
    justify-content: center;
  }

  .mode-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 14px;
    color: var(--accent);
  }
</style>

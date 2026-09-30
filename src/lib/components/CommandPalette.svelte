<script lang="ts">
  import { onMount } from 'svelte';

  import { showSettingsWindow, showCaptureWindow } from '$lib/api';
  import { bestScore } from '$lib/fuzzy';
  import { navigation } from '$lib/stores/navigation.svelte';
  import { notes } from '$lib/stores/notes.svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Section } from '$lib/types';

  interface Props {
    onclose: () => void;
  }

  let { onclose }: Props = $props();

  interface Entry {
    id: string;
    kind: string;
    label: string;
    detail: string;
    run: () => void;
  }

  const SECTIONS: { id: Section; label: string }[] = [
    { id: 'tasks', label: 'Tasks' },
    { id: 'notes', label: 'Notes' },
    { id: 'focus', label: 'Focus' },
    { id: 'reminders', label: 'Reminders' }
  ];

  const LIMIT = 8;

  let query = $state('');
  let active = $state(0);
  let input: HTMLInputElement | null = null;
  let dialog: HTMLDivElement | null = null;
  let list: HTMLUListElement | null = null;

  /// Everything is already in the stores, so the palette is a filter over memory
  /// rather than a query — which is why it can re-rank on every keystroke.
  const entries = $derived.by(() => {
    const all: Entry[] = [
      ...SECTIONS.map((section) => ({
        id: `section-${section.id}`,
        kind: 'Go to',
        label: section.label,
        detail: '',
        run: () => navigation.setSection(section.id)
      })),
      {
        id: 'create-task',
        kind: 'Create',
        label: 'New task',
        detail: 'Quick capture',
        run: () => void showCaptureWindow()
      },
      {
        id: 'create-note',
        kind: 'Create',
        label: 'New note',
        detail: '',
        run: () => { navigation.setSection('notes'); void notes.createNote(); }
      },
      {
        id: 'settings',
        kind: 'Open',
        label: 'Settings…',
        detail: 'Appearance, notifications, focus, shortcuts',
        run: () => void showSettingsWindow()
      },
      ...notes.tags.map((tag) => ({
        id: `tag-${tag.id}`,
        kind: 'Tag',
        label: tag.name,
        detail: `${tag.noteCount} note${tag.noteCount === 1 ? '' : 's'}`,
        run: () => {
          navigation.setSection('notes');
          notes.setActiveTag(tag.name);
        }
      })),
      ...workspace.tasks.map((task) => {
        const column = workspace.columns.find((entry) => entry.id === task.columnId);
        return {
          id: `task-${task.id}`,
          kind: task.completedAt === null ? 'Task' : 'Done',
          label: task.title,
          detail: column?.name ?? '',
          run: () => revealTask(task.id)
        };
      }),
      ...notes.notes.map((note) => ({
        id: `note-${note.id}`,
        kind: 'Note',
        label: note.title,
        detail: note.tags.join(', '),
        run: () => {
          navigation.setSection('notes');
          notes.open(note.id);
        }
      }))
    ];

    const trimmed = query.trim();
    if (trimmed.length === 0) return all.slice(0, LIMIT);

    return all
      .map((entry) => ({
        entry,
        rank: bestScore(trimmed, entry.detail.length > 0 ? [entry.label, entry.detail] : [entry.label])
      }))
      .filter((candidate): candidate is { entry: Entry; rank: number } => candidate.rank !== null)
      .sort((a, b) => b.rank - a.rank)
      .slice(0, LIMIT)
      .map((candidate) => candidate.entry);
  });

  onMount(() => input?.focus());

  $effect(() => {
    // A new query is a new list, so the selection starts at the top again.
    void query;
    active = 0;
  });

  $effect(() => {
    void active;
    list?.querySelector('[aria-selected="true"]')?.scrollIntoView({ block: 'nearest' });
  });

  /// Switching section first, then scrolling: the row only exists once the view
  /// it lives in has rendered.
  function revealTask(id: number): void {
    navigation.setSection('tasks');
    void Promise.resolve().then(() => {
      const reduce = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
      document
        .querySelector(`[data-task-id="${id}"]`)
        ?.scrollIntoView({ block: 'center', behavior: reduce ? 'auto' : 'smooth' });
    });
  }

  function move(delta: number): void {
    if (entries.length === 0) return;
    active = (active + delta + entries.length) % entries.length;
  }

  function choose(entry: Entry): void {
    onclose();
    entry.run();
  }

  function onKeydown(event: KeyboardEvent): void {
    if (event.key === 'Escape') {
      event.preventDefault();
      onclose();
    } else if (event.key === 'ArrowDown') {
      event.preventDefault();
      move(1);
    } else if (event.key === 'ArrowUp') {
      event.preventDefault();
      move(-1);
    } else if (event.key === 'Enter') {
      event.preventDefault();
      const entry = entries[active];
      if (entry !== undefined) choose(entry);
    }
  }

  function onPointerDown(event: PointerEvent): void {
    if (dialog && !dialog.contains(event.target as Node)) onclose();
  }
</script>

<svelte:window onpointerdown={onPointerDown} />

<div class="backdrop">
  <div class="palette" bind:this={dialog} role="dialog" aria-modal="true" aria-label="Quick find">
    <input
      bind:this={input}
      bind:value={query}
      onkeydown={onKeydown}
      role="combobox"
      aria-expanded="true"
      aria-controls="palette-list"
      aria-activedescendant={entries[active]?.id ?? undefined}
      aria-label="Find a task, note, tag or section"
      placeholder="Find a task, note, tag or section…"
      spellcheck="false"
    />

    <ul id="palette-list" bind:this={list} role="listbox" aria-label="Results">
      {#each entries as entry, index (entry.id)}
        <!-- svelte-ignore a11y_click_events_have_key_events -- the input owns focus
             and all keyboard navigation, by aria-activedescendant; this is only the
             pointer path into the same listbox. -->
        <li
          id={entry.id}
          role="option"
          aria-selected={index === active}
          class:active={index === active}
          onclick={() => choose(entry)}
          onpointermove={() => (active = index)}
        >
          <span class="kind">{entry.kind}</span>
          <span class="label">{entry.label}</span>
          {#if entry.detail.length > 0}<span class="detail">{entry.detail}</span>{/if}
        </li>
      {/each}

      {#if entries.length === 0}
        <li class="empty" role="presentation">Nothing matches “{query}”.</li>
      {/if}
    </ul>

    <p class="sr-only" role="status">{entries.length} results</p>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 70;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 14vh;
    background: color-mix(in srgb, #000 30%, transparent);
  }

  .palette {
    display: flex;
    flex-direction: column;
    width: 560px;
    max-width: calc(100% - 48px);
    max-height: 60vh;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--panel);
    box-shadow: var(--shadow-md);
    overflow: hidden;
  }

  input {
    height: 44px;
    padding: 0 14px;
    border: none;
    border-bottom: 1px solid var(--border);
    background: transparent;
    color: var(--fg);
    font-size: 15px;
    outline: none;
  }

  ul {
    margin: 0;
    padding: 6px;
    list-style: none;
    overflow-y: auto;
  }

  li {
    display: flex;
    align-items: baseline;
    gap: 10px;
    padding: 7px 9px;
    border-radius: var(--radius-sm);
    font-size: 13px;
    cursor: pointer;
  }

  li.active {
    background: var(--selection-bg);
    box-shadow: var(--shadow-sm);
  }

  .kind {
    flex-shrink: 0;
    width: 52px;
    color: var(--muted-fg);
    font-size: 11px;
  }

  .label {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .detail {
    flex-shrink: 0;
    max-width: 200px;
    overflow: hidden;
    color: var(--muted-fg);
    font-size: 11px;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .empty {
    color: var(--muted-fg);
    cursor: default;
  }
</style>

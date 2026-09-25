<script lang="ts">
  import { Trash2 } from '@lucide/svelte';

  import type { NoteCard } from '$lib/types';

  interface Props {
    note: NoteCard;
    ondragstart: (event: PointerEvent, id: number) => void;
    onopen: (id: number) => void;
    ondelete: (id: number) => void;
  }

  let { note, ondragstart, onopen, ondelete }: Props = $props();

  const updated = $derived(
    new Date(note.updatedAt).toLocaleDateString(undefined, { month: 'short', day: 'numeric' })
  );
</script>

<div
  class="sticky"
  role="button"
  tabindex="0"
  aria-label={note.title}
  style="left: {note.posX ?? 0}px; top: {note.posY ?? 0}px; background: {note.color}; transform: rotate({note.rotationDeg}deg);"
  onpointerdown={(event) => ondragstart(event, note.id)}
  onclick={() => onopen(note.id)}
  onkeydown={(event) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      onopen(note.id);
    }
  }}
>
  <header>
    <span class="title">{note.title}</span>
    <button
      class="icon-btn"
      aria-label="Delete note"
      title="Delete note"
      onpointerdown={(event) => event.stopPropagation()}
      onclick={(event) => {
        event.stopPropagation();
        ondelete(note.id);
      }}
    >
      <Trash2 size={13} />
    </button>
  </header>

  {#if note.tags.length > 0}
    <div class="tags">
      {#each note.tags as tag (tag)}
        <span class="tag">{tag}</span>
      {/each}
    </div>
  {/if}

  <span class="date">{updated}</span>
</div>

<style>
  .sticky {
    position: absolute;
    display: flex;
    flex-direction: column;
    gap: 8px;
    width: 224px;
    min-height: 140px;
    padding: 12px;
    border-radius: 6px;
    box-shadow:
      inset 0 1px 0 rgba(255, 255, 255, 0.5),
      0 6px 16px rgba(38, 32, 24, 0.18);
    color: #2a2418;
    cursor: grab;
    touch-action: none;
    user-select: none;
  }

  .sticky:focus-visible {
    outline: 2px solid #2a2418;
    outline-offset: 2px;
  }

  header {
    display: flex;
    align-items: flex-start;
    gap: 6px;
  }

  .title {
    flex: 1;
    min-width: 0;
    font-size: 13px;
    font-weight: 600;
    line-height: 1.35;
    overflow-wrap: anywhere;
  }

  header :global(.icon-btn) {
    color: #4a4130;
  }

  header :global(.icon-btn:hover) {
    background: rgba(42, 36, 24, 0.12);
    color: #2a2418;
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
  }

  .tag {
    padding: 2px 6px;
    border-radius: 999px;
    background: rgba(42, 36, 24, 0.1);
    color: #4a4130;
    font-size: 11px;
    font-weight: 500;
  }

  .date {
    margin-top: auto;
    color: #5a5040;
    font-size: 11px;
  }
</style>

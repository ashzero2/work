<script lang="ts">
  import { Plus } from '@lucide/svelte';

  import { notes } from '$lib/stores/notes.svelte';
  import EmptyState from './EmptyState.svelte';
  import LoadingState from './LoadingState.svelte';
  import StickyNote from './StickyNote.svelte';

  const NOTE_WIDTH = 224;
  const NOTE_HEIGHT = 168;
  const DRAG_THRESHOLD = 4;

  let board = $state<HTMLDivElement | null>(null);
  let draggingId: number | null = null;
  let grabOffset = { x: 0, y: 0 };
  let startPoint = { x: 0, y: 0 };
  let moved = false;

  const width = $derived(
    Math.max(1400, ...notes.visible.map((note) => (note.posX ?? 0) + NOTE_WIDTH + 48))
  );
  const height = $derived(
    Math.max(900, ...notes.visible.map((note) => (note.posY ?? 0) + NOTE_HEIGHT + 48))
  );

  function startDrag(event: PointerEvent, id: number): void {
    if (event.button !== 0 || !board) return;
    const note = notes.notes.find((candidate) => candidate.id === id);
    if (!note) return;

    const rect = board.getBoundingClientRect();
    draggingId = id;
    moved = false;
    startPoint = { x: event.clientX, y: event.clientY };
    grabOffset = {
      x: event.clientX - rect.left - (note.posX ?? 0),
      y: event.clientY - rect.top - (note.posY ?? 0)
    };
  }

  function onPointerMove(event: PointerEvent): void {
    if (draggingId === null || !board) return;

    if (
      !moved &&
      Math.hypot(event.clientX - startPoint.x, event.clientY - startPoint.y) > DRAG_THRESHOLD
    ) {
      moved = true;
    }
    if (!moved) return;

    const rect = board.getBoundingClientRect();
    notes.dragTo(
      draggingId,
      Math.max(0, event.clientX - rect.left - grabOffset.x),
      Math.max(0, event.clientY - rect.top - grabOffset.y)
    );
  }

  function endDrag(): void {
    if (draggingId === null) return;
    const id = draggingId;
    draggingId = null;

    const note = notes.notes.find((candidate) => candidate.id === id);
    if (note && moved) void notes.moveNote(id, note.posX ?? 0, note.posY ?? 0);
  }

  /// A drag ends with a click too — only treat it as "open" if nothing moved.
  function open(id: number): void {
    if (moved) return;
    notes.open(id);
  }
</script>

<svelte:window onpointermove={onPointerMove} onpointerup={endDrag} onpointercancel={endDrag} />

{#if notes.loading}
  <LoadingState label="Loading notes…" />
{:else if notes.visible.length === 0}
  <div class="center">
    <EmptyState
      variant="notes"
      title={notes.isSearching || notes.activeTag ? 'No matching notes' : 'No notes yet'}
      description={notes.isSearching || notes.activeTag
        ? 'Try a different search or clear the tag filter.'
        : 'Jot something down and pin it to the board.'}
    >
      {#snippet action()}
        <button class="btn btn-primary" onclick={() => void notes.createNote()}>
          <Plus size={15} /> New note
        </button>
      {/snippet}
    </EmptyState>
  </div>
{:else}
  <div class="scroll">
    <div class="canvas" bind:this={board} style="width: {width}px; height: {height}px;">
      {#each notes.visible as note (note.id)}
        <StickyNote
          {note}
          ondragstart={startDrag}
          onopen={open}
          ondelete={(id) => void notes.deleteNote(id)}
        />
      {/each}
    </div>
  </div>
{/if}

<style>
  .scroll {
    height: 100%;
    overflow: auto;
    background: var(--lane);
  }

  .canvas {
    position: relative;
  }

  .center {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
  }
</style>

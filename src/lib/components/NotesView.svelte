<script lang="ts">
  import { FileText, LayoutGrid, Move, Plus, Search } from '@lucide/svelte';
  import { notes } from '$lib/stores/notes.svelte';
  import NoteInspector from './NoteInspector.svelte';
  import Corkboard from './Corkboard.svelte';
  import LoadingState from './LoadingState.svelte';
  let selectedId = $state<number | null>(null);
  let spatial = $state(false);
  const selected = $derived(notes.visible.find(note => note.id === selectedId) ?? null);

  async function createNote(): Promise<void> {
    const id = await notes.createNote(false);
    if (id !== null) selectedId = id;
  }
</script>
<section class="workspace-view">
  <header class="workspace-head" data-tauri-drag-region>
    <div><h1>{notes.activeTag ? `#${notes.activeTag}` : 'Notes / Index'}</h1><p>{notes.visible.length} notes · {notes.activeTag ?? 'All notes'}</p></div>
    <div class="head-actions"><label class="search"><Search size={14} /><input aria-label="Search notes" value={notes.query} oninput={event => notes.setQuery(event.currentTarget.value)} placeholder="Search notes…" /></label><button class="btn btn-primary" onclick={() => void createNote()}><Plus size={15} /> New note</button></div>
  </header>
  <div class="workspace-body notes-body">
    <div class="view-tools"><button class="btn" onclick={() => spatial = !spatial}>{#if spatial}<LayoutGrid size={14} /> Card index{:else}<Move size={14} /> Spatial board{/if}</button></div>
    {#if spatial}<Corkboard />{:else if notes.loading}<LoadingState label="Loading notes…" />{:else}
      <div class="notes-layout">
        <nav class="note-list" aria-label="Notes">
        {#each notes.visible as note (note.id)}<button class="note" class:selected={selected?.id === note.id} aria-current={selected?.id === note.id ? 'true' : undefined} onclick={() => selectedId = note.id}>
          <h2>{note.title}</h2><p>{note.preview || 'No text yet'}</p>
          <footer><span class="tags">{#each note.tags as tag}<span>#{tag}</span>{/each}{#if !note.tags.length}<span>No tags</span>{/if}</span><time>{new Date(note.updatedAt).toLocaleDateString(undefined, { month: 'short', day: 'numeric' })}</time></footer>
        </button>{/each}
        {#if !notes.visible.length}<p class="empty">{notes.query || notes.activeTag ? 'No notes match this view.' : 'No notes yet. Create one to get started.'}</p>{/if}
        </nav>
        <div class="note-content">
          {#if selected}{#key selected.id}<NoteInspector note={selected} />{/key}
          {:else}<div class="note-prompt"><FileText size={22} /><p>Select a note to read or edit it.</p><button class="btn btn-primary" onclick={() => void createNote()}><Plus size={14} /> New note</button></div>{/if}
        </div>
      </div>
    {/if}
  </div>
</section>
<style>
  .notes-body { display: flex; flex-direction: column; overflow: hidden; }
  .view-tools { display: flex; flex-shrink: 0; justify-content: flex-end; margin-bottom: 14px; }
  .search { display: flex; align-items: center; gap: 6px; min-height: 32px; padding: 0 10px; border: 1px solid var(--field-border); border-radius: var(--radius-sm); background: var(--panel); color: var(--muted-fg); }
  .search input { width: 160px; min-width: 0; border: 0; background: transparent; color: var(--fg); font: inherit; font-size: var(--text-base); }
  .notes-layout { display: grid; flex: 1; grid-template-columns: minmax(220px, 280px) minmax(0, 1fr); gap: 16px; min-height: 0; }
  .note-list { display: flex; flex-direction: column; gap: 8px; min-height: 0; overflow: auto; padding-right: 2px; }
  .note { display: flex; flex-direction: column; flex: 0 0 auto; min-height: 112px; padding: 12px; border: 1px solid var(--border); border-radius: var(--radius); background: var(--panel); color: var(--fg); text-align: left; cursor: pointer; transition: border-color var(--motion-fast) var(--ease), background var(--motion-fast) var(--ease); }
  .note:hover { background: var(--hover); } .note.selected { border-color: var(--accent); background: color-mix(in srgb, var(--accent) 7%, var(--panel)); }
  h2 { margin: 0 0 5px; font-size: var(--text-base); line-height: var(--leading-snug); } .note p { margin: 0 0 9px; font-size: var(--text-sm); line-height: var(--leading-normal); color: var(--muted-fg); display: -webkit-box; -webkit-line-clamp: 2; line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
  footer { display: flex; justify-content: space-between; gap: 8px; border-top: 1px solid var(--divider); padding-top: 7px; margin-top: auto; font-size: var(--text-2xs); color: var(--muted-fg); } .tags { display: flex; flex-wrap: wrap; gap: 4px; min-width: 0; } time { flex-shrink: 0; font-variant-numeric: tabular-nums; } .empty { padding: 12px; color: var(--muted-fg); }
  .note-content { min-width: 0; min-height: 0; height: 100%; }
  .notes-body :global(.scroll) { flex: 1; min-height: 0; height: auto; }
  .note-prompt { display: flex; height: 100%; min-height: 320px; flex-direction: column; align-items: center; justify-content: center; gap: 12px; border: 1px dashed var(--border); border-radius: var(--radius-lg); color: var(--muted-fg); }
  .note-prompt p { margin: 0; font-size: var(--text-base); }
  @media (max-width: 760px) { .notes-layout { grid-template-columns: minmax(155px, 38%) minmax(0, 1fr); gap: 10px; } .note { padding: 9px; } }
</style>

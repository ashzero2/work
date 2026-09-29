<script lang="ts">
  import { Plus, Search } from '@lucide/svelte';

  import { notes } from '$lib/stores/notes.svelte';
  import Corkboard from './Corkboard.svelte';

  const heading = $derived(notes.activeTag ? `#${notes.activeTag}` : 'All notes');
  const summary = $derived(
    notes.visible.length === notes.notes.length
      ? `${notes.notes.length} ${notes.notes.length === 1 ? 'note' : 'notes'}`
      : `${notes.visible.length} of ${notes.notes.length} notes`
  );
</script>

<section class="view">
  <header class="view-head" data-tauri-drag-region>
    <div class="titles">
      <span class="title">{heading}</span>
      <span class="summary">{summary}</span>
    </div>
    <div class="controls">
      <div class="search">
        <Search size={14} />
        <input
          value={notes.query}
          oninput={(event) => notes.setQuery(event.currentTarget.value)}
          placeholder="Search notes…"
          aria-label="Search notes"
        />
      </div>
      <button class="btn btn-primary" onclick={() => void notes.createNote()}>
        <Plus size={15} /> New note
      </button>
    </div>
  </header>

  <div class="body">
    <Corkboard />
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
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    min-height: 68px;
    padding: 12px 28px;
    border-bottom: 1px solid var(--border);
    background: color-mix(in srgb, var(--bg) 92%, var(--panel));
  }

  .titles {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-width: 0;
  }

  .title {
    font-size: 20px;
    font-weight: 700;
    letter-spacing: -0.03em;
  }

  .summary {
    color: var(--muted-fg);
    font-size: 12px;
  }

  .controls {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    gap: 10px;
  }

  .search {
    display: flex;
    flex: 0 1 220px;
    min-width: 150px;
    align-items: center;
    gap: 6px;
    height: 36px;
    padding: 0 10px;
    border: 1px solid var(--field-border);
    border-radius: var(--radius-sm);
    background: var(--panel);
    color: var(--muted-fg);
  }

  .search:focus-within {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px var(--ring-soft);
  }

  .search input {
    flex: 1;
    min-width: 0;
    border: none;
    background: transparent;
    color: var(--fg);
    font-size: 13px;
    outline: none;
  }

  .body {
    flex: 1;
    min-height: 0;
  }
</style>

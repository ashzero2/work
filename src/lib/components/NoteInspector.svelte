<script lang="ts">
  import { Trash2 } from '@lucide/svelte';
  import { onMount, onDestroy } from 'svelte';
  import { noteDrafts } from '$lib/note-drafts';
  import { notes } from '$lib/stores/notes.svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import { toast } from '$lib/stores/toast.svelte';
  import type { NoteCard } from '$lib/types';
  import TagInput from './TagInput.svelte';
  let { note }: { note: NoteCard } = $props();
  let title = $state(''); let body = $state(''); let tags = $state<string[]>([]);
  let loaded = $state(false); let saving = $state(false); let dirty = $state(false);
  onMount(async () => {
    const draft = noteDrafts.get(note.id);
    title = draft?.title ?? note.title;
    tags = [...(draft?.tags ?? note.tags)];
    try {
      body = draft?.body ?? await notes.bodyFor(note.id);
      dirty = draft !== undefined;
      loaded = true;
    } catch (error) {
      toast.show(String(error));
    }
  });
  onDestroy(() => {
    if (dirty && loaded) noteDrafts.set(note.id, { title, body, tags: [...tags] });
  });
  async function save(): Promise<boolean> {
    if (!loaded || saving || !title.trim()) return false;
    saving = true;
    try {
      await notes.saveNote(note.id, title.trim(), body, tags);
      dirty = false;
      noteDrafts.delete(note.id);
      return true;
    } catch {
      return false;
    } finally {
      saving = false;
    }
  }

  function remove(): void {
    noteDrafts.delete(note.id);
    void notes.deleteNote(note.id);
  }
</script>
<aside class="inspector" aria-label="Note editor">
  <header>Note</header>
  <label class="sr-only" for="note-title">Note title</label><input id="note-title" aria-label="Note title" bind:value={title} oninput={() => dirty = true} />
  {#if loaded}<textarea aria-label="Note body" bind:value={body} oninput={() => dirty = true} placeholder="Write in Markdown…"></textarea>{:else}<p>Loading note…</p>{/if}
  <div class="meta">
    <TagInput {tags} onchange={(next) => { tags = next; dirty = true; }} />
    {#if note.linkedTaskId}<p>Linked task: {workspace.tasks.find(task => task.id === note.linkedTaskId)?.title ?? 'Task'}</p>{/if}
  </div>
  <footer><button class="btn danger" onclick={remove}><Trash2 size={14} /> Delete</button><span>{dirty ? 'Unsaved changes' : 'Saved locally'}</span><button class="btn btn-primary" onclick={() => void save()} disabled={!loaded || !dirty || !title.trim() || saving}>{saving ? 'Saving…' : 'Save'}</button></footer>
</aside>
<style>
  .inspector { display: flex; flex-direction: column; width: 100%; height: 100%; min-width: 0; min-height: 320px; border: 1px solid var(--border); border-radius: var(--radius-lg); background: var(--panel); overflow: hidden; }
  header, footer { display: flex; align-items: center; justify-content: space-between; gap: 8px; padding: 10px 14px; } header { border-bottom: 1px solid var(--border); color: var(--muted-fg); font-size: 12px; }
  input { padding: 18px 20px 12px; border: 0; background: transparent; color: var(--fg); font-size: 20px; font-weight: 650; width: 100%; }
  textarea { flex: 1; min-height: 220px; resize: none; padding: 8px 20px 20px; border: 0; background: transparent; color: var(--fg); font: 14px/1.65 var(--font); }
  .meta { padding: 12px 14px; border-top: 1px solid var(--border); } p, footer span { margin: 0; font-size: 12px; color: var(--muted-fg); } footer { border-top: 1px solid var(--border); }
  .danger { color: var(--danger); }
</style>

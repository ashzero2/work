<script lang="ts">
  import { Trash2 } from '@lucide/svelte';

  import { notes } from '$lib/stores/notes.svelte';
  import { toast } from '$lib/stores/toast.svelte';
  import type { NoteCard } from '$lib/types';
  import TagInput from './TagInput.svelte';

  interface Props {
    note: NoteCard;
    onclose: () => void;
  }

  let { note, onclose }: Props = $props();

  // svelte-ignore state_referenced_locally — the dialog is keyed per note, so these seed once
  let title = $state(note.title);
  let body = $state('');
  // svelte-ignore state_referenced_locally
  let tags = $state<string[]>([...note.tags]);
  let loading = $state(true);
  let saving = $state(false);

  let dialog: HTMLDivElement | null = null;
  let titleInput: HTMLInputElement | null = null;

  $effect(() => {
    let cancelled = false;
    void (async () => {
      try {
        const loaded = await notes.bodyFor(note.id);
        if (!cancelled) body = loaded;
      } catch (error) {
        if (!cancelled) toast.show(String(error));
      } finally {
        if (!cancelled) loading = false;
      }
    })();
    return () => {
      cancelled = true;
    };
  });

  $effect(() => {
    titleInput?.focus();
    titleInput?.select();
  });

  async function save(): Promise<void> {
    const trimmed = title.trim();
    if (!trimmed) return;
    saving = true;
    try {
      await notes.saveNote(note.id, trimmed, body, tags);
      onclose();
    } catch {
      // saveNote surfaces the error; keep the dialog open so nothing is lost
    } finally {
      saving = false;
    }
  }

  function onKeydown(event: KeyboardEvent): void {
    if (event.key === 'Escape') onclose();
    if (event.key === 'Enter' && (event.metaKey || event.ctrlKey)) void save();
  }

  function onPointerDown(event: PointerEvent): void {
    if (dialog && !dialog.contains(event.target as Node)) onclose();
  }
</script>

<svelte:window onpointerdown={onPointerDown} onkeydown={onKeydown} />

<div class="backdrop">
  <div class="dialog" bind:this={dialog} role="dialog" aria-modal="true" aria-label={note.title}>
    <label class="sr-only" for="note-title">Note title</label>
    <input
      id="note-title"
      class="title"
      bind:this={titleInput}
      bind:value={title}
      placeholder="Note title"
    />

    {#if loading}
      <div class="loading">Loading…</div>
    {:else}
      <textarea
        bind:value={body}
        placeholder="Write in markdown…"
        aria-label="Note body"
        spellcheck="false"
      ></textarea>
    {/if}

    <TagInput {tags} onchange={(next) => (tags = next)} />

    <footer>
      <button
        class="btn danger"
        onclick={() => {
          void notes.deleteNote(note.id);
          onclose();
        }}
      >
        <Trash2 size={14} /> Delete
      </button>
      <div class="spacer"></div>
      <button class="btn" onclick={onclose}>Cancel</button>
      <button class="btn btn-primary" onclick={() => void save()} disabled={saving || !title.trim()}>
        Save
      </button>
    </footer>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 60;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
    background: color-mix(in srgb, #000 32%, transparent);
  }

  .dialog {
    display: flex;
    flex-direction: column;
    gap: 12px;
    width: 560px;
    max-width: 100%;
    max-height: 100%;
    padding: 16px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--panel);
    box-shadow: var(--shadow-md);
  }

  .title {
    height: 36px;
    padding: 0 11px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg);
    color: var(--fg);
    font-size: 14px;
    font-weight: 600;
  }

  textarea {
    flex: 1;
    min-height: 240px;
    padding: 11px;
    resize: vertical;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg);
    color: var(--fg);
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 13px;
    line-height: 1.55;
  }

  .title:focus,
  textarea:focus {
    border-color: var(--accent);
  }

  .loading {
    min-height: 240px;
    padding: 11px;
    color: var(--muted-fg);
    font-size: 13px;
  }

  footer {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .spacer {
    flex: 1;
  }

  .btn.danger {
    color: var(--danger);
  }
</style>

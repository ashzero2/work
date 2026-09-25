<script lang="ts">
  import { X } from '@lucide/svelte';

  interface Props {
    tags: string[];
    onchange: (tags: string[]) => void;
  }

  let { tags, onchange }: Props = $props();

  let draft = $state('');

  function add(): void {
    const value = draft.trim();
    if (!value) return;
    if (!tags.some((tag) => tag.toLowerCase() === value.toLowerCase())) {
      onchange([...tags, value]);
    }
    draft = '';
  }

  function remove(tag: string): void {
    onchange(tags.filter((existing) => existing !== tag));
  }

  function onKeydown(event: KeyboardEvent): void {
    if (event.key === 'Enter' || event.key === ',') {
      event.preventDefault();
      add();
    } else if (event.key === 'Backspace' && !draft && tags.length > 0) {
      remove(tags[tags.length - 1]);
    }
  }
</script>

<div class="tag-input">
  {#each tags as tag (tag)}
    <span class="pill">
      {tag}
      <button class="icon-btn tiny" aria-label={`Remove tag ${tag}`} onclick={() => remove(tag)}>
        <X size={11} />
      </button>
    </span>
  {/each}
  <input
    bind:value={draft}
    onkeydown={onKeydown}
    onblur={add}
    placeholder="Add tag…"
    aria-label="Add tag"
  />
</div>

<style>
  .tag-input {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 6px;
    min-height: 36px;
    padding: 5px 8px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg);
  }

  .tag-input:focus-within {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px var(--ring-soft);
  }

  .pill {
    display: inline-flex;
    align-items: center;
    gap: 2px;
    padding: 3px 4px 3px 8px;
    border-radius: 999px;
    background: color-mix(in srgb, var(--accent) 14%, transparent);
    color: color-mix(in srgb, var(--accent) 82%, var(--fg));
    font-size: 11px;
  }

  input {
    flex: 1;
    min-width: 90px;
    border: none;
    background: transparent;
    color: var(--fg);
    font-size: 12px;
    outline: none;
  }
</style>

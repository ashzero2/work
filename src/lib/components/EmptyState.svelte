<script lang="ts">
  import { List, SquareKanban, StickyNote } from '@lucide/svelte';
  import type { Snippet } from 'svelte';

  interface Props {
    variant: 'board' | 'list' | 'notes';
    title: string;
    description: string;
    action?: Snippet;
  }

  let { variant, title, description, action }: Props = $props();
</script>

<div class="empty">
  <div class="glyph">
    {#if variant === 'board'}
      <SquareKanban size={20} />
    {:else if variant === 'notes'}
      <StickyNote size={20} />
    {:else}
      <List size={20} />
    {/if}
  </div>
  <div class="title">{title}</div>
  <p class="description">{description}</p>
  {#if action}
    <div class="action">{@render action()}</div>
  {/if}
</div>

<style>
  .empty {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
    max-width: 340px;
    padding: 2px 0 2px 18px;
    border-left: 2px solid var(--accent);
    text-align: left;
  }

  .glyph {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 3px;
    color: var(--muted-fg);
    opacity: 0.65;
  }

  .title {
    font-size: var(--text-md);
    font-weight: var(--weight-semibold);
  }

  .description {
    margin: 0;
    color: var(--muted-fg);
    font-size: var(--text-base);
    line-height: var(--leading-normal);
  }

  .action {
    margin-top: 8px;
  }
</style>

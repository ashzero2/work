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
    align-items: center;
    gap: 6px;
    max-width: 340px;
    text-align: center;
  }

  .glyph {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 2px;
    color: var(--muted-fg);
    opacity: 0.85;
  }

  .title {
    font-size: 15px;
    font-weight: 600;
  }

  .description {
    margin: 0;
    color: var(--muted-fg);
    font-size: 13px;
    line-height: 1.5;
  }

  .action {
    margin-top: 10px;
  }
</style>

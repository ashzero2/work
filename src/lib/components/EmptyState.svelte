<script lang="ts">
  import { List, SquareKanban } from '@lucide/svelte';
  import type { Snippet } from 'svelte';

  interface Props {
    variant: 'board' | 'list';
    title: string;
    description: string;
    action?: Snippet;
  }

  let { variant, title, description, action }: Props = $props();
</script>

<div class="empty">
  <div class="badge">
    {#if variant === 'board'}
      <SquareKanban size={22} />
    {:else}
      <List size={22} />
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
    gap: 8px;
    max-width: 320px;
    text-align: center;
  }

  .badge {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 44px;
    height: 44px;
    margin-bottom: 4px;
    border-radius: var(--radius);
    background: var(--lane);
    color: var(--muted-fg);
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
    margin-top: 8px;
  }
</style>

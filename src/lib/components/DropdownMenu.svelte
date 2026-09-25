<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    align?: 'start' | 'end';
    trigger: Snippet<[{ toggle: () => void; open: boolean }]>;
    content: Snippet<[{ close: () => void }]>;
  }

  let { align = 'end', trigger, content }: Props = $props();

  let open = $state(false);
  let root: HTMLDivElement | null = null;

  const toggle = (): void => {
    open = !open;
  };

  const close = (): void => {
    open = false;
  };

  function onPointerDown(event: PointerEvent): void {
    if (open && root && !root.contains(event.target as Node)) close();
  }

  function onKeydown(event: KeyboardEvent): void {
    if (open && event.key === 'Escape') close();
  }
</script>

<svelte:window onpointerdown={onPointerDown} onkeydown={onKeydown} />

<div class="dropdown" bind:this={root}>
  {@render trigger({ toggle, open })}
  {#if open}
    <div class="popup" class:start={align === 'start'} role="menu">
      {@render content({ close })}
    </div>
  {/if}
</div>

<style>
  .dropdown {
    position: relative;
    display: inline-flex;
  }

  .popup {
    position: absolute;
    top: calc(100% + 4px);
    right: 0;
    z-index: 30;
    min-width: 168px;
    padding: 4px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--panel);
    box-shadow: var(--shadow-md);
  }

  .popup.start {
    right: auto;
    left: 0;
  }
</style>

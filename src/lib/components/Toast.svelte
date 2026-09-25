<script lang="ts">
  import { X } from '@lucide/svelte';

  import { toast } from '$lib/stores/toast.svelte';
</script>

{#if toast.message}
  <div class="toast" role="status">
    <span>{toast.message}</span>
    <button class="icon-btn" aria-label="Dismiss" onclick={() => toast.clear()}>
      <X size={14} />
    </button>
  </div>
{/if}

<style>
  .toast {
    position: fixed;
    left: 50%;
    bottom: 24px;
    z-index: 80;
    display: flex;
    align-items: center;
    gap: 12px;
    max-width: 440px;
    padding: 9px 10px 9px 14px;
    transform: translateX(-50%);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--panel);
    box-shadow: var(--shadow-md);
    font-size: 13px;
  }
</style>

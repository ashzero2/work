<script lang="ts">
  import { navigation } from '$lib/stores/navigation.svelte';
  import { phaseLabel, pomodoro } from '$lib/stores/pomodoro.svelte';
</script>

{#if pomodoro.running}
  <button class="indicator" title="Open the timer" onclick={() => navigation.setSection('focus')}>
    <span class="phase">{phaseLabel[pomodoro.kind]}</span>
    <span class="clock">{pomodoro.clock}</span>
  </button>
{/if}

<style>
  .indicator {
    display: flex;
    flex: 1;
    min-width: 0;
    align-items: center;
    gap: 8px;
    padding: 5px 9px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--panel);
    color: var(--fg);
    font-size: 12px;
    font-variant-numeric: tabular-nums;
    cursor: pointer;
  }

  .indicator:hover {
    border-color: var(--accent);
  }

  .phase {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    color: var(--muted-fg);
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .clock {
    font-weight: 600;
  }
</style>

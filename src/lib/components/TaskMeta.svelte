<script lang="ts">
  import { CalendarDays, RotateCw } from '@lucide/svelte';

  import { dueChip, priorityLabel, priorityTone, repeatLabel } from '$lib/format';
  import type { Task } from '$lib/types';

  interface Props {
    task: Task;
  }

  let { task }: Props = $props();

  const due = $derived(task.dueAt ? dueChip(task.dueAt) : null);
  const tone = $derived(priorityTone[task.priority]);
</script>

{#if tone || due || task.repeatRule}
  <div class="meta">
    {#if tone}
      <span class="chip chip-{tone}">{priorityLabel[task.priority]}</span>
    {/if}
    {#if due}
      <span class="chip chip-{due.tone} tnum"><CalendarDays size={11} />{due.label}</span>
    {/if}
    {#if task.repeatRule}
      <span class="chip chip-muted"><RotateCw size={11} />{repeatLabel[task.repeatRule]}</span>
    {/if}
  </div>
{/if}

<style>
  .meta {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 6px;
  }
</style>

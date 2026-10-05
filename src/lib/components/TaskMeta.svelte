<script lang="ts">
  import { CalendarDays, ListChecks, RotateCw } from '@lucide/svelte';

  import { dueChip, priorityLabel, priorityTone, repeatLabel } from '$lib/format';
  import type { Task } from '$lib/types';

  interface Props {
    task: Task;
    subtasksDone?: number;
    subtasksTotal?: number;
  }

  let { task, subtasksDone = 0, subtasksTotal = 0 }: Props = $props();

  const due = $derived(task.dueAt ? dueChip(task.dueAt) : null);
  const tone = $derived(priorityTone[task.priority]);
</script>

{#if tone || due || task.repeatRule || subtasksTotal > 0}
  <div class="meta">
    {#if tone}
      <span class="chip chip-{tone}">{priorityLabel[task.priority]}</span>
    {/if}
    {#if due}
      <span class="chip chip-{due.tone}"><CalendarDays size={11} />{due.label}</span>
    {/if}
    {#if task.repeatRule}
      <span class="chip chip-muted"><RotateCw size={11} />{repeatLabel[task.repeatRule]}</span>
    {/if}
    {#if subtasksTotal > 0}
      <span class="chip chip-muted"><ListChecks size={11} />{subtasksDone}/{subtasksTotal}</span>
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

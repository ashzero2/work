<script lang="ts">
  import { Command, Plus } from '@lucide/svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import { matchesTask, type TaskStatusFilter, type TaskDueFilter } from '$lib/task-filter';
  import type { Column, Priority } from '$lib/types';
  import Board from './Board.svelte';
  import TaskList from './TaskList.svelte';
  import TaskCreateDialog from './TaskCreateDialog.svelte';
  import ViewToggle from './ViewToggle.svelte';

  let { onaddcolumn, onrename, oncommand }: { onaddcolumn: () => void; onrename: (column: Column) => void; oncommand: () => void } = $props();
  let status = $state<TaskStatusFilter>('all');
  let priority = $state<Priority | 'all'>('all');
  let due = $state<TaskDueFilter>('all');
  let creating = $state(false);
  let columnId = $state<number | null>(null);
  const filtered = $derived(workspace.tasks.filter(task => matchesTask(task, { status, priority, due })));
  const dueToday = $derived(workspace.tasks.filter(task => task.parentTaskId === null && matchesTask(task, { status: 'open', priority: 'all', due: 'today' })).length);
  const completedThisWeek = $derived.by(() => {
    const start = new Date();
    start.setDate(start.getDate() - (start.getDay() + 6) % 7);
    start.setHours(0, 0, 0, 0);
    return workspace.tasks.filter(task => task.parentTaskId === null && task.completedAt && new Date(task.completedAt) >= start).length;
  });
  function addTask(id: number | null = null): void { columnId = id; creating = true; }
</script>

<section class="workspace-view">
  <header class="workspace-head" data-tauri-drag-region>
    <div><h1>Tasks</h1><p>{workspace.openCount} open · {workspace.doneCount} done · Local workspace</p></div>
    <div class="head-actions"><button class="btn" onclick={oncommand}><Command size={14} /> Command menu</button><button class="btn btn-primary" onclick={() => workspace.columns.length ? addTask() : onaddcolumn()}><Plus size={15} /> New task</button></div>
  </header>
  <div class="overview" aria-label="Task overview">
    <div><strong>{workspace.openCount}</strong><span>Open tasks</span></div>
    <div><strong>{dueToday}</strong><span>Due today</span></div>
    <div><strong>{completedThisWeek}</strong><span>Completed this week</span></div>
  </div>
  <div class="tools">
    <div class="filters">
      <select aria-label="Task status" bind:value={status}><option value="all">All tasks</option><option value="open">Open tasks</option><option value="completed">Completed</option></select>
      <select aria-label="Priority" bind:value={priority}><option value="all">Priority</option><option value="high">High</option><option value="medium">Medium</option><option value="low">Low</option><option value="none">No priority</option></select>
      <select aria-label="Due date" bind:value={due}><option value="all">Due date</option><option value="today">Today</option><option value="overdue">Overdue</option><option value="undated">No date</option></select>
    </div>
    <ViewToggle />
  </div>
  <div class="workspace-body tasks-body">
    {#if workspace.viewMode === 'board'}<Board tasks={filtered} {onrename} {onaddcolumn} onaddtask={addTask} />{:else}<TaskList tasks={filtered} />{/if}
  </div>
</section>
{#if creating}<TaskCreateDialog {columnId} onclose={() => creating = false} />{/if}

<style>
  .overview { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); margin: 18px 28px; border: 1px solid var(--border); border-radius: var(--radius-lg); background: var(--panel); }
  .overview div { display: flex; align-items: baseline; gap: 9px; padding: 10px 14px; border-right: 1px solid var(--border); }
  .overview div:last-child { border: 0; } .overview strong { font-size: 17px; font-variant-numeric: tabular-nums; } .overview span { font-size: 12px; color: var(--muted-fg); }
  .tools, .filters { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
  .tools { justify-content: space-between; padding: 0 28px 14px; }
  select { appearance: none; min-height: 36px; padding: 0 28px 0 10px; border: 1px solid var(--field-border); border-radius: var(--radius-sm); background: var(--panel) url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath d='m3 5 3 3 3-3' fill='none' stroke='%23888' stroke-width='1.5'/%3E%3C/svg%3E") no-repeat right 9px center; color: var(--fg); font: inherit; font-size: 13px; }
  .tasks-body { padding: 0 28px 28px; }
  @media (max-width: 700px) { .overview { margin-inline: 16px; } .overview div { flex-direction: column; gap: 2px; padding: 10px; } .tools { padding-inline: 16px; } .tasks-body { padding-inline: 16px; } }
</style>

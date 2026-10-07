<script lang="ts">
  import { Plus, Zap } from '@lucide/svelte';
  import { showCaptureWindow } from '$lib/api';
  import { workspace } from '$lib/stores/workspace.svelte';
  import { matchesTask, type TaskStatusFilter, type TaskDueFilter } from '$lib/task-filter';
  import type { Column, Priority } from '$lib/types';
  import Board from './Board.svelte';
  import TaskList from './TaskList.svelte';
  import TaskCreateDialog from './TaskCreateDialog.svelte';
  import TaskEditDialog from './TaskEditDialog.svelte';
  import ViewToggle from './ViewToggle.svelte';

  let { onaddcolumn, onrename }: { onaddcolumn: () => void; onrename: (column: Column) => void } = $props();
  let status = $state<TaskStatusFilter>('all');
  let priority = $state<Priority | 'all'>('all');
  let due = $state<TaskDueFilter>('all');
  let creating = $state(false);
  let columnId = $state<number | null>(null);
  let editingId = $state<number | null>(null);
  const editing = $derived(workspace.tasks.find((task) => task.id === editingId) ?? null);
  const filtered = $derived.by(() => {
    const matching = workspace.tasks.filter(task => matchesTask(task, { status, priority, due }));
    const parentIds = new Set(matching.flatMap(task => task.parentTaskId === null ? [] : [task.parentTaskId]));
    const context = workspace.tasks.filter(task => task.parentTaskId === null && parentIds.has(task.id));
    return [...new Map([...context, ...matching].map(task => [task.id, task])).values()];
  });
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
    <div class="head-actions"><button class="btn bolt" title="Quick capture" aria-label="Quick capture" onclick={() => void showCaptureWindow()}><Zap size={16} /></button><button class="btn btn-primary" onclick={() => workspace.columns.length ? addTask() : onaddcolumn()}><Plus size={15} /> New task</button></div>
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
    {#if workspace.viewMode === 'board'}<Board tasks={filtered} {onrename} {onaddcolumn} onaddtask={addTask} onedit={(id) => (editingId = id)} />{:else}<TaskList tasks={filtered} onedit={(id) => (editingId = id)} />{/if}
  </div>
</section>
{#if creating}<TaskCreateDialog {columnId} onclose={() => creating = false} />{/if}
{#if editing}
  {#key editing.id}<TaskEditDialog task={editing} onclose={() => (editingId = null)} />{/key}
{/if}

<style>
  /* Icon-only, but sized and bordered like its neighbours so the header row reads
     as one set of controls. */
  .bolt { width: 36px; min-height: 36px; padding: 0; }
  .overview { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); margin: 18px var(--gutter); border: 1px solid var(--border); border-radius: var(--radius); background: var(--panel); }
  .overview div { display: flex; align-items: baseline; gap: 9px; padding: 9px 14px; border-right: 1px solid var(--divider); }
  .overview div:last-child { border-right: 0; }
  .overview strong { font-family: var(--font-mono); font-size: var(--text-lg); font-weight: var(--weight-medium); letter-spacing: var(--tracking-tight); }
  .overview span { font-size: var(--text-sm); color: var(--muted-fg); }
  .tools, .filters { display: flex; align-items: center; gap: var(--space-2); flex-wrap: wrap; }
  .tools { justify-content: space-between; padding: 0 var(--gutter) 14px; }
  select { appearance: none; min-height: 32px; padding: 0 28px 0 10px; border: 1px solid var(--field-border); border-radius: var(--radius-sm); background: var(--panel) url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath d='m3 5 3 3 3-3' fill='none' stroke='%23888' stroke-width='1.5'/%3E%3C/svg%3E") no-repeat right 9px center; color: var(--fg); font: inherit; font-size: var(--text-base); }
  .tasks-body { padding: 0 var(--gutter) 28px; }
  @media (max-width: 700px) { .overview { margin-inline: 16px; } .overview div { flex-direction: column; gap: 2px; padding: 10px; } .tools { padding-inline: 16px; } .tasks-body { padding-inline: 16px; } }
</style>

<script lang="ts">
  import { Check, Trash2 } from '@lucide/svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Task } from '$lib/types';
  import LoadingState from './LoadingState.svelte';
  let { tasks }: { tasks: Task[] } = $props();
  const ordered = $derived.by(() => {
    const columnOrder = new Map(workspace.columns.map((column, index) => [column.id, index]));
    const byPosition = (a: Task, b: Task): number =>
      (columnOrder.get(a.columnId) ?? 0) - (columnOrder.get(b.columnId) ?? 0) || a.position - b.position;
    const parents = tasks.filter(task => task.parentTaskId === null).sort(byPosition);
    const children = tasks.filter(task => task.parentTaskId !== null).sort(byPosition);
    const parentIds = new Set(parents.map(task => task.id));
    return parents.flatMap(parent => [parent, ...children.filter(child => child.parentTaskId === parent.id)])
      .concat(children.filter(child => !parentIds.has(child.parentTaskId!)));
  });
</script>
<div class="list">
  <table>
    <thead><tr><th aria-label="Completion"></th><th>Task</th><th>Column</th><th>Priority</th><th>Due</th><th aria-label="Actions"></th></tr></thead>
    <tbody>{#each ordered as task (task.id)}<tr class:done={task.completedAt !== null} data-task-id={task.id}>
      <td><button class="checkbox" disabled={task.completedAt !== null && task.repeatRule !== null} title={task.completedAt ? (task.repeatRule ? 'Repeating tasks can’t be reopened' : 'Reopen task') : 'Complete task'} aria-label={task.completedAt ? `Reopen ${task.title}` : `Complete ${task.title}`} onclick={() => void workspace.toggleTaskCompletion(task.id)}>{#if task.completedAt}<Check size={13} />{/if}</button></td>
      <td class="task-title" class:child={task.parentTaskId !== null}>{task.title}</td>
      <td>{workspace.columns.find(column => column.id === task.columnId)?.name ?? '—'}</td>
      <td class="priority">{task.priority === 'none' ? '—' : task.priority}</td>
      <td class="tnum">{task.dueAt ? new Date(task.dueAt).toLocaleDateString(undefined, { month: 'short', day: 'numeric' }) : '—'}</td>
      <td><button class="icon-btn" aria-label={`Delete ${task.title}`} onclick={() => void workspace.deleteTask(task.id)}><Trash2 size={14} /></button></td>
    </tr>{/each}</tbody>
  </table>
  {#if workspace.loading}<LoadingState label="Loading tasks…" />{:else if ordered.length === 0}<p class="empty">No tasks match these filters.</p>{/if}
</div>
<style>
  .list { height: 100%; overflow: auto; } table { width: 100%; border-collapse: collapse; font-size: var(--text-base); text-align: left; }
  th { position: sticky; top: 0; background: var(--lane); color: var(--muted-fg); font-size: var(--text-2xs); font-weight: var(--weight-semibold); text-transform: uppercase; letter-spacing: var(--tracking-label); }
  th, td { padding: 10px 8px; border-bottom: 1px solid var(--divider); } td { background: var(--panel); } tr:hover td { background: var(--hover); }
  .task-title { width: 45%; font-weight: var(--weight-medium); } .task-title.child { padding-left: 28px; } .priority { text-transform: capitalize; } .done .task-title { text-decoration: line-through; color: var(--muted-fg); }
  .checkbox { width: 18px; height: 18px; display: flex; align-items: center; justify-content: center; padding: 0; border: 1px solid var(--field-border); border-radius: var(--radius-xs); background: var(--panel); color: var(--accent); }
  .empty { padding: 30px; text-align: center; color: var(--muted-fg); }
</style>

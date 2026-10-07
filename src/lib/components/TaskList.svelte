<script lang="ts">
  import { Check, ListChecks, Pencil, Plus, Trash2 } from '@lucide/svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Task } from '$lib/types';
  import LoadingState from './LoadingState.svelte';
  import TaskSubtaskAdd from './TaskSubtaskAdd.svelte';
  let { tasks, onedit }: { tasks: Task[]; onedit: (id: number) => void } = $props();
  let addingFor = $state<number | null>(null);
  const groups = $derived.by(() => {
    const columnOrder = new Map(workspace.columns.map((column, index) => [column.id, index]));
    const byPosition = (a: Task, b: Task): number =>
      (columnOrder.get(a.columnId) ?? 0) - (columnOrder.get(b.columnId) ?? 0) || a.position - b.position;
    const parents = tasks.filter(task => task.parentTaskId === null).sort(byPosition);
    const children = tasks.filter(task => task.parentTaskId !== null).sort(byPosition);
    return parents.map(parent => ({
      parent,
      children: children.filter(child => child.parentTaskId === parent.id)
    }));
  });
</script>
<div class="list">
  <table>
    <thead><tr><th aria-label="Completion"></th><th>Task</th><th>Column</th><th>Priority</th><th>Due</th><th aria-label="Actions"></th></tr></thead>
    {#each groups as group (group.parent.id)}
      {@const adding = addingFor === group.parent.id}
      <tbody class:has-subtasks={group.children.length > 0 || adding}>
        {#each [group.parent, ...group.children] as task, index (task.id)}
          {@const child = index > 0}
          {@const progress = child ? null : workspace.subtaskProgress(task.id)}
          <tr class:done={task.completedAt !== null} class:child-row={child} class:child-last={child && index === group.children.length && !adding} class:parent-row={!child && (group.children.length > 0 || adding)} data-task-id={task.id}>
            <td class="completion-cell">
              {#if !child}
                <button class="checkbox" disabled={task.completedAt !== null && task.repeatRule !== null} title={task.completedAt ? (task.repeatRule ? 'Repeating tasks can’t be reopened' : 'Reopen task') : 'Complete task'} aria-label={task.completedAt ? `Reopen ${task.title}` : `Complete ${task.title}`} onclick={() => void workspace.toggleTaskCompletion(task.id)}>{#if task.completedAt}<Check size={13} />{/if}</button>
              {/if}
            </td>
            <td class="task-title" class:child={child}>
              {#if child}
                <div class="subtask-entry">
                  <button class="checkbox" disabled={task.completedAt !== null && task.repeatRule !== null} title={task.completedAt ? (task.repeatRule ? 'Repeating tasks can’t be reopened' : 'Reopen subtask') : 'Complete subtask'} aria-label={task.completedAt ? `Reopen ${task.title}` : `Complete ${task.title}`} onclick={() => void workspace.toggleTaskCompletion(task.id)}>{#if task.completedAt}<Check size={13} />{/if}</button>
                  <button class="child-title" onclick={() => onedit(task.id)}>{task.title}</button>
                </div>
              {:else}
                {task.title}
                {#if progress && progress.total > 0}<span class="subtask-progress"><ListChecks size={12} />{progress.done}/{progress.total}</span>{/if}
              {/if}
            </td>
            <td>{workspace.columns.find(column => column.id === task.columnId)?.name ?? '—'}</td>
            <td class="priority">{task.priority === 'none' ? '—' : task.priority}</td>
            <td class="tnum">{task.dueAt ? new Date(task.dueAt).toLocaleDateString(undefined, { month: 'short', day: 'numeric' }) : '—'}</td>
            <td class="actions">{#if !child}<button class="icon-btn" title="Add subtask" aria-label={`Add subtask to ${task.title}`} onclick={() => (addingFor = task.id)}><Plus size={14} /></button>{/if}<button class="icon-btn" aria-label={`Edit ${task.title}`} onclick={() => onedit(task.id)}><Pencil size={14} /></button><button class="icon-btn" aria-label={`Delete ${task.title}`} onclick={() => void workspace.deleteTask(task.id)}><Trash2 size={14} /></button></td>
          </tr>
        {/each}
        {#if adding}
          <tr class="child-row child-last subtask-add-row">
            <td class="completion-cell"></td>
            <td class="task-title"><TaskSubtaskAdd parentId={group.parent.id} onclose={() => (addingFor = null)} /></td>
            <td colspan="4"></td>
          </tr>
        {/if}
      </tbody>
    {/each}
  </table>
  {#if workspace.loading}<LoadingState label="Loading tasks…" />{:else if groups.length === 0}<p class="empty">No tasks match these filters.</p>{/if}
</div>
<style>
  .list { height: 100%; overflow: auto; } table { width: 100%; border-collapse: collapse; font-size: var(--text-base); text-align: left; }
  th { position: sticky; top: 0; background: var(--lane); color: var(--muted-fg); font-size: var(--text-2xs); font-weight: var(--weight-semibold); text-transform: uppercase; letter-spacing: var(--tracking-label); }
  th, td { padding: 10px 8px; border-bottom: 1px solid var(--divider); } td { background: var(--panel); } tr:hover td { background: var(--hover); }
  .task-title { width: 45%; font-weight: var(--weight-medium); } .priority { text-transform: capitalize; } .done .task-title { text-decoration: line-through; color: var(--muted-fg); }
  .subtask-progress { display: inline-flex; align-items: center; gap: 4px; margin-left: 9px; color: var(--muted-fg); font-size: var(--text-xs); font-weight: var(--weight-normal); font-variant-numeric: tabular-nums; text-decoration: none; vertical-align: middle; }
  tbody.has-subtasks tr.parent-row td, tbody.has-subtasks tr.child-row td { background: color-mix(in srgb, var(--panel) 96%, var(--accent) 4%); }
  tbody.has-subtasks tr.parent-row td { border-bottom-width: 0; }
  tbody.has-subtasks tr:hover td { background: var(--hover); }
  tr.child-row td { border-bottom-color: var(--divider); }
  .subtask-entry { display: flex; align-items: center; gap: 9px; min-height: 24px; padding: 2px 0; }
  /* The rail hangs in the tick column so it sits under the parent's checkbox, and
     the branch carries the connector across the gutter to the subtask's checkbox,
     which lines up under the parent's title. 17px is that column's padding plus
     half a tick, so the branch starts there and ends one gutter past the column. */
  tr.child-row .completion-cell { position: relative; }
  tr.child-row .completion-cell::before { position: absolute; left: 17px; top: -1px; bottom: -1px; border-left: 1px solid color-mix(in srgb, var(--muted-fg) 55%, transparent); content: ''; }
  tr.child-row .completion-cell::after { position: absolute; left: 17px; top: calc(50% - 0.5px); width: calc(100% - 9px); border-top: 1px solid color-mix(in srgb, var(--muted-fg) 55%, transparent); content: ''; }
  tr.child-last .completion-cell::before { bottom: 50%; }
  .child-title { min-width: 0; padding: 3px 0; border: 0; background: transparent; color: var(--fg); font: inherit; font-size: var(--text-base); text-align: left; cursor: pointer; }
  .child-title:hover { color: var(--accent); }
  .actions { white-space: nowrap; }
  .checkbox { width: 18px; height: 18px; display: flex; align-items: center; justify-content: center; padding: 0; border: 1px solid var(--field-border); border-radius: var(--radius-xs); background: var(--panel); color: var(--accent); }
  .empty { padding: 30px; text-align: center; color: var(--muted-fg); }
</style>

<script lang="ts">
  import { Check, ListChecks, Pencil, Trash2 } from '@lucide/svelte';

  import { workspace } from '$lib/stores/workspace.svelte';
  import type { Task } from '$lib/types';
  import TaskMeta from './TaskMeta.svelte';
  import TaskSubtaskAdd from './TaskSubtaskAdd.svelte';

  let {
    parent,
    onedit,
    adding = $bindable(false)
  }: { parent: Task; onedit: (id: number) => void; adding?: boolean } = $props();
  const subtasks = $derived(
    workspace.tasks
      .filter((task) => task.parentTaskId === parent.id)
      .sort((a, b) => a.position - b.position)
  );

</script>

{#if subtasks.length > 0 || adding}
<section class="subtasks" aria-label="Subtasks">
  {#if subtasks.length > 0}
    <div class="section-head">
      <span class="section-title"><ListChecks size={13} />Subtasks</span>
      <span class="count">{workspace.subtaskProgress(parent.id).done}/{subtasks.length}</span>
    </div>
    <ul>
      {#each subtasks as subtask (subtask.id)}
        <li class:completed={subtask.completedAt !== null}>
          <button
            class="checkbox"
            aria-label={subtask.completedAt ? `Reopen ${subtask.title}` : `Complete ${subtask.title}`}
            title={subtask.completedAt ? 'Reopen subtask' : 'Complete subtask'}
            disabled={subtask.completedAt !== null && subtask.repeatRule !== null}
            onclick={() => void workspace.toggleTaskCompletion(subtask.id)}
          >
            {#if subtask.completedAt}<Check size={13} />{/if}
          </button>
          <button class="title" onclick={() => onedit(subtask.id)}>{subtask.title}</button>
          <button class="icon-btn" aria-label={`Edit ${subtask.title}`} title="Edit subtask" onclick={() => onedit(subtask.id)}>
            <Pencil size={14} />
          </button>
          <button class="icon-btn" aria-label={`Delete ${subtask.title}`} title="Delete subtask" onclick={() => void workspace.deleteTask(subtask.id)}>
            <Trash2 size={14} />
          </button>
        </li>
        {#if subtask.priority !== 'none' || subtask.dueAt}
          <li class="metadata"><TaskMeta task={subtask} /></li>
        {/if}
      {/each}
    </ul>
  {/if}
  {#if adding}
    <TaskSubtaskAdd parentId={parent.id} onclose={() => (adding = false)} />
  {/if}
</section>
{/if}

<style>
  .subtasks { display: flex; flex-direction: column; gap: 5px; padding-top: 8px; border-top: 1px solid var(--divider); }
  .section-head { display: flex; align-items: center; justify-content: space-between; padding-inline: 2px; }
  .section-title { display: inline-flex; align-items: center; gap: 5px; color: var(--muted-fg); font-size: var(--text-xs); font-weight: var(--weight-semibold); }
  .count { color: var(--muted-fg); font-family: var(--font-mono); font-size: var(--text-xs); font-variant-numeric: tabular-nums; }
  ul { display: flex; flex-direction: column; gap: 1px; max-height: 190px; overflow-y: auto; margin: 0; padding: 0; list-style: none; }
  li { display: flex; align-items: center; gap: 5px; min-height: 28px; padding-left: 3px; }
  .title { flex: 1; min-width: 0; padding: 3px 0; overflow-wrap: anywhere; border: 0; background: transparent; color: var(--fg); font: inherit; font-size: var(--text-sm); text-align: left; cursor: pointer; }
  .title:hover { color: var(--accent); }
  .completed .title { color: var(--muted-fg); text-decoration: line-through; }
  .checkbox { display: flex; flex: 0 0 18px; align-items: center; justify-content: center; width: 18px; height: 18px; padding: 0; border: 1px solid var(--field-border); border-radius: var(--radius-xs); background: var(--panel); color: var(--accent); cursor: pointer; }
  .metadata { min-height: 0; padding: 0 0 3px 26px; }
</style>

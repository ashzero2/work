<script lang="ts">
  import { ChevronDown } from '@lucide/svelte';

  import { fromDatetimeLocal, toDatetimeLocal } from '$lib/format';
  import { kindHint, kindLabel, reminders } from '$lib/stores/reminders.svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import type { ReminderKind } from '$lib/types';
  import DropdownMenu from './DropdownMenu.svelte';
  import DateTimeField from './DateTimeField.svelte';

  interface Props {
    onclose: () => void;
  }

  let { onclose }: Props = $props();

  const kinds = Object.keys(kindLabel) as ReminderKind[];

  function defaultWhen(): string {
    const date = new Date();
    date.setHours(date.getHours() + 1, 0, 0, 0);
    return toDatetimeLocal(date.toISOString());
  }

  let kind = $state<ReminderKind>('custom');
  let taskId = $state<number | null>(null);
  let when = $state(defaultWhen());
  let saving = $state(false);

  let dialog: HTMLDivElement | null = null;

  const openTasks = $derived(
    workspace.tasks.filter((task) => task.completedAt === null && task.parentTaskId === null)
  );
  const attached = $derived(openTasks.find((task) => task.id === taskId) ?? null);
  const ready = $derived(when.length > 0 && !saving);

  async function save(): Promise<void> {
    if (!ready) return;
    saving = true;
    try {
      await reminders.create(kind, taskId, fromDatetimeLocal(when));
      onclose();
    } finally {
      saving = false;
    }
  }

  function onKeydown(event: KeyboardEvent): void {
    if (event.key === 'Escape') onclose();
  }

  function onPointerDown(event: PointerEvent): void {
    if (dialog && !dialog.contains(event.target as Node)) onclose();
  }
</script>

<svelte:window onpointerdown={onPointerDown} onkeydown={onKeydown} />

<div class="backdrop">
  <div class="dialog" bind:this={dialog} role="dialog" aria-modal="true" aria-label="New reminder">
    <div class="block">
      <span class="field-label">Kind</span>
      <DropdownMenu align="start">
        {#snippet trigger({ toggle })}
          <button class="picker" onclick={toggle}>
            {kindLabel[kind]}
            <ChevronDown size={14} />
          </button>
        {/snippet}
        {#snippet content({ close })}
          {#each kinds as option (option)}
            <button
              class="menu-item"
              onclick={() => {
                kind = option;
                close();
              }}
            >
              {kindLabel[option]}
            </button>
          {/each}
        {/snippet}
      </DropdownMenu>
      <p class="hint">{kindHint[kind]}</p>
    </div>

    <div class="block">
      <span class="field-label">Task</span>
      <DropdownMenu align="start">
        {#snippet trigger({ toggle })}
          <button class="picker" onclick={toggle}>
            {attached?.title ?? 'None'}
            <ChevronDown size={14} />
          </button>
        {/snippet}
        {#snippet content({ close })}
          <button
            class="menu-item"
            onclick={() => {
              taskId = null;
              close();
            }}
          >
            None
          </button>
          {#each openTasks as task (task.id)}
            <button
              class="menu-item"
              onclick={() => {
                taskId = task.id;
                close();
              }}
            >
              {task.title}
            </button>
          {/each}
        {/snippet}
      </DropdownMenu>
    </div>

    <DateTimeField bind:value={when} label="When" />

    <footer>
      <div class="spacer"></div>
      <button class="btn" onclick={onclose}>Cancel</button>
      <button class="btn btn-primary" onclick={() => void save()} disabled={!ready}>
        Add reminder
      </button>
    </footer>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 60;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
    background: color-mix(in srgb, #000 32%, transparent);
  }

  .dialog {
    display: flex;
    flex-direction: column;
    gap: 14px;
    width: 380px;
    max-width: 100%;
    padding: 16px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--panel);
    box-shadow: var(--shadow-md);
  }

  .block {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .field-label {
    color: var(--muted-fg);
    font-size: var(--text-2xs);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-label);
    text-transform: uppercase;
  }

  .picker {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    height: 34px;
    padding: 0 11px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg);
    color: var(--fg);
    font-size: var(--text-base);
    cursor: pointer;
  }

  .picker:hover {
    border-color: var(--accent);
  }

  .hint {
    margin: 0;
    color: var(--muted-fg);
    font-size: var(--text-xs);
  }

  footer {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 2px;
  }

  .spacer {
    flex: 1;
  }
</style>

<script lang="ts">
  import { ChevronDown, Play, SkipForward, Square } from '@lucide/svelte';

  import { phaseLabel, pomodoro } from '$lib/stores/pomodoro.svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import DropdownMenu from './DropdownMenu.svelte';
  import PomodoroDurations from './PomodoroDurations.svelte';

  const openTasks = $derived(
    workspace.tasks.filter((task) => task.completedAt === null && task.parentTaskId === null)
  );
  const attached = $derived(openTasks.find((task) => task.id === pomodoro.taskId) ?? null);
  const cycleStep = $derived((pomodoro.cycle % pomodoro.settings.sessionsPerLongBreak) + 1);

  const when = (iso: string): string =>
    new Date(iso).toLocaleTimeString(undefined, { hour: '2-digit', minute: '2-digit' });
</script>

<section class="view">
  <header class="view-head" data-tauri-drag-region>
    <div class="titles">
      <span class="title">Focus</span>
      <span class="summary">
        {#if pomodoro.running}
          {phaseLabel[pomodoro.kind]} · session {cycleStep} of
          {pomodoro.settings.sessionsPerLongBreak}
        {:else}
          Next up: {phaseLabel[pomodoro.nextKind].toLowerCase()}
        {/if}
      </span>
    </div>
  </header>

  <div class="body">
    <div class="dial">
      <span class="clock" class:idle={!pomodoro.running}>{pomodoro.clock}</span>
      <span class="phase">{pomodoro.running ? phaseLabel[pomodoro.kind] : 'Ready'}</span>
      <div class="track">
        <div
          class="fill"
          class:resting={pomodoro.kind !== 'work'}
          style="width: {Math.round(pomodoro.progress * 100)}%"
        ></div>
      </div>
    </div>

    <div class="controls">
      {#if pomodoro.running}
        <button class="btn" onclick={() => void pomodoro.skip()}>
          <SkipForward size={15} /> Skip
        </button>
        <button class="btn" onclick={() => void pomodoro.stop()}>
          <Square size={15} /> Stop
        </button>
      {:else}
        <button class="btn btn-primary" onclick={() => void pomodoro.start()}>
          <Play size={15} /> Start {phaseLabel[pomodoro.nextKind].toLowerCase()}
        </button>
      {/if}
    </div>

    <div class="block">
      <span class="block-label">Attached task</span>
      <DropdownMenu align="start">
        {#snippet trigger({ toggle })}
          <button class="btn picker" onclick={toggle} disabled={pomodoro.running}>
            {attached?.title ?? 'None'}
            <ChevronDown size={14} />
          </button>
        {/snippet}
        {#snippet content({ close })}
          <button
            class="menu-item"
            onclick={() => {
              pomodoro.setTask(null);
              close();
            }}
          >
            None
          </button>
          {#each openTasks as task (task.id)}
            <button
              class="menu-item"
              onclick={() => {
                pomodoro.setTask(task.id);
                close();
              }}
            >
              {task.title}
            </button>
          {/each}
        {/snippet}
      </DropdownMenu>
      {#if pomodoro.running}
        <p class="hint">A running session keeps the task it started with.</p>
      {/if}
    </div>

    <div class="block">
      <span class="block-label">Durations, in minutes</span>
      <PomodoroDurations />
    </div>

    {#if pomodoro.recent.length > 0}
      <div class="block">
        <span class="block-label">Recent sessions</span>
        <div class="recent">
          {#each pomodoro.recent as session (session.id)}
            <div class="row">
              <span class="row-phase">{phaseLabel[session.kind]}</span>
              <span class="row-outcome">{session.completed ? 'done' : 'stopped'}</span>
              <span class="row-when">{when(session.startedAt)}</span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</section>

<style>
  .view {
    display: flex;
    flex-direction: column;
    height: 100%;
    min-height: 0;
  }

  .view-head {
    display: flex;
    flex-shrink: 0;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    min-height: 56px;
    padding: 8px 20px;
    border-bottom: 1px solid var(--border);
  }

  .titles {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .title {
    font-size: 15px;
    font-weight: 600;
  }

  .summary {
    color: var(--muted-fg);
    font-size: 11px;
  }

  .body {
    display: flex;
    flex: 1;
    min-height: 0;
    flex-direction: column;
    gap: 24px;
    width: 100%;
    max-width: 420px;
    margin: 0 auto;
    padding: 32px 20px;
    overflow-y: auto;
  }

  .dial {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
  }

  .clock {
    font-size: 44px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
    letter-spacing: -0.01em;
    line-height: 1.1;
  }

  .clock.idle {
    color: var(--muted-fg);
  }

  .phase {
    color: var(--muted-fg);
    font-size: 12px;
  }

  .track {
    width: 100%;
    height: 4px;
    margin-top: 10px;
    border-radius: 999px;
    background: var(--lane);
    overflow: hidden;
  }

  .fill {
    height: 100%;
    border-radius: 999px;
    background: var(--accent);
  }

  .fill.resting {
    background: var(--muted-fg);
  }

  .controls {
    display: flex;
    justify-content: center;
    gap: 8px;
  }

  .block {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .block-label {
    color: var(--muted-fg);
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.03em;
    text-transform: uppercase;
  }

  .picker {
    justify-content: space-between;
    width: 100%;
  }

  .hint {
    margin: 0;
    color: var(--muted-fg);
    font-size: 11px;
  }

  .recent {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 9px;
    border-radius: var(--radius-sm);
    font-size: 12px;
  }

  .row:hover {
    background: var(--lane);
  }

  .row-phase {
    flex: 1;
    min-width: 0;
  }

  .row-outcome {
    color: var(--muted-fg);
  }

  .row-when {
    color: var(--muted-fg);
    font-variant-numeric: tabular-nums;
  }
</style>

<script lang="ts">
  import { ChevronDown, Play, SkipForward, Square } from '@lucide/svelte';

  import { phaseLabel, pomodoro } from '$lib/stores/pomodoro.svelte';
  import { workspace } from '$lib/stores/workspace.svelte';
  import DropdownMenu from './DropdownMenu.svelte';
  import PomodoroDurations from './PomodoroDurations.svelte';

  let durationsOpen = $state(false);

  const openTasks = $derived(
    workspace.tasks.filter((task) => task.completedAt === null && task.parentTaskId === null)
  );
  const attached = $derived(openTasks.find((task) => task.id === pomodoro.taskId) ?? null);
  const cycleStep = $derived((pomodoro.cycle % pomodoro.settings.sessionsPerLongBreak) + 1);

  const when = (iso: string): string =>
    new Date(iso).toLocaleTimeString(undefined, { hour: '2-digit', minute: '2-digit' });
</script>

<section class="workspace-view">
  <header class="workspace-head" data-tauri-drag-region>
    <div class="titles">
      <h1>Focus</h1>
      <p>
        {#if pomodoro.running}
          {phaseLabel[pomodoro.kind]} · session {cycleStep} of
          {pomodoro.settings.sessionsPerLongBreak}
        {:else}
          Next up: {phaseLabel[pomodoro.nextKind].toLowerCase()}
        {/if}
      </p>
    </div>
    <button class="btn" onclick={() => durationsOpen = !durationsOpen}>Adjust durations</button>
  </header>

  <div class="workspace-body body">
    <div class="focus-stage">
      <div class="dial">
        <strong>{attached?.title ?? "Choose a task or focus freely"}</strong>
        <span class="eyebrow">{pomodoro.running ? 'Current session' : 'Ready when you are'}</span>
        <span class="clock" class:idle={!pomodoro.running}>{pomodoro.clock}</span>
        <span class="phase">{pomodoro.running ? phaseLabel[pomodoro.kind] : 'Next up'}</span>
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
          <button class="btn" onclick={() => void pomodoro.stop()}>
            <Square size={15} /> Stop
          </button>
          <button class="btn" onclick={() => void pomodoro.skip()}>
            <SkipForward size={15} /> Skip
          </button>
          <button class="btn btn-primary" onclick={() => void pomodoro.finish()}>
            Finish
          </button>
        {:else}
          <button class="btn btn-primary" onclick={() => void pomodoro.start()}>
            <Play size={15} /> Start {phaseLabel[pomodoro.nextKind].toLowerCase()}
          </button>
        {/if}
      </div>
    </div>

    <div class="focus-details">
      <strong>Session setup</strong>
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

      <div class="setup-row"><span>Next session</span><strong>{phaseLabel[pomodoro.nextKind]}</strong></div>
      <div class="setup-row"><span>Cycle</span><strong>{cycleStep} of {pomodoro.settings.sessionsPerLongBreak}</strong></div>
      {#if durationsOpen}<div class="block"><span class="block-label">Durations, in minutes</span><PomodoroDurations /></div>{/if}

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
  </div>
</section>

<style>
  .body { display: grid; grid-template-columns: minmax(280px, 1fr) minmax(250px, .7fr); align-items: start; gap: 18px; }
  .focus-stage { display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 20px; min-height: 335px; padding: 22px; border: 1px solid var(--border); border-radius: var(--radius-lg); background: var(--panel); }
  .setup-row { display: flex; justify-content: space-between; gap: 12px; padding-bottom: 12px; border-bottom: 1px solid var(--border); font-size: 13px; }
  .dial {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    width: min(100%, 460px);
    text-align: center;
  }

  .eyebrow {
    color: var(--muted-fg);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .clock {
    color: var(--accent);
    font-size: clamp(54px, 8vw, 88px);
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    letter-spacing: -0.06em;
    line-height: 1.1;
  }

  .clock.idle {
    color: var(--muted-fg);
  }

  .phase {
    color: var(--muted-fg);
    font-size: 13px;
  }

  .track {
    width: 100%;
    height: 6px;
    /* Far enough below the clock that it reads as progress, not as an underline
       under the readout. */
    margin-top: 22px;
    border-radius: 999px;
    background: color-mix(in srgb, var(--muted-fg) 22%, transparent);
    overflow: hidden;
  }

  .fill {
    height: 100%;
    border-radius: 999px;
    background: var(--accent);
    transition: width var(--motion-fast) linear;
  }

  .fill.resting {
    background: var(--muted-fg);
  }

  .controls {
    display: flex;
    justify-content: center;
    gap: 8px;
  }

  .focus-details { display: flex; flex-direction: column; gap: 16px; min-width: 0; padding: 14px; border: 1px solid var(--border); border-radius: var(--radius-lg); background: var(--lane); }
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
    padding: 8px 9px;
    border-radius: var(--radius-sm);
    font-size: 13px;
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

  @media (max-width: 800px) {
    .body {
      grid-template-columns: 1fr;
      align-content: start;
      gap: 28px;
      padding: 32px 20px 40px;
    }

  }
</style>

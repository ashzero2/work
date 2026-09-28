<script lang="ts">
  import { pomodoro } from '$lib/stores/pomodoro.svelte';

  let work = $state(25);
  let shortBreak = $state(5);
  let longBreak = $state(15);
  let interval = $state(4);
  let seeded = $state(false);

  $effect(() => {
    if (seeded || !pomodoro.loaded) return;
    seed();
    seeded = true;
  });

  function seed(): void {
    work = Math.round(pomodoro.settings.workSeconds / 60);
    shortBreak = Math.round(pomodoro.settings.breakSeconds / 60);
    longBreak = Math.round(pomodoro.settings.longBreakSeconds / 60);
    interval = pomodoro.settings.sessionsPerLongBreak;
  }

  async function save(): Promise<void> {
    await pomodoro.saveSettings({
      workSeconds: Math.max(1, Math.round(work)) * 60,
      breakSeconds: Math.max(1, Math.round(shortBreak)) * 60,
      longBreakSeconds: Math.max(1, Math.round(longBreak)) * 60,
      sessionsPerLongBreak: Math.round(interval)
    });
    // Show what the backend actually stored, which may have been clamped.
    seed();
  }
</script>

<div class="durations">
  <label>
    <span>Work</span>
    <input type="number" min="1" max="120" bind:value={work} onchange={() => void save()} />
  </label>
  <label>
    <span>Break</span>
    <input type="number" min="1" max="120" bind:value={shortBreak} onchange={() => void save()} />
  </label>
  <label>
    <span>Long break</span>
    <input type="number" min="1" max="120" bind:value={longBreak} onchange={() => void save()} />
  </label>
  <label>
    <span>To long break</span>
    <input type="number" min="1" max="12" bind:value={interval} onchange={() => void save()} />
  </label>
</div>

<style>
  .durations {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 8px;
  }

  label {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  span {
    color: var(--muted-fg);
    font-size: 11px;
  }

  input {
    height: 32px;
    padding: 0 9px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--panel);
    color: var(--fg);
    font-size: 13px;
    font-variant-numeric: tabular-nums;
  }

  input:focus {
    border-color: var(--accent);
  }
</style>

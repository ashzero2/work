<script lang="ts">
  import { kindHint, kindLabel, reminders } from '$lib/stores/reminders.svelte';
</script>

<div class="toggles">
  {#each reminders.toggles as toggle (toggle.kind)}
    <label class="toggle">
      <input
        type="checkbox"
        checked={toggle.enabled}
        onchange={(event) =>
          void reminders.setEnabled(toggle.kind, event.currentTarget.checked)}
      />
      <span class="toggle-text">
        <span class="toggle-label">{kindLabel[toggle.kind]}</span>
        <span class="toggle-hint">{kindHint[toggle.kind]}</span>
      </span>
    </label>
  {/each}
</div>

<style>
  .toggles {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .toggle {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    padding: 7px 9px;
    border-radius: var(--radius-sm);
    cursor: pointer;
  }

  .toggle:hover {
    background: var(--lane);
  }

  .toggle input {
    margin-top: 2px;
    accent-color: var(--accent);
  }

  .toggle-text {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .toggle-label {
    font-size: var(--text-base);
  }

  .toggle-hint {
    color: var(--muted-fg);
    font-size: var(--text-xs);
  }
</style>

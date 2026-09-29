<script lang="ts">
  import PomodoroDurations from '../PomodoroDurations.svelte';
</script>

<section class="setting-pane">
  <h1 class="setting-title">Focus</h1>

  <div class="setting-row">
    <span class="setting-main">
      <span class="setting-label">Durations, in minutes</span>
      <span class="setting-hint">The same values the Focus screen shows.</span>
    </span>
  </div>

  <PomodoroDurations />
</section>

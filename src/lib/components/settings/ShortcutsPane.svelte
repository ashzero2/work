<script lang="ts">
  import ShortcutRecorder from '../ShortcutRecorder.svelte';
</script>

<section class="setting-pane">
  <h1 class="setting-title">Shortcuts</h1>

  <div class="setting-row">
    <span class="setting-main">
      <span class="setting-label">Quick capture</span>
      <span class="setting-hint">Opens the capture field from anywhere, even unfocused.</span>
    </span>
  </div>

  <ShortcutRecorder />
</section>

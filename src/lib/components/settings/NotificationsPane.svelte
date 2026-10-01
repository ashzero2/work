<script lang="ts">
  import { Settings as SettingsIcon } from '@lucide/svelte';

  import { reminders } from '$lib/stores/reminders.svelte';
  import NotificationKindToggles from '../NotificationKindToggles.svelte';
</script>

<section class="setting-pane">
  <h1 class="setting-title">Notifications</h1>

  {#if !reminders.available}
    <p class="setting-note">
      Reminders are delivered by macOS, which needs the packaged app rather than a dev server.
      Nothing can be shown until then.
    </p>
  {:else if reminders.authorized === false}
    <div class="setting-row">
      <p class="setting-note">
        macOS is not currently allowing notifications from Tittle, so reminders are saved
        but never shown.
      </p>
      <button class="btn" onclick={() => void reminders.openSettings()}>
        <SettingsIcon size={14} /> Open settings
      </button>
    </div>
  {/if}

  <div class="setting-row">
    <span class="setting-main">
      <span class="setting-label">Notify me about</span>
      <span class="setting-hint">Also editable on the Reminders screen.</span>
    </span>
  </div>
  <NotificationKindToggles />
</section>

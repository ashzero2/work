<script lang="ts">
  import { Check } from '@lucide/svelte';

  import { theme } from '$lib/stores/theme.svelte';
  import { availableModes, themeModeLabel } from '$lib/theme';
  import DropdownMenu from '../DropdownMenu.svelte';

  const modes = $derived(availableModes(theme.state.macos));
</script>

<section class="setting-pane">
  <h1 class="setting-title">Appearance</h1>

  <div class="setting-row">
    <span class="setting-main">
      <span class="setting-label">Theme</span>
      <span class="setting-hint">System mirrors macOS, including its accent colour.</span>
    </span>
    <DropdownMenu align="end">
      {#snippet trigger({ toggle })}
        <button class="btn" onclick={toggle}>{themeModeLabel[theme.state.mode]}</button>
      {/snippet}
      {#snippet content({ close })}
        {#each modes as mode (mode)}
          <button
            class="menu-item"
            onclick={() => {
              void theme.set(mode);
              close();
            }}
          >
            <span class="setting-tick">
              {#if mode === theme.state.mode}<Check size={14} />{/if}
            </span>
            {themeModeLabel[mode]}
          </button>
        {/each}
      {/snippet}
    </DropdownMenu>
  </div>
</section>

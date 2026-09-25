<script lang="ts">
  import { Check, ChevronDown } from '@lucide/svelte';

  import { workspace } from '$lib/stores/workspace.svelte';
  import DropdownMenu from './DropdownMenu.svelte';
</script>

<DropdownMenu align="end">
  {#snippet trigger({ toggle })}
    <button
      class="btn"
      onclick={toggle}
      disabled={workspace.columns.length === 0}
      title="New tasks are added to this column"
    >
      {workspace.targetColumnName ?? 'No column'}
      <ChevronDown size={14} />
    </button>
  {/snippet}

  {#snippet content({ close })}
    {#each workspace.columns as column (column.id)}
      <button
        class="menu-item"
        onclick={() => {
          workspace.setTargetColumn(column.id);
          close();
        }}
      >
        <span class="check">
          {#if column.id === workspace.targetColumnId}<Check size={14} />{/if}
        </span>
        {column.name}
      </button>
    {/each}
  {/snippet}
</DropdownMenu>

<style>
  .check {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 14px;
    color: var(--accent);
  }
</style>

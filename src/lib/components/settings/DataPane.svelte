<script lang="ts">
  import { Download } from '@lucide/svelte';

  import { exportTasks } from '$lib/api';
  import { toast } from '$lib/stores/toast.svelte';
  import type { ExportFormat } from '$lib/types';

  let exporting = $state<ExportFormat | null>(null);

  async function exportAs(format: ExportFormat): Promise<void> {
    exporting = format;
    try {
      const path = await exportTasks(format);
      // A cancelled dialog is not a failure, and says nothing.
      if (path !== null) toast.show(`Exported to ${path}`);
    } catch (error) {
      toast.show(String(error));
    } finally {
      exporting = null;
    }
  }
</script>

<section class="setting-pane">
  <h1 class="setting-title">Data</h1>

  <div class="setting-row">
    <span class="setting-main">
      <span class="setting-label">Export tasks</span>
      <span class="setting-hint">Every task with its column, due date and history.</span>
    </span>
    <div class="actions">
      <button class="btn" disabled={exporting !== null} onclick={() => void exportAs('json')}>
        <Download size={14} /> JSON
      </button>
      <button class="btn" disabled={exporting !== null} onclick={() => void exportAs('csv')}>
        <Download size={14} /> CSV
      </button>
    </div>
  </div>
</section>

<style>
  .actions {
    display: flex;
    flex-shrink: 0;
    gap: 8px;
  }
</style>

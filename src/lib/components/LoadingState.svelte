<script lang="ts">
  interface Props {
    label: string;
  }

  let { label }: Props = $props();
</script>

<div class="loading" role="status">
  <div class="bars">
    <div class="bar"></div>
    <div class="bar short"></div>
    <div class="bar"></div>
  </div>
  <span class="label">{label}</span>
</div>

<style>
  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    height: 100%;
    min-height: 220px;
    padding-top: 56px;
  }

  .bars {
    display: flex;
    flex-direction: column;
    gap: 6px;
    width: 200px;
  }

  .bar {
    height: 8px;
    border-radius: 4px;
    background: color-mix(in srgb, var(--muted-fg) 14%, transparent);
  }

  .bar.short {
    width: 62%;
  }

  .label {
    color: var(--muted-fg);
    font-size: 11px;
  }
</style>

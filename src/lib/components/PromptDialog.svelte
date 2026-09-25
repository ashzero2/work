<script lang="ts">
  interface Props {
    title: string;
    confirmLabel: string;
    placeholder?: string;
    initial?: string;
    onsubmit: (value: string) => void;
    oncancel: () => void;
  }

  let { title, confirmLabel, placeholder = '', initial = '', onsubmit, oncancel }: Props = $props();

  // svelte-ignore state_referenced_locally — the field is seeded once per mount
  let value = $state(initial);
  let dialog: HTMLDivElement | null = null;
  let input: HTMLInputElement | null = null;

  $effect(() => {
    input?.focus();
    input?.select();
  });

  function submit(): void {
    const trimmed = value.trim();
    if (trimmed) onsubmit(trimmed);
  }

  function onKeydown(event: KeyboardEvent): void {
    if (event.key === 'Enter') submit();
    if (event.key === 'Escape') oncancel();
  }

  function onPointerDown(event: PointerEvent): void {
    if (dialog && !dialog.contains(event.target as Node)) oncancel();
  }
</script>

<svelte:window onpointerdown={onPointerDown} onkeydown={onKeydown} />

<div class="backdrop">
  <div class="dialog" bind:this={dialog} role="dialog" aria-modal="true" aria-label={title}>
    <h2>{title}</h2>
    <input bind:this={input} bind:value={value} {placeholder} />
    <div class="actions">
      <button class="btn" onclick={oncancel}>Cancel</button>
      <button class="btn btn-primary" onclick={submit} disabled={!value.trim()}>{confirmLabel}</button>
    </div>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 60;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 18vh;
    background: color-mix(in srgb, #000 32%, transparent);
  }

  .dialog {
    display: flex;
    flex-direction: column;
    gap: 12px;
    width: 340px;
    padding: 16px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--panel);
    box-shadow: var(--shadow-md);
  }

  h2 {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
  }

  input {
    height: 34px;
    padding: 0 11px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg);
    color: var(--fg);
    font-size: 13px;
  }

  input:focus {
    border-color: var(--accent);
    outline: none;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }
</style>

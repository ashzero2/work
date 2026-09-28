<script lang="ts">
  import { onMount } from 'svelte';

  import { getQuickCaptureShortcut, setQuickCaptureShortcut } from '$lib/api';
  import { toast } from '$lib/stores/toast.svelte';
  import type { ShortcutState } from '$lib/types';

  /// Keys that only ever modify another key, so pressing one on its own is not a
  /// combination to record.
  const MODIFIER_CODES = new Set([
    'ControlLeft',
    'ControlRight',
    'ShiftLeft',
    'ShiftRight',
    'AltLeft',
    'AltRight',
    'MetaLeft',
    'MetaRight'
  ]);

  let current = $state<ShortcutState | null>(null);
  let recording = $state(false);
  let notice = $state('');

  onMount(async () => {
    try {
      current = await getQuickCaptureShortcut();
    } catch (error) {
      toast.show(String(error));
    }
  });

  /// Built from the physical key, so a binding survives a keyboard layout change —
  /// `event.key` would record whichever character that key produces.
  function combination(event: KeyboardEvent): string | null {
    if (MODIFIER_CODES.has(event.code)) return null;
    if (!event.metaKey && !event.ctrlKey && !event.altKey && !event.shiftKey) return null;

    const parts: string[] = [];
    if (event.metaKey || event.ctrlKey) parts.push('CmdOrCtrl');
    if (event.altKey) parts.push('Alt');
    if (event.shiftKey) parts.push('Shift');
    parts.push(event.code);

    return parts.join('+');
  }

  /// Apple's modifier order, then the key.
  function glyphs(shortcut: string | null): string {
    if (shortcut === null) return 'Not set';

    const parts = shortcut.split('+');
    const key = parts[parts.length - 1].replace(/^Key/, '').replace(/^Digit/, '');

    return [
      parts.includes('Control') ? '⌃' : '',
      parts.includes('Alt') ? '⌥' : '',
      parts.includes('Shift') ? '⇧' : '',
      parts.includes('CmdOrCtrl') ? '⌘' : '',
      key
    ].join('');
  }

  async function apply(shortcut: string): Promise<void> {
    try {
      current = await setQuickCaptureShortcut(shortcut);
      notice = `Set to ${glyphs(current.shortcut)}`;
    } catch (error) {
      // A refused combination leaves the previous binding in place, so say what
      // went wrong rather than pretending it took.
      notice = String(error);
    } finally {
      recording = false;
    }
  }

  async function onKeydown(event: KeyboardEvent): Promise<void> {
    if (!recording) return;

    event.preventDefault();
    event.stopPropagation();
    if (event.repeat) return;

    if (event.key === 'Escape') {
      recording = false;
      notice = 'Cancelled';
      return;
    }

    if (event.key === 'Backspace' || event.key === 'Delete') {
      if (current !== null) await apply(current.default);
      return;
    }

    const next = combination(event);
    if (next === null) {
      notice = 'Hold at least one modifier';
      return;
    }

    await apply(next);
  }
</script>

<svelte:window onkeydown={onKeydown} />

<div class="recorder">
  <button
    class="btn"
    class:recording
    aria-keyshortcuts={current?.shortcut ?? undefined}
    aria-describedby="shortcut-hint"
    onclick={() => {
      recording = true;
      notice = 'Press the combination you want';
    }}
  >
    {recording ? 'Press keys…' : glyphs(current?.shortcut ?? null)}
  </button>

  <p id="shortcut-hint" class="hint">
    At least one modifier. Esc cancels, Backspace resets to
    {glyphs(current?.default ?? null)}.
  </p>

  <p class="status" role="status">{notice}</p>

  {#if current !== null && current.shortcut !== current.preferred}
    <p class="hint warn">
      Saved as {glyphs(current.preferred)}, but {current.shortcut === null
        ? 'nothing'
        : glyphs(current.shortcut)} is active — the saved combination couldn't be registered, which
      usually means another app already has it.
    </p>
  {/if}
</div>

<style>
  .recorder {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 6px;
  }

  .btn.recording {
    border-color: var(--accent);
    color: var(--accent);
  }

  .hint {
    max-width: 320px;
    margin: 0;
    color: var(--muted-fg);
    font-size: 11px;
    text-align: right;
  }

  .hint.warn {
    color: var(--warning);
  }

  .status {
    max-width: 320px;
    margin: 0;
    color: var(--muted-fg);
    font-size: 11px;
    text-align: right;
  }
</style>

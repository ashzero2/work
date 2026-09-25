import { deriveAccent, type Appearance } from './accent';
import { getSetting, setSetting, systemAccent } from './api';
import { toast } from './stores/toast.svelte';

export type ThemeMode = 'light' | 'dark' | 'macos';

export interface ThemeState {
  mode: ThemeMode;
  appearance: Appearance;
  macos: boolean;
}

/// The choice lives in the app's own SQLite settings rather than webview
/// `localStorage`: that storage is scoped per origin, so the dev server and the
/// packaged app would each keep a separate answer, and it can't be inspected or
/// recovered the way a row in `settings` can.
const THEME_KEY = 'theme';

/// Read from the webview rather than over IPC on purpose: the window is
/// undecorated on macOS, so the layout has to know that *before* any async call
/// can fail to land, or the sidebar collides with the traffic lights.
const detectMacos = (): boolean => /Mac OS X|Macintosh/.test(navigator.userAgent);

const prefersDark = (): boolean =>
  window.matchMedia('(prefers-color-scheme: dark)').matches;

const appearanceOf = (): Appearance => (prefersDark() ? 'dark' : 'light');

let accentRequest: Promise<string | null> | null = null;

/// The system accent is a bonus, not a dependency: if it can't be read the
/// theme falls back to its own accent rather than losing the platform theme.
function accent(): Promise<string | null> {
  if (accentRequest === null) {
    accentRequest = detectMacos()
      ? systemAccent().catch(() => null)
      : Promise.resolve(null);
  }
  return accentRequest;
}

async function storedMode(): Promise<string | null> {
  try {
    return await getSetting(THEME_KEY);
  } catch {
    return null;
  }
}

function apply(mode: ThemeMode, systemAccentHex: string | null): void {
  const root = document.documentElement;

  if (systemAccentHex === null) {
    root.style.removeProperty('--accent');
    root.style.removeProperty('--accent-fg');
  } else {
    const tokens = deriveAccent(systemAccentHex, appearanceOf());
    root.style.setProperty('--accent', tokens.accent);
    root.style.setProperty('--accent-fg', tokens.accentFg);
  }

  root.dataset.theme = mode;
}

function resolve(stored: string | null, macos: boolean): ThemeMode {
  const fallback: ThemeMode = prefersDark() ? 'dark' : 'light';
  if (stored === 'light' || stored === 'dark' || stored === 'macos') {
    return stored === 'macos' && !macos ? fallback : stored;
  }
  return fallback;
}

/// Runs before first paint. An unset preference leaves `data-theme` off so the
/// OS appearance wins through CSS rather than being forced.
export async function initTheme(): Promise<ThemeState> {
  const macos = detectMacos();
  const stored = await storedMode();
  const mode = resolve(stored, macos);
  const systemAccentHex = macos ? await accent() : null;

  if (stored === null && mode !== 'macos') {
    document.documentElement.removeAttribute('data-theme');
  } else {
    apply(mode, systemAccentHex);
  }

  // The accent is derived per appearance, so it is re-derived when the system
  // flips rather than reused across the change.
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
    if (mode === 'macos' && systemAccentHex !== null) apply('macos', systemAccentHex);
  });

  return { mode, appearance: appearanceOf(), macos };
}

export async function setTheme(next: ThemeMode): Promise<ThemeState> {
  const macos = detectMacos();
  const mode = resolve(next, macos);
  const systemAccentHex = macos ? await accent() : null;

  apply(mode, systemAccentHex);

  try {
    await setSetting(THEME_KEY, mode);
  } catch (error) {
    toast.show(`Couldn't save the theme: ${String(error)}`);
  }

  return { mode, appearance: appearanceOf(), macos };
}

export const themeModeLabel: Record<ThemeMode, string> = {
  light: 'Light',
  dark: 'Dark',
  macos: 'System (macOS)'
};

export const availableModes = (macos: boolean): ThemeMode[] =>
  macos ? ['light', 'dark', 'macos'] : ['light', 'dark'];

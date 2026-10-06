import { deriveAccent, type Appearance } from './accent';
import { getSetting, setSetting, systemAccent } from './api';
import { toast } from './stores/toast.svelte';

export type ThemeMode = 'light' | 'dark' | 'macos';
export type PaletteMode = 'warm' | 'cool';

export interface ThemeState {
  mode: ThemeMode;
  palette: PaletteMode;
  appearance: Appearance;
  macos: boolean;
}

/// The choices live in the app's own SQLite settings rather than webview
/// `localStorage`: that storage is scoped per origin, so the dev server and the
/// packaged app would each keep a separate answer, and it can't be inspected or
/// recovered the way a row in `settings` can.
const THEME_KEY = 'theme';
const PALETTE_KEY = 'palette';

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

async function storedSetting(key: string): Promise<string | null> {
  try {
    return await getSetting(key);
  } catch {
    return null;
  }
}

function resolveMode(stored: string | null, macos: boolean): ThemeMode {
  const fallback: ThemeMode = prefersDark() ? 'dark' : 'light';
  if (stored === 'light' || stored === 'dark' || stored === 'macos') {
    return stored === 'macos' && !macos ? fallback : stored;
  }
  return fallback;
}

function resolvePalette(stored: string | null): PaletteMode {
  return stored === 'cool' ? 'cool' : 'warm';
}

/// The last resolved state, so a palette change can re-apply the active theme
/// (and vice versa) without re-reading storage.
let current: ThemeState | null = null;

function remember(state: ThemeState): ThemeState {
  current = state;
  return state;
}

/// An unset preference leaves `data-theme` off so the OS appearance wins through
/// CSS rather than being forced; warm is likewise the attribute-less default.
function reflect(mode: ThemeMode | null, palette: PaletteMode, systemAccentHex: string | null): void {
  const root = document.documentElement;

  if (systemAccentHex === null) {
    root.style.removeProperty('--accent');
    root.style.removeProperty('--accent-fg');
  } else {
    const tokens = deriveAccent(systemAccentHex, appearanceOf());
    root.style.setProperty('--accent', tokens.accent);
    root.style.setProperty('--accent-fg', tokens.accentFg);
  }

  if (mode === null) root.removeAttribute('data-theme');
  else root.dataset.theme = mode;

  if (palette === 'warm') root.removeAttribute('data-palette');
  else root.dataset.palette = palette;
}

/// Runs before first paint.
export async function initTheme(): Promise<ThemeState> {
  const macos = detectMacos();
  const [storedTheme, storedPalette] = await Promise.all([
    storedSetting(THEME_KEY),
    storedSetting(PALETTE_KEY)
  ]);
  const mode = resolveMode(storedTheme, macos);
  const palette = resolvePalette(storedPalette);
  const systemAccentHex = macos ? await accent() : null;

  if (storedTheme === null) reflect(null, palette, null);
  else reflect(mode, palette, mode === 'macos' ? systemAccentHex : null);

  // The accent is derived per appearance, so it is re-derived when the system
  // flips rather than reused across the change.
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
    if (mode === 'macos' && systemAccentHex !== null) {
      reflect(mode, palette, systemAccentHex);
    }
  });

  return remember({ mode, palette, appearance: appearanceOf(), macos });
}

export async function setTheme(next: ThemeMode): Promise<ThemeState> {
  const macos = detectMacos();
  const mode = resolveMode(next, macos);
  const palette = current?.palette ?? resolvePalette(await storedSetting(PALETTE_KEY));
  const systemAccentHex = macos && mode === 'macos' ? await accent() : null;

  reflect(mode, palette, systemAccentHex);

  try {
    await setSetting(THEME_KEY, mode);
  } catch (error) {
    toast.show(`Couldn't save the theme: ${String(error)}`);
  }

  return remember({ mode, palette, appearance: appearanceOf(), macos });
}

export async function setPalette(next: PaletteMode): Promise<ThemeState> {
  const macos = detectMacos();
  const palette = resolvePalette(next);
  const mode = current?.mode ?? resolveMode(await storedSetting(THEME_KEY), macos);

  reflect(mode, palette, null);

  try {
    await setSetting(PALETTE_KEY, palette);
  } catch (error) {
    toast.show(`Couldn't save the palette: ${String(error)}`);
  }

  return remember({ mode, palette, appearance: appearanceOf(), macos });
}

export const themeModeLabel: Record<ThemeMode, string> = {
  light: 'Light',
  dark: 'Dark',
  macos: 'System (macOS)'
};

export const availableModes = (macos: boolean): ThemeMode[] =>
  macos ? ['light', 'dark', 'macos'] : ['light', 'dark'];

export const paletteLabel: Record<PaletteMode, string> = {
  warm: 'Warm',
  cool: 'Cool'
};

export const availablePalettes: PaletteMode[] = ['warm', 'cool'];

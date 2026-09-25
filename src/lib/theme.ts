export type ThemeMode = 'light' | 'dark';

const STORAGE_KEY = 'work-dashboard.theme';

const prefersDark = (): boolean =>
  window.matchMedia('(prefers-color-scheme: dark)').matches;

function applyMode(mode: ThemeMode): void {
  document.documentElement.dataset.theme = mode;
}

/// Runs before first paint. When the user has never chosen a mode, the
/// `data-theme` attribute is left off so the OS preference wins via CSS.
export function initTheme(): ThemeMode {
  const stored = localStorage.getItem(STORAGE_KEY);
  if (stored === 'light' || stored === 'dark') {
    applyMode(stored);
    return stored;
  }
  document.documentElement.removeAttribute('data-theme');
  return prefersDark() ? 'dark' : 'light';
}

export function toggleMode(current: ThemeMode): ThemeMode {
  const next: ThemeMode = current === 'dark' ? 'light' : 'dark';
  localStorage.setItem(STORAGE_KEY, next);
  applyMode(next);
  return next;
}

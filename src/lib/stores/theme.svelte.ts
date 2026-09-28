import { initTheme, setTheme, type ThemeMode, type ThemeState } from '$lib/theme';

/// The theme, as a store rather than a prop: the sidebar needs to know whether it
/// is the macOS theme, and Settings is where it is changed.
class ThemeStore {
  state = $state<ThemeState>({ mode: 'light', appearance: 'light', macos: false });

  async load(): Promise<void> {
    this.state = await initTheme();
  }

  async set(mode: ThemeMode): Promise<void> {
    this.state = await setTheme(mode);
  }
}

export const theme = new ThemeStore();

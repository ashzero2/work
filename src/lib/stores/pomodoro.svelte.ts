import * as api from '$lib/api';
import { toast } from '$lib/stores/toast.svelte';
import type { PomodoroSession, PomodoroSettings, SessionKind } from '$lib/types';

const RECENT_LIMIT = 6;
const TICK_MS = 1000;

const DEFAULT_SETTINGS: PomodoroSettings = {
  workSeconds: 25 * 60,
  breakSeconds: 5 * 60,
  longBreakSeconds: 15 * 60,
  sessionsPerLongBreak: 4
};

export const phaseLabel: Record<SessionKind, string> = {
  work: 'Work',
  break: 'Break',
  long_break: 'Long break'
};

export function formatClock(totalSeconds: number): string {
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = Math.floor(totalSeconds % 60);
  return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
}

/// The timer. Time remaining is always derived from the session's start time
/// rather than decremented from a counter, so sleeping, a throttled interval, or
/// a backgrounded window cannot make the count drift.
class PomodoroStore {
  settings = $state<PomodoroSettings>({ ...DEFAULT_SETTINGS });
  session = $state<PomodoroSession | null>(null);
  recent = $state<PomodoroSession[]>([]);
  cycle = $state(0);
  nextKind = $state<SessionKind>('work');
  taskId = $state<number | null>(null);
  loaded = $state(false);

  private now = $state(Date.now());
  private ticker: number | undefined;
  private advancing = false;

  get running(): boolean {
    return this.session !== null;
  }

  get kind(): SessionKind {
    return this.session?.kind ?? this.nextKind;
  }

  get remainingSeconds(): number {
    const session = this.session;
    if (session === null) return 0;
    const elapsed = (this.now - Date.parse(session.startedAt)) / 1000;
    return Math.max(0, Math.ceil(session.plannedSeconds - elapsed));
  }

  get progress(): number {
    const session = this.session;
    if (session === null || session.plannedSeconds <= 0) return 0;
    const elapsed = (this.now - Date.parse(session.startedAt)) / 1000;
    return Math.min(1, Math.max(0, elapsed / session.plannedSeconds));
  }

  /// Idle, this previews the phase that Start would begin rather than sitting at
  /// zero.
  get clock(): string {
    if (this.session === null) return formatClock(this.secondsFor(this.nextKind));
    return formatClock(this.remainingSeconds);
  }

  async load(): Promise<void> {
    try {
      this.settings = await api.getPomodoroSettings();

      const resumed = await api.reconcileSession();
      if (resumed !== null) {
        this.session = resumed;
        this.taskId = resumed.taskId;
        this.startTicking();
      }

      await this.refreshHistory();
    } catch (error) {
      toast.show(String(error));
    } finally {
      this.loaded = true;
    }
  }

  async start(): Promise<void> {
    if (this.session !== null) return;
    await this.begin(await api.nextPhase());
  }

  async skip(): Promise<void> {
    const session = this.session;
    if (session === null) return;
    await this.advance(session.id, false);
  }

  async stop(): Promise<void> {
    const session = this.session;
    if (session === null) return;

    this.session = null;
    this.stopTicking();

    try {
      await api.finishSession(session.id, false);
      await this.refreshHistory();
    } catch (error) {
      toast.show(String(error));
    }
  }

  /// A session's task is fixed when it starts, so this only affects the next one.
  setTask(taskId: number | null): void {
    if (this.session !== null) return;
    this.taskId = taskId;
  }

  async saveSettings(next: PomodoroSettings): Promise<void> {
    try {
      this.settings = await api.setPomodoroSettings(next);
    } catch (error) {
      toast.show(String(error));
    }
  }

  private async begin(kind: SessionKind): Promise<void> {
    try {
      const session = await api.startSession(kind, this.taskId, this.secondsFor(kind));
      this.session = session;
      this.now = Date.now();
      this.startTicking();
      await this.refreshHistory();
    } catch (error) {
      toast.show(String(error));
    }
  }

  /// Ends the current session and starts the next phase. Guarded so waking from
  /// sleep, or a double tick, can never advance twice.
  private async advance(id: number, completed: boolean): Promise<void> {
    if (this.advancing) return;
    this.advancing = true;

    try {
      await api.finishSession(id, completed);
      await this.begin(await api.nextPhase());
    } catch (error) {
      toast.show(String(error));
    } finally {
      this.advancing = false;
    }
  }

  private secondsFor(kind: SessionKind): number {
    if (kind === 'break') return this.settings.breakSeconds;
    if (kind === 'long_break') return this.settings.longBreakSeconds;
    return this.settings.workSeconds;
  }

  private async refreshHistory(): Promise<void> {
    [this.recent, this.cycle, this.nextKind] = await Promise.all([
      api.recentSessions(RECENT_LIMIT),
      api.cyclePosition(),
      api.nextPhase()
    ]);
  }

  private startTicking(): void {
    if (this.ticker !== undefined) return;
    this.ticker = window.setInterval(() => this.tick(), TICK_MS);
  }

  private stopTicking(): void {
    if (this.ticker === undefined) return;
    window.clearInterval(this.ticker);
    this.ticker = undefined;
  }

  private tick(): void {
    this.now = Date.now();
    if (this.session !== null && this.remainingSeconds <= 0) {
      void this.advance(this.session.id, true);
    }
  }
}

export const pomodoro = new PomodoroStore();

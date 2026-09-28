import * as api from '$lib/api';
import { toast } from '$lib/stores/toast.svelte';
import type { KindToggle, Reminder, ReminderKind, ReminderStatus } from '$lib/types';

export const kindLabel: Record<ReminderKind, string> = {
  task_due: 'Task due',
  priority_alert: 'Priority task',
  pomodoro_break: 'Pomodoro break',
  custom: 'Custom'
};

export const kindHint: Record<ReminderKind, string> = {
  task_due: 'When a task’s due date arrives',
  priority_alert: 'When something high-priority is still open',
  pomodoro_break: 'When a focus session ends',
  custom: 'Anything you schedule yourself'
};

export const statusLabel: Record<ReminderStatus, string> = {
  pending: 'Scheduled',
  fired: 'Seen',
  snoozed: 'Snoozed',
  dismissed: 'Dismissed'
};

export function isOutstanding(reminder: Reminder): boolean {
  return reminder.status === 'pending' || reminder.status === 'snoozed';
}

class RemindersStore {
  reminders = $state<Reminder[]>([]);
  available = $state(true);
  authorized = $state<boolean | null>(null);
  toggles = $state<KindToggle[]>([]);
  loaded = $state(false);

  get outstanding(): Reminder[] {
    return this.reminders.filter(isOutstanding);
  }

  get settled(): Reminder[] {
    return this.reminders.filter((reminder) => !isOutstanding(reminder));
  }

  enabledFor(kind: ReminderKind): boolean {
    return this.toggles.find((toggle) => toggle.kind === kind)?.enabled ?? true;
  }

  async load(): Promise<void> {
    try {
      await this.refresh();
    } catch (error) {
      toast.show(String(error));
    } finally {
      this.loaded = true;
    }
  }

  async create(kind: ReminderKind, taskId: number | null, triggerAt: string): Promise<void> {
    await this.mutate(() => api.createReminder(kind, taskId, triggerAt));
  }

  async reschedule(id: number, triggerAt: string): Promise<void> {
    await this.mutate(() => api.rescheduleReminder(id, triggerAt));
  }

  async snooze(id: number): Promise<void> {
    await this.mutate(() => api.snoozeReminder(id));
  }

  async dismiss(id: number): Promise<void> {
    await this.mutate(() => api.dismissReminder(id));
  }

  async remove(id: number): Promise<void> {
    await this.mutate(() => api.deleteReminder(id));
  }

  async setEnabled(kind: ReminderKind, enabled: boolean): Promise<void> {
    await this.mutate(() => api.setKindEnabled(kind, enabled));
  }

  async openSettings(): Promise<void> {
    try {
      await api.openNotificationSettings();
    } catch (error) {
      toast.show(String(error));
    }
  }

  /// Every edit changes which reminders the OS has registered, so the list is
  /// re-read rather than patched in place.
  private async mutate(action: () => Promise<unknown>): Promise<void> {
    try {
      await action();
    } catch (error) {
      toast.show(String(error));
    }
    try {
      await this.refresh();
    } catch (error) {
      toast.show(String(error));
    }
  }

  private async refresh(): Promise<void> {
    const [list, state] = await Promise.all([api.listReminders(), api.notificationState()]);
    this.reminders = list;
    this.available = state.available;
    this.authorized = state.authorized;
    this.toggles = state.enabled;
  }
}

export const reminders = new RemindersStore();

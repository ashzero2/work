export type ChipTone = 'muted' | 'info' | 'warning' | 'danger';

export interface DueChip {
  label: string;
  tone: ChipTone;
}

const startOfDay = (date: Date): number =>
  new Date(date.getFullYear(), date.getMonth(), date.getDate()).getTime();

/// Mirrors the gpui due chip: today/tomorrow by name, overdue by day count,
/// otherwise a short calendar date.
export function dueChip(iso: string, now: Date = new Date()): DueChip {
  const due = new Date(iso);
  const days = Math.round((startOfDay(due) - startOfDay(now)) / 86_400_000);

  if (days === 0) return { label: 'Today', tone: 'warning' };
  if (days === 1) return { label: 'Tomorrow', tone: 'muted' };
  if (days < 0) return { label: `${-days}d overdue`, tone: 'danger' };
  return {
    label: due.toLocaleDateString(undefined, { month: 'short', day: 'numeric' }),
    tone: 'muted'
  };
}

export const priorityLabel: Record<string, string> = {
  low: 'Low',
  medium: 'Medium',
  high: 'High'
};

export const repeatLabel: Record<string, string> = {
  daily: 'Daily',
  weekly: 'Weekly',
  monthly: 'Monthly'
};

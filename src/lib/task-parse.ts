import * as chrono from 'chrono-node';

import type { Priority } from './types';

export interface ParsedTask {
  /// The input with every recognised fragment removed — what the user actually wants to do.
  title: string;
  /// Local `YYYY-MM-DDTHH:mm`, the shape `DateTimeField` binds to; `''` when no date was found.
  due: string;
  /// True when a date was found with no clock time, so it should read as a day rather than an instant.
  allDay: boolean;
  priority: Priority;
  /// The exact fragments that were consumed, so a confirmation UI can echo them back.
  matched: string[];
}

/// A date with no clock time still needs a time component to fit `DateTimeField`, which defaults
/// a picked date to 09:00 — so a parsed all-day task lands on the same convention.
const ALL_DAY_HOUR = 9;

const SIGIL_PRIORITY: Record<number, Priority> = { 1: 'low', 2: 'medium', 3: 'high' };

function pad(value: number): string {
  return String(value).padStart(2, '0');
}

function localDateTime(date: Date, hour: number, minute: number): string {
  const day = `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`;
  return `${day}T${pad(hour)}:${pad(minute)}`;
}

function collapse(text: string): string {
  return text.replace(/\s+/g, ' ').trim();
}

/// Only a whitespace-delimited run of `!` counts, so a title like `wow!!!` keeps its punctuation
/// instead of silently losing it and gaining a priority.
function takePriority(input: string): { priority: Priority; text: string; rest: string } {
  const tokens = input.split(/\s+/);
  const index = tokens.findIndex((token) => /^!+$/.test(token));
  if (index === -1) return { priority: 'none', text: '', rest: input };

  const token = tokens[index];
  tokens.splice(index, 1);
  return {
    priority: SIGIL_PRIORITY[Math.min(token.length, 3)],
    text: token,
    rest: tokens.join(' ')
  };
}

/// Reads a date, time, and priority out of one line of free text.
///
/// Parsing is best-effort and biased forward, so `friday` means the coming Friday. Bare times such
/// as `at 5` are genuinely ambiguous and will resolve to a time today; callers must show the result
/// for confirmation rather than committing it silently.
export function parseTaskInput(input: string, now: Date = new Date()): ParsedTask {
  const { priority, text: sigil, rest } = takePriority(input);
  const matched = sigil ? [sigil] : [];

  const result = chrono.casual.parse(rest, now, { forwardDate: true })[0];
  if (!result) {
    return { title: collapse(rest), due: '', allDay: false, priority, matched };
  }

  const date = result.start.date();
  const hasClockTime = result.start.isCertain('hour');
  const due = hasClockTime
    ? localDateTime(date, date.getHours(), date.getMinutes())
    : localDateTime(date, ALL_DAY_HOUR, 0);

  const remainder = `${rest.slice(0, result.index)} ${rest.slice(result.index + result.text.length)}`;
  matched.push(result.text);

  return {
    title: collapse(remainder) || collapse(rest),
    due,
    allDay: !hasClockTime,
    priority,
    matched
  };
}

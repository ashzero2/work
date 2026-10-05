import { describe, expect, it } from 'vitest';

import { parseTaskInput, stripFragments } from './task-parse';

// A fixed Saturday, so `friday` means the coming Friday and every expectation is stable.
const NOW = new Date(2026, 9, 3, 10, 0, 0);

const parse = (input: string) => parseTaskInput(input, NOW);

describe('date and time', () => {
  it('parses the flagship case — relative day, dot-separated time, priority sigil', () => {
    const result = parse('Meet with mentor tomorrow at 09.00pm !!!');

    expect(result.title).toBe('Meet with mentor');
    expect(result.due).toBe('2026-10-04T21:00');
    expect(result.allDay).toBe(false);
    expect(result.priority).toBe('high');
  });

  it('resolves a weekday forward to the coming one', () => {
    const result = parse('Call dentist on friday at 3pm');

    expect(result.title).toBe('Call dentist');
    expect(result.due).toBe('2026-10-09T15:00');
    expect(result.allDay).toBe(false);
  });

  it('reads a bare time as today', () => {
    const result = parse('Lunch at 12.30');

    expect(result.title).toBe('Lunch');
    expect(result.due).toBe('2026-10-03T12:30');
    expect(result.allDay).toBe(false);
  });

  it('treats a date with no clock time as all-day, not noon', () => {
    const result = parse('Submit report next monday');

    expect(result.title).toBe('Submit report');
    expect(result.allDay).toBe(true);
    expect(result.due).toBe('2026-10-05T09:00');
  });
});

describe('priority sigils', () => {
  it.each([
    ['Fix the bug !', 'low'],
    ['Fix the bug !!', 'medium'],
    ['Fix the bug !!!', 'high'],
    ['Fix the bug', 'none']
  ])('maps %j to %s', (input, expected) => {
    expect(parse(input).priority).toBe(expected);
  });

  it('caps a longer run at high rather than failing', () => {
    expect(parse('Fix the bug !!!!!').priority).toBe('high');
  });

  it('strips the sigil from the title but keeps other punctuation', () => {
    const result = parse('Fix the bug !!!');

    expect(result.title).toBe('Fix the bug');
    expect(result.priorityText).toBe('!!!');
  });

  it('ignores exclamation marks attached to a word', () => {
    const result = parse('Deploy wow!!!');

    expect(result.priority).toBe('none');
    expect(result.title).toBe('Deploy wow!!!');
  });
});

describe('text that must not be treated as a date', () => {
  it.each(['Buy milk', 'Review Q4 numbers', 'Buy 2 apples', 'Update the 3 amigos doc'])(
    'leaves %j alone',
    (input) => {
      const result = parse(input);

      expect(result.due).toBe('');
      expect(result.title).toBe(input);
      expect(result.dateText).toBeNull();
      expect(result.priorityText).toBeNull();
    }
  );
});

describe('known false positives — accepted, and why the UI must confirm', () => {
  // These are indistinguishable from real dates by any confidence signal chrono exposes, so they
  // are pinned here as documented behaviour rather than bugs. A confirmation UI is the mitigation.
  //
  // Note the resolved time: a bare `at 5` with forwardDate rolls to 05:00 the next day, not 17:00
  // today, because 5am has already passed. That is a genuinely surprising value to commit silently.
  it.each([
    ['Fix bug in v1.2 at 5 files', '2026-10-04T05:00'],
    ['Meet at 5', '2026-10-04T05:00']
  ])('resolves the bare time in %j to the next 05:00, which the user must be able to undo', (input, due) => {
    expect(parse(input).due).toBe(due);
  });

  it('has no confidence signal separating a false positive from a real time', () => {
    expect(parse('Fix bug in v1.2 at 5 files').due).toBe(parse('Lunch at 5').due);
  });
});

describe('recurrence is deliberately out of scope', () => {
  it('does not invent a repeat rule, and leaves the phrase in the title', () => {
    const result = parse('Standup every day at 9am');

    expect(result.title).toBe('Standup every day');
    expect(result.due).toBe('2026-10-04T09:00');
    expect(result).not.toHaveProperty('repeatRule');
  });
});

describe('title handling', () => {
  it('collapses the gap left by a removed fragment', () => {
    expect(parse('Prepare tuesday deck').title).toBe('Prepare deck');
  });

  it('falls back to the raw text when parsing would leave no title at all', () => {
    expect(parse('tomorrow at 9pm').title).toBe('tomorrow at 9pm');
  });

  it('tolerates surrounding whitespace', () => {
    expect(parse('   Call dentist on friday at 3pm   ').title).toBe('Call dentist');
  });
});

describe('selective stripping — what a removed chip leaves behind', () => {
  const input = 'Meet with mentor tomorrow at 09.00pm !!!';
  const result = parse(input);

  it('removes both fragments when both chips are kept', () => {
    expect(stripFragments(input, [result.dateText, result.priorityText])).toBe('Meet with mentor');
  });

  it('keeps a rejected sigil in the title', () => {
    expect(stripFragments(input, [result.dateText, null])).toBe('Meet with mentor !!!');
  });

  it('keeps a rejected date phrase in the title', () => {
    expect(stripFragments(input, [null, result.priorityText])).toBe(
      'Meet with mentor tomorrow at 09.00pm'
    );
  });

  it('keeps everything when both chips are removed', () => {
    expect(stripFragments(input, [null, null])).toBe(input);
  });
});

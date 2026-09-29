/// A subsequence match, scored so the tightest and earliest match wins: typing
/// "rnd" finds "Renew the domain" ahead of something that merely contains the
/// letters spread out.
///
/// Returns null when the query does not match at all, so callers can filter with
/// a single call.
export function score(query: string, text: string): number | null {
  if (query.length === 0) return 0;

  const haystack = text.toLowerCase();
  const needle = query.toLowerCase();

  let score = 0;
  let index = -1;
  let previous = -1;

  for (const character of needle) {
    const found = haystack.indexOf(character, index + 1);
    if (found === -1) return null;

    // Adjacent characters are worth much more than scattered ones.
    score += found === previous + 1 ? 6 : 1;
    // And matching early in the string is worth more than matching late.
    score -= Math.min(found, 12) * 0.1;

    previous = found;
    index = found;
  }

  // A match on a word boundary reads as intentional rather than incidental.
  if (index === haystack.indexOf(needle)) score += 8;

  return score;
}

/// The best score across several fields, so a note can match on its title or on
/// one of its tags.
export function bestScore(query: string, fields: string[]): number | null {
  let best: number | null = null;
  for (const field of fields) {
    const candidate = score(query, field);
    if (candidate !== null && (best === null || candidate > best)) best = candidate;
  }
  return best;
}

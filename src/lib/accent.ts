import { contrastRatio, hexToRgb, oklchToRgb, rgbToHex, srgbToOklch, type Rgb } from './color';

export interface AccentTokens {
  accent: string;
  accentFg: string;
}

export type Appearance = 'light' | 'dark';

const MIN_CONTRAST = 4.6;
const MAX_CHROMA = 0.16;
const MAX_SEARCH_STEPS = 160;
const STEP = 0.005;

const LIGHT_SURFACE = '#ffffff';
const DARK_SURFACE = '#1b1917';
const LIGHT_INK = '#ffffff';
const DARK_INK = '#0b0d12';

/// A system accent can be any colour the user picked, including ones that are
/// unreadable as a fill or as text (graphite is ~3:1 against white; yellow is
/// worse). This keeps the source hue and searches for the nearest lightness that
/// actually passes, instead of trusting the colour the OS handed us.
export function deriveAccent(systemHex: string, appearance: Appearance): AccentTokens {
  const source = srgbToOklch(hexToRgb(systemHex));
  const chroma = Math.min(source.c, MAX_CHROMA);
  const surface = hexToRgb(appearance === 'light' ? LIGHT_SURFACE : DARK_SURFACE);
  const ink = hexToRgb(appearance === 'light' ? LIGHT_INK : DARK_INK);

  const start =
    appearance === 'light' ? Math.min(source.l, 0.6) : Math.max(source.l, 0.45);
  const step = appearance === 'light' ? -STEP : STEP;

  let result = oklchToRgb({ l: start, c: chroma, h: source.h });
  for (let index = 1; index <= MAX_SEARCH_STEPS; index += 1) {
    const lightness = start + step * index;
    if (lightness < 0.2 || lightness > 0.95) break;

    const candidate = oklchToRgb({ l: lightness, c: chroma, h: source.h });
    result = candidate;

    if (passes(candidate, surface, ink)) break;
  }

  return { accent: rgbToHex(result), accentFg: rgbToHex(ink) };
}

function passes(candidate: Rgb, surface: Rgb, ink: Rgb): boolean {
  return (
    contrastRatio(candidate, surface) >= MIN_CONTRAST &&
    contrastRatio(ink, candidate) >= MIN_CONTRAST
  );
}

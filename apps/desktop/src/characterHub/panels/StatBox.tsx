import type { CSSProperties, ReactNode } from 'react';

/**
 * Shared stat-panel building blocks for the character sheet and the Create screen (Brief 2:
 * extracted from `CharacterSheet.tsx` so Create can reuse the sheet's boxes instead of copies).
 */

/** Signed modifier: `+3`, `0` reads `+0`, `-1`. */
export function fmt(value: number): string {
  return value >= 0 ? `+${value}` : `${value}`;
}

/**
 * Renders a cell the engine did not compute as an honest em dash rather than a
 * fabricated number — the convention `deriveRaceTraits` already established for
 * an unknown race's size and vision.
 *
 * SD-27 `decisions.md §28`: touch AC, flat-footed AC, CMB and CMD used to be
 * computed in the view, so there was no such thing as "the engine did not
 * compute it" — the sheet always produced a number, and for every Small
 * character (touch/CMB/CMD) or every character holding a dodge bonus
 * (flat-footed) it produced the wrong one. They are engine values now, so
 * their absence is a real state and must read as absent.
 */
export function fmtOrAbsent(value: number | null, signed: boolean): string {
  if (value === null) {
    return '—';
  }
  return signed ? fmt(value) : `${value}`;
}

/** The bordered surface every sheet panel sits on. */
export const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

/** A titled stat panel. */
export function StatBox(props: { title?: string; children: ReactNode }) {
  return (
    <div style={{ ...panel, padding: '0.6rem 0.75rem' }}>
      {props.title ? (
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.66rem', letterSpacing: '0.06em', margin: '0 0 0.45rem', textTransform: 'uppercase' }}>
          {props.title}
        </p>
      ) : null}
      {props.children}
    </div>
  );
}

/** A boxed value with a caption, e.g. AC / Touch / CMB. */
export function StatTile(props: { label: string; value: ReactNode; emphasize?: boolean }) {
  return (
    <div style={{ ...panel, backgroundColor: 'var(--color-surface-2)', flex: 1, minWidth: 0, padding: '0.35rem 0.3rem', textAlign: 'center' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.56rem', fontWeight: 700, letterSpacing: '0.03em', margin: 0, textTransform: 'uppercase' }}>
        {props.label}
      </p>
      <p style={{ fontSize: props.emphasize ? '1.3rem' : '1rem', fontWeight: 800, margin: '0.1rem 0 0' }}>{props.value}</p>
    </div>
  );
}

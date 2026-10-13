import type { CSSProperties } from 'react';
import type { AbilityScoresDto } from '../../boundary/loadCreateCharacter';
import { printedAbilityScore } from '../abilityScoresModel';
import { SHEET_LAYOUT, layoutBox } from '../sheetLayout';
import { StatBox, fmt, panel } from './StatBox';

const ABILITY_COLUMNS: ReadonlyArray<{ key: keyof AbilityScoresDto; label: string }> = [
  { key: 'strength', label: 'STR' },
  { key: 'dexterity', label: 'DEX' },
  { key: 'constitution', label: 'CON' },
  { key: 'intelligence', label: 'INT' },
  { key: 'wisdom', label: 'WIS' },
  { key: 'charisma', label: 'CHA' },
];

/**
 * `scores` is the engine's effective score (`printedAbilityScore`); `abilities` its modifiers.
 * SD-36 F7a: the score used to be reconstructed as `10 + 2 x modifier`, one low for every odd score.
 *
 * Brief 2 L1: one vertical column, STR to CHA top to bottom, each row label | score | modifier
 * (was a six-across grid). `boxStyle` defaults to the sheet registry's.
 */
export function AbilitiesPanel(props: { scores: AbilityScoresDto | null; abilities: AbilityScoresDto; boxStyle?: CSSProperties }) {
  return (
    <StatBox title="Abilities">
      <div style={props.boxStyle ?? layoutBox(SHEET_LAYOUT, 'abilities').boxStyle}>
        {ABILITY_COLUMNS.map((col) => (
          <div key={col.key} style={{ ...panel, alignItems: 'baseline', backgroundColor: 'var(--color-surface-2)', display: 'grid', gap: '0.4rem', gridTemplateColumns: '2.4rem 2.2rem 2.2rem', padding: '0.25rem 0.45rem' }}>
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.66rem', fontWeight: 700, margin: 0 }}>{col.label}</p>
            <p style={{ fontSize: '1.05rem', fontWeight: 800, margin: 0, textAlign: 'right' }}>{printedAbilityScore(props.scores, col.key)}</p>
            <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem', margin: 0, textAlign: 'right' }}>{fmt(props.abilities[col.key])}</p>
          </div>
        ))}
      </div>
    </StatBox>
  );
}

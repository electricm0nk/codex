import { useState, type ReactNode } from 'react';

/**
 * The sheet's Abilities tab (SD-43 E3.1; operator ruling Q-22.3, 2026-11-22: the Feats tab is renamed
 * Abilities and Feats becomes one of its sub-tabs). The seven sub-tabs are PCGen's, in PCGen's order
 * (technical-design.md §3.3). Each panel is rendered by the sheet; this component owns only the bar.
 */
export const ABILITY_SUB_TABS = ['Feats', 'Racial Abilities', 'Traits', 'Archetype', 'Background', 'Class Features', 'Misc'] as const;
export type AbilitySubTabName = (typeof ABILITY_SUB_TABS)[number];

/** The card that brings each sub-tab's engine budget, picks and prerequisite checks (epic-breakdown.md Epic E3). */
export const PENDING_ENGINE_CARD: Partial<Record<AbilitySubTabName, string>> = {
  'Racial Abilities': 'E3.6',
  Archetype: 'E3.7',
  Background: 'E3.9',
  Misc: 'E3.9',
};

export function AbilitiesTab(props: { panels: Record<AbilitySubTabName, ReactNode>; initial?: AbilitySubTabName }) {
  const [active, setActive] = useState<AbilitySubTabName>(props.initial ?? 'Feats');
  const card = PENDING_ENGINE_CARD[active];
  return (
    <div>
      <div role="tablist" aria-label="Abilities" style={{ borderBottom: '1px solid var(--color-border)', display: 'flex', flexWrap: 'wrap', gap: '0.9rem', marginBottom: '1rem' }}>
        {ABILITY_SUB_TABS.map((name) => (
          <button
            key={name}
            type="button"
            role="tab"
            aria-selected={name === active}
            onClick={() => setActive(name)}
            style={{
              background: 'none',
              border: 'none',
              borderBottom: `2px solid ${name === active ? 'var(--color-accent)' : 'transparent'}`,
              color: name === active ? 'var(--color-text)' : 'var(--color-text-muted)',
              cursor: 'pointer',
              fontSize: '0.82rem',
              fontWeight: name === active ? 700 : 500,
              padding: '0.2rem 0.1rem 0.45rem',
            }}
          >
            {name}
          </button>
        ))}
      </div>
      {card !== undefined ? (
        <p role="status" style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0 0 1rem', textAlign: 'center' }}>
          Picking {active.toLowerCase()} on the sheet arrives with engine card {card}: its budget, choices and
          prerequisite checks are not wired yet.
        </p>
      ) : null}
      {props.panels[active]}
    </div>
  );
}

import { useState } from 'react';
import { classSupportLevelSuffix, type ClassOption } from './characterHubModel';
import { groupClassOptionsByFamily } from './classRoster';
import {
  canAddLevel,
  characterLevel,
  levelHitPoints,
  totalHitPoints,
  type CreationLevel,
} from './levelsModel';

const TAG_LABEL = { max: 'Max', average: 'Average', rolled: 'Rolled' } as const;

function tagFor(entry: CreationLevel, index: number): keyof typeof TAG_LABEL {
  if (index === 0) return 'max';
  return entry.rolled ? 'rolled' : 'average';
}

const buttonStyle = {
  background: 'none',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.8rem',
  lineHeight: 1,
  padding: '0.3rem 0.5rem',
} as const;

/**
 * The Levels column of the Create screen: pick a class, Add level, and the level appears in the
 * list below with the hit points it contributes. The first level always takes the full hit die; each
 * later level defaults to the die's average and has a dice button to roll it instead. The totals
 * feed the Level and HP boxes. See `levelsModel.ts` for the rules.
 */
export function LevelsPanel(props: {
  classOptions: readonly ClassOption[];
  levels: CreationLevel[];
  constitutionModifier: number;
  onAdd: (classId: string) => void;
  onReroll: (index: number) => void;
  onRemove: (index: number) => void;
}) {
  const [selectedId, setSelectedId] = useState(props.classOptions[0]?.id ?? '');
  const selected = props.classOptions.find((option) => option.id === selectedId) ?? props.classOptions[0];
  const labelOf = (classId: string) => props.classOptions.find((option) => option.id === classId)?.label ?? classId;
  const availability = selected ? canAddLevel(props.levels, selected) : { ok: false, reason: 'No classes are available.' };

  return (
    <section
      id="levels-panel"
      aria-label="Levels"
      style={{
        backgroundColor: 'var(--color-surface)',
        border: '1px solid var(--color-border)',
        borderRadius: 10,
        flex: '2 1 380px',
        minWidth: 0,
        padding: '1rem',
      }}
    >
      <div style={{ alignItems: 'baseline', display: 'flex', justifyContent: 'space-between', marginBottom: '0.5rem' }}>
        <p style={{ fontSize: '0.95rem', fontWeight: 700, margin: 0 }}>Levels</p>
        <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem', fontWeight: 700, margin: 0 }}>
          Level {characterLevel(props.levels)} · HP {totalHitPoints(props.levels, props.constitutionModifier)}
        </p>
      </div>

      <div style={{ display: 'flex', gap: '0.5rem', marginBottom: '0.4rem' }}>
        <select
          id="character-class"
          aria-label="Class to add"
          value={selected?.id ?? ''}
          onChange={(event) => setSelectedId(event.target.value)}
          style={{ flex: 1, minWidth: 0 }}
        >
          {groupClassOptionsByFamily(props.classOptions).map((group) => {
            const rows = group.options.map((option) => (
              <option key={option.id} value={option.id}>
                {option.label}
                {classSupportLevelSuffix(option.supportLevel)}
              </option>
            ));
            return group.familyLabel ? (
              <optgroup key={group.family} label={group.familyLabel}>
                {rows}
              </optgroup>
            ) : (
              rows
            );
          })}
        </select>
        <button
          type="button"
          id="add-level-button"
          disabled={!availability.ok}
          onClick={() => selected && props.onAdd(selected.id)}
          style={{ ...buttonStyle, fontWeight: 700, opacity: availability.ok ? 1 : 0.5 }}
        >
          Add level
        </button>
      </div>
      {!availability.ok && availability.reason ? (
        <p role="note" style={{ color: 'var(--color-warn)', fontSize: '0.75rem', margin: '0 0 0.5rem' }}>
          {availability.reason}
        </p>
      ) : null}

      {props.levels.length === 0 ? (
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', margin: '0.75rem 0' }}>
          No levels yet. Pick a class and Add level; a character needs at least one.
        </p>
      ) : (
        <ol style={{ listStyle: 'none', margin: '0.5rem 0', padding: 0 }}>
          {props.levels.map((entry, index) => {
            const tag = tagFor(entry, index);
            const gained = levelHitPoints(entry, props.constitutionModifier);
            return (
              <li
                key={`${index}-${entry.classId}`}
                data-level-row
                style={{ alignItems: 'center', borderBottom: '1px solid var(--color-border)', display: 'flex', gap: '0.5rem', padding: '0.35rem 0' }}
              >
                <span style={{ color: 'var(--color-text-muted)', width: 22 }}>{index + 1}</span>
                <span style={{ flex: 1, minWidth: 0 }}>
                  <strong>{labelOf(entry.classId)}</strong> <span style={{ color: 'var(--color-text-muted)' }}>d{entry.hitDie}</span>
                </span>
                <span style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem' }}>
                  {entry.value} + {props.constitutionModifier} = <strong>{gained}</strong> HP
                </span>
                <span style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', minWidth: 52, textAlign: 'right' }}>{TAG_LABEL[tag]}</span>
                {index > 0 ? (
                  <button
                    type="button"
                    aria-label={`Reroll level ${index + 1}`}
                    title={`Roll d${entry.hitDie} for this level`}
                    onClick={() => props.onReroll(index)}
                    style={buttonStyle}
                  >
                    🎲
                  </button>
                ) : (
                  <span style={{ width: 34 }} />
                )}
                <button
                  type="button"
                  aria-label={`Remove level ${index + 1}`}
                  title="Remove this level"
                  onClick={() => props.onRemove(index)}
                  style={buttonStyle}
                >
                  ✕
                </button>
              </li>
            );
          })}
        </ol>
      )}

      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0.4rem 0 0' }}>
        The first level takes the full hit die. Each later level takes the average (half the die, plus 1); the dice button rolls it instead.
        The Constitution modifier applies to every level.
      </p>
    </section>
  );
}

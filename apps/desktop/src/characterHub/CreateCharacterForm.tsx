import { useState, type CSSProperties, type FormEvent, type ReactNode } from 'react';
import {
  ABILITY_ABBREVIATIONS,
  ABILITY_KEYS,
  ALIGNMENT_OPTIONS,
  AGE_OPTIONS,
  CLASS_OPTIONS,
  DEFAULT_ABILITY_SCORES,
  RACE_OPTIONS,
  ageEffectForAbility,
  describeClassSupportLevel,
  formatHeight,
  maxHitPointsAtLevelOne,
  rollDice,
  type AbilityKey,
  type AgeCategory,
  type BodyProfile,
  type RaceOption,
  type Sex,
} from './characterHubModel';
import { composeCreateCharacterRequest } from './composeCreateCharacterRequest';
import { createCharacterRuntime } from './characterHubRuntime';
import type { CreateCharacterOutcomeSurface } from './buildCreateCharacterOutcomeSurface';

const LABEL_STYLE: CSSProperties = {
  color: 'var(--color-text-secondary)',
  display: 'block',
  fontSize: '0.875rem',
  fontWeight: 600,
  marginBottom: '0.35rem',
};
const INPUT_STYLE: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  boxSizing: 'border-box',
  color: 'var(--color-text)',
  padding: '0.5rem 0.65rem',
  width: '100%',
};
const FIELD_STYLE: CSSProperties = { marginBottom: '1rem' };
const ROW_STYLE: CSSProperties = { ...FIELD_STYLE, display: 'flex', gap: '1rem' };

/** Label + control wrapper for one field. */
function LabeledField(props: { label: string; htmlFor?: string; children: ReactNode; flex?: string }) {
  return (
    <div style={{ flex: props.flex ?? '1', minWidth: 0 }}>
      <label style={LABEL_STYLE} htmlFor={props.htmlFor}>
        {props.label}
      </label>
      {props.children}
    </div>
  );
}

/** Read-only computed value styled like an input, with an optional trailing action (e.g. a reroll button). */
function ReadOnlyBox(props: { value: string; action?: ReactNode }) {
  return (
    <div style={{ ...INPUT_STYLE, alignItems: 'center', display: 'flex', gap: '0.5rem', justifyContent: 'space-between' }}>
      <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{props.value}</span>
      {props.action}
    </div>
  );
}

function DiceButton(props: { onClick: () => void; label: string }) {
  return (
    <button
      type="button"
      onClick={props.onClick}
      title={props.label}
      aria-label={props.label}
      style={{
        background: 'none',
        border: '1px solid var(--color-border)',
        borderRadius: 6,
        cursor: 'pointer',
        fontSize: '0.9rem',
        lineHeight: 1,
        padding: '0.2rem 0.35rem',
      }}
    >
      🎲
    </button>
  );
}

function stepButtonStyle(enabled: boolean): CSSProperties {
  return {
    backgroundColor: enabled ? 'var(--color-accent)' : 'var(--color-surface-2)',
    border: '1px solid var(--color-border)',
    borderRadius: 6,
    color: enabled ? 'var(--color-on-accent)' : 'var(--color-text-muted)',
    cursor: enabled ? 'pointer' : 'not-allowed',
    fontSize: '1rem',
    fontWeight: 800,
    height: 22,
    lineHeight: 1,
    width: 22,
  };
}

const FIXED_LEVEL = 1;

type Allocation = Record<AbilityKey, number>;
const ZERO_ALLOCATION: Allocation = {
  strength: 0,
  dexterity: 0,
  constitution: 0,
  intelligence: 0,
  wisdom: 0,
  charisma: 0,
};

function rollHeight(body: BodyProfile): number {
  return body.baseHeightInches + rollDice(body.heightModDice.count, body.heightModDice.sides);
}

function rollWeight(body: BodyProfile): number {
  return body.baseWeightLb + rollDice(body.heightModDice.count, body.heightModDice.sides) * body.weightMultiplierLb;
}

export function CreateCharacterForm(props: { onCreated: () => void }) {
  const [displayLabel, setDisplayLabel] = useState('');
  const [playerName, setPlayerName] = useState('');
  const [raceId, setRaceId] = useState(RACE_OPTIONS[0].id);
  const [classId, setClassId] = useState(CLASS_OPTIONS[0].id);
  const [abilityScores, setAbilityScores] = useState({ ...DEFAULT_ABILITY_SCORES });
  const [allocation, setAllocation] = useState<Allocation>({ ...ZERO_ALLOCATION });
  const [alignment, setAlignment] = useState<string>(ALIGNMENT_OPTIONS[4]); // True Neutral
  const [deity, setDeity] = useState('');
  const [sex, setSex] = useState<Sex>('male');
  const [age, setAge] = useState<AgeCategory>('Adult');
  const [eyes, setEyes] = useState('');
  const [hair, setHair] = useState('');
  const [heightInches, setHeightInches] = useState(() => rollHeight(RACE_OPTIONS[0].body.male));
  const [weightLb, setWeightLb] = useState(() => rollWeight(RACE_OPTIONS[0].body.male));
  const [submitting, setSubmitting] = useState(false);
  const [outcome, setOutcome] = useState<CreateCharacterOutcomeSurface | null>(null);
  const [error, setError] = useState<string | null>(null);

  const selectedClass = CLASS_OPTIONS.find((option) => option.id === classId) ?? CLASS_OPTIONS[0];
  const selectedRace = RACE_OPTIONS.find((option) => option.id === raceId) ?? RACE_OPTIONS[0];
  const body = selectedRace.body[sex];

  const allocatedPoints = ABILITY_KEYS.reduce((sum, key) => sum + allocation[key], 0);
  const remainingPoints = selectedRace.floatingBonusPoints - allocatedPoints;

  function calculatedScore(key: AbilityKey): number {
    return abilityScores[key] + (selectedRace.abilityAdjustments[key] ?? 0) + allocation[key] + ageEffectForAbility(age, key);
  }

  const maxHp = maxHitPointsAtLevelOne(selectedClass.hitDie, calculatedScore('constitution'));

  function reroll(nextRace: RaceOption, nextSex: Sex) {
    setHeightInches(rollHeight(nextRace.body[nextSex]));
    setWeightLb(rollWeight(nextRace.body[nextSex]));
  }

  function handleRaceChange(nextRaceId: string) {
    const nextRace = RACE_OPTIONS.find((option) => option.id === nextRaceId) ?? RACE_OPTIONS[0];
    setRaceId(nextRaceId);
    setAllocation({ ...ZERO_ALLOCATION });
    reroll(nextRace, sex);
  }

  function handleSexChange(nextSex: Sex) {
    setSex(nextSex);
    reroll(selectedRace, nextSex);
  }

  function adjustAllocation(key: AbilityKey, delta: 1 | -1) {
    setAllocation((prev) => {
      const next = prev[key] + delta;
      if (next < 0) {
        return prev;
      }
      if (delta === 1 && remainingPoints <= 0) {
        return prev;
      }
      return { ...prev, [key]: next };
    });
  }

  // The engine still applies a single "+2 to one ability" via abilityBonusTarget;
  // derive it from whichever ability received the most distributed points.
  function deriveAbilityBonusTarget(): AbilityKey {
    let target: AbilityKey = 'strength';
    let best = 0;
    for (const key of ABILITY_KEYS) {
      if (allocation[key] > best) {
        best = allocation[key];
        target = key;
      }
    }
    return target;
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setSubmitting(true);
    setError(null);
    try {
      const request = composeCreateCharacterRequest(
        { displayLabel, raceId, classId, level: FIXED_LEVEL, abilityScores, abilityBonusTarget: deriveAbilityBonusTarget() },
        { generateId: () => crypto.randomUUID(), now: () => new Date().toISOString() }
      );
      const result = await createCharacterRuntime(request);
      setOutcome(result);
      if (result.kind === 'saved') {
        props.onCreated();
      }
    } catch (cause: unknown) {
      setError(cause instanceof Error ? cause.message : 'Unknown character creation failure');
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <form onSubmit={handleSubmit} style={{ border: '1px solid var(--color-border)', borderRadius: 12, padding: '1.25rem' }}>
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '1.5rem' }}>
        {/* Left column: identity + rule-set fields */}
        <div style={{ flex: '1 1 360px', minWidth: 0 }}>
          {/* Character name + Player name on one line */}
          <div style={ROW_STYLE}>
            <div style={{ flex: 1, minWidth: 0 }}>
              <label style={LABEL_STYLE} htmlFor="character-name">
                Character name
              </label>
              <input
                id="character-name"
                style={INPUT_STYLE}
                value={displayLabel}
                onChange={(event) => setDisplayLabel(event.target.value)}
                required
              />
            </div>
            <div style={{ flex: 1, minWidth: 0 }}>
              <label style={LABEL_STYLE} htmlFor="player-name">
                Player name
              </label>
              <input
                id="player-name"
                style={INPUT_STYLE}
                value={playerName}
                onChange={(event) => setPlayerName(event.target.value)}
              />
            </div>
          </div>

          {/* Race + Class on one line */}
          <div style={ROW_STYLE}>
            <div style={{ flex: 1, minWidth: 0 }}>
              <label style={LABEL_STYLE} htmlFor="character-race">
                Race
              </label>
              <select id="character-race" style={INPUT_STYLE} value={raceId} onChange={(event) => handleRaceChange(event.target.value)}>
                {RACE_OPTIONS.map((option) => (
                  <option key={option.id} value={option.id}>
                    {option.label}
                  </option>
                ))}
              </select>
            </div>
            <div style={{ flex: 1, minWidth: 0 }}>
              <label style={LABEL_STYLE} htmlFor="character-class">
                Class
              </label>
              <select id="character-class" style={INPUT_STYLE} value={classId} onChange={(event) => setClassId(event.target.value)}>
                {CLASS_OPTIONS.map((option) => (
                  <option key={option.id} value={option.id}>
                    {option.label}
                    {option.supportLevel === 'full' ? '' : option.supportLevel === 'partial-human-only' ? ' (Human only, partial)' : ' (not yet computed)'}
                  </option>
                ))}
              </select>
            </div>
          </div>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '-0.5rem 0 1rem' }}>
            {describeClassSupportLevel(selectedClass.supportLevel, selectedClass.label)}
          </p>

          {/* HP (computed) + Alignment + Deity — the old level row */}
          <div style={ROW_STYLE}>
            <LabeledField label="HP" flex="0 0 96px">
              <ReadOnlyBox value={String(maxHp)} />
            </LabeledField>
            <LabeledField label="Alignment" htmlFor="character-alignment">
              <select id="character-alignment" style={INPUT_STYLE} value={alignment} onChange={(event) => setAlignment(event.target.value)}>
                {ALIGNMENT_OPTIONS.map((option) => (
                  <option key={option} value={option}>
                    {option}
                  </option>
                ))}
              </select>
            </LabeledField>
            <LabeledField label="Deity" htmlFor="character-deity">
              <input id="character-deity" style={INPUT_STYLE} value={deity} onChange={(event) => setDeity(event.target.value)} />
            </LabeledField>
          </div>

          {/* Physical attributes */}
          <p style={{ ...LABEL_STYLE, borderTop: '1px solid var(--color-border)', color: 'var(--color-text)', fontSize: '0.95rem', marginTop: '0.5rem', paddingTop: '1rem' }}>
            Physical Attributes
          </p>
          <div style={{ display: 'grid', gap: '1rem', gridTemplateColumns: 'repeat(3, 1fr)' }}>
            <LabeledField label="Size">
              <ReadOnlyBox value={selectedRace.size} />
            </LabeledField>
            <LabeledField label="Sex" htmlFor="character-sex">
              <select id="character-sex" style={INPUT_STYLE} value={sex} onChange={(event) => handleSexChange(event.target.value as Sex)}>
                <option value="male">Male</option>
                <option value="female">Female</option>
              </select>
            </LabeledField>
            <LabeledField label="Vision">
              <ReadOnlyBox value={selectedRace.vision} />
            </LabeledField>

            <LabeledField label="Height">
              <ReadOnlyBox
                value={formatHeight(heightInches)}
                action={<DiceButton label="Reroll height" onClick={() => setHeightInches(rollHeight(body))} />}
              />
            </LabeledField>
            <LabeledField label="Weight">
              <ReadOnlyBox
                value={`${weightLb} lb`}
                action={<DiceButton label="Reroll weight" onClick={() => setWeightLb(rollWeight(body))} />}
              />
            </LabeledField>
            <LabeledField label="Age" htmlFor="character-age">
              <select id="character-age" style={INPUT_STYLE} value={age} onChange={(event) => setAge(event.target.value as AgeCategory)}>
                {AGE_OPTIONS.map((option) => (
                  <option key={option} value={option}>
                    {option}
                  </option>
                ))}
              </select>
            </LabeledField>

            <LabeledField label="Eyes" htmlFor="character-eyes">
              <input id="character-eyes" style={INPUT_STYLE} value={eyes} onChange={(event) => setEyes(event.target.value)} />
            </LabeledField>
            <LabeledField label="Hair" htmlFor="character-hair">
              <input id="character-hair" style={INPUT_STYLE} value={hair} onChange={(event) => setHair(event.target.value)} />
            </LabeledField>
          </div>
        </div>

        {/* Right column: ability scores panel */}
        <div
          style={{
            backgroundColor: 'var(--color-surface)',
            border: '1px solid var(--color-border)',
            borderRadius: 10,
            flex: '0 0 320px',
            padding: '1rem',
          }}
        >
          <p style={{ ...LABEL_STYLE, marginBottom: '0.75rem' }}>Ability scores</p>
          <div
            style={{
              alignItems: 'center',
              color: 'var(--color-text-muted)',
              display: 'grid',
              fontSize: '0.7rem',
              gap: '0.4rem 0.5rem',
              gridTemplateColumns: '44px 1fr 110px',
              letterSpacing: '0.04em',
              marginBottom: '0.35rem',
              textTransform: 'uppercase',
            }}
          >
            <span />
            <span>Raw</span>
            <span style={{ textAlign: 'center' }}>Calculated</span>
          </div>

          {ABILITY_KEYS.map((key) => (
            <div
              key={key}
              style={{ alignItems: 'center', display: 'grid', gap: '0.4rem 0.5rem', gridTemplateColumns: '44px 1fr 110px', marginBottom: '0.4rem' }}
            >
              <label style={{ fontSize: '0.75rem', fontWeight: 700 }} htmlFor={`ability-${key}`}>
                {ABILITY_ABBREVIATIONS[key]}
              </label>
              <input
                id={`ability-${key}`}
                type="number"
                style={{ ...INPUT_STYLE, padding: '0.35rem 0.5rem' }}
                value={abilityScores[key]}
                onChange={(event) => setAbilityScores((prev) => ({ ...prev, [key]: Number(event.target.value) }))}
              />
              <div style={{ alignItems: 'center', display: 'flex', gap: '0.35rem', justifyContent: 'center' }}>
                {selectedRace.floatingBonusPoints > 0 ? (
                  <button
                    type="button"
                    aria-label={`Decrease ${key}`}
                    onClick={() => adjustAllocation(key, -1)}
                    disabled={allocation[key] <= 0}
                    style={stepButtonStyle(allocation[key] > 0)}
                  >
                    −
                  </button>
                ) : null}
                <span style={{ fontWeight: 800, minWidth: 24, textAlign: 'center' }}>{calculatedScore(key)}</span>
                {selectedRace.floatingBonusPoints > 0 ? (
                  <button
                    type="button"
                    aria-label={`Increase ${key}`}
                    onClick={() => adjustAllocation(key, 1)}
                    disabled={remainingPoints <= 0}
                    style={stepButtonStyle(remainingPoints > 0)}
                  >
                    +
                  </button>
                ) : null}
              </div>
            </div>
          ))}

          {(() => {
            const fixed = ABILITY_KEYS.filter((key) => selectedRace.abilityAdjustments[key]).map(
              (key) => `${(selectedRace.abilityAdjustments[key] as number) > 0 ? '+' : ''}${selectedRace.abilityAdjustments[key]} ${ABILITY_ABBREVIATIONS[key]}`
            );
            return fixed.length ? (
              <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.5rem 0 0' }}>
                {selectedRace.label} racial modifiers: {fixed.join(', ')}
              </p>
            ) : null;
          })()}

          {selectedRace.floatingBonusPoints > 0 ? (
            <div
              style={{
                alignItems: 'center',
                backgroundColor: 'var(--color-surface-2)',
                border: `1px solid ${remainingPoints > 0 ? 'var(--color-accent)' : 'var(--color-border)'}`,
                borderRadius: 8,
                display: 'flex',
                justifyContent: 'space-between',
                marginTop: '0.75rem',
                padding: '0.6rem 0.75rem',
              }}
            >
              <span style={{ fontSize: '0.85rem' }}>Ability enhancement points</span>
              <span style={{ color: remainingPoints > 0 ? 'var(--color-accent)' : 'var(--color-text-muted)', fontWeight: 800 }}>
                {remainingPoints} remaining
              </span>
            </div>
          ) : null}
        </div>
      </div>

      <button
        type="submit"
        disabled={submitting}
        style={{
          backgroundColor: 'var(--color-accent)',
          border: 'none',
          borderRadius: 8,
          color: 'var(--color-on-accent)',
          cursor: submitting ? 'default' : 'pointer',
          marginTop: '1.5rem',
          padding: '0.6rem 1.25rem',
        }}
      >
        {submitting ? 'Creating…' : 'Create character'}
      </button>

      {error ? <p style={{ color: 'var(--color-error)', marginTop: '0.75rem' }}>{error}</p> : null}

      {outcome ? (
        <div style={{ borderTop: '1px solid var(--color-border)', marginTop: '1.25rem', paddingTop: '1rem' }}>
          <h3 style={{ margin: '0 0 0.35rem' }}>{outcome.headline}</h3>
          <p style={{ color: 'var(--color-text-secondary)', margin: '0 0 0.75rem' }}>{outcome.detail}</p>
          {outcome.kind === 'saved' ? (
            <div style={{ display: 'grid', gap: '0.5rem', gridTemplateColumns: 'repeat(auto-fit, minmax(140px, 1fr))' }}>
              {outcome.highlights.map((highlight) => (
                <div
                  key={highlight.label}
                  style={{ backgroundColor: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 8, padding: '0.5rem 0.75rem' }}
                >
                  <p style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', margin: 0, textTransform: 'uppercase' }}>
                    {highlight.label}
                  </p>
                  <p style={{ color: 'var(--color-text)', fontSize: '1rem', fontWeight: 700, margin: '0.2rem 0 0' }}>
                    {highlight.value}
                  </p>
                </div>
              ))}
            </div>
          ) : (
            <div>
              {outcome.diagnosticGroups.map((group) => (
                <div key={group.label} style={{ marginBottom: '0.75rem' }}>
                  <p style={{ color: 'var(--color-warn)', fontSize: '0.8rem', fontWeight: 600, margin: '0 0 0.35rem' }}>
                    {group.label}
                  </p>
                  <ul style={{ color: 'var(--color-warn)', margin: 0, paddingLeft: '1.1rem' }}>
                    {group.messages.map((message) => (
                      <li key={message} style={{ marginBottom: '0.4rem' }}>
                        {message}
                      </li>
                    ))}
                  </ul>
                </div>
              ))}
              <details style={{ marginTop: '0.5rem' }}>
                <summary style={{ color: 'var(--color-text-muted)', cursor: 'pointer', fontSize: '0.8rem' }}>
                  Technical diagnostic details
                </summary>
                <ul style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.5rem 0 0', paddingLeft: '1.1rem' }}>
                  {outcome.rawDiagnostics.map((diagnostic) => (
                    <li key={diagnostic.id} style={{ marginBottom: '0.3rem' }}>
                      <code>{diagnostic.id}</code> ({diagnostic.claimBlocking ? 'blocking' : 'non-blocking'}):{' '}
                      {diagnostic.message}
                    </li>
                  ))}
                </ul>
              </details>
            </div>
          )}
        </div>
      ) : null}
    </form>
  );
}

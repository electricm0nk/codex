import { useState, type CSSProperties, type FormEvent } from 'react';
import {
  CLASS_OPTIONS,
  DEFAULT_ABILITY_SCORES,
  FIXED_LOADOUT_SUMMARY,
  RACE_OPTIONS,
  describeClassSupportLevel,
  getLevelOptionsForClass,
} from './characterHubModel';
import { composeCreateCharacterRequest } from './composeCreateCharacterRequest';
import { createCharacterRuntime } from './characterHubRuntime';
import type { CreateCharacterOutcomeSurface } from './buildCreateCharacterOutcomeSurface';

const LABEL_STYLE: CSSProperties = {
  color: '#334155',
  display: 'block',
  fontSize: '0.875rem',
  fontWeight: 600,
  marginBottom: '0.35rem',
};
const INPUT_STYLE: CSSProperties = {
  border: '1px solid #cbd5e1',
  borderRadius: 8,
  padding: '0.5rem 0.65rem',
  width: '100%',
};
const FIELD_STYLE: CSSProperties = { marginBottom: '1rem' };

const ABILITY_KEYS = ['strength', 'dexterity', 'constitution', 'intelligence', 'wisdom', 'charisma'] as const;
const HUMAN_RACE_ID = 'race:human';

export function CreateCharacterForm(props: { onCreated: () => void }) {
  const [displayLabel, setDisplayLabel] = useState('');
  const [raceId, setRaceId] = useState(RACE_OPTIONS[0].id);
  const [classId, setClassId] = useState(CLASS_OPTIONS[0].id);
  const [level, setLevel] = useState(getLevelOptionsForClass(CLASS_OPTIONS[0].id)[0]);
  const [abilityScores, setAbilityScores] = useState({ ...DEFAULT_ABILITY_SCORES });
  const [abilityBonusTarget, setAbilityBonusTarget] = useState<(typeof ABILITY_KEYS)[number]>('strength');
  const [submitting, setSubmitting] = useState(false);
  const [outcome, setOutcome] = useState<CreateCharacterOutcomeSurface | null>(null);
  const [error, setError] = useState<string | null>(null);

  const selectedClass = CLASS_OPTIONS.find((option) => option.id === classId) ?? CLASS_OPTIONS[0];
  const levelOptions = getLevelOptionsForClass(classId);

  function handleClassChange(nextClassId: string) {
    setClassId(nextClassId);
    const nextLevelOptions = getLevelOptionsForClass(nextClassId);
    if (!nextLevelOptions.includes(level)) {
      setLevel(nextLevelOptions[0]);
    }
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setSubmitting(true);
    setError(null);
    try {
      const request = composeCreateCharacterRequest(
        { displayLabel, raceId, classId, level, abilityScores, abilityBonusTarget },
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
    <form onSubmit={handleSubmit} style={{ border: '1px solid #cbd5e1', borderRadius: 12, padding: '1.25rem' }}>
      <div style={FIELD_STYLE}>
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

      <div style={FIELD_STYLE}>
        <label style={LABEL_STYLE} htmlFor="character-race">
          Race
        </label>
        <select
          id="character-race"
          style={INPUT_STYLE}
          value={raceId}
          onChange={(event) => setRaceId(event.target.value)}
        >
          {RACE_OPTIONS.map((option) => (
            <option key={option.id} value={option.id}>
              {option.label}
            </option>
          ))}
        </select>
      </div>

      {raceId === HUMAN_RACE_ID ? (
        <div style={FIELD_STYLE}>
          <label style={LABEL_STYLE} htmlFor="character-ability-bonus-target">
            Human ability bonus
          </label>
          <select
            id="character-ability-bonus-target"
            style={INPUT_STYLE}
            value={abilityBonusTarget}
            onChange={(event) => setAbilityBonusTarget(event.target.value as (typeof ABILITY_KEYS)[number])}
          >
            {ABILITY_KEYS.map((key) => (
              <option key={key} value={key} style={{ textTransform: 'capitalize' }}>
                {key.charAt(0).toUpperCase() + key.slice(1)}
              </option>
            ))}
          </select>
        </div>
      ) : null}

      <div style={FIELD_STYLE}>
        <label style={LABEL_STYLE} htmlFor="character-class">
          Class
        </label>
        <select
          id="character-class"
          style={INPUT_STYLE}
          value={classId}
          onChange={(event) => handleClassChange(event.target.value)}
        >
          {CLASS_OPTIONS.map((option) => (
            <option key={option.id} value={option.id}>
              {option.label}
              {option.supportLevel === 'full' ? '' : option.supportLevel === 'partial-human-only' ? ' (Human only, partial)' : ' (not yet computed)'}
            </option>
          ))}
        </select>
        <p style={{ color: '#64748b', fontSize: '0.8rem', marginTop: '0.35rem' }}>
          {describeClassSupportLevel(selectedClass.supportLevel, selectedClass.label)}
        </p>
      </div>

      <div style={FIELD_STYLE}>
        <label style={LABEL_STYLE} htmlFor="character-level">
          Level
        </label>
        <select
          id="character-level"
          style={INPUT_STYLE}
          value={level}
          onChange={(event) => setLevel(Number(event.target.value))}
        >
          {levelOptions.map((option) => (
            <option key={option} value={option}>
              {option}
            </option>
          ))}
        </select>
      </div>

      <div style={FIELD_STYLE}>
        <p style={LABEL_STYLE}>Ability scores</p>
        <div style={{ display: 'grid', gap: '0.6rem', gridTemplateColumns: 'repeat(auto-fit, minmax(120px, 1fr))' }}>
          {ABILITY_KEYS.map((key) => (
            <div key={key}>
              <label
                style={{ ...LABEL_STYLE, fontWeight: 400, textTransform: 'capitalize' }}
                htmlFor={`ability-${key}`}
              >
                {key}
              </label>
              <input
                id={`ability-${key}`}
                type="number"
                style={INPUT_STYLE}
                value={abilityScores[key]}
                onChange={(event) => setAbilityScores((prev) => ({ ...prev, [key]: Number(event.target.value) }))}
              />
            </div>
          ))}
        </div>
      </div>

      <div
        style={{
          ...FIELD_STYLE,
          backgroundColor: '#f8fafc',
          border: '1px solid #cbd5e1',
          borderRadius: 8,
          padding: '0.75rem 1rem',
        }}
      >
        <p style={{ color: '#0f172a', fontSize: '0.875rem', fontWeight: 600, margin: '0 0 0.5rem' }}>
          What this build includes
        </p>
        <p style={{ color: '#475569', fontSize: '0.8rem', margin: '0 0 0.25rem' }}>
          Feats: {FIXED_LOADOUT_SUMMARY.feats.join(', ')}
        </p>
        <p style={{ color: '#475569', fontSize: '0.8rem', margin: '0 0 0.25rem' }}>
          Skills: {FIXED_LOADOUT_SUMMARY.skills.join(', ')}
        </p>
        <p style={{ color: '#475569', fontSize: '0.8rem', margin: 0 }}>
          Equipment: {FIXED_LOADOUT_SUMMARY.equipment.join(', ')}
        </p>
      </div>

      <button
        type="submit"
        disabled={submitting}
        style={{
          backgroundColor: '#0f172a',
          border: 'none',
          borderRadius: 8,
          color: 'white',
          cursor: submitting ? 'default' : 'pointer',
          padding: '0.6rem 1.25rem',
        }}
      >
        {submitting ? 'Creating…' : 'Create character'}
      </button>

      {error ? <p style={{ color: '#991b1b', marginTop: '0.75rem' }}>{error}</p> : null}

      {outcome ? (
        <div style={{ borderTop: '1px solid #e2e8f0', marginTop: '1.25rem', paddingTop: '1rem' }}>
          <h3 style={{ margin: '0 0 0.35rem' }}>{outcome.headline}</h3>
          <p style={{ color: '#475569', margin: '0 0 0.75rem' }}>{outcome.detail}</p>
          {outcome.kind === 'saved' ? (
            <div style={{ display: 'grid', gap: '0.5rem', gridTemplateColumns: 'repeat(auto-fit, minmax(140px, 1fr))' }}>
              {outcome.highlights.map((highlight) => (
                <div
                  key={highlight.label}
                  style={{ backgroundColor: '#f8fafc', border: '1px solid #cbd5e1', borderRadius: 8, padding: '0.5rem 0.75rem' }}
                >
                  <p style={{ color: '#64748b', fontSize: '0.7rem', margin: 0, textTransform: 'uppercase' }}>
                    {highlight.label}
                  </p>
                  <p style={{ color: '#0f172a', fontSize: '1rem', fontWeight: 700, margin: '0.2rem 0 0' }}>
                    {highlight.value}
                  </p>
                </div>
              ))}
            </div>
          ) : (
            <div>
              {outcome.diagnosticGroups.map((group) => (
                <div key={group.label} style={{ marginBottom: '0.75rem' }}>
                  <p style={{ color: '#7c2d12', fontSize: '0.8rem', fontWeight: 600, margin: '0 0 0.35rem' }}>
                    {group.label}
                  </p>
                  <ul style={{ color: '#7c2d12', margin: 0, paddingLeft: '1.1rem' }}>
                    {group.messages.map((message) => (
                      <li key={message} style={{ marginBottom: '0.4rem' }}>
                        {message}
                      </li>
                    ))}
                  </ul>
                </div>
              ))}
              <details style={{ marginTop: '0.5rem' }}>
                <summary style={{ color: '#64748b', cursor: 'pointer', fontSize: '0.8rem' }}>
                  Technical diagnostic details
                </summary>
                <ul style={{ color: '#64748b', fontSize: '0.75rem', margin: '0.5rem 0 0', paddingLeft: '1.1rem' }}>
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

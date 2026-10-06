import { useEffect, useRef, useState, type CSSProperties, type FormEvent } from 'react';
import {
  createStarfinderCharacter,
  previewStarfinderCharacter,
  type SfCreationPreviewDto,
  type SfCreationSlotDto,
} from '../boundary/starfinderCreation';
import type { DiagnosticDto } from '../boundary/loadCreateCharacter';
import {
  EMPTY_STARFINDER_DRAFT,
  SF_ABILITY_ABBREVIATIONS,
  SF_ABILITY_KEYS,
  adjustPoints,
  blockingProblems,
  buildStarfinderCreationRequest,
  buildStarfinderPreviewRequest,
  keyAbilityAfterClassChange,
  openOptionalPicks,
  picksForSlot,
  reconcilePicks,
  samePicks,
  setSlotPick,
  starfinderOutcomeTiles,
  type StarfinderCreationDraft,
  type StarfinderOutcomeTile,
} from './starfinderCreationModel';

/**
 * Starfinder 1e character creation (SD-37 E6.2): race -> theme -> class ->
 * point buy. Every list, pick, budget and score on this form is the engine's
 * answer for the current choices (`preview_starfinder_character`); "Create
 * character" saves through `create_starfinder_character`, which recomputes the
 * character with every Starfinder total and refuses (saving nothing) a build
 * the rules do not allow.
 */

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
const FIELD_STYLE: CSSProperties = { flex: '1 1 200px', marginBottom: '1rem', minWidth: 0 };
const ROW_STYLE: CSSProperties = { display: 'flex', flexWrap: 'wrap', gap: '1rem' };
const SMALL_BUTTON_STYLE: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.8rem',
  padding: '0.2rem 0.5rem',
};

type Outcome =
  | { kind: 'saved'; headline: string; detail: string; tiles: StarfinderOutcomeTile[] }
  | { kind: 'blocked'; headline: string; detail: string; diagnostics: DiagnosticDto[] };

export function SlotPicker(props: {
  slot: SfCreationSlotDto;
  chosen: string[];
  onChange: (position: number, optionId: string) => void;
}) {
  const { slot } = props;
  const emptyLabel = `— ${slot.required ? 'choose' : 'not chosen'}: ${slot.label} —`;
  return (
    <div style={FIELD_STYLE}>
      <span style={LABEL_STYLE}>
        {slot.label}
        {slot.count > 1 ? ` (choose ${slot.count})` : ''}
        {slot.required ? ' — adjusts ability scores' : ''}
      </span>
      {Array.from({ length: slot.count }, (_, position) => (
        <select
          key={position}
          aria-label={`${slot.label} ${position + 1}`}
          style={{ ...INPUT_STYLE, marginBottom: '0.35rem' }}
          value={props.chosen[position] ?? ''}
          onChange={(event) => props.onChange(position, event.target.value)}
        >
          <option value="">{emptyLabel}</option>
          {slot.options.map((option) => (
            <option key={option.id} value={option.id}>
              {option.label}
            </option>
          ))}
        </select>
      ))}
    </div>
  );
}

export function StarfinderCreateForm(props: { onCreated: () => void }) {
  const [draft, setDraft] = useState<StarfinderCreationDraft>(EMPTY_STARFINDER_DRAFT);
  const [preview, setPreview] = useState<SfCreationPreviewDto | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [outcome, setOutcome] = useState<Outcome | null>(null);
  const previewRound = useRef(0);

  // Every choice change asks the engine for the form's next state. The name is not a
  // rules choice, so typing it never re-previews.
  const previewKey = JSON.stringify(buildStarfinderPreviewRequest({ ...draft, displayLabel: '' }));
  useEffect(() => {
    const round = ++previewRound.current;
    previewStarfinderCharacter(buildStarfinderPreviewRequest(draft))
      .then((next) => {
        if (round !== previewRound.current) {
          return; // a later choice already asked again
        }
        setPreview(next);
        setError(null);
        const picks = reconcilePicks(draft.picks, next.slots);
        if (!samePicks(picks, draft.picks)) {
          setDraft((current) => ({ ...current, picks }));
        }
      })
      .catch((cause: unknown) => {
        if (round === previewRound.current) {
          setError(cause instanceof Error ? cause.message : String(cause));
        }
      });
    // `previewKey` is the draft minus the name: the effect re-runs exactly when a choice changes.
  }, [previewKey]);

  const classes = preview?.classes ?? [];
  const selectedClass = classes.find((candidate) => candidate.id === draft.classId);
  const blocking = preview ? blockingProblems(preview) : [];
  const optionalOpen = preview ? openOptionalPicks(preview) : [];
  const rules = preview?.pointBuyRules;

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setSubmitting(true);
    setError(null);
    try {
      const request = buildStarfinderCreationRequest(draft, {
        generateId: () => crypto.randomUUID(),
        now: () => new Date().toISOString(),
      });
      const result = await createStarfinderCharacter(request);
      if (result.kind === 'Saved') {
        setOutcome({
          kind: 'saved',
          headline: `${result.summary.displayLabel} is ready`,
          detail: 'Your character was computed and saved.',
          tiles: starfinderOutcomeTiles(result.explanations),
        });
        props.onCreated();
      } else {
        setOutcome({
          kind: 'blocked',
          headline: 'Character not created',
          detail: 'Nothing was saved. Resolve these first:',
          diagnostics: result.diagnostics,
        });
      }
    } catch (cause: unknown) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <form onSubmit={handleSubmit} style={{ border: '1px solid var(--color-border)', borderRadius: 12, padding: '1.25rem' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0 0 1rem' }}>
        Starfinder 1st Edition — a 1st-level character: race, theme, class, then the point buy.
      </p>
      <div style={ROW_STYLE}>
        <div style={FIELD_STYLE}>
          <label style={LABEL_STYLE} htmlFor="sf-character-name">
            Character name
          </label>
          <input
            id="sf-character-name"
            style={INPUT_STYLE}
            value={draft.displayLabel}
            onChange={(event) => setDraft({ ...draft, displayLabel: event.target.value })}
            required
          />
        </div>
      </div>

      <div style={ROW_STYLE}>
        <div style={FIELD_STYLE}>
          <label style={LABEL_STYLE} htmlFor="sf-race">
            Race
          </label>
          <select
            id="sf-race"
            style={INPUT_STYLE}
            value={draft.raceId ?? ''}
            onChange={(event) => setDraft({ ...draft, raceId: event.target.value || null })}
          >
            <option value="">— choose a race —</option>
            {(preview?.races ?? []).map((option) => (
              <option key={option.id} value={option.id}>
                {option.label}
              </option>
            ))}
          </select>
        </div>
        <div style={FIELD_STYLE}>
          <label style={LABEL_STYLE} htmlFor="sf-theme">
            Theme
          </label>
          <select
            id="sf-theme"
            style={INPUT_STYLE}
            value={draft.themeId ?? ''}
            onChange={(event) => setDraft({ ...draft, themeId: event.target.value || null })}
          >
            <option value="">— choose a theme —</option>
            {(preview?.themes ?? []).map((option) => (
              <option key={option.id} value={option.id}>
                {option.label}
              </option>
            ))}
          </select>
        </div>
        <div style={FIELD_STYLE}>
          <label style={LABEL_STYLE} htmlFor="sf-class">
            Class
          </label>
          <select
            id="sf-class"
            style={INPUT_STYLE}
            value={draft.classId ?? ''}
            onChange={(event) => {
              const classId = event.target.value || null;
              setDraft({ ...draft, classId, keyAbility: keyAbilityAfterClassChange(classes, classId, draft.keyAbility) });
            }}
          >
            <option value="">— choose a class —</option>
            {classes.map((option) => (
              <option key={option.id} value={option.id}>
                {option.label}
              </option>
            ))}
          </select>
        </div>
        {selectedClass && selectedClass.keyAbilityOptions.length > 1 ? (
          <div style={FIELD_STYLE}>
            <label style={LABEL_STYLE} htmlFor="sf-key-ability">
              Key ability
            </label>
            <select
              id="sf-key-ability"
              style={INPUT_STYLE}
              value={draft.keyAbility ?? ''}
              onChange={(event) => setDraft({ ...draft, keyAbility: event.target.value || null })}
            >
              <option value="">— choose the key ability —</option>
              {selectedClass.keyAbilityOptions.map((option) => (
                <option key={option.id} value={option.id}>
                  {option.label}
                </option>
              ))}
            </select>
          </div>
        ) : null}
      </div>

      {preview && preview.slots.length > 0 ? (
        <fieldset style={{ border: '1px solid var(--color-border)', borderRadius: 8, margin: '0 0 1rem', padding: '0.75rem 1rem 0' }}>
          <legend style={{ fontSize: '0.85rem', fontWeight: 700 }}>Choices for this race, theme and class</legend>
          <div style={ROW_STYLE}>
            {preview.slots.map((slot) => (
              <SlotPicker
                key={slot.slotId}
                slot={slot}
                chosen={picksForSlot(draft.picks, slot.slotId)}
                onChange={(position, optionId) => setDraft({ ...draft, picks: setSlotPick(draft.picks, slot, position, optionId) })}
              />
            ))}
          </div>
        </fieldset>
      ) : null}

      <fieldset style={{ border: '1px solid var(--color-border)', borderRadius: 8, margin: '0 0 1rem', padding: '0.75rem 1rem' }}>
        <legend style={{ fontSize: '0.85rem', fontWeight: 700 }}>Ability scores (point buy)</legend>
        {rules ? (
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: '0 0 0.6rem' }}>
            Points spent: {preview?.pointsSpent ?? 0} of {rules.budget}. Each point raises a score by 1; no score above{' '}
            {rules.maxScoreAtCreation} at creation ({rules.source}).
          </p>
        ) : null}
        <div style={{ display: 'grid', gap: '0.4rem' }}>
          {SF_ABILITY_KEYS.map((key, index) => {
            const abbreviation = SF_ABILITY_ABBREVIATIONS[key];
            const score = preview?.abilityScores[index];
            return (
              <div key={key} style={{ alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.6rem' }}>
                <span style={{ fontWeight: 700, width: '3rem' }}>{abbreviation}</span>
                <button
                  type="button"
                  style={SMALL_BUTTON_STYLE}
                  disabled={!rules}
                  onClick={() => rules && setDraft({ ...draft, pointBuy: adjustPoints(draft.pointBuy, key, -1, rules) })}
                >
                  -1 {abbreviation}
                </button>
                <span style={{ minWidth: '4.5rem' }}>{draft.pointBuy[key]} points</span>
                <button
                  type="button"
                  style={SMALL_BUTTON_STYLE}
                  disabled={!rules}
                  onClick={() => rules && setDraft({ ...draft, pointBuy: adjustPoints(draft.pointBuy, key, 1, rules) })}
                >
                  +1 {abbreviation}
                </button>
                {score ? (
                  <span>
                    <strong>
                      {score.label} {score.score}
                    </strong>{' '}
                    ({score.modifier >= 0 ? `+${score.modifier}` : score.modifier}) —{' '}
                    <span style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem' }}>
                      {score.terms.map((term) => `${term.label} ${term.value >= 0 ? `+${term.value}` : term.value}`).join(', ')}
                    </span>
                  </span>
                ) : null}
              </div>
            );
          })}
        </div>
      </fieldset>

      {preview && preview.chosenOnTheSheet.length > 0 ? (
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: '0 0 0.75rem' }}>
          Chosen on the sheet: {preview.chosenOnTheSheet.join('; ')}
        </p>
      ) : null}
      {optionalOpen.length > 0 ? (
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: '0 0 0.75rem' }}>
          Not chosen yet (optional): {optionalOpen.map((problem) => problem.message).join('; ')}
        </p>
      ) : null}
      {blocking.length > 0 ? (
        <ul style={{ color: 'var(--color-warn)', fontSize: '0.85rem', margin: '0 0 0.75rem', paddingLeft: '1.1rem' }}>
          {blocking.map((problem) => (
            <li key={`${problem.id}:${problem.message}`}>{problem.message}</li>
          ))}
        </ul>
      ) : null}

      <button
        type="submit"
        disabled={submitting || !preview || blocking.length > 0}
        style={{
          backgroundColor: 'var(--color-accent)',
          border: 'none',
          borderRadius: 8,
          color: 'var(--color-on-accent)',
          cursor: submitting ? 'default' : 'pointer',
          marginTop: '0.5rem',
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
              {outcome.tiles.map((tile) => (
                <div
                  key={tile.label}
                  style={{ backgroundColor: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 8, padding: '0.5rem 0.75rem' }}
                >
                  <p style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', margin: 0, textTransform: 'uppercase' }}>{tile.label}</p>
                  <p style={{ color: 'var(--color-text)', fontSize: '1rem', fontWeight: 700, margin: '0.2rem 0 0' }}>{tile.value}</p>
                </div>
              ))}
            </div>
          ) : (
            <ul style={{ color: 'var(--color-warn)', margin: 0, paddingLeft: '1.1rem' }}>
              {outcome.diagnostics.map((diagnostic) => (
                <li key={`${diagnostic.id}:${diagnostic.message}`} style={{ marginBottom: '0.4rem' }}>
                  {diagnostic.message} <code style={{ fontSize: '0.75rem' }}>{diagnostic.id}</code>
                </li>
              ))}
            </ul>
          )}
        </div>
      ) : null}
    </form>
  );
}

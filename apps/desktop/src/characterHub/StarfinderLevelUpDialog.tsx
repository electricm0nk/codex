import { useEffect, useRef, useState, type CSSProperties } from 'react';
import {
  levelUpStarfinderCharacter,
  previewStarfinderLevelUp,
  type SfLevelUpPreviewDto,
} from '../boundary/starfinderLevelUp';
import type { DiagnosticDto } from '../boundary/loadCreateCharacter';
import { picksForSlot, reconcilePicks, samePicks, setSlotPick } from './starfinderCreationModel';
import { SlotPicker } from './StarfinderCreateForm';
import {
  EMPTY_LEVEL_UP_DRAFT,
  adjustSkillRanks,
  buildStarfinderLevelUpRequest,
  canAcceptLevelUp,
  classChoiceAfterPreview,
  draftAfterClassChange,
  levelUpChangeLines,
  levelUpNeedsKeyAbility,
  toggleIncrease,
  type StarfinderLevelUpDraft,
} from './starfinderLevelUpModel';

/**
 * The Starfinder level-up (SD-37 E6.5): the class for the next character level (a held class
 * advances by one; another class starts at 1), the ability increase when the level has one,
 * this level's skill ranks and the picks the level opens. Every list, cap and number here is
 * the engine's answer for the current choices (`preview_starfinder_level_up`), including the
 * sheet totals the level changes; "Accept level-up" saves through
 * `level_up_starfinder_character`, which recomputes the character at the new level and refuses
 * (saving nothing) a level the rules do not allow.
 */

const SECTION_STYLE: CSSProperties = { border: '1px solid var(--color-border)', borderRadius: 8, margin: '0 0 0.75rem', padding: '0.6rem 0.9rem' };
const MUTED_STYLE: CSSProperties = { color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: '0.2rem 0' };
const SMALL_BUTTON_STYLE: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.8rem',
  padding: '0.2rem 0.5rem',
};
const SELECT_STYLE: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  color: 'var(--color-text)',
  padding: '0.45rem 0.6rem',
};

export function StarfinderLevelUpDialog(props: {
  characterId: string;
  onClose: () => void;
  /** Called with the new level's line once the level is saved. */
  onLeveledUp: (levelLine: string) => void;
}) {
  const [draft, setDraft] = useState<StarfinderLevelUpDraft>(EMPTY_LEVEL_UP_DRAFT);
  const [preview, setPreview] = useState<SfLevelUpPreviewDto | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [refused, setRefused] = useState<DiagnosticDto[]>([]);
  const [saving, setSaving] = useState(false);
  const round = useRef(0);

  const requestKey = JSON.stringify(buildStarfinderLevelUpRequest(props.characterId, draft, () => ''));
  useEffect(() => {
    const mine = ++round.current;
    previewStarfinderLevelUp(buildStarfinderLevelUpRequest(props.characterId, draft, () => ''))
      .then((next) => {
        if (mine !== round.current) {
          return;
        }
        setPreview(next);
        setError(null);
        const withClass = classChoiceAfterPreview(draft, next);
        const picks = reconcilePicks(withClass.picks, next.slots);
        if (withClass !== draft || !samePicks(picks, draft.picks)) {
          setDraft({ ...withClass, picks });
        }
      })
      .catch((cause: unknown) => {
        if (mine === round.current) {
          setError(cause instanceof Error ? cause.message : String(cause));
        }
      });
  }, [requestKey]);

  async function accept() {
    if (preview === null || preview.levelLine === null) {
      return;
    }
    setSaving(true);
    setRefused([]);
    try {
      const result = await levelUpStarfinderCharacter(buildStarfinderLevelUpRequest(props.characterId, draft, () => new Date().toISOString()));
      if (result.kind === 'Saved') {
        props.onLeveledUp(preview.levelLine);
      } else {
        setRefused(result.diagnostics);
      }
    } catch (cause: unknown) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setSaving(false);
    }
  }

  const classes = preview?.classes ?? [];
  const blocking = preview?.problems.filter((problem) => problem.claimBlocking) ?? [];
  const optional = preview?.problems.filter((problem) => !problem.claimBlocking) ?? [];
  const selected = classes.find((option) => option.id === draft.classId);

  return (
    <section role="dialog" aria-label="Level up" data-testid="sf-level-up" style={{ border: '1px solid var(--color-accent)', borderRadius: 10, margin: '0 0 1rem', padding: '1rem' }}>
      <h3 style={{ margin: '0 0 0.75rem' }}>Level up</h3>
      {preview ? <p style={MUTED_STYLE}>Character level now: {preview.characterLevel}</p> : null}

      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.75rem', marginBottom: '0.75rem' }}>
        <select
          aria-label="Class for this level"
          style={SELECT_STYLE}
          value={draft.classId ?? ''}
          onChange={(event) => setDraft(draftAfterClassChange(draft, event.target.value || null))}
        >
          <option value="">— choose the class for this level —</option>
          {classes.map((option) => (
            <option key={option.id} value={option.id}>
              {option.currentLevel > 0 ? `${option.label} (advance)` : `${option.label} (new class)`}
            </option>
          ))}
        </select>
        {preview && levelUpNeedsKeyAbility(preview, draft.classId) && selected ? (
          <select
            aria-label="Key ability for the new class"
            style={SELECT_STYLE}
            value={draft.keyAbility ?? ''}
            onChange={(event) => setDraft({ ...draft, keyAbility: event.target.value || null })}
          >
            <option value="">— choose the new class's key ability —</option>
            {selected.keyAbilityOptions.map((option) => (
              <option key={option.id} value={option.id}>
                {option.label}
              </option>
            ))}
          </select>
        ) : null}
      </div>

      {preview?.levelLine ? (
        <div style={SECTION_STYLE}>
          <p style={{ fontWeight: 700, margin: '0 0 0.3rem' }}>{preview.levelLine}</p>
          {[...preview.classLines, ...preview.ruleLines].map((line) => (
            <p key={line} style={MUTED_STYLE}>
              {line}
            </p>
          ))}
        </div>
      ) : null}

      {preview?.increaseDue ? (
        <div style={SECTION_STYLE}>
          <p style={{ fontWeight: 700, margin: '0 0 0.4rem' }}>Ability increase: choose four scores</p>
          <div style={{ display: 'grid', gap: '0.3rem' }}>
            {preview.abilities.map((ability) => (
              <div key={ability.ability} style={{ alignItems: 'center', display: 'flex', gap: '0.6rem' }}>
                <button
                  type="button"
                  aria-pressed={draft.abilityIncreases.includes(ability.ability)}
                  style={SMALL_BUTTON_STYLE}
                  onClick={() => setDraft(toggleIncrease(draft, ability.ability))}
                >
                  Increase {ability.label}
                </button>
                <span>
                  {ability.label} {ability.score}
                  {ability.newScore !== ability.score ? ` → ${ability.newScore}` : ''}
                  {ability.increase ? <span style={MUTED_STYLE}> ({ability.increase})</span> : null}
                </span>
              </div>
            ))}
          </div>
        </div>
      ) : null}

      {preview ? (
        <div style={SECTION_STYLE}>
          <p style={{ fontWeight: 700, margin: '0 0 0.2rem' }}>Skill ranks this level</p>
          <p style={MUTED_STYLE}>{preview.skillRule}</p>
          <div style={{ display: 'grid', gap: '0.25rem', gridTemplateColumns: 'repeat(auto-fill, minmax(17rem, 1fr))', marginTop: '0.4rem' }}>
            {preview.skills.map((skill) => {
              const added = draft.skillRanks[skill.skill] ?? 0;
              return (
                <div key={skill.skill} style={{ alignItems: 'center', display: 'flex', gap: '0.4rem' }}>
                  <button type="button" style={SMALL_BUTTON_STYLE} onClick={() => setDraft(adjustSkillRanks(draft, preview, skill.skill, -1))}>
                    -1 rank {skill.label}
                  </button>
                  <button type="button" style={SMALL_BUTTON_STYLE} onClick={() => setDraft(adjustSkillRanks(draft, preview, skill.skill, 1))}>
                    +1 rank {skill.label}
                  </button>
                  <span style={{ fontSize: '0.8rem' }}>
                    {skill.ranks + added}
                    {added > 0 ? ` (+${added})` : ''}
                  </span>
                </div>
              );
            })}
          </div>
        </div>
      ) : null}

      {preview && preview.slots.length > 0 ? (
        <div style={SECTION_STYLE}>
          <p style={{ fontWeight: 700, margin: '0 0 0.4rem' }}>Choices this level opens</p>
          {preview.slots.map((slot) => (
            <SlotPicker
              key={slot.slotId}
              slot={slot}
              chosen={picksForSlot(draft.picks, slot.slotId)}
              onChange={(position, optionId) => setDraft({ ...draft, picks: setSlotPick(draft.picks, slot, position, optionId) })}
            />
          ))}
        </div>
      ) : null}

      {preview && preview.changes.length > 0 ? (
        <div style={SECTION_STYLE} data-testid="sf-level-up-changes">
          <p style={{ fontWeight: 700, margin: '0 0 0.3rem' }}>What this level changes on the sheet</p>
          <ul style={{ margin: 0, paddingLeft: '1.1rem' }}>
            {levelUpChangeLines(preview).map((line) => (
              <li key={line} style={{ fontSize: '0.85rem' }}>
                {line}
              </li>
            ))}
          </ul>
        </div>
      ) : null}

      {preview && preview.chosenOnTheSheet.length > 0 ? <p style={MUTED_STYLE}>Chosen on the sheet: {preview.chosenOnTheSheet.join('; ')}</p> : null}
      {optional.length > 0 ? <p style={MUTED_STYLE}>Not chosen yet (optional): {optional.map((problem) => problem.message).join('; ')}</p> : null}
      {[...blocking, ...refused].length > 0 ? (
        <ul style={{ color: 'var(--color-warn)', fontSize: '0.85rem', margin: '0 0 0.75rem', paddingLeft: '1.1rem' }}>
          {[...blocking, ...refused].map((problem) => (
            <li key={`${problem.id}:${problem.message}`}>{problem.message}</li>
          ))}
        </ul>
      ) : null}
      {error ? <p style={{ color: 'var(--color-error)' }}>{error}</p> : null}

      <div style={{ display: 'flex', gap: '0.5rem' }}>
        <button
          type="button"
          disabled={saving || preview === null || !canAcceptLevelUp(preview)}
          onClick={() => void accept()}
          style={{ backgroundColor: 'var(--color-accent)', border: 'none', borderRadius: 8, color: 'var(--color-on-accent)', cursor: 'pointer', padding: '0.5rem 1rem' }}
        >
          {saving ? 'Saving…' : 'Accept level-up'}
        </button>
        <button type="button" onClick={props.onClose} style={{ ...SMALL_BUTTON_STYLE, padding: '0.5rem 1rem' }}>
          Cancel level-up
        </button>
      </div>
    </section>
  );
}

import { useEffect, useState } from 'react';
import { createPortal } from 'react-dom';
import { previewLevelUp, totalSkillPoints, type HeldClass } from './characterProgression';
import { knownClass } from './classCatalog';
import {
  describeEntryRequirement,
  ensureClassRosterLoaded,
  fallbackLevelUpResponse,
  levelUpChoiceGroups,
  loadLevelUpClassOptions,
  useClassCatalog,
  type LevelUpClassOptionsResponse,
} from './classRoster';
import {
  previewLevelUp as previewLevelUpGrants,
  type PreviewLevelUpResponse,
} from '../boundary/previewLevelUp';

/**
 * "Level up" popup: pick a class to take the next character level in and see
 * exactly what that level grants (hit die, skill points, class features)
 * before committing. Patterned after ThemeBrowserModal's portal-based overlay
 * shell so it matches the rest of the app's modal conventions.
 *
 * The class-feature half of that preview comes from the real engine
 * (`preview_level_up` -> `level_up::compute_level_up_grants_for_class`),
 * not from a hand-authored table. Every grant shown carries the engine's
 * own name and its own effect descriptions, verbatim.
 *
 * SD-36 F4c: the classes offered are the engine's (`list_level_up_class_options`), in three
 * groups — advance a class the character has, add a base class, add a prestige class. A prestige
 * class prints its entry requirements, each with its met/unmet note for this character, and is
 * offered either way (ruling §9.2: print, never block). Character level 20 is the cap. If the
 * command fails, advance/add-base come from the class catalog and the failure is printed.
 *
 * Accepting calls `onAccept(classId)` and closes; the caller
 * (`CharacterSheet`'s `handleLevelUpAccept`) persists the level-up via the
 * real `level_up_character` Tauri command — including a recorded hit-die
 * choice — and refreshes the sheet on success, or surfaces the engine's
 * real diagnostics if the resulting build doesn't reach `Computed`.
 */

export function LevelUpDialog(props: {
  open: boolean;
  onClose: () => void;
  /** Needed to ask the engine what the next level actually grants. */
  characterId: string;
  heldClasses: HeldClass[];
  intelligenceModifier: number;
  isHuman: boolean;
  onAccept: (classId: string) => void;
}) {
  const [classId, setClassId] = useState<string>('');
  /**
   * The engine's grants for the currently selected class. `null` while the
   * request is in flight or after it failed — rendered as an explicit
   * "not available" line rather than as "this level grants nothing", which
   * would be a different and false claim.
   */
  const [enginePlan, setEnginePlan] = useState<PreviewLevelUpResponse | null>(null);
  const [enginePlanFailed, setEnginePlanFailed] = useState(false);
  /**
   * The feat-option list is the whole eligible pool for this character, which
   * is hundreds of records — a search box is what makes it usable, not a
   * decoration. Purely local: the backend already did the rules filtering.
   */
  const [featSearch, setFeatSearch] = useState('');
  /**
   * Refused options are collapsed by default and shown on request, never
   * dropped: a player who cannot see why an option is missing cannot plan
   * toward it.
   */
  const [showRefusedFeats, setShowRefusedFeats] = useState(false);

  const catalog = useClassCatalog();
  /** The engine's answer for this character; `null` while loading. */
  const [classOptions, setClassOptions] = useState<LevelUpClassOptionsResponse | null>(null);

  useEffect(() => {
    if (!props.open) {
      return;
    }
    let live = true;
    setClassOptions(null);
    void ensureClassRosterLoaded();
    loadLevelUpClassOptions(props.characterId)
      .then((response) => {
        if (live) {
          setClassOptions(response);
        }
      })
      .catch((cause: unknown) => {
        if (live) {
          setClassOptions(
            fallbackLevelUpResponse(props.heldClasses, catalog.options, cause instanceof Error ? cause.message : String(cause))
          );
        }
      });
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        props.onClose();
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => {
      live = false;
      window.removeEventListener('keydown', onKeyDown);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [props.open, props.characterId]);

  const groups = classOptions ? levelUpChoiceGroups(classOptions) : [];
  const choices = groups.flatMap((group) => group.choices);

  // Default to the first held class that can advance, else the first offered class.
  useEffect(() => {
    setClassId((current) => {
      if (current && choices.some((choice) => choice.classId === current)) {
        return current;
      }
      return choices[0]?.classId ?? '';
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [classOptions]);

  useEffect(() => {
    if (!props.open || !classId) {
      setEnginePlan(null);
      setEnginePlanFailed(false);
      return;
    }
    let cancelled = false;
    setEnginePlan(null);
    setEnginePlanFailed(false);
    previewLevelUpGrants({ characterId: props.characterId, classId })
      .then((plan) => {
        if (!cancelled) {
          setEnginePlan(plan);
        }
      })
      .catch(() => {
        if (!cancelled) {
          setEnginePlanFailed(true);
        }
      });
    return () => {
      cancelled = true;
    };
  }, [props.open, props.characterId, classId]);

  if (!props.open) {
    return null;
  }

  const selectedChoice = choices.find((choice) => choice.classId === classId) ?? null;
  const preview = selectedChoice ? previewLevelUp(props.heldClasses, classId) : null;
  const skillPoints = preview ? totalSkillPoints(preview.skillPointsBase, props.intelligenceModifier, props.isHuman) : null;
  const hitDie = selectedChoice ? knownClass(classId)?.hitDie ?? null : null;
  const isNewClass = !props.heldClasses.some((held) => held.classId === classId);

  return createPortal(
    <div
      role="presentation"
      onClick={props.onClose}
      style={{
        alignItems: 'center',
        backgroundColor: 'rgba(0, 0, 0, 0.6)',
        display: 'flex',
        inset: 0,
        justifyContent: 'center',
        padding: '2rem',
        position: 'fixed',
        zIndex: 1100,
      }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Level up"
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 12,
          boxShadow: '0 24px 60px rgba(0, 0, 0, 0.55)',
          display: 'flex',
          flexDirection: 'column',
          width: 'min(480px, 94vw)',
        }}
      >
        <header
          style={{
            alignItems: 'center',
            borderBottom: '1px solid var(--color-border)',
            display: 'flex',
            justifyContent: 'space-between',
            padding: '1rem 1.5rem',
          }}
        >
          <h2 style={{ fontSize: '1.1rem', margin: 0 }}>Level up</h2>
          <button
            type="button"
            aria-label="Close"
            onClick={props.onClose}
            style={{ background: 'none', border: 'none', color: 'var(--color-text-muted)', cursor: 'pointer', fontSize: '1.4rem', lineHeight: 1, padding: '0.15rem 0.35rem' }}
          >
            ×
          </button>
        </header>

        <div style={{ padding: '1.25rem 1.5rem' }}>
          <label style={{ color: 'var(--color-text-muted)', display: 'block', fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.04em', marginBottom: '0.35rem', textTransform: 'uppercase' }}>
            Class
          </label>
          {classOptions === null ? (
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: 0 }}>Reading the classes you can take from the rules engine&hellip;</p>
          ) : (
            <>
              {classOptions.diagnostics.length > 0 ? (
                <ul role="alert" style={{ color: 'var(--color-warn)', fontSize: '0.75rem', margin: '0 0 0.5rem', paddingLeft: '1.1rem' }}>
                  {classOptions.diagnostics.map((line) => (
                    <li key={line}>{line}</li>
                  ))}
                </ul>
              ) : null}
              {choices.length === 0 ? (
                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: 0 }}>
                  {classOptions.atLevelCap
                    ? `Character level ${classOptions.characterLevel} is the level cap of ${classOptions.levelCap}.`
                    : 'No class can be taken at the next level.'}
                </p>
              ) : (
                <select
                  aria-label="Class to take the next level in"
                  value={classId}
                  onChange={(event) => setClassId(event.target.value)}
                  style={{
                    backgroundColor: 'var(--color-surface-2)',
                    border: '1px solid var(--color-border)',
                    borderRadius: 6,
                    boxSizing: 'border-box',
                    color: 'var(--color-text)',
                    padding: '0.5rem 0.6rem',
                    width: '100%',
                  }}
                >
                  {groups
                    .filter((group) => group.choices.length > 0)
                    .map((group) => (
                      <optgroup key={group.kind} label={`${group.heading} (${group.choices.length})`}>
                        {group.choices.map((choice) => (
                          <option key={`${group.kind}:${choice.classId}`} value={choice.classId}>
                            {choice.label}
                            {choice.kind === 'advance'
                              ? ` (currently ${choice.currentLevel} → ${choice.nextLevel})`
                              : choice.kind === 'add_base'
                                ? ' (new class)'
                                : choice.requirementsAllMet
                                  ? ' (prestige — requirements met)'
                                  : ' (prestige — requirements not all met)'}
                          </option>
                        ))}
                      </optgroup>
                    ))}
                </select>
              )}
            </>
          )}

          {/*
            A prestige class's entry requirements, in the rule's words, each with its note for
            this character. Printed, never a gate (§9.2): Accept stays enabled.
          */}
          {selectedChoice && selectedChoice.kind === 'add_prestige' ? (
            <div style={{ marginTop: '0.6rem' }}>
              <p style={{ color: 'var(--color-accent)', fontSize: '0.8rem', fontWeight: 700, margin: '0 0 0.25rem' }}>
                Entry requirements{selectedChoice.requirementsAllMet ? ' — all met' : ' — not all met (you may still take it)'}
              </p>
              {selectedChoice.requirements.length === 0 ? (
                <p style={{ color: 'var(--color-text-faint)', fontSize: '0.75rem', margin: 0 }}>
                  The converted record states no entry requirement for this class.
                </p>
              ) : (
                <ul aria-label="Entry requirements" style={{ margin: 0, paddingLeft: '1.1rem' }}>
                  {selectedChoice.requirements.map((requirement) => (
                    <li
                      key={requirement.text}
                      data-requirement-status={requirement.status}
                      style={{
                        color: requirement.status === 'unmet' ? 'var(--color-warn)' : 'var(--color-text-secondary)',
                        fontSize: '0.78rem',
                        marginBottom: '0.1rem',
                      }}
                    >
                      {describeEntryRequirement(requirement)}
                    </li>
                  ))}
                </ul>
              )}
            </div>
          ) : null}

          {preview ? (
            <div
              style={{
                backgroundColor: 'var(--color-surface-2)',
                border: '1px solid var(--color-border)',
                borderRadius: 8,
                marginTop: '1rem',
                padding: '0.7rem 0.85rem',
              }}
            >
              <p style={{ color: 'var(--color-accent)', fontSize: '0.85rem', fontWeight: 800, margin: 0 }}>
                {isNewClass ? `${preview.classLabel} 1 (new class)` : `${preview.classLabel} ${preview.classLevel}`} — character level {preview.characterLevel}
              </p>
              <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.2rem 0 0.5rem' }}>
                Hit die: {hitDie === null ? 'Unknown' : `d${hitDie}`} · Skill points: {skillPoints === null ? 'Unknown' : skillPoints}
              </p>
              <ul style={{ margin: 0, paddingLeft: '1.1rem' }}>
                {preview.features.map((feature) => (
                  <li key={feature} style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem', marginBottom: '0.15rem' }}>
                    {feature}
                  </li>
                ))}
                {/*
                  The engine's real grants for this exact transition. Each
                  carries the engine's own name and effect descriptions,
                  verbatim — never a hand-authored label.
                */}
                {enginePlan?.automaticFeatures.map((grant) => (
                  <li key={grant.name} style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem', marginBottom: '0.15rem' }}>
                    {grant.name}
                    {grant.effects.map((effect) => (
                      <span key={effect.description} style={{ color: 'var(--color-text-faint)', display: 'block', fontSize: '0.72rem' }}>
                        {effect.description}
                      </span>
                    ))}
                  </li>
                ))}
                {enginePlan?.pickFromLists.map((list) => (
                  <li key={`${list.category}-${list.count}`} style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem', marginBottom: '0.15rem' }}>
                    Pick {list.count} {list.category}
                    {list.filter ? ` — ${list.filter}` : ''}
                  </li>
                ))}
                {enginePlan?.resourcePoolChanges.map((pool) => (
                  <li key={pool.poolId} style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem', marginBottom: '0.15rem' }}>
                    {pool.poolId}: {pool.fromValue} &rarr; {pool.toValue}
                  </li>
                ))}
                {enginePlan?.capstoneThreshold ? (
                  <li style={{ color: 'var(--color-accent)', fontSize: '0.85rem', marginBottom: '0.15rem' }}>
                    Capstone level
                  </li>
                ) : null}
              </ul>

              {/*
                SD-35 AT-35-E5-004 — the per-character choice filter. The
                backend joined the feat pool against THIS character's own
                prerequisites (`SheetRule.applies`); this list renders that
                answer and re-derives nothing. The refused half is shown on
                request with the requirement each option failed, in the
                engine's own words.
              */}
              {enginePlan && enginePlan.optionFilterUnavailableReason === null ? (
                <div style={{ borderTop: '1px solid var(--color-border)', marginTop: '0.6rem', paddingTop: '0.6rem' }}>
                  <p style={{ color: 'var(--color-accent)', fontSize: '0.8rem', fontWeight: 700, margin: '0 0 0.35rem' }}>
                    Feats you qualify for — {enginePlan.featOptions.length} of{' '}
                    {enginePlan.featOptions.length + enginePlan.refusedFeatOptions.length}
                  </p>
                  <label htmlFor="level-up-feat-search" style={{ color: 'var(--color-text-muted)', display: 'block', fontSize: '0.72rem', marginBottom: '0.2rem' }}>
                    Search feats
                  </label>
                  <input
                    id="level-up-feat-search"
                    type="search"
                    value={featSearch}
                    onChange={(event) => setFeatSearch(event.target.value)}
                    style={{
                      backgroundColor: 'var(--color-surface)',
                      border: '1px solid var(--color-border)',
                      borderRadius: 6,
                      color: 'var(--color-text)',
                      fontSize: '0.8rem',
                      padding: '0.3rem 0.5rem',
                      width: '100%',
                    }}
                  />
                  <ul style={{ margin: '0.4rem 0 0', maxHeight: 180, overflowY: 'auto', paddingLeft: '1.1rem' }}>
                    {enginePlan.featOptions
                      .filter((option) => option.name.toLowerCase().includes(featSearch.trim().toLowerCase()))
                      .map((option) => (
                        <li key={option.id} style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem', marginBottom: '0.1rem' }}>
                          {option.name}
                          {option.condition ? (
                            <span style={{ color: 'var(--color-text-faint)', fontSize: '0.7rem' }}> — {option.condition}</span>
                          ) : null}
                        </li>
                      ))}
                  </ul>
                  {enginePlan.refusedFeatOptions.length > 0 ? (
                    <>
                      <button
                        type="button"
                        onClick={() => setShowRefusedFeats((shown) => !shown)}
                        style={{
                          background: 'none',
                          border: 'none',
                          color: 'var(--color-accent)',
                          cursor: 'pointer',
                          fontSize: '0.72rem',
                          padding: '0.35rem 0 0',
                        }}
                      >
                        {showRefusedFeats ? 'Hide' : 'Show'} the {enginePlan.refusedFeatOptions.length} you do not
                        yet qualify for
                      </button>
                      {showRefusedFeats ? (
                        <ul style={{ margin: '0.25rem 0 0', maxHeight: 180, overflowY: 'auto', paddingLeft: '1.1rem' }}>
                          {enginePlan.refusedFeatOptions
                            .filter((option) => option.name.toLowerCase().includes(featSearch.trim().toLowerCase()))
                            .map((option) => (
                              <li key={option.id} style={{ color: 'var(--color-text-faint)', fontSize: '0.78rem', marginBottom: '0.1rem' }}>
                                {option.name} — {option.unmet}
                              </li>
                            ))}
                        </ul>
                      ) : null}
                    </>
                  ) : null}
                </div>
              ) : enginePlan ? (
                <p style={{ color: 'var(--color-text-faint)', fontSize: '0.72rem', margin: '0.35rem 0 0' }}>
                  Feat options could not be read from the rules package: {enginePlan.optionFilterUnavailableReason}
                </p>
              ) : null}

              {/*
                Three genuinely different states, kept apart. "Not yet
                loaded" and "could not be read" are not the same as "the
                engine grounds no class features for this transition", and
                only the last of the three is a statement about the rules.
              */}
              {enginePlanFailed ? (
                <p style={{ color: 'var(--color-text-faint)', fontSize: '0.72rem', margin: '0.35rem 0 0' }}>
                  Class features for this level could not be read from the rules engine.
                </p>
              ) : enginePlan === null ? (
                <p style={{ color: 'var(--color-text-faint)', fontSize: '0.72rem', margin: '0.35rem 0 0' }}>
                  Reading class features from the rules engine&hellip;
                </p>
              ) : enginePlan.automaticFeatures.length === 0 &&
                enginePlan.pickFromLists.length === 0 &&
                enginePlan.resourcePoolChanges.length === 0 ? (
                <p style={{ color: 'var(--color-text-faint)', fontSize: '0.72rem', margin: '0.35rem 0 0' }}>
                  The rules engine grounds no class features for this class at this level.
                </p>
              ) : null}
            </div>
          ) : null}
        </div>

        <footer style={{ borderTop: '1px solid var(--color-border)', display: 'flex', gap: '0.6rem', justifyContent: 'flex-end', padding: '0.85rem 1.5rem' }}>
          <button
            type="button"
            onClick={props.onClose}
            style={{
              backgroundColor: 'var(--color-surface-2)',
              border: '1px solid var(--color-border)',
              borderRadius: 6,
              color: 'var(--color-text)',
              cursor: 'pointer',
              fontSize: '0.85rem',
              fontWeight: 600,
              padding: '0.45rem 0.9rem',
            }}
          >
            Cancel
          </button>
          <button
            type="button"
            onClick={() => {
              props.onAccept(classId);
              props.onClose();
            }}
            disabled={!classId}
            style={{
              backgroundColor: 'var(--color-accent)',
              border: '1px solid var(--color-border)',
              borderRadius: 6,
              color: 'var(--color-on-accent)',
              cursor: 'pointer',
              fontSize: '0.85rem',
              fontWeight: 600,
              opacity: classId ? 1 : 0.6,
              padding: '0.45rem 0.9rem',
            }}
          >
            Accept
          </button>
        </footer>
      </div>
    </div>,
    document.body
  );
}

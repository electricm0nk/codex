import { useEffect, useState, type CSSProperties, type FormEvent, type ReactNode } from 'react';
import {
  ABILITY_ABBREVIATIONS,
  ABILITY_KEYS,
  ALIGNMENT_OPTIONS,
  AGE_OPTIONS,
  CLASS_OPTIONS,
  DEFAULT_ABILITY_SCORES,
  abilityModifier,
  ageEffectForAbility,
  clampLevelForClass,
  classSupportLevelSuffix,
  describeClassSupportLevel,
  formatHeight,
  getLevelOptionsForClass,
  rollDice,
  type AbilityKey,
  type AgeCategory,
  type BodyProfile,
  type RaceOption,
  type Sex,
} from './characterHubModel';
import {
  applyFloatingAbilityAllocation,
  applyRacialAbilityAdjustments,
  composeCreateCharacterRequest,
} from './composeCreateCharacterRequest';
import { loadRaceRosterSurface, type RaceRosterSurface } from './raceRoster';
import { createCharacterRuntime } from './characterHubRuntime';
import {
  buildAlternateTraitRows,
  creationSelectionWarnings,
  describeCreationSelection,
  retainSelectionsValidForRace,
} from './alternateTraitSelection';
import {
  loadAlternateRacialTraitsRuntime,
  resolveRaceAlternateSelectionRuntime,
} from '../raceCatalog/alternateTraitPickerRuntime';
import type {
  AlternateRacialTraitsResponse,
  RaceSelectionResponse,
} from '../boundary/loadAlternateRacialTraits';
import { loadCharacterTraits, type CharacterTraitOptionDto } from '../boundary/loadCharacterTraits';
import type { CreateCharacterOutcomeSurface } from './buildCreateCharacterOutcomeSurface';
import {
  ABILITY_SCORE_METHOD_OPTIONS,
  POINT_BUY_DEFAULT_POOL,
  POINT_BUY_DEFAULT_SCORE,
  POINT_BUY_MAX_SCORE,
  POINT_BUY_MIN_SCORE,
  POINT_BUY_POOL_PRESETS,
  abilityScoreMethodOption,
  generateAbilityScorePool,
  pointBuyCost,
  rollStraightAbilityScores,
  type AbilityScoreMethodId,
} from './abilityScoreMethods';
import { maxHitPoints } from './characterProgression';

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

/**
 * Every character starts at level 1 unless the player raises the Level
 * picker. This is the starting value, not a cap — the cap is per class, and
 * comes from `getLevelOptionsForClass`.
 */
const STARTING_LEVEL = 1;

type Allocation = Record<AbilityKey, number>;
const ZERO_ALLOCATION: Allocation = {
  strength: 0,
  dexterity: 0,
  constitution: 0,
  intelligence: 0,
  wisdom: 0,
  charisma: 0,
};

type PoolAssignment = Record<AbilityKey, number | null>;
const EMPTY_POOL_ASSIGNMENT: PoolAssignment = {
  strength: null,
  dexterity: null,
  constitution: null,
  intelligence: null,
  wisdom: null,
  charisma: null,
};

const POINT_BUY_BASE_SCORES: Record<AbilityKey, number> = {
  strength: POINT_BUY_DEFAULT_SCORE,
  dexterity: POINT_BUY_DEFAULT_SCORE,
  constitution: POINT_BUY_DEFAULT_SCORE,
  intelligence: POINT_BUY_DEFAULT_SCORE,
  wisdom: POINT_BUY_DEFAULT_SCORE,
  charisma: POINT_BUY_DEFAULT_SCORE,
};

/**
 * `null` for a race this repo carries no height/weight profile for. The
 * corpus carries one for no race at all (PCGen keeps them in
 * `<race>_biosettings.lst`, which no book's ingest reads), and the seven
 * hand-entered profiles that ship are not extended by guesswork — see
 * `RACE_BODY_PROFILES`. The form prints the absence instead of a number.
 */
function rollHeight(body: BodyProfile | null): number | null {
  return body === null ? null : body.baseHeightInches + rollDice(body.heightModDice.count, body.heightModDice.sides);
}

function rollWeight(body: BodyProfile | null): number | null {
  return body === null
    ? null
    : body.baseWeightLb + rollDice(body.heightModDice.count, body.heightModDice.sides) * body.weightMultiplierLb;
}

/** What a read-only physical field shows when this repo has no profile behind it. */
const NO_BODY_PROFILE = 'No height/weight profile';

/**
 * Loads the corpus-derived race roster, then renders the real form.
 *
 * The roster is served by the `list_race_creation_roster` command out of
 * `data/corpus/<book>/race` and `race_trait` — 18 races across the Core Rulebook and
 * Bestiary 1, where this form previously offered a hardcoded 7. It is
 * fetched rather than compiled in for the reason spelled out in
 * `raceRoster.ts`: the identical hand-maintained table one layer down
 * silently drifted from the corpus on four races' ability modifiers.
 *
 * There is no sample-data fallback. Creation already requires the desktop
 * backend (`loadCreateCharacter` throws without it), so a preview roster
 * would be a picker whose every choice fails at submit.
 */
export function CreateCharacterForm(props: { onCreated: () => void }) {
  const [roster, setRoster] = useState<RaceRosterSurface | null>(null);
  const [rosterError, setRosterError] = useState<string | null>(null);

  useEffect(() => {
    let live = true;
    loadRaceRosterSurface()
      .then((surface) => {
        if (!live) {
          return;
        }
        if (surface.options.length === 0) {
          setRosterError(
            surface.diagnostics.length > 0
              ? `No race could be read from the corpus: ${surface.diagnostics.join('; ')}`
              : 'No race could be read from the corpus.'
          );
          return;
        }
        setRoster(surface);
      })
      .catch((cause: unknown) => {
        if (live) {
          setRosterError(cause instanceof Error ? cause.message : String(cause));
        }
      });
    return () => {
      live = false;
    };
  }, []);

  if (rosterError !== null) {
    return (
      <div style={{ border: '1px solid var(--color-border)', borderRadius: 12, padding: '1.25rem' }}>
        <p style={{ color: 'var(--color-danger, #c0392b)', margin: 0 }}>{rosterError}</p>
      </div>
    );
  }
  if (roster === null) {
    return (
      <div style={{ border: '1px solid var(--color-border)', borderRadius: 12, padding: '1.25rem' }}>
        <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>Loading races from the corpus…</p>
      </div>
    );
  }
  return <CreateCharacterFields races={roster.options} rosterDiagnostics={roster.diagnostics} onCreated={props.onCreated} />;
}

function CreateCharacterFields(props: {
  races: RaceOption[];
  rosterDiagnostics: string[];
  onCreated: () => void;
}) {
  const races = props.races;
  const [displayLabel, setDisplayLabel] = useState('');
  const [playerName, setPlayerName] = useState('');
  const [raceId, setRaceId] = useState(races[0].id);
  const [classId, setClassId] = useState(CLASS_OPTIONS[0].id);
  const [level, setLevel] = useState(STARTING_LEVEL);
  const [abilityScores, setAbilityScores] = useState({ ...DEFAULT_ABILITY_SCORES });
  const [allocation, setAllocation] = useState<Allocation>({ ...ZERO_ALLOCATION });
  const [method, setMethod] = useState<AbilityScoreMethodId>('manual');
  const [pool, setPool] = useState<number[]>([]);
  const [poolAssignment, setPoolAssignment] = useState<PoolAssignment>({ ...EMPTY_POOL_ASSIGNMENT });
  const [pointBuyPool, setPointBuyPool] = useState(POINT_BUY_DEFAULT_POOL);
  const [alignment, setAlignment] = useState<string>(ALIGNMENT_OPTIONS[4]); // True Neutral
  const [deity, setDeity] = useState('');
  const [sex, setSex] = useState<Sex>('male');
  const [age, setAge] = useState<AgeCategory>('Adult');
  const [eyes, setEyes] = useState('');
  const [hair, setHair] = useState('');
  const [heightInches, setHeightInches] = useState(() => rollHeight(races[0].body?.male ?? null));
  const [weightLb, setWeightLb] = useState(() => rollWeight(races[0].body?.male ?? null));
  const [submitting, setSubmitting] = useState(false);
  const [outcome, setOutcome] = useState<CreateCharacterOutcomeSurface | null>(null);
  const [error, setError] = useState<string | null>(null);
  // SD-27: ARG's alternate racial traits, taken at creation. The menu is the
  // same `race_trait_picker` payload the Race Traits screen browses; the live
  // resolution is the same `RaceCorpus::resolve` call. Nothing about which
  // trait replaces what, or which pairs are illegal, is decided here.
  const [alternateMenu, setAlternateMenu] = useState<AlternateRacialTraitsResponse | null>(null);
  const [alternateMenuError, setAlternateMenuError] = useState<string | null>(null);
  const [selectedAlternateTraitKeys, setSelectedAlternateTraitKeys] = useState<string[]>([]);
  const [alternateResolution, setAlternateResolution] = useState<RaceSelectionResponse | null>(null);
  // AT-34-E4-002: character traits/drawbacks, taken at creation. Real, real
  // computed skill bonuses (`trait_effects::skill_bonuses_from_traits`), for
  // exactly the `ultimate_campaign` traits `list_available_character_traits`
  // returns -- no other trait shape is offered here, because no other shape
  // computes anything yet.
  const [traitOptions, setTraitOptions] = useState<CharacterTraitOptionDto[] | null>(null);
  const [traitOptionsError, setTraitOptionsError] = useState<string | null>(null);
  const [selectedTraits, setSelectedTraits] = useState<string[]>([]);
  // AT-34-E4-002 (second slice): the player's resolved skill choice for
  // each selected fixed-choice `%LIST` trait, keyed by trait id. A trait
  // with no entry here yet (just checked, choice not made) submits no
  // `traitSkillChoices` entry for it -- `skill_choice_bonuses_from_traits`
  // honestly contributes nothing for a trait with no recorded choice,
  // never a first-guessed default (see that function's own doc comment).
  const [traitSkillChoices, setTraitSkillChoices] = useState<Record<string, string>>({});

  const selectedClass = CLASS_OPTIONS.find((option) => option.id === classId) ?? CLASS_OPTIONS[0];
  const selectedRace = races.find((option) => option.id === raceId) ?? races[0];
  const body = selectedRace.body?.[sex] ?? null;

  const allocatedPoints = ABILITY_KEYS.reduce((sum, key) => sum + allocation[key], 0);
  const remainingPoints = selectedRace.floatingBonusPoints - allocatedPoints;

  const methodOption = abilityScoreMethodOption(method);
  const pointBuySpent = ABILITY_KEYS.reduce((sum, key) => sum + pointBuyCost(abilityScores[key]), 0);
  const pointBuyRemaining = pointBuyPool - pointBuySpent;
  const unassignedPoolSlots = methodOption.kind === 'pool' ? ABILITY_KEYS.filter((key) => poolAssignment[key] == null).length : 0;

  /** The raw score feeding `calculatedScore`/submission — from `abilityScores` for every
   * kind except `pool`, where the source of truth is the generated pool + per-ability assignment. */
  function rawScore(key: AbilityKey): number {
    if (methodOption.kind === 'pool') {
      const index = poolAssignment[key];
      return index == null ? 0 : (pool[index] ?? 0);
    }
    return abilityScores[key];
  }

  function calculatedScore(key: AbilityKey): number {
    return rawScore(key) + (selectedRace.abilityAdjustments[key] ?? 0) + allocation[key] + ageEffectForAbility(age, key);
  }

  const levelOptions = getLevelOptionsForClass(classId);

  // Shares `maxHitPoints` with the character sheet rather than a level-1-only
  // shortcut, so the HP shown here matches what the sheet will show for the
  // level actually being created (PF1: max hit die at 1st, average after).
  const maxHp = maxHitPoints(
    [{ classId, classLabel: selectedClass.label, level }],
    abilityModifier(calculatedScore('constitution'))
  );

  /**
   * Selecting a class can strand a level that class does not offer (Fighter 20
   * → Monk, whose only offered level is 1), so the level is re-clamped here
   * rather than left to fail at submit time.
   */
  function handleClassChange(nextClassId: string) {
    setClassId(nextClassId);
    setLevel((current) => clampLevelForClass(nextClassId, current));
  }

  function handleMethodChange(nextMethod: AbilityScoreMethodId) {
    setMethod(nextMethod);
    const nextOption = abilityScoreMethodOption(nextMethod);
    if (nextOption.kind === 'pool') {
      setPool(generateAbilityScorePool(nextMethod));
      setPoolAssignment({ ...EMPTY_POOL_ASSIGNMENT });
    } else if (nextOption.kind === 'straight') {
      setAbilityScores(rollStraightAbilityScores());
    } else if (nextOption.kind === 'pointBuy') {
      setAbilityScores({ ...POINT_BUY_BASE_SCORES });
      setPointBuyPool(POINT_BUY_DEFAULT_POOL);
    }
  }

  function handleReroll() {
    if (methodOption.kind === 'pool') {
      setPool(generateAbilityScorePool(method));
      setPoolAssignment({ ...EMPTY_POOL_ASSIGNMENT });
    } else if (methodOption.kind === 'straight') {
      setAbilityScores(rollStraightAbilityScores());
    }
  }

  function assignPoolValue(key: AbilityKey, index: number | null) {
    setPoolAssignment((prev) => ({ ...prev, [key]: index }));
  }

  function adjustPointBuyScore(key: AbilityKey, delta: 1 | -1) {
    setAbilityScores((prev) => {
      const current = prev[key];
      const next = current + delta;
      if (next < POINT_BUY_MIN_SCORE || next > POINT_BUY_MAX_SCORE) {
        return prev;
      }
      const spent = ABILITY_KEYS.reduce((sum, k) => sum + pointBuyCost(k === key ? next : prev[k]), 0);
      if (spent > pointBuyPool) {
        return prev;
      }
      return { ...prev, [key]: next };
    });
  }

  function reroll(nextRace: RaceOption, nextSex: Sex) {
    const nextBody = nextRace.body?.[nextSex] ?? null;
    setHeightInches(rollHeight(nextBody));
    setWeightLb(rollWeight(nextBody));
  }

  // The alternate-racial-trait menu, loaded once. A failure is shown rather
  // than swallowed: the rest of the form still works, and the player is told
  // why the trait list is absent instead of concluding this race has none.
  useEffect(() => {
    let live = true;
    loadAlternateRacialTraitsRuntime()
      .then((menu) => {
        if (live) {
          setAlternateMenu(menu);
        }
      })
      .catch((cause: unknown) => {
        if (live) {
          setAlternateMenuError(cause instanceof Error ? cause.message : String(cause));
        }
      });
    return () => {
      live = false;
    };
  }, []);

  // Every change of race or selection is re-resolved by the engine, so the
  // "replaces X" line and the mutual lock-out the player sees are the
  // resolver's own answers for this exact selection — never a frontend guess.
  useEffect(() => {
    let live = true;
    const raceKey = raceId.replace(/^race:/, '');
    resolveRaceAlternateSelectionRuntime(raceKey, selectedAlternateTraitKeys)
      .then((resolved) => {
        if (live) {
          setAlternateResolution(resolved);
        }
      })
      .catch(() => {
        if (live) {
          setAlternateResolution(null);
        }
      });
    return () => {
      live = false;
    };
  }, [raceId, selectedAlternateTraitKeys]);

  // The character trait/drawback menu, loaded once, the same shape the
  // alternate-racial-trait menu above uses. A failure is shown rather than
  // swallowed: the rest of the form still works, and the player is told why
  // the trait list is absent instead of concluding none exist.
  useEffect(() => {
    let live = true;
    loadCharacterTraits()
      .then((options) => {
        if (live) {
          setTraitOptions(options);
        }
      })
      .catch((cause: unknown) => {
        if (live) {
          setTraitOptionsError(cause instanceof Error ? cause.message : String(cause));
        }
      });
    return () => {
      live = false;
    };
  }, []);

  function toggleTrait(id: string) {
    const wasSelected = selectedTraits.includes(id);
    setSelectedTraits((current) =>
      wasSelected ? current.filter((existing) => existing !== id) : [...current, id]
    );
    if (wasSelected) {
      // Unchecking a choice-based trait drops its recorded skill choice too
      // -- an unselected trait must never leave a stale choice behind that
      // a later re-check could silently pick back up.
      setTraitSkillChoices((current) => {
        const { [id]: _removed, ...rest } = current;
        return rest;
      });
    } else {
      // Checking a choice-based trait defaults its choice to the first
      // `skillOptions` entry, so a submit before the player touches the
      // dropdown still records a real, in-list choice rather than none at
      // all -- the option is still visibly a `<select>` the player can
      // change, this only avoids an accidentally-empty submission.
      const option = traitOptions?.find((candidate) => candidate.id === id);
      if (option !== undefined && option.skillOptions.length > 0) {
        setTraitSkillChoices((current) => ({ ...current, [id]: option.skillOptions[0]!.skillId }));
      }
    }
  }

  function setTraitSkillChoice(traitId: string, skillId: string) {
    setTraitSkillChoices((current) => ({ ...current, [traitId]: skillId }));
  }

  const alternateTraitRows = buildAlternateTraitRows(
    alternateMenu,
    raceId,
    selectedAlternateTraitKeys,
    alternateResolution
  );
  const alternateTraitWarnings = creationSelectionWarnings(alternateResolution);

  function toggleAlternateTrait(key: string) {
    setSelectedAlternateTraitKeys((current) =>
      current.includes(key) ? current.filter((existing) => existing !== key) : [...current, key]
    );
  }

  function handleRaceChange(nextRaceId: string) {
    const nextRace = races.find((option) => option.id === nextRaceId) ?? races[0];
    setRaceId(nextRaceId);
    setAllocation({ ...ZERO_ALLOCATION });
    // A Dwarf's trait cannot be carried onto an Elf: the backend would refuse
    // the save, and refusing is a worse answer than clearing the choice at the
    // moment it stops applying.
    setSelectedAlternateTraitKeys((current) =>
      retainSelectionsValidForRace(alternateMenu, nextRaceId, current)
    );
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
    if (unassignedPoolSlots > 0) {
      setError(`Assign all six generated scores to abilities before creating (${unassignedPoolSlots} remaining).`);
      return;
    }
    if (methodOption.kind === 'pointBuy' && pointBuyRemaining < 0) {
      setError(`Point buy is over budget by ${-pointBuyRemaining} points — lower a score or raise the pool before creating.`);
      return;
    }
    setSubmitting(true);
    setError(null);
    try {
      const rawAbilityScores = ABILITY_KEYS.reduce(
        (scores, key) => ({ ...scores, [key]: rawScore(key) }),
        {} as Record<AbilityKey, number>
      );
      // The raw entered/rolled scores don't yet include the race's fixed
      // ability adjustments (Elf +2 DEX/-2 CON/+2 INT etc.) — `calculatedScore`
      // applies them for the on-screen preview only. The compute engine
      // expects them baked into the submitted score for every race except
      // Human (see `applyRacialAbilityAdjustments`'s own doc comment).
      const adjustedAbilityScores = applyRacialAbilityAdjustments(rawAbilityScores, selectedRace.abilityAdjustments);
      // The freely-distributed "+2 to one ability score" points, for the
      // races the backend does not apply them for. See
      // `applyFloatingAbilityAllocation` — this is the seam that was missing
      // entirely, which cost Half-Elf and Half-Orc their +2.
      const finalAbilityScores = applyFloatingAbilityAllocation(adjustedAbilityScores, allocation, raceId);
      // AT-34-E4-002 (second slice): one `traitSkillChoices` entry per
      // selected trait that both is choice-based (`choiceSetId !== null`)
      // and has a recorded skill choice. A choice-based trait somehow
      // selected with no recorded choice yet (should not happen --
      // `toggleTrait` seeds a default the moment it is checked) is simply
      // omitted rather than sent with a fabricated skill.
      const resolvedTraitSkillChoices = selectedTraits.flatMap((traitId) => {
        const option = traitOptions?.find((candidate) => candidate.id === traitId);
        const skillId = traitSkillChoices[traitId];
        if (option?.choiceSetId == null || skillId === undefined) {
          return [];
        }
        return [{ choiceSetId: option.choiceSetId, selectionId: skillId }];
      });
      const request = composeCreateCharacterRequest(
        {
          displayLabel,
          raceId,
          classId,
          level,
          abilityScores: finalAbilityScores,
          abilityBonusTarget: deriveAbilityBonusTarget(),
          selectedAlternateTraitKeys,
          selectedTraits,
          traitSkillChoices: resolvedTraitSkillChoices,
        },
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
                {races.map((option) => (
                  <option key={option.id} value={option.id}>
                    {option.label} ({option.book})
                  </option>
                ))}
              </select>
            </div>
            <div style={{ flex: 1, minWidth: 0 }}>
              <label style={LABEL_STYLE} htmlFor="character-class">
                Class
              </label>
              <select id="character-class" style={INPUT_STYLE} value={classId} onChange={(event) => handleClassChange(event.target.value)}>
                {CLASS_OPTIONS.map((option) => (
                  <option key={option.id} value={option.id}>
                    {option.label}
                    {classSupportLevelSuffix(option.supportLevel)}
                  </option>
                ))}
              </select>
            </div>
          </div>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '-0.5rem 0 1rem' }}>
            {describeClassSupportLevel(selectedClass.supportLevel, selectedClass.label)}
          </p>

          {/* Level + HP (computed) + Alignment + Deity */}
          <div style={ROW_STYLE}>
            {/* Only the levels `getLevelOptionsForClass` reports for this
                class — i.e. exactly the levels the engine dump computes.
                A single-option select still renders (Monk), so the ceiling
                is visible rather than silently absent. */}
            <LabeledField label="Level" htmlFor="character-level" flex="0 0 96px">
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
            </LabeledField>
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

            {/* Height and weight are the one field the corpus carries for no
                race at all; only the seven hand-entered profiles exist. A
                race without one shows the absence and offers no reroll
                button, rather than a button that would roll nothing. */}
            <LabeledField label="Height">
              <ReadOnlyBox
                value={heightInches === null ? NO_BODY_PROFILE : formatHeight(heightInches)}
                action={
                  body === null ? undefined : (
                    <DiceButton label="Reroll height" onClick={() => setHeightInches(rollHeight(body))} />
                  )
                }
              />
            </LabeledField>
            <LabeledField label="Weight">
              <ReadOnlyBox
                value={weightLb === null ? NO_BODY_PROFILE : `${weightLb} lb`}
                action={
                  body === null ? undefined : (
                    <DiceButton label="Reroll weight" onClick={() => setWeightLb(rollWeight(body))} />
                  )
                }
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

          {/* Alternate racial traits, from every book
              `race_catalog::RACE_CORPUS_BOOKS` loads — whichever those are.
              This comment named ARG alone until SD-29's race-trait lane, then
              named three books, and each list went stale the moment the next
              book landed (four, at Inner Sea Races). It names none now, on
              purpose: the surface is book-agnostic and the backing list is one
              `grep RACE_CORPUS_BOOKS` away.

              Every fact rendered here comes from the backend: which traits
              exist, what each replaces, and which are locked out by the
              current selection. `create_character` re-validates the submitted
              keys against the corpus and returns `Blocked` rather than
              persisting a swap that did not happen. */}
          <p
            style={{
              ...LABEL_STYLE,
              borderTop: '1px solid var(--color-border)',
              color: 'var(--color-text)',
              fontSize: '0.95rem',
              marginTop: '0.5rem',
              paddingTop: '1rem',
            }}
          >
            Alternate Racial Traits
          </p>
          {alternateMenuError !== null ? (
            <p style={{ color: 'var(--color-danger, #c0392b)', fontSize: '0.78rem', margin: 0 }}>
              Alternate racial traits are unavailable: {alternateMenuError}
            </p>
          ) : alternateMenu === null ? (
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: 0 }}>
              Loading alternate racial traits…
            </p>
          ) : alternateTraitRows.length === 0 ? (
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: 0 }}>
              No ingested book declares an alternate racial trait for {selectedRace.label}.
            </p>
          ) : (
            <>
              <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: '0 0 0.5rem' }}>
                {describeCreationSelection(selectedAlternateTraitKeys, alternateResolution)}
              </p>
              <div
                style={{
                  border: '1px solid var(--color-border)',
                  borderRadius: 8,
                  // Each row now carries its rendered description, so the old
                  // 220px showed barely two of them.
                  maxHeight: 320,
                  overflowY: 'auto',
                  padding: '0.35rem 0.5rem',
                }}
              >
                {alternateTraitRows.map((row) => (
                  <label
                    key={row.alternate.key}
                    style={{
                      alignItems: 'flex-start',
                      cursor: row.disabledReason === null ? 'pointer' : 'not-allowed',
                      display: 'flex',
                      gap: '0.5rem',
                      opacity: row.disabledReason === null ? 1 : 0.55,
                      padding: '0.3rem 0',
                    }}
                    title={row.disabledReason ?? row.description}
                  >
                    <input
                      type="checkbox"
                      checked={row.selected}
                      disabled={row.disabledReason !== null}
                      onChange={() => toggleAlternateTrait(row.alternate.key)}
                      style={{ marginTop: '0.2rem' }}
                    />
                    <span style={{ minWidth: 0 }}>
                      <span style={{ fontSize: '0.85rem', fontWeight: 600 }}>{row.alternate.name}</span>
                      <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem' }}>
                        {' '}
                        · {row.alternate.book}
                        {row.alternate.sourcePage === null ? '' : ` ${row.alternate.sourcePage}`}
                      </span>
                      {/* What the trait actually does, with its numbers.

                          This row used to show a name, a page and "Replaces X"
                          — three facts, none of them a magnitude — while the
                          rendered sentence stating the number sat in the same
                          payload, reaching only a hover tooltip. A player
                          choosing between two alternates could not compare
                          them. `description` is rendered verbatim: it is
                          corpus prose with the engine's own numbers resolved
                          into it (`decisions.md §29.1`). */}
                      <span
                        style={{
                          color: 'var(--color-text-secondary)',
                          display: 'block',
                          fontSize: '0.72rem',
                        }}
                      >
                        {row.description}
                      </span>
                      {row.droppedArgs.length > 0 ? (
                        <span
                          style={{ color: 'var(--color-text-muted)', display: 'block', fontSize: '0.7rem' }}
                        >
                          The engine could not resolve {row.droppedArgs.join(', ')}, so this description is
                          incomplete.
                        </span>
                      ) : null}
                      <span
                        style={{
                          color: 'var(--color-text-muted)',
                          display: 'block',
                          fontSize: '0.72rem',
                        }}
                      >
                        {row.disabledReason ??
                          (row.alternate.replaces.length > 0
                            ? `Replaces ${row.alternate.replaces.map((link) => link.name).join(', ')}`
                            : `Replaces nothing in the loaded books (${row.alternate.setsFlags.join(', ')})`)}
                      </span>
                    </span>
                  </label>
                ))}
              </div>
              {alternateTraitWarnings.map((warning) => (
                <p
                  key={warning}
                  style={{ color: 'var(--color-danger, #c0392b)', fontSize: '0.75rem', margin: '0.4rem 0 0' }}
                >
                  {warning}
                </p>
              ))}
            </>
          )}

          {/* AT-34-E4-002: character traits/drawbacks. Every option here
              genuinely computes -- `list_available_character_traits` returns
              only the 52 `ultimate_campaign` traits whose `BONUS:SKILL`,
              `BONUS:SAVE`, `BONUS:SITUATION`, `BONUS:COMBAT|INITIATIVE`/
              `BONUS:CONCENTRATION|ALLSPELLS`, or ability-score-difference
              formula this crate's `trait_effects` compute paths really
              apply (31 flat skill + 5 fixed-choice skill + 4 open-family
              skill + 2 flat save + 3 situational + 3 initiative/
              concentration + 4 ability-substitution). No wider trait roster
              is offered, because no wider roster computes anything yet. */}
          <p
            style={{
              ...LABEL_STYLE,
              borderTop: '1px solid var(--color-border)',
              color: 'var(--color-text)',
              fontSize: '0.95rem',
              marginTop: '0.5rem',
              paddingTop: '1rem',
            }}
          >
            Traits
          </p>
          {traitOptionsError !== null ? (
            <p style={{ color: 'var(--color-danger, #c0392b)', fontSize: '0.78rem', margin: 0 }}>
              Traits are unavailable: {traitOptionsError}
            </p>
          ) : traitOptions === null ? (
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: 0 }}>Loading traits…</p>
          ) : (
            <div
              style={{
                border: '1px solid var(--color-border)',
                borderRadius: 8,
                maxHeight: 320,
                overflowY: 'auto',
                padding: '0.35rem 0.5rem',
              }}
            >
              {traitOptions.map((option) => {
                const isChoiceBased = option.skillOptions.length > 0;
                const isSelected = selectedTraits.includes(option.id);
                return (
                  <div key={option.id} style={{ padding: '0.3rem 0' }}>
                    <label
                      style={{
                        alignItems: 'flex-start',
                        cursor: 'pointer',
                        display: 'flex',
                        gap: '0.5rem',
                      }}
                      title={option.description}
                    >
                      <input
                        type="checkbox"
                        checked={isSelected}
                        onChange={() => toggleTrait(option.id)}
                        style={{ marginTop: '0.2rem' }}
                      />
                      <span style={{ minWidth: 0 }}>
                        <span style={{ fontSize: '0.85rem', fontWeight: 600 }}>{option.name}</span>
                        <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem' }}>
                          {' '}
                          ·{' '}
                          {option.otherPillars.length > 0
                            ? option.otherPillars
                                .map((pillar) => `${pillar.bonus >= 0 ? `+${pillar.bonus}` : pillar.bonus} ${pillar.label}`)
                                .join(', ')
                            : option.abilitySubstitution !== null
                              ? `${option.skills.join(', ')} (ability-based${
                                  option.abilitySubstitution.flatBonus !== 0
                                    ? `, +${option.abilitySubstitution.flatBonus} flat`
                                    : ''
                                })`
                              : `${option.bonus >= 0 ? `+${option.bonus}` : option.bonus} ${
                                  isChoiceBased
                                    ? `choice of ${option.skillOptions.map((choice) => choice.name).join(', ')}`
                                    : option.save !== null
                                      ? `${option.save} save`
                                      : option.skills.join(', ')
                                }`}
                        </span>
                        <span
                          style={{
                            color: 'var(--color-text-secondary)',
                            display: 'block',
                            fontSize: '0.72rem',
                          }}
                        >
                          {option.description}
                        </span>
                      </span>
                    </label>
                    {isChoiceBased && isSelected ? (
                      <select
                        aria-label={`${option.name} skill choice`}
                        value={traitSkillChoices[option.id] ?? option.skillOptions[0]!.skillId}
                        onChange={(event) => setTraitSkillChoice(option.id, event.target.value)}
                        style={{ fontSize: '0.78rem', marginLeft: '1.6rem', marginTop: '0.25rem' }}
                      >
                        {option.skillOptions.map((choice) => (
                          <option key={choice.skillId} value={choice.skillId}>
                            {choice.name}
                          </option>
                        ))}
                      </select>
                    ) : null}
                  </div>
                );
              })}
            </div>
          )}
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
          <p style={{ ...LABEL_STYLE, marginBottom: '0.35rem' }}>Ability scores</p>

          <div style={{ marginBottom: '0.5rem' }}>
            <label style={LABEL_STYLE} htmlFor="ability-score-method">
              Generation method
            </label>
            <div style={{ alignItems: 'center', display: 'flex', gap: '0.4rem' }}>
              <select
                id="ability-score-method"
                style={{ ...INPUT_STYLE, flex: 1 }}
                value={method}
                onChange={(event) => handleMethodChange(event.target.value as AbilityScoreMethodId)}
              >
                {ABILITY_SCORE_METHOD_OPTIONS.map((option) => (
                  <option key={option.id} value={option.id}>
                    {option.label}
                  </option>
                ))}
              </select>
              {methodOption.kind === 'pool' && method !== 'eliteArray' ? (
                <DiceButton label="Reroll all six scores" onClick={handleReroll} />
              ) : null}
              {methodOption.kind === 'straight' ? <DiceButton label="Reroll all six scores" onClick={handleReroll} /> : null}
            </div>
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0.35rem 0 0' }}>{methodOption.description}</p>
            {methodOption.kind === 'pool' ? (
              <div style={{ alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.35rem', marginTop: '0.5rem' }}>
                <span style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', textTransform: 'uppercase' }}>Rolled:</span>
                {pool.map((value, index) => {
                  const assigned = ABILITY_KEYS.some((key) => poolAssignment[key] === index);
                  return (
                    <span
                      key={index}
                      style={{
                        backgroundColor: assigned ? 'var(--color-surface-2)' : 'var(--color-accent)',
                        border: '1px solid var(--color-border)',
                        borderRadius: 6,
                        color: assigned ? 'var(--color-text-muted)' : 'var(--color-on-accent)',
                        fontSize: '0.85rem',
                        fontWeight: 700,
                        opacity: assigned ? 0.6 : 1,
                        padding: '0.15rem 0.5rem',
                        textDecoration: assigned ? 'line-through' : 'none',
                      }}
                    >
                      {value}
                    </span>
                  );
                })}
              </div>
            ) : null}
          </div>

          {methodOption.kind === 'pointBuy' ? (
            <div style={{ alignItems: 'center', display: 'flex', gap: '0.4rem', marginBottom: '0.5rem' }}>
              <select
                style={{ ...INPUT_STYLE, flex: 1 }}
                value=""
                onChange={(event) => {
                  if (event.target.value) {
                    setPointBuyPool(Number(event.target.value));
                  }
                }}
              >
                <option value="">Pool presets…</option>
                {POINT_BUY_POOL_PRESETS.map((preset) => (
                  <option key={preset.points} value={preset.points}>
                    {preset.label}
                  </option>
                ))}
              </select>
              <input
                type="number"
                aria-label="Point buy pool"
                style={{ ...INPUT_STYLE, flex: '0 0 72px' }}
                value={pointBuyPool}
                onChange={(event) => setPointBuyPool(Number(event.target.value))}
              />
            </div>
          ) : null}

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
              {methodOption.kind === 'manual' ? (
                <input
                  id={`ability-${key}`}
                  type="number"
                  style={{ ...INPUT_STYLE, padding: '0.35rem 0.5rem' }}
                  value={abilityScores[key]}
                  onChange={(event) => setAbilityScores((prev) => ({ ...prev, [key]: Number(event.target.value) }))}
                />
              ) : methodOption.kind === 'straight' ? (
                <ReadOnlyBox value={String(abilityScores[key])} />
              ) : methodOption.kind === 'pool' ? (
                <select
                  id={`ability-${key}`}
                  style={{ ...INPUT_STYLE, padding: '0.35rem 0.5rem' }}
                  value={poolAssignment[key] ?? ''}
                  onChange={(event) => assignPoolValue(key, event.target.value === '' ? null : Number(event.target.value))}
                >
                  <option value="">— choose —</option>
                  {pool.map((value, index) => {
                    const takenByOther = ABILITY_KEYS.some((otherKey) => otherKey !== key && poolAssignment[otherKey] === index);
                    if (takenByOther) {
                      return null;
                    }
                    return (
                      <option key={index} value={index}>
                        {value}
                      </option>
                    );
                  })}
                </select>
              ) : (
                <div style={{ alignItems: 'center', display: 'flex', gap: '0.35rem' }}>
                  <button
                    type="button"
                    aria-label={`Decrease ${key}`}
                    onClick={() => adjustPointBuyScore(key, -1)}
                    disabled={abilityScores[key] <= POINT_BUY_MIN_SCORE}
                    style={stepButtonStyle(abilityScores[key] > POINT_BUY_MIN_SCORE)}
                  >
                    −
                  </button>
                  <span style={{ flex: 1, fontWeight: 700, textAlign: 'center' }}>{abilityScores[key]}</span>
                  <button
                    type="button"
                    aria-label={`Increase ${key}`}
                    onClick={() => adjustPointBuyScore(key, 1)}
                    disabled={
                      abilityScores[key] >= POINT_BUY_MAX_SCORE ||
                      pointBuyCost(abilityScores[key] + 1) - pointBuyCost(abilityScores[key]) > pointBuyRemaining
                    }
                    style={stepButtonStyle(
                      abilityScores[key] < POINT_BUY_MAX_SCORE &&
                        pointBuyCost(abilityScores[key] + 1) - pointBuyCost(abilityScores[key]) <= pointBuyRemaining
                    )}
                  >
                    +
                  </button>
                </div>
              )}
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

          {methodOption.kind === 'pointBuy' ? (
            <div
              style={{
                alignItems: 'center',
                backgroundColor: 'var(--color-surface-2)',
                border: `1px solid ${pointBuyRemaining >= 0 ? 'var(--color-accent)' : 'var(--color-error-border)'}`,
                borderRadius: 8,
                display: 'flex',
                justifyContent: 'space-between',
                marginTop: '0.75rem',
                padding: '0.6rem 0.75rem',
              }}
            >
              <span style={{ fontSize: '0.85rem' }}>Point buy</span>
              <span style={{ color: pointBuyRemaining >= 0 ? 'var(--color-accent)' : 'var(--color-error)', fontWeight: 800 }}>
                {pointBuyRemaining} of {pointBuyPool} remaining
              </span>
            </div>
          ) : null}

          {methodOption.kind === 'pool' && unassignedPoolSlots > 0 ? (
            <p style={{ color: 'var(--color-warn)', fontSize: '0.78rem', margin: '0.75rem 0 0' }}>
              Assign all six generated scores to abilities ({unassignedPoolSlots} remaining).
            </p>
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

      {/* A race the backend could not read completely is withheld from the
          picker rather than offered with a guessed size or speed. Naming it
          here is the difference between a roster that is short and a roster
          that is short and says nothing. */}
      {props.rosterDiagnostics.length > 0 ? (
        <div style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', marginTop: '0.75rem' }}>
          <p style={{ margin: '0 0 0.25rem' }}>Races not offered:</p>
          <ul style={{ margin: 0, paddingLeft: '1.1rem' }}>
            {props.rosterDiagnostics.map((diagnostic) => (
              <li key={diagnostic}>{diagnostic}</li>
            ))}
          </ul>
        </div>
      ) : null}

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

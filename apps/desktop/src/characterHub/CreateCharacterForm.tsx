import { useEffect, useState, type CSSProperties, type FormEvent, type ReactNode } from 'react';
import { buildCreationRacialTraitsPreview } from './creationRacialTraitsPreview';
import { composeCreationBio } from './creationBio';
import { buildClassPreview } from './classPreviewModel';
import { loadClassCatalog, type ClassCatalogEntryDto } from '../boundary/loadClassCatalog';
import { updateCharacterBio } from '../boundary/characterBio';
import {
  ABILITY_ABBREVIATIONS,
  ABILITY_KEYS,
  ALIGNMENT_OPTIONS,
  AGE_OPTIONS,
  DEFAULT_ABILITY_SCORES,
  abilityModifier,
  describeClassSupportLevel,
  formatHeight,
  rollDice,
  type AbilityKey,
  type AgeCategory,
  type BodyProfile,
  type ClassOption,
  type RaceOption,
  type Sex,
} from './characterHubModel';
import {
  applyFloatingAbilityAllocation,
  applyRacialAbilityAdjustments,
  composeCreateCharacterRequest,
} from './composeCreateCharacterRequest';
import { loadRaceRosterSurface, rosterErrorMessage, type RaceRosterSurface } from './raceRoster';
import { ensureClassRosterLoaded, useClassCatalog } from './classRoster';
import { LevelsPanel } from './LevelsPanel';
import { ManageBox } from './ManageBox';
import { TransferListDialog } from './TransferListDialog';
import { CHARACTER_TRAIT_LIMIT, alternateTraitItems, characterTraitItems } from './manageItems';
import { remainingSelections } from './transferListModel';
import { SkillAllocationDialog } from './SkillAllocationDialog';
import {
  allocationFromPersisted,
  classSkillLookup,
  persistedFromAllocation,
  skillPointsSpent,
  skillPointsStatus,
  totalSkillPointsAvailable,
} from './skillsModel';
import { LOADING_CLASS_FACTS, failedClassFacts, loadedClassFacts, type ClassFactsState } from './classFactsModel';
import { classFactsQueries } from './classFactsModel';
import { listClassFacts } from '../boundary/listClassFacts';
import { NO_FEAT_SKILL_BONUSES } from '../boundary/loadSavedCharacterDetail';
import { listFeatsForDraft, type FeatCatalogEntryDto } from '../boundary/listFeats';
import { creationFeatSlots, featItems } from './manageItems';
import { CreationEquipmentDialog, type CreationEquipmentItem } from './CreationEquipmentDialog';
import { DEFAULT_PRICE_MODE, type PriceMode } from './priceMode';
import { equipmentBudget } from './creationEquipmentModel';
import { loadStartingWealth, type StartingWealthDto } from '../boundary/startingWealth';
import { CustomDialog } from './CustomDialog';
import { EMPTY_CUSTOM, customIsEmpty, customSummaryLines, withCustomHitPoints, withCustomSkillPoints, type CharacterCustom } from './customModel';
import { saveCharacterCustom } from '../boundary/characterCustom';
import { loadDraftSpellOptions } from '../boundary/draftSpellOptions';
import { listSpells } from '../boundary/listSpells';
import { loadClassSpellLevels } from '../boundary/loadClassSpellLevels';
import { creationSpellDialog, keepOfferedSpells, spellQuotaOverruns, spellSelectionsFromIds, type SpellDialog } from './spellDialogModel';
import type { HeldClass } from './characterProgression';
import { heldClassesOf } from './levelsModel';
import {
  addLevel,
  creationRequestShape,
  removeLevel,
  rerollLevel,
  totalHitPoints,
  characterLevel,
  type CreationLevel,
} from './levelsModel';
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
import type { CreateCharacterRequest } from '../boundary/loadCreateCharacter';
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
  // SD-36 F4c: the class picker is the engine's served roster (`list_class_creation_roster`).
  // If the command fails the catalog carries `CLASS_OPTIONS_FALLBACK` plus a notice, which the
  // form prints above the class select.
  const classRoster = useClassCatalog();

  useEffect(() => {
    void ensureClassRosterLoaded();
  }, []);

  useEffect(() => {
    let live = true;
    loadRaceRosterSurface()
      .then((surface) => {
        if (!live) {
          return;
        }
        const message = rosterErrorMessage(surface);
        if (message !== null) {
          setRosterError(message);
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
  if (roster === null || classRoster.source === 'loading') {
    return (
      <div style={{ border: '1px solid var(--color-border)', borderRadius: 12, padding: '1.25rem' }}>
        <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>
          {roster === null ? 'Loading races from the corpus…' : 'Loading classes from the rules engine…'}
        </p>
      </div>
    );
  }
  return (
    <CreateCharacterFields
      races={roster.options}
      rosterDiagnostics={roster.diagnostics}
      classOptions={classRoster.options}
      classRosterNotice={classRoster.notice}
      onCreated={props.onCreated}
    />
  );
}

function CreateCharacterFields(props: {
  races: RaceOption[];
  rosterDiagnostics: string[];
  /** The served class roster, or the announced fallback (never empty: an empty roster is a failure). */
  classOptions: readonly ClassOption[];
  /** `class roster unavailable: <diagnostic>` when the fallback is in use; printed, never hidden. */
  classRosterNotice: string | null;
  onCreated: () => void;
}) {
  const races = props.races;
  const classOptions = props.classOptions;
  const [displayLabel, setDisplayLabel] = useState('');
  const [playerName, setPlayerName] = useState('');
  const [raceId, setRaceId] = useState(races[0].id);
  // The character's levels, one entry per level in the order they were added (see levelsModel.ts).
  const [levels, setLevels] = useState<CreationLevel[]>([]);
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
  const [bioSaveWarning, setBioSaveWarning] = useState<string | null>(null);
  const [classCatalog, setClassCatalog] = useState<ClassCatalogEntryDto[] | null>(null);
  const [classCatalogError, setClassCatalogError] = useState<string | null>(null);
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
  // Manage dialogs: the selection edits live; Cancel restores what it was when the dialog opened.
  const [racialDialogOpen, setRacialDialogOpen] = useState(false);
  const [racialSnapshot, setRacialSnapshot] = useState<string[]>([]);
  const [traitsDialogOpen, setTraitsDialogOpen] = useState(false);
  const [traitsSnapshot, setTraitsSnapshot] = useState<string[]>([]);
  // Skills: starts at the ranks every new character is seeded with; only sent when the player edits them.
  const [skillAllocation, setSkillAllocation] = useState<Record<string, number>>(() =>
    allocationFromPersisted([
      { skillId: 'skill:climb', ranks: 1 },
      { skillId: 'skill:intimidate', ranks: 1 },
      { skillId: 'skill:swim', ranks: 1 },
    ])
  );
  const [skillsTouched, setSkillsTouched] = useState(false);
  const [skillDialogOpen, setSkillDialogOpen] = useState(false);
  const [classFacts, setClassFacts] = useState<ClassFactsState>(LOADING_CLASS_FACTS);
  // Feats: chosen in the dialog against the draft character's own verdicts.
  const [selectedFeats, setSelectedFeats] = useState<string[]>([]);
  const [featsSnapshot, setFeatsSnapshot] = useState<string[]>([]);
  const [featDialogOpen, setFeatDialogOpen] = useState(false);
  const [draftFeats, setDraftFeats] = useState<FeatCatalogEntryDto[] | null>(null);
  const [draftFeatsError, setDraftFeatsError] = useState<string | null>(null);
  // Spells: quotas per spell level and the race's innate spells come from the backend for this draft.
  const [selectedSpellIds, setSelectedSpellIds] = useState<string[]>([]);
  const [spellsSnapshot, setSpellsSnapshot] = useState<string[]>([]);
  const [spellDialogOpen, setSpellDialogOpen] = useState(false);
  const [spellDialog, setSpellDialog] = useState<SpellDialog | null>(null);
  const [spellDialogError, setSpellDialogError] = useState<string | null>(null);
  // Equipment: bought out of the class's maximum starting money. The pricing choice is session state, never saved.
  const [chosenEquipment, setChosenEquipment] = useState<CreationEquipmentItem[]>([]);
  const [priceMode, setPriceMode] = useState<PriceMode>(DEFAULT_PRICE_MODE);
  const [equipmentDialogOpen, setEquipmentDialogOpen] = useState(false);
  const [wealth, setWealth] = useState<StartingWealthDto | null>(null);
  const [wealthError, setWealthError] = useState<string | null>(null);
  // Custom: GM grants and house-rule records; saved right after the character is created.
  const [custom, setCustom] = useState<CharacterCustom>(EMPTY_CUSTOM);
  const [customDialogOpen, setCustomDialogOpen] = useState(false);
  // AT-34-E4-002 (second slice): the player's resolved skill choice for
  // each selected fixed-choice open-slot trait, keyed by trait id. A trait
  // with no entry here yet (just checked, choice not made) submits no
  // `traitSkillChoices` entry for it -- `skill_choice_bonuses_from_traits`
  // honestly contributes nothing for a trait with no recorded choice,
  // never a first-guessed default (see that function's own doc comment).
  const [traitSkillChoices, setTraitSkillChoices] = useState<Record<string, string>>({});

  const primaryClassId = levels[0]?.classId ?? null;
  const primaryLevels = primaryClassId === null ? 0 : levels.filter((entry) => entry.classId === primaryClassId).length;
  const selectedClass = classOptions.find((option) => option.id === primaryClassId) ?? classOptions[0];
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

  // Age category deliberately contributes nothing here (v0.8 F-12): the
  // engine has no aging model, and previewing an aging modifier that
  // submission never sent made the sheet disagree with this column.
  function calculatedScore(key: AbilityKey): number {
    return rawScore(key) + (selectedRace.abilityAdjustments[key] ?? 0) + allocation[key];
  }

  // The HP box is the sum of the Levels list: each level's die result plus the Constitution modifier.
  const maxHp = levels.length === 0 ? null : withCustomHitPoints(totalHitPoints(levels, abilityModifier(calculatedScore('constitution'))), custom);

  const heldClasses: HeldClass[] = heldClassesOf(levels).map((held) => ({
    classId: held.classId,
    classLabel: classOptions.find((option) => option.id === held.classId)?.label ?? held.classId,
    level: held.level,
  }));
  const heldClassKey = heldClasses.map((held) => held.classId).join(',');
  const isHuman = raceId === 'race:human';
  const skillPointsTotal = withCustomSkillPoints(totalSkillPointsAvailable(heldClasses, abilityModifier(calculatedScore('intelligence')), isHuman), custom);
  const skillPointsLeft = (skillPointsTotal ?? 0) - skillPointsSpent(skillAllocation);
  const feats = creationFeatSlots(characterLevel(levels), raceId);

  // The engine's class skills for the classes in the Levels list (the skills dialog marks them).
  useEffect(() => {
    if (heldClasses.length === 0) {
      setClassFacts(LOADING_CLASS_FACTS);
      return undefined;
    }
    let cancelled = false;
    setClassFacts(LOADING_CLASS_FACTS);
    listClassFacts(classFactsQueries(heldClasses))
      .then((response) => {
        if (!cancelled) setClassFacts(loadedClassFacts(response));
      })
      .catch((cause: unknown) => {
        if (!cancelled) setClassFacts(failedClassFacts(cause));
      });
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [heldClassKey]);

  // While the feats dialog is open, ask the backend which feats this draft character qualifies for.
  const draftFeatsKey = JSON.stringify([heldClassKey, levels.map((entry) => entry.classId), raceId, selectedFeats, selectedAlternateTraitKeys, selectedTraits]);
  useEffect(() => {
    if (!featDialogOpen) {
      return undefined;
    }
    const draft = buildRequest();
    if (draft === null) {
      setDraftFeats(null);
      return undefined;
    }
    let cancelled = false;
    setDraftFeatsError(null);
    listFeatsForDraft(draft, { nameContains: null, category: null })
      .then((response) => {
        if (!cancelled) setDraftFeats(response.entries);
      })
      .catch((cause: unknown) => {
        if (!cancelled) setDraftFeatsError(cause instanceof Error ? cause.message : String(cause));
      });
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [featDialogOpen, draftFeatsKey]);

  // The first class in the Levels list decides the starting money; load it whenever that class changes.
  useEffect(() => {
    if (primaryClassId === null) {
      setWealth(null);
      return undefined;
    }
    let cancelled = false;
    setWealthError(null);
    loadStartingWealth(primaryClassId)
      .then((value) => {
        if (!cancelled) setWealth(value);
      })
      .catch((cause: unknown) => {
        if (!cancelled) setWealthError(cause instanceof Error ? cause.message : String(cause));
      });
    return () => {
      cancelled = true;
    };
  }, [primaryClassId]);

  // Spell quotas follow the draft (class levels, Intelligence/Wisdom/Charisma, race). Reload whenever the
  // inputs change while the dialog is open, and drop picks that are no longer offered.
  const spellKey = JSON.stringify([heldClassKey, raceId, ABILITY_KEYS.map((key) => calculatedScore(key)), selectedAlternateTraitKeys]);
  useEffect(() => {
    if (!spellDialogOpen && selectedSpellIds.length === 0) {
      return undefined;
    }
    const draft = buildRequest();
    if (draft === null) {
      setSpellDialog(null);
      return undefined;
    }
    let cancelled = false;
    setSpellDialogError(null);
    (async () => {
      const options = await loadDraftSpellOptions(draft);
      const tokens = [...new Set(options.perDay.flatMap((row) => {
        const match = /^class_spell\.(?:.+\.)?([a-z_]+)\.(?:total|base)_/.exec(row.id);
        return match ? [`class:${match[1]}`] : [];
      }))];
      const [levels, catalog] = await Promise.all([
        tokens.length === 0 ? Promise.resolve({ classes: [] }) : loadClassSpellLevels(tokens),
        listSpells({ nameContains: null, school: null }),
      ]);
      return creationSpellDialog({ perDay: options.perDay, classLevels: levels.classes, catalog: catalog.entries, innate: options.innate });
    })()
      .then((dialog) => {
        if (cancelled) return;
        setSpellDialog(dialog);
        setSelectedSpellIds((current) => keepOfferedSpells(dialog.items, current));
      })
      .catch((cause: unknown) => {
        if (!cancelled) setSpellDialogError(cause instanceof Error ? cause.message : String(cause));
      });
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [spellDialogOpen, spellKey]);

  function handleAddLevel(addedClassId: string) {
    const option = classOptions.find((candidate) => candidate.id === addedClassId);
    if (option === undefined || option.hitDie === null) {
      return;
    }
    setLevels((current) => addLevel(current, option.id, option.hitDie as number));
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

  // v0.8 F-11: the class progression catalog, loaded once for the preview
  // beside the class select. A failure is shown in place of the preview.
  useEffect(() => {
    let live = true;
    loadClassCatalog()
      .then((response) => {
        if (live) {
          setClassCatalog(response.entries);
        }
      })
      .catch((cause: unknown) => {
        if (live) {
          setClassCatalogError(cause instanceof Error ? cause.message : String(cause));
        }
      });
    return () => {
      live = false;
    };
  }, []);

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

  function setTraitSkillChoice(traitId: string, skillId: string) {
    setTraitSkillChoices((current) => ({ ...current, [traitId]: skillId }));
  }

  /** Applies the Traits dialog's selection: a trait leaving drops its skill choice, one arriving seeds a default. */
  function applyTraitSelection(next: string[]) {
    for (const id of selectedTraits.filter((existing) => !next.includes(existing))) {
      setTraitSkillChoices((current) => {
        const { [id]: _removed, ...rest } = current;
        return rest;
      });
    }
    for (const id of next.filter((added) => !selectedTraits.includes(added))) {
      const option = traitOptions?.find((candidate) => candidate.id === id);
      if (option !== undefined && option.skillOptions.length > 0) {
        setTraitSkillChoices((current) => ({ ...current, [id]: option.skillOptions[0]!.skillId }));
      }
    }
    setSelectedTraits(next);
  }

  const alternateTraitRows = buildAlternateTraitRows(
    alternateMenu,
    raceId,
    selectedAlternateTraitKeys,
    alternateResolution
  );
  const alternateTraitWarnings = creationSelectionWarnings(alternateResolution);
  const racialTraitsPreview = buildCreationRacialTraitsPreview(alternateResolution);
  const classPreview = buildClassPreview(classCatalog, selectedClass, Math.max(1, primaryLevels));

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

  /**
   * The creation request for the form as it stands, or `null` when there is no level yet. Used to
   * create the character and, unsaved, to ask the backend what the draft qualifies for.
   */
  function buildRequest(): CreateCharacterRequest | null {
    const levelShape = creationRequestShape(levels);
    if (levelShape === null) {
      return null;
    }
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
    // `applyTraitSelection` seeds a default the moment it is chosen) is simply
    // omitted rather than sent with a fabricated skill.
    const resolvedTraitSkillChoices = selectedTraits.flatMap((traitId) => {
      const option = traitOptions?.find((candidate) => candidate.id === traitId);
      const skillId = traitSkillChoices[traitId];
      if (option?.choiceSetId == null || skillId === undefined) {
        return [];
      }
      return [{ choiceSetId: option.choiceSetId, selectionId: skillId }];
    });
    return composeCreateCharacterRequest(
      {
        displayLabel,
        raceId,
        classId: levelShape.primaryClassId,
        level: levelShape.primaryLevel,
        additionalLevels: levelShape.additionalLevels,
        hitPointLevels: levelShape.hitPointLevels,
        abilityScores: finalAbilityScores,
        abilityBonusTarget: deriveAbilityBonusTarget(),
        selectedAlternateTraitKeys,
        selectedTraits,
        traitSkillChoices: resolvedTraitSkillChoices,
        selectedFeats: selectedFeats.map((featId) => ({ featId, target: null })),
        selectedSpells: spellSelectionsFromIds(selectedSpellIds),
        selectedEquipment: chosenEquipment.map((item) => ({ itemId: item.itemId })),
        priceMode,
        skillAllocations: skillsTouched ? persistedFromAllocation(skillAllocation) : [],
      },
      { generateId: () => crypto.randomUUID(), now: () => new Date().toISOString() }
    );
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    const levelShape = creationRequestShape(levels);
    if (levelShape === null) {
      setError('Add at least one level before creating the character.');
      return;
    }
    if (unassignedPoolSlots > 0) {
      setError(`Assign all six generated scores to abilities before creating (${unassignedPoolSlots} remaining).`);
      return;
    }
    if (methodOption.kind === 'pointBuy' && pointBuyRemaining < 0) {
      setError(`Point buy is over budget by ${-pointBuyRemaining} points — lower a score or raise the pool before creating.`);
      return;
    }
    if (skillsTouched && skillPointsTotal !== null && skillPointsLeft < 0) {
      setError(`Skills are over budget by ${-skillPointsLeft} points: open Manage on Skills and lower a rank before creating.`);
      return;
    }
    const spellOverruns = spellDialog === null ? [] : spellQuotaOverruns(spellDialog, selectedSpellIds);
    if (spellOverruns.length > 0) {
      setError(`Too many spells chosen (${spellOverruns.join('; ')}): open Manage on Spells and remove some.`);
      return;
    }
    const equipmentSpend = equipmentBudget({ startingGp: wealth?.maxGp ?? null, mode: priceMode, costsGp: chosenEquipment.map((item) => item.costGp ?? 0) });
    if (equipmentSpend.over) {
      setError(`Equipment costs more than the starting money (${equipmentSpend.text}) Open Manage on Equipment and remove something.`);
      return;
    }
    if (selectedFeats.length > feats) {
      setError(`${selectedFeats.length} feats are chosen but this character has ${feats}: open Manage on Feats and remove ${selectedFeats.length - feats}.`);
      return;
    }
    setSubmitting(true);
    setError(null);
    setBioSaveWarning(null);
    try {
      const request = buildRequest();
      if (request === null) {
        return;
      }
      const result = await createCharacterRuntime(request);
      setOutcome(result);
      if (result.kind === 'saved') {
        // v0.8 F-1: the bio sidecar is a separate command from
        // `create_character`; persist the nine fields the form collected
        // now that the character exists. A failure here is reported but is
        // not a creation failure — the character is already saved.
        try {
          await updateCharacterBio(
            request.characterId,
            composeCreationBio({ playerName, alignment, deity, sex, age, eyes, hair, heightInches, weightLb }),
          );
        } catch (cause: unknown) {
          setBioSaveWarning(
            `Character saved, but its bio fields were not: ${cause instanceof Error ? cause.message : String(cause)}. Edit them on the sheet.`,
          );
        }
        if (!customIsEmpty(custom)) {
          try {
            await saveCharacterCustom(request.characterId, custom, new Date().toISOString());
          } catch (cause: unknown) {
            const text = `Character saved, but its Custom data was not: ${cause instanceof Error ? cause.message : String(cause)}. Open Custom from the sheet's menu to add it again.`;
            setBioSaveWarning((previous) => (previous ? `${previous} ${text}` : text));
          }
        }
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
        <div style={{ flex: '3 1 520px', minWidth: 0 }}>
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
          </div>
          {props.classRosterNotice !== null ? (
            <p role="alert" style={{ color: 'var(--color-warn)', fontSize: '0.8rem', margin: '-0.5rem 0 0.5rem' }}>
              {props.classRosterNotice} — offering the built-in list of {classOptions.length} classes instead.
            </p>
          ) : null}
          {levels.length > 0 ? (
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '-0.5rem 0 0.5rem' }}>
              {describeClassSupportLevel(selectedClass.supportLevel, selectedClass.label)}
            </p>
          ) : null}
          {/* v0.8 F-11: what this class is mechanically at the level being
              created — the `list_class_catalog` row, verbatim. Skill points
              per level are not on that DTO, so none are shown. */}
          {levels.length > 0 ? (
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0 0 1rem' }}>
              {classCatalogError !== null
                ? `Class preview unavailable: ${classCatalogError}`
                : classPreview.kind === 'Loading'
                  ? 'Loading class preview…'
                  : classPreview.kind === 'Unavailable'
                    ? classPreview.message
                    : `${selectedClass.label} ${classPreview.level}: BAB ${classPreview.baseAttackBonus} · Fort ${classPreview.fortSave} · Ref ${classPreview.refSave} · Will ${classPreview.willSave}`}
            </p>
          ) : null}

          {/* Level + HP (computed) + Alignment + Deity */}
          <div style={ROW_STYLE}>
            {/* The character level: how many levels are in the Levels list. */}
            <LabeledField label="Level" flex="0 0 96px">
              <ReadOnlyBox value={String(characterLevel(levels))} />
            </LabeledField>
            <LabeledField label="HP" flex="0 0 96px">
              {/* The sum of the Levels list; a dash until the first level is added. */}
              <ReadOnlyBox value={maxHp === null ? '—' : String(maxHp)} />
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

          {/* Racial traits: the standard traits the race grants are innate; alternate racial traits are
              chosen in the Manage dialog. Every fact comes from the backend (`resolve_race_alternate_selection`),
              and `create_character` re-validates the keys against the corpus. */}
          <ManageBox
            title="Racial Traits"
            remaining={racialTraitsPreview.unavailableReason === null ? `${racialTraitsPreview.rows.length} innate` : undefined}
            summary={selectedAlternateTraitKeys.map(
              (key) => alternateTraitRows.find((row) => row.alternate.key === key)?.alternate.name ?? key
            )}
            disabledReason={
              alternateMenuError !== null
                ? `Alternate racial traits are unavailable: ${alternateMenuError}`
                : alternateMenu === null
                  ? 'Loading alternate racial traits…'
                  : alternateTraitRows.length === 0
                    ? `No ingested book declares an alternate racial trait for ${selectedRace.label}.`
                    : undefined
            }
            onManage={() => {
              setRacialSnapshot(selectedAlternateTraitKeys);
              setRacialDialogOpen(true);
            }}
          >
            {alternateResolution !== null && selectedAlternateTraitKeys.length > 0 ? (
              <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.4rem 0 0' }}>
                {describeCreationSelection(selectedAlternateTraitKeys, alternateResolution)}
              </p>
            ) : null}
            {alternateTraitWarnings.map((warning) => (
              <p key={warning} style={{ color: 'var(--color-danger, #c0392b)', fontSize: '0.75rem', margin: '0.4rem 0 0' }}>
                {warning}
              </p>
            ))}
          </ManageBox>
          <TransferListDialog
            open={racialDialogOpen}
            title={`Racial traits: ${selectedRace.label}`}
            notice="Traits the race grants are listed under Innate. Pick alternate racial traits to replace some of them; ones the current picks rule out are struck through."
            items={alternateTraitItems(alternateTraitRows, racialTraitsPreview.rows)}
            selected={selectedAlternateTraitKeys}
            onSelectedChange={setSelectedAlternateTraitKeys}
            limit={null}
            remainingNoun="alternate traits"
            onAccept={() => setRacialDialogOpen(false)}
            onCancel={() => {
              setSelectedAlternateTraitKeys(racialSnapshot);
              setRacialDialogOpen(false);
            }}
          />

          {/* AT-34-E4-002: character traits. Every option offered genuinely computes (the 53 `ultimate_campaign`
              traits whose skill, save, situational, initiative/concentration, ability-difference or mixed
              caster-level bonuses the `trait_effects` compute paths apply); no wider roster is offered. */}
          <ManageBox
            title="Traits"
            remaining={`${remainingSelections(CHARACTER_TRAIT_LIMIT, selectedTraits)} of ${CHARACTER_TRAIT_LIMIT} remaining`}
            summary={selectedTraits.map((id) => traitOptions?.find((option) => option.id === id)?.name ?? id)}
            disabledReason={
              traitOptionsError !== null ? `Traits are unavailable: ${traitOptionsError}` : traitOptions === null ? 'Loading traits…' : undefined
            }
            onManage={() => {
              setTraitsSnapshot(selectedTraits);
              setTraitsDialogOpen(true);
            }}
          >
            {selectedTraits.map((id) => {
              const option = traitOptions?.find((candidate) => candidate.id === id);
              if (option === undefined || option.skillOptions.length === 0) {
                return null;
              }
              return (
                <select
                  key={id}
                  aria-label={`${option.name} skill choice`}
                  value={traitSkillChoices[id] ?? option.skillOptions[0]!.skillId}
                  onChange={(event) => setTraitSkillChoice(id, event.target.value)}
                  style={{ display: 'block', fontSize: '0.78rem', marginTop: '0.35rem' }}
                >
                  {option.skillOptions.map((choice) => (
                    <option key={choice.skillId} value={choice.skillId}>
                      {option.name}: {choice.name}
                    </option>
                  ))}
                </select>
              );
            })}
          </ManageBox>
          <TransferListDialog
            open={traitsDialogOpen}
            title="Traits"
            notice="A new character chooses two traits."
            items={characterTraitItems(traitOptions ?? [])}
            selected={selectedTraits}
            onSelectedChange={applyTraitSelection}
            limit={CHARACTER_TRAIT_LIMIT}
            remainingNoun="traits"
            onAccept={() => setTraitsDialogOpen(false)}
            onCancel={() => {
              applyTraitSelection(traitsSnapshot);
              setTraitsDialogOpen(false);
            }}
          />

          {/* Skills: a full list with + and - to add ranks; the remaining points show at the top of the dialog. */}
          <ManageBox
            title="Skills"
            remaining={heldClasses.length === 0 ? undefined : skillPointsStatus(skillPointsTotal, skillPointsLeft).text}
            summary={Object.entries(skillAllocation)
              .filter(([, ranks]) => ranks > 0)
              .map(([name, ranks]) => `${name}: ${ranks} rank${ranks === 1 ? '' : 's'}`)}
            disabledReason={heldClasses.length === 0 ? 'Add a level first: skill points come from the class levels.' : undefined}
            onManage={() => setSkillDialogOpen(true)}
          />
          <SkillAllocationDialog
            open={skillDialogOpen}
            onClose={() => setSkillDialogOpen(false)}
            heldClasses={heldClasses}
            classSkills={classSkillLookup(heldClasses, classFacts)}
            characterLevel={characterLevel(levels)}
            abilities={{
              strength: abilityModifier(calculatedScore('strength')),
              dexterity: abilityModifier(calculatedScore('dexterity')),
              constitution: abilityModifier(calculatedScore('constitution')),
              intelligence: abilityModifier(calculatedScore('intelligence')),
              wisdom: abilityModifier(calculatedScore('wisdom')),
              charisma: abilityModifier(calculatedScore('charisma')),
            }}
            totalPoints={skillPointsTotal}
            allocation={skillAllocation}
            featSkillBonuses={NO_FEAT_SKILL_BONUSES}
            onAccept={(next) => {
              setSkillAllocation(next);
              setSkillsTouched(true);
              setSkillDialogOpen(false);
            }}
          />

          {/* Feats: qualification is the draft character's own verdict from the backend. */}
          <ManageBox
            title="Feats"
            remaining={heldClasses.length === 0 ? undefined : `${remainingSelections(feats, selectedFeats)} of ${feats} remaining`}
            summary={selectedFeats.map((id) => draftFeats?.find((entry) => entry.key === id)?.name ?? id)}
            disabledReason={heldClasses.length === 0 ? 'Add a level first: feats come with the character level.' : undefined}
            onManage={() => {
              setFeatsSnapshot(selectedFeats);
              setFeatDialogOpen(true);
            }}
          />
          <TransferListDialog
            open={featDialogOpen}
            title="Feats"
            notice={
              draftFeatsError !== null
                ? `Feats could not be checked for this character: ${draftFeatsError}`
                : draftFeats === null
                  ? 'Checking which feats this character qualifies for…'
                  : 'One feat at every odd level, plus a human\'s bonus feat. Class bonus feats (a Fighter\'s, say) are chosen on the sheet, as are the targets of feats that name a weapon, skill or school.'
            }
            items={featItems(draftFeats ?? [])}
            selected={selectedFeats}
            onSelectedChange={setSelectedFeats}
            limit={feats}
            remainingNoun="feats"
            onAccept={() => setFeatDialogOpen(false)}
            onCancel={() => {
              setSelectedFeats(featsSnapshot);
              setFeatDialogOpen(false);
            }}
          />

          {/* Spells: a quota per spell level from the engine; racial spell-like abilities sit in a read-only Innate column. */}
          <ManageBox
            title="Spells"
            remaining={
              spellDialog === null || Object.keys(spellDialog.groupLimits).length === 0
                ? undefined
                : `${Object.values(spellDialog.groupLimits).reduce((sum, n) => sum + n, 0) - selectedSpellIds.length} of ${Object.values(spellDialog.groupLimits).reduce((sum, n) => sum + n, 0)} remaining`
            }
            summary={selectedSpellIds.map((id) => id.split('|')[1] ?? id)}
            disabledReason={heldClasses.length === 0 ? 'Add a level first: spell slots come from the class levels.' : undefined}
            onManage={() => {
              setSpellsSnapshot(selectedSpellIds);
              setSpellDialogOpen(true);
            }}
          />
          <TransferListDialog
            open={spellDialogOpen}
            title="Spells"
            notice={
              spellDialogError !== null
                ? `Spells could not be loaded for this character: ${spellDialogError}`
                : spellDialog === null
                  ? 'Working out how many spells of each level this character may pick…'
                  : Object.keys(spellDialog.groupLimits).length === 0
                    ? 'This character has no spell slots at this level, so there is nothing to pick. Innate abilities are listed below.'
                    : 'Quotas are the engine\'s spells per day for each level (a wizard\'s spellbook holds more than it prepares; a spontaneous caster knows a different count). Picks are added as known spells.'
            }
            items={spellDialog?.items ?? []}
            selected={selectedSpellIds}
            onSelectedChange={setSelectedSpellIds}
            limit={null}
            groupLimits={spellDialog?.groupLimits}
            groupLabels={spellDialog?.groupLabels}
            remainingNoun="spells"
            onAccept={() => setSpellDialogOpen(false)}
            onCancel={() => {
              setSelectedSpellIds(spellsSnapshot);
              setSpellDialogOpen(false);
            }}
          />

          {/* Equipment: starting money is the class's maximum; categories, search and the pricing choice live in the dialog. */}
          <ManageBox
            title="Equipment"
            remaining={
              primaryClassId === null
                ? undefined
                : equipmentBudget({ startingGp: wealth?.maxGp ?? null, mode: priceMode, costsGp: chosenEquipment.map((item) => item.costGp ?? 0) }).text
            }
            summary={chosenEquipment.map((item) => item.name)}
            disabledReason={primaryClassId === null ? 'Add a level first: starting money comes from the class.' : undefined}
            onManage={() => setEquipmentDialogOpen(true)}
          />
          <CreationEquipmentDialog
            open={equipmentDialogOpen}
            wealth={wealth}
            wealthError={wealthError}
            mode={priceMode}
            onModeChange={setPriceMode}
            chosen={chosenEquipment}
            onChange={setChosenEquipment}
            onClose={() => setEquipmentDialogOpen(false)}
          />

          {/* Custom: the GM's grants and house-rule records, available on Create and on the sheet. */}
          <ManageBox title="Custom" summary={customSummaryLines(custom)} onManage={() => setCustomDialogOpen(true)}>
            {custom.grants.some((grant) => grant.target.startsWith('ability:')) ? (
              <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.4rem 0 0' }}>Ability grants are applied to the saved scores when the character is created.</p>
            ) : null}
          </ManageBox>
          <CustomDialog
            open={customDialogOpen}
            value={custom}
            onSave={(next) => {
              setCustom(next);
              setCustomDialogOpen(false);
            }}
            onCancel={() => setCustomDialogOpen(false)}
          />
        </div>

        {/* Levels column: classes are added one level at a time; the HP and Level boxes follow this list. */}
        <LevelsPanel
          classOptions={classOptions}
          levels={levels}
          constitutionModifier={abilityModifier(calculatedScore('constitution'))}
          onAdd={handleAddLevel}
          onReroll={(index) => setLevels((current) => rerollLevel(current, index))}
          onRemove={(index) => setLevels((current) => removeLevel(current, index))}
        />

        {/* Right column: ability scores panel */}
        <div
          style={{
            backgroundColor: 'var(--color-surface)',
            border: '1px solid var(--color-border)',
            borderRadius: 10,
            flex: '1 1 360px',
            maxWidth: 560,
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
        disabled={submitting || levels.length === 0}
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
      {bioSaveWarning ? <p style={{ color: 'var(--color-warn)', marginTop: '0.75rem' }}>{bioSaveWarning}</p> : null}

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

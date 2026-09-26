/**
 * The class roster character creation and Level Up are built from, served by the engine out of
 * the class census (SD-36 Epic F4, spec `epic-f-class-completion.md` §6), not compiled into this
 * file.
 *
 * # What changed and why
 *
 * `characterHubModel.CLASS_OPTIONS` was a hand-written table of 31 classes. The census offers 59
 * at creation (every non-prestige, non-Ex-* class Computed at every level with a stated hit die)
 * and 74 prestige classes at level-up. The hand table also drifted from the converted records it
 * mirrored: the skill-ranks table it sat beside was wrong for 33 of the 59 roster classes (e.g.
 * Arcanist 3 against the record's 2; every class it did not list silently defaulted to 2), and an
 * unknown class's hit die silently defaulted to d8.
 *
 * Hit points read `hitPointsDie`, the die the engine's own hit-point fold reads (the class's
 * chassis record). 5 of the 59 roster classes have no chassis record — the CRB Monk (whose printed
 * row carries the FS-23 oracle defect, `HD:10` against CRB p.56's d8) and the four Pathfinder
 * Unchained classes — so their HP prints Unknown, as the engine's does, rather than a total built
 * on a defective or borrowed die.
 *
 * `CLASS_OPTIONS_FALLBACK` is that table, kept ONLY for the case the command fails, and then
 * installed with a visible notice (`class roster unavailable: <diagnostic>`) that the Create form
 * and Level Up both print. There is no silent path to it.
 */

import { useSyncExternalStore } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from '../boundary/runtime';
import { CLASS_OPTIONS_FALLBACK, type ClassOption } from './characterHubModel';
import {
  getClassCatalog,
  setClassCatalog,
  subscribeClassCatalog,
  type ClassCatalogState,
  type KnownClass,
} from './classCatalog';

// ----- Wire shapes: verbatim from `apps/desktop/src-tauri/src/character_hub.rs` -----

/** `ClassCreationEntryDto`. */
export interface ClassCreationEntryDto {
  classId: string;
  label: string;
  /** Census family, snake_case (`crb`, `apg`, …): the grouping key. */
  family: string;
  familyLabel: string;
  book: string;
  /** The principal's printed `Hit die` row (the roster rule's input). */
  hitDie: number;
  /**
   * The die the engine's hit-point fold reads (the chassis record's); `null` when the class has
   * no chassis record — the engine then reports its HP Unknown, and so does every HP here.
   */
  hitPointsDie: number | null;
  skillRanksPerLevel: number | null;
  maxLevel: number;
}

/** `WithheldClassDto`: a census class not offered at creation, with its named reason. */
export interface WithheldClassDto {
  classId: string;
  label: string;
  reason: 'prestige' | 'ex_state' | 'not_computed' | 'hit_die_absent';
  hitDie: number | null;
  hitPointsDie: number | null;
  skillRanksPerLevel: number | null;
}

/** `ClassCreationRosterResponse`. */
export interface ClassCreationRosterResponse {
  classes: ClassCreationEntryDto[];
  withheld: WithheldClassDto[];
  /** One line per class withheld for a GAP; empty in a healthy checkout. */
  diagnostics: string[];
}

/** `LevelUpClassOptionDto`. */
export interface LevelUpClassOptionDto {
  classId: string;
  label: string;
  family: string;
  familyLabel: string;
  currentLevel: number;
  nextLevel: number;
  maxLevel: number;
}

/** `EntryRequirementDto`: one printed prestige entry requirement and its note for this character. */
export interface EntryRequirementDto {
  text: string;
  status: 'met' | 'unmet' | 'situational';
  condition: string | null;
}

/** `PrestigeClassOptionDto`. */
export interface PrestigeClassOptionDto {
  classId: string;
  label: string;
  book: string;
  nextLevel: number;
  maxLevel: number;
  hitDie: number | null;
  entryRequirements: EntryRequirementDto[];
  /** A note, never a gate (ruling §9.2). */
  requirementsAllMet: boolean;
}

/** `LevelUpClassOptionsResponse`. */
export interface LevelUpClassOptionsResponse {
  characterLevel: number;
  levelCap: number;
  atLevelCap: boolean;
  advance: LevelUpClassOptionDto[];
  addBase: LevelUpClassOptionDto[];
  addPrestige: PrestigeClassOptionDto[];
  diagnostics: string[];
}

// ----- Pure mapping -----

function levelsUpTo(maxLevel: number): number[] {
  return Array.from({ length: Math.max(0, maxLevel) }, (_unused, index) => index + 1);
}

/**
 * Wire rows to picker options. Pure: every served class becomes one option in served order,
 * carrying its served label, hit die, family and level range (1..max: the census swept every
 * level Computed). Nothing is added, dropped or reordered.
 */
export function classOptionsFromRoster(response: ClassCreationRosterResponse): ClassOption[] {
  return response.classes.map((row) => ({
    id: row.classId,
    label: row.label,
    supportLevel: 'full',
    levelOptions: levelsUpTo(row.maxLevel),
    hitDie: row.hitPointsDie,
    family: row.family,
    familyLabel: row.familyLabel,
    book: row.book,
  }));
}

export interface ClassOptionGroup {
  family: string;
  familyLabel: string;
  options: ClassOption[];
}

/**
 * Contiguous family runs, in option order. A fallback option carries no family; the fallback
 * then renders as one ungrouped run.
 */
export function groupClassOptionsByFamily(options: readonly ClassOption[]): ClassOptionGroup[] {
  const groups: ClassOptionGroup[] = [];
  for (const option of options) {
    const family = option.family ?? '';
    const last = groups[groups.length - 1];
    if (last && last.family === family) {
      last.options.push(option);
    } else {
      groups.push({ family, familyLabel: option.familyLabel ?? '', options: [option] });
    }
  }
  return groups;
}

function knownFromRoster(response: ClassCreationRosterResponse): Map<string, KnownClass> {
  const known = new Map<string, KnownClass>();
  for (const row of response.classes) {
    known.set(row.classId, { id: row.classId, label: row.label, hitDie: row.hitPointsDie, skillRanksPerLevel: row.skillRanksPerLevel });
  }
  for (const row of response.withheld) {
    known.set(row.classId, { id: row.classId, label: row.label, hitDie: row.hitPointsDie, skillRanksPerLevel: row.skillRanksPerLevel });
  }
  return known;
}

export function rosterCatalogState(response: ClassCreationRosterResponse): ClassCatalogState {
  return { source: 'roster', options: classOptionsFromRoster(response), known: knownFromRoster(response), notice: null };
}

/**
 * The fallback catalog: the hardcoded 31 with the failure named. Skill ranks are `null` — the
 * fallback table never carried them correctly, so the sheet prints Unknown rather than a guess.
 */
export function fallbackCatalogState(diagnostic: string): ClassCatalogState {
  const known = new Map<string, KnownClass>();
  for (const option of CLASS_OPTIONS_FALLBACK) {
    known.set(option.id, { id: option.id, label: option.label, hitDie: option.hitDie, skillRanksPerLevel: null });
  }
  return { source: 'fallback', options: CLASS_OPTIONS_FALLBACK, known, notice: `class roster unavailable: ${diagnostic}` };
}

export function installClassRoster(response: ClassCreationRosterResponse): void {
  if (response.classes.length === 0) {
    const reason = response.diagnostics.length > 0 ? response.diagnostics.join('; ') : 'the engine offered no class at all';
    installClassRosterFailure(reason);
    return;
  }
  setClassCatalog(rosterCatalogState(response));
}

export function installClassRosterFailure(diagnostic: string): void {
  setClassCatalog(fallbackCatalogState(diagnostic));
}

// ----- Boundary -----

/**
 * Invokes `list_class_creation_roster`. Throws without the desktop runtime rather than returning
 * sample classes: the caller then installs the fallback WITH that reason as its notice.
 */
export async function loadClassCreationRoster(): Promise<ClassCreationRosterResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('the class roster is served by the desktop app’s rules engine (no desktop runtime here)');
  }
  try {
    return await invoke<ClassCreationRosterResponse>('list_class_creation_roster');
  } catch (cause: unknown) {
    throw new Error(`Failed to load the class roster: ${formatError(cause)}`);
  }
}

let inflight: Promise<void> | null = null;

/**
 * Loads the roster once per session and installs the outcome: the roster, or the fallback with
 * the failure as its notice. Later calls return the same promise; `reload` asks again.
 */
export function ensureClassRosterLoaded(
  loader: () => Promise<ClassCreationRosterResponse> = loadClassCreationRoster,
  options: { reload?: boolean } = {}
): Promise<void> {
  if (inflight && !options.reload) {
    return inflight;
  }
  inflight = loader().then(
    (response) => installClassRoster(response),
    (cause: unknown) => installClassRosterFailure(formatError(cause))
  );
  return inflight;
}

/** React: the current catalog, re-rendering when the roster (or the fallback) is installed. */
export function useClassCatalog(): ClassCatalogState {
  return useSyncExternalStore(subscribeClassCatalog, getClassCatalog, getClassCatalog);
}

/** Invokes `list_level_up_class_options` for a saved character. */
export async function loadLevelUpClassOptions(characterId: string): Promise<LevelUpClassOptionsResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('level-up class options are served by the desktop app’s rules engine (no desktop runtime here)');
  }
  try {
    return await invoke<LevelUpClassOptionsResponse>('list_level_up_class_options', { request: { characterId } });
  } catch (cause: unknown) {
    throw new Error(`Failed to load the level-up class options: ${formatError(cause)}`);
  }
}

// ----- Level Up -----

export type LevelUpChoiceKind = 'advance' | 'add_base' | 'add_prestige';

export interface LevelUpChoice {
  kind: LevelUpChoiceKind;
  classId: string;
  label: string;
  currentLevel: number;
  nextLevel: number;
  maxLevel: number;
  /** Prestige only: the printed entry requirements with their notes. Empty otherwise. */
  requirements: EntryRequirementDto[];
  requirementsAllMet: boolean;
  /** Always `true`: an unmet prestige requirement is printed, never a block (§9.2). */
  selectable: true;
}

export interface LevelUpChoiceGroup {
  kind: LevelUpChoiceKind;
  heading: string;
  choices: LevelUpChoice[];
}

/** The three Level Up groups, in order, straight off the response. */
export function levelUpChoiceGroups(response: LevelUpClassOptionsResponse): LevelUpChoiceGroup[] {
  const fromOption = (kind: LevelUpChoiceKind) => (option: LevelUpClassOptionDto): LevelUpChoice => ({
    kind,
    classId: option.classId,
    label: option.label,
    currentLevel: option.currentLevel,
    nextLevel: option.nextLevel,
    maxLevel: option.maxLevel,
    requirements: [],
    requirementsAllMet: true,
    selectable: true,
  });
  return [
    { kind: 'advance', heading: 'Advance a class you have', choices: response.advance.map(fromOption('advance')) },
    { kind: 'add_base', heading: 'Add a base class', choices: response.addBase.map(fromOption('add_base')) },
    {
      kind: 'add_prestige',
      heading: 'Add a prestige class',
      choices: response.addPrestige.map((option) => ({
        kind: 'add_prestige' as const,
        classId: option.classId,
        label: option.label,
        currentLevel: 0,
        nextLevel: option.nextLevel,
        maxLevel: option.maxLevel,
        requirements: option.entryRequirements,
        requirementsAllMet: option.requirementsAllMet,
        selectable: true as const,
      })),
    },
  ];
}

/** PF1's character level cap (`CHARACTER_LEVEL_CAP` in `character_hub.rs`). */
export const CHARACTER_LEVEL_CAP = 20;

/**
 * What Level Up offers when `list_level_up_class_options` itself fails: advance a held class the
 * catalog offers below its own top level, or add a class the catalog offers — no prestige class
 * (only the command judges entry requirements) — with the failure as the first diagnostic, which
 * the dialog prints. The character level cap still holds.
 */
export function fallbackLevelUpResponse(
  heldClasses: readonly { classId: string; classLabel: string; level: number }[],
  catalogOptions: readonly ClassOption[],
  diagnostic: string
): LevelUpClassOptionsResponse {
  const characterLevel = heldClasses.reduce((sum, held) => sum + held.level, 0);
  const notice = `level-up class options unavailable: ${diagnostic}`;
  const base: LevelUpClassOptionsResponse = {
    characterLevel,
    levelCap: CHARACTER_LEVEL_CAP,
    atLevelCap: characterLevel >= CHARACTER_LEVEL_CAP,
    advance: [],
    addBase: [],
    addPrestige: [],
    diagnostics: [notice],
  };
  if (base.atLevelCap) {
    return base;
  }
  const toDto = (option: ClassOption, currentLevel: number): LevelUpClassOptionDto => ({
    classId: option.id,
    label: option.label,
    family: option.family ?? '',
    familyLabel: option.familyLabel ?? '',
    currentLevel,
    nextLevel: currentLevel + 1,
    maxLevel: option.levelOptions[option.levelOptions.length - 1] ?? 1,
  });
  for (const held of heldClasses) {
    const option = catalogOptions.find((entry) => entry.id === held.classId);
    if (option && option.levelOptions.includes(held.level + 1)) {
      base.advance.push(toDto(option, held.level));
    }
  }
  base.addBase = catalogOptions
    .filter((option) => !heldClasses.some((held) => held.classId === option.id))
    .map((option) => toDto(option, 0));
  return base;
}

/** One printed requirement line: the rule's words, then its note for this character. */
export function describeEntryRequirement(requirement: EntryRequirementDto): string {
  if (requirement.status === 'situational') {
    return requirement.condition
      ? `${requirement.text} — situational: ${requirement.condition}`
      : `${requirement.text} — situational`;
  }
  return `${requirement.text} — ${requirement.status}`;
}

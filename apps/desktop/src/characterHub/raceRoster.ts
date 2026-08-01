/**
 * The race roster character creation is built from — served by the backend
 * out of the real race corpus, not compiled into this file.
 *
 * # What changed and why
 *
 * `characterHubModel.RACE_OPTIONS` used to be a hand-written table of the 7
 * Core Rulebook races. The corpus carries **18** — CRB's 7 plus Bestiary 1's
 * aasimar, drow, duergar, goblin, hobgoblin, kobold, merfolk, orc,
 * svirfneblin, tengu and tiefling — and `raceCreationCoverage.test.ts` proves
 * against the on-disk JSON that every one of them supplies every
 * rules-bearing field creation reads. Those eleven were ingested, resolvable
 * by `race_resolver`, and browsable in the Race Trait Catalog. No player
 * could make one.
 *
 * A hand-maintained mirror of corpus facts is also how the same table one
 * layer down (`rules_tables/crb/race_tables.rs`) silently drifted from the
 * corpus on four races' ability modifiers, because `BONUS:STAT|CON,WIS|2`
 * states two grants in one token and a transcription read only up to the
 * comma. `list_race_creation_roster` derives instead, which removes the
 * class of defect rather than re-checking for it.
 *
 * # Height and weight are the one exception, and deliberately so
 *
 * PCGen keeps height/weight in `<race>/<race>_biosettings.lst`, which no
 * book's ingest reads for any book — asserted, not assumed, by
 * `verifiesTheCorpusCarriesNoHeightOrWeightProfileForAnyRace`. The 7 profiles
 * in [`RACE_BODY_PROFILES`] are hand-entered constants with no corpus behind
 * them and are kept verbatim so the races that already shipped keep rolling
 * exactly what they rolled before. The other 11 get `body: null` and the form
 * says so, because inventing eleven more would produce numbers on screen that
 * are indistinguishable from real PF1 ones.
 */

import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from '../boundary/runtime';
import type { AbilityKey, BodyProfile, RaceOption, Sex } from './characterHubModel';

/**
 * One race's creation chassis, verbatim from `RaceCreationChassisDto` in
 * `apps/desktop/src-tauri/src/character_hub.rs`.
 */
export interface RaceCreationChassisDto {
  /** The `race:<slug>` token submitted with the character and resolved by the engine's size seam. */
  raceId: string;
  /** The corpus race key verbatim, e.g. `Half-Elf`, `Svirfneblin`. */
  label: string;
  /** Short sourcebook code — `CRB` or `B1` today, the same codes the Race Trait Catalog emits. */
  book: string;
  /** `Small` or `Medium`: the race's `~ Size` trait template over the chassis `FACT:BaseSize`. */
  size: string;
  /** e.g. `Darkvision 60 ft.`, `Low-light vision`, `Darkvision 120 ft., Low-light vision`, `Normal`. */
  vision: string;
  /** Base land speed in feet, after any `~ Speed` trait override of the chassis `MOVE:Walk`. */
  baseSpeedFt: number;
  /** Fixed racial ability modifiers, keyed by ability name. Only non-zero entries appear. */
  abilityAdjustments: Partial<Record<AbilityKey, number>>;
  /** Freely distributed "+2 to one ability score" points; `0` for a race with no such pool. */
  floatingBonusPoints: number;
}

export interface RaceCreationRosterResponse {
  races: RaceCreationChassisDto[];
  /**
   * Corpus files the backend could not read, plus one entry naming each race
   * it had to withhold and the field it was missing. Empty in a healthy
   * checkout. Surfaced rather than swallowed so a shrunken roster reports why
   * it shrank instead of silently offering less than it claims to.
   */
  diagnostics: string[];
}

/**
 * Random height/weight profiles, by race id.
 *
 * **These are hand-entered approximate PF1 Core values with no corpus
 * source.** They predate this module and are reproduced verbatim so the 7
 * races that already shipped keep the exact roll ranges they shipped with.
 * Height and weight are display-only in the creation form: neither is part of
 * `CreateCharacterRequest`, so nothing is persisted or computed from them.
 *
 * A race absent from this table gets no profile at all rather than a borrowed
 * one — see this module's own doc comment.
 */
export const RACE_BODY_PROFILES: Record<string, Record<Sex, BodyProfile>> = {
  'race:human': {
    male: { baseHeightInches: 58, heightModDice: { count: 2, sides: 10 }, baseWeightLb: 120, weightMultiplierLb: 5 },
    female: { baseHeightInches: 53, heightModDice: { count: 2, sides: 10 }, baseWeightLb: 85, weightMultiplierLb: 5 },
  },
  'race:dwarf': {
    male: { baseHeightInches: 45, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 150, weightMultiplierLb: 7 },
    female: { baseHeightInches: 43, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 120, weightMultiplierLb: 7 },
  },
  'race:elf': {
    male: { baseHeightInches: 60, heightModDice: { count: 2, sides: 6 }, baseWeightLb: 100, weightMultiplierLb: 3 },
    female: { baseHeightInches: 60, heightModDice: { count: 2, sides: 6 }, baseWeightLb: 90, weightMultiplierLb: 3 },
  },
  'race:gnome': {
    male: { baseHeightInches: 36, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 35, weightMultiplierLb: 1 },
    female: { baseHeightInches: 34, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 30, weightMultiplierLb: 1 },
  },
  'race:half-elf': {
    male: { baseHeightInches: 55, heightModDice: { count: 2, sides: 8 }, baseWeightLb: 110, weightMultiplierLb: 5 },
    female: { baseHeightInches: 55, heightModDice: { count: 2, sides: 8 }, baseWeightLb: 90, weightMultiplierLb: 5 },
  },
  'race:half-orc': {
    male: { baseHeightInches: 58, heightModDice: { count: 2, sides: 12 }, baseWeightLb: 150, weightMultiplierLb: 7 },
    female: { baseHeightInches: 58, heightModDice: { count: 2, sides: 12 }, baseWeightLb: 120, weightMultiplierLb: 7 },
  },
  'race:halfling': {
    male: { baseHeightInches: 32, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 30, weightMultiplierLb: 1 },
    female: { baseHeightInches: 30, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 25, weightMultiplierLb: 1 },
  },
};

/**
 * Wire rows to picker options. Pure: adds only the hand-entered body profile
 * (or an explicit `null`), reorders nothing, and invents no value.
 */
export function raceOptionsFromChassis(races: RaceCreationChassisDto[]): RaceOption[] {
  return races.map((race) => ({
    id: race.raceId,
    label: race.label,
    book: race.book,
    abilityAdjustments: { ...race.abilityAdjustments },
    floatingBonusPoints: race.floatingBonusPoints,
    size: race.size,
    vision: race.vision,
    baseSpeedFt: race.baseSpeedFt,
    body: RACE_BODY_PROFILES[race.raceId] ?? null,
  }));
}

/**
 * Invokes the `list_race_creation_roster` Tauri command.
 *
 * Throws without the desktop runtime rather than returning sample races.
 * Creation is already Tauri-only — `loadCreateCharacter` throws the same way —
 * so a browser-preview roster would be a picker whose every choice fails at
 * submit, and its sample values would be fixture data standing where real
 * rules values are read.
 */
export async function loadRaceCreationRoster(): Promise<RaceCreationRosterResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Character creation needs the desktop app: the race roster is served from the on-disk race corpus.');
  }
  try {
    return await invoke<RaceCreationRosterResponse>('list_race_creation_roster');
  } catch (cause: unknown) {
    throw new Error(`Failed to load the race roster: ${formatError(cause)}`);
  }
}

/**
 * The roster as the UI consumes it: options plus whatever the backend
 * reported it could not serve.
 */
export interface RaceRosterSurface {
  options: RaceOption[];
  diagnostics: string[];
}

export async function loadRaceRosterSurface(): Promise<RaceRosterSurface> {
  const response = await loadRaceCreationRoster();
  return { options: raceOptionsFromChassis(response.races), diagnostics: response.diagnostics };
}

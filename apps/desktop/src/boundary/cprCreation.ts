import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over Cyberpunk RED creation (SD-38 E3.1):
 * `preview_cyberpunk_character` and `create_cyberpunk_character`
 * (`apps/desktop/src-tauri/src/cpr_creation.rs`). The STATs come from the Role's template
 * (Streetrat row rolled or picked; Edgerunner one 1d10 per STAT) or the player's Complete
 * Package allocation, and every number is the engine's `cpr.` row. The dice come from one
 * seedable roller: send the preview's `seed` back on create to keep the previewed dice.
 *
 * SD-38 E3.3 adds the Lifepath: every general table and the Role's tables are rolled with the
 * same seed or picked (`lifepath`), the answers are saved with the character, and the Cultural
 * Origin language comes back as the engine's `cpr.skill_level.language.*` row (level 4).
 *
 * SD-38 E3.4 adds the skills allocator (`skills`): Streetrat takes the Role's printed list,
 * Edgerunner moves the same 20 skills (2-6), Complete Package buys any of the 66 (cap 6, Basic
 * skills at 2); the engine validates the allocation and returns the points spent and left as
 * its `cpr.skills.creation_points_*` rows.
 */

export type CyberpunkCreationMethod = 'streetrat' | 'edgerunner' | 'completePackage';

export interface CyberpunkCreationRequest {
  /** A `role` record name, e.g. `Solo`. */
  role: string;
  method: CyberpunkCreationMethod;
  /** The roller's seed (0 to 4294967295); absent, Rust draws one and returns it. */
  seed?: number | null;
  /** Streetrat only: the picked template row, 1-10; absent, it is rolled on 1d10. */
  templateRoll?: number | null;
  /** Complete Package only: the allocation keyed by STAT id (`int` ... `emp`). */
  stats?: Record<string, number> | null;
  /** The Lifepath picks; every table not picked is rolled. */
  lifepath?: CyberpunkLifepathRequest | null;
  /** The skill allocation; anything not sent starts where the method starts. */
  skills?: CyberpunkSkillsRequest | null;
}

/**
 * Streetrat and Edgerunner key `levels` by the Role list's line (`Tactics`,
 * `Play Instrument (choose 1)`); Complete Package by skill (`Martial Arts (Karate)`), 0 dropping it.
 */
export interface CyberpunkSkillsRequest {
  levels: Record<string, number>;
  /** The named instrument or field of each `(choose 1)` line, by line. */
  specialties: Record<string, string>;
}

export interface CyberpunkSkillLine {
  /** The key a level change is sent under. */
  line: string;
  /** The skill key saved (`Play Instrument (Guitar)`). */
  key: string;
  level: number;
  basic: boolean;
  x2: boolean;
  /** The Cultural Origin language (its 4 levels are free). */
  cultural: boolean;
  /** The method lets the player change this level. */
  adjustable: boolean;
  /** A `(choose 1)` line: the player names the instrument or field. */
  chooseOne: boolean;
  /** The id of this skill's `cpr.skill_level.*` row. */
  levelRow: string;
}

export interface CyberpunkSkills {
  lines: CyberpunkSkillLine[];
  /** The engine's `cpr.skills.creation_points_spent` and `_remaining` rows; empty while refused. */
  rows: CyberpunkCreationRow[];
}

/** One picked Lifepath row: `entry` 0 for a table answered once, 1.. for the nth friend/enemy/affair. */
export interface CyberpunkLifepathPick {
  table: string;
  entry: number;
  /** 0 unless the table is rolled "for each column". */
  column: number;
  roll: number;
}

export interface CyberpunkLifepathRequest {
  picks: CyberpunkLifepathPick[];
  /** Picked counts of the "Roll 1d10 and subtract 7" tables, by table name. */
  counts: Record<string, number>;
  /** Tables behind a printed choice the player did not take. */
  skips: string[];
  /** The Cultural Origin language; null rolls one of the region's. */
  culturalLanguage: string | null;
}

export interface CyberpunkLifepathAnswer {
  entry: number;
  columnIndex: number;
  column: string;
  roll: number;
  picked: boolean;
  text: string;
  detail: string;
}

export interface CyberpunkLifepathTable {
  table: string;
  group: string;
  sourcePage: string;
  die: string;
  rollInstruction: string;
  count: number | null;
  countMax: number | null;
  countPicked: boolean;
  skippable: boolean;
  skipped: boolean;
  perColumn: boolean;
  answers: CyberpunkLifepathAnswer[];
}

export interface CyberpunkCulturalLanguage {
  name: string;
  picked: boolean;
  /** The `Language (<name>)` skill key it is saved under. */
  skill: string;
  level: number;
}

export interface CyberpunkCreateRequest {
  characterId: string;
  displayLabel: string;
  savedAt: string;
  creation: CyberpunkCreationRequest;
}

export interface CyberpunkRoll {
  /** `streetrat_row`, or `stat.<id>` for an Edgerunner roll. */
  table: string;
  die: string;
  result: number;
}

export interface CyberpunkStatValue {
  id: string;
  label: string;
  value: number;
}

export interface CyberpunkCreationRow {
  id: string;
  value: number;
  detail: string;
}

export interface CyberpunkCreationProblem {
  id: string;
  message: string;
  claimBlocking: boolean;
}

export interface CyberpunkCreationPreview {
  role: string;
  method: CyberpunkCreationMethod;
  seed: number;
  rolls: CyberpunkRoll[];
  templateRoll: number | null;
  stats: CyberpunkStatValue[];
  statPointsSpent: number | null;
  statPointsTotal: number | null;
  /** Every Lifepath table answered (general, then the Role's). */
  lifepath: CyberpunkLifepathTable[];
  culturalLanguage: CyberpunkCulturalLanguage | null;
  /** The skill allocation and the engine's points rows. */
  skills: CyberpunkSkills;
  /** The engine's `cpr.stat.*`, `cpr.derived.*`, skill and skill-points rows; empty while a problem stands. */
  explanations: CyberpunkCreationRow[];
  problems: CyberpunkCreationProblem[];
}

export interface CyberpunkCharacterSummary {
  characterId: string;
  displayLabel: string;
  gameSystem: string;
  schemaVersion: number;
  savedAt: string;
  /** Empty: a Cyberpunk RED character has no Pathfinder race or class levels. */
  raceId: string;
  classSummary: string;
}

export type CyberpunkCreateResponse =
  | { kind: 'Saved'; summary: CyberpunkCharacterSummary; seed: number; explanations: CyberpunkCreationRow[] }
  | { kind: 'Blocked'; diagnostics: CyberpunkCreationProblem[] };

function wire(request: CyberpunkCreationRequest) {
  return {
    role: request.role,
    method: request.method,
    seed: request.seed ?? null,
    templateRoll: request.templateRoll ?? null,
    stats: request.stats ?? null,
    lifepath: request.lifepath ?? { picks: [], counts: {}, skips: [], culturalLanguage: null },
    skills: request.skills ?? { levels: {}, specialties: {} },
  };
}

export async function previewCyberpunkCharacter(request: CyberpunkCreationRequest): Promise<CyberpunkCreationPreview> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for previewing a Cyberpunk RED character');
  }

  try {
    return await invoke<CyberpunkCreationPreview>('preview_cyberpunk_character', { request: wire(request) });
  } catch (cause: unknown) {
    throw new Error(`Failed to preview the Cyberpunk RED character: ${formatError(cause)}`);
  }
}

export async function createCyberpunkCharacter(request: CyberpunkCreateRequest): Promise<CyberpunkCreateResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for creating a Cyberpunk RED character');
  }

  try {
    return await invoke<CyberpunkCreateResponse>('create_cyberpunk_character', {
      request: {
        characterId: request.characterId,
        displayLabel: request.displayLabel,
        savedAt: request.savedAt,
        creation: wire(request.creation),
      },
    });
  } catch (cause: unknown) {
    throw new Error(`Failed to create the Cyberpunk RED character: ${formatError(cause)}`);
  }
}

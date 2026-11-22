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
 *
 * SD-38 E3.5 adds the Role Ability (`roleAbility`): every Role starts at Rank 4, and the
 * sub-choices the rank asks for (Combat Awareness points, Maker specialty ranks, Medicine points
 * and drugs, Family Motorpool picks, the Exec's Team Member) are listed, checked and answered with
 * `cpr.role.<role>.` rows by the engine.
 *
 * SD-38 E3.6 adds the starting kit (`kit`): Streetrat and Edgerunner take the Role's fixed kit
 * (one option of each "or" line, the first printed until picked) and 500eb; a Complete Package
 * buys everything with 2,550eb and an 800eb fashion-only budget lost if unspent. Every method buys
 * from the engine's shop at the printed prices; the money is the engine's `cpr.kit.*` rows and
 * `cpr.cash`, the cash left.
 *
 * SD-38 E3.7 adds cyberware: the Role's pack is the fixed kit's `cyberware` lines (an "or" line is
 * a kit pick), a Complete Package buys pieces from the shop's `cyberware` kind (Fashionware from
 * the fashion budget first); the engine places each piece, refuses an install over a container's
 * slots (`cpr_creation.cyberware`), and answers with the Humanity Loss, Humanity, EMP, maximum
 * Humanity and slot rows (`cyberware`).
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
  /** The Role Ability sub-choices; none sent, none made. */
  roleAbility?: CyberpunkRoleAbilityRequest | null;
  /** The starting kit's "or" picks and purchases; none sent, the fixed kit's first options and nothing bought. */
  kit?: CyberpunkKitRequest | null;
}

/** The option an "or" line of the fixed kit takes, by the line's index in `kit.lines`. */
export interface CyberpunkKitPick {
  line: number;
  option: string;
}

/** One purchase: a shop kind and record name, its variant (armour location, ammunition caliber, fashion piece) and how many. */
export interface CyberpunkKitPurchase {
  kind: string;
  name: string;
  variant: string | null;
  quantity: number;
}

export interface CyberpunkKitRequest {
  picks: CyberpunkKitPick[];
  purchases: CyberpunkKitPurchase[];
}

export interface CyberpunkKitOption {
  name: string;
  quantity: number;
}

/** One printed line of the fixed kit; an "or" line takes one option (`taken`). */
export interface CyberpunkKitLine {
  /** `weapons_armor`, `outfit` or `cyberware` (the Role's pack, p117). */
  section: string;
  text: string;
  sourcePage: string;
  pickOne: boolean;
  fashion: boolean;
  options: CyberpunkKitOption[];
  /** The option an "or" line takes (the first until picked); empty for a plain line. */
  taken: string;
  /** The player picked `taken`. */
  picked: boolean;
}

export interface CyberpunkShopVariant {
  name: string;
  /** A fashion piece's own price. */
  costEb: number | null;
}

/** One buyable record at its printed price (ammunition: per ten rounds, or per grenade or rocket). */
export interface CyberpunkShopItem {
  kind: string;
  name: string;
  costEb: number;
  costText: string;
  sourcePage: string;
  /** `Location`, `Caliber` or `Piece` when the purchase names a variant. */
  variantLabel: string | null;
  variantNote: string;
  variants: CyberpunkShopVariant[];
}

/** One item the character starts with, as the engine built it. */
export interface CyberpunkKitItem {
  /** `weapons_armor`, `outfit` or `cyberware` (fixed kit), or `bought`. */
  section: string;
  kind: string;
  name: string;
  quantity: number;
  /** The engine's price of this purchase (0 for a fixed-kit item). */
  costEb: number;
  fashion: boolean;
}

export interface CyberpunkKit {
  method: 'fixed' | 'completePackage';
  lines: CyberpunkKitLine[];
  /** Empty while the kit is refused. */
  items: CyberpunkKitItem[];
  shop: CyberpunkShopItem[];
  /** The engine's `cpr.kit.*` rows and `cpr.cash`; empty while refused. */
  rows: CyberpunkCreationRow[];
}

/** One installed piece: as printed or bought, the record it is, and where the engine placed it. */
export interface CyberpunkCyberwarePiece {
  printed: string;
  /** The cyberware record, as saved. */
  name: string;
  /** `eye 0`, `arm 1`, `meat_arm 0`; null for a piece that sits in no foundation. */
  mount: string | null;
  bought: boolean;
}

export interface CyberpunkCyberware {
  pieces: CyberpunkCyberwarePiece[];
  /** The engine's `cpr.cyberware.*` rows and `cpr.derived.humanity_max`/`humanity_current`/`emp_current`; empty while refused. */
  rows: CyberpunkCreationRow[];
}

/** One Role Ability pick, with what it goes on (an upgrade's vehicle, a Team Member's chart row). */
export interface CyberpunkRolePick {
  name: string;
  on: string | null;
}

/** Points by ability or specialty (Combat Awareness, Maker, Medicine) and picks (Motorpool, Team Members, drugs). */
export interface CyberpunkRoleAbilityRequest {
  points: Record<string, number>;
  picks: CyberpunkRolePick[];
}

export interface CyberpunkPointsOption {
  name: string;
  max: number;
  /** The values this option takes, 0 first (a tiered ability's printed costs). */
  steps: number[];
  text: string;
}

export interface CyberpunkPointsMenu {
  label: string;
  total: number;
  /** Every point must be placed (Maker, Medicine). */
  exact: boolean;
  sourcePage: string;
  options: CyberpunkPointsOption[];
}

export interface CyberpunkPickOption {
  name: string;
  /** `vehicle`, `upgrade`, `pharmaceutical` or `team_member`. */
  kind: string;
  needsTarget: boolean;
  /** Fixed targets (a Team Member's chart rows); empty when the targets are earlier picks (an upgrade's vehicle). */
  targets: string[];
  text: string;
}

export interface CyberpunkPicksMenu {
  label: string;
  /** The most picks the choices allow. */
  count: number;
  sourcePage: string;
  options: CyberpunkPickOption[];
}

/** What the Role Ability at its rank divides or picks, from the engine. */
export interface CyberpunkRoleChoiceMenu {
  ability: string;
  rank: number;
  points: CyberpunkPointsMenu | null;
  picks: CyberpunkPicksMenu | null;
}

export interface CyberpunkRoleAbility {
  ability: string;
  rank: number;
  menu: CyberpunkRoleChoiceMenu;
  /** The engine's `cpr.role.<role>.` rows (the rank's and the choices'); empty while refused. */
  rows: CyberpunkCreationRow[];
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
  /** The Role Ability at its starting rank: the engine's menu and rows. */
  roleAbility: CyberpunkRoleAbility | null;
  /** The starting kit: fixed lines, items, the shop and the engine's money rows. */
  kit: CyberpunkKit | null;
  /** The cyberware, placed, and the engine's Humanity/EMP rows. */
  cyberware: CyberpunkCyberware | null;
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
    roleAbility: request.roleAbility ?? { points: {}, picks: [] },
    kit: request.kit ?? { picks: [], purchases: [] },
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

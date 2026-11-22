import type {
  CyberpunkCreateRequest,
  CyberpunkCreationMethod,
  CyberpunkCreationPreview,
  CyberpunkCreationProblem,
  CyberpunkCreationRequest,
  CyberpunkCreationRow,
  CyberpunkKitLine,
  CyberpunkKitPurchase,
  CyberpunkKitRequest,
  CyberpunkLifepathRequest,
  CyberpunkPickOption,
  CyberpunkPointsOption,
  CyberpunkRoleAbilityRequest,
  CyberpunkRoleChoiceMenu,
  CyberpunkShopItem,
  CyberpunkSkillLine,
  CyberpunkSkillsRequest,
  CyberpunkStatValue,
} from '../boundary/cprCreation';
import type { CyberpunkRecord } from '../boundary/loadCyberpunkCatalog';

/**
 * Pure state for the Cyberpunk RED creation flow (`CyberpunkCreateForm.tsx`): the method picker
 * and the STAT step. The form holds only the player's choices (name, Role, method, the picked
 * Streetrat row, the Complete Package allocation) and the roller's seed; every number it shows
 * (each STAT, the points spent and to spend, the derived rows) is the engine's answer from
 * `preview_cyberpunk_character`. Nothing here is a Cyberpunk RED rules table.
 *
 * The Lifepath (E3.3): the draft holds only the player's picks, counts, not-taken branch tables
 * and Cultural Origin language; the tables offered and their rows are the `lifepath_table`
 * records, and every answer not picked is the engine's roll.
 *
 * The skills allocator (E3.4): the draft holds only the levels the player moved and the named
 * instrument or field of each `(choose 1)` line. Which skills are listed, their levels, whether
 * the method lets them move, and the points spent and left are the engine's preview; the
 * Complete Package add-list is the `skill` records.
 *
 * The Role Ability (E3.5): the draft holds only the points the player placed and the picks made.
 * What can be divided or picked, each option's allowed values, the rank and every number shown
 * are the engine's preview (`roleAbility.menu`, `roleAbility.rows`).
 *
 * The starting kit (E3.6): the draft holds only the "or" picks (by line) and the purchases. The
 * kit lines, the items, the shop and its prices, and every money figure (budget, spent, cash left,
 * the fashion budget) are the engine's preview (`kit`).
 */

export interface CyberpunkCreationDraft {
  displayLabel: string;
  /** A `role` record name, e.g. `Solo`; null until chosen. */
  role: string | null;
  method: CyberpunkCreationMethod;
  /** The roller's seed: null asks the engine to draw one; kept once drawn so create rolls the same dice. */
  seed: number | null;
  /** Streetrat: the picked template row; null rolls it on 1d10. */
  templateRoll: number | null;
  /** Complete Package: the allocation keyed by STAT id. */
  stats: Record<string, number>;
  /** The Lifepath choices; everything not chosen is rolled with the seed. */
  lifepath: CyberpunkLifepathRequest;
  /** The skill levels the player moved and the named `(choose 1)` specialties. */
  skills: CyberpunkSkillsRequest;
  /** The Role Ability points placed and picks made. */
  roleAbility: CyberpunkRoleAbilityRequest;
  /** The starting kit's "or" picks and purchases. */
  kit: CyberpunkKitRequest;
}

export const EMPTY_LIFEPATH: CyberpunkLifepathRequest = { picks: [], counts: {}, skips: [], culturalLanguage: null };
export const EMPTY_SKILLS: CyberpunkSkillsRequest = { levels: {}, specialties: {} };
export const EMPTY_ROLE_ABILITY: CyberpunkRoleAbilityRequest = { points: {}, picks: [] };
export const EMPTY_KIT: CyberpunkKitRequest = { picks: [], purchases: [] };

export const EMPTY_CYBERPUNK_DRAFT: CyberpunkCreationDraft = {
  displayLabel: '',
  role: null,
  method: 'streetrat',
  seed: null,
  templateRoll: null,
  stats: {},
  lifepath: EMPTY_LIFEPATH,
  skills: EMPTY_SKILLS,
  roleAbility: EMPTY_ROLE_ABILITY,
  kit: EMPTY_KIT,
};

/** The three ways to make STATs (p. 73), in the book's order. */
export const CYBERPUNK_CREATION_METHODS: readonly { id: CyberpunkCreationMethod; label: string; detail: string }[] = [
  { id: 'streetrat', label: 'Streetrat', detail: "One row of your Role's template, rolled on 1d10 or picked." },
  { id: 'edgerunner', label: 'Edgerunner', detail: "Roll 1d10 for each STAT on your Role's template." },
  { id: 'completePackage', label: 'Complete Package', detail: 'Spend the STAT points yourself.' },
];

/** The STAT-step request for the draft; null until a Role is chosen. */
export function buildCyberpunkPreviewRequest(draft: CyberpunkCreationDraft): CyberpunkCreationRequest | null {
  if (draft.role === null) {
    return null;
  }
  return {
    role: draft.role,
    method: draft.method,
    seed: draft.seed,
    templateRoll: draft.method === 'streetrat' ? draft.templateRoll : null,
    stats: draft.method === 'completePackage' ? { ...draft.stats } : null,
    lifepath: {
      picks: draft.lifepath.picks.map((pick) => ({ ...pick })),
      counts: { ...draft.lifepath.counts },
      skips: [...draft.lifepath.skips],
      culturalLanguage: draft.lifepath.culturalLanguage,
    },
    skills: { levels: { ...draft.skills.levels }, specialties: { ...draft.skills.specialties } },
    roleAbility: {
      points: { ...draft.roleAbility.points },
      picks: draft.roleAbility.picks.map((pick) => ({ ...pick })),
    },
    kit: {
      picks: draft.kit.picks.map((pick) => ({ ...pick })),
      purchases: draft.kit.purchases.map((purchase) => ({ ...purchase })),
    },
  };
}

/** The create request: a fresh identity plus exactly the previewed STAT step (same seed, same dice). */
export function buildCyberpunkCreateRequest(
  draft: CyberpunkCreationDraft,
  deps: { generateId: () => string; now: () => string },
): CyberpunkCreateRequest | null {
  const creation = buildCyberpunkPreviewRequest(draft);
  if (creation === null) {
    return null;
  }
  return {
    characterId: deps.generateId(),
    displayLabel: draft.displayLabel.trim(),
    savedAt: deps.now(),
    creation,
  };
}

/** Keeps the seed the engine drew; the same seed returns the same draft (no preview loop). */
export function withPreviewSeed(draft: CyberpunkCreationDraft, preview: Pick<CyberpunkCreationPreview, 'seed'>): CyberpunkCreationDraft {
  return draft.seed === preview.seed ? draft : { ...draft, seed: preview.seed };
}

/** Drops the seed so the next preview draws fresh dice. */
export function rollAgain(draft: CyberpunkCreationDraft): CyberpunkCreationDraft {
  return { ...draft, seed: null };
}

/**
 * Changes the method: the dice and the Streetrat row reset; a Complete Package with no allocation
 * yet starts from the STATs on screen (`shown`, the engine's preview), which the player then moves.
 * The skill levels start over (each method keys them differently); named specialties are kept.
 * The kit starts over (a Complete Package has no fixed kit and its own budget).
 */
export function selectMethod(
  draft: CyberpunkCreationDraft,
  method: CyberpunkCreationMethod,
  shown: readonly CyberpunkStatValue[],
): CyberpunkCreationDraft {
  if (draft.method === method) {
    return draft;
  }
  const startFromShown = method === 'completePackage' && Object.keys(draft.stats).length === 0;
  const stats = startFromShown ? Object.fromEntries(shown.map((stat) => [stat.id, stat.value])) : draft.stats;
  return { ...draft, method, seed: null, templateRoll: null, stats, skills: { ...draft.skills, levels: {} }, kit: EMPTY_KIT };
}

/** One point up or down on a STAT, never below zero; the 2-8 range and the total are the engine's to refuse. */
export function adjustStat(stats: Record<string, number>, id: string, delta: number): Record<string, number> {
  return { ...stats, [id]: Math.max(0, (stats[id] ?? 0) + delta) };
}

export interface TemplateRowOption {
  roll: number;
  label: string;
}

/** The chosen Role's Streetrat rows, read from its `streetrat_template` records, by roll. */
export function templateRowOptions(records: readonly CyberpunkRecord[], role: string | null): TemplateRowOption[] {
  if (role === null) {
    return [];
  }
  return records
    .filter((record) => record.role === role && typeof record.roll === 'number')
    .map((record) => ({
      roll: record.roll as number,
      label: typeof record.text === 'string' ? record.text : `Roll ${String(record.roll)}`,
    }))
    .sort((a, b) => a.roll - b.roll);
}

/** The problems that keep the character from being created. */
export function blockingProblems(preview: Pick<CyberpunkCreationPreview, 'problems'>): CyberpunkCreationProblem[] {
  return preview.problems.filter((problem) => problem.claimBlocking);
}

export interface CyberpunkTile {
  id: string;
  label: string;
  value: number;
}

/** The derived rows the Create screen shows, in sheet order. */
const DERIVED_ROWS: readonly { id: string; label: string }[] = [
  { id: 'cpr.derived.hit_points', label: 'Hit Points' },
  { id: 'cpr.derived.seriously_wounded', label: 'Seriously Wounded' },
  { id: 'cpr.derived.death_save', label: 'Death Save' },
  { id: 'cpr.derived.humanity_max', label: 'Humanity' },
  { id: 'cpr.derived.emp_current', label: 'Current EMP' },
];

/** One tile per `cpr.derived.*` row the engine returned; a row it did not return gets no tile. */
export function cyberpunkDerivedTiles(rows: readonly CyberpunkCreationRow[]): CyberpunkTile[] {
  return DERIVED_ROWS.flatMap((wanted) => {
    const found = rows.find((row) => row.id === wanted.id);
    return found ? [{ id: found.id, label: wanted.label, value: found.value }] : [];
  });
}

/** One tile per `cpr.stat.*` row the engine returned, labelled by the engine's STAT list. */
export function cyberpunkStatTiles(rows: readonly CyberpunkCreationRow[], stats: readonly CyberpunkStatValue[]): CyberpunkTile[] {
  return stats.flatMap((stat) => {
    const found = rows.find((row) => row.id === `cpr.stat.${stat.id}`);
    return found ? [{ id: found.id, label: stat.label, value: found.value }] : [];
  });
}

// ---- Lifepath (E3.3) ----

/** The Lifepath tables offered for `role`: every general `lifepath_table` record, then the Role's, corpus order. */
export function lifepathTablesFor(records: readonly CyberpunkRecord[], role: string | null): CyberpunkRecord[] {
  if (role === null) {
    return [];
  }
  return [...records.filter((r) => r.group === 'general'), ...records.filter((r) => r.group === 'role' && r.role === role)];
}

interface LifepathRecordRow {
  roll: number | string;
  cells: string[];
}

function recordRows(record: CyberpunkRecord): LifepathRecordRow[] {
  return Array.isArray(record.rows) ? (record.rows as LifepathRecordRow[]) : [];
}

function perColumn(record: CyberpunkRecord): boolean {
  const instruction = typeof record.roll_instruction === 'string' ? record.roll_instruction : '';
  return instruction.includes('each column') || instruction.includes('each of the columns');
}

export interface LifepathRowOption {
  /** The roll a pick sends (the first roll of a "1-2" row). */
  roll: number;
  label: string;
}

/**
 * The rows a Lifepath answer can be picked from, read from the record: the printed roll and the
 * cell of `column` for a table rolled "for each column", the whole row otherwise.
 */
export function lifepathRowOptions(record: CyberpunkRecord, column: number): LifepathRowOption[] {
  const each = perColumn(record);
  return recordRows(record).map((row) => {
    const printed = String(row.roll);
    const cells = each ? [row.cells[column] ?? ''] : row.cells;
    return { roll: Number(printed.split('-')[0]), label: `${printed}: ${cells.join(' / ')}` };
  });
}

/** The Cultural Origins record's languages for the rolled or picked region. */
export function culturalLanguageOptions(record: CyberpunkRecord, regionRoll: number | null): string[] {
  if (regionRoll === null || typeof record.languages_by_roll !== 'object' || record.languages_by_roll === null) {
    return [];
  }
  const listed = (record.languages_by_roll as Record<string, unknown>)[String(regionRoll)];
  return Array.isArray(listed) ? listed.map(String) : [];
}

/** Picks one answer's row; null drops the pick so the engine rolls it. The seed is kept. */
export function setLifepathPick(
  draft: CyberpunkCreationDraft,
  table: string,
  entry: number,
  column: number,
  roll: number | null,
): CyberpunkCreationDraft {
  const others = draft.lifepath.picks.filter((p) => !(p.table === table && p.entry === entry && p.column === column));
  const picks = roll === null ? others : [...others, { table, entry, column, roll }];
  return { ...draft, lifepath: { ...draft.lifepath, picks } };
}

/** Picks how many entries a count table has; null rolls it. A smaller count drops the picks of the removed entries. */
export function setLifepathCount(draft: CyberpunkCreationDraft, table: string, count: number | null): CyberpunkCreationDraft {
  const counts = { ...draft.lifepath.counts };
  let picks = draft.lifepath.picks;
  if (count === null) {
    delete counts[table];
  } else {
    counts[table] = count;
    picks = picks.filter((p) => p.table !== table || p.entry <= count);
  }
  return { ...draft, lifepath: { ...draft.lifepath, counts, picks } };
}

/** Marks a table behind a printed choice as not taken (or taken again). */
export function setLifepathSkip(draft: CyberpunkCreationDraft, table: string, skipped: boolean): CyberpunkCreationDraft {
  const others = draft.lifepath.skips.filter((name) => name !== table);
  return { ...draft, lifepath: { ...draft.lifepath, skips: skipped ? [...others, table] : others } };
}

/** The Cultural Origin language; empty rolls one of the region's. */
export function setCulturalLanguage(draft: CyberpunkCreationDraft, name: string | null): CyberpunkCreationDraft {
  const trimmed = name?.trim() ?? '';
  return { ...draft, lifepath: { ...draft.lifepath, culturalLanguage: trimmed === '' ? null : trimmed } };
}

/** Changes the Role: fresh dice, no template row, only the general Lifepath choices kept, skill levels, Role Ability choices and kit start over. */
export function selectRole(draft: CyberpunkCreationDraft, role: string | null, lifepathRecords: readonly CyberpunkRecord[]): CyberpunkCreationDraft {
  const general = new Set(lifepathRecords.filter((r) => r.group === 'general').map((r) => String(r.name)));
  const counts = Object.fromEntries(Object.entries(draft.lifepath.counts).filter(([table]) => general.has(table)));
  return {
    ...draft,
    role,
    seed: null,
    templateRoll: null,
    skills: { ...draft.skills, levels: {} },
    roleAbility: EMPTY_ROLE_ABILITY,
    kit: EMPTY_KIT,
    lifepath: {
      ...draft.lifepath,
      picks: draft.lifepath.picks.filter((p) => general.has(p.table)),
      counts,
      skips: draft.lifepath.skips.filter((name) => general.has(name)),
    },
  };
}

/** The Cultural Origin language's level tile: the engine's `cpr.skill_level.*` row of its skill line, or none. */
export function culturalLanguageTile(
  preview: Pick<CyberpunkCreationPreview, 'culturalLanguage' | 'explanations' | 'skills'>,
): CyberpunkTile | null {
  const language = preview.culturalLanguage;
  const line = preview.skills.lines.find((l) => l.cultural);
  if (language === null || line === undefined) {
    return null;
  }
  const found = preview.explanations.find((row) => row.id === line.levelRow);
  return found ? { id: found.id, label: language.skill, value: found.value } : null;
}

// ---- Skills allocator (E3.4) ----

/** One level up or down from the level the engine showed, never below zero; the 2-6 range, the cap and the 86 points are the engine's to refuse. */
export function adjustSkill(draft: CyberpunkCreationDraft, line: CyberpunkSkillLine, delta: number): CyberpunkCreationDraft {
  const levels = { ...draft.skills.levels, [line.line]: Math.max(0, line.level + delta) };
  return { ...draft, skills: { ...draft.skills, levels } };
}

/** Names the instrument or field of a `(choose 1)` line; empty clears it. */
export function setSkillSpecialty(draft: CyberpunkCreationDraft, line: string, name: string): CyberpunkCreationDraft {
  const specialties = { ...draft.skills.specialties };
  const trimmed = name.trim();
  if (trimmed === '') {
    delete specialties[line];
  } else {
    specialties[line] = trimmed;
  }
  return { ...draft, skills: { ...draft.skills, specialties } };
}

/** A skill key: `Handgun`, or `Martial Arts (Karate)` for a skill with a specialty. */
export function skillKey(name: string, specialty: string): string {
  const trimmed = specialty.trim();
  return trimmed === '' ? name : `${name} (${trimmed})`;
}

/** Complete Package: buys a skill at level 1 (a held skill keeps its level). */
export function addSkill(draft: CyberpunkCreationDraft, key: string): CyberpunkCreationDraft {
  if ((draft.skills.levels[key] ?? 0) > 0) {
    return draft;
  }
  return { ...draft, skills: { ...draft.skills, levels: { ...draft.skills.levels, [key]: 1 } } };
}

/** Complete Package: drops a skill (sent at 0; a Basic skill dropped is the engine's to refuse). */
export function dropSkill(draft: CyberpunkCreationDraft, key: string): CyberpunkCreationDraft {
  return { ...draft, skills: { ...draft.skills, levels: { ...draft.skills.levels, [key]: 0 } } };
}

export interface SkillOption {
  name: string;
  stat: string;
  x2: boolean;
  specialtyRequired: boolean;
}

/** The Master Skill List the Complete Package buys from: the `skill` records, in record order. */
export function skillOptions(records: readonly CyberpunkRecord[]): SkillOption[] {
  return records.map((record) => ({
    name: String(record.name),
    stat: String(record.stat),
    x2: record.x2 === true,
    specialtyRequired: record.specialty_required === true,
  }));
}

const SKILL_POINT_ROWS: readonly { id: string; label: string }[] = [
  { id: 'cpr.skills.creation_points_spent', label: 'Skill points spent' },
  { id: 'cpr.skills.creation_points_remaining', label: 'Skill points left' },
];

/** The points spent and left: the engine's `cpr.skills.creation_points_*` rows; none while the allocation is refused. */
export function skillPointTiles(preview: Pick<CyberpunkCreationPreview, 'skills'>): CyberpunkTile[] {
  return SKILL_POINT_ROWS.flatMap((wanted) => {
    const found = preview.skills.rows.find((row) => row.id === wanted.id);
    return found ? [{ id: found.id, label: wanted.label, value: found.value }] : [];
  });
}

// ---- Role Ability (E3.5) ----

/**
 * Moves an option's points to its next allowed value up or down (the engine's `steps`: a tiered
 * ability jumps between its printed costs); 0 drops it from the request. No step beyond the ends.
 * The pool total and the rules are the engine's to refuse.
 */
export function adjustRolePoints(draft: CyberpunkCreationDraft, option: CyberpunkPointsOption, delta: 1 | -1): CyberpunkCreationDraft {
  const current = draft.roleAbility.points[option.name] ?? 0;
  const next = delta > 0 ? option.steps.find((step) => step > current) : [...option.steps].reverse().find((step) => step < current);
  if (next === undefined) {
    return draft;
  }
  const points = { ...draft.roleAbility.points };
  if (next === 0) {
    delete points[option.name];
  } else {
    points[option.name] = next;
  }
  return { ...draft, roleAbility: { ...draft.roleAbility, points } };
}

/** Adds one pick, with what it goes on (null for a pick that goes on nothing). */
export function addRolePick(draft: CyberpunkCreationDraft, name: string, on: string | null): CyberpunkCreationDraft {
  return { ...draft, roleAbility: { ...draft.roleAbility, picks: [...draft.roleAbility.picks, { name, on }] } };
}

/** Removes the pick at `index`. */
export function removeRolePick(draft: CyberpunkCreationDraft, index: number): CyberpunkCreationDraft {
  return { ...draft, roleAbility: { ...draft.roleAbility, picks: draft.roleAbility.picks.filter((_, i) => i !== index) } };
}

/**
 * What a pick can go on: the engine's fixed targets (a Team Member's chart rows), or, for an
 * option whose targets are earlier picks (an upgrade), the menu's vehicles the draft has picked.
 */
export function rolePickTargets(menu: CyberpunkRoleChoiceMenu, option: CyberpunkPickOption, draft: CyberpunkCreationDraft): string[] {
  if (!option.needsTarget) {
    return [];
  }
  if (option.targets.length > 0) {
    return option.targets;
  }
  const vehicles = new Set((menu.picks?.options ?? []).filter((o) => o.kind === 'vehicle').map((o) => o.name));
  return draft.roleAbility.picks.filter((pick) => vehicles.has(pick.name)).map((pick) => pick.name);
}

/** One tile per engine `cpr.role.*` row of the Role Ability, labelled by the row's detail up to its colon. */
export function roleAbilityTiles(preview: Pick<CyberpunkCreationPreview, 'roleAbility'>): CyberpunkTile[] {
  return (preview.roleAbility?.rows ?? []).map((row) => ({ id: row.id, label: row.detail.split(':')[0], value: row.value }));
}

// ---- Starting kit + cash (E3.6) ----

/** Picks `option` on the fixed kit's "or" line `line`, replacing an earlier pick of that line. Whether it is an option is the engine's to refuse. */
export function setKitPick(draft: CyberpunkCreationDraft, line: number, option: string): CyberpunkCreationDraft {
  const picks = draft.kit.picks.filter((pick) => pick.line !== line);
  const at = picks.findIndex((pick) => pick.line > line);
  picks.splice(at === -1 ? picks.length : at, 0, { line, option });
  return { ...draft, kit: { ...draft.kit, picks } };
}

/** Adds one purchase; its price and the budget are the engine's to answer or refuse. */
export function addKitPurchase(draft: CyberpunkCreationDraft, purchase: CyberpunkKitPurchase): CyberpunkCreationDraft {
  return { ...draft, kit: { ...draft.kit, purchases: [...draft.kit.purchases, { ...purchase }] } };
}

/** Removes the purchase at `index`. */
export function removeKitPurchase(draft: CyberpunkCreationDraft, index: number): CyberpunkCreationDraft {
  return { ...draft, kit: { ...draft.kit, purchases: draft.kit.purchases.filter((_, i) => i !== index) } };
}

const KIT_MONEY_LABELS: Readonly<Record<string, string>> = {
  'cpr.kit.budget': 'Starting cash',
  'cpr.kit.spent': 'Spent',
  'cpr.cash': 'Cash left',
  'cpr.kit.fashion_budget': 'Fashion budget',
  'cpr.kit.fashion_spent': 'Fashion bought',
  'cpr.kit.fashion_left': 'Fashion left (lost if unspent)',
};

/** One tile per engine money row (`cpr.kit.*`, `cpr.cash`) among `rows`, in the engine's order; other rows are skipped. */
export function kitRowTiles(rows: readonly CyberpunkCreationRow[]): CyberpunkTile[] {
  return rows.filter((row) => row.id in KIT_MONEY_LABELS).map((row) => ({ id: row.id, label: KIT_MONEY_LABELS[row.id], value: row.value }));
}

/** The kit step's money tiles: the engine's `kit.rows`. */
export function kitMoneyTiles(preview: Pick<CyberpunkCreationPreview, 'kit'>): CyberpunkTile[] {
  return kitRowTiles(preview.kit?.rows ?? []);
}

const SHOP_KIND_LABELS: Readonly<Record<string, string>> = {
  weapon: 'Weapons',
  attachment: 'Weapon attachments',
  ammo: 'Ammunition',
  armor: 'Armor',
  gear: 'Gear',
  fashion: 'Fashion',
  drug: 'Street drugs',
  cyberdeck: 'Cyberdecks',
  net_hardware: 'Cyberdeck hardware',
  program: 'Programs',
  cyberware: 'Cyberware',
};

/** The shop's kinds, each once, in the shop's order, with a label. */
export function shopKinds(shop: readonly CyberpunkShopItem[]): { kind: string; label: string }[] {
  const seen: string[] = [];
  for (const item of shop) {
    if (!seen.includes(item.kind)) {
      seen.push(item.kind);
    }
  }
  return seen.map((kind) => ({ kind, label: SHOP_KIND_LABELS[kind] ?? kind }));
}

// ---- Cyberware at creation (E3.7) ----

/** A fixed-kit line with its index in `kit.lines` (the index an "or" pick is sent under). */
export interface IndexedKitLine {
  index: number;
  line: CyberpunkKitLine;
}

/** The Role's cyberware pack: the kit's `cyberware` lines, each with its kit index. */
export function cyberwareLines(lines: readonly CyberpunkKitLine[]): IndexedKitLine[] {
  return lines.flatMap((line, index) => (line.section === 'cyberware' ? [{ index, line }] : []));
}

/** Every other kit line (weapons and armour, outfit), each with its kit index. */
export function kitLinesBesideCyberware(lines: readonly CyberpunkKitLine[]): IndexedKitLine[] {
  return lines.flatMap((line, index) => (line.section === 'cyberware' ? [] : [{ index, line }]));
}

const CYBERWARE_TILES: readonly { id: string; label: string }[] = [
  { id: 'cpr.cyberware.humanity_loss', label: 'Humanity Loss' },
  { id: 'cpr.derived.humanity_current', label: 'Humanity' },
  { id: 'cpr.derived.emp_current', label: 'Current EMP' },
  { id: 'cpr.cyberware.humanity_cap', label: 'Maximum Humanity' },
];

/** The Humanity Loss, Humanity, EMP and maximum Humanity tiles: one per engine row present. */
export function cyberwareTiles(rows: readonly CyberpunkCreationRow[]): CyberpunkTile[] {
  return CYBERWARE_TILES.flatMap((wanted) => {
    const found = rows.find((row) => row.id === wanted.id);
    return found ? [{ id: found.id, label: wanted.label, value: found.value }] : [];
  });
}

const SLOT_USED = /^cpr\.cyberware\.slots\.(.+)\.used$/;

/** One tile per container the engine returned: its slots used, labelled with the capacity row's value. */
export function cyberwareSlotTiles(rows: readonly CyberpunkCreationRow[]): CyberpunkTile[] {
  return rows.flatMap((row) => {
    const match = SLOT_USED.exec(row.id);
    if (!match) {
      return [];
    }
    const capacity = rows.find((r) => r.id === `cpr.cyberware.slots.${match[1]}.capacity`);
    const container = row.detail.split(':')[0];
    return capacity ? [{ id: row.id, label: `${container} slots used (of ${capacity.value})`, value: row.value }] : [];
  });
}

/** The preset Humanity Loss of the `index`th installed piece: its engine row, or null. */
export function cyberwarePieceLoss(rows: readonly CyberpunkCreationRow[], index: number): number | null {
  return rows.find((row) => row.id === `cpr.cyberware.piece.${index}.humanity_loss`)?.value ?? null;
}

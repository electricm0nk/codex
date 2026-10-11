import type { ClassSpellLevelsDto } from '../boundary/loadClassSpellLevels';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';
import type { CreateSpellDto } from '../boundary/loadCreateCharacter';
import { buildSpellsPerDaySurface, type SpellsPerDayRow } from './spellsPerDayModel';
import type { SharedLimit, TransferItem } from './transferListModel';

/**
 * The Create screen's spells dialog: how many spells of each level the draft character KNOWS (the
 * engine's own spells-known and spellbook-size rows, one quota per class and spell level), the
 * spells that fill each quota, and the race's innate spell-like abilities for the read-only Innate
 * column.
 *
 * Authors no rules data. A spell's level for a class comes from `list_class_spell_levels`; a quota
 * comes from a `*.spells_known.spell_level_<n>` row (a spontaneous caster) or the wizard's
 * `class_spell.wizard.spellbook_size.*` rows. A caster that knows its whole list (Cleric, Druid ...)
 * gets no picker: its spells per day are shown as text (operator ruling 2026-10-11). So does a caster
 * the engine states no known count for: per day is not a known-spell quota.
 */

export interface SpellDialogInput {
  perDay: readonly ExplanationDto[];
  /** `draft_spell_options`' `known` rows. */
  known: readonly ExplanationDto[];
  classLevels: readonly ClassSpellLevelsDto[];
  catalog: readonly SpellCatalogEntryDto[];
  innate: readonly string[];
}

/** A caster with no picker on the Create screen, and the sentence that says why and what it casts. */
export interface ReadOnlyCaster {
  classToken: string;
  text: string;
}

export interface SpellDialog {
  items: TransferItem[];
  groupLimits: Record<string, number>;
  groupLabels: Record<string, string>;
  /** Pools spanning several levels (a wizard's spells gained after 1st level). */
  sharedLimits: SharedLimit[];
  readOnly: ReadOnlyCaster[];
}

const ID_SEPARATOR = '|';

/** Prepared casters whose spells known are their whole class list (PF1 CRB, APG, ACG). */
const WHOLE_LIST_CASTERS = new Set(['cleric', 'druid', 'paladin', 'ranger', 'warpriest', 'shaman', 'antipaladin']);

/** `class_chassis.sorcerer.spontaneous.spells_known.spell_level_1`, `class_spell.acg.bloodrager.spells_known.spell_level_2`. */
const KNOWN_ID = /^class_(?:spell|chassis)\.(?:.+\.)?(?!spontaneous\.)([a-z_]+)\.(?:spontaneous\.)?spells_known\.spell_level_(\d+)$/;

/** `class_spell.wizard.spellbook_size.spell_level_0` and `..._spell_level_2_and_higher`. */
const SPELLBOOK_ID = /^class_spell\.wizard\.spellbook_size\.spell_level_(\d+)(_and_higher)?$/;

function titleCase(token: string): string {
  return token
    .split('_')
    .map((word) => (word.length === 0 ? word : word[0]!.toUpperCase() + word.slice(1)))
    .join(' ');
}

interface Quota {
  classToken: string;
  spellLevel: number;
  count: number;
}

function knownQuotas(known: readonly ExplanationDto[]): Quota[] {
  return known.flatMap((row) => {
    const match = KNOWN_ID.exec(row.id);
    return match ? [{ classToken: match[1]!, spellLevel: Number(match[2]), count: row.value }] : [];
  });
}

/** The wizard's spellbook bounds: level 0 is every cantrip; `<k>_and_higher` caps levels k and up together. */
function spellbookBounds(known: readonly ExplanationDto[]): { cantrips: number | null; atOrAbove: Map<number, number> } {
  let cantrips: number | null = null;
  const atOrAbove = new Map<number, number>();
  for (const row of known) {
    const match = SPELLBOOK_ID.exec(row.id);
    if (match === null) continue;
    if (match[2] === undefined) {
      cantrips = row.value;
    } else {
      atOrAbove.set(Number(match[1]), row.value);
    }
  }
  return { cantrips, atOrAbove };
}

/** The `class:<token>` ids the dialog needs spell lists for: every class with a per-day or known row. */
export function spellClassTokens(perDay: readonly ExplanationDto[], known: readonly ExplanationDto[]): string[] {
  const tokens = new Set(buildSpellsPerDaySurface(perDay).rows.map((row) => row.classToken));
  for (const quota of knownQuotas(known)) tokens.add(quota.classToken);
  if (spellbookBounds(known).cantrips !== null || spellbookBounds(known).atOrAbove.size > 0) tokens.add('wizard');
  return [...tokens].sort().map((token) => `class:${token}`);
}

function perDayText(rows: SpellsPerDayRow[]): string {
  return `Spells per day: ${rows.map((row) => `level ${row.spellLevel}: ${row.count}`).join(', ')}.`;
}

export function creationSpellDialog(input: SpellDialogInput): SpellDialog {
  const perDayRows = buildSpellsPerDaySurface(input.perDay).rows;
  const groupLimits: Record<string, number> = {};
  const groupLabels: Record<string, string> = {};
  const sharedLimits: SharedLimit[] = [];
  const readOnly: ReadOnlyCaster[] = [];
  const items: TransferItem[] = [];
  const byKey = new Map(input.catalog.map((entry) => [entry.key, entry]));

  const quotas = knownQuotas(input.known);
  const book = spellbookBounds(input.known);
  if (book.cantrips !== null) quotas.push({ classToken: 'wizard', spellLevel: 0, count: book.cantrips });
  for (const [spellLevel, count] of book.atOrAbove) quotas.push({ classToken: 'wizard', spellLevel, count });
  const levels = [...book.atOrAbove.keys()].sort((a, b) => a - b);
  for (const from of levels) {
    const groups = levels.filter((level) => level >= from).map((level) => `wizard:${level}`);
    if (groups.length > 1) {
      sharedLimits.push({ key: `wizard:${from}+`, label: `Wizard spells of level ${from} or higher`, limit: book.atOrAbove.get(from)!, groups });
    }
  }

  quotas.sort((left, right) => left.classToken.localeCompare(right.classToken) || left.spellLevel - right.spellLevel);
  for (const quota of quotas) {
    if (quota.count <= 0) continue;
    const classId = `class:${quota.classToken}`;
    const group = `${quota.classToken}:${quota.spellLevel}`;
    groupLimits[group] = quota.count;
    groupLabels[group] =
      quota.classToken === 'wizard' && quota.spellLevel === 0
        ? 'Wizard level 0 (all cantrips)'
        : `${titleCase(quota.classToken)} level ${quota.spellLevel}`;
    const list = input.classLevels.find((entry) => entry.classId === classId);
    for (const listed of list?.entries ?? []) {
      if (listed.level !== quota.spellLevel) continue;
      items.push({
        id: `${classId}${ID_SEPARATOR}${listed.key}`,
        label: listed.key,
        description: byKey.get(listed.key)?.description ?? 'No description available.',
        qualified: true,
        group,
      });
    }
  }

  const pickers = new Set(quotas.map((quota) => quota.classToken));
  const perDayTokens = [...new Set(perDayRows.map((row) => row.classToken))].filter((token) => !pickers.has(token));
  for (const classToken of perDayTokens) {
    const name = titleCase(classToken);
    const rows = perDayRows.filter((row) => row.classToken === classToken);
    const why = WHOLE_LIST_CASTERS.has(classToken)
      ? `${name} knows every spell on its list.`
      : `${name}: the engine does not state how many spells a ${name.toLowerCase()} knows yet, so its spells are chosen on the sheet.`;
    readOnly.push({ classToken, text: `${why} ${perDayText(rows)}` });
  }

  for (const name of input.innate) {
    const entry = input.catalog.find((candidate) => candidate.key === name);
    items.push({
      id: `innate${ID_SEPARATOR}${name}`,
      label: name,
      description: entry?.description ?? 'A racial spell-like ability. The catalog has no description for it.',
      qualified: true,
      innate: true,
    });
  }

  return { items, groupLimits, groupLabels, sharedLimits, readOnly };
}

/**
 * What the spells dialog's quotas depend on: each class and its level count, race, ability scores
 * and alternate racial traits. A change to any of them reloads the quotas (a 2nd Wizard level
 * included, which a key of class ids alone missed).
 */
export function spellOptionsKey(
  heldClasses: readonly { classId: string; level: number }[],
  raceId: string,
  abilityScores: readonly number[],
  alternateTraitKeys: readonly string[]
): string {
  return JSON.stringify([heldClasses.map((held) => `${held.classId}:${held.level}`), raceId, abilityScores, alternateTraitKeys]);
}

/** The create requests for the chosen item ids; every pick is added as Known. Innate ids are never requests. */
export function spellSelectionsFromIds(ids: readonly string[]): CreateSpellDto[] {
  return ids.flatMap((id) => {
    const [classId, spellId] = id.split(ID_SEPARATOR);
    if (classId === undefined || spellId === undefined || classId === 'innate') {
      return [];
    }
    return [{ spellId, sourceClassId: classId, acquisitionMode: 'Known' as const }];
  });
}

/** Keeps only the picks the dialog still offers (not innate, class and level still have slots). */
export function keepOfferedSpells(items: readonly TransferItem[], selected: readonly string[]): string[] {
  const offered = new Set(items.filter((item) => item.innate !== true).map((item) => item.id));
  return selected.filter((id) => offered.has(id));
}

function chosenIn(dialog: SpellDialog, selected: readonly string[], groups: readonly string[]): number {
  return selected.filter((id) => {
    const group = dialog.items.find((item) => item.id === id)?.group;
    return group !== undefined && groups.includes(group);
  }).length;
}

/**
 * How many picks are left and the whole quota. A group inside a shared pool counts once, through the
 * widest pool over it, so a wizard's nested spellbook bounds are not added together.
 */
export function spellPicksLeft(dialog: SpellDialog, selected: readonly string[]): { left: number; total: number } {
  const pooled = new Set(dialog.sharedLimits.flatMap((shared) => shared.groups));
  const widest = dialog.sharedLimits.filter(
    (shared) => !dialog.sharedLimits.some((other) => other !== shared && shared.groups.every((group) => other.groups.includes(group)))
  );
  const caps = [
    ...Object.entries(dialog.groupLimits)
      .filter(([group]) => !pooled.has(group))
      .map(([group, limit]) => ({ groups: [group], limit })),
    ...widest,
  ];
  const total = caps.reduce((sum, cap) => sum + cap.limit, 0);
  const left = caps.reduce((sum, cap) => sum + Math.max(0, cap.limit - chosenIn(dialog, selected, cap.groups)), 0);
  return { left, total };
}

/** One sentence per quota group or shared pool with more picks than slots (empty when every one fits). */
export function spellQuotaOverruns(dialog: SpellDialog, selected: readonly string[]): string[] {
  const groups = Object.entries(dialog.groupLimits).flatMap(([group, limit]) => {
    const chosen = chosenIn(dialog, selected, [group]);
    return chosen > limit ? [`${dialog.groupLabels[group] ?? group}: ${chosen} chosen, ${limit} allowed`] : [];
  });
  const pools = dialog.sharedLimits.flatMap((shared) => {
    const chosen = chosenIn(dialog, selected, shared.groups);
    return chosen > shared.limit ? [`${shared.label}: ${chosen} chosen, ${shared.limit} allowed`] : [];
  });
  return [...groups, ...pools];
}

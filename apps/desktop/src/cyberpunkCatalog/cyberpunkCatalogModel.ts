import type { CyberpunkRecord } from '../boundary/loadCyberpunkCatalog';

/**
 * SD-38 E6.2 -- the Cyberpunk RED catalogs' tabs, labels and filtering. Holds no record and no
 * number: every row is the corpus's, served by `list_cyberpunk_catalog` (`cpr_catalog.rs`). The
 * tab list is every non-`print_only` kind of `data/cyberpunk-red/corpus/MANIFEST.json`
 * (`cyberpunkCatalog.test.ts` proves it equal); the `print_only` kinds are the GM reference
 * pages (E6.4). Later E6 cards add their entries here.
 */

export interface CyberpunkCatalogTab {
  /** The MANIFEST kind the tab lists. */
  kind: string;
  /** The tab's label and the screen heading. */
  title: string;
  /** The catalog group the tab sits in. */
  group: string;
  /** Set on a tool tab: the screen shows that tool instead of listing the kind's records. */
  tool?: 'deck';
}

export interface CyberpunkCatalogGroup {
  id: string;
  /** The landing screen's link text. */
  link: string;
  title: string;
}

export const CYBERPUNK_CATALOG_GROUPS: readonly CyberpunkCatalogGroup[] = [
  { id: 'character', link: 'Browse Roles and Lifepath', title: 'Roles and Lifepath' },
  { id: 'skills', link: 'Browse Skills', title: 'Skills and Checks' },
  { id: 'gear', link: 'Browse Gear and Weapons', title: 'Gear and Weapons' },
  { id: 'cyberware', link: 'Browse Cyberware', title: 'Cyberware and Humanity' },
  { id: 'combat', link: 'Browse Combat Rules', title: 'Combat Rules' },
  { id: 'netrunning', link: 'Browse Netrunning', title: 'Netrunning' },
];

const tab = (group: string, kind: string, title: string): CyberpunkCatalogTab => ({ kind, title, group });

export const CYBERPUNK_CATALOG_TABS: readonly CyberpunkCatalogTab[] = [
  tab('character', 'role', 'Roles'),
  tab('character', 'role_ability', 'Role Abilities'),
  tab('character', 'role_skill_list', 'Role Skill Lists'),
  tab('character', 'stat', 'Stats'),
  tab('character', 'streetrat_template', 'Streetrat Templates'),
  tab('character', 'complete_package_points', 'Complete Package Points'),
  tab('character', 'lifepath_table', 'Lifepath Tables'),
  tab('character', 'starting_kit', 'Starting Kits'),
  tab('character', 'ip_cost', 'Improvement Point Costs'),
  tab('character', 'reputation_level', 'Reputation Levels'),
  tab('skills', 'skill', 'Skills'),
  tab('skills', 'difficulty_value', 'Difficulty Values'),
  tab('skills', 'skill_modifier', 'Skill Modifiers'),
  tab('gear', 'price_category', 'Price Categories'),
  tab('gear', 'weapon', 'Weapons'),
  tab('gear', 'ammo', 'Ammo'),
  tab('gear', 'attachment', 'Attachments'),
  tab('gear', 'clip_chart', 'Clip Chart'),
  tab('gear', 'weapon_quality', 'Weapon Qualities'),
  tab('gear', 'armor', 'Armor'),
  tab('gear', 'gear', 'Gear'),
  tab('gear', 'fashion', 'Fashion'),
  tab('gear', 'drug', 'Drugs'),
  tab('gear', 'service', 'Services'),
  tab('gear', 'lifestyle', 'Lifestyles'),
  tab('gear', 'housing', 'Housing'),
  tab('gear', 'vehicle', 'Vehicles'),
  tab('cyberware', 'cyberware', 'Cyberware'),
  tab('cyberware', 'therapy', 'Therapies'),
  tab('cyberware', 'cyberpsychosis_band', 'Cyberpsychosis Bands'),
  tab('combat', 'action', 'Actions'),
  tab('combat', 'range_dv', 'Range DVs'),
  tab('combat', 'damage_band', 'Damage Bands'),
  tab('combat', 'martial_arts', 'Martial Arts'),
  tab('combat', 'cover', 'Cover'),
  tab('combat', 'critical_injury', 'Critical Injuries'),
  tab('combat', 'wound_state', 'Wound States'),
  tab('combat', 'hazard', 'Hazards'),
  tab('combat', 'vehicle_maneuver', 'Vehicle Maneuvers'),
  tab('combat', 'trauma_team', 'Trauma Team'),
  tab('netrunning', 'interface_ability', 'Interface Abilities'),
  tab('netrunning', 'net_action', 'NET Actions'),
  tab('netrunning', 'program', 'Programs'),
  tab('netrunning', 'cyberdeck', 'Cyberdecks'),
  tab('netrunning', 'net_hardware', 'Hardware'),
  { kind: 'deck_calculator', title: 'Deck Calculator', group: 'netrunning', tool: 'deck' },
];

export function cyberpunkCatalogTab(kind: string): CyberpunkCatalogTab {
  return CYBERPUNK_CATALOG_TABS.find((entry) => entry.kind === kind) ?? CYBERPUNK_CATALOG_TABS[0];
}

/** The first tab of a group: where a landing link opens. */
export function firstKindOfGroup(group: string): string {
  return (CYBERPUNK_CATALOG_TABS.find((entry) => entry.group === group) ?? CYBERPUNK_CATALOG_TABS[0]).kind;
}

/** A corpus field name as words: `cost_eb` -> `Cost eb`. */
export function fieldLabel(field: string): string {
  const words = field.split('_').filter((word) => word.length > 0).join(' ');
  return words.length === 0 ? field : words[0].toUpperCase() + words.slice(1);
}

/** The record's display name: its `name`, else its group and role for a table, else its kind. */
export function recordName(record: CyberpunkRecord): string {
  if (typeof record.name === 'string' && record.name.length > 0) {
    return record.name;
  }
  return fieldLabel(record.kind);
}

/** A corpus value as the words it prints: scalars as written, lists and objects field by field. */
export function describeValue(value: unknown): string {
  if (value === null || value === undefined) {
    return '';
  }
  if (typeof value === 'boolean') {
    return value ? 'yes' : 'no';
  }
  if (typeof value === 'string' || typeof value === 'number') {
    return String(value);
  }
  if (Array.isArray(value)) {
    return value.map((item) => describeValue(item)).filter((part) => part.length > 0).join('; ');
  }
  if (typeof value === 'object') {
    return Object.entries(value as Record<string, unknown>)
      .map(([key, item]) => [key, describeValue(item)] as const)
      .filter(([, text]) => text.length > 0)
      .map(([key, text]) => `${fieldLabel(key)}: ${text}`)
      .join(', ');
  }
  return '';
}

/** Fields shown in the card header or body paragraph, not in the field list. */
const HEADER_FIELDS = new Set(['name', 'kind', 'text', 'source_page']);

/** The record's remaining fields as label / text pairs, in the corpus's order; empty values are left out. */
export function recordFields(record: CyberpunkRecord): { field: string; label: string; text: string }[] {
  return Object.entries(record)
    .filter(([field]) => !HEADER_FIELDS.has(field))
    .map(([field, value]) => ({ field, label: fieldLabel(field), text: describeValue(value) }))
    .filter((entry) => entry.text.length > 0);
}

/** The records whose printed fields contain every word of `query` (case-insensitive). */
export function filterCyberpunkRecords(records: readonly CyberpunkRecord[], query: string): CyberpunkRecord[] {
  const words = query.toLowerCase().split(/\s+/).filter((word) => word.length > 0);
  if (words.length === 0) {
    return [...records];
  }
  return records.filter((record) => {
    const haystack = [recordName(record), String(record.text ?? ''), ...recordFields(record).map((entry) => entry.text)]
      .join('\n')
      .toLowerCase();
    return words.every((word) => haystack.includes(word));
  });
}

/** The screen's one-line summary: how many records the tab holds against the MANIFEST's count. */
export function describeCyberpunkCatalog(title: string, held: number, expected: number, shown: number): string {
  const all = `${held} of ${expected} ${title.toLowerCase()}`;
  return shown === held ? all : `${shown} of ${all}`;
}

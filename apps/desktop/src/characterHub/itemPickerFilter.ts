import type { EquipmentCatalogEntryDto } from '../boundary/loadEquipmentCatalog';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import type { FeatCatalogEntryDto } from '../boundary/listFeats';

/**
 * Pure logic backing `ItemPickerModal`: mapping the two real catalog DTOs
 * (equipment, spell — otherwise shaped nothing alike) onto one generic
 * display row, and filtering those rows by the search box's text. Kept
 * separate from the React component so it's unit-testable without a DOM
 * (this repo has no jsdom/testing-library — see `characterSheetRefresh.ts`
 * for the same split applied to the mutation-outcome mapping).
 */

export interface ItemPickerEntry {
  /** The catalog `key` — what gets sent back as `itemId` / `spellId` on select. */
  key: string;
  name: string;
  detail: string;
  /**
   * Feat entries only: `'Weapon'`, `'Skill'` or `'SpellSchool'` when this
   * feat needs a chosen target, absent otherwise.
   *
   * Carried on the picker entry so the Add Feat flow can tell, at the
   * moment of the pick, whether a second target step is required — without
   * re-querying the catalog or hardcoding a list of chooser feats in the
   * frontend, which would be rules knowledge duplicated out of the engine.
   */
  chooserTargetKind?: string | null;
}

/** Friendly labels for `EquipmentCategory` variants — mirrors `EquipmentCatalogScreen`'s own map. */
const EQUIPMENT_CATEGORY_LABELS: Record<string, string> = {
  ArmsArmor: 'Arms & Armor',
  General: 'General',
  MagicItems: 'Magic Items',
  Equipmods: 'Equipment Mods',
};

export function mapEquipmentCatalogEntries(entries: EquipmentCatalogEntryDto[]): ItemPickerEntry[] {
  return entries.map((entry) => ({
    key: entry.key,
    name: entry.name,
    // Unknown/future categories fall back to the raw variant string verbatim
    // rather than a fabricated label.
    detail: EQUIPMENT_CATEGORY_LABELS[entry.category] ?? entry.category,
  }));
}

export function mapSpellCatalogEntries(entries: SpellCatalogEntryDto[]): ItemPickerEntry[] {
  return entries.map((entry) => ({
    key: entry.key,
    // The spell catalog DTO has no separate display-name field (see
    // `SpellCatalogEntryDto`'s doc comment) — `key` is the spell's real
    // corpus identity and doubles as the display name.
    name: entry.key,
    // Book first, since the catalog spans CRB, APG, ACG and ARG and a
    // player picking a spell needs to know which book it comes from.
    // `school`/`level` are omitted rather than defaulted when the corpus
    // row genuinely lacks them (a real `apg_spells.lst` gap), so the
    // detail line never asserts a school or level the corpus never gave.
    //
    // The level is labelled "Lowest class level", not "Level", because
    // that is what the catalog record's own field is: the MINIMUM across
    // every class named in its corpus `CLASSES:` tag. Hideous Laughter is
    // `CLASSES:Bard=1|Sorcerer,Wizard=2`, so it reads 1 here even for a
    // Wizard who learns it at 2. This picker browses all 1185 records
    // across every class, so it has no one class to answer for — unlike
    // the Spells tab, which resolves each row against its own
    // `sourceClassId` via `list_class_spell_levels` (see
    // `spellsTabModel.ts`). Same wording as `SpellCatalogScreen.tsx`,
    // the other cross-class browse.
    detail: [
      entry.book,
      entry.school,
      entry.level === null ? null : `Lowest class level ${entry.level}`,
    ]
      .filter((part): part is string => part !== null)
      .join(' · '),
  }));
}

/**
 * Friendly book labels for `RuleSetId` variants — mirrors the spell
 * catalog's own `book` strings. Every variant `list_feat_catalog` can
 * actually emit needs an entry: a missing one reaches the player as the raw
 * `RuleSetId` variant name (`Arg`, `Pu`) sitting beside properly-coded
 * CRB/APG/ACG rows. `feat_catalog.rs` serves Crb 185, Apg 172, Acg 129, Arg
 * 187 and Pu 17 (690 total), so ARG and PU alone are 204 of the picker's
 * rows. `Bestiary1` is deliberately absent: that book contributes equipment
 * but no feats.
 */
const FEAT_SOURCE_LABELS: Record<string, string> = {
  Crb: 'CRB',
  Apg: 'APG',
  Acg: 'ACG',
  Arg: 'ARG',
  Pu: 'PU',
};

export function mapFeatCatalogEntries(entries: FeatCatalogEntryDto[]): ItemPickerEntry[] {
  return entries.map((entry) => ({
    key: entry.key,
    name: entry.name,
    // Book first, then category, then the corpus description — the
    // catalog spans CRB, APG, ACG, ARG and PU, and a player picking a feat
    // needs to know which book it comes from, exactly as
    // `mapSpellCatalogEntries` already does. An unknown/future book falls
    // back to the raw variant string rather than a fabricated label, and
    // the description is omitted rather than invented when the corpus
    // record has no `DESC:` token (a real gap: CRB's "Heighten Spell +N"
    // records and APG's base "Elemental Fist" — see
    // `FeatTableEntry.description`'s own doc comment).
    detail: [FEAT_SOURCE_LABELS[entry.source] ?? entry.source, entry.category, entry.description]
      .filter((part): part is string => Boolean(part))
      .join(' · '),
    chooserTargetKind: entry.chooserTargetKind,
  }));
}

/** Case-insensitive substring match against either the name or the detail line. */
export function filterItemPickerEntries(entries: ItemPickerEntry[], searchTerm: string): ItemPickerEntry[] {
  const term = searchTerm.trim().toLowerCase();
  if (!term) {
    return entries;
  }
  return entries.filter(
    (entry) => entry.name.toLowerCase().includes(term) || entry.detail.toLowerCase().includes(term)
  );
}

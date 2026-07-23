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
    detail: `${entry.school} · Level ${entry.level}`,
  }));
}

export function mapFeatCatalogEntries(entries: FeatCatalogEntryDto[]): ItemPickerEntry[] {
  return entries.map((entry) => ({
    key: entry.key,
    name: entry.name,
    // Falls back to the bare category when the corpus record has no
    // `DESC:` token (real corpus gap, e.g. the "Heighten Spell +N"
    // bonus-tier records — see `FeatTableEntry.description`'s own doc
    // comment) rather than fabricating description text.
    detail: entry.description ? `${entry.category} · ${entry.description}` : entry.category,
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

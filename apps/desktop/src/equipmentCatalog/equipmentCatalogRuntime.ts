import { loadEquipmentCatalog, type EquipmentCatalogEntryDto } from '../boundary/loadEquipmentCatalog';
import { hasTauriRuntime } from '../boundary/runtime';

/** Sample data for the browser preview (no Tauri backend) — keeps the
 * catalog screen walkable without the desktop runtime, matching the
 * `characterHub/previewData.ts` convention.
 *
 * Every sample row is a real CRB record and is tagged `CRB` accordingly.
 * No ARG/PU/B1 sample is invented here: the screen derives its book
 * summary and filter buttons from the records it actually loaded, so the
 * preview honestly reports one book rather than advertising six it does
 * not have. The full six-book catalog arrives from `list_equipment_catalog`
 * under the desktop runtime.
 *
 * Each row's `description` is that record's **real** corpus description prose,
 * transcribed from `crb::equipment_data::{arms_armor,magic_items}`, not
 * sample text written for the preview. Two rows are `null` because those
 * two corpus records genuinely carry no description — `Backpack`
 * (`general.rs`) and `Material ~ Cloth` (`equipmods.rs`) — which keeps the
 * preview representative of the real catalog, where 974 of 3830 records
 * have none. Each row's `weightLbs` is likewise that record's own
 * `weight_lbs` from the same tables (v0.8 F-15) — `null` where the table
 * records none. */
function buildPreviewCatalog(): EquipmentCatalogEntryDto[] {
  return [
    {
      types: ['Weapon', 'Resizable', 'Melee', 'Martial', 'OneHanded', 'Slashing', 'Sword', 'BladeHeavy', 'Weapon Group Blades Heavy'],
      key: 'Longsword (Base)',
      category: 'ArmsArmor',
      name: 'Longsword',
      costGp: 15,
      weightLbs: 4,
      book: 'CRB',
      description: 'This sword is about 3-1/2 feet in length.',
    },
    {
      types: ['Armor', 'Light', 'ArmorProfLight', 'Suit'],
      key: 'Chain Shirt (Base)',
      category: 'ArmsArmor',
      name: 'Chain Shirt',
      costGp: 100,
      weightLbs: 25,
      book: 'CRB',
      description:
        'Covering the torso, this shirt is made up of thousands of interlocking metal rings.',
    },
    { types: ['Goods', 'Container', 'General', 'Resizable'], key: 'Backpack', category: 'General', name: 'Backpack', costGp: 2, weightLbs: 2, book: 'CRB', description: null },
    {
      types: ['Magic', 'Potion', 'Consumable', 'Combat Gear'],
      key: 'Potion of Aid',
      category: 'MagicItems',
      name: 'Potion of Aid',
      costGp: null,
      weightLbs: 0,
      book: 'CRB',
      description:
        '+1 morale bonus on attack rolls and saves vs. fear, plus 1d8+1 temporary hp for 1 minute',
    },
    { types: ['BaseMaterial', 'Mundane', 'Ammunition', 'Armor', 'Shield', 'Weapon', 'Instruments', 'Tools', 'Goods'], key: 'Material ~ Cloth', category: 'Equipmods', name: 'Cloth', costGp: 0, weightLbs: null, book: 'CRB', description: null },
  ];
}

/** The catalog plus why its type data (the category source) could not be read, when it could not. */
export async function loadEquipmentCatalogWithNotice(): Promise<{ entries: EquipmentCatalogEntryDto[]; typesError: string | null }> {
  if (!hasTauriRuntime()) {
    return { entries: buildPreviewCatalog(), typesError: null };
  }
  const response = await loadEquipmentCatalog();
  return { entries: response.entries, typesError: response.typesError ?? null };
}

export async function loadEquipmentCatalogRuntime(): Promise<EquipmentCatalogEntryDto[]> {
  return (await loadEquipmentCatalogWithNotice()).entries;
}

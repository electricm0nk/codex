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
 * Each row's `description` is that record's **real** corpus `DESC:` prose,
 * transcribed from `crb::equipment_data::{arms_armor,magic_items}`, not
 * sample text written for the preview. Two rows are `null` because those
 * two corpus records genuinely carry no description — `Backpack`
 * (`general.rs`) and `Material ~ Cloth` (`equipmods.rs`) — which keeps the
 * preview representative of the real catalog, where 974 of 3830 records
 * have none. */
function buildPreviewCatalog(): EquipmentCatalogEntryDto[] {
  return [
    {
      key: 'Longsword (Base)',
      category: 'ArmsArmor',
      name: 'Longsword',
      costGp: 15,
      book: 'CRB',
      description: 'This sword is about 3-1/2 feet in length.',
    },
    {
      key: 'Chain Shirt (Base)',
      category: 'ArmsArmor',
      name: 'Chain Shirt',
      costGp: 100,
      book: 'CRB',
      description:
        'Covering the torso, this shirt is made up of thousands of interlocking metal rings.',
    },
    { key: 'Backpack', category: 'General', name: 'Backpack', costGp: 2, book: 'CRB', description: null },
    {
      key: 'Potion of Aid',
      category: 'MagicItems',
      name: 'Potion of Aid',
      costGp: null,
      book: 'CRB',
      description:
        '+1 morale bonus on attack rolls and saves vs. fear, plus 1d8+1 temporary hp for 1 minute',
    },
    { key: 'Material ~ Cloth', category: 'Equipmods', name: 'Cloth', costGp: 0, book: 'CRB', description: null },
  ];
}

export async function loadEquipmentCatalogRuntime(): Promise<EquipmentCatalogEntryDto[]> {
  if (!hasTauriRuntime()) {
    return buildPreviewCatalog();
  }
  const response = await loadEquipmentCatalog();
  return response.entries;
}

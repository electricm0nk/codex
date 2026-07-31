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
 * under the desktop runtime. */
function buildPreviewCatalog(): EquipmentCatalogEntryDto[] {
  return [
    { key: 'Longsword (Base)', category: 'ArmsArmor', name: 'Longsword', costGp: 15, book: 'CRB' },
    { key: 'Chain Shirt (Base)', category: 'ArmsArmor', name: 'Chain Shirt', costGp: 100, book: 'CRB' },
    { key: 'Backpack', category: 'General', name: 'Backpack', costGp: 2, book: 'CRB' },
    { key: 'Potion of Aid', category: 'MagicItems', name: 'Potion of Aid', costGp: null, book: 'CRB' },
    { key: 'Material ~ Cloth', category: 'Equipmods', name: 'Cloth', costGp: 0, book: 'CRB' },
  ];
}

export async function loadEquipmentCatalogRuntime(): Promise<EquipmentCatalogEntryDto[]> {
  if (!hasTauriRuntime()) {
    return buildPreviewCatalog();
  }
  const response = await loadEquipmentCatalog();
  return response.entries;
}

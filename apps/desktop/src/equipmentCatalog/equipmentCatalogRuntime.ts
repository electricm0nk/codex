import { loadEquipmentCatalog, type EquipmentCatalogEntryDto } from '../boundary/loadEquipmentCatalog';
import { hasTauriRuntime } from '../boundary/runtime';

/** Sample data for the browser preview (no Tauri backend) — keeps the
 * catalog screen walkable without the desktop runtime, matching the
 * `characterHub/previewData.ts` convention. */
function buildPreviewCatalog(): EquipmentCatalogEntryDto[] {
  return [
    { key: 'Longsword (Base)', category: 'ArmsArmor', name: 'Longsword', costGp: 15 },
    { key: 'Chain Shirt (Base)', category: 'ArmsArmor', name: 'Chain Shirt', costGp: 100 },
    { key: 'Backpack', category: 'General', name: 'Backpack', costGp: 2 },
    { key: 'Potion of Aid', category: 'MagicItems', name: 'Potion of Aid', costGp: null },
    { key: 'Material ~ Cloth', category: 'Equipmods', name: 'Cloth', costGp: 0 },
  ];
}

export async function loadEquipmentCatalogRuntime(): Promise<EquipmentCatalogEntryDto[]> {
  if (!hasTauriRuntime()) {
    return buildPreviewCatalog();
  }
  const response = await loadEquipmentCatalog();
  return response.entries;
}

import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CharacterSummaryDto } from './loadListSavedCharacters';
import type { PilotSnapshotDto, CorpusDerivedDto, DiagnosticDto } from './loadCreateCharacter';
import type { ActiveStateDto } from './addEquipmentSelection';
import type { CharacterMoneyDto } from './characterMoney';

/**
 * Write desktop boundary for atomically purchasing an equipment item: the
 * `purchase_equipment` Tauri command resolves the item's real catalog
 * `cost_gp`, pre-checks the current money balance, and only if affordable
 * appends the equipment selection AND deducts the cost in one round trip —
 * see `purchase_equipment_at_root`'s own doc comment in `character_hub.rs`
 * for the full transaction-shape reasoning. Replaces `addEquipmentSelection`
 * for anything with a real gold cost (weapons/armor/general gear bought
 * through the Add Weapon/Add Armor pickers); `addEquipmentSelection` itself
 * stays correct for free/starting/DM-granted items, so it is not removed.
 *
 * A distinct outcome union from `CreateCharacterOutcome` (not reused) since
 * the `Purchased` variant carries the refreshed `money` balance alongside
 * the usual summary/snapshot/corpusDerived — one response covers both the
 * sheet refresh and the Money panel refresh, so callers don't need a
 * separate `loadCharacterMoney` round trip after a purchase.
 */

export interface PurchaseEquipmentRequest {
  characterId: string;
  itemId: string;
  activeState: ActiveStateDto;
  savedAt: string;
}

export type PurchaseEquipmentOutcome =
  | {
      kind: 'Purchased';
      summary: CharacterSummaryDto;
      snapshot: PilotSnapshotDto;
      corpusDerived: CorpusDerivedDto;
      money: CharacterMoneyDto;
    }
  | { kind: 'Blocked'; diagnostics: DiagnosticDto[] };

export async function purchaseEquipment(request: PurchaseEquipmentRequest): Promise<PurchaseEquipmentOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for purchasing equipment');
  }

  try {
    return await invoke<PurchaseEquipmentOutcome>('purchase_equipment', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to purchase equipment: ${formatError(cause)}`);
  }
}

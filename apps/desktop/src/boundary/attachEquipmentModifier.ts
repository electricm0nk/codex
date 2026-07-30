import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CharacterSummaryDto } from './loadListSavedCharacters';
import type { PilotSnapshotDto, CorpusDerivedDto, DiagnosticDto } from './loadCreateCharacter';
import type { CharacterMoneyDto } from './characterMoney';

/**
 * Write desktop boundary for atomically attaching an Equipmods-category
 * modifier (e.g. a "+1 Enhancement to Weapon" record) to a specific
 * already-equipped selection: the `attach_equipment_modifier` Tauri command
 * validates `modifierItemId` against the real equipment catalog and
 * `itemId` against the character's actual equipment selections, then
 * mirrors `purchase_equipment`'s resolve-cost -> check-affordability ->
 * mutate -> charge sequencing — with one deliberate difference: an unknown
 * `cost_gp` attaches for free rather than blocking, since the real CRB
 * weapon/armor enhancement records (+1 through +10) have no flat catalog
 * price at all (real PF1 enhancement pricing is a bonus-squared formula) —
 * see `attach_equipment_modifier_at_root`'s own doc comment in
 * `character_hub.rs` for the full reasoning. Only a modifier with a real,
 * known `cost_gp` (e.g. Masterwork) is ever actually charged.
 *
 * A distinct outcome union from `PurchaseEquipmentOutcome` (not reused,
 * despite the identical field set) so the two commands' `kind` tags stay
 * distinct on the wire (`"Attached"` vs `"Purchased"`) — same per-command
 * outcome convention every other mutation here already follows.
 */

export interface AttachEquipmentModifierRequest {
  characterId: string;
  /** The target equipment selection to attach to, e.g. an equipped Longsword's item_id. */
  itemId: string;
  /** The Equipmods-category item being attached. */
  modifierItemId: string;
  savedAt: string;
}

export type AttachEquipmentModifierOutcome =
  | {
      kind: 'Attached';
      summary: CharacterSummaryDto;
      snapshot: PilotSnapshotDto;
      corpusDerived: CorpusDerivedDto;
      money: CharacterMoneyDto;
    }
  | { kind: 'Blocked'; diagnostics: DiagnosticDto[] };

export async function attachEquipmentModifier(
  request: AttachEquipmentModifierRequest
): Promise<AttachEquipmentModifierOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for attaching an equipment modifier');
  }

  try {
    return await invoke<AttachEquipmentModifierOutcome>('attach_equipment_modifier', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to attach equipment modifier: ${formatError(cause)}`);
  }
}

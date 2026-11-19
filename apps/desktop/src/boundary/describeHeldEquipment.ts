import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { ResolvedEquipmentEffectDto } from './loadCreateCharacter';
import type { EquipmentCatalogEntryDto } from './loadEquipmentCatalog';
import type { CopyKeyDto, PlacedEquipmentCopyDto } from './listHeldEquipment';
import type { ContainerCapacityDto, WeaponDamageDto } from './loadSavedCharacterDetail';

/**
 * Read desktop boundary for the gear detail panel (SD-42 `decisions.md §27.4`). Invokes the
 * `describe_held_equipment` Tauri command: the record as the catalog serves it, the weapon row and
 * armor stats the sheet computes for the copy, its container facts, and the copy's own facts.
 * `null` is a value the record does not state, never 0.
 */

/** A container's own facts, off its record. */
export interface ContainerStatsDto {
  capacity: ContainerCapacityDto;
  /** What the contents weigh toward the carried total, in percent: 0 for a Bag of Holding. */
  contentsWeightPct: number;
}

/** The copy as the held list sends it, with the Buy Custom facts that list's type leaves out. */
export type DescribedCopyDto = PlacedEquipmentCopyDto & {
  /** Special abilities, material and size, each as `<name>: <the record's words>`; absent when none. */
  abilities?: string[];
  /** The wielder size the copy was made for; absent for Medium. */
  size?: string;
};

export interface HeldEquipmentDetailDto {
  entry: EquipmentCatalogEntryDto;
  /** The sheet's Weapons row for this copy; null for a non-weapon and for a copy left behind. */
  weapon: WeaponDamageDto | null;
  armor: ResolvedEquipmentEffectDto | null;
  container: ContainerStatsDto | null;
  copy: DescribedCopyDto;
}

export interface DescribeHeldEquipmentRequest {
  characterId: string;
  copy: CopyKeyDto;
}

export async function describeHeldEquipment(request: DescribeHeldEquipmentRequest): Promise<HeldEquipmentDetailDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for describing equipment');
  }

  try {
    return await invoke<HeldEquipmentDetailDto>('describe_held_equipment', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to describe equipment: ${formatError(cause)}`);
  }
}

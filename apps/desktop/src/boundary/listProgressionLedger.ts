import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the Show Progression drawer's rows: the character level by level.
 * Invokes `list_progression_ledger`, which reads `ledger.json` and the character as it stood at each level.
 */

export interface ProgressionFeatDto {
  id: string;
  name: string;
  /** A chooser feat's target. */
  target: string | null;
}

export interface ProgressionSpellDto {
  /** The catalog's key, which the sheet's Spells tab prints. */
  id: string;
  classId: string | null;
}

export interface ProgressionRanksDto {
  skillId: string;
  ranks: number;
}

export interface ProgressionAbilityIncreaseDto {
  /** `wisdom`, from a grant's target `ability:wisdom`. */
  ability: string;
  points: number;
}

export interface ProgressionChoicesDto {
  feats: ProgressionFeatDto[];
  spells: ProgressionSpellDto[];
  ranks: ProgressionRanksDto[];
  abilityIncreases: ProgressionAbilityIncreaseDto[];
}

/** A class feature line of the sheet, new at this level or, when `raised`, with its value moved here. */
export interface ProgressionClassFeatureDto {
  id: string;
  label: string;
  /** What the sheet prints for it: `+2`, or nothing. */
  value: string;
  raised: boolean;
}

export interface ProgressionLevelDto extends ProgressionChoicesDto {
  characterLevel: number;
  classId: string;
  classLevel: number;
  /** The hit die result saved for this class level; `null` when none was. */
  hitDieResult: number | null;
  classFeatures: ProgressionClassFeatureDto[];
  /** This character level grants an ability increase. */
  abilityIncreaseDue: boolean;
  /** Slots this level opened that nothing fills. */
  unspent: { feats: number; spells: number; ranks: number };
  /** Any choice at this level had its level worked out afterwards. */
  assignedOnUpgrade: boolean;
}

export interface ProgressionLedgerDto {
  levels: ProgressionLevelDto[];
  /** An over-budget choice of an old save, or one stamped above the character's level. */
  withoutLevel: ProgressionChoicesDto;
}

export async function listProgressionLedger(characterId: string): Promise<ProgressionLedgerDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for listing the level history');
  }

  try {
    return await invoke<ProgressionLedgerDto>('list_progression_ledger', { request: { characterId } });
  } catch (cause: unknown) {
    throw new Error(`Failed to list the level history: ${formatError(cause)}`);
  }
}

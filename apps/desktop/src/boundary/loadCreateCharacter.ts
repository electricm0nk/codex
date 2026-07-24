import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CharacterSummaryDto } from './loadListSavedCharacters';

/**
 * Read/write desktop boundary over character creation.
 *
 * Invokes the `create_character` Tauri command, which composes a
 * `CharacterInput` from the caller's race/class/level choice server-side
 * and computes it via the real rules-core engine. Returns a tagged
 * `Saved` / `Blocked` outcome verbatim — never fabricates a computed
 * result, and a `Blocked` outcome means nothing was persisted.
 */

export interface AbilityScoresDto {
  strength: number;
  dexterity: number;
  constitution: number;
  intelligence: number;
  wisdom: number;
  charisma: number;
}

export interface CreateCharacterRequest {
  characterId: string;
  displayLabel: string;
  raceId: string;
  classId: string;
  level: number;
  abilityScores: AbilityScoresDto;
  abilityBonusTarget: string;
  savedAt: string;
}

export interface BaseSavesDto {
  fortitude: number;
  reflex: number;
  will: number;
}

export interface SelectedSkillModifiersDto {
  climb: number;
  intimidate: number;
  swim: number;
}

export interface PilotSnapshotDto {
  abilityModifiers: AbilityScoresDto;
  baseAttackBonus: number;
  baseSaves: BaseSavesDto;
  baselineMeleeAttackBonus: number;
  baselineArmorClass: number;
  totalSaves: BaseSavesDto;
  selectedSkillModifiers: SelectedSkillModifiersDto;
  /**
   * The flat DR magnitude from a grounded class-feature DR explanation
   * (currently only Barbarian's) — see `PilotSnapshotDto`'s own doc comment
   * in `character_hub.rs`. Absent (not zero) when no class currently
   * reachable through this UI grounds one; `#[serde(skip_serializing_if)]`
   * on the Rust side means the key itself may not be present on the wire.
   */
  damageReduction?: number;
}

export interface DiagnosticDto {
  id: string;
  message: string;
  claimBlocking: boolean;
}

/** One PF1 strict spell school's corpus-derived reachability, e.g. "Abjuration". */
export interface SchoolCoverageDto {
  school: string;
  spells: string[];
  /** Whether the resolved spell(s) also ground through the foundation-slice table cell. */
  grounded: boolean;
}

export interface ResolvedEquipmentDto {
  itemId: string;
  equipmentRecordName: string;
  equipmentRecordKey: string;
  /** Whether this item also grounds through the foundation-slice table cell. */
  grounded: boolean;
}

/**
 * Corpus-derived spell/equipment reachability from `compute_pilot_with_corpus`,
 * resolved against a small bundled corpus-fixture set (see
 * `src-tauri/src/corpus_fixtures.rs`) — not the full PCGen corpus.
 */
export interface CorpusDerivedDto {
  schoolCoverage: SchoolCoverageDto[];
  equippedItems: ResolvedEquipmentDto[];
}

export type CreateCharacterOutcome =
  | {
      kind: 'Saved';
      summary: CharacterSummaryDto;
      snapshot: PilotSnapshotDto;
      corpusDerived: CorpusDerivedDto;
    }
  | { kind: 'Blocked'; diagnostics: DiagnosticDto[] };

export async function loadCreateCharacter(
  request: CreateCharacterRequest
): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for creating a character');
  }

  try {
    return await invoke<CreateCharacterOutcome>('create_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to create character: ${formatError(cause)}`);
  }
}

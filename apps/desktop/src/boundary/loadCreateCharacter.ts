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
 * Real, corpus-resolved aggregate equipment-effect totals for the
 * character's currently `EquippedActive` items (v0.6 alpha swarm item 1,
 * shape (c)). Explicitly NOT claim-gated — reflects whatever gear is
 * actually equipped regardless of whether the build reaches `Computed`.
 *
 * `armorClassDelta`/`armorCheckPenaltyTotal` are always real numbers,
 * including a real `0` when nothing equipped grants either (not an
 * "absent" case — sum of nothing is a real sum). `maxDexCap`/
 * `spellFailureChance`/`attackBonusDelta` are genuinely absent
 * (`undefined` on the wire) rather than zero when no equipped item sets
 * them — `attackBonusDelta` specifically is also absent whenever zero or
 * two-or-more weapons are equipped (which weapon a modifier attaches to is
 * ambiguous with more than one; see `character_hub.rs`'s own doc comment) —
 * a real `0` there means exactly one weapon equipped with no enhancement,
 * and must render as "+0", not be treated the same as absent.
 */
export interface EquipmentEffectsDto {
  armorClassDelta: number;
  armorCheckPenaltyTotal: number;
  maxDexCap?: number;
  spellFailureChance?: number;
  attackBonusDelta?: number;
}

/**
 * Corpus-derived spell/equipment reachability from `compute_pilot_with_corpus`,
 * resolved against a small bundled corpus-fixture set (see
 * `src-tauri/src/corpus_fixtures.rs`) — not the full PCGen corpus.
 */
export interface CorpusDerivedDto {
  schoolCoverage: SchoolCoverageDto[];
  equippedItems: ResolvedEquipmentDto[];
  equipmentEffects: EquipmentEffectsDto;
  /**
   * Every `spellId`/`itemId` the character actually has selected and
   * persisted that did NOT resolve against this build's tiny bundled demo
   * corpus (`corpus_fixtures.rs`, ~4 records total) — before this field
   * existed, such a selection simply vanished from `schoolCoverage`/
   * `equippedItems` with no signal at all, indistinguishable from "nothing
   * selected" even though the underlying data was never lost (v0.6 alpha
   * swarm, found in the frontend's own live smoke test). Render as an
   * honest "not shown — outside demo corpus" indicator, never silence.
   */
  unresolvedSpellIds: string[];
  unresolvedEquipmentItemIds: string[];
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

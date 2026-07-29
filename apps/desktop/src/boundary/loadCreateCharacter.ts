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
  /**
   * This selection's own resolved `applied_modifiers` (e.g. a resolved
   * "+1 Enhancement to Weapon" attached to this Longsword) — v0.6 alpha
   * swarm items 1+27 sub-task 6. Reuses this same DTO shape rather than a
   * new type, since a resolved modifier is structurally just another
   * resolved equipment record. Empty for a selection with no attached
   * modifiers, or whose modifiers all failed to resolve — those surface via
   * `CorpusDerivedDto.unresolvedEquipmentItemIds` instead, same list a
   * top-level unresolvable selection already uses.
   */
  appliedModifiers: ResolvedEquipmentDto[];
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
/**
 * One equipped item's own contribution to the defensive totals — the data
 * behind an "AC breakdown by source" view. Every optional field is a
 * genuine corpus absence (a longsword has no armor bonus), omitted on the
 * wire rather than zero-filled, so `undefined` means "this item does not
 * have one" and a real `0` means "it has one, and it is zero".
 */
export interface ResolvedEquipmentEffectDto {
  itemId: string;
  equipmentRecordKey: string;
  /** `'ArmsArmor' | 'General' | 'MagicItems' | 'Equipmods'`. */
  category: string;
  armorClassBonus?: number;
  maxDex?: number;
  spellFailure?: number;
  armorCheckPenalty?: number;
}

export interface EquipmentEffectsDto {
  /** Per-item contributions behind the aggregate totals below. */
  perItem: ResolvedEquipmentEffectDto[];
  armorClassDelta: number;
  armorCheckPenaltyTotal: number;
  maxDexCap?: number;
  spellFailureChance?: number;
  attackBonusDelta?: number;
}

/** One carried item's real corpus weight and price. */
export interface CarriedItemDto {
  itemId: string;
  weightLbs: number;
  /**
   * Absent when the corpus genuinely carries no price for the record (an
   * unpriced `(Base)` template, or a modifier priced by formula over its
   * base item) — never a fabricated `0`.
   */
  costGp?: number;
}

/**
 * Real carried weight against PF1's Strength-derived carrying-capacity
 * thresholds, plus the load tier's own penalties.
 *
 * Thresholds come from the real PCGen Pathfinder game mode's `load.lst`
 * (`LOAD:<Strength>|<heavy>`, with light = 1/3 and medium = 2/3 of that);
 * the load penalties come from PCGen's own engine (`PlayerCharacter.java`).
 */
export interface EncumbranceDto {
  totalCarriedWeightLbs: number;
  /** Floor on the loadout's gp value — unpriced items contribute nothing. */
  totalCarriedCostGp: number;
  lightMaxLbs: number;
  mediumMaxLbs: number;
  heavyMaxLbs: number;
  /** `'Light' | 'Medium' | 'Heavy' | 'OverHeavyCapacity'`. */
  level: string;
  /**
   * Max-Dex cap from the *load tier alone*, absent under a light load. An
   * effective cap is the lower of this and `EquipmentEffectsDto.maxDexCap`
   * — they do not sum.
   */
  loadMaxDexCap?: number;
  /**
   * Armor check penalty from the *load tier alone*; a real `0` under a
   * light load. Does not sum with worn armor's own penalty — PF1 takes the
   * more punishing of the two.
   */
  loadArmorCheckPenalty: number;
  perItem: CarriedItemDto[];
  /** Carried items whose weight could not be resolved against the corpus. */
  unresolvedItemIds: string[];
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
  encumbrance: EncumbranceDto;
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

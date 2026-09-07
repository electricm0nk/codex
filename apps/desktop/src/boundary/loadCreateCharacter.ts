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
  /**
   * ARG alternate racial traits chosen for this race, as corpus record keys
   * ("Dwarf ~ Saltbeard") — the same identifiers
   * `loadAlternateRacialTraits` serves and `resolveRaceAlternateSelection`
   * takes, so the picker round-trips its own keys unchanged.
   *
   * The backend re-validates every key against the real corpus before
   * persisting, and returns `Blocked` (never a quiet drop) for a key that
   * matches no alternate for the race or that violates ARG's own mutual-
   * exclusion guard.
   */
  selectedAlternateTraitKeys: string[];
  /**
   * Character trait/drawback selections (AT-34-E4-002), as the wire ids
   * `loadCharacterTraits` serves and `skill_bonuses_from_traits` reads --
   * `"trait:trait_acrobat"`, the same flat compound-string idiom
   * `selected_feats`' `"feat:weapon_focus"` uses. Passed through verbatim
   * into the character's `selected_traits`; an id the backend does not
   * recognize is simply inert (see `CreateCharacterRequest::
   * selected_traits`'s own Rust doc comment), never a blocked save.
   */
  selectedTraits: string[];
  /**
   * Character trait/drawback selections (AT-34-E4-002, second slice): the
   * player's resolved skill choice for each *fixed-choice* `%LIST` trait
   * named in `selectedTraits` — one entry per such trait, with
   * `choiceSetId` exactly `loadCharacterTraits`'s own `choiceSetId` for
   * that option and `selectionId` one of its `skillOptions`. Mirrors
   * `LevelUpCharacterRequest.additionalChoices`'s own `SelectedChoiceEntryDto`
   * shape — appended to `chosen.selected_choices` verbatim, never a
   * blocked save for an id this crate does not recognize (see
   * `CreateCharacterRequest::trait_skill_choices`'s own Rust doc comment).
   * A flat trait needs no entry here.
   */
  traitSkillChoices: TraitSkillChoiceDto[];
}

/** One player-resolved skill choice for a fixed-choice `%LIST` trait. Mirrors `SelectedChoiceDto` in `character_hub.rs`. */
export interface TraitSkillChoiceDto {
  choiceSetId: string;
  selectionId: string;
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

/**
 * One grounded companion statistic — mirrors `CompanionStatDto` in
 * `character_hub.rs`.
 */
export interface CompanionStatDto {
  /**
   * The engine's own honest name for what `value` is, e.g. `'Armor Class'`
   * or `'Attack Bonus'`. Deliberately not a title-casing of the underlying
   * record id: the record named `base_attack_bonus` carries the
   * companion's base attack bonus *plus* its Strength modifier, and
   * `bite_attack` carries only a flat damage bonus. See
   * `COMPANION_STAT_ROWS` in `pilot_view_model.rs`.
   */
  label: string;
  value: number;
  /** The engine's own corpus-cited derivation prose, verbatim. */
  detail: string;
}

/**
 * The character's animal companion or mount — mirrors
 * `AnimalCompanionDto` in `character_hub.rs`, projected from the
 * `class_chassis.<class>.<role>.*` explanation records the engine grounds
 * across all twenty master levels (Druid's and Hunter's Wolf animal
 * companion, the Cavalier's Horse mount).
 *
 * A wholly separate creature: none of these values are applied to the
 * character's own integrated totals, and the sheet must not mix them in.
 */
export interface AnimalCompanionDto {
  /** e.g. `'Druid'` — read from the record that grounded the companion. */
  ownerClassLabel: string;
  /** What the owning class calls it: `'Animal Companion'` or `'Mount'`. */
  roleLabel: string;
  /** The canonical species this seam grounds: `'Wolf'` or `'Horse'`. */
  species: string;
  summaryDetail: string;
  /** Only statistics the engine actually emitted — never zero-filled. */
  stats: CompanionStatDto[];
  /** Provably-vacuous named abilities (Link, Share Spells). */
  notes: string[];
  /**
   * The engine's non-blocking `advancement_absent` note: the honest list of
   * companion columns deliberately left ungrounded because nothing in the
   * codebase consumes them (bonus tricks, companion skills and feats, the
   * player-chosen stat increase at master levels 4/9/14/20, the size
   * advance, Evasion/Devotion/Multiattack).
   *
   * It travels on the companion rather than in `diagnostics` because it is
   * non-blocking, and `load_saved_character` returns an empty diagnostics
   * list on the `Computed` path — so this is the player's only route to it.
   * `skip_serializing_if` on the Rust side means the key may be absent.
   */
  advancementNote?: string;
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
  /**
   * The character's animal companion or mount, or absent when this build
   * grounds none. The key itself is omitted on the wire (same
   * `skip_serializing_if` discipline as `damageReduction`), so a
   * companion-less class is `undefined` here rather than an empty or
   * zeroed stat block.
   */
  companion?: AnimalCompanionDto;
  /**
   * The character's real spellbook coverage (spell save DCs) — mirrors
   * `PilotSpellbookDto` in `character_hub.rs`, projected from
   * `spellbook::compute_spellbook_coverage`. Absent (not zeroed), same
   * `skip_serializing_if` discipline as `damageReduction`/`companion`, for
   * a non-caster or a build with no spell yet resolved against the corpus.
   * Carries no slot totals/used counts — those would duplicate the
   * already-real `spellsPerDayModel.ts` surface (`decisions.md` Decision
   * 37, epic-31-spell-wiring gap closure, 2026-08-07).
   */
  spellbook?: PilotSpellbookDto;
}

/** One class's spell save DC — mirrors `SpellSaveDcDto` in `character_hub.rs`. */
export interface SpellSaveDcDto {
  classId: string;
  dc: number;
}

/**
 * The character's real spellbook coverage — mirrors `PilotSpellbookDto` in
 * `character_hub.rs`. No `slotsTotal`/`slotsUsed` fields — see
 * `spellbook?`'s doc comment above.
 */
export interface PilotSpellbookDto {
  spellSaveDc: SpellSaveDcDto[];
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
  /** This item's own armor-slot "Spell Resistance" special-ability contribution. */
  spellResistanceBonus?: number;
}

export interface EquipmentEffectsDto {
  /** Per-item contributions behind the aggregate totals below. */
  perItem: ResolvedEquipmentEffectDto[];
  armorClassDelta: number;
  armorCheckPenaltyTotal: number;
  maxDexCap?: number;
  spellFailureChance?: number;
  /**
   * The highest `perItem[].spellResistanceBonus` among everything equipped
   * -- PF1's real rule: multiple SR sources take the highest value, they
   * do not stack.
   */
  spellResistanceTotal?: number;
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

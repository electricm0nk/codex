import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CharacterSummaryDto } from './loadListSavedCharacters';
import type { CorpusDerivedDto, DiagnosticDto, PilotSnapshotDto } from './loadCreateCharacter';
import type { AcquisitionModeDto } from './addSpellSelection';
import type { RaceSelectionResponse } from './loadAlternateRacialTraits';

/** Mirrors `SpellSelectionImportDto` in `character_hub.rs` — a general-purpose round-trip shape, not import-only despite the name. */
export interface SpellSelectionDto {
  spellId: string;
  sourceClassId: string;
  acquisitionMode: AcquisitionModeDto;
}

/**
 * Read-only desktop boundary over a single saved character's detail.
 *
 * Invokes the `load_saved_character` Tauri command, which re-computes the
 * saved build via the real rules-core engine on every load (the receipt is
 * never itself persisted) and returns the summary, snapshot (when
 * `Computed`), and diagnostics verbatim.
 */

export interface LoadSavedCharacterRequest {
  characterId: string;
}

export interface LoadSavedCharacterResponse {
  summary: CharacterSummaryDto;
  snapshot: PilotSnapshotDto | null;
  diagnostics: DiagnosticDto[];
  corpusDerived: CorpusDerivedDto;
  /** The character's full persisted `chosen.selected_feats`, verbatim — not just feats added this session. */
  selectedFeats: string[];
  /**
   * The character's full persisted `chosen.spells_selected`, verbatim — not
   * just spells added this session. Lets a Wizard spell add tell whether
   * this is truly "the first spell" (needs the atomic
   * `recordAndPrepareSpellSelection` bootstrap) or whether the deadlock is
   * already broken (the cheaper plain `addSpellSelection` suffices).
   */
  spellsSelected: SpellSelectionDto[];
  /**
   * The resolved target(s) for every chooser feat the character holds.
   *
   * `selectedFeats` alone cannot say *which* weapon a Weapon Focus names,
   * and a repeatable feat taken twice appears there as two identical
   * strings. One entry per chooser feat, not per pick — nothing in the data
   * model pairs pick N with target N, so that pairing is not invented.
   */
  chosenFeatTargets: ChosenFeatTargetsDto[];
  /**
   * Every `ComputationExplanation` the engine emitted for this build.
   *
   * This is the channel the sheet's Class Features section reads. Before it
   * existed the engine's 636+ corpus-cited explanation records — every
   * sneak-attack die, rage bonus, lay-on-hands pool, channel die, ki pool
   * and hex DC — were computed and tested on every load and then dropped
   * right here at the IPC boundary.
   */
  explanations: ExplanationDto[];
  /**
   * One entry per equipped item the engine identifies as a weapon. Drives
   * the Weapons tab's rows.
   *
   * Carries no summed damage total, deliberately — see
   * `WeaponDamageDto`.
   */
  weaponDamage: WeaponDamageDto[];
  /**
   * The ARG alternate racial traits this character actually holds, as corpus
   * record keys ("Dwarf ~ Minesight"), read back out of its persisted
   * `chosen.selected_choices` (SD-27).
   *
   * Same shape of gap, and same fix, as `selectedFeats` and `spellsSelected`:
   * without it nothing in the frontend can tell a loaded character's racial-
   * trait choices, so the sheet would show a Dwarf with 90 ft darkvision and
   * no way to say why.
   */
  selectedAlternateTraitKeys: string[];
  /**
   * **AT-34-E4-002**: the character's full persisted `chosen.selected_traits`,
   * verbatim — not just traits added this session. Same shape of gap, and
   * same fix, as `selectedFeats`: without it a re-opened trait picker would
   * show no checkboxes checked, and a mutation refresh that dropped it would
   * silently clear a saved character's trait choices.
   */
  selectedTraits: string[];
  /**
   * **This character's racial traits, resolved and rendered for *it*.**
   *
   * `selectedAlternateTraitKeys` carries the choice; this carries what the
   * choice *says*. Produced by `character_hub::resolve_racial_traits_for_character`,
   * which calls the same `race_trait_picker` renderer the Race Traits picker
   * screen consumes — one renderer, several consumers (`decisions.md §29.1`).
   *
   * Every `description` / `text` in it is rendered against this character's own
   * persisted `selectedFeats`, so a halfling holding ARG's `Fortunate One`
   * reads "4 times per day" where the book prints three. **Render verbatim** —
   * it is corpus prose with the engine's numbers resolved into it.
   *
   * Project it with `characterHub/racialTraitsModel.ts`; do not read the fields
   * directly in a component.
   *
   * **`load_saved_character` always sends one.** `null` marks a detail object
   * this frontend *built* rather than loaded — a mutation response reshaped by
   * `toCharacterMutationRefresh`, or the browser preview's sample. Those
   * responses genuinely carry no resolution, and a mutation can change one
   * (adding `Fortunate One` moves the halfling luck sentence), so carrying the
   * pre-mutation prose forward would show a stale number as if it were current.
   * The same reasoning, and the same fix, as `explanations` and `weaponDamage`:
   * left absent, and re-read through `refreshEngineRecords`.
   */
  resolvedRacialTraits: RaceSelectionResponse | null;
}

/** Mirrors `ExplanationDto` in `character_hub.rs`. */
export interface ExplanationDto {
  /** Stable engine id, e.g. `class_chassis.rogue.sneak_attack`. */
  id: string;
  /** The computed magnitude this record explains. */
  value: number;
  /**
   * The engine's own corpus-cited derivation text.
   *
   * **Render verbatim.** Never paraphrase, summarise, or regenerate it:
   * it is the rules citation, and rewriting it here would create a second,
   * unverified source of rules prose — the hand-authored-rules-data debt
   * `docs/governance/no-stub-mvp-doctrine.md` forbids, and the same
   * discipline the Pets tab established.
   */
  detail: string;
}

/** Mirrors `DiceExpressionDto` in `character_hub.rs`. */
export interface DiceExpressionDto {
  count: number;
  dieSize: number;
}

/** Mirrors `WeaponFeatEffectDto` in `character_hub.rs`. */
export interface WeaponFeatEffectDto {
  featKey: string;
  damageBonus: number;
}

/**
 * Mirrors `WeaponDamageDto` in `character_hub.rs` — one equipped weapon's
 * corpus-cited damage facets.
 *
 * Every facet is nullable, and `null` means the engine found no corpus
 * token for it on this weapon. That is honest absence, not zero: a null
 * critical multiplier is not `x0` and not a defaulted `x2`, and the sheet
 * must render it as absent.
 *
 * There is deliberately no summed damage total. No such formula exists
 * anywhere in the engine, and the wield multiplier needed to build one
 * honestly is unknown — see `contract.rs`'s `PilotReceipt::weapon_damage`
 * boundary note. The sheet shows the breakdown as separate columns.
 */
export interface WeaponDamageDto {
  weaponItemId: string;
  /** The resolved corpus record key, e.g. `Longsword (Base)`. */
  weaponRecordKey: string | null;
  baseDice: DiceExpressionDto | null;
  /**
   * The Strength modifier's contribution to this weapon's damage. Bounded
   * to the primary hand — `EquipmentSelection` carries no hand-slot field
   * yet, so a genuine off-hand weapon currently shows its primary-hand
   * fraction.
   */
  strDamageModifier: number | null;
  /** `OneHanded` / `TwoHanded` / `Light`, from the corpus `WIELD:` token. */
  wieldCategory: string | null;
  enhancementAttackBonus: number | null;
  enhancementDamageBonus: number | null;
  /** Inclusive `[low, high]` threat bounds, e.g. `[19, 20]`. */
  criticalThreatRange: [number, number] | null;
  criticalMultiplier: number | null;
  /** Constant feat damage bonuses; gathered per character, not per weapon. */
  featEffects: WeaponFeatEffectDto[];
}

export interface ChosenFeatTargetsDto {
  /** Verbatim as it appears in `selectedFeats`. */
  featId: string;
  /** `'Weapon'`, `'Skill'` or `'SpellSchool'`. */
  targetKind: string;
  /**
   * Prefix-stripped targets. Empty when the feat is held but no target was
   * ever recorded — a real state to display, not an error.
   */
  targets: string[];
}

export async function loadSavedCharacterDetail(
  request: LoadSavedCharacterRequest
): Promise<LoadSavedCharacterResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading a saved character');
  }

  try {
    return await invoke<LoadSavedCharacterResponse>('load_saved_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to load saved character: ${formatError(cause)}`);
  }
}

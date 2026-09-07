import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over the character trait/drawback picker
 * (`apps/desktop/src-tauri/src/trait_picker.rs`, AT-34-E4-002).
 *
 * A single command: `list_available_character_traits` returns the real,
 * corpus-derived roster of traits this crate's compute path genuinely
 * supports — `ultimate_campaign`'s 31 flat `BONUS:SKILL` traits, 5
 * fixed-choice `BONUS:SKILL|%LIST` traits (second slice), 4 open-family
 * `BONUS:SKILL|%LIST` traits (third slice), 2 flat `BONUS:SAVE`
 * traits (fourth slice), 3 flat `BONUS:COMBAT|INITIATIVE`/
 * `BONUS:CONCENTRATION|ALLSPELLS` traits (fifth slice, `otherPillars`),
 * 4 ability-score-difference-formula traits (sixth slice,
 * `abilitySubstitution`), and 3 `BONUS:SITUATION` traits (seventh slice)
 * — every option returned really does grant its stated bonus once
 * selected (and, for a choice-based option, a valid `skillOptions` choice
 * recorded) and submitted on
 * `CreateCharacterRequest.selectedTraits`/`.traitSkillChoices`
 * (`trait_effects::skill_bonuses_from_traits` +
 * `trait_effects::skill_choice_bonuses_from_traits` +
 * `trait_effects::family_choice_bonuses_from_traits` +
 * `trait_effects::save_bonuses_from_traits` +
 * `pilot_compute::ground_orphan_trait_facts`'s situational-fact channel +
 * `trait_effects::situational_flat_skill_bonuses_from_traits` +
 * `pilot_compute::ground_orphan_trait_facts`'s initiative/concentration
 * standalone-fact channel +
 * `skill_allocation::allocate_skill_ranks`'s
 * `trait_effects::ability_diff_skill_bonuses_from_traits` fold-in). No
 * "resolve" step exists (unlike `loadAlternateRacialTraits`'s pair): no
 * trait shape here has alternate-swap exclusivity or per-character
 * rendered prose to compute ahead of submission.
 *
 * **Corrected note (this cycle):** the fifth/sixth-slice traits (Tactician,
 * Arcane Temper, Desperate Resolve, Bruising Intellect, Planar Savant,
 * Pragmatic Activator, Precise Treatment) were genuinely computed but
 * absent from this command's roster before this cycle — a prior claim that
 * "the picker already surfaces every selected trait generically" held only
 * for a trait selected some other way, never for a brand-new selection.
 * This cycle chains both remaining tables in.
 */

/** One skill a choice-based trait's `%LIST` can resolve to. */
export interface TraitSkillOptionDto {
  /** Echoed back as `TraitSkillChoiceDto.selectionId`. */
  skillId: string;
  name: string;
}

export interface CharacterTraitOptionDto {
  /** Echoes back on `CreateCharacterRequest.selectedTraits` verbatim. */
  id: string;
  name: string;
  description: string;
  /** Display-name skill(s) this trait's bonus applies to, e.g. `['Acrobatics']`. Empty for a choice-based trait (`skillOptions` non-empty instead). */
  skills: string[];
  bonus: number;
  /** Non-empty only for a fixed-choice `%LIST` trait: the concrete skills the player may pick between. */
  skillOptions: TraitSkillOptionDto[];
  /** `choiceSetId` to echo back (paired with the picked `skillOptions` entry) on `CreateCharacterRequest.traitSkillChoices`. `null` for a flat trait. */
  choiceSetId: string | null;
  /** `'Fortitude' | 'Reflex' | 'Will'` only for a fourth-slice flat `BONUS:SAVE` trait; `null` for every skill-pillar trait. */
  save: string | null;
  /**
   * Non-empty only for a fifth-slice flat `BONUS:COMBAT|INITIATIVE`
   * and/or `BONUS:CONCENTRATION|ALLSPELLS` trait — one entry per pillar
   * the record's corpus tokens carry, so `Arcane Temper` (both pillars on
   * one record) carries two entries on ONE option, never two options.
   */
  otherPillars: TraitOtherPillarBonusDto[];
  /** `non-null` only for a sixth-slice ability-score-difference-formula trait; `null` for every other slice. */
  abilitySubstitution: TraitAbilitySubstitutionDto | null;
}

/** One non-skill, non-save pillar bonus a fifth-slice option grants. */
export interface TraitOtherPillarBonusDto {
  /** `'Initiative checks'` or `'Concentration checks'`. */
  label: string;
  bonus: number;
}

/**
 * A sixth-slice ability-score-substitution formula. The real numeric
 * result depends on the character's own computed ability modifiers
 * (evaluated backend-side, never by this DTO), so this carries the
 * formula text itself plus any additional flat component rather than a
 * single pre-computed number.
 */
export interface TraitAbilitySubstitutionDto {
  /** The formula's own literal text (`'max(INT,CHA)-CHA'`), verbatim from the corpus token. */
  formula: string;
  /** A second, flat bonus on the SAME skill this record also carries (`0` for three of the four records). */
  flatBonus: number;
}

export async function loadCharacterTraits(): Promise<CharacterTraitOptionDto[]> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading character traits');
  }

  try {
    return await invoke<CharacterTraitOptionDto[]>('list_available_character_traits');
  } catch (cause: unknown) {
    throw new Error(`Failed to load character traits: ${formatError(cause)}`);
  }
}

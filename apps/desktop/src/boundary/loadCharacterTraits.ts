import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over the character trait/drawback picker
 * (`apps/desktop/src-tauri/src/trait_picker.rs`, AT-34-E4-002).
 *
 * A single command: `list_available_character_traits` returns the real,
 * corpus-derived roster of traits this cycle's compute path genuinely
 * supports — `ultimate_campaign`'s 31 flat `BONUS:SKILL` traits, 5
 * fixed-choice `BONUS:SKILL|%LIST` traits (second slice), 4 open-family
 * `BONUS:SKILL|%LIST` traits (third slice), and 2 flat `BONUS:SAVE`
 * traits (fourth slice) — every option returned really does grant its
 * stated bonus once selected (and, for a choice-based option, a valid
 * `skillOptions` choice recorded) and submitted on
 * `CreateCharacterRequest.selectedTraits`/`.traitSkillChoices`
 * (`trait_effects::skill_bonuses_from_traits` +
 * `trait_effects::skill_choice_bonuses_from_traits` +
 * `trait_effects::family_choice_bonuses_from_traits` +
 * `trait_effects::save_bonuses_from_traits`). No "resolve" step exists
 * (unlike `loadAlternateRacialTraits`'s pair): no trait shape here has
 * alternate-swap exclusivity or per-character rendered prose to compute
 * ahead of submission.
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

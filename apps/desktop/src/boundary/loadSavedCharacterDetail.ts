import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CharacterSummaryDto } from './loadListSavedCharacters';
import type { CorpusDerivedDto, DiagnosticDto, PilotSnapshotDto } from './loadCreateCharacter';
import type { AcquisitionModeDto } from './addSpellSelection';

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

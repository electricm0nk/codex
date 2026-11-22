import {
  loadAlternateRacialTraits,
  resolveRaceAlternateSelection,
  type AlternateRacialTraitsResponse,
  type RaceSelectionResponse,
} from '../boundary/loadAlternateRacialTraits';
import { loadListSavedCharacters, type CharacterSummaryDto } from '../boundary/loadListSavedCharacters';
import { loadSavedCharacterDetail } from '../boundary/loadSavedCharacterDetail';
import { hasTauriRuntime } from '../boundary/runtime';

/**
 * Runtime access for the Alternate Racial Traits picker.
 *
 * **Deliberately no browser-preview sample data.** The sibling
 * `raceCatalogRuntime.ts` ships one because the Race Traits catalog is a flat
 * read-only list, so a handful of illustrative rows misleads nobody. This
 * screen is different in kind: its whole job is to show a *resolution* —
 * which standard trait a chosen alternate suppresses, and which sibling
 * alternates that choice locks out. Faking either in the browser would mean
 * re-implementing `decisions.md §26`'s protocol in TypeScript, producing a
 * screen that swaps traits according to frontend logic rather than the engine.
 * That is precisely the fixture-in-a-production-path failure
 * `docs/governance/no-stub-mvp-doctrine.md` prohibits.
 *
 * So without the desktop runtime the screen says so, plainly, and offers
 * nothing to click.
 */
export const NO_RUNTIME_MESSAGE =
  'Alternate racial traits need the desktop runtime: every swap on this screen is resolved by the ' +
  'rules engine reading the corpus, and the browser preview has no engine to ask.';

export function alternateTraitPickerAvailable(): boolean {
  return hasTauriRuntime();
}

export async function loadAlternateRacialTraitsRuntime(): Promise<AlternateRacialTraitsResponse> {
  if (!hasTauriRuntime()) {
    throw new Error(NO_RUNTIME_MESSAGE);
  }
  return loadAlternateRacialTraits();
}

export async function resolveRaceAlternateSelectionRuntime(
  raceKey: string,
  selectedAlternateKeys: readonly string[],
  heldFeats: readonly string[] = [],
): Promise<RaceSelectionResponse> {
  if (!hasTauriRuntime()) {
    throw new Error(NO_RUNTIME_MESSAGE);
  }
  return resolveRaceAlternateSelection(raceKey, selectedAlternateKeys, heldFeats);
}

/**
 * The saved characters this screen can show numbers *for*.
 *
 * A racial trait's description states magnitudes — "three times per day", "a +1
 * bonus" — and several of those magnitudes are raised by feats the character
 * holds. Without a character in hand the screen can only show the racial base,
 * which is a true answer for the book and the wrong one for the player.
 *
 * The roster comes from `list_saved_characters` and the feats from
 * `load_saved_character`'s own `selected_feats`, so the context is a real
 * persisted character. **Nothing here fabricates a character to demonstrate
 * the feature**: with none saved, the screen says so and shows the base.
 */
export async function loadCharacterContextsRuntime(): Promise<CharacterSummaryDto[]> {
  if (!hasTauriRuntime()) {
    return [];
  }
  const response = await loadListSavedCharacters();
  return [...response.characters].sort((left, right) => left.displayLabel.localeCompare(right.displayLabel));
}

/** One saved character's real persisted feat list, verbatim. */
export async function loadHeldFeatsRuntime(characterId: string): Promise<string[]> {
  if (!hasTauriRuntime()) {
    throw new Error(NO_RUNTIME_MESSAGE);
  }
  const detail = await loadSavedCharacterDetail({ characterId });
  return detail.selectedFeats;
}

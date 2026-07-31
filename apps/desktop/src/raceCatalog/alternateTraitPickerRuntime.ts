import {
  loadAlternateRacialTraits,
  resolveRaceAlternateSelection,
  type AlternateRacialTraitsResponse,
  type RaceSelectionResponse,
} from '../boundary/loadAlternateRacialTraits';
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
): Promise<RaceSelectionResponse> {
  if (!hasTauriRuntime()) {
    throw new Error(NO_RUNTIME_MESSAGE);
  }
  return resolveRaceAlternateSelection(raceKey, selectedAlternateKeys);
}

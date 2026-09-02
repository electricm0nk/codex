import type { RaceSelectionResponse } from '../boundary/loadAlternateRacialTraits';
import { buildRacialTraitsSurface, type RacialTraitRow, type ReplacedTrait } from './racialTraitsModel';

/**
 * What the create form shows under "Racial Traits" for the currently
 * picked race and alternate selection (v0.8 F-2, scout audit item 5).
 *
 * Reuses the sheet's own `buildRacialTraitsSurface` over the same
 * `resolve_race_alternate_selection` payload the alternate picker already
 * receives on every race change, and then hides the `alternate` rows —
 * those are the checkboxes rendered directly beneath, so repeating them
 * here would only double the list. Standard (`default`) and
 * `flagGranted` rows stay, in the resolver's order, with its prose.
 */
export interface CreationRacialTraitsPreview {
  rows: RacialTraitRow[];
  replaced: ReplacedTrait[];
  unavailableReason: string | null;
}

export function buildCreationRacialTraitsPreview(
  resolved: RaceSelectionResponse | null | undefined,
): CreationRacialTraitsPreview {
  const surface = buildRacialTraitsSurface(resolved);
  return {
    rows: surface.rows.filter((row) => row.role !== 'alternate'),
    replaced: surface.replaced,
    unavailableReason: surface.unavailableReason,
  };
}

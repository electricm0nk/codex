import type {
  AlternateRacialTraitsResponse,
  AlternateTraitDto,
  RacePickerDto,
  RaceSelectionResponse,
} from '../boundary/loadAlternateRacialTraits';

/**
 * Pure helpers for taking an alternate racial trait *during character
 * creation*, from any book the race corpus loads.
 *
 * The Race Traits picker screen (`raceCatalog/`) already browses every one of
 * them and resolves any swap live. This module is the missing half: matching
 * that catalogue up to the creation form's own race identity, and narrowing
 * the menu to the race being created.
 *
 * The total is deliberately not written down here. It read 153 while ARG was
 * the only contributing book, then 157, then 158; a count in a comment is the
 * kind that goes stale without anything failing.
 *
 * **Nothing here decides whether a trait applies, or whether two traits may be
 * held together.** Both answers come from the backend — `RaceCorpus::resolve`
 * for the swap, the record's own `PREMULT` guard (read by
 * `race_trait_picker.rs`) for
 * the exclusion — and `create_character` re-validates the submitted keys
 * against the corpus before persisting anything. Re-deciding either here would
 * give the form a second, divergent opinion, and the form would be the one the
 * player sees.
 */

/**
 * The picker race whose corpus key matches a creation-form `race:<slug>` id.
 *
 * The two identities come from the same place — `race_catalog.rs` builds the
 * creation roster's id as `format!("race:{}", race.race_key.to_lowercase())`
 * off the same corpus record the picker serves — so this is a match on the
 * backend's own construction rule, not a mapping table that could drift. A
 * race the picker did not serve returns `null`; the form then offers nothing
 * rather than an empty box implying "this race has no alternates".
 */
export function pickerRaceForRaceId(
  menu: AlternateRacialTraitsResponse | null,
  raceId: string,
): RacePickerDto | null {
  if (!menu) {
    return null;
  }
  const slug = raceId.trim().replace(/^race:/, '').toLowerCase();
  return menu.races.find((race) => race.raceKey.toLowerCase() === slug) ?? null;
}

/** The alternates offered for a creation-form race id, alphabetical by name. */
export function alternatesForRaceId(
  menu: AlternateRacialTraitsResponse | null,
  raceId: string,
): AlternateTraitDto[] {
  const race = pickerRaceForRaceId(menu, raceId);
  if (!race) {
    return [];
  }
  return [...race.alternates].sort((left, right) => left.name.localeCompare(right.name));
}

/**
 * Selections that are no longer valid after the player switches race.
 *
 * Switching race must not silently carry a Dwarf's traits onto an Elf —
 * `create_character` would refuse the save, which is correct but is a worse
 * experience than clearing the choices at the moment they stop applying.
 */
export function retainSelectionsValidForRace(
  menu: AlternateRacialTraitsResponse | null,
  raceId: string,
  selected: readonly string[],
): string[] {
  const offered = new Set(alternatesForRaceId(menu, raceId).map((alternate) => alternate.key));
  return selected.filter((key) => offered.has(key));
}

/** One row of the creation form's alternate-trait list. */
export interface AlternateTraitRow {
  alternate: AlternateTraitDto;
  selected: boolean;
  /**
   * Non-null when the current selection locks this alternate out, carrying the
   * backend's own reason. A selected alternate is never disabled — the player
   * must always be able to undo a choice.
   */
  disabledReason: string | null;
  /**
   * **The prose to show, with its numbers in it.**
   *
   * Rendered by `race_trait_picker::render_trait_description` from the corpus
   * row's own `DESC:` tokens — never the stored `data.description`, whose
   * magnitudes were collapsed (and, for `Halfling ~ Adaptable Luck`, lost) at
   * ingest time.
   *
   * Taken from the live resolution when one has arrived, because that is the
   * rendering produced for whoever the resolve call named; the menu's own
   * rendering (the same renderer, called with no feats) until then. Never
   * blank: a row with a name and no sentence is what this screen showed
   * before, and it told a player nothing about what they were choosing.
   */
  description: string;
  /**
   * True when the feats passed to the resolve call changed this sentence from
   * its racial base. The engine's claim, derived by rendering the record twice
   * and comparing — never inferred here from a feat list.
   */
  movedByFeats: boolean;
  /**
   * `DESC:` arguments the engine could not resolve to a literal and therefore
   * dropped, so a partially-resolved sentence is visibly partial.
   */
  droppedArgs: string[];
}

/**
 * The list to render, with the backend's live resolution applied.
 *
 * `resolution` is the `resolve_race_alternate_selection` response for the
 * current selection. Until it arrives, nothing is disabled — showing a
 * lockout the engine has not confirmed would be the frontend guessing.
 */
export function buildAlternateTraitRows(
  menu: AlternateRacialTraitsResponse | null,
  raceId: string,
  selected: readonly string[],
  resolution: RaceSelectionResponse | null,
): AlternateTraitRow[] {
  const blocked = new Map<string, string>();
  for (const entry of resolution?.blockedAlternates ?? []) {
    if (!blocked.has(entry.key)) {
      blocked.set(entry.key, `Locked out by ${entry.blockedByName} — both replace the same standard trait (${entry.flag}).`);
    }
  }
  // The resolution renders every trait record the race declares — selected or
  // not — precisely so an alternate the player has not ticked yet still shows
  // the number *they* would get.
  const rendered = new Map(
    (resolution?.renderedTraitDescriptions ?? []).map((row) => [row.key, row] as const),
  );
  const chosen = new Set(selected);
  return alternatesForRaceId(menu, raceId).map((alternate) => {
    const row = rendered.get(alternate.key);
    return {
      alternate,
      selected: chosen.has(alternate.key),
      disabledReason: chosen.has(alternate.key) ? null : (blocked.get(alternate.key) ?? null),
      description: row?.text ?? alternate.description,
      movedByFeats: row?.movedByFeats ?? false,
      droppedArgs: row?.droppedArgs ?? [],
    };
  });
}

/**
 * The one-line summary under the list: what the engine says this selection
 * actually does to the character.
 *
 * Every fact in it is the resolver's, carried through. The "replaces nothing"
 * case is stated rather than left blank — an alternate whose swap target is
 * missing upstream is a real corpus finding (9 Aasimar alternates today), and
 * a player is entitled to see that the trait they took removed nothing.
 */
export function describeCreationSelection(
  selected: readonly string[],
  resolution: RaceSelectionResponse | null,
): string {
  if (selected.length === 0) {
    return 'No alternate racial traits chosen — this character keeps every standard trait for its race.';
  }
  if (!resolution) {
    return 'Resolving…';
  }
  if (resolution.errors.length > 0) {
    return resolution.errors.join('; ');
  }
  const traitWord = selected.length === 1 ? 'trait' : 'traits';
  if (resolution.suppressions.length === 0) {
    return `${selected.length} alternate ${traitWord} chosen, replacing nothing in the loaded books.`;
  }
  const replaced = resolution.suppressions
    .map((suppression) => `${suppression.suppressedTraitName} (by ${suppression.setByTraitName})`)
    .join(', ');
  return `${selected.length} alternate ${traitWord} chosen. Replaces: ${replaced}.`;
}

/**
 * Problems the player must see before submitting, in the backend's words.
 *
 * These are exactly the conditions `create_character` refuses to persist, so a
 * form that shows none of them and then reports "Blocked" would be hiding the
 * reason it already had.
 */
export function creationSelectionWarnings(resolution: RaceSelectionResponse | null): string[] {
  if (!resolution) {
    return [];
  }
  const warnings: string[] = [];
  for (const key of resolution.unmatchedSelections) {
    warnings.push(`"${key}" matches no alternate racial trait for this race.`);
  }
  for (const conflict of resolution.conflictingSelections) {
    warnings.push(
      `${conflict.name} and ${conflict.blockedByName} cannot both be taken — both replace the same standard trait (${conflict.flag}).`,
    );
  }
  for (const flag of resolution.inertFlags) {
    warnings.push(
      `${flag} fired but replaced nothing in the loaded books, so the standard trait would still apply — this character cannot be saved with that combination.`,
    );
  }
  return warnings;
}

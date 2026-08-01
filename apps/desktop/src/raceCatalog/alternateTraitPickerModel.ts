import type {
  AlternateRacialTraitsResponse,
  AlternateTraitDto,
  BlockedAlternateDto,
  RacePickerDto,
  RaceSelectionResponse,
  RenderedTraitDescriptionDto,
  SuppressionDto,
} from '../boundary/loadAlternateRacialTraits';

/**
 * Pure view-model helpers for the Alternate Racial Traits picker.
 *
 * Everything here is presentation of backend facts. Nothing decides whether a
 * trait applies, whether an alternate is available, or what replaces what —
 * those three answers come from `race_trait_picker.rs`, which in turn calls
 * `RaceCorpus::resolve` for the protocol itself. Re-deciding any of them here
 * would give the screen a second, divergent opinion.
 */

/** One selection change: add the key if absent, remove it if present. */
export function toggleSelection(selected: readonly string[], key: string): string[] {
  return selected.includes(key) ? selected.filter((existing) => existing !== key) : [...selected, key];
}

/**
 * The screen's one-line description of its own contents, derived from what the
 * backend actually served rather than a compiled-in roster.
 */
export function describePicker(response: AlternateRacialTraitsResponse): string {
  const raceCount = response.races.length;
  const alternateCount = response.races.reduce((total, race) => total + race.alternates.length, 0);
  const alternateWord = alternateCount === 1 ? 'alternate racial trait' : 'alternate racial traits';
  const raceWord = raceCount === 1 ? 'race' : 'races';
  return `${alternateCount} ${alternateWord} across ${raceCount} ${raceWord}`;
}

/**
 * What an alternate replaces, in a sentence.
 *
 * The empty case is *not* silently blank: an alternate whose replace-flag
 * matches no standard trait is a corpus finding, and the reader is told which
 * flag and that nothing in the loaded books declares it.
 *
 * No shipped record reaches that branch today — ARG's nine Aasimar alternates
 * did until `ingest_races` learned to read PCGen's `globalvar` gate, and
 * `race_trait_picker` now pins the live menu as carrying no unmatched flag.
 * The branch stays because the next book can reintroduce the shape, and a row
 * that reads "Replaces —" is how a dead affordance goes unnoticed.
 */
export function describeReplacement(alternate: AlternateTraitDto): string {
  const parts: string[] = [];
  if (alternate.replaces.length > 0) {
    parts.push(`Replaces ${alternate.replaces.map((link) => link.name).join(', ')}`);
  }
  if (alternate.grants.length > 0) {
    parts.push(`Grants ${alternate.grants.map((link) => link.name).join(', ')}`);
  }
  if (alternate.unmatchedFlags.length > 0) {
    const flagWord = alternate.unmatchedFlags.length === 1 ? 'flag' : 'flags';
    parts.push(
      `Replaces nothing in the loaded books — ${flagWord} ${alternate.unmatchedFlags.join(', ')} ` +
        'matches no standard trait (corpus finding, not a display gap)',
    );
  }
  return parts.join('. ');
}

/** Suppressed standard-trait key → the suppression the engine reported. */
export function suppressionsByTraitKey(selection: RaceSelectionResponse | null): Map<string, SuppressionDto> {
  const index = new Map<string, SuppressionDto>();
  for (const suppression of selection?.suppressions ?? []) {
    index.set(suppression.suppressedTraitKey, suppression);
  }
  return index;
}

/**
 * Alternate key → why it is currently unavailable. Built from the backend's
 * `blockedAlternates`, which reads ARG's PREMULT self-exclusion guard; a
 * selected alternate is never blocked.
 */
export function blocksByAlternateKey(selection: RaceSelectionResponse | null): Map<string, BlockedAlternateDto> {
  const index = new Map<string, BlockedAlternateDto>();
  for (const blocked of selection?.blockedAlternates ?? []) {
    if (!index.has(blocked.key)) {
      index.set(blocked.key, blocked);
    }
  }
  return index;
}

export function describeBlock(blocked: BlockedAlternateDto): string {
  return `Locked out by ${blocked.blockedByName} — both replace the same standard trait (${blocked.flag}).`;
}

/**
 * The races worth offering, and the order to offer them in: every race the
 * backend served, most alternates first, ties broken by name so the list is
 * stable. A race with no alternates is kept rather than hidden — "this race
 * has none" is a true and useful answer, and hiding it would make the picker
 * silently disagree with the Race Traits catalog's roster.
 */
export function orderRacesByAlternateCount(races: readonly RacePickerDto[]): RacePickerDto[] {
  return [...races].sort((left, right) =>
    left.alternates.length === right.alternates.length
      ? left.raceName.localeCompare(right.raceName)
      : right.alternates.length - left.alternates.length,
  );
}

/**
 * The line under the resolved trait list. Reports what the engine did, and
 * says so plainly when it did nothing.
 */
export function describeSelectionOutcome(selection: RaceSelectionResponse | null): string {
  if (!selection) {
    return 'Resolving…';
  }
  if (selection.errors.length > 0) {
    return selection.errors.join('; ');
  }
  const applied = selection.appliedTraits.length;
  const traitWord = applied === 1 ? 'trait' : 'traits';
  if (selection.suppressions.length === 0) {
    return `${applied} ${traitWord} apply. No alternate selected, so nothing is replaced.`;
  }
  const replaced = selection.suppressions
    .map((suppression) => `${suppression.suppressedTraitName} (by ${suppression.setByTraitName})`)
    .join(', ');
  return `${applied} ${traitWord} apply. Replaced: ${replaced}.`;
}

// --- rendered descriptions -------------------------------------------------

/**
 * Trait key → the description the engine rendered for the current character.
 *
 * Empty until the resolve call returns, which is why every reader pairs it with
 * a fallback rather than rendering a blank cell.
 */
export function descriptionsByTraitKey(
  selection: RaceSelectionResponse | null,
): Map<string, RenderedTraitDescriptionDto> {
  const index = new Map<string, RenderedTraitDescriptionDto>();
  for (const rendered of selection?.renderedTraitDescriptions ?? []) {
    index.set(rendered.key, rendered);
  }
  return index;
}

/**
 * The prose to put on screen for one trait.
 *
 * Prefers the engine's per-character rendering and falls back to the menu's own
 * description — which is the *same renderer* run with no feats, so the fallback
 * is the racial base rather than a different kind of text. **The fallback is
 * never a locally-assembled sentence**: this screen has no way to compute a
 * display value and must not appear to.
 */
export function traitDescription(
  rendered: Map<string, RenderedTraitDescriptionDto>,
  key: string,
  menuDescription: string,
): string {
  return rendered.get(key)?.text ?? menuDescription;
}

/**
 * The line stating whose numbers are on screen, and why they differ from the
 * book's printed ones.
 *
 * `displayValueFeats` is the engine's own list of held feats that actually
 * moved a value — so a character holding twelve feats is credited for the one
 * that changed the sentence, and a character whose feats changed nothing is
 * told exactly that instead of being implied to have altered something.
 */
export function describeCharacterContext(
  characterLabel: string | null,
  selection: RaceSelectionResponse | null,
): string {
  if (!characterLabel) {
    return 'Showing each trait as the book prints it. Pick a saved character to see the numbers their feats give them.';
  }
  if (!selection) {
    // "None of their feats moved anything" is a claim only the engine can
    // make, and it has not answered yet. Saying it here would be a guess that
    // happens to be right most of the time — the worst kind.
    return `Reading ${characterLabel}'s feats…`;
  }
  const moved = selection.displayValueFeats;
  if (moved.length === 0) {
    return `Showing ${characterLabel}'s numbers. None of their feats changes a racial trait's stated value.`;
  }
  const changed = selection.renderedTraitDescriptions.filter((row) => row.movedByFeats).length;
  const traitWord = changed === 1 ? 'trait' : 'traits';
  return `Showing ${characterLabel}'s numbers. ${moved.join(', ')} ${
    moved.length === 1 ? 'raises' : 'raise'
  } the value stated by ${changed} ${traitWord}.`;
}

/**
 * Problems in the current selection that the player must see: keys that
 * matched no alternate, pairs that violate each other's exclusion guard, and
 * flags that fired without replacing or granting anything.
 */
export function selectionWarnings(selection: RaceSelectionResponse | null): string[] {
  if (!selection) {
    return [];
  }
  const warnings: string[] = [];
  for (const key of selection.unmatchedSelections) {
    warnings.push(`Selection "${key}" matches no alternate racial trait for this race.`);
  }
  for (const conflict of selection.conflictingSelections) {
    warnings.push(
      `${conflict.name} and ${conflict.blockedByName} cannot both be taken — both replace the same ` +
        `standard trait (${conflict.flag}).`,
    );
  }
  for (const flag of selection.inertFlags) {
    warnings.push(`Flag ${flag} fired but replaced and granted nothing in the loaded books.`);
  }
  return warnings;
}

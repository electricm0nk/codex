import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over the Alternate Racial Traits picker
 * (`apps/desktop/src-tauri/src/race_trait_picker.rs`).
 *
 * Two commands, and the split between them matters:
 *
 * - `list_alternate_racial_traits` returns the **menu**: every race, its
 *   standard traits, and every ARG alternate with the standard trait(s) each
 *   one replaces. The replace links are resolved in the backend by matching
 *   the corpus replace-flag on both ends — never by name.
 * - `resolve_race_alternate_selection` returns what a chosen set of alternates
 *   actually **resolves to**, by calling `RaceCorpus::resolve`, the single
 *   implementation of `decisions.md §26`'s protocol ("a standard trait applies
 *   iff no selected alternate has set its `suppressed_by_flag`").
 *
 * The frontend never decides whether a trait applies. Every suppression the
 * screen renders is one the engine reported. No count or roster is pinned
 * here: the screen derives everything from what arrives.
 *
 * # Descriptions carry numbers, and the numbers depend on the character
 *
 * Every `description` below is *rendered* by the engine from the corpus row's
 * own `DESC:` tokens — not transcribed from the stored prose, whose numbers
 * were collapsed (and, for `Halfling ~ Adaptable Luck`, lost) at ingest time.
 *
 * `resolveRaceAlternateSelection` therefore takes the character's held feats.
 * They change no *resolution* — a feat fires no replace-flag and blocks no
 * alternate — only the magnitudes the sentences state, which is why they are a
 * separate argument rather than part of the selection request. Pass a
 * character's real `selectedFeats` from `loadSavedCharacterDetail`, or nothing
 * at all to get the racial base.
 */

/** A standard trait an alternate swaps out, or a replacement row it brings in. */
export interface LinkedTraitDto {
  key: string;
  name: string;
  /** The corpus flag that made the link — the evidence for the relationship. */
  flag: string;
}

export interface AlternateTraitDto {
  /** Corpus key, e.g. "Dwarf ~ Saltbeard". Round-trips into the resolve call. */
  key: string;
  name: string;
  book: string;
  description: string;
  /** Real page, or null when the corpus row carries the placeholder `p.xx`. */
  sourcePage: string | null;
  setsFlags: string[];
  replaces: LinkedTraitDto[];
  grants: LinkedTraitDto[];
  /**
   * Flags that match no standard trait and grant nothing — a corpus finding
   * the screen shows rather than hides.
   */
  unmatchedFlags: string[];
  /**
   * Flags named by this alternate's PREMULT self-exclusion guard. While any is
   * already set by a different selected alternate, this one cannot be taken.
   */
  exclusionGuardFlags: string[];
}

export interface StandardTraitDto {
  key: string;
  name: string;
  book: string;
  description: string;
  suppressedByFlag: string | null;
}

export interface RacePickerDto {
  /** Alphanumeric identity shared with the Race Traits catalog ("HalfElf"). */
  raceId: string;
  /** The corpus race key ("Half-Elf") — what the resolve command takes. */
  raceKey: string;
  raceName: string;
  book: string;
  standardTraits: StandardTraitDto[];
  alternates: AlternateTraitDto[];
}

/** A trait an {@link AdoptiveParentageOptionDto} grants, already ingested. */
export interface AdoptiveParentageGrantDto {
  key: string;
  name: string;
}

/**
 * One "Adoptive Parentage" option (`decisions.md §16` item 2): a member of
 * `Human ~ Adoptive Parentage`'s CHOOSE pool. A Human character replaces
 * Bonus Feat with that alternate trait (one of `RacePickerDto.alternates`
 * for `Human`, elsewhere on this same response), then picks one of these —
 * "you were adopted and raised by <race>". Not race-scoped the way
 * `RacePickerDto` is: picking one is a Human character's choice of which
 * other race raised them, not a trait of the race named here.
 */
export interface AdoptiveParentageOptionDto {
  /** The corpus key, e.g. "Dwarf" — also the adopted race's own name. */
  key: string;
  name: string;
  book: string;
  adoptedRace: string;
  /** Real corpus `DESC:` prose, verbatim — a fixed sentence, never rendered. */
  description: string;
  /** The already-ingested traits this option grants. Empty is a real answer. */
  grants: AdoptiveParentageGrantDto[];
}

/** One Trait an {@link AdoptedRaceOptionDto} can grant, from the real Trait pool. */
export interface AdoptedRaceTraitGrantDto {
  key: string;
  name: string;
  description: string | null;
  book: string;
}

/**
 * One "Adopted Race" selector (`decisions.md §25`): a character of the named
 * race's own type may pick ONE trait from that race's real Trait pool
 * (`kind: trait`, a different content kind from this response's own
 * `race_trait` population).
 */
export interface AdoptedRaceOptionDto {
  key: string;
  name: string;
  book: string;
  adoptedRace: string;
  /**
   * The real Trait pool this option offers. Empty is a legitimate, honestly
   * reported answer for a race whose Trait pool is not (yet) ingested — never
   * papered over with a fabricated trait.
   */
  grants: AdoptedRaceTraitGrantDto[];
  /**
   * `true` for a row whose `CHOOSE:` token could not be read at all — a
   * malformed-row finding surfaced rather than treated as "empty pool".
   */
  malformedChooseToken: boolean;
}

export interface AlternateRacialTraitsResponse {
  races: RacePickerDto[];
  /**
   * `Human ~ Adoptive Parentage`'s CHOOSE pool, resolved. See
   * {@link AdoptiveParentageOptionDto}.
   */
  adoptiveParentageOptions: AdoptiveParentageOptionDto[];
  /**
   * The "Adopted Race" selectors (`decisions.md §25`), resolved against the
   * real Trait pool. See {@link AdoptedRaceOptionDto}.
   */
  adoptedRaceOptions: AdoptedRaceOptionDto[];
  /** Corpus files the adapter could not read. Empty in a healthy checkout. */
  diagnostics: string[];
  /** Corpus-quality findings the backend refuses to hide. */
  findings: string[];
}

export interface SuppressionDto {
  suppressedTraitKey: string;
  suppressedTraitName: string;
  flag: string;
  setByTraitKey: string;
  setByTraitName: string;
}

export interface BlockedAlternateDto {
  key: string;
  name: string;
  flag: string;
  blockedByKey: string;
  blockedByName: string;
}

export interface AppliedTraitDto {
  key: string;
  name: string;
  book: string;
  /** "default" | "alternate" | "flagGranted" — the resolver's own reading. */
  role: string;
  description: string;
}

/**
 * One trait's description rendered against a specific character's display
 * values. Emitted for every trait the race declares — standard, alternate and
 * flag-granted — whether or not it is currently selected.
 */
export interface RenderedTraitDescriptionDto {
  key: string;
  name: string;
  /** The prose to show, rendered from the record's own `DESC:` tokens. */
  text: string;
  /**
   * `DESC:` arguments the engine could not resolve and therefore dropped.
   * Carried so an incomplete description is visibly incomplete, never guessed.
   */
  droppedArgs: string[];
  /** True when the held feats changed this sentence from its racial base. */
  movedByFeats: boolean;
}

export interface RaceSelectionResponse {
  raceId: string;
  raceKey: string;
  raceName: string;
  book: string;
  appliedTraits: AppliedTraitDto[];
  suppressions: SuppressionDto[];
  firedFlags: string[];
  /** Flags that fired but suppressed nothing and granted nothing. */
  inertFlags: string[];
  /** Selection keys that matched no alternate for this race. */
  unmatchedSelections: string[];
  /** Alternates the current selection locks out, per the PREMULT guard. */
  blockedAlternates: BlockedAlternateDto[];
  /** Selections that violate each other's guard. */
  conflictingSelections: BlockedAlternateDto[];
  /**
   * Every trait this race declares, its description rendered against the held
   * feats passed to the call. Sorted by key.
   */
  renderedTraitDescriptions: RenderedTraitDescriptionDto[];
  /**
   * The subset of the held feats that actually moved a display value, derived
   * by the engine one feat at a time. The screen's evidence for *why* a number
   * differs from the book's printed one.
   */
  displayValueFeats: string[];
  /** Non-empty only when the request could not be served. */
  errors: string[];
}

export async function loadAlternateRacialTraits(): Promise<AlternateRacialTraitsResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading alternate racial traits');
  }

  try {
    return await invoke<AlternateRacialTraitsResponse>('list_alternate_racial_traits');
  } catch (cause: unknown) {
    throw new Error(`Failed to load alternate racial traits: ${formatError(cause)}`);
  }
}

export async function resolveRaceAlternateSelection(
  raceKey: string,
  selectedAlternateKeys: readonly string[],
  heldFeats: readonly string[] = [],
): Promise<RaceSelectionResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for resolving an alternate racial trait selection');
  }

  try {
    return await invoke<RaceSelectionResponse>('resolve_race_alternate_selection', {
      request: { raceKey, selectedAlternateKeys: [...selectedAlternateKeys] },
      heldFeats: [...heldFeats],
    });
  } catch (cause: unknown) {
    throw new Error(`Failed to resolve alternate racial trait selection: ${formatError(cause)}`);
  }
}

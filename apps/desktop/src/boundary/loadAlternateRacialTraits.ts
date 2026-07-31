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

export interface AlternateRacialTraitsResponse {
  races: RacePickerDto[];
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
): Promise<RaceSelectionResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for resolving an alternate racial trait selection');
  }

  try {
    return await invoke<RaceSelectionResponse>('resolve_race_alternate_selection', {
      request: { raceKey, selectedAlternateKeys: [...selectedAlternateKeys] },
    });
  } catch (cause: unknown) {
    throw new Error(`Failed to resolve alternate racial trait selection: ${formatError(cause)}`);
  }
}

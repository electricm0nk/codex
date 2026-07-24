import type { CreateCharacterOutcome, DiagnosticDto } from '../boundary/loadCreateCharacter';
import type { LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';

/**
 * Maps a `CreateCharacterResponse`-shaped mutation outcome — the shape
 * every `mutate_saved_character` operation returns (`level_up_character`
 * today; `add_equipment_selection` / `add_spell_selection` in a follow-on
 * cycle) — into the character sheet's refreshed `detail`, or a user-facing
 * blocked message. Shared so every mutation caller computes the refresh
 * the same way.
 *
 * A `Blocked` outcome means nothing was persisted on disk; the real
 * diagnostics are surfaced verbatim in the message, never swallowed.
 */

export type CharacterMutationRefresh =
  | { kind: 'refreshed'; detail: LoadSavedCharacterResponse }
  | { kind: 'blocked'; message: string };

const FALLBACK_BLOCKED_MESSAGE = 'The requested change could not be applied — the recomputed build was not ready.';

/**
 * Shared with `purchaseEquipment`'s caller — `PurchaseEquipmentOutcome`'s
 * `Blocked` variant carries the exact same `diagnostics: DiagnosticDto[]`
 * shape as `CreateCharacterOutcome`'s, but isn't itself a
 * `CreateCharacterOutcome` (it has a `Purchased` tag, not `Saved`, plus a
 * `money` field `toCharacterMutationRefresh` knows nothing about), so it
 * can't be passed to `toCharacterMutationRefresh` directly.
 */
export function blockedMessageFromDiagnostics(diagnostics: DiagnosticDto[]): string {
  const messages = diagnostics.filter((diagnostic) => diagnostic.claimBlocking).map((diagnostic) => diagnostic.message);
  return messages.length > 0 ? messages.join(' ') : FALLBACK_BLOCKED_MESSAGE;
}

/**
 * `selectedFeats` isn't part of `CreateCharacterOutcome` (only
 * `load_saved_character` returns it, not the mutation commands), so callers
 * pass the value the refreshed `detail` should carry explicitly — the
 * previous known list carried forward unchanged for any mutation that
 * doesn't touch feats, or that list plus the newly added feat id for a feat
 * mutation. Never fabricated: either real prior data or a real just-applied
 * append, matching exactly what the backend mutation itself did.
 */
export function toCharacterMutationRefresh(
  outcome: CreateCharacterOutcome,
  selectedFeats: string[]
): CharacterMutationRefresh {
  if (outcome.kind === 'Blocked') {
    return { kind: 'blocked', message: blockedMessageFromDiagnostics(outcome.diagnostics) };
  }

  return {
    kind: 'refreshed',
    detail: {
      summary: outcome.summary,
      snapshot: outcome.snapshot,
      diagnostics: [],
      corpusDerived: outcome.corpusDerived,
      selectedFeats,
    },
  };
}

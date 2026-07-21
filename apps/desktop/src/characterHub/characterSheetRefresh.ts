import type { CreateCharacterOutcome } from '../boundary/loadCreateCharacter';
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

export function toCharacterMutationRefresh(outcome: CreateCharacterOutcome): CharacterMutationRefresh {
  if (outcome.kind === 'Blocked') {
    const messages = outcome.diagnostics.filter((diagnostic) => diagnostic.claimBlocking).map((diagnostic) => diagnostic.message);
    return { kind: 'blocked', message: messages.length > 0 ? messages.join(' ') : FALLBACK_BLOCKED_MESSAGE };
  }

  return {
    kind: 'refreshed',
    detail: {
      summary: outcome.summary,
      snapshot: outcome.snapshot,
      diagnostics: [],
      corpusDerived: outcome.corpusDerived,
    },
  };
}

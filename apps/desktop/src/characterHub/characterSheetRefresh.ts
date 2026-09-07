import type { CreateCharacterOutcome, DiagnosticDto } from '../boundary/loadCreateCharacter';
import type {
  ChosenFeatTargetsDto,
  LoadSavedCharacterResponse,
  SpellSelectionDto,
} from '../boundary/loadSavedCharacterDetail';

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
 * True exactly when a spell add for `classId` is a genuine Wizard bootstrap
 * -- this character's first-ever spell recorded under the Wizard class --
 * and therefore needs the atomic `recordAndPrepareSpellSelection` rather
 * than the cheaper single-mode `addSpellSelection`. Once one Wizard spell
 * already exists, `unmet_wizard_spellbook_conditions` (pilot_compute.rs)
 * already sees non-empty Known and Prepared sets, so every later Wizard
 * pick can take the plain path every other class already uses
 * (risks-and-open-questions.md item 9a).
 */
export function isWizardSpellBootstrap(
  existingSpells: SpellSelectionDto[],
  classId: string,
  wizardClassId: string
): boolean {
  return classId === wizardClassId && !existingSpells.some((selection) => selection.sourceClassId === wizardClassId);
}

/**
 * `selectedFeats`/`spellsSelected` aren't part of `CreateCharacterOutcome`
 * (only `load_saved_character` returns them, not the mutation commands), so
 * callers pass the value the refreshed `detail` should carry explicitly —
 * the previous known list carried forward unchanged for any mutation that
 * doesn't touch that list, or that list plus the newly applied entry/entries
 * for a mutation that does. Never fabricated: either real prior data or a
 * real just-applied append, matching exactly what the backend mutation
 * itself did.
 */
export function toCharacterMutationRefresh(
  outcome: CreateCharacterOutcome,
  selectedFeats: string[],
  spellsSelected: SpellSelectionDto[],
  // Defaulted so the many call sites that cannot change a feat's target
  // (equipment, modifiers, spells) stay unchanged. A caller that DID change
  // a chooser target must pass the updated list, exactly as it already must
  // for `selectedFeats`.
  chosenFeatTargets: ChosenFeatTargetsDto[] = [],
  // Same reasoning as `chosenFeatTargets`: no mutation command changes a
  // racial-trait choice, so every existing call site carries its current list
  // through unchanged and the default is the empty one.
  selectedAlternateTraitKeys: string[] = [],
  // AT-34-E4-002: same reasoning as `selectedAlternateTraitKeys` -- no
  // mutation command changes a character's trait selections, so every
  // existing call site carries its current list through unchanged and the
  // default is the empty one.
  selectedTraits: string[] = []
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
      // Carried through from the pre-mutation detail: a re-save changes no
      // racial-trait choice, and dropping them here would make the sheet
      // forget a swap the saved character still has.
      selectedAlternateTraitKeys,
      selectedTraits,
      selectedFeats,
      spellsSelected,
      chosenFeatTargets,
      // The mutation commands return a `CreateCharacterResponse`, which
      // carries neither of these. Left empty rather than guessed: a
      // mutation changes what the engine would explain (a level-up moves
      // every class-feature magnitude), so carrying the pre-mutation
      // records forward would show stale numbers as if they were current.
      // Callers that need the post-mutation records re-read the character
      // through `loadSavedCharacterDetail` — see `refreshEngineRecords` in
      // `CharacterSheet.tsx`, the same post-mutation re-read pattern
      // `refreshDurability` already established there.
      explanations: [],
      weaponDamage: [],
      // Same reasoning, one field further: a mutation that adds a feat really
      // does change what a racial trait's prose says (ARG's `Fortunate One`
      // moves halfling luck from "three times per day" to "4"), so the
      // pre-mutation resolution is stale the moment this returns. Absent
      // rather than stale; `refreshEngineRecords` re-reads it.
      resolvedRacialTraits: null,
    },
  };
}

import type { SpellSelectionDto } from '../boundary/loadSavedCharacterDetail';
import { isWizardSpellBootstrap } from './characterSheetRefresh';
import type { HeldClass } from './characterProgression';

/**
 * The two real decisions `CharacterSheet.tsx`'s `handleAddSpell` needs
 * before calling a spell-add mutation: which held class the pick is
 * attributed to, and whether that pick needs the atomic Wizard-bootstrap
 * command or the plain single-mode one. Extracted into a pure function so
 * both are unit-testable without a DOM (risks-and-open-questions.md item 25
 * — this module previously had no clean boundary to test against).
 */
export interface SpellRoutingDecision {
  /** Which held class this spell pick is attributed to. */
  primaryClassId: string;
  /**
   * True when this needs the atomic `recordAndPrepareSpellSelection` (a
   * genuine Wizard-spellbook bootstrap — see `isWizardSpellBootstrap`),
   * false for the plain `addSpellSelection` path every other pick uses.
   */
  useAtomicBootstrap: boolean;
}

/**
 * Prefers a held Wizard class over `heldClasses[0]` (so a Fighter/Wizard
 * multiclass build attributes a spell pick to Wizard, not whichever class
 * happens to be first in `classSummary`), then falls back to the first
 * held class for any non-Wizard build. `null` only for a character with no
 * held class at all — shouldn't happen for any real saved character, but
 * not assumed away.
 */
export function resolveSpellRouting(
  heldClasses: HeldClass[],
  existingSpells: SpellSelectionDto[],
  wizardClassId: string
): SpellRoutingDecision | null {
  const heldWizardClass = heldClasses.find((held) => held.classId === wizardClassId);
  const primaryClassId = heldWizardClass?.classId ?? heldClasses[0]?.classId;
  if (!primaryClassId) {
    return null;
  }
  return {
    primaryClassId,
    useAtomicBootstrap: isWizardSpellBootstrap(existingSpells, primaryClassId, wizardClassId),
  };
}

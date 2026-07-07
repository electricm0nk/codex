import type { CreateCharacterOutcome } from '../boundary/loadCreateCharacter';

export interface CreateCharacterOutcomeSurface {
  kind: 'saved' | 'blocked';
  headline: string;
  detail: string;
  highlights: Array<{ label: string; value: string }>;
  diagnosticMessages: string[];
}

function formatSigned(value: number): string {
  return value >= 0 ? `+${value}` : String(value);
}

/**
 * Maps a raw `Saved` / `Blocked` outcome to friendly copy. On `Blocked`, the
 * real diagnostic messages are surfaced verbatim underneath the summary line
 * — transparency, not a generic "something went wrong," consistent with the
 * engine's own no-fake-completion posture.
 */
export function buildCreateCharacterOutcomeSurface(
  outcome: CreateCharacterOutcome
): CreateCharacterOutcomeSurface {
  if (outcome.kind === 'Saved') {
    const { snapshot } = outcome;
    return {
      kind: 'saved',
      headline: `${outcome.summary.displayLabel} is ready`,
      detail: 'Your character was computed and saved.',
      highlights: [
        { label: 'Armor Class', value: String(snapshot.baselineArmorClass) },
        { label: 'Melee Attack Bonus', value: formatSigned(snapshot.baselineMeleeAttackBonus) },
        { label: 'Base Attack Bonus', value: formatSigned(snapshot.baseAttackBonus) },
        { label: 'Fortitude Save', value: formatSigned(snapshot.totalSaves.fortitude) },
        { label: 'Reflex Save', value: formatSigned(snapshot.totalSaves.reflex) },
        { label: 'Will Save', value: formatSigned(snapshot.totalSaves.will) },
      ],
      diagnosticMessages: [],
    };
  }

  return {
    kind: 'blocked',
    headline: "This build isn't ready yet",
    detail: 'The character engine could not fully compute this combination. Here is exactly what is still missing:',
    highlights: [],
    diagnosticMessages: outcome.diagnostics.map((diagnostic) => diagnostic.message),
  };
}

import type { CreateCharacterOutcome, DiagnosticDto } from '../boundary/loadCreateCharacter';
import type { ClassSupportLevel } from './characterHubModel';

/**
 * The 4 diagnostic ids the compute engine's generic Fighter-shaped fallback
 * path always emits, additively, for every class it has no dedicated seam
 * for. They carry no class-specific content, so they're suppressed from the
 * grouped view whenever real named diagnostics exist, and collapsed into a
 * single honest sentence when they're all there is.
 */
const GENERIC_DIAGNOSTIC_IDS = new Set([
  'class_chassis.unsupported',
  'combat.baseline_unsupported',
  'defense.total_save.unsupported',
  'skill.selected_modifier.unsupported',
]);

export interface CreateCharacterOutcomeContext {
  raceId: string;
  classId: string;
  classLabel: string;
  supportLevel: ClassSupportLevel;
}

export interface DiagnosticGroup {
  label: string;
  messages: string[];
}

export interface CreateCharacterOutcomeSurface {
  kind: 'saved' | 'blocked';
  headline: string;
  detail: string;
  highlights: Array<{ label: string; value: string }>;
  diagnosticGroups: DiagnosticGroup[];
  rawDiagnostics: DiagnosticDto[];
}

function formatSigned(value: number): string {
  return value >= 0 ? `+${value}` : String(value);
}

function groupLabelForDiagnosticId(id: string): string {
  if (id.startsWith('class_feature.')) return 'Class features not yet computed';
  if (id.startsWith('class_spell.')) return 'Spellcasting not yet computed';
  return 'Other diagnostics';
}

function groupNamedDiagnostics(named: DiagnosticDto[]): DiagnosticGroup[] {
  const order: string[] = [];
  const messagesByLabel = new Map<string, string[]>();
  for (const diagnostic of named) {
    const label = groupLabelForDiagnosticId(diagnostic.id);
    if (!messagesByLabel.has(label)) {
      messagesByLabel.set(label, []);
      order.push(label);
    }
    messagesByLabel.get(label)?.push(diagnostic.message);
  }
  return order.map((label) => ({ label, messages: messagesByLabel.get(label) ?? [] }));
}

function honestGenericOnlySentence(context: CreateCharacterOutcomeContext): string {
  if (context.supportLevel === 'partial-human-only' && context.raceId !== 'race:human') {
    return `${context.classLabel} support today only covers Human. Pick Human to see what's computed, or choose a different class.`;
  }
  // `human-diagnostics-only` classes never reach a savable build for any
  // race, including Human — Human only swaps in named diagnostics instead
  // of these 4 generic ones (see `verifiesBlockedOutcomeGroupsNamedDiagnostics`
  // in the test file, which exercises that named-diagnostics path directly).
  // This branch only fires for a non-Human pick, where even that detail is
  // unavailable — the message must not imply switching to Human would help.
  if (context.supportLevel === 'human-diagnostics-only') {
    return `${context.classLabel} isn't computed by the engine for any race yet — not even Human. Human only surfaces more specific diagnostics about what's missing; it won't produce a savable build either. Choose a different class for a build that saves today.`;
  }
  if (context.supportLevel === 'none') {
    return `${context.classLabel} isn't computed by the engine yet — no class-specific mechanics are implemented for it today.`;
  }
  return 'This build is blocked for an unexpected reason. See the technical diagnostic details below.';
}

/**
 * Maps a raw `Saved` / `Blocked` outcome to friendly copy. On `Blocked`, the
 * real diagnostics are surfaced verbatim (in `rawDiagnostics`) and, for the
 * headline narrative, grouped into `diagnosticGroups` — transparency, not a
 * generic "something went wrong," consistent with the engine's own
 * no-fake-completion posture.
 *
 * `map_diagnostics_dto` on the Rust side maps every diagnostic the engine
 * returns, not just claim-blocking ones — always-present non-blocking
 * diagnostics (e.g. race semantics notes) ride along in the same array as
 * the true blockers. `claimBlocking` must be filtered on before any id-based
 * classification, or those non-blocking entries get misclassified as
 * blocking content.
 */
export function buildCreateCharacterOutcomeSurface(
  outcome: CreateCharacterOutcome,
  context: CreateCharacterOutcomeContext
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
      diagnosticGroups: [],
      rawDiagnostics: [],
    };
  }

  const rawDiagnostics = outcome.diagnostics;
  const blocking = rawDiagnostics.filter((diagnostic) => diagnostic.claimBlocking);
  const named = blocking.filter((diagnostic) => !GENERIC_DIAGNOSTIC_IDS.has(diagnostic.id));

  const diagnosticGroups =
    named.length > 0
      ? groupNamedDiagnostics(named)
      : [{ label: "What's missing", messages: [honestGenericOnlySentence(context)] }];

  return {
    kind: 'blocked',
    headline: "This build isn't ready yet",
    detail: 'The character engine could not fully compute this combination. Here is exactly what is still missing:',
    highlights: [],
    diagnosticGroups,
    rawDiagnostics,
  };
}

import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';
import type { HeldClass } from './characterProgression';

/**
 * Projects the engine's own `class_feature.*` / `class_chassis.*`
 * explanation records into the sheet's Class Features section.
 *
 * This module authors **no rules data**. It filters, groups and labels the
 * records `load_saved_character` now carries across the IPC boundary, and
 * passes every magnitude and every word of derivation text through
 * untouched. It replaces the hand-authored class-feature table that
 * used to live in `characterProgression.ts` — bare labels like
 * `'Bravery +1'` with no magnitude and no provenance, standing in for 411
 * cited `class_feature.*` records.
 *
 * Two rules govern everything here:
 *
 * 1. **`detail` is rendered verbatim.** It is the engine's corpus citation.
 * 2. **Absence is rendered as absence.** A record whose id ends in
 *    `.unsupported` is the engine saying "this facet is not grounded", and
 *    its `value` is a filler zero, not a magnitude. Those records are
 *    separated into `notComputed` and rendered without a number, so the
 *    sheet never flattens `Blocked` into `0`.
 */

const CLASS_RECORD_PREFIXES = ['class_feature.', 'class_chassis.'] as const;
const UNSUPPORTED_SUFFIX = '.unsupported';

export interface ClassFeatureRow {
  /** The engine id, verbatim — the row's stable key and its audit handle. */
  id: string;
  /**
   * The held-class token this record belongs to (e.g. `'rogue'`), or
   * `null` for a record whose id carries no class segment (the
   * pre-namespacing `class_chassis.base_attack_bonus` family).
   *
   * Derived by matching the id's second segment against the character's
   * own held classes — not against a hand-maintained list of class names.
   */
  classToken: string | null;
  /** Humanised from the id's remaining segments. Never invented text. */
  label: string;
  /** The engine's computed magnitude, verbatim. */
  value: number;
  /** The engine's corpus-cited derivation, verbatim. */
  detail: string;
}

export interface ClassFeatureNotice {
  id: string;
  classToken: string | null;
  label: string;
  /** The engine's own explanation of why this facet is not grounded. */
  detail: string;
}

export interface ClassFeatureSurface {
  /** Grounded records, in the engine's own emission order. */
  features: ClassFeatureRow[];
  /**
   * `.unsupported` records — real "not computed here" notices. Carries no
   * `value` at all, precisely so no caller can render the filler zero as a
   * magnitude.
   */
  notComputed: ClassFeatureNotice[];
}

/** `'sneak_attack'` -> `'Sneak Attack'`; `'base_save.will'` -> `'Base Save Will'`. */
function humanise(segments: string[]): string {
  return segments
    .join(' ')
    .split(/[\s._]+/)
    .filter((word) => word.length > 0)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
}

/** `'class:rogue'` -> `'rogue'`. */
function classIdToken(classId: string): string {
  const parts = classId.split(':');
  return parts[parts.length - 1] ?? classId;
}

function isClassRecord(id: string): boolean {
  return CLASS_RECORD_PREFIXES.some((prefix) => id.startsWith(prefix));
}

/**
 * Splits an id into its owning class token (when the second segment names
 * one of the character's held classes) and the remaining, label-bearing
 * segments.
 *
 * Matching against the character's own held classes rather than a static
 * class-name list is deliberate: a static list would be one more piece of
 * hand-authored data to drift, and the character already tells us which
 * classes it holds.
 */
function splitId(id: string, heldTokens: Set<string>): { classToken: string | null; label: string } {
  const segments = id.split('.');
  const afterPrefix = segments.slice(1);
  const candidate = afterPrefix[0];
  if (candidate !== undefined && heldTokens.has(candidate) && afterPrefix.length > 1) {
    return { classToken: candidate, label: humanise(afterPrefix.slice(1)) };
  }
  return { classToken: null, label: humanise(afterPrefix) };
}

export function buildClassFeatureSurface(
  explanations: readonly ExplanationDto[],
  heldClasses: readonly HeldClass[]
): ClassFeatureSurface {
  const heldTokens = new Set(heldClasses.map((held) => classIdToken(held.classId)));
  const features: ClassFeatureRow[] = [];
  const notComputed: ClassFeatureNotice[] = [];

  for (const explanation of explanations) {
    if (!isClassRecord(explanation.id)) {
      continue;
    }
    if (explanation.id.endsWith(UNSUPPORTED_SUFFIX)) {
      const trimmedId = explanation.id.slice(0, -UNSUPPORTED_SUFFIX.length);
      const { classToken, label } = splitId(trimmedId, heldTokens);
      notComputed.push({ id: explanation.id, classToken, label, detail: explanation.detail });
      continue;
    }
    const { classToken, label } = splitId(explanation.id, heldTokens);
    features.push({
      id: explanation.id,
      classToken,
      label,
      value: explanation.value,
      detail: explanation.detail,
    });
  }

  return { features, notComputed };
}

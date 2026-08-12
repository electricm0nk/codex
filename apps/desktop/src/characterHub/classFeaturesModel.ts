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

/**
 * How many segments may sit between the record prefix and the class segment.
 *
 * The engine namespaces a book's records under its own id
 * (`class_feature.pu.unchained_summoner.bond_senses_rounds_per_day`), so the
 * class is not always the first segment. The scan is bounded to **one** such
 * segment rather than searching the whole id: an unbounded search would let a
 * class name appearing inside a *feature* name be mistaken for the owner, and
 * one is the depth the engine actually emits.
 */
const MAX_NAMESPACE_SEGMENTS = 1;

/**
 * Segments that name a record *family* rather than the feature itself.
 *
 * `class_feature.pu.<class>.corpus_record.<slug>` is the engine's per-record
 * roster row: `corpus_record` says which kind of row this is, and putting it in
 * front of every Pathfinder Unchained feature name ("Corpus Record Maker S
 * Call") describes the id rather than the rules. It is dropped from the label
 * and nowhere else — the id itself still crosses verbatim as the row's key.
 */
const RECORD_FAMILY_SEGMENTS = new Set(['corpus_record']);

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
  /**
   * The held class's own display label (`'Unchained Summoner'`), or `null`
   * when no held class owns this record.
   *
   * Carried alongside `classToken` because the token is an id segment and
   * reads like one on screen (`unchained_summoner`); the label is the name the
   * character already calls that class, taken from the same `HeldClass` the
   * token was matched against, so the two can never name different classes.
   */
  classLabel: string | null;
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
  /** See [`ClassFeatureRow.classLabel`]. */
  classLabel: string | null;
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

/** Drops a leading record-family segment, when dropping it leaves a label. */
function stripRecordFamily(segments: string[]): string[] {
  const first = segments[0];
  if (first !== undefined && RECORD_FAMILY_SEGMENTS.has(first) && segments.length > 1) {
    return segments.slice(1);
  }
  return segments;
}

/**
 * Splits an id into its owning class token (when one of its leading segments
 * names one of the character's held classes) and the remaining, label-bearing
 * segments.
 *
 * Matching against the character's own held classes rather than a static
 * class-name list is deliberate: a static list would be one more piece of
 * hand-authored data to drift, and the character already tells us which
 * classes it holds.
 *
 * The class is looked for at the first segment **and** one segment further in,
 * because the engine namespaces a book's records under the book id — every
 * Pathfinder Unchained record is `class_feature.pu.<class>.…`. Before that,
 * `pu` was tested against the held classes, never matched, and the whole
 * remainder became the label: 30-plus rows per character reading
 * `Pu Unchained Summoner Bond Senses Rounds Per Day` under a `Chassis` gutter.
 * See [`MAX_NAMESPACE_SEGMENTS`] for why the scan is bounded rather than
 * exhaustive.
 */
function splitId(id: string, heldTokens: Set<string>): { classToken: string | null; label: string } {
  const segments = id.split('.');
  const afterPrefix = segments.slice(1);
  for (let index = 0; index <= MAX_NAMESPACE_SEGMENTS && index < afterPrefix.length - 1; index += 1) {
    const candidate = afterPrefix[index];
    if (candidate !== undefined && heldTokens.has(candidate)) {
      return {
        classToken: candidate,
        label: humanise(stripRecordFamily(afterPrefix.slice(index + 1))),
      };
    }
  }
  return { classToken: null, label: humanise(afterPrefix) };
}

export function buildClassFeatureSurface(
  explanations: readonly ExplanationDto[],
  heldClasses: readonly HeldClass[]
): ClassFeatureSurface {
  // Token -> the held class's own label, so an attributed row can render the
  // name the character already uses instead of the raw id segment.
  const heldLabels = new Map(heldClasses.map((held) => [classIdToken(held.classId), held.classLabel]));
  const heldTokens = new Set(heldLabels.keys());
  const features: ClassFeatureRow[] = [];
  const notComputed: ClassFeatureNotice[] = [];

  const labelFor = (classToken: string | null): string | null =>
    classToken === null ? null : heldLabels.get(classToken) ?? null;

  for (const explanation of explanations) {
    if (!isClassRecord(explanation.id)) {
      continue;
    }
    if (explanation.id.endsWith(UNSUPPORTED_SUFFIX)) {
      const trimmedId = explanation.id.slice(0, -UNSUPPORTED_SUFFIX.length);
      const { classToken, label } = splitId(trimmedId, heldTokens);
      notComputed.push({
        id: explanation.id,
        classToken,
        classLabel: labelFor(classToken),
        label,
        detail: explanation.detail,
      });
      continue;
    }
    const { classToken, label } = splitId(explanation.id, heldTokens);
    features.push({
      id: explanation.id,
      classToken,
      classLabel: labelFor(classToken),
      label,
      value: explanation.value,
      detail: explanation.detail,
    });
  }

  return { features, notComputed };
}

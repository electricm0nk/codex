import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';
import type { ClassFeatureDescriptionDto } from '../boundary/loadClassFeatureDescriptions';
import type { HeldClass } from './characterProgression';
import { normalizeFeatIdentity } from './featsTabModel';

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
  /**
   * The real rulebook `DESC:` text for this feature, or `null`.
   *
   * SD31-D7-PROSE-003. A SEPARATE field from `detail`: `detail` is the
   * engine's own computed derivation (may be a bare magnitude with no prose
   * at all), this is the corpus row's actual rules text, joined via
   * {@link matchesCorpusFeature}. `null` when no `classToken`-scoped
   * description candidate's `(classSlug, featureSlug)` matches this row's
   * own `id` — never a guess, never the nearest match.
   */
  corpusDescription: string | null;
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

/**
 * Whether a `ClassFeatureDescriptionDto` candidate is the SAME record a
 * class-feature explanation id names.
 *
 * Reuses, verbatim, the join `v06_work_inventory.rs`'s `Kind::ClassFeature`
 * classify arm already trusts to decide `grounded` evidence:
 * `id.contains(".{owner}.") && id.ends_with(&feature_slug)`. This is
 * DELIBERATE reuse, not a second invented rule — the frontend's own join can
 * never be more permissive than the join the board's own doneness
 * measurement already relies on. `endsWith`, never exact equality: some ids
 * carry an extra `corpus_record` segment ahead of the feature slug
 * (`RECORD_FAMILY_SEGMENTS`).
 */
export function matchesCorpusFeature(
  id: string,
  classSlug: string,
  featureSlug: string
): boolean {
  return id.includes(`.${classSlug}.`) && id.endsWith(featureSlug);
}

/**
 * Looks up the corpus description for one class-feature row, or `null` when
 * no candidate matches — a near-miss is refused, never guessed.
 *
 * `classToken === null` (the pre-namespacing `class_chassis.*` family, which
 * names no class at all) always refuses: there is no `classSlug` to match
 * against, and matching by `featureSlug` alone would be exactly the
 * shared-NAME hazard `decisions.md §10`'s first guard exists to prevent
 * (`corpus_record ~ ...` names collide far more than `(class, feature)`
 * pairs do).
 */
function findCorpusDescription(
  id: string,
  classToken: string | null,
  descriptions: readonly ClassFeatureDescriptionDto[]
): string | null {
  if (classToken === null) {
    return null;
  }
  const match = descriptions.find(
    (d) => d.classSlug === classToken && matchesCorpusFeature(id, d.classSlug, d.featureSlug)
  );
  return match?.description ?? null;
}

/**
 * Every real corpus `class_feature` description for a class the character
 * holds that {@link buildClassFeatureSurface} does NOT already attach to a
 * grounded row (T4 / `epic-breakdown.md` Epic 2, "built-but-unreachable
 * render surface").
 *
 * `class_feature_descriptions.rs` (and the disjoint feat-bridge population,
 * `class_feature_feat_bridge.rs`, concatenated by the caller into the same
 * `descriptions` array) transcribe real, PI-screened, leak-checked corpus
 * text into `ClassFeatureDescriptionDto` — but `buildClassFeatureSurface`
 * only ever attaches `corpusDescription` as enrichment onto a row an
 * `ExplanationDto` already created (`findCorpusDescription`, above). A corpus
 * feature the engine emits no explanation for — grounded or `.unsupported`
 * — never reaches `features`/`notComputed` at all, so its real rulebook text
 * is never shown anywhere, no matter how many such records exist. This is
 * the cause T4 names: content exists and is fully verified, but no code path
 * ever puts it on screen.
 *
 * The fix is a browsable REFERENCE list, the same shape
 * `ClassFeaturePoolReferenceSection` (`CharacterSheet.tsx`) already uses for
 * option-pool members: every real description for a class the character
 * holds, shown independent of whether the engine also emits a computed row
 * for it. **Only grounded (non-`.unsupported`) explanation ids count as
 * "already shown"** — a `.unsupported` notice never carries a
 * `corpusDescription` either (`buildClassFeatureSurface` only sets it inside
 * the `features` loop), so a description matching only an `.unsupported` id
 * is still unreachable today and must still appear here, not be excluded as
 * a false duplicate.
 *
 * **T4-L9 (`decisions.md §13`) — a second, feat-held reachability arm.**
 * `class_feature_feat_bridge.rs`'s own 471-record population names a
 * synthetic pool-group `classSlug` (e.g. `"golden_legionnaire"`), never a
 * real class token, so the class-held check above (`heldTokens.has(d.
 * classSlug)`) can never match any of them — confirmed corpus-wide by that
 * module's own test that only 1 of 471 group slugs is even a holdable class
 * token. Every one of those records instead carries `grantedFeat`: the
 * exact, already-verified feat name the record's sole content grants (see
 * `ClassFeatureDescriptionDto.grantedFeat`'s own doc comment). This function
 * gates those records on the character holding THAT FEAT — reusing
 * `normalizeFeatIdentity`, the SAME fold `feat_identity.rs`'s `holds()`
 * mirrors on the Rust side (that module's own doc comment names the
 * pairing), not a second invented comparison. **Closed by class, not by
 * instance**: this is a predicate over `grantedFeat` presence, not a
 * hand-listed set of the 471 keys — any future bridge record the Rust side
 * emits is covered automatically, with no per-record entry here.
 */
export function unmatchedClassFeatureDescriptions(
  explanations: readonly ExplanationDto[],
  heldClasses: readonly HeldClass[],
  descriptions: readonly ClassFeatureDescriptionDto[],
  selectedFeats: readonly string[] = []
): ClassFeatureDescriptionDto[] {
  const heldTokens = new Set(heldClasses.map((held) => classIdToken(held.classId)));
  const heldFeatIdentities = new Set(selectedFeats.map((feat) => normalizeFeatIdentity(feat)));
  const isReachableByHeldCause = (d: ClassFeatureDescriptionDto): boolean =>
    d.grantedFeat !== null
      ? heldFeatIdentities.has(normalizeFeatIdentity(d.grantedFeat))
      : heldTokens.has(d.classSlug);
  const heldDescriptions = descriptions.filter(isReachableByHeldCause);
  const groundedIds = explanations
    .filter((e) => isClassRecord(e.id) && !e.id.endsWith(UNSUPPORTED_SUFFIX))
    .map((e) => e.id);
  return heldDescriptions.filter(
    (d) => !groundedIds.some((id) => matchesCorpusFeature(id, d.classSlug, d.featureSlug))
  );
}

export function buildClassFeatureSurface(
  explanations: readonly ExplanationDto[],
  heldClasses: readonly HeldClass[],
  descriptions: readonly ClassFeatureDescriptionDto[] = []
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
      corpusDescription: findCorpusDescription(explanation.id, classToken, descriptions),
    });
  }

  return { features, notComputed };
}

import type { FeatCatalogEntryDto } from '../boundary/listFeats';

/**
 * What a feat mutation changed about eligibility (v0.8 G-3), as the
 * difference between two `list_feats_for_character` answers. The engine
 * decides eligibility each time; this only compares its two verdicts.
 * A feat with no verdict on either side is never reported — reporting
 * it would be a guess.
 */
export interface EligibilityDelta {
  nowEligible: string[];
  noLongerEligible: string[];
}

export function eligibilityDelta(
  before: readonly FeatCatalogEntryDto[],
  after: readonly FeatCatalogEntryDto[],
  mutatedFeatKey: string,
): EligibilityDelta {
  const verdictBefore = new Map(before.map((entry) => [entry.key, entry.eligibility?.eligible]));
  const nowEligible: string[] = [];
  const noLongerEligible: string[] = [];
  for (const entry of after) {
    if (entry.key === mutatedFeatKey) {
      continue;
    }
    const was = verdictBefore.get(entry.key);
    const is = entry.eligibility?.eligible;
    if (was === undefined || is === undefined) {
      continue;
    }
    if (is && !was) {
      nowEligible.push(entry.name);
    } else if (was && !is) {
      noLongerEligible.push(entry.name);
    }
  }
  return { nowEligible, noLongerEligible };
}

/** One status line, or `null` when nothing changed (silence beats noise). */
export function describeEligibilityDelta(delta: EligibilityDelta): string | null {
  const parts: string[] = [];
  if (delta.nowEligible.length > 0) {
    parts.push(`Now eligible: ${delta.nowEligible.join(', ')}.`);
  }
  if (delta.noLongerEligible.length > 0) {
    parts.push(`No longer eligible: ${delta.noLongerEligible.join(', ')}.`);
  }
  return parts.length === 0 ? null : parts.join(' ');
}

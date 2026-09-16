/**
 * Presentation of a `rate_encounter` response (v0.8 E-2).
 *
 * Nothing here rates anything. Every number is copied from the engine's
 * response (`encounter_rating.rs`); this module decides only how it is
 * written and what stands next to it. In particular `crAsRated` is read
 * back from the engine — the CR rounding rule exists nowhere in this
 * layer — and `elMinusApl` is the engine's own field, never EL − APL
 * recomputed here.
 *
 * The two disclosures this screen exists for:
 *   1. A rating with any monster outside the engine's verified XP table
 *      is marked unverified in the headline itself, and each such monster
 *      carries the engine's reason.
 *   2. The four-tier scale is a collapse of the rulebook's five; the
 *      collapse is stated wherever the tier is shown.
 */

/** Mirrors `RateEncounterResponse` in `encounter_rating.rs` (the stable, response half). */
export interface RatedMonsterView {
  catalogKey?: string;
  name?: string;
  challengeRating: number;
  crAsRated: number;
  outsideVerifiedRange: boolean;
  reason?: string;
}
export interface RateEncounterResponseView {
  difficulty: string;
  averagePartyLevel: number;
  encounterLevel: number;
  elMinusApl: number;
  monsters: RatedMonsterView[];
  anyOutsideVerifiedRange: boolean;
  verifiedCrRange: [number, number];
  difficultyScale: { tiers: number; collapsedFrom: number; note: string };
  caveats: string[];
}

export interface EncounterView {
  tierLabel: string;
  tierConfidence: 'verified' | 'unverified';
  headline: string;
  apl: string;
  el: string;
  elMinusApl: string;
  monsters: Array<{ name: string; crShown: string; flag: string | null }>;
  verifiedRangeLabel: string;
  scaleNote: string;
  caveats: string[];
}

/** The engine's tier string, glossed with the rulebook tier(s) it stands for. Unknown strings pass through verbatim. */
export function describeTier(difficulty: string): string {
  switch (difficulty) {
    case 'easy':
      return 'Easy';
    case 'medium':
      return 'Medium (rulebook Average)';
    case 'hard':
      return 'Hard (rulebook Challenging or Hard — merged)';
    case 'deadly':
      return 'Deadly (rulebook Epic)';
    default:
      return difficulty;
  }
}

const KNOWN_TIERS: Record<string, string> = { easy: 'Easy', medium: 'Medium', hard: 'Hard', deadly: 'Deadly' };

/** Writes a corpus CR the way the rulebook prints it (1/8, 1/4, 1/3, 1/2, else the number). Display only. */
export function formatCr(cr: number): string {
  const fractions: Array<[number, string]> = [
    [1 / 8, '1/8'],
    [1 / 6, '1/6'],
    [1 / 4, '1/4'],
    [1 / 3, '1/3'],
    [1 / 2, '1/2'],
  ];
  const match = fractions.find(([value]) => Math.abs(value - cr) < 0.01);
  return match ? match[1] : String(cr);
}

function signed(value: number): string {
  return value < 0 ? String(value) : `+${value}`;
}

export function buildEncounterView(response: RateEncounterResponseView): EncounterView {
  const tierLabel = KNOWN_TIERS[response.difficulty] ?? response.difficulty;
  const tierConfidence = response.anyOutsideVerifiedRange ? 'unverified' : 'verified';
  const headline =
    tierConfidence === 'unverified'
      ? `${tierLabel} — unverified: one or more creatures are outside the engine's verified CR ${response.verifiedCrRange[0]}–${response.verifiedCrRange[1]} table`
      : tierLabel;
  return {
    tierLabel,
    tierConfidence,
    headline,
    apl: String(response.averagePartyLevel),
    el: String(response.encounterLevel),
    elMinusApl: signed(response.elMinusApl),
    monsters: response.monsters.map((monster) => {
      const corpus = formatCr(monster.challengeRating);
      const rerated = Math.abs(monster.challengeRating - monster.crAsRated) >= 0.01;
      return {
        name: monster.name ?? monster.catalogKey ?? `CR ${corpus} creature`,
        crShown: rerated ? `${corpus} → rated as ${monster.crAsRated}` : corpus,
        flag: monster.outsideVerifiedRange ? (monster.reason ?? 'Outside the verified CR table.') : null,
      };
    }),
    verifiedRangeLabel: `CR ${response.verifiedCrRange[0]}–${response.verifiedCrRange[1]}`,
    scaleNote: `${response.difficultyScale.tiers} tiers, collapsed from the rulebook's ${response.difficultyScale.collapsedFrom}. ${response.difficultyScale.note}`,
    caveats: [...response.caveats],
  };
}

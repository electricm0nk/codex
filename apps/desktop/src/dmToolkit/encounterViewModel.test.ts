import { buildEncounterView, describeTier, type RateEncounterResponseView } from './encounterViewModel';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * v0.8 E-2: how an engine rating is presented. Every number and every
 * caveat here is copied from the `rate_encounter` response — this module
 * decides only what is shown next to what, and pins the two honesty rules
 * that are the screen's whole job:
 *   1. the tier never appears alone and confident when any monster's CR
 *      is outside the engine's verified table, and the offending monsters
 *      are marked individually;
 *   2. the four-tier scale is disclosed as a collapse of the rulebook's
 *      five wherever the tier is shown.
 */
const base: RateEncounterResponseView = {
  difficulty: 'deadly',
  averagePartyLevel: 1,
  encounterLevel: 4,
  elMinusApl: 3,
  monsters: [
    { catalogKey: 'b1:monster:dire_rat', name: 'Dire rat', challengeRating: 0.333, crAsRated: 1, outsideVerifiedRange: true, reason: 'CR 1/3 rounds to 1; the verified XP table starts at CR 1.' },
    { catalogKey: 'b1:monster:ogre', name: 'Ogre', challengeRating: 3, crAsRated: 3, outsideVerifiedRange: false },
  ],
  anyOutsideVerifiedRange: true,
  verifiedCrRange: [1, 10],
  difficultyScale: { tiers: 4, collapsedFrom: 5, note: 'Challenging and Hard are merged into Hard.' },
  caveats: ['One or more CRs fall outside the verified table.'],
};

const view = buildEncounterView(base);
assertEqual(view.tierLabel, 'Deadly', 'tier is the engine string, capitalised for display only');
assertEqual(view.tierConfidence, 'unverified', 'anyOutsideVerifiedRange → the tier is marked unverified');
assert(view.headline.includes('Deadly') && view.headline.includes('unverified'), 'the headline itself carries the caveat, not a footnote');
assertEqual(view.apl, '1', 'APL verbatim');
assertEqual(view.el, '4', 'EL verbatim');
assertEqual(view.elMinusApl, '+3', 'EL−APL is the engine field, signed, never recomputed');
assertEqual(view.monsters[0].flag, 'CR 1/3 rounds to 1; the verified XP table starts at CR 1.', 'the offending monster carries the engine reason');
assertEqual(view.monsters[0].crShown, '1/3 → rated as 1', 'shows the corpus CR and what the engine rated it as');
assertEqual(view.monsters[1].flag, null, 'a verified monster carries no flag');
assertEqual(view.monsters[1].crShown, '3', 'a verified CR shows plainly');
assertEqual(view.verifiedRangeLabel, 'CR 1–10', 'verified range from the response');
assert(view.scaleNote.includes('4') && view.scaleNote.includes('5') && view.scaleNote.includes('Challenging and Hard are merged into Hard.'), 'tier collapse disclosed with the engine note');
assertEqual(view.caveats.length, 1, 'engine caveats passed through verbatim');

const clean = buildEncounterView({ ...base, difficulty: 'medium', anyOutsideVerifiedRange: false, caveats: [], monsters: [base.monsters[1]] });
assertEqual(clean.tierConfidence, 'verified', 'all CRs inside the range → verified');
assert(!clean.headline.includes('unverified'), 'no false alarm on a clean rating');
assert(clean.scaleNote.length > 0, 'the tier-collapse disclosure is shown even on a clean rating');

assertEqual(describeTier('hard'), 'Hard (rulebook Challenging or Hard — merged)', 'the merged tier says so');
assertEqual(describeTier('medium'), 'Medium (rulebook Average)', 'renamed tiers name the rulebook tier');
assertEqual(describeTier('deadly'), 'Deadly (rulebook Epic)', 'renamed tiers name the rulebook tier');
assertEqual(describeTier('easy'), 'Easy', 'a tier that aligns 1:1 needs no gloss');
assertEqual(describeTier('weird'), 'weird', 'an unknown engine string is shown verbatim, never mapped');

assertEqual(buildEncounterView({ ...base, elMinusApl: -2 }).elMinusApl, '-2', 'negative EL−APL keeps its sign');
assertEqual(buildEncounterView({ ...base, monsters: [{ ...base.monsters[0], challengeRating: 0.5 }] }).monsters[0].crShown, '1/2 → rated as 1', 'halves are written as fractions');
assertEqual(buildEncounterView({ ...base, monsters: [{ ...base.monsters[1], challengeRating: 12, crAsRated: 12, outsideVerifiedRange: true, reason: 'x' }] }).monsters[0].crShown, '12', 'an above-range CR that was not re-rated shows once');

console.log('encounterViewModel tests passed');

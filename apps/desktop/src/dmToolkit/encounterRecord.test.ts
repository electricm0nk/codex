import { formatEncounterRecord } from './encounterRecord';
import type { RateEncounterResponse } from '../boundary/rateEncounter';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * v0.8 G-4: a rated encounter saved onto a Scene is the ENGINE'S ANSWER
 * written down — tier with its rulebook gloss, APL/EL/EL−APL, the party
 * as rated, every monster with the CR it was rated at, and every caveat.
 * Nothing is re-rated on save or export; a saved "Deadly" can never appear
 * without the disclosure the builder showed beside it.
 */
const response: RateEncounterResponse = {
  difficulty: 'deadly',
  averagePartyLevel: 1,
  encounterLevel: 4,
  elMinusApl: 3,
  party: [{ characterId: 'c1', level: 1 }, { level: 2 }],
  monsters: [
    { catalogKey: 'b1:monster:dire_rat', name: 'Dire rat', challengeRating: 0.333, crAsRated: 1, outsideVerifiedRange: true, reason: 'CR 1/3 rounds to 1; the verified table starts at CR 1.' },
    { catalogKey: 'b1:monster:dire_rat', name: 'Dire rat', challengeRating: 0.333, crAsRated: 1, outsideVerifiedRange: true, reason: 'CR 1/3 rounds to 1; the verified table starts at CR 1.' },
    { catalogKey: 'b1:monster:ogre', name: 'Ogre', challengeRating: 3, crAsRated: 3, outsideVerifiedRange: false },
  ],
  anyOutsideVerifiedRange: true,
  verifiedCrRange: [1, 10],
  difficultyScale: { tiers: 4, collapsedFrom: 5, note: 'Challenging and Hard are merged into Hard.' },
  caveats: ['One or more CRs fall outside the verified table.'],
};
const text = formatEncounterRecord(response, { c1: 'Thrain' }, '2026-09-02T10:00:00Z');

assert(text.startsWith('Difficulty: Deadly (rulebook Epic) — UNVERIFIED'), 'the first line carries the tier, its gloss and the unverified mark together');
assert(text.includes('outside the engine’s verified CR 1–10 table'), 'the disclosure names the range');
assert(text.includes('APL 1 · EL 4 · EL−APL +3'), 'the three engine numbers, verbatim');
assert(text.includes('Party: Thrain (level 1), level 2'), 'party as rated, saved characters by name with the engine-resolved level');
assert(text.includes('2 × Dire rat — CR 1/3, rated as CR 1 ⚠ CR 1/3 rounds to 1'), 'monsters grouped, with corpus CR, rated CR and the engine reason');
assert(text.includes('1 × Ogre — CR 3'), 'a verified monster shows plainly');
assert(text.includes('Scale: 4 tiers, collapsed from the rulebook’s 5. Challenging and Hard are merged into Hard.'), 'tier collapse disclosed');
assert(text.includes('One or more CRs fall outside the verified table.'), 'engine caveats copied');
assert(text.includes('Rated 2026-09-02T10:00:00Z by the engine; not re-rated on export.'), 'provenance line');

const clean = formatEncounterRecord({ ...response, difficulty: 'medium', anyOutsideVerifiedRange: false, caveats: [], monsters: [response.monsters[2]] }, {}, 'now');
assert(clean.startsWith('Difficulty: Medium (rulebook Average)') && !clean.includes('UNVERIFIED'), 'a clean rating has no false alarm');
assert(clean.includes('every creature inside the engine’s verified CR 1–10 table'), 'and states its verification positively');
assert(clean.includes('level 1, level 2') === false && clean.includes('c1 (level 1)'), 'an unknown saved-character id falls back to the id, never dropped');

console.log('encounterRecord tests passed');

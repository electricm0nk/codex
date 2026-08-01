import { deriveRaceFacets, describeRaceCatalog, raceLabel, RACE_LABEL_OVERRIDES } from './RaceCatalogScreen';
import type { RaceCatalogEntryDto } from '../boundary/loadRaceCatalog';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The Race Traits browser must describe the races the adapter actually
 * served, not a roster compiled into the frontend.
 *
 * The screen used to hardcode a seven-entry `RACE_ORDER`/`RACE_LABELS` pair
 * and print the literal sentence "trait rows across all 7 CRB races". That
 * is the same defect class as `itemPickerFilter`'s book map, which mapped
 * only Crb/Apg/Acg and so showed 204 of 690 feat rows under a raw wire
 * code: a frontend-side roster silently drops, and then actively
 * misdescribes, anything the backend starts serving beyond it.
 *
 * SD-27 widens race coverage from the 7 Core Rulebook races to 18 across
 * two books (the 7 CRB races plus 11 from Bestiary 1: aasimar, drow,
 * duergar, goblin, hobgoblin, kobold, merfolk, orc, svirfneblin, tengu,
 * tiefling — the corpus directories `data/corpus/core_rulebook/race/` and
 * `data/corpus/beastiary/race/` hold exactly those 7 and 11 files). With
 * the old hardcoded roster those 11 races would have had no filter button
 * and the screen would have kept claiming "7 CRB races" while rendering
 * rows from two books.
 */

function entry(raceId: string, traitName: string): RaceCatalogEntryDto {
  // `book` is required on the wire (`race_catalog.rs` tags every row); these
  // helpers exercise the roster-derivation logic, which reads only `raceId`.
  const book = CRB_RACES.includes(raceId as (typeof CRB_RACES)[number]) ? 'CRB' : 'B1';
  return { raceId, traitName, value: 0, detail: `${traitName} detail`, book };
}

/** The 7 CRB races, in the order `race_traits()` groups them (RaceId::ALL). */
const CRB_RACES = ['Human', 'Dwarf', 'Elf', 'Gnome', 'HalfElf', 'HalfOrc', 'Halfling'] as const;

/** The 11 Bestiary 1 races in SD-27 scope, per `data/corpus/beastiary/race/`. */
const B1_RACES = [
  'Aasimar',
  'Drow',
  'Duergar',
  'Goblin',
  'Hobgoblin',
  'Kobold',
  'Merfolk',
  'Orc',
  'Svirfneblin',
  'Tengu',
  'Tiefling',
] as const;

function testFacetsCoverEveryRaceTheAdapterServed() {
  const entries = [...CRB_RACES, ...B1_RACES].flatMap((raceId) => [
    entry(raceId, 'Size'),
    entry(raceId, 'Speed'),
  ]);

  const facets = deriveRaceFacets(entries);

  assertEqual(facets.length, 18, 'every served race gets a facet, not just the 7 compiled-in ones');
  assertEqual(
    facets.map((facet) => facet.raceId).join(','),
    [...CRB_RACES, ...B1_RACES].join(','),
    'facets follow the order the adapter served the rows in'
  );
  for (const facet of facets) {
    assertEqual(facet.count, 2, `${facet.raceId} counts its own rows`);
  }
}

function testFacetsAreEmptyWhenNothingLoaded() {
  assertEqual(deriveRaceFacets([]).length, 0, 'no rows means no race buttons to claim');
}

function testLabelsNeverFabricateANameForAnUnknownRace() {
  assertEqual(raceLabel('HalfElf'), 'Half-Elf', 'the hyphenated CRB variants get their real names');
  assertEqual(raceLabel('HalfOrc'), 'Half-Orc', 'the hyphenated CRB variants get their real names');
  for (const raceId of B1_RACES) {
    assertEqual(
      raceLabel(raceId),
      raceId,
      `${raceId} falls back to its own variant name rather than an invented label`
    );
  }
  assertEqual(raceLabel('Svirfneblin'), 'Svirfneblin', 'a single-word variant name is already the display name');
}

function testOverridesOnlyCoverVariantsWhoseNameIsNotTheirLabel() {
  for (const [raceId, label] of Object.entries(RACE_LABEL_OVERRIDES)) {
    assert(label !== raceId, `${raceId}'s override is a different string than the raw variant`);
    assert(label.length > 0, `${raceId}'s override is a real label`);
  }
}

function testSummaryCountsOnlyWhatLoaded() {
  const crbOnly = CRB_RACES.flatMap((raceId) => [entry(raceId, 'Size')]);
  assertEqual(
    describeRaceCatalog(crbOnly),
    '7 trait rows across 7 races',
    'the summary is derived from the rows, so a CRB-only catalog reads 7'
  );

  const bothBooks = [...CRB_RACES, ...B1_RACES].flatMap((raceId) => [entry(raceId, 'Size'), entry(raceId, 'Speed')]);
  assertEqual(
    describeRaceCatalog(bothBooks),
    '36 trait rows across 18 races',
    'a widened catalog is described at its real width, with no book claim the DTO cannot back'
  );
}

function testSummaryNeverNamesABookTheDtoDoesNotCarry() {
  const text = describeRaceCatalog([entry('Human', 'Size')]);
  assert(!/CRB|Core Rulebook|Bestiary/i.test(text), 'the race DTO has no book field, so the summary names no book');
  assertEqual(describeRaceCatalog([entry('Human', 'Size')]), '1 trait row across 1 race', 'singular reads correctly');
}

function testSummaryIsHonestAboutAnEmptyCatalog() {
  assertEqual(describeRaceCatalog([]), '0 trait rows across 0 races', 'an empty catalog claims nothing');
}

testFacetsCoverEveryRaceTheAdapterServed();
testFacetsAreEmptyWhenNothingLoaded();
testLabelsNeverFabricateANameForAnUnknownRace();
testOverridesOnlyCoverVariantsWhoseNameIsNotTheirLabel();
testSummaryCountsOnlyWhatLoaded();
testSummaryNeverNamesABookTheDtoDoesNotCarry();
testSummaryIsHonestAboutAnEmptyCatalog();

console.log('RaceCatalogScreen.test.ts: all assertions passed');

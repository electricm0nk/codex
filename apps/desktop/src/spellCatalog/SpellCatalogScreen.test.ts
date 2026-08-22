import { BOOK_LABELS, BOOK_ORDER, formatBookList } from './SpellCatalogScreen';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The catalog screen's book surface must cover every book the Rust adapter
 * actually serves. `spell_catalog.rs`'s `build_spell_catalog` chains
 * CRB -> APG -> ACG -> ARG -> UI, and its pinned test
 * `the_catalog_serves_every_ingested_book_not_only_crb` asserts
 * 652 + 297 + 144 + 92 + 101 = 1286. A code the adapter serves but this
 * screen omits has no filter button and renders its badge under a raw wire
 * code, so the book's records are reachable only under "All books" and are
 * labelled with a code no player-facing text explains.
 *
 * **Why the UI entry is here, and the lesson it carries (SD-29 Epic 4,
 * spell lane).** Ultimate Intrigue joined the served chain when its 101
 * spells were ingested; this file was not updated, and neither was
 * `BOOK_ORDER`. Every test below still passed, because
 * `CHAINED_BOOK_CODES` was a *copy* of the list under test rather than an
 * independent statement of what the backend chains — so the oracle drifted
 * in lockstep with the defect it existed to catch. The screen shipped
 * reading "1286 spells across the Core Rulebook, Advanced Player's Guide,
 * Advanced Class Guide and Advanced Race Guide" above four chips summing
 * to 1185, with 101 spells present in the list and nameable by nothing.
 * Caught by looking at the running screen, not by any test. When adding a
 * book here, derive this constant from `spell_catalog.rs`'s chain, not
 * from `BOOK_ORDER`.
 */

/** The wire codes `build_spell_catalog` chains, in that order. */
const CHAINED_BOOK_CODES = ['CRB', 'APG', 'ACG', 'ARG', 'UI', 'UM', 'OA', 'UC', 'ISG', 'UW'] as const;

function testBookOrderCoversEveryServedBookInChainOrder() {
  assertEqual(
    BOOK_ORDER.join(','),
    CHAINED_BOOK_CODES.join(','),
    'BOOK_ORDER matches the Rust adapter chain order'
  );
}

function testEveryOrderedBookHasARealDisplayLabel() {
  for (const code of BOOK_ORDER) {
    const label = BOOK_LABELS[code];
    assert(
      typeof label === 'string' && label.length > 0,
      `book ${code} has a display label (raw wire codes are not player-facing text)`
    );
    assert(label !== code, `book ${code}'s label is a real book name, not the wire code`);
  }
}

function testLabelsDefineNoBookTheCatalogDoesNotServe() {
  assertEqual(
    Object.keys(BOOK_LABELS).sort().join(','),
    [...CHAINED_BOOK_CODES].sort().join(','),
    'BOOK_LABELS defines exactly the served books'
  );
}

function testArgIsLabelledWithItsRealBookName() {
  assertEqual(BOOK_LABELS.ARG, 'Advanced Race Guide', "ARG's display label");
}

function testUmIsLabelledWithItsRealBookName() {
  assertEqual(BOOK_LABELS.UM, 'Ultimate Magic', "UM's display label");
}

function testOaIsLabelledWithItsRealBookName() {
  assertEqual(BOOK_LABELS.OA, 'Occult Adventures', "OA's display label");
}

function testUcIsLabelledWithItsRealBookName() {
  assertEqual(BOOK_LABELS.UC, 'Ultimate Combat', "UC's display label");
}

function testFormatBookListReadsAsProseOverTheRealLabels() {
  assertEqual(formatBookList(['CRB']), 'Core Rulebook', 'single book');
  assertEqual(
    formatBookList(['CRB', 'ACG']),
    'Core Rulebook and Advanced Class Guide',
    'two books'
  );
  assertEqual(
    formatBookList(BOOK_ORDER),
    "Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Advanced Race Guide, Ultimate Intrigue, Ultimate Magic, Occult Adventures, Ultimate Combat, Inner Sea Gods and Ultimate Wilderness",
    'every served book'
  );
}

function testFormatBookListNeverInventsALabelForAnUnknownCode() {
  assertEqual(formatBookList(['B1']), 'B1', 'an unlabelled code falls back to the wire code');
}

function main() {
  testBookOrderCoversEveryServedBookInChainOrder();
  testEveryOrderedBookHasARealDisplayLabel();
  testLabelsDefineNoBookTheCatalogDoesNotServe();
  testArgIsLabelledWithItsRealBookName();
  testUmIsLabelledWithItsRealBookName();
  testOaIsLabelledWithItsRealBookName();
  testUcIsLabelledWithItsRealBookName();
  testFormatBookListReadsAsProseOverTheRealLabels();
  testFormatBookListNeverInventsALabelForAnUnknownCode();
}

main();

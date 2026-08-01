import { BOOK_LABELS, BOOK_ORDER, formatBookList } from './SpellCatalogScreen';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The catalog screen's book surface must cover every book the Rust adapter
 * actually serves. `spell_catalog.rs`'s `build_spell_catalog` chains CRB ->
 * APG -> ACG -> ARG, and its pinned test
 * `the_catalog_serves_every_ingested_book_not_only_crb` asserts 652 + 297 +
 * 144 + 92 = 1185. A code the adapter serves but this screen omits has no
 * filter button and renders its badge under a raw wire code, so the book's
 * records are reachable only under "All books" and are labelled with a code
 * no player-facing text explains.
 */

/** The wire codes `build_spell_catalog` chains, in that order. */
const CHAINED_BOOK_CODES = ['CRB', 'APG', 'ACG', 'ARG'] as const;

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

function testFormatBookListReadsAsProseOverTheRealLabels() {
  assertEqual(formatBookList(['CRB']), 'Core Rulebook', 'single book');
  assertEqual(
    formatBookList(['CRB', 'ACG']),
    'Core Rulebook and Advanced Class Guide',
    'two books'
  );
  assertEqual(
    formatBookList(BOOK_ORDER),
    "Core Rulebook, Advanced Player's Guide, Advanced Class Guide and Advanced Race Guide",
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
  testFormatBookListReadsAsProseOverTheRealLabels();
  testFormatBookListNeverInventsALabelForAnUnknownCode();
}

main();

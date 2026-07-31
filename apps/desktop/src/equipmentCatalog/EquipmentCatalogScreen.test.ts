import { BOOK_LABELS, BOOK_ORDER, CATEGORY_ORDER, formatBookList } from './EquipmentCatalogScreen';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The equipment catalog screen's book surface must cover every book the Rust
 * adapter actually serves. `equipment_catalog.rs`'s `build_equipment_catalog`
 * chains CRB -> APG -> ACG -> B1 -> ARG -> PU and exports those codes as
 * `EQUIPMENT_CATALOG_BOOKS`; its pinned test
 * `catalog_spans_every_ingested_book_with_their_real_counts` asserts
 * 2977 + 338 + 269 + 4 + 200 + 42 = 3830.
 *
 * A code the adapter serves but this screen omits has no filter button and
 * renders its badge under a raw wire code, so that book's records are
 * reachable only by scrolling the undifferentiated 3830-row list and are
 * labelled with a code no player-facing text explains.
 */

/** The wire codes `EQUIPMENT_CATALOG_BOOKS` declares, in chain order. */
const CHAINED_BOOK_CODES = ['CRB', 'APG', 'ACG', 'B1', 'ARG', 'PU'] as const;

/**
 * The four `EquipmentCategory` variant names the adapter emits. Derived, not
 * assumed: `per_book_category_counts_are_pinned` pins per-book counts that sum
 * to each book's own pinned total (CRB 310+453+1556+658 = 2977, APG 75+93+170
 * = 338, ACG 20+60+141+48 = 269, B1 2+1+1 = 4, ARG 28+79+78+15 = 200, PU 42),
 * so these four categories exhaustively account for all 3830 rows.
 */
const SERVED_CATEGORIES = ['ArmsArmor', 'General', 'MagicItems', 'Equipmods'] as const;

function testBookOrderCoversEveryServedBookInChainOrder() {
  assertEqual(
    BOOK_ORDER.join(','),
    CHAINED_BOOK_CODES.join(','),
    'BOOK_ORDER matches EQUIPMENT_CATALOG_BOOKS chain order'
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

function testTheNewlyReachedBooksAreLabelledWithTheirRealNames() {
  assertEqual(BOOK_LABELS.ARG, 'Advanced Race Guide', "ARG's display label");
  assertEqual(BOOK_LABELS.PU, 'Pathfinder Unchained', "PU's display label");
  assertEqual(BOOK_LABELS.B1, 'Bestiary 1', "B1's display label");
}

function testCategoryOrderCoversEveryServedCategory() {
  assertEqual(
    [...CATEGORY_ORDER].sort().join(','),
    [...SERVED_CATEGORIES].sort().join(','),
    'CATEGORY_ORDER covers exactly the categories the adapter emits'
  );
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
    "Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Bestiary 1, " +
      'Advanced Race Guide and Pathfinder Unchained',
    'every served book'
  );
}

function testFormatBookListNeverInventsALabelForAnUnknownCode() {
  assertEqual(formatBookList(['UM']), 'UM', 'an unlabelled code falls back to the wire code');
}

function testFormatBookListOfNothingIsEmptyRatherThanAFabricatedBook() {
  assertEqual(formatBookList([]), '', 'no books yields no prose');
}

function main() {
  testBookOrderCoversEveryServedBookInChainOrder();
  testEveryOrderedBookHasARealDisplayLabel();
  testLabelsDefineNoBookTheCatalogDoesNotServe();
  testTheNewlyReachedBooksAreLabelledWithTheirRealNames();
  testCategoryOrderCoversEveryServedCategory();
  testFormatBookListReadsAsProseOverTheRealLabels();
  testFormatBookListNeverInventsALabelForAnUnknownCode();
  testFormatBookListOfNothingIsEmptyRatherThanAFabricatedBook();
}

main();

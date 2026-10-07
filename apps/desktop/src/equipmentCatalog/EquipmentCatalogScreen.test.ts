import { BOOK_LABELS, BOOK_ORDER, formatBookList, hasDescription } from './EquipmentCatalogScreen';
import { loadEquipmentCatalogRuntime } from './equipmentCatalogRuntime';
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

/**
 * `equipment_catalog.rs` renders a real `description` for 2856 of the 3830
 * served records, and until this cycle the TypeScript side did not declare
 * the field, so it crossed the IPC boundary and reached no screen. The
 * predicate below is what decides whether a row shows one.
 */
function testDescriptionPresenceIsDecidedByRealContentNotByPresenceOfAField() {
  assert(hasDescription('This sword is about 3-1/2 feet in length.'), 'real prose is a description');
  assert(!hasDescription(null), 'a null description is an absence, not an empty string to render');
  assert(!hasDescription(undefined), 'an omitted description is an absence');
  assert(!hasDescription(''), 'an empty string renders nothing rather than an empty line');
  assert(!hasDescription('   \n  '), 'whitespace-only text is the same absence wearing a different shape');
}

/**
 * The preview catalog must exercise both branches, or the browser preview
 * silently stops representing the real catalog (where 974 of 3830 rows have
 * no description at all) and the empty-state rendering goes unwalked.
 */
async function testThePreviewCatalogCarriesBothRealProseAndRealAbsences() {
  const entries = await loadEquipmentCatalogRuntime();
  const described = entries.filter((entry) => hasDescription(entry.description));
  const undescribed = entries.filter((entry) => !hasDescription(entry.description));

  assert(described.length > 0, 'the preview shows at least one real corpus description');
  assert(undescribed.length > 0, 'the preview keeps at least one genuinely description-less record');

  const longsword = entries.find((entry) => entry.key === 'Longsword (Base)');
  assertEqual(
    longsword?.description,
    'This sword is about 3-1/2 feet in length.',
    "the preview's Longsword carries its verbatim corpus DESC prose, not sample text"
  );

  const backpack = entries.find((entry) => entry.key === 'Backpack');
  assertEqual(
    backpack?.description,
    null,
    "Backpack's corpus row genuinely has no description; the preview says so rather than inventing one"
  );

  for (const entry of entries) {
    assert(
      !(entry.description ?? '').includes('%%'),
      `${entry.key} would put a raw PCGen escape on screen: ${entry.description}`
    );
  }
}

async function main() {
  await testThePreviewCatalogCarriesBothRealProseAndRealAbsences();
  testDescriptionPresenceIsDecidedByRealContentNotByPresenceOfAField();
  testBookOrderCoversEveryServedBookInChainOrder();
  testEveryOrderedBookHasARealDisplayLabel();
  testLabelsDefineNoBookTheCatalogDoesNotServe();
  testTheNewlyReachedBooksAreLabelledWithTheirRealNames();
  testFormatBookListReadsAsProseOverTheRealLabels();
  testFormatBookListNeverInventsALabelForAnUnknownCode();
  testFormatBookListOfNothingIsEmptyRatherThanAFabricatedBook();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

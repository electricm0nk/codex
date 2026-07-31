import {
  INGESTED_SOURCEBOOKS,
  NOT_STARTED_SOURCEBOOKS,
  SOURCEBOOK_STATUS_ROWS,
} from './sourcebookSupportStatus';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The Support Debt panel's "Other sourcebooks" list used to render
 * "Advanced Player's Guide — NOT STARTED" and "Advanced Class Guide — NOT
 * STARTED" next to the Core Rulebook rows. Both were false by then: the
 * APG and ACG catalogs ship in the app and a player can browse them.
 *
 * Derived, not assumed:
 *   spells      `src/rules_core/rules_tables/<book>/spell_list.rs`
 *               CRB 652, APG 297, ACG 144, ARG 92
 *   equipment   `.../<book>/equipment_data*`
 *               CRB 2977, APG 338, ACG 269, B1 4, ARG 200, PU 42
 *   feats       `feat_catalog.rs`'s pinned per-source counts
 *               Crb 185, Apg 172, Acg 129, Arg 187, Pu 17
 */
function verifiesNoIngestedBookIsListedAsNotStarted() {
  for (const book of INGESTED_SOURCEBOOKS) {
    assert(
      !NOT_STARTED_SOURCEBOOKS.includes(book.name),
      `${book.name} ships real catalog content and must never be listed as not started`
    );
  }
}

function verifiesTheApgAndAcgAreListedAsIngested() {
  const names = INGESTED_SOURCEBOOKS.map((book) => book.name);
  for (const expected of [
    "Advanced Player's Guide",
    'Advanced Class Guide',
    'Advanced Race Guide',
    'Bestiary 1',
    'Pathfinder Unchained',
  ]) {
    assert(names.includes(expected), `${expected} has ingested content and must be listed as such`);
  }
}

/**
 * Every ingested book must say what it actually contributes, so the row is
 * a checkable claim rather than a vague reassurance.
 */
function verifiesEveryIngestedBookNamesItsRealContribution() {
  for (const book of INGESTED_SOURCEBOOKS) {
    assert(book.contributes.trim().length > 0, `${book.name} must name what it contributes`);
    assert(/\d/.test(book.contributes), `${book.name}'s contribution must carry a real count`);
  }
}

function verifiesOnlyBooksWithNoContentRemainNotStarted() {
  assertEqual(
    NOT_STARTED_SOURCEBOOKS.join(' | '),
    'Ultimate Combat | Ultimate Magic | Ultimate Equipment',
    'only the books with no rules_tables module at all stay on the not-started list'
  );
}

/** The rendered list must be exactly the two lists, with no book dropped. */
function verifiesRenderedRowsCoverBothListsExactly() {
  assertEqual(
    SOURCEBOOK_STATUS_ROWS.length,
    INGESTED_SOURCEBOOKS.length + NOT_STARTED_SOURCEBOOKS.length,
    'every ingested and not-started book gets exactly one rendered row'
  );
  const ingestedRows = SOURCEBOOK_STATUS_ROWS.filter((row) => row.status === 'Ingested');
  assertEqual(ingestedRows.length, INGESTED_SOURCEBOOKS.length, 'each ingested book renders once');
  const notStartedRows = SOURCEBOOK_STATUS_ROWS.filter((row) => row.status === 'Not started');
  assertEqual(
    notStartedRows.length,
    NOT_STARTED_SOURCEBOOKS.length,
    'each not-started book renders once'
  );
}

function main() {
  verifiesNoIngestedBookIsListedAsNotStarted();
  verifiesTheApgAndAcgAreListedAsIngested();
  verifiesEveryIngestedBookNamesItsRealContribution();
  verifiesOnlyBooksWithNoContentRemainNotStarted();
  verifiesRenderedRowsCoverBothListsExactly();
}

main();

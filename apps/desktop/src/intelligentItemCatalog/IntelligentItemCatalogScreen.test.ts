import { formatBook, formatCost, formatMechanic } from './IntelligentItemCatalogScreen';
import { loadIntelligentItemCatalogRuntime } from './intelligentItemCatalogRuntime';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * `formatCost` must state real numbers in a player-readable form and must
 * say plainly, rather than silently drop, that some rows carry no separate
 * price (`Intelligent Item ~ Purpose / Slay All`'s real corpus `cost_gp` is
 * `null` — folded into the Base row's own price instead).
 */
function testFormatCostRendersRealPricesAndNamesAnAbsentOne() {
  assertEqual(formatCost(1000), '1,000 gp', 'a real price formats with a thousands separator');
  assertEqual(formatCost(0), '0 gp', 'a real zero price is still a number, not folded into the absent case');
  assertEqual(formatCost(null), 'no separate cost stated', 'an absent price says so in words');
}

/**
 * `formatMechanic` must never collapse the Base row's full price-band
 * sentence into a redundant `"Ego: Base Ego from..."` prefix, and must
 * append a translated condition and bonus-type tag when the row states
 * them — this is the ONLY place the DTO's separate `condition`/`bonusType`
 * fields are joined back into one reader-facing line, so a regression here
 * would silently drop real corpus-derived information from the screen even
 * though the backend still served it correctly.
 */
function testFormatMechanicJoinsEffectFormulaConditionAndType() {
  assertEqual(
    formatMechanic({ variable: 'IntItemStatINT', effect: 'Intelligence', formula: '+4', condition: null, bonusType: null }),
    'Intelligence +4',
    'a plain literal delta reads as "<effect> <formula>"'
  );
  assertEqual(
    formatMechanic({
      variable: 'IntelligentItemEgo',
      effect: 'Ego',
      formula: '+2',
      condition: null,
      bonusType: 'Purpose',
    }),
    'Ego +2 [Purpose]',
    'a bonus-type tag is appended in brackets'
  );
  assertEqual(
    formatMechanic({
      variable: 'NegLevels',
      effect: 'Negative levels while attuned',
      formula: '1+IntItemNegativeLevel',
      condition: "wielder's alignment is not Lawful Good",
      bonusType: null,
    }),
    "Negative levels while attuned 1+IntItemNegativeLevel (if wielder's alignment is not Lawful Good)",
    'a translated condition is appended as "(if ...)"'
  );
  const priceBandFormula =
    'Base Ego from item price (cumulative): price \u{2265} 1001 gp: +1 Ego; price \u{2265} 5001 gp: +1 Ego';
  assertEqual(
    formatMechanic({ variable: 'IntelligentItemEgo', effect: 'Ego', formula: priceBandFormula, condition: null, bonusType: null }),
    priceBandFormula,
    'the Base row price-band sentence is never prefixed with a redundant "Ego:" label'
  );
}

function testFormatBookMapsKnownDirectoriesAndFallsThroughForAnUnmapped() {
  assertEqual(formatBook('core_rulebook'), 'Core Rulebook', 'CRB label');
  assertEqual(formatBook('mythic_adventures'), 'Mythic Adventures (Legendary Item)', 'Mythic label');
  assertEqual(formatBook('some_future_book'), 'some_future_book', 'an unmapped directory falls through as itself');
}

/**
 * The browser-preview fallback (used when no Tauri runtime is present) must
 * never state a fabricated total Ego score, must never leak a raw `|`
 * argument tail from an unresolved PCGen `%` placeholder, and must contain
 * at least one real component from each book — the same "the preview
 * cannot show what the corpus does not contain" discipline
 * `EquipmentCatalogScreen.test.ts` already holds `loadEquipmentCatalogRuntime`
 * to.
 */
async function testThePreviewCatalogNeverFabricatesAnEgoTotalOrLeaksSyntax() {
  const entries = await loadIntelligentItemCatalogRuntime();
  assert(entries.length > 0, 'the preview catalog is non-empty');
  assert(entries.some((entry) => entry.book === 'core_rulebook'), 'preview includes a CRB row');
  assert(entries.some((entry) => entry.book === 'mythic_adventures'), 'preview includes a Mythic row');

  const base = entries.find((entry) => entry.key === 'Intelligent Item ~ Base');
  assert(!!base, 'preview includes the CRB Base row');
  assertEqual(base ? base.egoDelta : 'MISSING', null, 'the Base row never states a resolved egoDelta number');

  for (const entry of entries) {
    assert(!(entry.description ?? '').includes('|'), `${entry.key} would leak a raw pipe-argument tail: ${entry.description}`);
    for (const mechanic of entry.mechanics) {
      assert(!mechanic.formula.includes('var("'), `${entry.key}'s ${mechanic.variable} formula leaks raw var(...) syntax`);
    }
  }
}

async function main() {
  testFormatCostRendersRealPricesAndNamesAnAbsentOne();
  testFormatMechanicJoinsEffectFormulaConditionAndType();
  testFormatBookMapsKnownDirectoriesAndFallsThroughForAnUnmapped();
  await testThePreviewCatalogNeverFabricatesAnEgoTotalOrLeaksSyntax();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

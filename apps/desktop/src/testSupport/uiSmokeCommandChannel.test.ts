/**
 * Self-executing, DOM-free test for `uiProbe.ts`'s DOM command channel name
 * matcher (`matchCommandTarget`) — the piece that decides which on-screen
 * element a `{op:'click', target:'X'}` command from
 * `scripts/ui-smoke/lib/commandChannel.mjs` actually resolves to. A
 * regression here (e.g. falling back to substring before prefix, or picking
 * the wrong tier's candidate for a given `index`) would make the harness's
 * deterministic click/type channel silently drive the wrong control instead
 * of failing loudly — worse than the `xdotool` pixel-coordinate driving it
 * replaces, which at least always hits *something* at a fixed point.
 *
 * Pure function, no DOM: importing `uiProbe.ts` itself is safe under plain
 * Node/tsx because every DOM/Tauri access it makes lives inside function
 * bodies (`installUiProbe`, `executeCommand`, ...), never at module scope.
 */
import { assert, assertEqual } from './asserts';
import { __testables } from './uiProbe';

const { matchCommandTarget } = __testables;

function verifiesExactMatchWinsOverPrefixAndContains() {
  const names = ['Back to landing', 'Back', 'Go Back'];
  assertEqual(matchCommandTarget(names, 'Back'), 1, 'the exact match (index 1) is chosen over the prefix/contains matches');
}

function verifiesPrefixWinsOverContainsWhenNoExactMatchExists() {
  // 'Backpack' and 'Back to landing' both start with 'Back' (the prefix
  // tier); 'Go Back' only contains it. The first prefix candidate (index 1,
  // 'Backpack') wins over both the second prefix candidate and the
  // contains-only one.
  const names = ['Go Back', 'Backpack', 'Back to landing'];
  assertEqual(matchCommandTarget(names, 'Back'), 1, 'the first prefix match is chosen over the contains-only match');
}

function verifiesContainsIsTheLastResortTier() {
  const names = ['Go Back now'];
  assertEqual(matchCommandTarget(names, 'Back'), 0, 'falls through to a substring match when nothing exact or prefixed exists');
}

function verifiesNoMatchReturnsNegativeOne() {
  const names = ['Cancel', 'Close'];
  assertEqual(matchCommandTarget(names, 'Back'), -1, 'no candidate at any tier reports -1');
}

function verifiesIndexSelectsWithinTheMatchedTier() {
  const names = ['Add', 'Remove', 'Add', 'Add'];
  assertEqual(matchCommandTarget(names, 'Add', 0), 0, 'index 0 (default) is the first exact match');
  assertEqual(matchCommandTarget(names, 'Add', 1), 2, 'index 1 is the second exact match, not the second candidate overall');
  assertEqual(matchCommandTarget(names, 'Add', 2), 3, 'index 2 is the third exact match');
}

function verifiesAnOutOfRangeIndexFallsBackToTheFirstMatchInTier() {
  const names = ['Add', 'Add'];
  assertEqual(matchCommandTarget(names, 'Add', 5), 0, 'an index past the end of the matched tier does not report -1');
}

function verifiesEmptyNamesListReportsNoMatch() {
  assert(matchCommandTarget([], 'anything') === -1, 'an empty candidate list can never match');
}

const cases: [string, () => void][] = [
  ['exact match wins over prefix and contains', verifiesExactMatchWinsOverPrefixAndContains],
  ['prefix wins over contains when no exact match exists', verifiesPrefixWinsOverContainsWhenNoExactMatchExists],
  ['contains is the last-resort tier', verifiesContainsIsTheLastResortTier],
  ['no match returns -1', verifiesNoMatchReturnsNegativeOne],
  ['index selects within the matched tier', verifiesIndexSelectsWithinTheMatchedTier],
  ['an out-of-range index falls back to the first match in tier', verifiesAnOutOfRangeIndexFallsBackToTheFirstMatchInTier],
  ['an empty names list reports no match', verifiesEmptyNamesListReportsNoMatch],
];

let failures = 0;
for (const [label, run] of cases) {
  try {
    run();
    console.log(`  ok - ${label}`);
  } catch (cause) {
    failures += 1;
    console.error(`  FAIL - ${label}: ${cause instanceof Error ? cause.message : String(cause)}`);
  }
}

if (failures > 0) {
  console.error(`${failures}/${cases.length} matchCommandTarget cases failed.`);
  process.exit(1);
}
console.log(`All ${cases.length} matchCommandTarget cases passed.`);

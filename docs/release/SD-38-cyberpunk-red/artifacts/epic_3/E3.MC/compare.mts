// E3.MC evidence: is every number on the LOADED in-app sheet the number compute_sheet gives the
// character's SAVED input?
//
//   cd apps/desktop && npx tsx <this file> <expected.json> <probe.json> [<report.json>]
//
// expected.json: sheet_tool's output for the saved character directory captured from the ui-smoke
//   run (compose_sheet + cpr_sheet_print over the saved files, in load_saved_character's shape).
// probe.json:    the app's own UI probe snapshot (uiProbe.ts) taken while the loaded sheet was on
//   screen; `bodyText` is the page's innerText.
//
// The expected sheet is the app's own component (CyberpunkCharacterSheet) rendered over
// expected.json with the corpus skill and critical-injury catalogs, so the labels and layout are
// the app's and only the numbers' source differs. Checks:
//   A. every computed cell (a `data-cpr-row` value) the expected sheet shows: its table row
//      (label + every cell, in order) is a line of the live sheet's text, as often as expected;
//   B. every line of the live sheet's text that holds a digit is a line of the expected sheet's
//      text, no more often than expected (no number on screen that compute_sheet did not give);
//   C. the multiset of computed values: expected `data-cpr-row` cells vs. explanations.
// Exit 0 only when A and B have no miss.
import { readFileSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const desktop = resolve(process.cwd());
const requireDesktop = createRequire(resolve(desktop, 'package.json'));
const { createElement } = requireDesktop('react') as typeof import('react');
const { renderToStaticMarkup } = requireDesktop('react-dom/server') as typeof import('react-dom/server');

const [expectedPath, probePath, reportPath] = process.argv.slice(2);
if (!expectedPath || !probePath) {
  console.error('usage: npx tsx compare.mts <expected.json> <probe.json> [<report.json>]');
  process.exit(2);
}

const sheetModule = await import(pathToFileURL(resolve(desktop, 'src/characterHub/CyberpunkCharacterSheet.tsx')).href);
const corpus = resolve(desktop, '../../data/cyberpunk-red/corpus');
const skills = JSON.parse(readFileSync(resolve(corpus, 'skill.json'), 'utf8'));
const criticals = JSON.parse(readFileSync(resolve(corpus, 'critical_injury.json'), 'utf8'));
const detail = JSON.parse(readFileSync(expectedPath, 'utf8'));
const probe = JSON.parse(readFileSync(probePath, 'utf8'));

const html: string = renderToStaticMarkup(
  createElement(sheetModule.CyberpunkCharacterSheet, {
    detail,
    onClose: () => undefined,
    onOpen: () => undefined,
    skillCatalog: skills,
    criticalCatalog: criticals,
  }),
);

const decode = (text: string) =>
  text.replace(/&quot;/g, '"').replace(/&#x27;/g, "'").replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&');
const norm = (text: string) => text.replace(/\s+/g, ' ').trim();
const textOf = (fragment: string) => norm(decode(fragment.replace(/<[^>]*>/g, '')));

// Expected text lines: a table row is its cells joined by a tab (innerText's form); every other
// block element is its own text.
const expectedRows: { line: string; rowIds: string[] }[] = [];
for (const m of html.matchAll(/<tr[^>]*>([\s\S]*?)<\/tr>/g)) {
  const cells = [...m[1].matchAll(/<t[hd][^>]*>([\s\S]*?)<\/t[hd]>|<t[hd][^>]*\/>/g)].map((c) => textOf(c[1] ?? ''));
  const rowIds = [...m[1].matchAll(/data-cpr-row="([^"]+)"/g)].map((r) => r[1]);
  expectedRows.push({ line: norm(cells.join('\t')), rowIds });
}
// In "Rules and features" each line's label, value and `also` figures are items of a flex row,
// which innerText puts on lines of their own: there a span is a block too.
const rulesAt = html.indexOf('data-rule-id=');
const blocked = rulesAt < 0 ? html : html.slice(0, rulesAt) + html.slice(rulesAt).replace(/<\/?span[^>]*>/g, '\n');
const blockText = blocked
  .replace(/<tr[^>]*>[\s\S]*?<\/tr>/g, '\n')
  .replace(/<(p|h[1-6]|li|label|div|section|button|summary|dt|dd)[^>]*>/g, '\n')
  .replace(/<\/(p|h[1-6]|li|label|div|section|button|summary|dt|dd)>/g, '\n')
  .replace(/<[^>]*>/g, '');
const expectedLines: string[] = [
  ...expectedRows.map((r) => r.line),
  ...decode(blockText).split('\n').map(norm).filter((l) => l !== ''),
];
// Lines are compared as multisets: a line printed twice must be on screen twice (the Hit Points
// figure is on a STAT-block row and again on a printed rule line).
const count = (lines: readonly string[]) => {
  const counts = new Map<string, number>();
  for (const l of lines) counts.set(l, (counts.get(l) ?? 0) + 1);
  return counts;
};
const expectedCount = count(expectedLines);

// The live sheet's lines: bodyText from the sheet's heading (its name) to the end.
const body: string = probe.bodyText ?? '';
const name: string = detail.summary.displayLabel;
const allLines = body.split('\n').map(norm);
const start = allLines.indexOf(name);
if (start < 0) {
  console.error(`the live page does not show the sheet heading '${name}'`);
  process.exit(1);
}
const liveLines = allLines.slice(start).filter((l) => l !== '');
const liveCount = count(liveLines);

const computedRows = expectedRows.filter((r) => r.rowIds.length > 0);
// A: each computed row's line is on screen at least as often as the expected sheet prints it.
const missA = computedRows.filter((r) => (liveCount.get(r.line) ?? 0) < (expectedCount.get(r.line) ?? 0));
// B: no line holding a digit is on screen more often than the expected sheet prints it.
const digitLines = liveLines.filter((l) => /\d/.test(l));
const missB = [...new Set(digitLines)].filter((l) => (liveCount.get(l) ?? 0) > (expectedCount.get(l) ?? 0));
const cellValues = [...html.matchAll(/data-cpr-row="([^"]+)"[^>]*>([^<]*)</g)].map((m) => [m[1], decode(m[2])]);
const explained = new Map<string, number>(detail.explanations.map((e: { id: string; value: number }) => [e.id, e.value]));
const missC = cellValues.filter(([id, v]) => String(explained.get(id)) !== v);

const report = {
  character: name,
  explanations: detail.explanations.length,
  printedLines: detail.sheetLines.length,
  computedCellsShown: cellValues.length,
  computedTableRows: computedRows.length,
  computedTableRowsOnLiveSheet: computedRows.length - missA.length,
  liveLinesWithADigit: digitLines.length,
  liveDigitLinesInExpectedSheet: digitLines.length - missB.reduce((n, l) => n + (liveCount.get(l) ?? 0) - (expectedCount.get(l) ?? 0), 0),
  cellsNotEqualToTheirRow: missC.length,
  missA: missA.map((r) => r.line),
  missB,
  missC,
};
if (reportPath) {
  writeFileSync(reportPath, JSON.stringify(report, null, 2) + '\n');
}
console.log(
  `${name}: A ${report.computedTableRowsOnLiveSheet}/${report.computedTableRows} computed rows on the live sheet; ` +
    `B ${report.liveDigitLinesInExpectedSheet}/${report.liveLinesWithADigit} live digit lines from compute_sheet; ` +
    `C ${cellValues.length - missC.length}/${cellValues.length} cells = their row (${report.explanations} rows, ${report.printedLines} printed lines)`,
);
for (const l of report.missA.slice(0, 20)) console.log(`  A miss: ${l} (on screen ${liveCount.get(l) ?? 0}x, expected ${expectedCount.get(l)}x)`);
for (const l of missB.slice(0, 20)) console.log(`  B miss: ${l} (on screen ${liveCount.get(l)}x, expected ${expectedCount.get(l) ?? 0}x)`);
for (const [id, v] of missC.slice(0, 20)) console.log(`  C miss: ${id} shows ${v}, row ${explained.get(id)}`);
process.exit(missA.length + missB.length + missC.length === 0 ? 0 : 1);

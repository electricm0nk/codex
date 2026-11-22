/**
 * SD-38 E6.4 -- the GM reference pages: every `print_only` MANIFEST kind renders read-only.
 *
 * Responses are built from the corpus files the way `list_cyberpunk_catalog` serves them. Proves,
 * per page (NET Architecture, Demons, Night Market, Playstyle IP awards, trauma Humanity Loss,
 * Mooks and Grunts, Encounters, Beat Charts):
 *
 * 1. The catalog has a GM Reference group whose tabs are exactly the MANIFEST's `print_only` kinds.
 * 2. The page lists every record once, in order, and its summary counts the MANIFEST's `expected`.
 * 3. Every printed string and number of every record appears on the page (nothing is dropped), and
 *    the page's own spot checks (a table row, a column, a stat block) show.
 * 4. A search narrows the page; the page holds no input that changes a record (read-only).
 */
import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { CyberpunkCatalogResponse, CyberpunkRecord } from '../../boundary/loadCyberpunkCatalog';
import { CYBERPUNK_CATALOG_GROUPS, CYBERPUNK_CATALOG_TABS, firstKindOfGroup } from '../cyberpunkCatalogModel';
import { CyberpunkGmPage } from './CyberpunkGmPage';
import { assert, assertEqual } from '../../testSupport/asserts';

interface ManifestKind {
  kind: string;
  file: string;
  expected: number;
  print_only: boolean;
}

const CORPUS = new URL('../../../../../data/cyberpunk-red/corpus/', import.meta.url);
const MANIFEST = (JSON.parse(readFileSync(new URL('MANIFEST.json', CORPUS), 'utf8')) as { kinds: ManifestKind[] }).kinds;
const PRINT_ONLY = MANIFEST.filter((entry) => entry.print_only);

function response(entry: ManifestKind): CyberpunkCatalogResponse {
  return {
    kind: entry.kind,
    source: 'data/cyberpunk-red/corpus',
    expected: entry.expected,
    printOnly: entry.print_only,
    records: JSON.parse(readFileSync(new URL(entry.file, CORPUS), 'utf8')) as CyberpunkRecord[],
  };
}

function decode(text: string): string {
  return text.replace(/&quot;/g, '"').replace(/&#x27;/g, "'").replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&');
}

function render(loaded: CyberpunkCatalogResponse, query = ''): string {
  return decode(renderToStaticMarkup(createElement(CyberpunkGmPage, { response: loaded, query })));
}

/** Objects whose keys are printed data (column names, stat names), not field names the page relabels. */
const DATA_KEYS = new Set(['columns', 'values', 'floors', 'stats', 'skill_bases']);

/** Every string and number the record prints, except the bookkeeping fields the page keeps off the body. */
function leaves(value: unknown, key: string, out: string[]) {
  if (key === 'kind' || key === 'source_page' || key === 'also_on' || key === 'continues_on') {
    return;
  }
  if (typeof value === 'string' && value.length > 0) {
    out.push(value);
  } else if (typeof value === 'number') {
    out.push(String(value));
  } else if (Array.isArray(value)) {
    value.forEach((item) => leaves(item, key, out));
  } else if (value !== null && typeof value === 'object') {
    for (const [field, item] of Object.entries(value as Record<string, unknown>)) {
      if (DATA_KEYS.has(key)) {
        out.push(field);
      }
      leaves(item, field, out);
    }
  }
}

const SPOT_CHECKS: Record<string, string[]> = {
  net_architecture: ['NET Architecture Difficulty Rating', 'Skunk', 'Air Swarm Drone Cloud', 'Hellhound x3', 'Very Expensive'],
  demon: ['Imp', 'Efreet', 'Balron', 'Combat number'],
  night_market_table: ['Food and Drugs', 'Canned Goods', 'Survival Gear'],
  ip_award: ['Playstyle I.P. award 10', 'Warrior', 'Roleplayer'],
  humanity_trauma: ['Torture', '1d6'],
  mooks_grunts: ['Bodyguard', 'Kevlar', 'Very Heavy Pistol', 'Hit points'],
  encounter: ['Local Law', 'Trauma Team'],
  beat_chart: ['Coronet Blue', 'Hook'],
};

function testTheGroupHoldsExactlyThePrintOnlyKinds() {
  assertEqual(PRINT_ONLY.length, 8, 'the MANIFEST holds eight print_only kinds');
  const group = CYBERPUNK_CATALOG_GROUPS.find((entry) => entry.id === 'gm');
  assert(group !== undefined, 'a GM Reference group exists');
  const tabs = CYBERPUNK_CATALOG_TABS.filter((entry) => entry.group === 'gm');
  assert(tabs.every((entry) => entry.tool === 'gm'), 'every GM tab is a gm tool tab');
  assertEqual(
    tabs.map((entry) => entry.kind).sort().join(','),
    PRINT_ONLY.map((entry) => entry.kind).sort().join(','),
    'the GM tabs are the MANIFEST print_only kinds'
  );
  assert(CYBERPUNK_CATALOG_TABS.some((entry) => entry.kind === firstKindOfGroup('gm')), 'the group link opens a real tab');
}

function testEveryPageShowsEveryRecordAndPrintedValue() {
  let shown = 0;
  for (const entry of PRINT_ONLY) {
    const loaded = response(entry);
    assertEqual(loaded.records.length, entry.expected, `${entry.kind}: the corpus file holds the MANIFEST count`);
    const html = render(loaded);
    const rendered = [...html.matchAll(/data-cpr-gm-entry="([^"]+)"/g)].map((match) => match[1]);
    assertEqual(rendered.join(','), loaded.records.map((_, index) => `${entry.kind}:${index}`).join(','), `${entry.kind}: every record once, in order`);
    assert(html.includes(`${entry.expected} of ${entry.expected} `), `${entry.kind}: summary counts ${entry.expected} of ${entry.expected}`);
    const missing: string[] = [];
    for (const record of loaded.records) {
      const text: string[] = [];
      leaves(record, '', text);
      for (const piece of text) {
        if (!html.includes(piece.trim())) {
          missing.push(piece.slice(0, 40));
        }
      }
    }
    assertEqual(missing.length, 0, `${entry.kind}: every printed value shows (missing ${missing.slice(0, 3).join(' | ')})`);
    for (const needle of SPOT_CHECKS[entry.kind] ?? []) {
      assert(html.toLowerCase().includes(needle.toLowerCase()), `${entry.kind}: shows ${needle}`);
    }
    assert(!/<input|<textarea|<select/.test(html), `${entry.kind}: read-only, no input`);
    shown += rendered.length;
  }
  console.log(`cyberpunkGm: ${shown} records across ${PRINT_ONLY.length} GM pages (${PRINT_ONLY.map((entry) => `${entry.kind} ${entry.expected}`).join(', ')})`);
}

function testASearchNarrowsAPage() {
  const encounters = response(PRINT_ONLY.find((entry) => entry.kind === 'encounter') as ManifestKind);
  const html = render(encounters, 'trauma team');
  const rendered = [...html.matchAll(/data-cpr-gm-entry="/g)].length;
  assert(rendered >= 1 && rendered < encounters.records.length, 'a search narrows the encounters');
  assert(html.includes(`${rendered} of ${encounters.records.length} of ${encounters.expected} `), 'the narrowed summary counts');
}

testTheGroupHoldsExactlyThePrintOnlyKinds();
testEveryPageShowsEveryRecordAndPrintedValue();
testASearchNarrowsAPage();
console.log('cyberpunkGm tests passed');

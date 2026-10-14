/**
 * SD-38 E6.2 -- the Cyberpunk RED catalogs read `data/cyberpunk-red/corpus`.
 *
 * The responses are built from the corpus files the way `list_cyberpunk_catalog` serves them
 * (`cpr_catalog.rs`: the kind's rows as they are, the MANIFEST's `expected` and `print_only`).
 * Proves:
 *
 * 1. The tab list is exactly the MANIFEST's non-`print_only` kinds: one tab per kind, none for a
 *    GM reference kind, none missing.
 * 2. The landing screen, with Cyberpunk Red selected, offers one link per catalog group and none
 *    of the Pathfinder or Starfinder ones; the other two systems offer no Cyberpunk link.
 * 3. For every tab, the catalog view lists every record of the kind once, in order, and its
 *    summary counts equal the MANIFEST's `expected`.
 * 4. A search narrows the list; a record's printed fields (a weapon's damage, a skill's stat) show.
 */
import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { CyberpunkCatalogResponse, CyberpunkRecord } from '../boundary/loadCyberpunkCatalog';
import { LandingScreen, type RuleSetId } from '../characterHub/LandingScreen';
import { CyberpunkCatalogView } from './CyberpunkCatalogScreen';
import {
  CYBERPUNK_CATALOG_GROUPS,
  CYBERPUNK_CATALOG_TABS,
  describeValue,
  filterCyberpunkRecords,
  firstKindOfGroup,
} from './cyberpunkCatalogModel';
import { assert, assertEqual } from '../testSupport/asserts';

interface ManifestKind {
  kind: string;
  file: string;
  expected: number;
  print_only: boolean;
}

const CORPUS = new URL('../../../../data/cyberpunk-red/corpus/', import.meta.url);
const MANIFEST = (JSON.parse(readFileSync(new URL('MANIFEST.json', CORPUS), 'utf8')) as { kinds: ManifestKind[] }).kinds;

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

const OTHER_LINKS = [
  'Browse Equipment Catalog',
  'Browse Spell Catalog',
  'Browse Class Progression',
  'Browse Race Traits',
  'Browse Monster Catalog',
  'Browse Companion Catalog',
  'Browse Intelligent Item Components',
  'Browse Races',
  'Browse Themes',
  'Browse Classes',
  'Browse Feats',
  'Browse Spells',
  'Browse Equipment',
];

function landing(selectedRuleSet: RuleSetId): string {
  const noop = () => undefined;
  return decode(
    renderToStaticMarkup(
      createElement(LandingScreen, {
        selectedRuleSet,
        onSelectRuleSet: noop,
        onCreate: noop,
        createGate: { enabled: true },
        onLoad: noop,
        onBrowseEquipment: noop,
        onBrowseSpells: noop,
        onBrowseClasses: noop,
        onBrowseRaces: noop,
        onBrowseMonsters: noop,
        onBrowseCompanions: noop,
        onBrowseIntelligentItems: noop,
        onBrowseStarfinder: noop,
        onBrowseCyberpunk: noop,
        onCorpusIngestDiagnostic: noop,
        onCampaignManager: noop,
        campaignManagerGate: { enabled: true, disabledHint: '' },
        onDmToolkit: noop,
      })
    )
  );
}

function testOneTabPerPlayerFacingKind() {
  const playerFacing = MANIFEST.filter((entry) => !entry.print_only).map((entry) => entry.kind);
  const tabs = CYBERPUNK_CATALOG_TABS.map((entry) => entry.kind);
  assertEqual(new Set(tabs).size, tabs.length, 'no kind has two tabs');
  assertEqual([...tabs].sort().join(','), [...playerFacing].sort().join(','), 'the tabs are the MANIFEST non-print_only kinds');
  const groups = new Set(CYBERPUNK_CATALOG_GROUPS.map((group) => group.id));
  for (const entry of CYBERPUNK_CATALOG_TABS) {
    assert(groups.has(entry.group), `${entry.kind} sits in a known group`);
  }
  for (const group of CYBERPUNK_CATALOG_GROUPS) {
    assert(CYBERPUNK_CATALOG_TABS.some((entry) => entry.group === group.id), `${group.id} holds a tab`);
  }
  console.log(`cyberpunkCatalog: ${tabs.length} tabs for ${MANIFEST.length} MANIFEST kinds (${MANIFEST.length - playerFacing.length} print_only)`);
}

function testTheLandingOffersTheSelectedSystemsCatalogs() {
  const cpr = landing('cyberpunk-red');
  for (const group of CYBERPUNK_CATALOG_GROUPS) {
    assert(cpr.includes(`>${group.link}<`), `Cyberpunk landing offers "${group.link}"`);
    assert(CYBERPUNK_CATALOG_TABS.some((entry) => entry.kind === firstKindOfGroup(group.id)), `${group.id} opens on a real tab`);
  }
  for (const link of OTHER_LINKS) {
    assert(!cpr.includes(`>${link}<`), `Cyberpunk landing does not offer "${link}"`);
  }
  for (const system of ['pathfinder-1e', 'starfinder-1e'] as const) {
    const other = landing(system);
    for (const group of CYBERPUNK_CATALOG_GROUPS) {
      assert(!other.includes(`>${group.link}<`), `${system} landing does not offer "${group.link}"`);
    }
  }
}

function testEveryTabListsEveryRecordOnce() {
  let records = 0;
  for (const tab of CYBERPUNK_CATALOG_TABS) {
    const entry = MANIFEST.find((candidate) => candidate.kind === tab.kind);
    assert(entry !== undefined, `${tab.kind} is in the MANIFEST`);
    const loaded = response(entry as ManifestKind);
    assertEqual(loaded.records.length, loaded.expected, `${tab.kind}: the corpus file holds the MANIFEST count`);
    const html = decode(renderToStaticMarkup(createElement(CyberpunkCatalogView, { response: loaded, query: '' })));
    const rendered = [...html.matchAll(/data-cpr-catalog-entry="([^"]+)"/g)].map((match) => match[1]);
    assertEqual(
      rendered.join(','),
      loaded.records.map((_, index) => `${tab.kind}:${index}`).join(','),
      `${tab.kind}: every record once, in order`
    );
    assert(html.includes(`${loaded.expected} of ${loaded.expected} `), `${tab.kind}: summary counts ${loaded.expected} of ${loaded.expected}`);
    for (const record of loaded.records) {
      if (typeof record.name === 'string') {
        assert(html.includes(record.name), `${tab.kind}: ${record.name} shown`);
      }
    }
    records += rendered.length;
  }
  console.log(`cyberpunkCatalog: ${records} records rendered across ${CYBERPUNK_CATALOG_TABS.length} tabs`);
}

function testSearchAndPrintedFields() {
  const weapons = response(MANIFEST.find((entry) => entry.kind === 'weapon') as ManifestKind);
  const light = filterCyberpunkRecords(weapons.records, 'light melee');
  assert(light.length >= 1 && light.length < weapons.records.length, 'a search narrows the weapons');
  const html = decode(renderToStaticMarkup(createElement(CyberpunkCatalogView, { response: weapons, query: 'light melee' })));
  assert(html.includes('Light Melee Weapon'), 'the matching weapon shows');
  assert(html.includes('Damage:</span> 1d6'), 'its printed damage shows');
  assert(html.includes(`${light.length} of ${weapons.records.length} of ${weapons.expected} weapons`), 'the narrowed summary counts');
  const skills = response(MANIFEST.find((entry) => entry.kind === 'skill') as ManifestKind);
  const concentration = skills.records.find((record) => record.name === 'Concentration');
  assertEqual(describeValue(concentration?.stat), 'WILL', 'the skill prints its stat');
  assertEqual(describeValue({ a: [1, { b: true }] }), 'A: 1; B: yes', 'nested values print field by field');
  assertEqual(filterCyberpunkRecords(weapons.records, 'zzzz-no-such-weapon').length, 0, 'a miss lists nothing');
}

testOneTabPerPlayerFacingKind();
testTheLandingOffersTheSelectedSystemsCatalogs();
testEveryTabListsEveryRecordOnce();
testSearchAndPrintedFields();
console.log('cyberpunkCatalog tests passed');

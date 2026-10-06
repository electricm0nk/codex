/**
 * SD-37 E6.4 -- the Starfinder catalogs read `data/starfinder-1e/sheet_rules`.
 *
 * The fixtures are the real `list_starfinder_catalog` responses for the race
 * and class catalogs (`starfinderCatalogFixtures/<kind>.json`, kept equal to
 * the command's answer by
 * `sf_catalog::tests::the_frontend_catalog_fixtures_are_the_commands_responses`).
 * Proves:
 *
 * 1. The landing screen, with Starfinder 1e selected, offers the six
 *    Starfinder catalogs and none of the Pathfinder ones; with Pathfinder 1e
 *    selected it offers the Pathfinder catalogs and no Starfinder one.
 * 2. The frontend's catalog kinds are exactly the command's (`SfCatalogKind`
 *    in `sf_catalog.rs`).
 * 3. The catalog view lists every Starfinder record the command served, once,
 *    with its rows that feed a sheet total, and a search narrows it.
 */
import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { StarfinderCatalogResponse } from '../boundary/loadStarfinderCatalog';
import { LandingScreen, type RuleSetId } from '../characterHub/LandingScreen';
import { StarfinderCatalogView, MAX_RENDERED_ENTRIES } from './StarfinderCatalogScreen';
import { bookLabel, filterStarfinderCatalog, STARFINDER_CATALOG_TABS } from './starfinderCatalogModel';
import { assert, assertEqual } from '../testSupport/asserts';

const PATHFINDER_LINKS = [
  'Browse Equipment Catalog',
  'Browse Spell Catalog',
  'Browse Class Progression',
  'Browse Race Traits',
  'Browse Monster Catalog',
  'Browse Companion Catalog',
  'Browse Intelligent Item Components',
];

function fixture(kind: string): StarfinderCatalogResponse {
  return JSON.parse(
    readFileSync(new URL(`./starfinderCatalogFixtures/${kind}.json`, import.meta.url), 'utf8')
  ) as StarfinderCatalogResponse;
}

function decode(text: string): string {
  return text.replace(/&quot;/g, '"').replace(/&#x27;/g, "'").replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&');
}

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
        onCorpusIngestDiagnostic: noop,
        onCampaignManager: noop,
        campaignManagerGate: { enabled: true, disabledHint: '' },
        onDmToolkit: noop,
      })
    )
  );
}

function testTheLandingOffersTheSelectedSystemsCatalogs() {
  const sf = landing('starfinder-1e');
  for (const tab of STARFINDER_CATALOG_TABS) {
    assert(sf.includes(`>${tab.link}<`), `Starfinder landing offers "${tab.link}"`);
  }
  for (const link of PATHFINDER_LINKS) {
    assert(!sf.includes(`>${link}<`), `Starfinder landing does not offer the Pathfinder "${link}"`);
  }
  const pf = landing('pathfinder-1e');
  for (const link of PATHFINDER_LINKS) {
    assert(pf.includes(`>${link}<`), `Pathfinder landing still offers "${link}"`);
  }
  for (const tab of STARFINDER_CATALOG_TABS) {
    assert(!pf.includes(`>${tab.link}<`), `Pathfinder landing does not offer "${tab.link}"`);
  }
}

function testTheKindsAreTheCommandsKinds() {
  const rust = readFileSync(new URL('../../src-tauri/src/sf_catalog.rs', import.meta.url), 'utf8');
  const body = /pub enum SfCatalogKind \{([^}]*)\}/.exec(rust);
  assert(body !== null, 'SfCatalogKind found in sf_catalog.rs');
  const variants = (body?.[1] ?? '')
    .split(',')
    .map((variant) => variant.trim())
    .filter((variant) => variant.length > 0)
    .map((variant) => variant[0].toLowerCase() + variant.slice(1));
  assertEqual(
    STARFINDER_CATALOG_TABS.map((tab) => tab.kind).join(','),
    variants.join(','),
    'the frontend tabs are the command kinds, in order'
  );
}

function testTheViewListsEveryServedRecord() {
  for (const kind of ['race', 'class'] as const) {
    const response = fixture(kind);
    assertEqual(response.kind, kind, `${kind} fixture kind`);
    assertEqual(response.source, 'data/starfinder-1e/sheet_rules', `${kind} rows come from the Starfinder package`);
    assert(response.entries.length > 0 && response.entries.length <= MAX_RENDERED_ENTRIES, `${kind} fits one page`);
    const html = decode(renderToStaticMarkup(createElement(StarfinderCatalogView, { response, query: '' })));
    const rendered = [...html.matchAll(/data-sf-catalog-entry="([^"]+)"/g)].map((match) => match[1]);
    assertEqual(rendered.join(','), response.entries.map((entry) => entry.id).join(','), `${kind}: every record once, in order`);
    const renderedRows = [...html.matchAll(/data-sf-catalog-row="([^"]+)"/g)].map((match) => match[1]);
    assertEqual(
      renderedRows.join(','),
      response.entries.flatMap((entry) => entry.rows.map((row) => row.id)).join(','),
      `${kind}: every total row once`
    );
    for (const entry of response.entries) {
      assert(html.includes(entry.name), `${kind}: ${entry.name} shown`);
      for (const row of entry.rows) {
        assert(html.includes(row.summary), `${kind}: ${row.id} summary shown`);
      }
    }
    const books = new Set(response.entries.map((entry) => entry.book)).size;
    assert(html.includes(`${response.entries.length} ${kind === 'race' ? 'races' : 'classes'} from ${books} books`), `${kind}: summary counts`);
    console.log(`starfinderCatalog: ${kind} ${response.entries.length} records, ${renderedRows.length} total rows rendered`);
  }
  const races = fixture('race');
  const ysoki = races.entries.find((entry) => entry.id === 'core:race:ysoki');
  assertEqual(ysoki?.name, 'Ysoki', 'the race is named by its race ability');
  assert(
    ysoki?.rows.some((row) => row.label === 'Ysoki (racial Hit Points)' && row.summary.startsWith('Value: 2; Adds to hit points')) ?? false,
    'Ysoki carries its racial Hit Points row'
  );
  const classes = fixture('class');
  for (const entry of classes.entries) {
    assert(!(entry.description ?? '').includes('Hit die'), `${entry.name}: no hit die printed`);
  }
}

function testSearchNarrowsTheList() {
  const races = fixture('race');
  const only = filterStarfinderCatalog(races.entries, 'ysoki');
  assertEqual(only.map((entry) => entry.id).join(','), 'core:race:ysoki', 'search by name');
  const html = decode(renderToStaticMarkup(createElement(StarfinderCatalogView, { response: races, query: 'ysoki' })));
  assert(html.includes(`1 of ${races.entries.length} races`), 'the summary says how many matched');
  assertEqual(filterStarfinderCatalog(races.entries, '').length, races.entries.length, 'an empty search shows all');
  assertEqual(bookLabel('alien_archive_2'), 'Alien Archive 2', 'book ids read as words');
}

testTheLandingOffersTheSelectedSystemsCatalogs();
testTheKindsAreTheCommandsKinds();
testTheViewListsEveryServedRecord();
testSearchNarrowsTheList();
console.log('PASS src/starfinderCatalog/starfinderCatalog.test.ts');

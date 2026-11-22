/**
 * SD-38 E6.3 -- the Netrunning tools screen.
 *
 * Proves, over the real corpus files:
 *
 * 1. The Netrunning group carries a Deck Calculator tool tab beside its catalog tabs; the catalog
 *    tabs stay one per MANIFEST non-`print_only` kind.
 * 2. The pickers list the corpus's decks (3), Programs (27) and installable Hardware (6: the
 *    Bodyweight Suit and the Cyberarm option are the deck's extras, not Hardware).
 * 3. The request the screen sends carries the selection and the Interface rank.
 * 4. The view prints exactly the rows the command returned (slots used of available, NET Actions)
 *    and, for a refusal, the engine's message and no slot figure. No number is derived here.
 */
import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { CyberpunkRecord } from '../../boundary/loadCyberpunkCatalog';
import type { CyberpunkDeckResponse } from '../../boundary/cprDeck';
import { CYBERPUNK_CATALOG_TABS, cyberpunkCatalogTab } from '../cyberpunkCatalogModel';
import { CyberpunkDeckView } from './CyberpunkDeckScreen';
import { buildDeckOptions, deckRequest, initialSelection, withoutIndex } from './cyberpunkDeckModel';
import { assert, assertEqual } from '../../testSupport/asserts';

const CORPUS = new URL('../../../../../data/cyberpunk-red/corpus/', import.meta.url);
const read = (file: string) => JSON.parse(readFileSync(new URL(file, CORPUS), 'utf8')) as CyberpunkRecord[];

function decode(text: string): string {
  return text.replace(/&quot;/g, '"').replace(/&#x27;/g, "'").replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&');
}

const options = buildDeckOptions(read('cyberdeck.json'), read('program.json'), read('net_hardware.json'));

function testTheDeckToolTab() {
  const tool = CYBERPUNK_CATALOG_TABS.filter((entry) => entry.tool === 'deck');
  assertEqual(tool.length, 1, 'one deck tool tab');
  assertEqual(tool[0].group, 'netrunning', 'it sits in the Netrunning group');
  assertEqual(cyberpunkCatalogTab(tool[0].kind).title, 'Deck Calculator', 'it is reachable by its kind');
}

function testOptionLists() {
  assertEqual(options.decks.length, 3, '3 cyberdecks');
  assertEqual(options.programs.length, 27, '27 programs');
  assertEqual(options.hardware.length, 6, '6 installable hardware');
  assert(!options.hardware.some((entry) => entry.name === 'Bodyweight Suit' || entry.name.includes('Cyberarm')), 'extras are not hardware');
  assertEqual(initialSelection(options).deck, 'Cyberdeck (Standard Quality)', 'starts on the standard deck');
}

function testRequest() {
  const selection = { ...initialSelection(options), programs: ['Sword', 'Worm'], hardware: ['Range Upgrade'], interfaceRank: 4 };
  const request = deckRequest(selection);
  assertEqual(request.deck, 'Cyberdeck (Standard Quality)', 'deck name');
  assertEqual(request.programs.join(','), 'Sword,Worm', 'programs in order');
  assertEqual(request.hardware.join(','), 'Range Upgrade', 'hardware');
  assertEqual(request.interfaceRank, 4, 'rank');
  assertEqual(withoutIndex(['Sword', 'Sword', 'Worm'], 1).join(','), 'Sword,Worm', 'remove one of two duplicates');
}

const answer: CyberpunkDeckResponse = {
  deck: 'Cyberdeck (Standard Quality)',
  slotsAvailable: 7,
  slotsUsed: 6,
  bodyweightSuitHardware: null,
  netActions: { id: 'cpr.netrunning.net_actions', value: 3, detail: 'Interface rank 4 → 3 NET Actions per Turn' },
  rows: [
    { id: 'cpr.netrunning.deck_slots', value: 7, detail: 'Cyberdeck (Standard Quality) has 7 slots' },
    { id: 'cpr.netrunning.slots_used', value: 6, detail: '6 of 7 slots used' },
  ],
};

function view(response: CyberpunkDeckResponse | null, refusal: string | null) {
  const selection = { ...initialSelection(options), programs: ['Sword', 'Sword'], interfaceRank: 4 };
  return decode(
    renderToStaticMarkup(
      createElement(CyberpunkDeckView, { options, selection, onChange: () => undefined, response, refusal }),
    ),
  );
}

function testTheViewPrintsTheCommandsRows() {
  const html = view(answer, null);
  assert(html.includes('Slots used 6 of 7'), 'slots used of available');
  assert(html.includes('NET Actions per Turn: 3'), 'NET Actions from the engine row');
  for (const row of answer.rows) {
    assert(html.includes(`data-cpr-deck-row="${row.id}"`) && html.includes(row.detail), `${row.id} printed`);
  }
  assert(html.includes('data-cpr-deck-item="Programs:Sword"'), 'chosen programs listed');
  assert(!html.includes('data-cpr-deck-refusal'), 'no refusal');
}

function testARefusalShowsTheEnginesMessage() {
  const message = 'The loadout needs 8 slots; Cyberdeck (Standard Quality) has 7';
  const html = view(null, message);
  assert(html.includes(`data-cpr-deck-refusal`) && html.includes(message), 'engine refusal printed');
  assert(!html.includes('Slots used'), 'no slot figure beside a refusal');
}

testTheDeckToolTab();
testOptionLists();
testRequest();
testTheViewPrintsTheCommandsRows();
testARefusalShowsTheEnginesMessage();
console.log('cyberpunkDeck tests passed');

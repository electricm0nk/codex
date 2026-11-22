import { clickFraction, pageName, pinTarget, wikiPages } from './mapsModel';
import type { CampaignTreeEntry } from '../boundary/campaignFolder/types';
import { assertEqual } from '../testSupport/asserts';

/**
 * SD-42 E2.8 (decisions.md §15.3, Q17, Q-E): a click on the map becomes a pin at fractions of the
 * image's size; a pin opens its wiki page; a pin whose page is missing offers to create it (SD-p).
 */

/** Deep equality through JSON (the shared `assertEqual` compares with `===`). */
const same = (actual: unknown, expected: unknown, message: string) => assertEqual(JSON.stringify(actual), JSON.stringify(expected), message);

const file = (path: string): CampaignTreeEntry => ({ name: path.split('/').pop()!, path, isDir: false, children: [] });
const dir = (path: string, children: CampaignTreeEntry[]): CampaignTreeEntry => ({ name: path.split('/').pop()!, path, isDir: true, children });

const tree: CampaignTreeEntry[] = [
  file('Party Resources.md'),
  dir('Maps', [file('Maps/Sandpoint.png'), file('Maps/Sandpoint.md')]),
  dir('Wiki', [dir('Wiki/Places', [file('Wiki/Places/Riverside Ferry.md')]), dir('Wiki/NPCs', [file('Wiki/NPCs/Ameiko.md'), file('Wiki/NPCs/notes.txt')])]),
];

function testClickFraction() {
  const rect = { left: 100, top: 50, width: 400, height: 200 };
  same(clickFraction(300, 100, rect), { x: 0.5, y: 0.25 }, 'the middle-top quarter');
  same(clickFraction(100 + 400 / 3, 50, rect), { x: 0.3333, y: 0 }, 'rounded to 4 places');
  same(clickFraction(90, 260, rect), { x: 0, y: 1 }, 'a click on the edge stays on the image');
}

function testPinOpensItsPage() {
  // Resolution itself (title first, then stem, any folder) is `resolve_wiki_link`'s: `wiki_resolve_by_title_then_stem_with_alias_and_heading`.
  same(pinTarget('Riverside Ferry', 'Wiki/Places/Riverside Ferry.md'), { kind: 'open', path: 'Wiki/Places/Riverside Ferry.md' }, 'the resolved file opens');
  assertEqual(pageName('ameiko|the innkeeper'), 'ameiko', 'alias dropped');
  assertEqual(pageName(' Ameiko#History '), 'Ameiko', 'a heading link names the page');
}

function testMissingPageOffersCreate() {
  same(pinTarget('Old Light#Top', null), { kind: 'missing', name: 'Old Light' }, 'SD-p: offered for creation by name');
  same(pinTarget('  ', null), { kind: 'none' }, 'a pin with no page');
}

function testWikiPages() {
  same(wikiPages(tree), ['Ameiko', 'Riverside Ferry'], 'the Markdown pages under Wiki/, sorted');
}

testClickFraction();
testPinOpensItsPage();
testMissingPageOffersCreate();
testWikiPages();
console.log('mapsModel: 4 cases passed');

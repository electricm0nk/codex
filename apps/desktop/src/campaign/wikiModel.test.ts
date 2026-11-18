import { completionNames, deleteRefusal, wikiRows } from './wikiModel';
import type { WikiPage } from '../boundary/campaignFolder/wiki';
import type { CampaignTreeEntry } from '../boundary/campaignFolder/types';
import { assertEqual } from '../testSupport/asserts';

/**
 * SD-42 E2.9 (decisions.md §15.4, Q18): the Wiki tab's folder tree with its pages, the `[[`
 * completion names (titles and filename stems), and the refusal that lists a folder's files.
 */

/** Deep equality through JSON (the shared `assertEqual` compares with `===`). */
const same = (actual: unknown, expected: unknown, message: string) => assertEqual(JSON.stringify(actual), JSON.stringify(expected), message);

const page = (path: string, title: string, frontmatterUnread = false): WikiPage => ({ path, title, type: null, tags: [], frontmatterUnread });
const file = (path: string): CampaignTreeEntry => ({ name: path.split('/').pop()!, path, isDir: false, children: [] });
const dir = (path: string, children: CampaignTreeEntry[]): CampaignTreeEntry => ({ name: path.split('/').pop()!, path, isDir: true, children });

function testRowsNestPagesUnderTheirFolders() {
  const rows = wikiRows(
    ['Wiki/NPCs', 'Wiki/Places', 'Wiki/Places/Towns'],
    [page('Wiki/Lore.md', 'Lore'), page('Wiki/NPCs/ameiko.md', 'Ameiko Kaijitsu'), page('Wiki/Places/Towns/Sandpoint.md', 'Sandpoint')]
  );
  same(
    rows.map((row) => `${row.depth}:${row.kind}:${row.kind === 'folder' ? `${row.name} (${row.pages})` : row.title}`),
    ['0:folder:Wiki (1)', '1:page:Lore', '1:folder:NPCs (1)', '2:page:Ameiko Kaijitsu', '1:folder:Places (0)', '2:folder:Towns (1)', '3:page:Sandpoint'],
    'Wiki first, then each folder followed by its own pages, nested by depth'
  );
  same(wikiRows([], []), [{ kind: 'folder', path: 'Wiki', name: 'Wiki', depth: 0, pages: 0 }], 'an empty wiki still offers its root');
}

function testCompletionNamesAreTitlesAndStems() {
  const tree = [file('Party Resources.md'), dir('Wiki', [dir('Wiki/NPCs', [file('Wiki/NPCs/ameiko.md')])]), dir('Maps', [file('Maps/Sandpoint.png')])];
  same(
    completionNames([page('Wiki/NPCs/ameiko.md', 'Ameiko Kaijitsu')], tree),
    ['ameiko', 'Ameiko Kaijitsu', 'Party Resources'],
    'every title and every Markdown stem, once each, sorted'
  );
}

function testDeleteRefusalListsTheFiles() {
  assertEqual(
    deleteRefusal('Wiki/NPCs', ['Wiki/NPCs/Ameiko.md', 'Wiki/NPCs/Old/portrait.png']),
    'Wiki/NPCs was not deleted: it holds 2 files (Wiki/NPCs/Ameiko.md, Wiki/NPCs/Old/portrait.png). Move or delete them first; Codex deletes only an empty folder.',
    'the refusal names every file'
  );
  assertEqual(deleteRefusal('Wiki/Items', ['Wiki/Items/Sword.md']).includes('holds 1 file ('), true, 'one file, singular');
}

function testUnreadFrontmatterRowCarriesTheMark() {
  const rows = wikiRows([], [page('Wiki/Inn.md', 'Inn', true), page('Wiki/Lore.md', 'Lore')]);
  same(
    rows.filter((row) => row.kind === 'page').map((row) => (row.kind === 'page' ? [row.title, row.mark] : null)),
    [['Inn', 'frontmatter not read'], ['Lore', null]],
    'SD-o: a page whose frontmatter Codex could not read is listed by filename with the mark'
  );
}

testRowsNestPagesUnderTheirFolders();
testCompletionNamesAreTitlesAndStems();
testDeleteRefusalListsTheFiles();
testUnreadFrontmatterRowCarriesTheMark();
console.log('wikiModel: 4 cases passed');

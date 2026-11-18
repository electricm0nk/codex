import { markdownStems, PARTY_RESOURCES_PATH } from './partyResourcesModel';
import type { CampaignTreeEntry } from '../../boundary/campaignFolder/types';
import { assertEqual } from '../../testSupport/asserts';

/**
 * SD-42 E2.6 (decisions.md §15.1): the Party Resources tab edits `Party Resources.md` in the
 * editor. `[[` completion offers every Markdown filename stem in the folder. (A clicked link is
 * resolved by `resolve_wiki_link` since E2.9: title first, then stem — `src/campaign/wiki.rs`.)
 */

const file = (path: string): CampaignTreeEntry => ({ name: path.split('/').pop()!, path, isDir: false, children: [] });
const dir = (path: string, children: CampaignTreeEntry[]): CampaignTreeEntry => ({ name: path.split('/').pop()!, path, isDir: true, children });

const TREE: CampaignTreeEntry[] = [
  dir('Attachments', [file('Attachments/sword.png')]),
  dir('Maps', [file('Maps/Sandpoint.png'), file('Maps/Sandpoint.md')]),
  file(PARTY_RESOURCES_PATH),
  dir('Wiki', [dir('Wiki/NPCs', [file('Wiki/NPCs/Ameiko Kaijitsu.md')]), dir('Wiki/Places', [])]),
  file('campaign.json'),
];

function testStemsAreEveryMarkdownFile() {
  assertEqual(markdownStems(TREE).join('|'), 'Sandpoint|Party Resources|Ameiko Kaijitsu', 'md stems in tree order; images and json skipped');
}

testStemsAreEveryMarkdownFile();
console.log('partyResourcesModel: 1 case passed');

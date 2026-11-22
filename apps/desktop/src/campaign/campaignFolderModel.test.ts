import { notACampaignFolder, partyCharacterIds, relocationNotice } from './campaignFolderModel';
import type { CampaignTreeEntry } from '../boundary/campaignFolder/types';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * SD-42 E2.3: the campaign sheet reads the folder. Its party is the characters copied into the
 * folder's `Characters/` (SD-41 wrote `characters/`), matched to this install's saved characters
 * by the `<slug>-<characterId>` folder name. (Each tab's own reader replaced E2.3's read-only file
 * lists: E2.6–E2.9.)
 */

const file = (path: string): CampaignTreeEntry => ({ name: path.split('/').pop()!, path, isDir: false, children: [] });
const dir = (path: string, children: CampaignTreeEntry[]): CampaignTreeEntry => ({ name: path.split('/').pop()!, path, isDir: true, children });

const TREE: CampaignTreeEntry[] = [
  dir('Adventure Log', [file('Adventure Log/2026-11-01 Gwen - Arrival.md'), file('Adventure Log/2026-11-08 Gwen - The Mill.md')]),
  dir('Attachments', []),
  dir('Characters', [dir('Characters/aldric-ironhand-1f0c-aa', [file('Characters/aldric-ironhand-1f0c-aa/bio.json')])]),
  dir('Maps', [file('Maps/Sandpoint.png'), file('Maps/Sandpoint.md')]),
  file('Party Resources.md'),
  dir('Wiki', [dir('Wiki/NPCs', [file('Wiki/NPCs/Ameiko.md')]), dir('Wiki/Places', [])]),
  dir('characters', [dir('characters/elowen-2b', [])]),
  file('campaign.json'),
];

function testPartyIsTheFolderCharactersThisInstallHolds() {
  const ids = partyCharacterIds(TREE, ['1f0c-aa', '2b', 'not-in-folder', 'aa']);
  assertEqual(ids.join(','), '1f0c-aa,2b', 'party = saved ids with a <slug>-<id> folder under Characters/ or characters/');
  assertEqual(partyCharacterIds([], ['1f0c-aa']).length, 0, 'no Characters/ folder -> no party');
}

function testNotACampaignFolderIsRecognised() {
  assert(notACampaignFolder('Failed adding a campaign folder: /x has no campaign.json: it is not a campaign folder'), 'no campaign.json');
  assert(!notACampaignFolder('Failed adding a campaign folder: /x has no campaign.json: it is an older campaign (.config/); migrate it first'), 'an older campaign is not offered New');
  assert(!notACampaignFolder('permission denied'), 'another error');
}

function testRelocationNoticeNamesBothFolders() {
  const base = { campaignDir: '/new/Rise', previousDir: '/old/Rise' };
  assertEqual(
    relocationNotice({ ...base, copiedFiles: 12 }),
    'Moved: 12 files copied to /new/Rise and checked byte for byte. The old folder is kept, unchanged, at /old/Rise; delete it yourself once you no longer need it.',
    'a copy names the count, the new folder and the kept old folder'
  );
  assertEqual(
    relocationNotice({ ...base, copiedFiles: 1 }).startsWith('Moved: 1 file copied'),
    true,
    'one file is singular'
  );
  assertEqual(
    relocationNotice({ ...base, copiedFiles: 0 }),
    '/new/Rise already holds this campaign, so it was opened as is and nothing was copied. The old folder is kept, unchanged, at /old/Rise; delete it yourself once you no longer need it.',
    'a synced copy is opened, nothing copied'
  );
}

testPartyIsTheFolderCharactersThisInstallHolds();
testNotACampaignFolderIsRecognised();
testRelocationNoticeNamesBothFolders();
console.log('campaignFolderModel.test.ts: 3 cases passed');

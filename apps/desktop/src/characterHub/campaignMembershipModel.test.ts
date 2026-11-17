import {
  applyCampaignChoices,
  activeCampaigns,
  campaignChoices,
  campaignFieldText,
  type FolderCampaign,
} from './campaignMembershipModel';
import type { CampaignMembershipDto } from '../boundary/setCharacterCampaigns';
import { assert, assertEqual } from '../testSupport/asserts';

const DRIVE = '/home/todd/Drive/Codex';
const RISE: FolderCampaign = { id: 'c-rise', name: 'Rise of the Runelords', folderName: 'Rise of the Runelords' };
const SKULL: FolderCampaign = { id: 'c-skull', name: 'Skull & Shackles', folderName: 'Skull _ Shackles' };

// Case 1: no Drive folder -> the dialog says so; nothing is listed.
for (const missing of [undefined, null, '', '   ']) {
  const none = campaignChoices(missing, [RISE], ['c-rise'], []);
  assertEqual(none.kind, 'no-folder', `no Drive folder (${JSON.stringify(missing)}) -> no-folder`);
}

// Case 2: add -> a checked campaign becomes an active membership and a party add.
{
  const listed = campaignChoices(DRIVE, [RISE], ['c-rise'], []);
  assert(listed.kind === 'listed', 'a folder lists its campaigns');
  if (listed.kind !== 'listed') throw new Error('unreachable');
  assertEqual(listed.choices.length, 1, 'one campaign listed');
  assertEqual(listed.choices[0].checked, false, 'not a member yet -> unchecked');
  assertEqual(listed.choices[0].campaignDir, '/home/todd/Drive/Codex/Rise of the Runelords', 'dir = <driveFolderPath>/<folderName>');
  const result = applyCampaignChoices([], listed.choices, ['c-rise']);
  assertEqual(JSON.stringify(result.memberships), JSON.stringify([
    { campaignId: 'c-rise', campaignName: 'Rise of the Runelords', campaignDir: '/home/todd/Drive/Codex/Rise of the Runelords', active: true },
  ]), 'add -> one active membership');
  assertEqual(JSON.stringify(result.partyAdds), JSON.stringify(['c-rise']), 'add -> party add');
  assertEqual(result.partyRemoves.length, 0, 'add -> no party remove');
  assertEqual(campaignFieldText(result.memberships), 'Rise of the Runelords', 'the field shows the active name');
}

// Case 3: remove keeps the folder (membership stays, active false) and drops the id from the party.
{
  const before: CampaignMembershipDto[] = [
    { campaignId: 'c-rise', campaignName: 'Rise of the Runelords', campaignDir: '/home/todd/Drive/Codex/Rise of the Runelords', active: true },
  ];
  const listed = campaignChoices(DRIVE + '/', [RISE], ['c-rise'], before);
  if (listed.kind !== 'listed') throw new Error('expected listed');
  assertEqual(listed.choices[0].checked, true, 'an active membership is checked');
  assertEqual(listed.choices[0].campaignDir, '/home/todd/Drive/Codex/Rise of the Runelords', 'a trailing separator is not doubled');
  const result = applyCampaignChoices(before, listed.choices, []);
  assertEqual(result.memberships.length, 1, 'remove keeps the membership row');
  assertEqual(result.memberships[0].active, false, 'remove -> inactive, the folder stays');
  assertEqual(JSON.stringify(result.partyRemoves), JSON.stringify(['c-rise']), 'remove -> party remove');
  assertEqual(result.partyAdds.length, 0, 'remove -> no party add');
  assertEqual(campaignFieldText(result.memberships), '—', 'no active campaign -> the field shows a dash');
  assertEqual(activeCampaigns(result.memberships).length, 0, 'no Open button');
  // Re-checking an inactive membership makes it active again and re-adds the party id.
  const again = applyCampaignChoices(result.memberships, listed.choices, ['c-rise']);
  assertEqual(again.memberships.length, 1, 're-check reuses the row');
  assertEqual(again.memberships[0].active, true, 're-check -> active');
  assertEqual(JSON.stringify(again.partyAdds), JSON.stringify(['c-rise']), 're-check -> party add');
}

// Case 4: two campaigns -> both active, names comma-joined, two Open buttons; unchanged ones are no party change.
{
  const before: CampaignMembershipDto[] = [
    { campaignId: 'c-skull', campaignName: 'Old name', campaignDir: '/old/dir', active: true },
  ];
  const listed = campaignChoices(DRIVE, [RISE, SKULL], ['c-rise', 'c-skull'], before);
  if (listed.kind !== 'listed') throw new Error('expected listed');
  assertEqual(listed.choices.map((choice) => `${choice.campaignId}:${choice.checked}`).join(','), 'c-rise:false,c-skull:true', 'checked follows active memberships');
  const result = applyCampaignChoices(before, listed.choices, ['c-rise', 'c-skull']);
  assertEqual(campaignFieldText(result.memberships), 'Skull & Shackles, Rise of the Runelords', 'two active names, comma-joined');
  assertEqual(activeCampaigns(result.memberships).map((m) => m.campaignId).join(','), 'c-skull,c-rise', 'one Open button per active campaign');
  assertEqual(result.memberships[0].campaignDir, '/home/todd/Drive/Codex/Skull _ Shackles', 'a listed campaign refreshes its name and dir');
  assertEqual(JSON.stringify(result.partyAdds), JSON.stringify(['c-rise']), 'only the newly checked one is a party add');
  assertEqual(result.partyRemoves.length, 0, 'nothing removed');
}

// Case 5: a folder campaign the Campaign Manager does not hold is not offered (its Open button could not open it);
// a membership whose campaign is not listed is kept as it is.
{
  const before: CampaignMembershipDto[] = [
    { campaignId: 'c-gone', campaignName: 'Gone', campaignDir: '/elsewhere/Gone', active: true },
  ];
  const listed = campaignChoices('C:\\Users\\todd\\Drive', [RISE, SKULL], ['c-rise'], before);
  if (listed.kind !== 'listed') throw new Error('expected listed');
  assertEqual(listed.choices.map((choice) => choice.campaignId).join(','), 'c-rise', 'only Manager-held campaigns are offered');
  assertEqual(listed.notInManager, 1, 'the folder-only campaign is counted');
  assertEqual(listed.choices[0].campaignDir, 'C:\\Users\\todd\\Drive\\Rise of the Runelords', 'a Windows folder joins with a backslash');
  const result = applyCampaignChoices(before, listed.choices, []);
  assertEqual(JSON.stringify(result.memberships), JSON.stringify(before), 'an unlisted membership is untouched');
  assertEqual(result.partyAdds.length + result.partyRemoves.length, 0, 'no party change');
}

console.log('campaignMembershipModel: 5 cases passed');

import { applyCampaignChoices, activeCampaigns, campaignChoices, campaignFieldText, standInCampaigns } from './campaignMembershipModel';
import type { CampaignMembershipDto } from '../boundary/setCharacterCampaigns';
import type { KnownCampaign } from '../boundary/campaignFolder/types';
import { assertEqual } from '../testSupport/asserts';

/**
 * SD-42 E2.3: the sheet's Campaign dialog offers the known campaign folders (`list_known_campaigns`),
 * not a Drive folder's listing. A campaign is its folder: a new membership's `campaignId` is the
 * folder path, and an existing membership is matched by `campaignDir`.
 */

const RISE: KnownCampaign = { path: '/home/gwen/Campaigns/Rise of the Runelords', lastOpenedMs: 2, name: 'Rise of the Runelords', error: null };
const SKULL: KnownCampaign = { path: 'C:\\Campaigns\\Skull _ Shackles', lastOpenedMs: 1, name: 'Skull & Shackles', error: null };
const BROKEN: KnownCampaign = { path: '/mnt/usb/Gone', lastOpenedMs: 0, name: null, error: '/mnt/usb/Gone has no campaign.json: it is not a campaign folder' };

// Case 1: add -> a checked folder becomes an active membership whose id and dir are the folder.
{
  const listed = campaignChoices([RISE], []);
  assertEqual(listed.choices.length, 1, 'one campaign listed');
  assertEqual(listed.choices[0].checked, false, 'not a member yet -> unchecked');
  const result = applyCampaignChoices([], listed.choices, [RISE.path]);
  assertEqual(JSON.stringify(result), JSON.stringify([
    { campaignId: RISE.path, campaignName: 'Rise of the Runelords', campaignDir: RISE.path, active: true },
  ]), 'add -> one active membership, id = dir');
  assertEqual(campaignFieldText(result), 'Rise of the Runelords', 'the field shows the active name');
}

// Case 2: remove keeps the row inactive (the copy stays); re-check reuses it.
{
  const before: CampaignMembershipDto[] = [{ campaignId: RISE.path, campaignName: 'Rise of the Runelords', campaignDir: RISE.path, active: true }];
  const listed = campaignChoices([RISE], before);
  assertEqual(listed.choices[0].checked, true, 'an active membership is checked');
  const removed = applyCampaignChoices(before, listed.choices, []);
  assertEqual(removed.length, 1, 'remove keeps the membership row');
  assertEqual(removed[0].active, false, 'remove -> inactive, the folder copy stays');
  assertEqual(campaignFieldText(removed), '—', 'no active campaign -> a dash');
  const again = applyCampaignChoices(removed, listed.choices, [RISE.path]);
  assertEqual(again.length, 1, 're-check reuses the row');
  assertEqual(again[0].active, true, 're-check -> active');
}

// Case 3: an SD-41 membership (a Drive-era id) is matched by its folder and takes the folder as its id and the current name.
{
  const before: CampaignMembershipDto[] = [{ campaignId: 'c-skull', campaignName: 'Old name', campaignDir: SKULL.path, active: true }];
  const listed = campaignChoices([RISE, SKULL], before);
  assertEqual(listed.choices.map((choice) => `${choice.campaignName}:${choice.checked}`).join(','), 'Rise of the Runelords:false,Skull & Shackles:true', 'checked follows the folder');
  const result = applyCampaignChoices(before, listed.choices, [RISE.path, SKULL.path]);
  assertEqual(campaignFieldText(result), 'Skull & Shackles, Rise of the Runelords', 'two active names, comma-joined');
  assertEqual(result[0].campaignId, SKULL.path, 'the old id becomes the folder path, so Open opens the folder');
  assertEqual(activeCampaigns(result).length, 2, 'one Open button per active campaign');
}

// Case 4: a known folder that does not open is counted, not offered; a membership of an unlisted folder is untouched.
{
  const before: CampaignMembershipDto[] = [{ campaignId: 'c-gone', campaignName: 'Gone', campaignDir: '/elsewhere/Gone', active: true }];
  const listed = campaignChoices([RISE, BROKEN], before);
  assertEqual(listed.choices.map((choice) => choice.campaignDir).join(','), RISE.path, 'only folders that open are offered');
  assertEqual(listed.unreadable, 1, 'the folder that does not open is counted');
  assertEqual(JSON.stringify(applyCampaignChoices(before, listed.choices, [])), JSON.stringify(before), 'an unlisted membership is untouched');
}

// Case 5 (SD-42 E4.3): a stand-in membership is marked in the dialog, keeps its mark and counter through a save, and drives the banner.
{
  const before: CampaignMembershipDto[] = [{ campaignId: RISE.path, campaignName: 'Rise of the Runelords', campaignDir: RISE.path, active: true, standIn: true, lastBackupCounter: 3 }];
  const listed = campaignChoices([RISE, SKULL], before);
  assertEqual(listed.choices.map((choice) => `${choice.campaignName}:${choice.standIn}`).join(','), 'Rise of the Runelords:true,Skull & Shackles:false', 'the stand-in folder is marked');
  const result = applyCampaignChoices(before, listed.choices, [RISE.path, SKULL.path]);
  assertEqual(JSON.stringify(result[0]), JSON.stringify(before[0]), 'the stand-in mark and counter ride through the save');
  assertEqual(result[1].standIn, undefined, 'a newly checked folder is never a stand-in');
  assertEqual(standInCampaigns(result).join(','), 'Rise of the Runelords', 'the banner names the campaign');
  assertEqual(standInCampaigns(applyCampaignChoices(before, listed.choices, [])).length, 0, 'an unchecked stand-in shows no banner');
}

console.log('campaignMembershipModel: 5 cases passed');

import { createCampaign, writeCampaignLocalFolderArtifacts } from './campaignModel';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * SD-23 Epic 4 (Campaign Manager Simplification): `createCampaign` no longer
 * returns a `driveActionSummary` string (Criterion 12), `CampaignMember` no
 * longer carries an `invited` field (Criterion 13), and the local-folder
 * write helper is exported as `writeCampaignLocalFolderArtifacts` rather
 * than `syncCampaignDriveArtifacts` (Criterion 14).
 */

async function main() {
  await testCreateCampaignReturnsSyncResultNotDriveActionSummary();
  await testCreateCampaignMembersHaveNoInvitedField();
  testWriteCampaignLocalFolderArtifactsExportedUnderNewName();
}

async function testCreateCampaignReturnsSyncResultNotDriveActionSummary() {
  const result = await createCampaign({
    name: 'Test Campaign',
    ruleSetId: 'core',
    ruleSetLabel: 'Core',
    description: '',
    memberEmails: [],
  });

  assert('campaign' in result, 'createCampaign result must carry campaign');
  assert('syncResult' in result, 'createCampaign result must carry syncResult');
  assert(!('driveActionSummary' in result), 'driveActionSummary must not be present on the createCampaign result');

  assertEqual(typeof result.syncResult.ok, 'boolean', 'syncResult.ok is a boolean');
  // No Tauri runtime exists in this test environment, so the local-folder
  // write is expected to fail deterministically — exercising the
  // ok:false/error branch of syncResult.
  assertEqual(result.syncResult.ok, false, 'no Tauri runtime in test env, so the local-folder write fails');
  assert(
    typeof result.syncResult.error === 'string' && result.syncResult.error.length > 0,
    'syncResult.error is populated when ok is false'
  );
  assertEqual(result.syncResult.campaignFolderPath, undefined, 'campaignFolderPath is absent when ok is false');
}

async function testCreateCampaignMembersHaveNoInvitedField() {
  const result = await createCampaign({
    name: 'Membership Test',
    ruleSetId: 'core',
    ruleSetLabel: 'Core',
    description: '',
    memberEmails: ['player@example.com'],
  });

  assertEqual(result.campaign.members.length, 1, 'one member created from one email');
  const member = result.campaign.members[0];
  assertEqual(member.email, 'player@example.com', 'member email is preserved');
  assert(!('invited' in member), 'CampaignMember must not carry an invited field');
  assertEqual(Object.keys(member).join(','), 'email', 'persisted member shape is {email} only');
}

function testWriteCampaignLocalFolderArtifactsExportedUnderNewName() {
  assertEqual(
    typeof writeCampaignLocalFolderArtifacts,
    'function',
    'writeCampaignLocalFolderArtifacts is exported under its new (non-Drive) name'
  );
}

main()
  .then(() => console.log('campaignModel.test.ts OK'))
  .catch((error: unknown) => {
    console.error(error);
    throw error;
  });

import * as campaignModel from './campaignModel';
import { writeCampaignLocalFolderArtifacts } from './campaignModel';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * SD-42 E2.2: a campaign is created as a folder (`create_campaign_folder`, the creator as
 * member #1), never as a localStorage record with invited emails, so the old `createCampaign`
 * and its `memberEmails` input are gone. The local-folder write helper keeps its SD-23 name
 * until E2.3 removes the Drive path.
 */

function testCreateCampaignIsGone() {
  assert(!('createCampaign' in campaignModel), 'campaignModel no longer creates a localStorage campaign');
}

function testWriteCampaignLocalFolderArtifactsExportedUnderNewName() {
  assertEqual(
    typeof writeCampaignLocalFolderArtifacts,
    'function',
    'writeCampaignLocalFolderArtifacts is exported under its new (non-Drive) name'
  );
}

testCreateCampaignIsGone();
testWriteCampaignLocalFolderArtifactsExportedUnderNewName();
console.log('campaignModel.test.ts OK');

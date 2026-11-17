import { invokeCampaignFolder } from './invokeCampaignFolder';
import type { CampaignFolderSnapshot, KnownCampaign, OpenedCampaign, SavedCampaignJson } from './types';

/** The known campaign folders, most recently opened first (`list_known_campaigns`). */
export function listKnownCampaigns(): Promise<KnownCampaign[]> {
  return invokeCampaignFolder('list_known_campaigns', {}, 'listing campaign folders');
}

/** "Open folder…": opens `campaignDir` as a campaign and remembers it (`add_known_campaign_folder`). */
export function addKnownCampaignFolder(campaignDir: string): Promise<OpenedCampaign> {
  return invokeCampaignFolder('add_known_campaign_folder', { campaignDir }, 'adding a campaign folder');
}

/** Removes `campaignDir` from the known list; the folder itself is untouched. */
export function forgetKnownCampaignFolder(campaignDir: string): Promise<void> {
  return invokeCampaignFolder('forget_known_campaign_folder', { campaignDir }, 'forgetting a campaign folder');
}

/** "New campaign…": creates `<parentDir>/<campaign.name>/` with the campaign layout. */
export function createCampaignFolder(parentDir: string, campaign: CampaignFolderSnapshot): Promise<OpenedCampaign> {
  return invokeCampaignFolder(
    'create_campaign_folder',
    { request: { parentDir, campaign } },
    'creating a campaign folder'
  );
}

/** Opens a known campaign folder and marks it most recently opened. */
export function openCampaignFolder(campaignDir: string): Promise<OpenedCampaign> {
  return invokeCampaignFolder('open_campaign_folder', { campaignDir }, 'opening a campaign folder');
}

/** Saves an edited `campaign.json` through the conflict-detecting nonce path (`save_campaign_json`). */
export function saveCampaignJson(
  campaignDir: string,
  campaign: CampaignFolderSnapshot,
  expectedNonce: number
): Promise<SavedCampaignJson> {
  return invokeCampaignFolder(
    'save_campaign_json',
    { request: { campaignDir, campaign, expectedNonce } },
    'saving campaign.json'
  );
}

import { invokeCampaignFolder } from './invokeCampaignFolder';
import type { CampaignFile, CampaignFileStat, CampaignTreeEntry } from './types';

/** Reads one file inside the campaign folder, byte for byte (`read_campaign_file`). */
export async function readCampaignFile(campaignDir: string, path: string): Promise<CampaignFile> {
  const file = await invokeCampaignFolder<{ bytes: number[]; modifiedMs: number }>(
    'read_campaign_file',
    { request: { campaignDir, path } },
    `reading ${path}`
  );
  return { bytes: Uint8Array.from(file.bytes), modifiedMs: file.modifiedMs };
}

/** Writes one file inside the campaign folder atomically; returns its new stat. */
export function writeCampaignFile(campaignDir: string, path: string, bytes: Uint8Array): Promise<CampaignFileStat> {
  return invokeCampaignFolder(
    'write_campaign_file',
    { request: { campaignDir, path, bytes: Array.from(bytes) } },
    `writing ${path}`
  );
}

export function statCampaignFile(campaignDir: string, path: string): Promise<CampaignFileStat> {
  return invokeCampaignFolder('stat_campaign_file', { request: { campaignDir, path } }, `checking ${path}`);
}

/** The whole folder as a tree, sorted by name (`list_campaign_tree`). */
export function listCampaignTree(campaignDir: string): Promise<CampaignTreeEntry[]> {
  return invokeCampaignFolder('list_campaign_tree', { campaignDir }, 'listing the campaign folder');
}

/** "Create starter file": writes the Party Resources starter into a folder that has none (`create_party_resources_starter`). */
export function createPartyResourcesStarter(campaignDir: string): Promise<CampaignFileStat> {
  return invokeCampaignFolder('create_party_resources_starter', { campaignDir }, 'creating Party Resources.md');
}

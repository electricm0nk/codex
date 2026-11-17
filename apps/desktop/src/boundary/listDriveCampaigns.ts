import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read desktop boundary for the campaigns in the Drive folder (`drive_list_campaigns`,
 * `campaign_drive.rs`): each `<driveFolderPath>/<folderName>/.config/<name>.json` the folder holds.
 */

export interface DriveCampaignSummaryDto {
  id: string;
  name: string;
  ruleSetLabel: string;
  updatedAt: string;
  partySize: number;
  /** The directory under the Drive folder this campaign lives in. */
  folderName: string;
}

export interface DriveListCampaignsResponse {
  campaigns: DriveCampaignSummaryDto[];
  /** Folder entries that are not readable campaigns. */
  unreadableCount: number;
}

export async function listDriveCampaigns(driveFolderPath: string): Promise<DriveListCampaignsResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for listing campaigns');
  }

  try {
    return await invoke<DriveListCampaignsResponse>('drive_list_campaigns', { request: { driveFolderPath } });
  } catch (cause: unknown) {
    throw new Error(`Failed to list campaigns: ${formatError(cause)}`);
  }
}

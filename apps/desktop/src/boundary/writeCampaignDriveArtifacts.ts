import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Real local-disk write-through for a campaign's Drive artifacts — a folder
 * per campaign under the configured Drive folder path, `.config/<name>.json`,
 * and markdown files for resources/adventure-log/maps/wiki. Replaces the old
 * simulated "Would create Drive folder..." string with real file I/O.
 */

export interface MarkdownAssetDto {
  title: string;
  body: string;
}

export interface CampaignAssetsDto {
  resources: MarkdownAssetDto[];
  adventureLog: MarkdownAssetDto[];
  maps: MarkdownAssetDto[];
  wiki: MarkdownAssetDto[];
}

export interface WriteCampaignDriveArtifactsRequest {
  driveFolderPath: string;
  campaignName: string;
  campaignConfigJson: string;
  assets: CampaignAssetsDto;
}

export interface WriteCampaignDriveArtifactsResponse {
  campaignFolderPath: string;
}

export async function writeCampaignDriveArtifacts(
  request: WriteCampaignDriveArtifactsRequest
): Promise<WriteCampaignDriveArtifactsResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for writing campaign Drive artifacts');
  }

  try {
    return await invoke<WriteCampaignDriveArtifactsResponse>('write_campaign_drive_artifacts', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to write campaign Drive artifacts: ${formatError(cause)}`);
  }
}

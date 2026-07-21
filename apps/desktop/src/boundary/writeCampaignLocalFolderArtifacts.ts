import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Real local-disk write-through for a campaign's artifacts — a folder per
 * campaign under the local folder path the user configured (Settings →
 * local folder), `.config/<name>.json`, and markdown files for
 * resources/adventure-log/maps/wiki. This is a plain local-filesystem write;
 * the app has no Google Drive/OAuth integration and never talks to a cloud
 * API — the underlying Tauri command name (`write_campaign_drive_artifacts`)
 * is legacy and unrelated to any Drive API call.
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

export interface WriteCampaignLocalFolderArtifactsRequest {
  driveFolderPath: string;
  campaignName: string;
  campaignConfigJson: string;
  assets: CampaignAssetsDto;
}

export interface WriteCampaignLocalFolderArtifactsResponse {
  campaignFolderPath: string;
}

export async function writeCampaignLocalFolderArtifacts(
  request: WriteCampaignLocalFolderArtifactsRequest
): Promise<WriteCampaignLocalFolderArtifactsResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for writing campaign local-folder artifacts');
  }

  try {
    return await invoke<WriteCampaignLocalFolderArtifactsResponse>('write_campaign_drive_artifacts', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to write campaign local-folder artifacts: ${formatError(cause)}`);
  }
}

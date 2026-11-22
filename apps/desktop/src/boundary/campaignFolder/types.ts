/**
 * Wire types of the campaign-folder commands (`campaign_folder.rs`, SD-42 `decisions.md §11`).
 * Paths inside a campaign are `/`-separated and relative to the campaign folder.
 */

export interface FolderMember {
  name: string;
  addedAt: string;
}

/** `campaign.json` at the campaign folder's root (schema 2). */
export interface CampaignFolderSnapshot {
  schemaVersion: 2;
  name: string;
  ruleSetId: string;
  ruleSetLabel: string;
  description: string;
  members: FolderMember[];
  createdAt: string;
  updatedAt: string;
}

export interface OpenedCampaign {
  campaignDir: string;
  campaign: CampaignFolderSnapshot;
  /** Pass back as `expectedNonce` when saving `campaign.json`. */
  nonce: number;
}

/** `relocate_campaign_folder`: the campaign at its new folder; the old folder is kept as it was. */
export interface RelocatedCampaign extends OpenedCampaign {
  previousDir: string;
  /** 0 when the new folder already held this campaign (a synced copy) and was opened as is. */
  copiedFiles: number;
}

/** One entry of `<app data>/campaign-folders.json`: `name` when it opens, else `error`. */
export interface KnownCampaign {
  path: string;
  lastOpenedMs: number;
  name: string | null;
  error: string | null;
}

export interface CampaignFile {
  bytes: Uint8Array;
  modifiedMs: number;
}

export interface CampaignFileStat {
  exists: boolean;
  isDir: boolean;
  size: number;
  modifiedMs: number;
}

export interface CampaignTreeEntry {
  name: string;
  path: string;
  isDir: boolean;
  children: CampaignTreeEntry[];
}

export interface SavedCampaignJson {
  nonce: number;
  /** Set when someone else saved since the campaign was opened: their copy was kept here. */
  conflictDir: string | null;
}

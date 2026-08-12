/**
 * Campaign data model and local persistence.
 *
 * The campaign record itself, its party, and its resources/adventure-log/
 * maps/wiki assets are the source of truth in localStorage (this module) —
 * that part works fully offline and needs no cloud connection.
 *
 * By design, this app never talks to a cloud API (see settings/googleDrive.ts)
 * — the "Drive folder" is just a local path (typically a Drive/Dropbox/Syncthing
 * desktop-sync-client mirror, or any plain local folder). Sharing a campaign
 * means handing someone that folder or its `.json`/`.md` files directly; the
 * app itself does no network calls, OAuth, or invite delivery.
 * `writeCampaignLocalFolderArtifacts` below writes an actual folder per
 * campaign at `<driveFolderPath>/<name>/`, with `.config/<name>.json` and
 * markdown files for resources/adventure-log/maps/wiki, via the
 * `write_campaign_drive_artifacts` Tauri command
 * (apps/desktop/src-tauri/src/campaign_drive.rs). This is a write-through
 * mirror of the localStorage data, not itself the source of truth — if the
 * write fails (no folder configured, disk error), the campaign still exists
 * and works entirely from localStorage.
 */

import { getGoogleDriveConfig } from '../settings/googleDrive';
import { writeCampaignLocalFolderArtifacts as writeCampaignLocalFolderArtifactsToDisk } from '../boundary/writeCampaignLocalFolderArtifacts';

export interface CampaignMember {
  email: string;
}

export interface Campaign {
  id: string;
  name: string;
  ruleSetId: string;
  ruleSetLabel: string;
  description: string;
  members: CampaignMember[];
  /** Saved-character ids that make up the active party. */
  partyCharacterIds: string[];
  createdAt: string;
  updatedAt: string;
}

/** One markdown-backed asset — party resources, adventure log entries, maps, and wiki pages all share this shape. */
export interface MarkdownAsset {
  id: string;
  title: string;
  body: string;
  updatedAt: string;
}

export interface CampaignAssets {
  resources: MarkdownAsset[];
  adventureLog: MarkdownAsset[];
  maps: MarkdownAsset[];
  wiki: MarkdownAsset[];
}

const CAMPAIGNS_KEY = 'codex.campaigns';
const ASSETS_KEY_PREFIX = 'codex.campaign.assets.';

export function getCampaigns(): Campaign[] {
  try {
    const raw = localStorage.getItem(CAMPAIGNS_KEY);
    return raw ? (JSON.parse(raw) as Campaign[]) : [];
  } catch {
    return [];
  }
}

function saveCampaigns(campaigns: Campaign[]): void {
  try {
    localStorage.setItem(CAMPAIGNS_KEY, JSON.stringify(campaigns));
  } catch {
    /* non-persistent environments still get the change applied this session */
  }
}

export function getCampaign(id: string): Campaign | null {
  return getCampaigns().find((campaign) => campaign.id === id) ?? null;
}

export interface CreateCampaignInput {
  name: string;
  ruleSetId: string;
  ruleSetLabel: string;
  description: string;
  memberEmails: string[];
}

/**
 * Creates the campaign record (localStorage) and immediately writes its
 * local-folder artifacts to disk — nothing here (or anywhere in this app)
 * sends an email, invite, or notification of any kind. Sharing a campaign
 * means handing someone its folder or files directly. `syncResult` reports
 * whether the local-folder write succeeded; the campaign record itself is
 * always created regardless of `syncResult.ok`, since localStorage is the
 * actual source of truth.
 */
export async function createCampaign(
  input: CreateCampaignInput
): Promise<{ campaign: Campaign; syncResult: WriteCampaignLocalFolderArtifactsResult }> {
  const now = new Date().toISOString();
  const campaign: Campaign = {
    id: crypto.randomUUID(),
    name: input.name,
    ruleSetId: input.ruleSetId,
    ruleSetLabel: input.ruleSetLabel,
    description: input.description,
    members: input.memberEmails.map((email) => ({ email })),
    partyCharacterIds: [],
    createdAt: now,
    updatedAt: now,
  };
  saveCampaigns([...getCampaigns(), campaign]);
  const syncResult = await writeCampaignLocalFolderArtifacts(campaign.id);
  return { campaign, syncResult };
}

export function updateCampaign(id: string, changes: Partial<Omit<Campaign, 'id' | 'createdAt'>>): Campaign | null {
  const campaigns = getCampaigns();
  const index = campaigns.findIndex((campaign) => campaign.id === id);
  if (index === -1) {
    return null;
  }
  const updated: Campaign = { ...campaigns[index], ...changes, updatedAt: new Date().toISOString() };
  campaigns[index] = updated;
  saveCampaigns(campaigns);
  return updated;
}

export function deleteCampaign(id: string): void {
  saveCampaigns(getCampaigns().filter((campaign) => campaign.id !== id));
  try {
    localStorage.removeItem(ASSETS_KEY_PREFIX + id);
  } catch {
    /* ignore */
  }
}

const EMPTY_ASSETS: CampaignAssets = { resources: [], adventureLog: [], maps: [], wiki: [] };

export function getCampaignAssets(campaignId: string): CampaignAssets {
  try {
    const raw = localStorage.getItem(ASSETS_KEY_PREFIX + campaignId);
    return raw ? (JSON.parse(raw) as CampaignAssets) : { ...EMPTY_ASSETS };
  } catch {
    return { ...EMPTY_ASSETS };
  }
}

function saveCampaignAssets(campaignId: string, assets: CampaignAssets): void {
  try {
    localStorage.setItem(ASSETS_KEY_PREFIX + campaignId, JSON.stringify(assets));
  } catch {
    /* ignore */
  }
}

export type CampaignAssetKind = keyof CampaignAssets;

/** Fire-and-forget: mirrors current localStorage state to disk. localStorage stays the source of truth regardless of outcome. */
function writeCampaignLocalFolderArtifactsInBackground(campaignId: string): void {
  writeCampaignLocalFolderArtifacts(campaignId).catch(() => {
    /* best-effort mirror — see module doc comment */
  });
}

export function addCampaignAsset(campaignId: string, kind: CampaignAssetKind, title: string): MarkdownAsset {
  const assets = getCampaignAssets(campaignId);
  const asset: MarkdownAsset = { id: crypto.randomUUID(), title, body: '', updatedAt: new Date().toISOString() };
  assets[kind] = [...assets[kind], asset];
  saveCampaignAssets(campaignId, assets);
  writeCampaignLocalFolderArtifactsInBackground(campaignId);
  return asset;
}

export function updateCampaignAsset(campaignId: string, kind: CampaignAssetKind, assetId: string, changes: Partial<Pick<MarkdownAsset, 'title' | 'body'>>): void {
  const assets = getCampaignAssets(campaignId);
  assets[kind] = assets[kind].map((asset) =>
    asset.id === assetId ? { ...asset, ...changes, updatedAt: new Date().toISOString() } : asset
  );
  saveCampaignAssets(campaignId, assets);
  writeCampaignLocalFolderArtifactsInBackground(campaignId);
}

export function deleteCampaignAsset(campaignId: string, kind: CampaignAssetKind, assetId: string): void {
  const assets = getCampaignAssets(campaignId);
  assets[kind] = assets[kind].filter((asset) => asset.id !== assetId);
  saveCampaignAssets(campaignId, assets);
  writeCampaignLocalFolderArtifactsInBackground(campaignId);
}

export interface WriteCampaignLocalFolderArtifactsResult {
  ok: boolean;
  campaignFolderPath?: string;
  error?: string;
}

/**
 * Writes the campaign's real, current localStorage state (record + all
 * assets) to disk under the local folder the user configured (Settings →
 * local folder path) — see the module doc comment above. Safe to call any
 * time the campaign or its assets change; each call rewrites the full
 * current state, so it's idempotent.
 */
export async function writeCampaignLocalFolderArtifacts(campaignId: string): Promise<WriteCampaignLocalFolderArtifactsResult> {
  const campaign = getCampaign(campaignId);
  if (!campaign) {
    return { ok: false, error: 'Campaign not found.' };
  }

  const driveFolderPath = getGoogleDriveConfig()?.driveFolderPath ?? '';
  const assets = getCampaignAssets(campaignId);
  const toAssetDto = (list: MarkdownAsset[]) => list.map((asset) => ({ title: asset.title, body: asset.body }));

  try {
    const response = await writeCampaignLocalFolderArtifactsToDisk({
      driveFolderPath,
      campaignName: campaign.name,
      campaignConfigJson: JSON.stringify(campaign, null, 2),
      assets: {
        resources: toAssetDto(assets.resources),
        adventureLog: toAssetDto(assets.adventureLog),
        maps: toAssetDto(assets.maps),
        wiki: toAssetDto(assets.wiki),
      },
    });
    return { ok: true, campaignFolderPath: response.campaignFolderPath };
  } catch (cause: unknown) {
    return { ok: false, error: cause instanceof Error ? cause.message : 'Failed to write campaign local folder artifacts.' };
  }
}

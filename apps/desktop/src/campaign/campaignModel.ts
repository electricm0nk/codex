/**
 * Campaign data model and local persistence.
 *
 * The real design calls for a Google Drive-backed campaign: a Drive folder
 * per campaign, a `.config/<campaign_name>.json` metadata file alongside
 * synced character-sheet JSON files, party resources / adventure logs /
 * maps / a wiki as markdown files, all in that Drive folder. None of that
 * exists yet — it needs the Google Drive integration (see settings/googleDrive.ts)
 * plus real file I/O from the backend. This module gives the UI a real,
 * locally-persisted stand-in for all of it (localStorage today) so the full
 * click-through flow — create, edit, build a party, load, manage resources —
 * works end-to-end now and can be swapped for real Drive storage later
 * without changing the screens built against it.
 */

export interface CampaignMember {
  email: string;
  /** Whether a Drive share invite would have gone out to this address. */
  invited: boolean;
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
 * Creates the campaign record and reports what the real Drive integration
 * would additionally do: create `<driveFolderPath>/<name>/`, write
 * `.config/<name>.json`, and send share invites to each member. That part
 * is simulated (returned as a description), not performed.
 */
export function createCampaign(input: CreateCampaignInput, driveFolderPath: string | null): { campaign: Campaign; driveActionSummary: string } {
  const now = new Date().toISOString();
  const campaign: Campaign = {
    id: crypto.randomUUID(),
    name: input.name,
    ruleSetId: input.ruleSetId,
    ruleSetLabel: input.ruleSetLabel,
    description: input.description,
    members: input.memberEmails.map((email) => ({ email, invited: true })),
    partyCharacterIds: [],
    createdAt: now,
    updatedAt: now,
  };
  saveCampaigns([...getCampaigns(), campaign]);

  const folderPath = driveFolderPath ? `${driveFolderPath}/${input.name}` : input.name;
  const driveActionSummary = input.memberEmails.length
    ? `Would create Drive folder "${folderPath}", write .config/${input.name}.json, and invite: ${input.memberEmails.join(', ')}.`
    : `Would create Drive folder "${folderPath}" and write .config/${input.name}.json.`;

  return { campaign, driveActionSummary };
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

export function addCampaignAsset(campaignId: string, kind: CampaignAssetKind, title: string): MarkdownAsset {
  const assets = getCampaignAssets(campaignId);
  const asset: MarkdownAsset = { id: crypto.randomUUID(), title, body: '', updatedAt: new Date().toISOString() };
  assets[kind] = [...assets[kind], asset];
  saveCampaignAssets(campaignId, assets);
  return asset;
}

export function updateCampaignAsset(campaignId: string, kind: CampaignAssetKind, assetId: string, changes: Partial<Pick<MarkdownAsset, 'title' | 'body'>>): void {
  const assets = getCampaignAssets(campaignId);
  assets[kind] = assets[kind].map((asset) =>
    asset.id === assetId ? { ...asset, ...changes, updatedAt: new Date().toISOString() } : asset
  );
  saveCampaignAssets(campaignId, assets);
}

export function deleteCampaignAsset(campaignId: string, kind: CampaignAssetKind, assetId: string): void {
  const assets = getCampaignAssets(campaignId);
  assets[kind] = assets[kind].filter((asset) => asset.id !== assetId);
  saveCampaignAssets(campaignId, assets);
}

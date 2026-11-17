import type { CampaignTreeEntry } from '../boundary/campaignFolder/types';

/**
 * What the campaign sheet shows of a campaign folder (SD-42 `decisions.md §11`). Pure; the sheet
 * reads the tree with `list_campaign_tree`.
 */

export type CampaignTab = 'resources' | 'adventureLog' | 'maps' | 'wiki';

export const CAMPAIGN_TABS: ReadonlyArray<{ tab: CampaignTab; label: string }> = [
  { tab: 'resources', label: 'Party Resources' },
  { tab: 'adventureLog', label: 'Adventure Log' },
  { tab: 'maps', label: 'Maps' },
  { tab: 'wiki', label: 'Wiki' },
];

/** The top-level entry each tab reads. */
const TAB_ROOT: Record<CampaignTab, string> = {
  resources: 'Party Resources.md',
  adventureLog: 'Adventure Log',
  maps: 'Maps',
  wiki: 'Wiki',
};

function filesUnder(entry: CampaignTreeEntry): string[] {
  return entry.isDir ? entry.children.flatMap(filesUnder) : [entry.path];
}

/** Every file of the tab's part of the folder, in tree order. */
export function tabFiles(tree: readonly CampaignTreeEntry[], tab: CampaignTab): string[] {
  const root = tree.find((entry) => entry.path === TAB_ROOT[tab]);
  return root ? filesUnder(root) : [];
}

/**
 * The party: the saved characters of this install with a `<slug>-<characterId>` copy under
 * `Characters/` (or SD-41's `characters/`), in `savedIds` order.
 */
export function partyCharacterIds(tree: readonly CampaignTreeEntry[], savedIds: readonly string[]): string[] {
  const folders = tree
    .filter((entry) => entry.isDir && (entry.name === 'Characters' || entry.name === 'characters'))
    .flatMap((entry) => entry.children.filter((child) => child.isDir).map((child) => child.name));
  // A folder belongs to the longest saved id it ends with ("…-1f0c-aa" is 1f0c-aa's, not aa's).
  const owners = new Set(
    folders.map((name) =>
      savedIds.filter((id) => name.endsWith(`-${id}`)).reduce((best, id) => (id.length > best.length ? id : best), '')
    )
  );
  return savedIds.filter((id) => owners.has(id));
}

/** "Open folder…" picked a folder with no `campaign.json` that is not an older campaign either. */
export function notACampaignFolder(message: string): boolean {
  return message.includes('it is not a campaign folder');
}

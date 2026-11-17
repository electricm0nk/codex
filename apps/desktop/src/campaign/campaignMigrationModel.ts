/**
 * Which pre-SD-42 campaigns are waiting to be migrated (`decisions.md §14`), and clearing one once
 * `migrate_campaign` has written its folder and its backup. Pure over a Storage-shaped object.
 */

type KeyValueStore = Pick<Storage, 'getItem' | 'setItem' | 'removeItem'>;

const CAMPAIGNS_KEY = 'codex.campaigns';
const ASSETS_KEY_PREFIX = 'codex.campaign.assets.';

export interface LegacyMigrationRow {
  id: string;
  name: string;
  /** The old record exactly as stored, re-serialised one campaign at a time. */
  recordJson: string;
  /** `codex.campaign.assets.<id>` exactly as stored, or null when the campaign had none. */
  assetsJson: string | null;
}

function readRecords(storage: KeyValueStore): Array<Record<string, unknown>> {
  try {
    const parsed: unknown = JSON.parse(storage.getItem(CAMPAIGNS_KEY) ?? '[]');
    return Array.isArray(parsed) ? (parsed as Array<Record<string, unknown>>) : [];
  } catch {
    return [];
  }
}

/** One row per old localStorage campaign; none when there is no old data. */
export function legacyMigrationRows(storage: KeyValueStore): LegacyMigrationRow[] {
  return readRecords(storage).map((record) => {
    const id = String(record.id ?? '');
    return {
      id,
      name: String(record.name ?? id),
      recordJson: JSON.stringify(record),
      assetsJson: storage.getItem(ASSETS_KEY_PREFIX + id),
    };
  });
}

/** Removes a migrated campaign's record and entries; the list key goes with the last one. */
export function clearMigrated(storage: KeyValueStore, id: string): void {
  const left = readRecords(storage).filter((record) => String(record.id ?? '') !== id);
  if (left.length === 0) {
    storage.removeItem(CAMPAIGNS_KEY);
  } else {
    storage.setItem(CAMPAIGNS_KEY, JSON.stringify(left));
  }
  storage.removeItem(ASSETS_KEY_PREFIX + id);
}

/** "Open folder…" picked an older `.config/` campaign (the refusal `folder::open` gives it). */
export function olderCampaignFolder(message: string): boolean {
  return message.includes('it is an older campaign (.config/)');
}

import { invokeCampaignFolder } from './invokeCampaignFolder';

/** The old campaign to migrate: a localStorage record (+ its entries), or an old `.config/` folder. */
export type MigrationSource =
  | { recordJson: string; assetsJson: string | null }
  | { configDir: string };

export interface MigratedCampaign {
  campaignDir: string;
  backupFile: string;
  /** Markdown files written besides `Party Resources.md`. */
  files: number;
}

/**
 * `migrate_campaign` (`decisions.md §14`): writes `<parentDir>/<campaign name>/` with the campaign
 * layout, one file per old entry, and the old data to `<app data>/campaign-migration-backup-<date>.json`.
 */
export function migrateCampaign(source: MigrationSource, parentDir: string, yourName: string): Promise<MigratedCampaign> {
  return invokeCampaignFolder(
    'migrate_campaign',
    { request: { ...source, parentDir, yourName, now: new Date().toISOString() } },
    'migrating a campaign'
  );
}

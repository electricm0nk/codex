import { invokeCampaignFolder } from './invokeCampaignFolder';

/** `session`: the real play date, one day or a range (`src/campaign/adventure_log.rs` `DateOrRange`). */
export interface DateOrRange {
  start: string;
  end: string | null;
}

/** One `Adventure Log/*.md` file as `list_adventure_log` reads it. */
export interface AdventureLogEntry {
  path: string;
  /** The frontmatter's `title`, else the filename stem. */
  title: string;
  author: string | null;
  posted: string | null;
  session: DateOrRange | null;
  inGameDate: string | null;
  /** False when the file has no frontmatter or Codex cannot read it (SD-o). */
  frontmatterOk: boolean;
}

/** The log, newest session first (`list_adventure_log`). */
export function listAdventureLog(campaignDir: string): Promise<AdventureLogEntry[]> {
  return invokeCampaignFolder('list_adventure_log', { campaignDir }, 'listing the Adventure Log');
}

/** "+": writes a new entry with the default frontmatter; resolves to its path in the campaign. */
export function newAdventureLogEntry(campaignDir: string, author: string, title: string, today: string): Promise<string> {
  return invokeCampaignFolder(
    'new_adventure_log_entry',
    { request: { campaignDir, author, title, today } },
    'creating an Adventure Log entry'
  );
}

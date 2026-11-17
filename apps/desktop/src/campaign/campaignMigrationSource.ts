/**
 * The pre-SD-42 campaigns: records kept in this install's localStorage before a campaign became a
 * folder (`decisions.md §11`). Read only — nothing creates or changes one any more; E2.4's migration
 * turns each into a campaign folder. The print dialog and the DM console still list them until they
 * read the campaign folders (discovered card E2.14).
 */

export interface LegacyCampaign {
  id: string;
  name: string;
  ruleSetId: string;
  ruleSetLabel: string;
  description: string;
  members: Array<{ email: string }>;
  /** Saved-character ids that made up the party. */
  partyCharacterIds: string[];
  createdAt: string;
  updatedAt: string;
}

const LEGACY_CAMPAIGNS_KEY = 'codex.campaigns';

export function getLegacyCampaigns(): LegacyCampaign[] {
  try {
    const raw = localStorage.getItem(LEGACY_CAMPAIGNS_KEY);
    return raw ? (JSON.parse(raw) as LegacyCampaign[]) : [];
  } catch {
    return [];
  }
}

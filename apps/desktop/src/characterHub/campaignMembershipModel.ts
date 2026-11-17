import type { CampaignMembershipDto } from '../boundary/setCharacterCampaigns';
import type { KnownCampaign } from '../boundary/campaignFolder/types';

/**
 * The sheet's campaign membership (SD-41 `decisions.md §29`, SD-42 `§11`): which campaign folders
 * the Campaign dialog offers, and what its checkboxes change. A campaign is its folder, so a
 * membership is matched by `campaignDir` and a new one takes the folder path as its `campaignId`.
 * Pure; the dialog does the write.
 */

export interface CampaignChoice {
  campaignName: string;
  campaignDir: string;
  /** Checked = an active membership. */
  checked: boolean;
}

export interface CampaignChoices {
  choices: CampaignChoice[];
  /** Known folders that do not open as a campaign (moved, unplugged, not migrated): not offered. */
  unreadable: number;
}

export function campaignChoices(known: readonly KnownCampaign[], memberships: readonly CampaignMembershipDto[]): CampaignChoices {
  const offered = known.filter((campaign) => campaign.name !== null);
  return {
    unreadable: known.length - offered.length,
    choices: offered.map((campaign) => ({
      campaignName: campaign.name ?? campaign.path,
      campaignDir: campaign.path,
      checked: memberships.some((membership) => membership.campaignDir === campaign.path && membership.active),
    })),
  };
}

/**
 * The checkbox state (the checked folder paths) → the memberships to save. A listed folder's row
 * takes the folder's current name and the folder as its id; unchecking keeps the row inactive (the
 * copy stays, saves stop); a membership whose folder is not listed is kept as it is.
 */
export function applyCampaignChoices(
  memberships: readonly CampaignMembershipDto[],
  choices: readonly CampaignChoice[],
  checkedDirs: readonly string[]
): CampaignMembershipDto[] {
  const next = memberships.map((membership) => ({ ...membership }));
  for (const choice of choices) {
    const active = checkedDirs.includes(choice.campaignDir);
    const existing = next.find((membership) => membership.campaignDir === choice.campaignDir);
    const row = { campaignId: choice.campaignDir, campaignName: choice.campaignName, campaignDir: choice.campaignDir, active };
    if (existing) {
      Object.assign(existing, row);
    } else if (active) {
      next.push(row);
    }
  }
  return next;
}

export function activeCampaigns(memberships: readonly CampaignMembershipDto[]): CampaignMembershipDto[] {
  return memberships.filter((membership) => membership.active);
}

/** The sheet's Campaign field: the active names comma-joined, or a dash. */
export function campaignFieldText(memberships: readonly CampaignMembershipDto[]): string {
  const names = activeCampaigns(memberships).map((membership) => membership.campaignName);
  return names.length ? names.join(', ') : '—';
}

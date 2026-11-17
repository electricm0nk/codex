import type { CampaignMembershipDto } from '../boundary/setCharacterCampaigns';

/**
 * The sheet's campaign membership (SD-41 `decisions.md §29`): which campaigns the
 * Campaign dialog offers, and what its checkboxes change. Pure; the dialog does the writes.
 */

/** One campaign found in the Drive folder (`drive_list_campaigns`). */
export interface FolderCampaign {
  id: string;
  name: string;
  /** The directory under the Drive folder the campaign lives in. */
  folderName: string;
}

export interface CampaignChoice {
  campaignId: string;
  campaignName: string;
  campaignDir: string;
  /** Checked = an active membership. */
  checked: boolean;
}

export type CampaignChoices =
  | { kind: 'no-folder' }
  /** `notInManager`: folder campaigns this app's Campaign Manager does not hold (not offered: "Open" could not open them). */
  | { kind: 'listed'; choices: CampaignChoice[]; notInManager: number };

export interface CampaignChoicesApplied {
  memberships: CampaignMembershipDto[];
  /** Campaign ids whose party gains the character. */
  partyAdds: string[];
  /** Campaign ids whose party loses the character. */
  partyRemoves: string[];
}

/** `<driveFolderPath>/<folderName>`, with the folder path's own separator. */
function campaignDir(driveFolderPath: string, folderName: string): string {
  const separator = driveFolderPath.includes('\\') && !driveFolderPath.includes('/') ? '\\' : '/';
  return `${driveFolderPath.replace(/[\\/]+$/, '')}${separator}${folderName}`;
}

export function campaignChoices(
  driveFolderPath: string | null | undefined,
  folderCampaigns: readonly FolderCampaign[],
  managerCampaignIds: readonly string[],
  memberships: readonly CampaignMembershipDto[]
): CampaignChoices {
  const folder = driveFolderPath?.trim() ?? '';
  if (!folder) {
    return { kind: 'no-folder' };
  }
  const offered = folderCampaigns.filter((campaign) => managerCampaignIds.includes(campaign.id));
  return {
    kind: 'listed',
    notInManager: folderCampaigns.length - offered.length,
    choices: offered.map((campaign) => ({
      campaignId: campaign.id,
      campaignName: campaign.name,
      campaignDir: campaignDir(folder, campaign.folderName),
      checked: memberships.some((membership) => membership.campaignId === campaign.id && membership.active),
    })),
  };
}

/**
 * The checkbox state → the memberships to save and the party changes. A listed campaign's row takes
 * the folder's current name and dir; unchecking keeps the row inactive (the copy stays, saves stop);
 * a membership whose campaign is not listed is kept as it is.
 */
export function applyCampaignChoices(
  memberships: readonly CampaignMembershipDto[],
  choices: readonly CampaignChoice[],
  checkedIds: readonly string[]
): CampaignChoicesApplied {
  const next = memberships.map((membership) => ({ ...membership }));
  const partyAdds: string[] = [];
  const partyRemoves: string[] = [];
  for (const choice of choices) {
    const active = checkedIds.includes(choice.campaignId);
    const existing = next.find((membership) => membership.campaignId === choice.campaignId);
    const wasActive = existing?.active ?? false;
    if (existing) {
      Object.assign(existing, { campaignName: choice.campaignName, campaignDir: choice.campaignDir, active });
    } else if (active) {
      next.push({ campaignId: choice.campaignId, campaignName: choice.campaignName, campaignDir: choice.campaignDir, active });
    }
    if (active && !wasActive) partyAdds.push(choice.campaignId);
    if (!active && wasActive) partyRemoves.push(choice.campaignId);
  }
  return { memberships: next, partyAdds, partyRemoves };
}

export function activeCampaigns(memberships: readonly CampaignMembershipDto[]): CampaignMembershipDto[] {
  return memberships.filter((membership) => membership.active);
}

/** The sheet's Campaign field: the active names comma-joined, or a dash. */
export function campaignFieldText(memberships: readonly CampaignMembershipDto[]): string {
  const names = activeCampaigns(memberships).map((membership) => membership.campaignName);
  return names.length ? names.join(', ') : '—';
}

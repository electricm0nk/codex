import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Write desktop boundary for the character's campaigns (SD-41 `decisions.md §29`).
 *
 * Invokes the `set_character_campaigns` Tauri command, which writes the character's
 * `campaigns.json` and at once copies the whole character folder to each active campaign's
 * `Characters/` folder (a stand-in's never). An inactive membership keeps its folder and stops being updated. Returns
 * one warning per campaign copy that could not be written ("<dir>: <reason>"); the memberships
 * are saved either way.
 */

export interface CampaignMembershipDto {
  campaignId: string;
  campaignName: string;
  /** The campaign folder (SD-42 `decisions.md §11`). */
  campaignDir: string;
  /** Checked: every save of the character is copied there. */
  active: boolean;
  /** Imported to play in another device's place: saves stay local (SD-42 `decisions.md §16`). Kept by the command. */
  standIn?: boolean;
  /** The backup counter this install last wrote. Kept by the command; the dialog never sends a new one. */
  lastBackupCounter?: number;
}

export interface SetCharacterCampaignsRequest {
  characterId: string;
  memberships: CampaignMembershipDto[];
}

export async function setCharacterCampaigns(request: SetCharacterCampaignsRequest): Promise<string[]> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for setting campaigns');
  }

  try {
    return await invoke<string[]>('set_character_campaigns', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to set campaigns: ${formatError(cause)}`);
  }
}

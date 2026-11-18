import type { LoadSavedCharacterResponse } from '../loadSavedCharacterDetail';
import { invokeCampaignFolder } from './invokeCampaignFolder';

/** One `Characters/<folder>/` backup (`src/campaign/characters.rs` `Backup`). */
export interface CampaignCharacterBackup {
  folder: string;
  characterId: string;
  label: string;
  gameSystem: string;
  /** `class_id:level` pairs, comma-separated (the hub list's `classSummary` shape). */
  classes: string;
  race: string;
  /** The master device's name; null for a backup with no `master.json`. */
  masterDevice: string | null;
  counter: number | null;
  savedAt: string;
  /** Why the folder could not be read; it is listed anyway. */
  error: string | null;
}

/** One file that differs between this device's copy and the backup (`characters.rs` `FileDiff`). */
export interface CampaignCharacterFileDiff {
  path: string;
  localBytes: number | null;
  backupBytes: number | null;
  newer: 'Local' | 'Backup' | 'Same';
  /** For `envelope.txt`: the fields whose values differ. */
  changedFields: string[];
}

export type ImportMode = 'StandIn' | 'MoveMaster';

export function listCampaignCharacters(campaignDir: string): Promise<CampaignCharacterBackup[]> {
  return invokeCampaignFolder('list_campaign_characters', { campaignDir }, 'listing the campaign characters');
}

/** Copies the backup onto this device; resolves to the new local character's id. */
export function importCampaignCharacter(campaignDir: string, folder: string, mode: ImportMode): Promise<{ characterId: string }> {
  return invokeCampaignFolder('import_campaign_character', { request: { campaignDir, folder, mode } }, 'importing the character');
}

export function diffCampaignCharacter(campaignDir: string, folder: string): Promise<CampaignCharacterFileDiff[]> {
  return invokeCampaignFolder('diff_campaign_character', { request: { campaignDir, folder } }, 'comparing with the backup');
}

/** The backup's sheet, read from a copy (the shared backup is never written). */
export function loadCampaignCharacter(campaignDir: string, folder: string): Promise<LoadSavedCharacterResponse> {
  return invokeCampaignFolder('load_campaign_character', { request: { campaignDir, folder } }, 'loading the campaign character');
}

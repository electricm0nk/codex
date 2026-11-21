import type { CampaignCharacterBackup, CampaignCharacterFileDiff, ImportMode } from '../boundary/campaignFolder/characters';
import { toRowSurface, type CharacterHubListRowSurface } from '../characterHub/buildCharacterHubListSurface';
import { formatHeldClasses } from '../characterHub/characterProgression';

/**
 * The campaign sheet's Characters list (SD-42 `decisions.md §16`): one row per `Characters/` backup,
 * the two import choices (stand-in is the default, Q21), and the diff between this device's copy and
 * the backup as lines (shown, never merged, Q22). Pure; the sheet reads through
 * `boundary/campaignFolder/characters.ts`.
 */

export interface CharacterRow {
  folder: string;
  characterId: string;
  title: string;
  subtitle: string;
  master: string;
  /** This device holds a copy (its own, or one imported earlier). */
  onThisDevice: boolean;
  readable: boolean;
  /** The hub-list row shape, so the existing detail view and full sheet open it. */
  surface: CharacterHubListRowSurface;
}

export function characterRows(backups: readonly CampaignCharacterBackup[], localIds: readonly string[]): CharacterRow[] {
  return backups.map((backup) => {
    const surface = toRowSurface({
      characterId: backup.characterId,
      displayLabel: backup.label || backup.folder,
      gameSystem: backup.gameSystem,
      schemaVersion: 0,
      savedAt: backup.savedAt,
      raceId: backup.race,
      classSummary: backup.classes,
    });
    const master = backup.error
      ? `Not readable: ${backup.error}`
      : backup.masterDevice
        ? `Master: ${backup.masterDevice} · saved ${backup.savedAt.slice(0, 10)}`
        : 'No master recorded (an older copy)';
    return {
      folder: backup.folder,
      characterId: backup.characterId,
      title: surface.displayLabel,
      subtitle: backup.error ? '' : `${surface.raceLabel} ${formatHeldClasses(backup.classes)}`.trim(),
      master,
      onThisDevice: backup.characterId !== '' && localIds.includes(backup.characterId),
      readable: !backup.error,
      surface,
    };
  });
}

export const IMPORT_CHOICES: ReadonlyArray<{ mode: ImportMode; label: string; help: string; isDefault: boolean }> = [
  {
    mode: 'StandIn',
    label: 'Play as stand-in',
    help: 'A copy on this device to play in their place. Your saves stay on this device; the campaign copy is never changed.',
    isDefault: true,
  },
  {
    mode: 'MoveMaster',
    label: 'Move the master here',
    help: 'This device becomes the master: your next save updates the campaign copy, and the old device is warned on its next save.',
    isDefault: false,
  },
];

export function diffLines(diffs: readonly CampaignCharacterFileDiff[]): string[] {
  if (diffs.length === 0) return ['No differences: this device’s copy matches the backup.'];
  return diffs.map((diff) => {
    if (diff.backupBytes === null) return `${diff.path} — only on this device`;
    if (diff.localBytes === null) return `${diff.path} — only in the backup`;
    const side = diff.newer === 'Local' ? 'this device’s copy is newer' : diff.newer === 'Backup' ? 'the backup is newer' : 'changed';
    return `${diff.path} — ${side}${diff.changedFields.length > 0 ? `; changed: ${diff.changedFields.join(', ')}` : ''}`;
  });
}

import type { CampaignMembershipDto } from '../boundary/setCharacterCampaigns';
import type { ImportedCustomItems } from '../boundary/customItemsExchange';
import type { CharacterCustom } from './customModel';
import { activeCampaigns } from './campaignMembershipModel';

/**
 * The custom item exchange flow (SD-43 `decisions.md §21`, item 4), with its native dialogs and
 * commands injected so it is tested without the desktop runtime. A place is a file anywhere on this
 * computer, or `Custom Items/` in one of the character's active campaigns (the file dialog starts
 * there). The backend does the reading, writing, same-name editing and game-system check.
 */

export interface ExchangePlace {
  key: string;
  label: string;
  /** `null`: a file on this computer, wherever the dialog is pointed. */
  campaignDir: string | null;
}

export interface ExchangeDeps {
  hasRuntime: () => boolean;
  /** Native save dialog; `null` when dismissed. */
  save: (defaultPath: string) => Promise<string | null>;
  /** Native open dialog for one JSON file; `null` when dismissed. */
  open: (defaultPath: string | null) => Promise<string | null>;
  folder: (campaignDir: string) => Promise<string>;
  exportItems: (request: { characterId: string; itemIds: string[]; path: string }) => Promise<number>;
  importItems: (request: { characterId: string; path: string; savedAt: string }) => Promise<ImportedCustomItems>;
  now: () => string;
}

export type ExchangeOutcome =
  | { kind: 'Exported'; message: string }
  | { kind: 'Imported'; message: string; custom: CharacterCustom }
  | { kind: 'Cancelled' }
  | { kind: 'Failed'; message: string };

export const LOCAL_PLACE: ExchangePlace = { key: 'local', label: 'A file on this computer', campaignDir: null };

/** This computer first, then `Custom Items/` in each active campaign. */
export function exchangePlaces(memberships: readonly CampaignMembershipDto[]): ExchangePlace[] {
  return [
    LOCAL_PLACE,
    ...activeCampaigns(memberships).map((membership) => ({
      key: membership.campaignDir,
      label: `Custom Items in ${membership.campaignName}`,
      campaignDir: membership.campaignDir,
    })),
  ];
}

/** `Aldric custom items.json`, filesystem-safe as the Print and Export names are. */
export function exchangeFileName(displayLabel: string): string {
  const safeName = displayLabel.replace(/[^a-z0-9-_ ]/gi, '').trim() || 'character';
  return `${safeName} custom items.json`;
}

const plural = (count: number, word: string) => `${count} ${word}${count === 1 ? '' : 's'}`;

export function importMessage(added: number, edited: number): string {
  return `Imported: ${plural(added, 'new item')} added, ${plural(edited, 'item')} with the same name edited.`;
}

async function startIn(place: ExchangePlace, deps: ExchangeDeps): Promise<string | null> {
  return place.campaignDir === null ? null : deps.folder(place.campaignDir);
}

const failed = (cause: unknown): ExchangeOutcome => ({ kind: 'Failed', message: cause instanceof Error ? cause.message : String(cause) });

export async function runExport(
  target: { characterId: string; displayLabel: string },
  itemIds: readonly string[],
  place: ExchangePlace,
  deps: ExchangeDeps
): Promise<ExchangeOutcome> {
  if (!deps.hasRuntime()) {
    return { kind: 'Failed', message: 'Exporting custom items requires the desktop runtime.' };
  }
  if (itemIds.length === 0) {
    return { kind: 'Failed', message: 'Tick at least one item to export.' };
  }
  try {
    const dir = await startIn(place, deps);
    const name = exchangeFileName(target.displayLabel);
    const path = await deps.save(dir === null ? name : `${dir}/${name}`);
    if (!path) {
      return { kind: 'Cancelled' };
    }
    const count = await deps.exportItems({ characterId: target.characterId, itemIds: [...itemIds], path });
    return { kind: 'Exported', message: `Exported ${plural(count, 'item')} to ${path}.` };
  } catch (cause: unknown) {
    return failed(cause);
  }
}

export async function runImport(characterId: string, place: ExchangePlace, deps: ExchangeDeps): Promise<ExchangeOutcome> {
  if (!deps.hasRuntime()) {
    return { kind: 'Failed', message: 'Importing custom items requires the desktop runtime.' };
  }
  try {
    const path = await deps.open(await startIn(place, deps));
    if (!path) {
      return { kind: 'Cancelled' };
    }
    const imported = await deps.importItems({ characterId, path, savedAt: deps.now() });
    return { kind: 'Imported', message: importMessage(imported.added, imported.edited), custom: imported.custom };
  } catch (cause: unknown) {
    return failed(cause);
  }
}

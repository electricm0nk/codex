import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assert, assertEqual } from '../testSupport/asserts';
import type { CampaignMembershipDto } from '../boundary/setCharacterCampaigns';
import { CustomTab } from './CustomTab';
import { EMPTY_CUSTOM } from './customModel';
import { LOCAL_PLACE, exchangeFileName, exchangePlaces, importMessage, runExport, runImport, type ExchangeDeps } from './customItemsExchangeModel';

/**
 * Custom item exchange (SD-43 item 4, `decisions.md §21`): the places offered, the flow with its
 * native dialogs and commands injected, and the Custom tab's button that opens it. The backend's
 * same-name edit and cross-system refusal are proven by `custom_items_exchange_*` in the desktop crate.
 */

const here = dirname(fileURLToPath(import.meta.url));

const MEMBERSHIPS: CampaignMembershipDto[] = [
  { campaignId: '/c/Reavers', campaignName: 'Reavers', campaignDir: '/c/Reavers', active: true },
  { campaignId: '/c/Old', campaignName: 'Old', campaignDir: '/c/Old', active: false },
];

interface Calls {
  saved: string[];
  opened: Array<string | null>;
  exported: Array<{ itemIds: string[]; path: string }>;
  imported: string[];
}

function deps(calls: Calls, overrides: Partial<ExchangeDeps> = {}): ExchangeDeps {
  return {
    hasRuntime: () => true,
    save: async (path) => {
      calls.saved.push(path);
      return `${path}`;
    },
    open: async (path) => {
      calls.opened.push(path);
      return '/c/Reavers/Custom Items/Aldric custom items.json';
    },
    folder: async (dir) => `${dir}/Custom Items`,
    exportItems: async (request) => {
      calls.exported.push({ itemIds: request.itemIds, path: request.path });
      return request.itemIds.length;
    },
    importItems: async (request) => {
      calls.imported.push(request.path);
      return { custom: { ...EMPTY_CUSTOM }, added: 2, edited: 1 };
    },
    now: () => '2026-11-22T00:00:00Z',
    ...overrides,
  };
}

const fresh = (): Calls => ({ saved: [], opened: [], exported: [], imported: [] });

function verifiesThePlacesAreThisComputerAndEachActiveCampaign() {
  const places = exchangePlaces(MEMBERSHIPS);
  assertEqual(places.length, 2, 'this computer and the one active campaign');
  assertEqual(places[0]?.key, LOCAL_PLACE.key, 'this computer first');
  assertEqual(places[1]?.label, 'Custom Items in Reavers', 'the active campaign');
  assertEqual(exchangeFileName('Aldric / the Bold'), 'Aldric  the Bold custom items.json', 'a filesystem-safe name');
  assertEqual(importMessage(1, 0), 'Imported: 1 new item added, 0 items with the same name edited.', 'singular and plural');
}

async function verifiesExportToACampaignStartsInItsCustomItemsFolder() {
  const calls = fresh();
  const outcome = await runExport({ characterId: 'c1', displayLabel: 'Aldric' }, ['a1', 'a2'], exchangePlaces(MEMBERSHIPS)[1]!, deps(calls));
  assertEqual(calls.saved[0], '/c/Reavers/Custom Items/Aldric custom items.json', 'the save dialog starts in Custom Items/');
  assertEqual(calls.exported[0]?.itemIds.join(','), 'a1,a2', 'both ticked items are sent');
  assertEqual(outcome.kind, 'Exported', 'exported');
  assert(outcome.kind === 'Exported' && outcome.message.includes('2 items'), 'the message counts what the backend wrote');
}

async function verifiesADismissedDialogWritesNothingAndNoTickIsRefused() {
  const calls = fresh();
  const cancelled = await runExport({ characterId: 'c1', displayLabel: 'Aldric' }, ['a1'], LOCAL_PLACE, deps(calls, { save: async () => null }));
  assertEqual(cancelled.kind, 'Cancelled', 'dismissed');
  assertEqual(calls.exported.length, 0, 'nothing exported');
  const none = await runExport({ characterId: 'c1', displayLabel: 'Aldric' }, [], LOCAL_PLACE, deps(calls));
  assertEqual(none.kind, 'Failed', 'no item ticked');
  const noFile = await runImport('c1', LOCAL_PLACE, deps(calls, { open: async () => null }));
  assertEqual(noFile.kind, 'Cancelled', 'import dismissed');
  assertEqual(calls.imported.length, 0, 'nothing imported');
}

async function verifiesImportHandsBackTheSavedCustomAndARefusalIsShown() {
  const calls = fresh();
  const outcome = await runImport('c1', LOCAL_PLACE, deps(calls));
  assertEqual(calls.opened[0], null, 'a local import opens the dialog wherever it last was');
  assert(outcome.kind === 'Imported' && outcome.message === importMessage(2, 1), 'the counts the backend returned');
  const refusal = 'Failed to import the custom items: x.json holds starfinder-1e custom items; this is a pathfinder-1e character, so nothing was imported';
  const refused = await runImport('c1', LOCAL_PLACE, deps(fresh(), { importItems: async () => Promise.reject(new Error(refusal)) }));
  assert(refused.kind === 'Failed' && refused.message === refusal, 'the backend refusal is shown as it is worded');
}

function verifiesTheCustomTabOpensTheDialog() {
  const html = renderToStaticMarkup(
    createElement(CustomTab, { custom: EMPTY_CUSTOM, loading: false, error: null, onEdit: () => undefined, onExchange: () => undefined, onOverrideChange: () => undefined, admittedLevels: [] })
  );
  assert(html.includes('Export / import items…'), 'the Custom tab offers the exchange');
  const sheet = readFileSync(join(here, 'CharacterSheet.tsx'), 'utf8');
  assert(sheet.includes('onExchange={() => setExchangeOpen(true)}') && sheet.includes('<CustomItemsExchangeDialog'), 'the sheet opens the dialog from the tab');
}

async function main() {
  verifiesThePlacesAreThisComputerAndEachActiveCampaign();
  await verifiesExportToACampaignStartsInItsCustomItemsFolder();
  await verifiesADismissedDialogWritesNothingAndNoTickIsRefused();
  await verifiesImportHandsBackTheSavedCustomAndARefusalIsShown();
  verifiesTheCustomTabOpensTheDialog();
  console.log('customItemsExchangeModel.test.ts: all assertions passed');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

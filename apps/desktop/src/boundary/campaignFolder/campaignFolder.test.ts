import { createCampaignFolder, listKnownCampaigns, openCampaignFolder, saveCampaignJson } from './knownCampaigns';
import { listCampaignTree, readCampaignFile, writeCampaignFile } from './campaignFiles';
import { assertEqual } from '../../testSupport/asserts';

/**
 * No Tauri runtime under `tsx`: every campaign-folder boundary fails with a descriptive message
 * instead of resolving to an empty value.
 */
async function messageOf(call: () => Promise<unknown>): Promise<string> {
  try {
    await call();
  } catch (cause) {
    return cause instanceof Error ? cause.message : String(cause);
  }
  return 'resolved';
}

async function main() {
  assertEqual(
    await messageOf(() => listKnownCampaigns()),
    'Tauri runtime not available for listing campaign folders',
    'list known'
  );
  assertEqual(
    await messageOf(() => openCampaignFolder('/x')),
    'Tauri runtime not available for opening a campaign folder',
    'open'
  );
  assertEqual(
    await messageOf(() =>
      createCampaignFolder('/x', {
        schemaVersion: 2,
        name: 'N',
        ruleSetId: 'crb',
        ruleSetLabel: 'Core',
        description: '',
        members: [],
        createdAt: 't',
        updatedAt: 't',
      })
    ),
    'Tauri runtime not available for creating a campaign folder',
    'create'
  );
  assertEqual(
    await messageOf(() =>
      saveCampaignJson(
        '/x',
        {
          schemaVersion: 2,
          name: 'N',
          ruleSetId: 'crb',
          ruleSetLabel: 'Core',
          description: '',
          members: [{ name: 'Gwen', addedAt: 't' }],
          createdAt: 't',
          updatedAt: 't',
        },
        0
      )
    ),
    'Tauri runtime not available for saving campaign.json',
    'save'
  );
  assertEqual(
    await messageOf(() => readCampaignFile('/x', 'Party Resources.md')),
    'Tauri runtime not available for reading Party Resources.md',
    'read'
  );
  assertEqual(
    await messageOf(() => writeCampaignFile('/x', 'a.md', new Uint8Array([35]))),
    'Tauri runtime not available for writing a.md',
    'write'
  );
  assertEqual(
    await messageOf(() => listCampaignTree('/x')),
    'Tauri runtime not available for listing the campaign folder',
    'tree'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

import { clearMigrated, legacyMigrationRows, olderCampaignFolder } from './campaignMigrationModel';
import { assertEqual } from '../testSupport/asserts';

/** A Map-backed stand-in for localStorage (only the three methods the model uses). */
function memoryStorage(entries: Record<string, string>) {
  const map = new Map(Object.entries(entries));
  return {
    getItem: (key: string) => map.get(key) ?? null,
    setItem: (key: string, value: string) => void map.set(key, value),
    removeItem: (key: string) => void map.delete(key),
    keys: () => [...map.keys()].sort().join(','),
  };
}

const RECORDS = JSON.stringify([
  { id: 'a', name: 'Alpha', ruleSetId: 'crb', ruleSetLabel: 'Core', description: '', members: [], partyCharacterIds: [], createdAt: 't', updatedAt: 't' },
  { id: 'b', name: 'Beta', ruleSetId: 'crb', ruleSetLabel: 'Core', description: '', members: [], partyCharacterIds: [], createdAt: 't', updatedAt: 't' },
]);
const ASSETS_A = JSON.stringify({ resources: [], adventureLog: [{ id: 'x', title: 'S1', body: 'b', updatedAt: 't' }], maps: [], wiki: [] });

const cases: [string, () => void][] = [
  [
    'one row per old campaign, with its record and entries as stored',
    () => {
      const storage = memoryStorage({ 'codex.campaigns': RECORDS, 'codex.campaign.assets.a': ASSETS_A });
      const rows = legacyMigrationRows(storage);
      assertEqual(rows.map((row) => row.name).join(','), 'Alpha,Beta', 'both old campaigns listed');
      assertEqual(JSON.parse(rows[0].recordJson).id, 'a', 'the record travels as JSON');
      assertEqual(rows[0].assetsJson, ASSETS_A, 'the entries travel exactly as stored');
      assertEqual(rows[1].assetsJson, null, 'no entries key: null');
      assertEqual(legacyMigrationRows(memoryStorage({})).length, 0, 'no old data: no rows');
      assertEqual(legacyMigrationRows(memoryStorage({ 'codex.campaigns': '{oops' })).length, 0, 'unreadable: no rows');
    },
  ],
  [
    'a migrated campaign is cleared; the others stay until migrated; then nothing is found',
    () => {
      const storage = memoryStorage({
        'codex.campaigns': RECORDS,
        'codex.campaign.assets.a': ASSETS_A,
        'codex.campaign.yourName': 'Gwen',
      });
      clearMigrated(storage, 'a');
      assertEqual(legacyMigrationRows(storage).map((row) => row.id).join(','), 'b', 'only Beta is left');
      assertEqual(storage.keys(), 'codex.campaign.yourName,codex.campaigns', 'Alpha entries removed, other keys kept');
      clearMigrated(storage, 'b');
      assertEqual(storage.keys(), 'codex.campaign.yourName', 'the list key goes with the last campaign');
      assertEqual(legacyMigrationRows(storage).length, 0, 'a second run finds nothing to migrate');
    },
  ],
  [
    'an older .config/ folder refused by Open folder is recognised',
    () => {
      assertEqual(
        olderCampaignFolder('Failed adding a campaign folder: /d/Old has no campaign.json: it is an older campaign (.config/); migrate it first'),
        true,
        'the refusal folder.rs gives a schema-1 folder'
      );
      assertEqual(olderCampaignFolder('/d has no campaign.json: it is not a campaign folder'), false, 'not a campaign');
    },
  ],
];

for (const [name, run] of cases) {
  run();
  console.log(`ok - ${name}`);
}
console.log(`campaignMigrationModel.test.ts: ${cases.length} cases passed`);

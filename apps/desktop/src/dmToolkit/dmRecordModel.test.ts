import {
  DM_RECORD_KINDS,
  createDmRecord,
  createMemoryDmStorage,
  deleteDmRecord,
  getDmRecord,
  getDmRecords,
  linkDmRecords,
  searchDmRecords,
  unlinkDmRecords,
  updateDmRecord,
  backlinksTo,
} from './dmRecordModel';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * v0.8 D-1: the DM Toolkit record store. Six NoDA kinds, typed links,
 * CRUD, persistence keyed per campaign. Storage is injected (the browser's
 * localStorage in production, mirroring `campaignModel.ts`; this test's
 * in-memory store here, because the Node test runtime has no
 * localStorage). Everything the console and the exporter read is what
 * this store returns from storage — nothing is held only in memory.
 */
const storage = createMemoryDmStorage();
const campaign = 'camp-1';

assertEqual(DM_RECORD_KINDS.join(','), 'World,Timeline,Place,Person,Clue,Scene,Rule', 'the six NoDA kinds plus Clue as a first-class seventh (D-3a), in authoring order');
assertEqual(getDmRecords(campaign, storage).length, 0, 'a fresh campaign has no records');

const bar = createDmRecord(campaign, { kind: 'Place', title: 'The Afterlife', summary: 'Bar in Watson.', body: '', fields: { gridRef: 'C4' } }, storage);
const rogue = createDmRecord(campaign, { kind: 'Person', title: 'Rogue Amendiares', summary: 'Fixer.', body: 'Runs the room.', fields: { role: 'Fixer' } }, storage);
assertEqual(getDmRecords(campaign, storage).length, 2, 'creates persist');
assert(bar.id !== rogue.id, 'ids are distinct');
assertEqual(getDmRecord(campaign, bar.id, storage)?.fields.gridRef, 'C4', 'per-kind extra fields persist');
assertEqual(getDmRecords('other-campaign', storage).length, 0, 'records are keyed per campaign');

// Persistence is what storage holds, not what a call returned: a fresh read
// through the same storage must see the same records.
const reread = getDmRecords(campaign, storage);
assertEqual(reread.map((r) => r.title).join('|'), 'The Afterlife|Rogue Amendiares', 'reads come from storage in creation order');

const updated = updateDmRecord(campaign, rogue.id, { summary: 'Fixer, ex-solo.' }, storage);
assertEqual(updated?.summary, 'Fixer, ex-solo.', 'update returns the new record');
assertEqual(getDmRecord(campaign, rogue.id, storage)?.summary, 'Fixer, ex-solo.', 'update persists');
assert((updated?.updatedAt ?? '') >= rogue.updatedAt, 'updatedAt moves forward');
assertEqual(updateDmRecord(campaign, 'nope', { title: 'x' }, storage), null, 'updating a missing record is null, not a throw');

// Typed links: stored on the source, labelled with the relation.
linkDmRecords(campaign, rogue.id, bar.id, 'Home base', storage);
assertEqual(getDmRecord(campaign, rogue.id, storage)?.links.length, 1, 'link persists on the source');
assertEqual(getDmRecord(campaign, rogue.id, storage)?.links[0].label, 'Home base', 'link carries its relation label');
linkDmRecords(campaign, rogue.id, bar.id, 'Home base', storage);
assertEqual(getDmRecord(campaign, rogue.id, storage)?.links.length, 1, 'the same link twice is one link');
assertEqual(backlinksTo(getDmRecords(campaign, storage), bar.id).map((r) => r.title).join(), 'Rogue Amendiares', 'backlinks are derived from the stored forward links');
unlinkDmRecords(campaign, rogue.id, bar.id, storage);
assertEqual(getDmRecord(campaign, rogue.id, storage)?.links.length, 0, 'unlink persists');
linkDmRecords(campaign, rogue.id, bar.id, 'Home base', storage);

// Search is per kind, case-insensitive, over title / summary / body / fields.
const scene = createDmRecord(campaign, { kind: 'Scene', title: 'Meet at the bar', summary: '', body: 'ROGUE waits upstairs.', fields: {} }, storage);
assertEqual(searchDmRecords(getDmRecords(campaign, storage), 'Person', 'rogue').length, 1, 'search narrows within the kind');
assertEqual(searchDmRecords(getDmRecords(campaign, storage), 'Scene', 'rogue').length, 1, 'search matches body text');
assertEqual(searchDmRecords(getDmRecords(campaign, storage), 'Place', 'rogue').length, 0, 'search does not leak across kinds');
assertEqual(searchDmRecords(getDmRecords(campaign, storage), 'Place', '  ').length, 1, 'blank search lists every record of the kind');
assertEqual(searchDmRecords(getDmRecords(campaign, storage), 'Place', 'c4').length, 1, 'search matches per-kind fields');

// Deleting a linked record removes every link that pointed at it (D-4
// decision, pinned here because the store owns it): no dangling link
// survives to be clicked.
linkDmRecords(campaign, scene.id, bar.id, 'Location', storage);
deleteDmRecord(campaign, bar.id, storage);
assertEqual(getDmRecord(campaign, bar.id, storage), null, 'delete persists');
assertEqual(getDmRecord(campaign, rogue.id, storage)?.links.length, 0, 'links from other records to the deleted one are removed');
assertEqual(getDmRecord(campaign, scene.id, storage)?.links.length, 0, '…from every record, not just the first');
assertEqual(getDmRecords(campaign, storage).length, 2, 'the other records survive');

// D-3a visibility: private by default, explicit to share, and records written
// before the field existed load as 'gm' — an upgrade never exposes prep.
const note = createDmRecord(campaign, { kind: 'World', title: 'Premise' }, storage);
assertEqual(getDmRecord(campaign, note.id, storage)?.visibility, 'gm', 'a new record is GM-only by default');
updateDmRecord(campaign, note.id, { visibility: 'players' }, storage);
assertEqual(getDmRecord(campaign, note.id, storage)?.visibility, 'players', 'sharing persists');
storage.setItem('codex.dmToolkit.records.legacy', JSON.stringify([{ id: 'old', kind: 'Place', title: 'Old', summary: '', body: '', fields: {}, links: [], createdAt: '', updatedAt: '' }]));
assertEqual(getDmRecords('legacy', storage)[0].visibility, 'gm', 'a pre-visibility record loads as gm, never as players');

// Corrupt storage reads as empty rather than throwing the console down.
storage.setItem('codex.dmToolkit.records.broken', '{not json');
assertEqual(getDmRecords('broken', storage).length, 0, 'unparseable storage reads as no records');

console.log('dmRecordModel tests passed');

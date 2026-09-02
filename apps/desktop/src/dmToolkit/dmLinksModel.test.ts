import { linkCandidates, resolveLinks } from './dmLinksModel';
import type { DmRecord } from './dmRecordModel';
import { assertEqual } from '../testSupport/asserts';

/**
 * v0.8 D-4: cross-links as the console renders them. A link button must
 * always focus a real record: the store already removes links when their
 * target is deleted (dmRecordModel.test.ts), and this second line of
 * defence pins that a link whose target is somehow absent is dropped from
 * the render rather than drawn as a button that focuses nothing.
 */
function rec(id: string, kind: DmRecord['kind'], title: string, links: DmRecord['links'] = []): DmRecord {
  return { id, kind, title, summary: '', body: '', fields: {}, links, visibility: 'gm', createdAt: '', updatedAt: '' };
}
const bar = rec('bar', 'Place', 'Afterlife');
const rogue = rec('rogue', 'Person', 'Rogue', [{ targetId: 'bar', label: 'Home base' }, { targetId: 'ghost', label: 'Ally' }]);
const scene = rec('scene', 'Scene', 'Meet', [{ targetId: 'rogue', label: 'NPC' }]);
const all = [bar, rogue, scene];

const links = resolveLinks(rogue, all);
assertEqual(links.length, 1, 'a link whose target does not exist is not rendered');
assertEqual(links[0].target.id, 'bar', 'resolved to the real target record');
assertEqual(links[0].label, 'Home base', 'label carried');

const candidates = linkCandidates(rogue, all);
assertEqual(candidates.map((c) => c.id).join(), 'bar,scene', 'every other record is a candidate, never the record itself');
assertEqual(candidates[0].kind, 'Place', 'candidates keep their kind for grouping');

console.log('dmLinksModel tests passed');

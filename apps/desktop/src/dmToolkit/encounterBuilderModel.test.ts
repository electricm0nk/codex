import { addMonster, addSavedMember, addTypedMember, emptyBuilder, removeMember, removeMonster, setMonsterCount, toRequest } from './encounterBuilderModel';
import { assertEqual } from '../testSupport/asserts';

/**
 * v0.8 E-2: the builder's pure state — who is in the party, which
 * monsters and how many — and the exact `rate_encounter` request it
 * becomes. No rating logic: levels for saved characters are NOT here (the
 * engine resolves them from the id), and a monster is sent as its catalog
 * key, never as a CR copied by the UI.
 */
let b = emptyBuilder();
assertEqual(toRequest(b), null, 'nothing to rate until there is a party and a monster');

b = addSavedMember(b, 'char-1');
b = addTypedMember(b, 3);
assertEqual(b.party.length, 2, 'two members');
assertEqual(toRequest(b), null, 'a party with no monsters is not rateable');

b = addMonster(b, 'b1:monster:dire_rat');
b = addMonster(b, 'b1:monster:dire_rat');
b = addMonster(b, 'b1:monster:ogre');
assertEqual(b.monsters.map((m) => `${m.catalogKey}x${m.count}`).join(), 'b1:monster:dire_ratx2,b1:monster:ogrex1', 'the same monster twice is one row with a count');

const req = toRequest(b);
assertEqual(JSON.stringify(req?.party), '[{"characterId":"char-1"},{"level":3}]', 'saved member sends only its id; typed member sends only its level');
assertEqual(req?.monsters.length, 3, 'counts expand to one entry per creature');
assertEqual(JSON.stringify(req?.monsters[0]), '{"catalogKey":"b1:monster:dire_rat"}', 'a monster is sent by key only — no CR copied from the row');

b = setMonsterCount(b, 'b1:monster:dire_rat', 4);
assertEqual(toRequest(b)?.monsters.length, 5, 'count edits change the request');
b = setMonsterCount(b, 'b1:monster:dire_rat', 0);
assertEqual(b.monsters.length, 1, 'a count of zero removes the row');
b = removeMonster(b, 'b1:monster:ogre');
assertEqual(toRequest(b), null, 'no monsters left → not rateable');

b = addMonster(b, 'b1:monster:ogre');
b = removeMember(b, 0);
assertEqual(JSON.stringify(toRequest(b)?.party), '[{"level":3}]', 'members are removed by position');
assertEqual(addTypedMember(b, 0).party.length, 1, 'level 0 is refused (engine takes u8 ≥ 1)');
assertEqual(addTypedMember(b, 21).party.length, 1, 'level above 20 is refused');
assertEqual(addSavedMember(b, 'x').party.length, 2, 'saved id is accepted');
assertEqual(addSavedMember(addSavedMember(b, 'x'), 'x').party.length, 3, 'the same saved character can appear twice (twins) — the engine rates what it is given');

console.log('encounterBuilderModel tests passed');

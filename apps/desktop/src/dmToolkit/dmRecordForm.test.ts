import { draftFromRecord, draftToInput, emptyDraft, validateDraft } from './dmRecordForm';
import type { DmRecord } from './dmRecordModel';
import { assertEqual } from '../testSupport/asserts';

/**
 * v0.8 D-3: the create/edit form's pure half — the draft shape the form
 * edits, what a valid draft is, and how it becomes a store write. The
 * acceptance test for D-3 (a write survives a reload) is the store's
 * round-trip already pinned in dmRecordModel.test.ts; this pins that the
 * form feeds it exactly what the DM typed.
 */
const blank = emptyDraft('Person');
assertEqual(blank.kind, 'Person', 'draft carries its kind');
assertEqual(blank.title, '', 'blank title');
assertEqual(Object.keys(blank.fields).join(), 'role,motivation,statBlock', 'every per-kind field is present, in spec order');
assertEqual(blank.fields.role, '', 'fields start blank');
assertEqual(blank.visibility, 'gm', 'a new draft is GM-only (D-3a)');

assertEqual(validateDraft(blank).join(), 'Title is required.', 'a blank title is the one hard error');
assertEqual(validateDraft({ ...blank, title: '   ' }).join(), 'Title is required.', 'whitespace is not a title');
assertEqual(validateDraft({ ...blank, title: 'Rogue' }).length, 0, 'a title is enough');

const input = draftToInput({ ...blank, title: '  Rogue ', summary: ' Fixer ', body: 'x', fields: { role: ' Fixer ', motivation: '', statBlock: '' } });
assertEqual(input.title, 'Rogue', 'title trimmed');
assertEqual(input.summary, 'Fixer', 'summary trimmed');
assertEqual(input.fields?.role, 'Fixer', 'field values trimmed');
assertEqual(Object.keys(input.fields ?? {}).join(), 'role', 'blank fields are not stored');

const record: DmRecord = { id: 'r', kind: 'Place', title: 'Afterlife', summary: 'Bar', body: 'b', fields: { gridRef: 'C4' }, links: [{ targetId: 'x', label: 'Resident' }], visibility: 'gm', createdAt: '', updatedAt: '' };
const draft = draftFromRecord(record);
assertEqual(draft.fields.gridRef, 'C4', 'stored fields load into the draft');
assertEqual(draft.fields.role, '', 'missing per-kind fields load blank, not undefined');
assertEqual(draftToInput({ ...draft, visibility: 'players' }).visibility, 'players', 'the toggle is written through');
assertEqual('links' in draftToInput(draft), false, 'the form never touches links — those are D-4’s, and an edit must not clobber them');

console.log('dmRecordForm tests passed');

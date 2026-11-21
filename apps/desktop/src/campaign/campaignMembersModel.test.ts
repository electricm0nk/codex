import {
  addMember,
  membersForCreate,
  membersForSave,
  removeMember,
  renameMember,
} from './campaignMembersModel';
import { assert, assertEqual } from '../testSupport/asserts';

const NOW = '2026-11-17T10:00:00.000Z';

function names(members: { name: string }[]): string {
  return members.map((member) => member.name).join(',');
}

const cases: [string, () => void][] = [
  [
    'creator is first',
    () => {
      const members = membersForCreate('  Gwen ', ['Ana', ' ', 'Bo '], NOW);
      assertEqual(names(members), 'Gwen,Ana,Bo', 'creator first, blank rows dropped, names trimmed');
      assertEqual(members[0].addedAt, NOW, 'addedAt stamped');
      assertEqual(Object.keys(members[0]).join(','), 'name,addedAt', 'member shape is {name, addedAt} — no email, no role');
      let refused = '';
      try {
        membersForCreate('  ', ['Ana'], NOW);
      } catch (cause) {
        refused = (cause as Error).message;
      }
      assertEqual(refused, 'Your name is required: the creator is member #1.', 'a blank creator is refused');
    },
  ],
  [
    'count never below one',
    () => {
      const one = membersForCreate('Gwen', [], NOW);
      assertEqual(one.length, 1, 'a campaign with no other member still has its creator');
      assertEqual(removeMember(one, 0).length, 1, 'removing the only member leaves one');
      const saved = membersForSave([{ name: 'Gwen', addedAt: NOW }, { name: '  ', addedAt: NOW }], NOW);
      assertEqual(saved.length, 1, 'blank rows are dropped on save, the creator stays');
      let refused = '';
      try {
        membersForSave([{ name: ' ', addedAt: NOW }], NOW);
      } catch (cause) {
        refused = (cause as Error).message;
      }
      assert(refused.length > 0, 'a save whose first member is blank is refused');
    },
  ],
  [
    'remove refuses the first',
    () => {
      const members = membersForCreate('Gwen', ['Ana', 'Bo'], NOW);
      assertEqual(names(removeMember(members, 0)), 'Gwen,Ana,Bo', 'index 0 is never removed');
      assertEqual(names(removeMember(members, 1)), 'Gwen,Bo', 'any other member is removed');
      assertEqual(names(removeMember(members, 9)), 'Gwen,Ana,Bo', 'an out-of-range index changes nothing');
    },
  ],
  [
    'rename keeps order',
    () => {
      const members = membersForCreate('Gwen', ['Ana', 'Bo'], NOW);
      const renamed = renameMember(members, 1, 'Anna');
      assertEqual(names(renamed), 'Gwen,Anna,Bo', 'the renamed member keeps its place');
      assertEqual(renamed[1].addedAt, NOW, 'rename keeps addedAt');
      assertEqual(names(renameMember(members, 0, 'Gwendolyn')), 'Gwendolyn,Ana,Bo', 'the creator can be renamed');
      const added = addMember(members, '2026-11-18T00:00:00.000Z');
      assertEqual(added.length, 4, 'add appends one row');
      assertEqual(added[3].name, '', 'the new row is blank until typed');
      assertEqual(names(membersForSave(renameMember(added, 3, ' Cy '), NOW)), 'Gwen,Ana,Bo,Cy', 'save keeps the order');
    },
  ],
];

for (const [name, run] of cases) {
  run();
  console.log(`ok - ${name}`);
}
console.log(`campaignMembersModel.test.ts: ${cases.length} cases passed`);

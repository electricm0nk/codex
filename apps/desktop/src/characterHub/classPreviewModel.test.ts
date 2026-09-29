import { buildClassPreview } from './classPreviewModel';
import type { ClassCatalogEntryDto } from '../boundary/loadClassCatalog';
import { assertEqual } from '../testSupport/asserts';
import { classOptionsFromRoster } from './classRoster';
import { classRosterWire } from '../testSupport/classRosterWire';

/** SD-36 F4c: the preview takes the picked ROSTER option (id + label), not a bare label string. */
const roster = classOptionsFromRoster(classRosterWire());
const option = (classId: string) => {
  const found = roster.find((entry) => entry.id === classId);
  if (!found) {
    throw new Error(`${classId} is not on the served roster`);
  }
  return found;
};

/**
 * F-11 (scout audit item 9): the create form's class select gave no hint
 * of what a class is mechanically. `list_class_catalog` already returns
 * BAB and the three base saves per class per level; this pins the preview
 * that reads the row for the picked class and level, verbatim. Skill
 * points per level are NOT on that DTO (checked in both
 * loadClassCatalog.ts and class_catalog.rs), so the preview does not show
 * them — that is a backend wiring gap, not a TS derivation.
 */
const catalog: ClassCatalogEntryDto[] = [
  { classId: 'Fighter', level: 1, baseAttackBonus: 1, fortSave: 2, refSave: 0, willSave: 0 },
  { classId: 'Fighter', level: 2, baseAttackBonus: 2, fortSave: 3, refSave: 0, willSave: 0 },
  { classId: 'Unchained Monk', level: 1, baseAttackBonus: 1, fortSave: 2, refSave: 2, willSave: 0 },
];

const fighter2 = buildClassPreview(catalog, option('class:fighter'), 2);
assertEqual(fighter2.kind, 'Row', 'a catalogued class at a catalogued level yields a row');
if (fighter2.kind === 'Row') {
  assertEqual(fighter2.level, 2, 'the row is for the picked level');
  assertEqual(fighter2.baseAttackBonus, '+2', 'BAB signed, verbatim');
  assertEqual(fighter2.fortSave, '+3', 'Fort');
  assertEqual(fighter2.refSave, '+0', 'Ref');
  assertEqual(fighter2.willSave, '+0', 'Will');
}

const monk = buildClassPreview(catalog, option('class:unchained_monk'), 1);
assertEqual(monk.kind, 'Row', 'joins on the catalog display name, so replacement classes resolve to their own row');

const oracle = buildClassPreview(catalog, option('class:oracle'), 1);
assertEqual(oracle.kind, 'Unavailable', 'a class absent from the catalog is stated, not zero-filled');
assertEqual(oracle.kind === 'Unavailable' ? oracle.message : '', 'The class catalog has no progression rows for Oracle (class:oracle) yet.', 'names the class');

const tooHigh = buildClassPreview(catalog, option('class:fighter'), 9);
assertEqual(tooHigh.kind, 'Unavailable', 'a level the catalog does not carry is stated');

const pending = buildClassPreview(null, option('class:fighter'), 1);
assertEqual(pending.kind, 'Loading', 'no catalog yet is a loading state, not an absence claim');

const samurai = buildClassPreview(catalog, option('class:samurai'), 1);
assertEqual(samurai.kind === 'Unavailable' ? samurai.message : '', 'The class catalog has no progression rows for Samurai (class:samurai) yet.', 'a newly offered class with no catalog rows names its roster id');

console.log('classPreviewModel tests passed');

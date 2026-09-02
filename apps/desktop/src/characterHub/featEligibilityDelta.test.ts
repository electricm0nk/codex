import { describeEligibilityDelta, eligibilityDelta } from './featEligibilityDelta';
import type { FeatCatalogEntryDto } from '../boundary/listFeats';
import { assertEqual } from '../testSupport/asserts';

/**
 * v0.8 G-3 (audit item 29): after a feat is added or removed, the sheet
 * says what that changed — by diffing TWO ENGINE ANSWERS from
 * `list_feats_for_character`, before and after the mutation. Nothing here
 * decides eligibility: a feat is "now eligible" exactly when the second
 * answer says eligible and the first did not. No prerequisite knowledge,
 * no feat-implies-feat table.
 */
function feat(key: string, eligible: boolean | undefined): FeatCatalogEntryDto {
  return {
    key,
    category: 'Combat',
    name: key,
    description: null,
    source: 'Crb',
    chooserTargetKind: null,
    eligibility: eligible === undefined ? undefined : { eligible, unavailableReason: eligible ? null : 'x', met: [], unmet: [], unverified: [], prerequisiteCount: 1 },
  };
}
const before = [feat('Power Attack', true), feat('Cleave', false), feat('Improved Bull Rush', false), feat('Dodge', true), feat('Great Cleave', false), feat('Mystery', undefined)];
const after = [feat('Power Attack', true), feat('Cleave', true), feat('Improved Bull Rush', true), feat('Dodge', false), feat('Great Cleave', false), feat('Mystery', true)];

const delta = eligibilityDelta(before, after, 'Power Attack');
assertEqual(delta.nowEligible.join(','), 'Cleave,Improved Bull Rush', 'eligible after and not before — in catalog order');
assertEqual(delta.noLongerEligible.join(','), 'Dodge', 'eligible before and not after');
assertEqual(delta.nowEligible.includes('Mystery'), false, 'a feat with no verdict before is not claimed as newly eligible — that would be a guess');
assertEqual(delta.nowEligible.includes('Power Attack'), false, 'the feat just added is not reported about itself');

assertEqual(describeEligibilityDelta({ nowEligible: ['Cleave', 'Improved Bull Rush'], noLongerEligible: [] }), 'Now eligible: Cleave, Improved Bull Rush.', 'positive wording');
assertEqual(describeEligibilityDelta({ nowEligible: [], noLongerEligible: ['Dodge'] }), 'No longer eligible: Dodge.', 'inverse wording');
assertEqual(describeEligibilityDelta({ nowEligible: ['Cleave'], noLongerEligible: ['Dodge'] }), 'Now eligible: Cleave. No longer eligible: Dodge.', 'both');
assertEqual(describeEligibilityDelta({ nowEligible: [], noLongerEligible: [] }), null, 'an empty delta is silence, not "nothing changed" noise');

assertEqual(eligibilityDelta(before, before, 'Power Attack').nowEligible.length, 0, 'identical answers → nothing');
assertEqual(eligibilityDelta([], after, 'x').nowEligible.length, 0, 'no before-answer → nothing claimed');

console.log('featEligibilityDelta tests passed');

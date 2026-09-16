import { buildCreationRacialTraitsPreview } from './creationRacialTraitsPreview';
import type { RaceSelectionResponse } from '../boundary/loadAlternateRacialTraits';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * F-2 (scout audit item 5): the create form rendered only the alternate
 * racial traits a player could swap in; the standard traits the race
 * actually grants (Hardy, Stonecunning, Keen Senses…) were never shown
 * before committing, even though `resolve_race_alternate_selection`'s
 * `appliedTraits` carries them with `role: "default"` on every resolve.
 * This pins the preview: applied non-alternate rows in engine order with
 * engine prose, the standard traits the chosen alternates removed listed
 * separately, and the engine's own reason when nothing can be shown.
 */
function resolution(overrides: Partial<RaceSelectionResponse> = {}): RaceSelectionResponse {
  return {
    raceId: 'Dwarf',
    raceKey: 'Dwarf',
    raceName: 'Dwarf',
    book: 'CRB',
    appliedTraits: [
      { key: 'Dwarf ~ Hardy', name: 'Hardy', book: 'CRB', role: 'default', description: '+2 vs poison.' },
      { key: 'Dwarf ~ Stonecunning', name: 'Stonecunning', book: 'CRB', role: 'default', description: 'Stone.' },
      { key: 'Dwarf ~ Deep Warrior', name: 'Deep Warrior', book: 'APG', role: 'alternate', description: 'Deep.' },
      { key: 'Dwarf ~ Bonus', name: 'Bonus Thing', book: 'APG', role: 'flagGranted', description: 'Granted.' },
    ],
    suppressions: [
      { suppressedTraitKey: 'Dwarf ~ Defensive Training', suppressedTraitName: 'Defensive Training', flag: 'f', setByTraitKey: 'Dwarf ~ Deep Warrior', setByTraitName: 'Deep Warrior' },
    ],
    firedFlags: ['f'],
    inertFlags: [],
    unmatchedSelections: [],
    blockedAlternates: [],
    conflictingSelections: [],
    renderedTraitDescriptions: [
      { key: 'Dwarf ~ Hardy', name: 'Hardy', text: '+2 vs poison, rendered.', droppedArgs: [], movedByFeats: false },
    ],
    displayValueFeats: [],
    errors: [],
    ...overrides,
  };
}

const preview = buildCreationRacialTraitsPreview(resolution());
assertEqual(preview.unavailableReason, null, 'resolved payload has no unavailable reason');
assertEqual(preview.rows.map((r) => r.name).join(','), 'Hardy,Stonecunning,Bonus Thing', 'standard + granted rows, alternates excluded (they render in their own picker)');
assertEqual(preview.rows[0].text, '+2 vs poison, rendered.', 'rendered prose preferred');
assertEqual(preview.rows[1].text, 'Stone.', 'applied description is the fallback');
assertEqual(preview.rows[2].roleLabel, 'Granted by an alternate', 'granted rows are labelled as such');
assertEqual(preview.replaced.length, 1, 'removed standard traits listed');
assertEqual(preview.replaced[0].name, 'Defensive Training', 'by name');
assertEqual(preview.replaced[0].byName, 'Deep Warrior', 'and by what removed it');

const none = buildCreationRacialTraitsPreview(null);
assert(none.unavailableReason !== null && none.rows.length === 0, 'no resolution → stated reason, no rows');

const errored = buildCreationRacialTraitsPreview(resolution({ errors: ['corpus missing'] }));
assertEqual(errored.unavailableReason, 'corpus missing', "engine's own error is the reason");

console.log('creationRacialTraitsPreview tests passed');

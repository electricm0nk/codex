import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { readFileSync } from 'node:fs';
import { assert, assertEqual } from '../../testSupport/asserts';
import type { CreateCharacterRequest, EncumbranceDto } from '../../boundary/loadCreateCharacter';
import type { ExplanationDto } from '../../boundary/loadSavedCharacterDetail';
import { SquareBlock } from './SquareBlock';
import { draftPreviewKey, squareBlockForDraft, type DraftPreviewState } from './createSheetBlock';

// Brief 2 L5: Create's Encumbrance + Initiative boxes, in the same 2x2 block as the sheet, fed by
// the engine's `draft_sheet_preview`. A rejected preview shows its error, never an em dash or a
// made-up number.

const INITIATIVE: ExplanationDto = { id: 'initiative.total', value: 6, detail: 'Initiative +6: Dexterity modifier +2; Improved Initiative +4.' } as ExplanationDto;
const LOAD: EncumbranceDto = {
  totalCarriedWeightLbs: 29,
  totalCarriedCostGp: 115,
  lightMaxLbs: 76,
  mediumMaxLbs: 153,
  heavyMaxLbs: 230,
  level: 'Light',
  loadArmorCheckPenalty: 0,
  perItem: [],
  unresolvedItemIds: [],
};

const render = (state: DraftPreviewState, hp: number | null = 12, conditions: string[] = []) =>
  renderToStaticMarkup(createElement(SquareBlock, squareBlockForDraft(state, hp, conditions)));

function verifiesAReadyPreviewRendersTheEngineValues() {
  const html = render({ kind: 'ready', preview: { initiative: INITIATIVE, encumbrance: LOAD } });
  assert(html.includes('+6'), 'the engine initiative value');
  assert(html.includes(INITIATIVE.detail), 'the engine initiative detail, verbatim');
  assert(html.includes('Light Load') && html.includes('29 lb'), 'the engine encumbrance via describeEncumbrance');
  assert(html.includes('12 / 12'), 'hit points from the Levels list');
  assert(html.includes('Normal'), 'no conditions reads Normal');
  assert(!html.includes('—'), 'nothing reads absent');
}

function verifiesARejectedPreviewShowsItsError() {
  const message = "Failed to load the draft character's sheet preview: unknown class";
  const html = render({ kind: 'error', message });
  const escaped = message.replace("'", '&#x27;'); // renderToStaticMarkup escapes the apostrophe
  assertEqual(html.split(escaped).length - 1, 2, 'the error shows in both the Initiative and the Encumbrance box');
  assert(!html.includes('—'), 'no em dash stands in for the error');
  assert(!html.includes('+0'), 'no made-up initiative');
}

function verifiesLoadingAndNoLevels() {
  const loading = render({ kind: 'loading' });
  assert(loading.includes('Loading'), 'loading says so');
  assert(!loading.includes('—'), 'loading is not an em dash');
  const empty = render({ kind: 'noLevels' }, null);
  assert(empty.includes('Add a level first'), 'no levels: the boxes say a level is needed');
  assert(!empty.includes('Unknown'), 'no levels is not an unknown hit die');
}

function verifiesTheSheetStillReadsAbsentAsAnEmDash() {
  const html = renderToStaticMarkup(
    createElement(SquareBlock, { hp: 12, initiative: null, encumbrance: undefined, conditions: [] }),
  );
  assert(html.includes('—'), 'without an absent reason the sheet keeps its em dash');
}

const BASE = {
  characterId: 'a',
  savedAt: '2026-10-12T00:00:00Z',
  displayLabel: 'Ann',
  raceId: 'race:human',
  classId: 'class:fighter',
  level: 1,
  additionalLevels: [],
  hitPointLevels: [{ classId: 'class:fighter', value: 10 }],
  selectedFeats: [],
  skillAllocations: [],
  selectedSpells: [],
  selectedEquipment: [],
  priceMode: 'standard',
  abilityScores: { strength: 10, dexterity: 14, constitution: 10, intelligence: 10, wisdom: 10, charisma: 10 },
  abilityBonusTarget: 'strength',
  selectedAlternateTraitKeys: [],
} as unknown as CreateCharacterRequest;

function verifiesTheRecomputeKeyFollowsLevelsAbilitiesFeatsAndEquipment() {
  const key = draftPreviewKey(BASE);
  assertEqual(draftPreviewKey({ ...BASE, characterId: 'b', savedAt: 'later', displayLabel: 'Bo' }), key, 'a fresh id, timestamp or name does not recompute');
  assert(draftPreviewKey({ ...BASE, level: 2 }) !== key, 'levels recompute');
  assert(draftPreviewKey({ ...BASE, abilityScores: { ...BASE.abilityScores, dexterity: 16 } }) !== key, 'abilities recompute');
  assert(draftPreviewKey({ ...BASE, selectedFeats: [{ featId: 'feat:improved_initiative', target: null }] } as CreateCharacterRequest) !== key, 'feats recompute');
  assert(draftPreviewKey({ ...BASE, selectedEquipment: [{ itemId: 'item:longsword' }] } as CreateCharacterRequest) !== key, 'equipment recomputes');
  assertEqual(draftPreviewKey(null), null, 'no draft, no key');
}

function verifiesCreateRendersTheBlockFromThePreview() {
  const source = readFileSync(new URL('../CreateCharacterForm.tsx', import.meta.url), 'utf8');
  assert(source.includes('loadDraftSheetPreview('), 'Create calls the engine draft preview');
  assert(source.includes('squareBlockForDraft('), 'Create renders the block from the preview state');
  assert(source.includes('custom.conditions'), 'conditions come from the custom record');
}

verifiesAReadyPreviewRendersTheEngineValues();
verifiesARejectedPreviewShowsItsError();
verifiesLoadingAndNoLevels();
verifiesTheSheetStillReadsAbsentAsAnEmDash();
verifiesTheRecomputeKeyFollowsLevelsAbilitiesFeatsAndEquipment();
verifiesCreateRendersTheBlockFromThePreview();
console.log('createSheetBlock.test.ts: all assertions passed');

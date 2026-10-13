import { loadDraftSheetPreview, readDraftSheetPreview } from './draftSheetPreview';
import type { CreateCharacterRequest } from './loadCreateCharacter';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The wire shape `draft_sheet_preview` returns (`DraftSheetPreviewDto` in `character_hub.rs`,
 * pinned on the Rust side by `the_draft_preview_crosses_the_boundary_in_the_camel_case_wire_shape`):
 * the engine's `initiative.total` row and the `EncumbranceDto` the sheet already reads.
 */
const WIRE = {
  initiative: { id: 'initiative.total', value: 6, detail: 'Initiative +6: Dexterity modifier +2, Improved Initiative +4' },
  encumbrance: {
    totalCarriedWeightLbs: 29,
    totalCarriedCostGp: 115,
    lightMaxLbs: 100,
    mediumMaxLbs: 200,
    heavyMaxLbs: 300,
    level: 'Light',
    loadArmorCheckPenalty: 0,
    perItem: [
      { itemId: 'item:chain_shirt', weightLbs: 25, costGp: 100 },
      { itemId: 'item:longsword', weightLbs: 4, costGp: 15 },
    ],
    unresolvedItemIds: [],
  },
};

function readsTheWireShape() {
  const preview = readDraftSheetPreview(WIRE);
  assertEqual(preview.initiative.id, 'initiative.total', 'the engine row id');
  assertEqual(preview.initiative.value, 6, 'the engine total, not a front-end sum');
  assertEqual(preview.encumbrance.level, 'Light', 'the load tier');
  assertEqual(preview.encumbrance.perItem.length, 2, 'the carried items');
}

function refusesAPayloadMissingAField() {
  for (const [label, raw] of [
    ['no initiative', { encumbrance: WIRE.encumbrance }],
    ['no encumbrance', { initiative: WIRE.initiative }],
    ['initiative without a value', { ...WIRE, initiative: { id: 'initiative.total', detail: '' } }],
    ['encumbrance without a level', { ...WIRE, encumbrance: { ...WIRE.encumbrance, level: undefined } }],
    ['not an object', null],
  ] as const) {
    let thrown: unknown;
    try {
      readDraftSheetPreview(raw);
    } catch (cause) {
      thrown = cause;
    }
    assert(thrown instanceof Error, `${label}: a malformed payload is refused, not rendered`);
  }
}

async function loadThrowsDescriptiveErrorWithNoRuntime() {
  let thrown: unknown;
  try {
    await loadDraftSheetPreview({} as CreateCharacterRequest);
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(message, 'Tauri runtime not available for loading the draft sheet preview', 'no-runtime failure is descriptive');
}

async function main() {
  readsTheWireShape();
  refusesAPayloadMissingAField();
  await loadThrowsDescriptiveErrorWithNoRuntime();
  console.log('draftSheetPreview.test.ts: all assertions passed');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

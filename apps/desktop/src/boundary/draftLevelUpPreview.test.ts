import { loadDraftLevelUpPreview, readDraftLevelUpPreview } from './draftLevelUpPreview';
import type { CreateCharacterRequest } from './loadCreateCharacter';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The wire shape `draft_level_up_preview` returns (`DraftLevelUpPreviewDto` in `character_hub.rs`,
 * pinned on the Rust side by `the_draft_level_up_preview_crosses_the_boundary_in_the_camel_case_wire_shape`):
 * the class levels of the transition and the engine's automatic grant names, verbatim.
 */
const WIRE = {
  fromLevel: 1,
  toLevel: 2,
  automaticFeatures: ['Fighter base_attack_bonus at level 2', 'bravery'],
};

function readsTheWireShape() {
  const preview = readDraftLevelUpPreview(WIRE);
  assertEqual(preview.fromLevel, 1, 'the class level before');
  assertEqual(preview.toLevel, 2, 'the class level after');
  assertEqual(preview.automaticFeatures.join('|'), 'Fighter base_attack_bonus at level 2|bravery', 'the engine names, verbatim and in order');
  assertEqual(readDraftLevelUpPreview({ ...WIRE, automaticFeatures: [] }).automaticFeatures.length, 0, 'no grants is a real answer');
}

function refusesAPayloadMissingAField() {
  for (const [label, raw] of [
    ['not an object', null],
    ['no fromLevel', { toLevel: 2, automaticFeatures: [] }],
    ['no toLevel', { fromLevel: 1, automaticFeatures: [] }],
    ['no automaticFeatures', { fromLevel: 1, toLevel: 2 }],
    ['a grant that is not a name', { ...WIRE, automaticFeatures: [{ name: 'bravery' }] }],
  ] as const) {
    let thrown: unknown;
    try {
      readDraftLevelUpPreview(raw);
    } catch (cause) {
      thrown = cause;
    }
    assert(thrown instanceof Error, `${label}: a malformed payload is refused, not rendered`);
  }
}

async function loadThrowsDescriptiveErrorWithNoRuntime() {
  let thrown: unknown;
  try {
    await loadDraftLevelUpPreview({} as CreateCharacterRequest, 'class:fighter', 1);
  } catch (cause) {
    thrown = cause;
  }
  const message = thrown instanceof Error ? thrown.message : String(thrown);
  assertEqual(message, 'Tauri runtime not available for loading the draft level-up preview', 'no-runtime failure is descriptive');
}

async function main() {
  readsTheWireShape();
  refusesAPayloadMissingAField();
  await loadThrowsDescriptiveErrorWithNoRuntime();
  console.log('draftLevelUpPreview.test.ts: all assertions passed');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

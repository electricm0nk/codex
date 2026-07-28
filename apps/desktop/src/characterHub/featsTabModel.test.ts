import { resolveSelectedFeatEntries } from './featsTabModel';
import { assert, assertEqual } from '../testSupport/asserts';
import type { ItemPickerEntry } from './itemPickerFilter';

/**
 * `selectedFeats: string[]` genuinely mixes two real shapes (confirmed
 * against source, not assumed):
 *  - the feat catalog's own `key` field, human-readable verbatim (e.g.
 *    "Deflect Arrows") -- what the Feats-tab "Add Feat" picker pushes
 *    (`CharacterSheet.tsx`'s `handleAddFeat`/`handleLevelUpFeatPick` both
 *    push `entry.key`).
 *  - the rules engine's own lowercase `feat:snake_case` selection token
 *    (e.g. "feat:deflect_arrows") -- what `pf1_adapter.rs`'s
 *    `compose_character_input` seeds a fresh character with
 *    (`selected_feats: vec!["feat:power_attack", "feat:dodge",
 *    "feat:weapon_focus"]`) and what `pilot_compute.rs`'s gates match
 *    against. No normalization layer exists anywhere between the two
 *    (checked `rule_system_adapter.rs`/`pf1_adapter.rs` directly --
 *    `selected_feats` is cloned/appended raw, never translated).
 */
const CATALOG: ItemPickerEntry[] = [
  { key: 'Deflect Arrows', name: 'Deflect Arrows', detail: 'Combat · Once per round, negate an incoming ranged attack.' },
  { key: 'Dodge', name: 'Dodge', detail: 'Combat · Your training and reflexes allow you to react swiftly.' },
  { key: 'Weapon Focus', name: 'Weapon Focus', detail: 'Combat · You have workshopped a single weapon.' },
  { key: "Gorgon's Fist", name: "Gorgon's Fist", detail: 'Combat · With one well-placed blow, you leave your target reeling.' },
];

function verifiesCatalogKeyFormatResolvesDirectly() {
  const [resolved] = resolveSelectedFeatEntries(['Deflect Arrows'], CATALOG);
  assertEqual(resolved.raw, 'Deflect Arrows', 'raw string is preserved verbatim');
  assert(resolved.entry !== null, 'the human-readable catalog-key form must resolve');
  assertEqual(resolved.entry?.key, 'Deflect Arrows', 'resolves to the Deflect Arrows catalog entry');
}

function verifiesEngineTokenFormatResolvesToSameCatalogEntry() {
  const [resolved] = resolveSelectedFeatEntries(['feat:deflect_arrows'], CATALOG);
  assertEqual(resolved.raw, 'feat:deflect_arrows', 'raw engine token is preserved verbatim');
  assert(resolved.entry !== null, 'the lowercase feat: token form must also resolve');
  assertEqual(resolved.entry?.key, 'Deflect Arrows', 'the engine token resolves to the same catalog entry as the human-readable key');
}

function verifiesApostropheInCatalogNameStillResolvesFromSlugToken() {
  const [resolved] = resolveSelectedFeatEntries(['feat:gorgons_fist'], CATALOG);
  assert(resolved.entry !== null, "an apostrophe in the catalog name (Gorgon's Fist) must not block matching its stripped-apostrophe engine token");
  assertEqual(resolved.entry?.key, "Gorgon's Fist", 'the resolved entry is the Gorgon\'s Fist catalog record');
}

function verifiesCompoundSubSelectionTokenResolvesToBaseFeat() {
  const [resolved] = resolveSelectedFeatEntries(['feat:weapon_focus:weapon:longsword'], CATALOG);
  assert(resolved.entry !== null, 'a compound token carrying a weapon sub-choice must still resolve to its base feat');
  assertEqual(resolved.entry?.key, 'Weapon Focus', 'the compound token resolves to the base Weapon Focus catalog entry');
}

function verifiesUnrecognizedRawStringFallsBackToNullEntryRatherThanDropping() {
  const [resolved] = resolveSelectedFeatEntries(['feat:some_non_crb_feat'], CATALOG);
  assertEqual(resolved.raw, 'feat:some_non_crb_feat', 'the raw string must be preserved for the fallback render');
  assertEqual(resolved.entry, null, 'an unresolved feat must be null, not silently dropped or fabricated');
}

function verifiesOrderAndCountAreParallelToInput() {
  const resolved = resolveSelectedFeatEntries(['Dodge', 'feat:unknown_thing', 'feat:deflect_arrows'], CATALOG);
  assertEqual(resolved.length, 3, 'one resolved row per input feat, in order');
  assertEqual(resolved[0].entry?.key, 'Dodge', 'first row resolves to Dodge');
  assertEqual(resolved[1].entry, null, 'second row is the unresolved fallback');
  assertEqual(resolved[2].entry?.key, 'Deflect Arrows', 'third row resolves to Deflect Arrows');
}

function main() {
  verifiesCatalogKeyFormatResolvesDirectly();
  verifiesEngineTokenFormatResolvesToSameCatalogEntry();
  verifiesApostropheInCatalogNameStillResolvesFromSlugToken();
  verifiesCompoundSubSelectionTokenResolvesToBaseFeat();
  verifiesUnrecognizedRawStringFallsBackToNullEntryRatherThanDropping();
  verifiesOrderAndCountAreParallelToInput();
  console.log('featsTabModel.test.ts: all assertions passed');
}

main();

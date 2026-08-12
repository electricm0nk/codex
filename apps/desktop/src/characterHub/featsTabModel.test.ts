import {
  describeFeatTarget,
  mergeChosenFeatTarget,
  normalizeFeatIdentity,
  resolveSelectedFeatEntries,
} from './featsTabModel';
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
 *    (`feat:power_attack` and `feat:weapon_focus` always, plus
 *    `feat:dodge` only when a seeded choice slot actually granted it) and
 *    what `pilot_compute.rs`'s gates match against. No normalization layer exists anywhere between the two
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


function verifiesAFeatWithNoTargetSaysNothingAboutTargets() {
  const [row] = resolveSelectedFeatEntries(['Dodge'], CATALOG, []);
  assertEqual(row.targetKind, null, 'Dodge takes no chosen target');
  assertEqual(row.targets.length, 0, 'and carries no targets');
  assertEqual(describeFeatTarget(row), null, 'so the row shows no target line at all');
}

function verifiesARecordedTargetIsShown() {
  const [row] = resolveSelectedFeatEntries(
    ['Weapon Focus'],
    CATALOG,
    [{ featId: 'Weapon Focus', targetKind: 'Weapon', targets: ['Longsword'] }]
  );
  assertEqual(row.targetKind, 'Weapon', 'the kind comes from the backend');
  assertEqual(describeFeatTarget(row), 'Longsword', 'the chosen weapon is shown');
}

function verifiesTheTargetJoinSurvivesTheTwoIdShapes() {
  // The backend reports `featId` verbatim, so it can arrive in the engine
  // token shape while `selectedFeats` holds the catalog key, or vice versa.
  const [row] = resolveSelectedFeatEntries(
    ['Weapon Focus'],
    CATALOG,
    [{ featId: 'feat:weapon_focus', targetKind: 'Weapon', targets: ['Longsword'] }]
  );
  assertEqual(describeFeatTarget(row), 'Longsword', 'the two id shapes must join');
}

function verifiesAHeldButUntargetedChooserFeatSaysSoExplicitly() {
  const [row] = resolveSelectedFeatEntries(
    ['Weapon Focus'],
    CATALOG,
    [{ featId: 'Weapon Focus', targetKind: 'Weapon', targets: [] }]
  );
  const described = describeFeatTarget(row);
  assert(described !== null, 'an untargeted chooser feat must not render silently as complete');
  assert(
    described!.includes('No weapon chosen'),
    `expected an explicit no-target message, got: ${described}`
  );
}

function verifiesBothTargetsOfARepeatedFeatAreShown() {
  const [row] = resolveSelectedFeatEntries(
    ['Weapon Focus'],
    CATALOG,
    [{ featId: 'Weapon Focus', targetKind: 'Weapon', targets: ['Longsword', 'Rapier'] }]
  );
  assertEqual(describeFeatTarget(row), 'Longsword, Rapier', 'both recorded targets are shown');
}

function verifiesMergingATargetNeverInventsOne() {
  const before = [{ featId: 'Weapon Focus', targetKind: 'Weapon', targets: ['Longsword'] }];
  assertEqual(
    mergeChosenFeatTarget(before, 'Skill Focus', null, null),
    before,
    'adding a feat with no target must leave the list untouched'
  );
}

function verifiesMergingAppendsToTheSameFeatRatherThanDuplicatingIt() {
  const merged = mergeChosenFeatTarget(
    [{ featId: 'Weapon Focus', targetKind: 'Weapon', targets: ['Longsword'] }],
    'Weapon Focus',
    'Rapier',
    'Weapon'
  );
  assertEqual(merged.length, 1, 'one entry per feat, not per pick');
  assertEqual(merged[0].targets.join(','), 'Longsword,Rapier', 'the second target is appended');
}

function verifiesMergingANewChooserFeatAddsAnEntry() {
  const merged = mergeChosenFeatTarget([], 'Skill Focus', 'Perception', 'Skill');
  assertEqual(merged.length, 1, 'a first target creates the entry');
  assertEqual(merged[0].targetKind, 'Skill', 'carrying the kind it was picked for');
}

/**
 * The exact table `src/rules_core/feat_identity.rs`'s own
 * `FRONTEND_SHAPE_CASES` pins, mirrored here verbatim.
 *
 * Display (this module) and effect resolution (the engine) must fold feat
 * identifiers identically: a shape that folds here but not there renders a
 * feat on the sheet whose producer never fires, which is exactly the defect
 * that left a player who picked Dodge with no armor class. Keeping the two
 * tables literally identical means a change to either fold fails one of them.
 */
const SHARED_IDENTITY_SHAPES: ReadonlyArray<readonly [string, string]> = [
  ['feat:deflect_arrows', 'deflectarrows'],
  ['Deflect Arrows', 'deflectarrows'],
  ['feat:weapon_focus:weapon:longsword', 'weaponfocus'],
  ["Gorgon's Fist", 'gorgonsfist'],
  ['feat:gorgons_fist', 'gorgonsfist'],
  ['Elemental Spell ~ Acid', 'elementalspellacid'],
  ['Elemental Spell (Acid)', 'elementalspellacid'],
];

function verifiesTheFoldMatchesTheEngineIdentityShapes() {
  for (const [raw, expected] of SHARED_IDENTITY_SHAPES) {
    assertEqual(
      normalizeFeatIdentity(raw),
      expected,
      `folding ${JSON.stringify(raw)} must match src/rules_core/feat_identity.rs`
    );
  }
}

function verifiesTheFoldIsWholeStringEqualityNotAPrefixMatch() {
  assert(
    normalizeFeatIdentity('Acrobatic Steps') !== normalizeFeatIdentity('Acrobatic'),
    'a longer feat whose name begins with another must not fold onto it'
  );
  assert(
    normalizeFeatIdentity('feat:greater_weapon_focus') !== normalizeFeatIdentity('Weapon Focus'),
    'Greater Weapon Focus is a different feat from Weapon Focus'
  );
}

function main() {
  verifiesCatalogKeyFormatResolvesDirectly();
  verifiesEngineTokenFormatResolvesToSameCatalogEntry();
  verifiesApostropheInCatalogNameStillResolvesFromSlugToken();
  verifiesCompoundSubSelectionTokenResolvesToBaseFeat();
  verifiesUnrecognizedRawStringFallsBackToNullEntryRatherThanDropping();
  verifiesOrderAndCountAreParallelToInput();
  verifiesAFeatWithNoTargetSaysNothingAboutTargets();
  verifiesARecordedTargetIsShown();
  verifiesTheTargetJoinSurvivesTheTwoIdShapes();
  verifiesAHeldButUntargetedChooserFeatSaysSoExplicitly();
  verifiesBothTargetsOfARepeatedFeatAreShown();
  verifiesMergingATargetNeverInventsOne();
  verifiesMergingAppendsToTheSameFeatRatherThanDuplicatingIt();
  verifiesMergingANewChooserFeatAddsAnEntry();
  verifiesTheFoldMatchesTheEngineIdentityShapes();
  verifiesTheFoldIsWholeStringEqualityNotAPrefixMatch();
  console.log('featsTabModel.test.ts: all assertions passed');
}

main();

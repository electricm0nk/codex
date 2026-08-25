import {
  buildClassFeatureSurface,
  matchesCorpusFeature,
  unmatchedClassFeatureDescriptions,
} from './classFeaturesModel';
import { assert, assertEqual } from '../testSupport/asserts';
import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';
import type { ClassFeatureDescriptionDto } from '../boundary/loadClassFeatureDescriptions';
import type { HeldClass } from './characterProgression';

const ROGUE: HeldClass[] = [{ classId: 'class:rogue', classLabel: 'Rogue', level: 11 }];

const SNEAK_ATTACK_DETAIL =
  'Rogue level 11 sneak attack from the PF1 Core Rulebook Rogue class table: the sneak ' +
  'attack die count increases by 1 every two rogue levels (1d6 at levels 1-2, 2d6 at level ' +
  '3+): (level + 1) / 2 = (11 + 1) / 2 = 6, i.e. 6d6 sneak attack damage die';

function explanation(id: string, value: number, detail = 'engine detail'): ExplanationDto {
  return { id, value, detail };
}

function verifiesALevel11RoguesSneakAttackKeepsItsMagnitudeAndCitation() {
  const surface = buildClassFeatureSurface(
    [explanation('class_chassis.rogue.sneak_attack', 6, SNEAK_ATTACK_DETAIL)],
    ROGUE
  );

  assertEqual(surface.features.length, 1, 'the sneak-attack record must reach the surface');
  assertEqual(surface.features[0].id, 'class_chassis.rogue.sneak_attack', 'id carried verbatim');
  assertEqual(surface.features[0].classToken, 'rogue', 'attributed to the held Rogue class');
  assertEqual(surface.features[0].label, 'Sneak Attack', 'label humanised from the id');
  assertEqual(surface.features[0].value, 6, 'PF1 sneak attack at Rogue 11 is 6 dice');
}

function verifiesTheDetailTextIsNeverRewritten() {
  const surface = buildClassFeatureSurface(
    [explanation('class_chassis.rogue.sneak_attack', 6, SNEAK_ATTACK_DETAIL)],
    ROGUE
  );

  assertEqual(
    surface.features[0].detail,
    SNEAK_ATTACK_DETAIL,
    'the engine detail is the rules citation and must cross byte-identical'
  );
  assert(
    surface.features[0].detail.includes('6d6'),
    'the dice expression the player needs is inside the engine detail'
  );
}

function verifiesNonClassRecordsAreIgnored() {
  const surface = buildClassFeatureSurface(
    [
      explanation('ability_modifier.strength', 4, 'STR 18 -> +4'),
      explanation('race.human.ability_bonus_applied', 18, 'Human +2 racial'),
      explanation('class_feature.rogue.evasion', 1, 'Rogue level 2 Evasion'),
    ],
    ROGUE
  );

  assertEqual(surface.features.length, 1, 'only the class record is a class feature');
  assertEqual(surface.features[0].id, 'class_feature.rogue.evasion', 'the class record survives');
}

function verifiesUnsupportedNoticesNeverRenderTheirFillerZeroAsAMagnitude() {
  const surface = buildClassFeatureSurface(
    [
      explanation('class_chassis.barbarian.rage_rounds_per_day', 8, 'Barbarian level 2 rage'),
      explanation(
        'class_chassis.barbarian.rage_rounds_per_day.unsupported',
        0,
        'no rage-round budget is grounded for this posture'
      ),
    ],
    [{ classId: 'class:barbarian', classLabel: 'Barbarian', level: 2 }]
  );

  assertEqual(surface.features.length, 1, 'only the grounded record is a feature row');
  assertEqual(
    surface.features[0].id,
    'class_chassis.barbarian.rage_rounds_per_day',
    'the grounded record keeps its magnitude'
  );
  assertEqual(surface.notComputed.length, 1, 'the unsupported record becomes a notice');
  assertEqual(surface.notComputed[0].label, 'Rage Rounds Per Day', 'notice label drops the suffix');
  assertEqual(
    surface.notComputed[0].detail,
    'no rage-round budget is grounded for this posture',
    'the engine explains the absence in its own words'
  );
  assert(
    !Object.prototype.hasOwnProperty.call(surface.notComputed[0], 'value'),
    'a notice carries no value at all, so no caller can render the filler zero'
  );
}

function verifiesPreNamespacingChassisIdsGetNoGuessedOwner() {
  const surface = buildClassFeatureSurface(
    [
      explanation('class_chassis.base_attack_bonus', 5, 'Fighter level 5 BAB'),
      explanation('class_chassis.base_save.will', 1, 'Fighter level 5 Will'),
    ],
    [{ classId: 'class:fighter', classLabel: 'Fighter', level: 5 }]
  );

  assertEqual(surface.features[0].classToken, null, 'no class segment means no owner is invented');
  assertEqual(surface.features[0].label, 'Base Attack Bonus', 'the whole remainder is the label');
  assertEqual(surface.features[1].label, 'Base Save Will', 'dotted segments humanise too');
}

function verifiesARecordIsOnlyAttributedToAClassTheCharacterHolds() {
  // `wizard` is a real class segment, but this character is a Rogue — the
  // segment must not be silently stripped as if it were the owner.
  const surface = buildClassFeatureSurface(
    [explanation('class_chassis.wizard.scribe_scroll', 1, 'Wizard Scribe Scroll')],
    ROGUE
  );

  assertEqual(surface.features[0].classToken, null, 'an unheld class is not treated as the owner');
  assertEqual(surface.features[0].label, 'Wizard Scribe Scroll', 'the segment stays in the label');
}

function verifiesTheEngineEmissionOrderIsPreserved() {
  const surface = buildClassFeatureSurface(
    [
      explanation('class_chassis.rogue.base_attack_bonus', 8),
      explanation('class_chassis.rogue.trapfinding', 5),
      explanation('class_chassis.rogue.sneak_attack', 6),
    ],
    ROGUE
  );

  assertEqual(
    surface.features.map((row) => row.label).join(' | '),
    'Base Attack Bonus | Trapfinding | Sneak Attack',
    'rows stay in the order the engine emitted them'
  );
}

function verifiesABuildWithNoClassRecordsGetsCleanEmptySurfaces() {
  const surface = buildClassFeatureSurface([], ROGUE);

  assertEqual(surface.features.length, 0, 'no features');
  assertEqual(surface.notComputed.length, 0, 'no notices');
}

// ---------------------------------------------------------------------------
// SD-27: a book-namespaced id (`class_feature.<book>.<class>.…`) is attributed
// to its class, not swallowed into the label.
// ---------------------------------------------------------------------------

const UNCHAINED_SUMMONER: HeldClass[] = [
  { classId: 'class:unchained_summoner', classLabel: 'Unchained Summoner', level: 20 },
];

/**
 * The defect, verbatim: every one of Pathfinder Unchained's receipt rows
 * rendered as `Pu Unchained Summoner Bond Senses Rounds Per Day` under a
 * `Chassis` gutter, because `class_feature.pu.<class>.*` puts the literal book
 * segment `pu` in the position `splitId` matched against held classes.
 */
function verifiesABookNamespacedIdIsAttributedToItsClassRatherThanToNoOne() {
  const surface = buildClassFeatureSurface(
    [explanation('class_feature.pu.unchained_summoner.bond_senses_rounds_per_day', 20)],
    UNCHAINED_SUMMONER
  );

  assertEqual(
    surface.features[0].classToken,
    'unchained_summoner',
    'the class segment sits behind a book namespace and must still be found'
  );
  assertEqual(
    surface.features[0].label,
    'Bond Senses Rounds Per Day',
    'the label is the feature, with neither the book nor the class in it'
  );
  assertEqual(
    surface.features[0].classLabel,
    'Unchained Summoner',
    "the gutter reads the held class's own label, not its raw id token"
  );
}

/**
 * The per-record roster rows carry an extra `corpus_record` segment naming the
 * record family. It is id structure, not feature text, and must not reach the
 * label.
 *
 * The `makers_call` id here is the one the engine really emits
 * (`pilot_compute::pu_feature_slug` swallows the apostrophe in
 * `Unchained Summoner ~ Maker's Call`). It previously read `maker_s_call` and
 * asserted the label `'Maker S Call'` — an id the engine can no longer produce
 * and the rendering of a fixed defect. A synthetic fixture that pins an
 * impossible input proves nothing, so it was corrected rather than left to pass.
 */
function verifiesTheRecordFamilySegmentIsNotPartOfTheFeatureName() {
  const surface = buildClassFeatureSurface(
    [
      explanation('class_feature.pu.unchained_summoner.corpus_record.makers_call', 6),
      explanation('class_feature.pu.unchained_summoner.corpus_record.greater_shield_ally', 12),
    ],
    UNCHAINED_SUMMONER
  );

  assertEqual(surface.features[0].label, 'Makers Call', 'the record family segment is dropped');
  assertEqual(surface.features[1].label, 'Greater Shield Ally', 'and dropped on every row');
  assertEqual(surface.features[0].classToken, 'unchained_summoner', 'still attributed');
}

/** The same treatment must reach the "Not computed" lane. */
function verifiesABookNamespacedNoticeIsAlsoAttributedAndLabelled() {
  const surface = buildClassFeatureSurface(
    [
      explanation(
        'class_feature.pu.unchained_summoner.other_features_deferred.unsupported',
        0,
        'the engine explains the deferral'
      ),
      explanation('class_feature.pu.unchained_summoner.corpus_record.transposition.unsupported', 0),
    ],
    UNCHAINED_SUMMONER
  );

  assertEqual(surface.notComputed.length, 2, 'both notices survive');
  assertEqual(surface.notComputed[0].label, 'Other Features Deferred', 'notice label');
  assertEqual(surface.notComputed[0].classLabel, 'Unchained Summoner', 'notice gutter');
  assertEqual(surface.notComputed[1].label, 'Transposition', 'record-family segment dropped here too');
}

/**
 * The namespace scan must not invent an owner. A book-namespaced record for a
 * class this character does not hold keeps every segment in its label, exactly
 * as an unheld class segment already does.
 */
function verifiesABookNamespacedIdForAnUnheldClassStillGetsNoGuessedOwner() {
  const surface = buildClassFeatureSurface(
    [explanation('class_feature.pu.unchained_monk.ki_points', 6)],
    UNCHAINED_SUMMONER
  );

  assertEqual(surface.features[0].classToken, null, 'a class the character does not hold is not the owner');
  assertEqual(surface.features[0].classLabel, null, 'and carries no label to render in the gutter');
  assertEqual(surface.features[0].label, 'Pu Unchained Monk Ki Points', 'nothing is silently stripped');
}

/**
 * The scan is bounded to one namespace segment, so a class token appearing
 * deeper in an id — inside a feature name — can never be mistaken for the
 * owner.
 */
function verifiesTheNamespaceScanIsBoundedToOneSegment() {
  const surface = buildClassFeatureSurface(
    [explanation('class_feature.pu.extra.unchained_summoner.something', 1)],
    UNCHAINED_SUMMONER
  );

  assertEqual(surface.features[0].classToken, null, 'a class token two namespaces deep is not the owner');
  assertEqual(
    surface.features[0].label,
    'Pu Extra Unchained Summoner Something',
    'and the whole remainder stays in the label'
  );
}

/** The pre-namespacing single-segment ids must be untouched by all of this. */
function verifiesTheUnnamespacedIdsKeepTheirExistingAttribution() {
  const surface = buildClassFeatureSurface(
    [
      explanation('class_feature.rogue.evasion', 1),
      explanation('class_chassis.rogue.sneak_attack', 6),
    ],
    ROGUE
  );

  assertEqual(surface.features[0].classToken, 'rogue', 'class in position 0 still resolves');
  assertEqual(surface.features[0].classLabel, 'Rogue', 'and carries its held label');
  assertEqual(surface.features[0].label, 'Evasion', 'label unchanged');
  assertEqual(surface.features[1].label, 'Sneak Attack', 'label unchanged');
}

// --- SD31-D7-PROSE-003: the corpus-description join ------------------------

function descriptionDto(
  classSlug: string,
  featureSlug: string,
  description: string,
  grantedFeat: string | null = null
): ClassFeatureDescriptionDto {
  return {
    book: 'core_rulebook',
    classSlug,
    featureSlug,
    key: `${classSlug} ~ ${featureSlug}`,
    name: featureSlug,
    description,
    grantedFeat,
  };
}

const SNEAK_ATTACK_DESC =
  'This is exactly like the rogue ability of the same name, except that the attack must be made ' +
  'with a sneak attack from hiding or while the target is denied its Dexterity bonus.';

function verifiesMatchesCorpusFeatureRequiresTheClassSegmentAndTheTrailingSlug() {
  assert(
    matchesCorpusFeature('class_chassis.rogue.sneak_attack', 'rogue', 'sneak_attack'),
    'a plain single-segment id must match its own class and feature slug'
  );
  assert(
    matchesCorpusFeature(
      'class_feature.pu.unchained_summoner.corpus_record.bond_senses_rounds_per_day',
      'unchained_summoner',
      'bond_senses_rounds_per_day'
    ),
    'a book-namespaced id with a corpus_record segment still matches by suffix'
  );
  assert(
    !matchesCorpusFeature('class_chassis.cleric.sneak_attack', 'rogue', 'sneak_attack'),
    'a different class segment must not match'
  );
  assert(
    !matchesCorpusFeature('class_chassis.rogue.evasion', 'rogue', 'sneak_attack'),
    'a different trailing feature slug must not match'
  );
}

function verifiesARealClassFeatureRowJoinsToItsMatchingCorpusDescription() {
  const surface = buildClassFeatureSurface(
    [explanation('class_chassis.rogue.sneak_attack', 6, SNEAK_ATTACK_DETAIL)],
    ROGUE,
    [descriptionDto('rogue', 'sneak_attack', SNEAK_ATTACK_DESC)]
  );

  assertEqual(
    surface.features[0].corpusDescription,
    SNEAK_ATTACK_DESC,
    'the real corpus description must join onto the matching row, byte for byte'
  );
  assertEqual(
    surface.features[0].detail,
    SNEAK_ATTACK_DETAIL,
    'the engine detail is untouched by the new field'
  );
}

/** PROVE THE JOIN CAN FAIL, case 1: no candidate at all (empty description population). */
function verifiesNoMatchingCandidateLeavesCorpusDescriptionNull() {
  const surface = buildClassFeatureSurface(
    [explanation('class_chassis.rogue.sneak_attack', 6)],
    ROGUE,
    []
  );
  assertEqual(surface.features[0].corpusDescription, null, 'no candidate means no description');
}

/**
 * PROVE THE JOIN CAN FAIL, case 2 (description not matching the corpus
 * row): a candidate that shares the feature slug but belongs to a
 * DIFFERENT class must never attach -- the exact `decisions.md §10` shared-
 * NAME hazard the join's own class-segment check exists to prevent.
 */
function verifiesAWrongClassCandidateNeverAttachesEvenWithTheSameFeatureSlug() {
  const surface = buildClassFeatureSurface(
    [explanation('class_chassis.rogue.sneak_attack', 6)],
    ROGUE,
    [descriptionDto('ninja', 'sneak_attack', 'A ninja ability of the same name.')]
  );
  assertEqual(
    surface.features[0].corpusDescription,
    null,
    'a same-named feature under a different class must never attach'
  );
}

/**
 * PROVE THE JOIN CAN FAIL, case 3: a chassis record with no class token
 * (`classToken === null`) never attaches a description, even when a
 * same-named candidate exists -- there is no `classSlug` to gate the match,
 * and matching by feature slug alone is the shared-NAME hazard.
 */
function verifiesAChasisRecordWithNoClassTokenNeverAttachesADescription() {
  const surface = buildClassFeatureSurface(
    [explanation('class_chassis.base_attack_bonus', 5)],
    ROGUE,
    [descriptionDto('rogue', 'base_attack_bonus', 'Some unrelated rogue feature.')]
  );
  assertEqual(surface.features[0].classToken, null, 'no held class matched this id');
  assertEqual(
    surface.features[0].corpusDescription,
    null,
    'a chassis record with no class token must never guess a description'
  );
}

/**
 * SD-31 wave 29 (THE-BOX.md §2.1 F1, load-bearing negative finding, RE-
 * CONFIRMED against this file rather than assumed): a `ClassFeatureDescriptionDto`
 * the corpus-description catalog (`class_feature_descriptions.rs`) can render
 * for a record does NOT, by itself, put that record on the character sheet.
 * `buildClassFeatureSurface` only ever creates a `features`/`notComputed` row
 * by iterating `explanations` (see the `for (const explanation of
 * explanations)` loop above) -- `descriptions` is consulted only as
 * enrichment on a row that already exists. A record with no matching
 * `ExplanationDto` (exactly what `v06_work_inventory.rs`'s G1/G2 groups are)
 * produces ZERO rows, no matter how permissive `classify()`'s eligibility
 * check becomes or how many corpus descriptions the catalog can render for
 * it.
 *
 * This pins the reason wave 29's F1 widening (crediting `class_feature`
 * `text-complete` off `class_feature_descriptions.rs`'s render catalog alone,
 * the same way `class_feature_pool_catalog_holds` already credits pool
 * members) was correctly declined: unlike
 * `ClassFeaturePoolReferenceSection` (`CharacterSheet.tsx`), which renders
 * `loadClassFeaturePoolOptions()`'s FULL catalog independent of
 * `props.explanations`, there is no equivalent browsable reference surface
 * for `loadClassFeatureDescriptions()` -- it is wired ONLY as the
 * `corpusDescription` join inside this function. Widening `classify()` to
 * treat the catalog's presence as proof-of-holds without first building that
 * surface would credit `done` for a description no player screen ever
 * shows -- the exact gap Decision 7 condition 3 / DoD-8 forbid crediting on.
 *
 * If a future cycle builds that reference surface, THIS test's assertion
 * flips (0 -> a real row count) and should be revisited alongside it, not
 * deleted quietly.
 */
function verifiesADescriptionWithNoMatchingExplanationProducesNoRowAtAllRegardlessOfHowManyDescriptionsExist() {
  const surface = buildClassFeatureSurface(
    [], // no engine explanations at all -- exactly the G1/G2 shape
    ROGUE,
    [
      descriptionDto('rogue', 'sneak_attack', SNEAK_ATTACK_DESC),
      descriptionDto('rogue', 'evasion', 'Some other real rogue feature description.'),
    ]
  );
  assertEqual(
    surface.features.length,
    0,
    'a description with no matching explanation must not manufacture a feature row'
  );
  assertEqual(
    surface.notComputed.length,
    0,
    'a description with no matching explanation must not manufacture a not-computed notice either'
  );
}

/**
 * T4 closure (`epic-breakdown.md` Epic 2, "built-but-unreachable render
 * surface"). This is the OTHER half of the test directly above:
 * `buildClassFeatureSurface` alone still produces zero rows for an
 * unmatched description (unchanged, correctly), but
 * `unmatchedClassFeatureDescriptions` is the new reference-list surface
 * that DOES surface it — the fix site the comment above named as missing.
 */
function verifiesAnUnmatchedDescriptionForAHeldClassIsReturnedByTheReferenceSurface() {
  const unmatched = unmatchedClassFeatureDescriptions(
    [], // no engine explanations at all -- exactly the shape above
    ROGUE,
    [descriptionDto('rogue', 'sneak_attack', SNEAK_ATTACK_DESC)]
  );
  assertEqual(unmatched.length, 1, 'a real corpus description for a held class with no explanation must surface');
  assertEqual(unmatched[0].description, SNEAK_ATTACK_DESC, 'the real corpus text, verbatim');
}

/** A description already attached to a grounded feature row must not also appear in the reference list -- no duplicate display. */
function verifiesADescriptionAlreadyAttachedToAGroundedRowIsNotDuplicatedInTheReferenceSurface() {
  const unmatched = unmatchedClassFeatureDescriptions(
    [explanation('class_chassis.rogue.sneak_attack', 6)],
    ROGUE,
    [descriptionDto('rogue', 'sneak_attack', SNEAK_ATTACK_DESC)]
  );
  assertEqual(unmatched.length, 0, 'already-shown-as-enrichment descriptions must not duplicate in the reference list');
}

/**
 * A description matching only an `.unsupported` explanation is STILL
 * unreachable today (`buildClassFeatureSurface` never attaches
 * `corpusDescription` inside the `notComputed` loop) -- it must still
 * surface here, not be wrongly excluded as though it were already shown.
 */
function verifiesADescriptionMatchingOnlyAnUnsupportedNoticeStillSurfaces() {
  const unmatched = unmatchedClassFeatureDescriptions(
    [explanation('class_chassis.rogue.sneak_attack.unsupported', 0, 'not grounded for this posture')],
    ROGUE,
    [descriptionDto('rogue', 'sneak_attack', SNEAK_ATTACK_DESC)]
  );
  assertEqual(
    unmatched.length,
    1,
    'an .unsupported match never attaches a description anywhere else, so it must still surface here'
  );
}

/** A description for a class this build does not hold must never surface. */
function verifiesADescriptionForAnUnheldClassNeverSurfaces() {
  const unmatched = unmatchedClassFeatureDescriptions(
    [],
    ROGUE,
    [descriptionDto('barbarian', 'rage', 'A barbarian feature this build does not hold.')]
  );
  assertEqual(unmatched.length, 0, 'an unheld class must never surface a reference row');
}

// --- T4-L9 (`decisions.md §13`): the feat-held reachability arm ------------
//
// `class_feature_feat_bridge.rs`'s 471-record population carries a synthetic
// pool-group `classSlug` (e.g. `"golden_legionnaire"`), never a real class
// token -- these four tests prove the SECOND, feat-held gate this cycle adds,
// mirroring the four class-held cases directly above rather than replacing
// them.

/**
 * The core positive case: a bridge record (`grantedFeat` set, synthetic
 * `classSlug`) surfaces when the character holds the granted feat, even
 * though `heldClasses` names no class matching `classSlug` at all.
 */
function verifiesABridgeRecordSurfacesWhenTheCharacterHoldsTheGrantedFeat() {
  const unmatched = unmatchedClassFeatureDescriptions(
    [],
    ROGUE, // holds no class named "golden_legionnaire"
    [descriptionDto('golden_legionnaire', 'swift_aid', 'With a quick but harmless swipe...', 'Swift Aid')],
    ['Swift Aid']
  );
  assertEqual(
    unmatched.length,
    1,
    'a bridge record must surface on the feat-held arm even though no held class matches its synthetic classSlug'
  );
  assertEqual(unmatched[0].grantedFeat, 'Swift Aid', 'the granted feat identity is carried through unchanged');
}

/** The negative twin: the same record must NOT surface when the feat is not held. */
function verifiesABridgeRecordNeverSurfacesWhenTheGrantedFeatIsNotHeld() {
  const unmatched = unmatchedClassFeatureDescriptions(
    [],
    ROGUE,
    [descriptionDto('golden_legionnaire', 'swift_aid', 'With a quick but harmless swipe...', 'Swift Aid')],
    ['Dodge', 'Toughness']
  );
  assertEqual(unmatched.length, 0, 'a bridge record must never surface when its granted feat is not held');
}

/**
 * The identity comparison must fold both real shapes `selectedFeats` can
 * carry -- the catalog key (`"Swift Aid"`) and the engine's lowercase
 * `feat:`-prefixed token (`"feat:swift_aid"`) -- exactly like
 * `feat_identity.rs::holds` does on the Rust side. A caller passing either
 * shape must get the same reachability answer.
 */
function verifiesTheFeatHeldCheckFoldsBothSelectedFeatsShapes() {
  const unmatched = unmatchedClassFeatureDescriptions(
    [],
    ROGUE,
    [descriptionDto('golden_legionnaire', 'swift_aid', 'With a quick but harmless swipe...', 'Swift Aid')],
    ['feat:swift_aid']
  );
  assertEqual(
    unmatched.length,
    1,
    'the engine token shape ("feat:swift_aid") must fold to the same identity as the catalog key ("Swift Aid")'
  );
}

/**
 * A bridge record must never surface merely because the character happens
 * to hold a class whose token equals `classSlug` -- for these records
 * `classSlug` is a synthetic pool-group name, and the class-held arm must
 * stay disabled whenever `grantedFeat` is present, even in the
 * (unrealistic, but not to be trusted blindly) case that a real class
 * shares that token.
 */
function verifiesABridgeRecordIgnoresTheClassHeldArmEvenIfClassSlugCoincidentallyMatchesAHeldClass() {
  const heldRogueAsGoldenLegionnaire: HeldClass[] = [
    { classId: 'class:golden_legionnaire', classLabel: 'Golden Legionnaire', level: 1 },
  ];
  const unmatched = unmatchedClassFeatureDescriptions(
    [],
    heldRogueAsGoldenLegionnaire,
    [descriptionDto('golden_legionnaire', 'swift_aid', 'With a quick but harmless swipe...', 'Swift Aid')],
    [] // the feat itself is NOT held
  );
  assertEqual(
    unmatched.length,
    0,
    'a bridge record (grantedFeat present) must be gated on the held feat only, never on classSlug, even ' +
      'when a held class token happens to equal it'
  );
}

async function main() {
  verifiesALevel11RoguesSneakAttackKeepsItsMagnitudeAndCitation();
  verifiesTheDetailTextIsNeverRewritten();
  verifiesNonClassRecordsAreIgnored();
  verifiesUnsupportedNoticesNeverRenderTheirFillerZeroAsAMagnitude();
  verifiesPreNamespacingChassisIdsGetNoGuessedOwner();
  verifiesARecordIsOnlyAttributedToAClassTheCharacterHolds();
  verifiesTheEngineEmissionOrderIsPreserved();
  verifiesABuildWithNoClassRecordsGetsCleanEmptySurfaces();
  verifiesABookNamespacedIdIsAttributedToItsClassRatherThanToNoOne();
  verifiesTheRecordFamilySegmentIsNotPartOfTheFeatureName();
  verifiesABookNamespacedNoticeIsAlsoAttributedAndLabelled();
  verifiesABookNamespacedIdForAnUnheldClassStillGetsNoGuessedOwner();
  verifiesTheNamespaceScanIsBoundedToOneSegment();
  verifiesTheUnnamespacedIdsKeepTheirExistingAttribution();
  verifiesMatchesCorpusFeatureRequiresTheClassSegmentAndTheTrailingSlug();
  verifiesARealClassFeatureRowJoinsToItsMatchingCorpusDescription();
  verifiesNoMatchingCandidateLeavesCorpusDescriptionNull();
  verifiesAWrongClassCandidateNeverAttachesEvenWithTheSameFeatureSlug();
  verifiesAChasisRecordWithNoClassTokenNeverAttachesADescription();
  verifiesADescriptionWithNoMatchingExplanationProducesNoRowAtAllRegardlessOfHowManyDescriptionsExist();
  verifiesAnUnmatchedDescriptionForAHeldClassIsReturnedByTheReferenceSurface();
  verifiesADescriptionAlreadyAttachedToAGroundedRowIsNotDuplicatedInTheReferenceSurface();
  verifiesADescriptionMatchingOnlyAnUnsupportedNoticeStillSurfaces();
  verifiesADescriptionForAnUnheldClassNeverSurfaces();
  verifiesABridgeRecordSurfacesWhenTheCharacterHoldsTheGrantedFeat();
  verifiesABridgeRecordNeverSurfacesWhenTheGrantedFeatIsNotHeld();
  verifiesTheFeatHeldCheckFoldsBothSelectedFeatsShapes();
  verifiesABridgeRecordIgnoresTheClassHeldArmEvenIfClassSlugCoincidentallyMatchesAHeldClass();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

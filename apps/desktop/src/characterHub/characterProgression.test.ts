import {
  buildLevelEntries,
  buildNextEntries,
  casterLevel,
  classHitDie,
  classSkillPointsBase,
  classWeaponProficiency,
  formatHeldClasses,
  levelGrantsFeat,
  maxHitPoints,
  parseHeldClasses,
  previewLevelUp,
  totalCharacterLevel,
  totalSkillPoints,
  type HeldClass,
} from './characterProgression';
import { assert, assertEqual } from '../testSupport/asserts';

async function main() {
  verifiesParseHeldClassesSingleClass();
  verifiesParseHeldClassesRealMulticlassSummary();
  verifiesParseHeldClassesFallsBackToDerivedLabelForAnUnknownClass();
  verifiesFormatHeldClassesJoinsWithSlash();
  verifiesTotalCharacterLevelSumsAcrossClasses();
  verifiesCasterLevelOnlyCountsFullCasterClasses();
  verifiesClassSkillPointsBaseAndDefault();
  verifiesClassHitDieAndDefault();
  verifiesClassWeaponProficiency();
  verifiesMaxHitPointsSingleClass();
  verifiesMaxHitPointsMulticlassOnlyMaximizesTheVeryFirstLevel();
  verifiesTotalSkillPointsFlooredAndHumanBonus();
  verifiesBuildLevelEntriesNumbersByCharacterLevel();
  verifiesLevelEntriesCarryAbilityIncreasesAtEveryFourthLevel();
  verifiesBuildNextEntriesPreviewsEachHeldClassAtTheNextTotalLevel();
  verifiesPreviewLevelUpForAnExistingClassAndForANewClass();
  verifiesLevelGrantsFeatDetectsEveryRealFeatSource();
  verifiesFighterEvenLevelBonusFeatIsNoLongerClaimedLocally();
}

/**
 * Regression guard for d03bc89d: `summarize_envelope` (character_hub.rs)
 * joins multiclass segments with a comma, not a slash. Before that fix,
 * `parseHeldClasses` assumed `/` and silently mis-parsed every multiclass
 * character into one garbled pseudo-class, corrupting HP, skill points,
 * caster level, and the Progression rail for any real multiclass build
 * while single-class characters (no separator needed) looked completely
 * fine. This file previously had zero test coverage for that bug or for
 * this parser at all.
 */
function verifiesParseHeldClassesRealMulticlassSummary() {
  const held = parseHeldClasses('class:fighter:3,class:wizard:1');

  assertEqual(held.length, 2, 'a real multiclass classSummary should parse into two held classes');
  assertEqual(held[0].classId, 'class:fighter', 'first held class id');
  assertEqual(held[0].classLabel, 'Fighter', 'first held class label');
  assertEqual(held[0].level, 3, 'first held class level');
  assertEqual(held[1].classId, 'class:wizard', 'second held class id');
  assertEqual(held[1].classLabel, 'Wizard', 'second held class label');
  assertEqual(held[1].level, 1, 'second held class level');
}

function verifiesParseHeldClassesSingleClass() {
  const held = parseHeldClasses('class:rogue:2');

  assertEqual(held.length, 1, 'a single-class classSummary should parse into one held class');
  assertEqual(held[0].classId, 'class:rogue', 'classId');
  assertEqual(held[0].classLabel, 'Rogue', 'classLabel');
  assertEqual(held[0].level, 2, 'level');
}

function verifiesParseHeldClassesFallsBackToDerivedLabelForAnUnknownClass() {
  const held = parseHeldClasses('class:some_future_class:1');

  assertEqual(held.length, 1, 'an unrecognized class id should still parse, not be dropped');
  // `\b\w` word-boundary capitalization only fires at a true regex word
  // boundary, and `_` counts as a word character -- so "some_future_class"
  // is one contiguous word to the regex, and only its leading "s" gets
  // capitalized. Documenting the real behavior, not the (wrong) assumption
  // that underscores become spaces.
  assertEqual(held[0].classLabel, 'Some_future_class', 'an unknown class derives its label from the id segment, capitalizing only the leading letter');
}

function verifiesFormatHeldClassesJoinsWithSlash() {
  assertEqual(
    formatHeldClasses('class:fighter:3,class:wizard:1'),
    'Fighter 3 / Wizard 1',
    'formatHeldClasses should join multiple held classes with " / "'
  );
  assertEqual(formatHeldClasses('class:rogue:2'), 'Rogue 2', 'formatHeldClasses on a single class has no separator');
}

function verifiesTotalCharacterLevelSumsAcrossClasses() {
  assertEqual(totalCharacterLevel('class:fighter:3,class:wizard:1'), 4, 'total character level sums every held class');
  assertEqual(totalCharacterLevel('class:rogue:2'), 2, 'total character level for a single class is just its level');
}

function verifiesCasterLevelOnlyCountsFullCasterClasses() {
  assertEqual(
    casterLevel('class:fighter:3,class:wizard:1'),
    1,
    'caster level only sums full-caster classes (Wizard), not Fighter'
  );
  assertEqual(casterLevel('class:wizard:2,class:sorcerer:3'), 5, 'caster level sums across multiple caster classes held at once');
  assertEqual(casterLevel('class:fighter:5'), 0, 'a character with no caster class has caster level 0');
  // Arcanist is a full arcane caster (ACG): its caster level is its class
  // level, exactly like Wizard's. Now that it is selectable in CLASS_OPTIONS,
  // omitting it here would silently show every Arcanist a caster level of 0.
  assertEqual(casterLevel('class:arcanist:7'), 7, 'Arcanist is a full caster — caster level equals its class level');
  assertEqual(casterLevel('class:fighter:2,class:arcanist:3'), 3, 'a Fighter/Arcanist counts only the Arcanist levels');
}

function verifiesClassSkillPointsBaseAndDefault() {
  assertEqual(classSkillPointsBase('class:rogue'), 8, 'Rogue has 8 base skill points per level');
  assertEqual(classSkillPointsBase('class:fighter'), 2, 'Fighter has 2 base skill points per level');
  assertEqual(classSkillPointsBase('class:some_future_class'), 2, 'an unrecognized class defaults to 2 base skill points');
  // ACG Arcanist: 3 + Int modifier skill ranks per level. Without its own
  // entry it would fall through to the unrecognized-class default of 2 and
  // quietly under-report every Arcanist's skill ranks.
  assertEqual(classSkillPointsBase('class:arcanist'), 3, 'Arcanist has 3 base skill points per level');
}

function verifiesClassHitDieAndDefault() {
  assertEqual(classHitDie('class:wizard'), 6, 'Wizard has a d6 hit die');
  assertEqual(classHitDie('class:fighter'), 10, 'Fighter has a d10 hit die');
  assertEqual(classHitDie('class:arcanist'), 6, 'Arcanist has a d6 hit die');
  assertEqual(classHitDie('class:some_future_class'), 8, 'an unrecognized class defaults to a d8 hit die');
}

function verifiesClassWeaponProficiency() {
  const fighter = classWeaponProficiency('class:fighter');
  assert(fighter.simple && fighter.martial && !fighter.exotic, 'Fighter is proficient with simple and martial weapons, never exotic by default');

  const wizard = classWeaponProficiency('class:wizard');
  assert(wizard.simple && !wizard.martial && !wizard.exotic, 'Wizard is only proficient with simple weapons by default');
}

function verifiesMaxHitPointsSingleClass() {
  const heldClasses: HeldClass[] = [{ classId: 'class:fighter', classLabel: 'Fighter', level: 3 }];
  // Level 1 takes the max die (10) + CON mod; levels 2-3 take the average (floor(10/2)+1=6) + CON mod each.
  assertEqual(maxHitPoints(heldClasses, 2), 10 + 2 + (6 + 2) + (6 + 2), 'Fighter 3 max HP with +2 CON: max die then average per later level, CON mod every level');
}

function verifiesMaxHitPointsMulticlassOnlyMaximizesTheVeryFirstLevel() {
  // Only the character's very first level ever (Fighter 1) takes the max die;
  // every level after -- including the first level of a later class dipped
  // into -- takes that class's own average.
  const heldClasses: HeldClass[] = [
    { classId: 'class:fighter', classLabel: 'Fighter', level: 1 },
    { classId: 'class:rogue', classLabel: 'Rogue', level: 1 },
  ];
  const expected = (10 + 0) + (Math.floor(8 / 2) + 1 + 0);
  assertEqual(maxHitPoints(heldClasses, 0), expected, 'only the character\'s first-ever level maximizes its hit die; a later multiclass dip uses the average');
}

function verifiesTotalSkillPointsFlooredAndHumanBonus() {
  assertEqual(totalSkillPoints(2, 1, false), 3, 'base + positive Int modifier');
  assertEqual(totalSkillPoints(2, 1, true), 4, 'Human gets +1 bonus skill point per level');
  assertEqual(totalSkillPoints(2, -5, false), 1, 'skill points per level are floored at 1, never negative or zero');
}

/**
 * `LevelEntry.features` now carries only the universal PF1 benefits — a
 * feat at every odd character level, an ability score increase every 4th.
 * Class features are no longer authored here at all: they come from the
 * engine's own `class_feature.*` / `class_chassis.*` records
 * (`classFeaturesModel.ts`), which carry real magnitudes and real
 * citations that the deleted hand-authored table never had.
 */
function verifiesBuildLevelEntriesNumbersByCharacterLevel() {
  const heldClasses: HeldClass[] = [{ classId: 'class:fighter', classLabel: 'Fighter', level: 2 }];
  const entries = buildLevelEntries(heldClasses);

  assertEqual(entries.length, 2, 'Fighter 2 should produce two level entries');
  assertEqual(entries[0].characterLevel, 1, 'first entry is character level 1');
  assertEqual(entries[0].classLevel, 1, 'first entry is class level 1');
  assert(entries[0].features.includes('Feat'), 'character level 1 is odd, so a Feat is granted');
  assertEqual(entries[1].characterLevel, 2, 'second entry is character level 2');
  assertEqual(entries[1].classLevel, 2, 'second entry is class level 2');
  assert(!entries[1].features.includes('Feat'), 'character level 2 is even, so no Feat is granted');
  assertEqual(
    entries[1].features.length,
    0,
    'no class-feature string is authored here — a class level with no universal benefit carries nothing'
  );
}

function verifiesLevelEntriesCarryAbilityIncreasesAtEveryFourthLevel() {
  const entries = buildLevelEntries([{ classId: 'class:fighter', classLabel: 'Fighter', level: 4 }]);

  assert(
    entries[3].features.includes('Ability score increase'),
    'character level 4 grants an ability score increase — a PF1 general rule, not a class-table entry'
  );
  assert(
    !entries[2].features.includes('Ability score increase'),
    'character level 3 does not'
  );
}

function verifiesBuildNextEntriesPreviewsEachHeldClassAtTheNextTotalLevel() {
  const heldClasses: HeldClass[] = [{ classId: 'class:fighter', classLabel: 'Fighter', level: 2 }];
  const next = buildNextEntries(heldClasses);

  assertEqual(next.length, 1, 'one held class produces one next-level preview');
  assertEqual(next[0].classLevel, 3, 'the next Fighter level is class level 3');
  assertEqual(next[0].characterLevel, 3, 'the next level is also character level 3 for a single-class build');
  assert(
    next[0].features.includes('Feat'),
    'character level 3 is odd, so the universal feat is previewed'
  );
  // What Fighter 3 grants as a *class feature* (Armor Training 1) is the
  // engine's answer, delivered by `preview_level_up`, not this module's.
}

function verifiesPreviewLevelUpForAnExistingClassAndForANewClass() {
  const heldClasses: HeldClass[] = [{ classId: 'class:fighter', classLabel: 'Fighter', level: 2 }];

  const continuing = previewLevelUp(heldClasses, 'class:fighter');
  assertEqual(continuing.classLevel, 3, 'leveling up an already-held class increments its own class level');
  assertEqual(continuing.characterLevel, 3, 'leveling up any class advances the total character level by one');

  const dip = previewLevelUp(heldClasses, 'class:rogue');
  assertEqual(dip.classLevel, 1, 'a brand-new class dip starts at class level 1');
  assertEqual(dip.classLabel, 'Rogue', 'a brand-new class dip resolves its label from CLASS_OPTIONS');
  assertEqual(dip.characterLevel, 3, 'a new class dip still advances the total character level by one');
}

/**
 * `handleLevelUpAccept` needs to know which level-ups must collect a real
 * feat pick before persisting. Two sources now, OR'd: the universal
 * odd-character-level feat computed here, and the grant names the engine
 * reports for this exact transition (`preview_level_up`).
 */
function verifiesLevelGrantsFeatDetectsEveryRealFeatSource() {
  assert(
    levelGrantsFeat(previewLevelUp([], 'class:fighter').features),
    'character level 1 is odd, so the universal feat fires with no engine input at all'
  );
  assert(
    levelGrantsFeat(
      previewLevelUp([{ classId: 'class:wizard', classLabel: 'Wizard', level: 4 }], 'class:wizard').features
    ),
    'character level 5 is odd, so the universal feat fires'
  );
  assert(
    !levelGrantsFeat(
      previewLevelUp([{ classId: 'class:rogue', classLabel: 'Rogue', level: 1 }], 'class:rogue').features
    ),
    'character level 2 is even and the engine reported nothing — no feat pick is demanded'
  );
  assert(
    levelGrantsFeat(
      previewLevelUp([{ classId: 'class:rogue', classLabel: 'Rogue', level: 1 }], 'class:rogue').features,
      ['Rogue bonus feat at level 2']
    ),
    'an engine-reported grant naming a feat demands a pick even at an even character level'
  );
  assert(
    !levelGrantsFeat([], ['Rogue class features at level 2']),
    '"features" contains "feat" as a substring and must not be read as a feat grant'
  );
}

/**
 * Pins the one behaviour the deleted hand-authored table used to assert and
 * nothing now does.
 *
 * That table claimed Fighter's bonus combat feat at every even class level.
 * The engine cannot confirm it: `level_up/fighter.rs`'s own module doc
 * records that `pick_from_lists` stays empty for those ten slots, and
 * `class_feature.fighter.level_N_bonus_feat` only fires once
 * `choice:fighter_bonus_feat_N` is already selected — after the level-up,
 * not before. So a Fighter taking an even class level is no longer prompted
 * for a bonus feat.
 *
 * This test exists so that gap is a recorded, deliberate state rather than
 * a silent regression, and so it fails loudly the day the engine closes it
 * (at which point the engine-grant branch above starts covering this case
 * for free and this test should be deleted).
 */
function verifiesFighterEvenLevelBonusFeatIsNoLongerClaimedLocally() {
  const fighterLevel2 = previewLevelUp(
    [{ classId: 'class:fighter', classLabel: 'Fighter', level: 1 }],
    'class:fighter'
  );

  assert(
    !levelGrantsFeat(fighterLevel2.features),
    'with no engine grants, Fighter class level 2 no longer claims a bonus combat feat locally'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

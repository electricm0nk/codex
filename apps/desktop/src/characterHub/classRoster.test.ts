/**
 * SD-36 Epic F4c (spec §6, acceptance F4.2): the Create picker and Level Up read the class roster
 * the engine serves (`list_class_creation_roster`, `list_level_up_class_options`), never a list
 * compiled into the frontend. `CLASS_OPTIONS_FALLBACK` (the old hardcoded 31) is used ONLY when the
 * command fails, and then with a visible notice naming the failure.
 *
 * Every figure below is read off the real wire artifacts (`testSupport/classRosterWire.ts`), which a
 * Rust test keeps byte-identical to the live commands.
 */

import {
  classOptionsFromRoster,
  ensureClassRosterLoaded,
  groupClassOptionsByFamily,
  installClassRoster,
  installClassRosterFailure,
  levelUpChoiceGroups,
  levelUpChoiceSuffix,
  canAcceptLevelUp,
  defaultLevelUpChoice,
  describeEntryRequirement,
  fallbackLevelUpResponse,
  type ClassCreationRosterResponse,
  type LevelUpClassOptionsResponse,
} from './classRoster';
import { LOADING_CLASS_CATALOG, findClassOption, getClassCatalog, knownClass, setClassCatalog } from './classCatalog';
import { CLASS_OPTIONS_FALLBACK, MAX_CLASS_LEVEL, getLevelOptionsForClass } from './characterHubModel';
import { classRosterWire, levelUpFighter6Wire } from '../testSupport/classRosterWire';
import { assert, assertEqual } from '../testSupport/asserts';
import { maxHitPoints } from './characterProgression';

const wire = classRosterWire();

/** Every served class becomes exactly one option, in served order, carrying its served figures. */
function verifiesEveryServedClassBecomesAnOptionVerbatim() {
  const options = classOptionsFromRoster(wire);
  assertEqual(options.length, wire.classes.length, 'option count = roster length');
  assertEqual(options.length, 59, 'the census roster offers 59 of its 137 ids at creation (f4b-receipt §3)');
  options.forEach((option, index) => {
    const row = wire.classes[index];
    assertEqual(option.id, row.classId, `option ${index} id`);
    assertEqual(option.label, row.label, `${row.classId} label`);
    assertEqual(option.hitDie, row.hitPointsDie, `${row.classId} hit die: the one the HP fold reads`);
    assertEqual(option.family, row.family, `${row.classId} family`);
    assertEqual(option.familyLabel, row.familyLabel, `${row.classId} family label`);
    assertEqual(option.supportLevel, 'full', `${row.classId}: the census says Computed at every level`);
    assertEqual(option.levelOptions.join(','), Array.from({ length: row.maxLevel }, (_u, i) => i + 1).join(','), `${row.classId} levels 1..max`);
  });
  const fallbackIds = new Set(CLASS_OPTIONS_FALLBACK.map((option) => option.id));
  const newlyOffered = options.filter((option) => !fallbackIds.has(option.id)).map((option) => option.id);
  assertEqual(newlyOffered.length, 28, 'roster classes the hardcoded 31 never offered (59 - 31)');
  for (const id of ['class:samurai', 'class:magus', 'class:warrior', 'class:kineticist', 'class:gunslinger']) {
    assert(newlyOffered.includes(id), `${id} is offered now`);
  }
  assert(!options.some((option) => option.id.startsWith('class:ex_')), 'Ex-* states are census-only (§9)');
  assert(!options.some((option) => option.id === 'class:arcane_archer'), 'prestige is offered only at level-up (§9)');
}

/** The picker groups by family: contiguous runs, in served order, covering every option once. */
function verifiesTheRosterGroupsByFamily() {
  const groups = groupClassOptionsByFamily(classOptionsFromRoster(wire));
  assertEqual(groups.map((group) => group.familyLabel).join(' | '),
    'CRB | APG | ACG | Pathfinder Unchained | Ultimate Combat | Untabled exotic base classes | CRB NPC / Ex-* classes',
    'family order as served');
  assertEqual(groups.reduce((sum, group) => sum + group.options.length, 0), 59, 'every option in exactly one group');
  assertEqual(groups.map((group) => group.options.length).join(','), '11,6,10,4,3,20,5', 'per-family counts');
}

/** Before the roster arrives nothing is offered and the fallback is NOT silently consulted. */
function verifiesTheLoadingStateOffersNothingAndConsultsNoFallback() {
  setClassCatalog(LOADING_CLASS_CATALOG);
  assertEqual(getClassCatalog().source, 'loading', 'initial source');
  assertEqual(getClassCatalog().options.length, 0, 'no option while loading');
  assertEqual(findClassOption('class:fighter'), undefined, 'the fallback is not consulted while loading');
  assertEqual(getLevelOptionsForClass('class:fighter').join(','), '1', 'no class claims levels while loading');
}

/** A loaded roster is what every lookup reads, including withheld classes' own figures. */
function verifiesAnInstalledRosterIsWhatLookupsRead() {
  installClassRoster(wire);
  const catalog = getClassCatalog();
  assertEqual(catalog.source, 'roster', 'source');
  assertEqual(catalog.notice, null, 'a healthy roster carries no notice');
  assertEqual(catalog.options.length, 59, 'options');
  assertEqual(findClassOption('class:samurai')?.label, 'Samurai', 'a newly offered class resolves');
  assertEqual(getLevelOptionsForClass('class:samurai').length, MAX_CLASS_LEVEL, 'Samurai 1-20');
  // SD-36 F6b: the served die is the engine's one hit-die rule (the bespoke class module that
  // computes the class's hit points first, then the converted record). The CRB Monk is the CRB
  // table's d8 (CRB p.56) — never its converted record's FS-23 d10 — and the four Unchained classes
  // read their own module's die. No offered class is HP Unknown.
  assertEqual(findClassOption('class:monk')?.hitDie, 8, 'Monk d8 (CRB p.56), from the source that computes its HP');
  assertEqual(findClassOption('class:unchained_monk')?.hitDie, 10, 'Unchained Monk d10 (Pathfinder Unchained)');
  assertEqual(findClassOption('class:unchained_rogue')?.hitDie, 8, 'Unchained Rogue d8 (Pathfinder Unchained)');
  const noDie = wire.classes.filter((row) => row.hitPointsDie === null).map((row) => row.classId);
  assertEqual(noDie.join(','), '', '0 of 59 roster classes: HP Unknown');
  // Monk 5 on Con +2: 8+2 + 4 x (5+2) = 38 (level 1 the full die, then the average die/2 + 1).
  assertEqual(maxHitPoints([{ classId: 'class:monk', classLabel: 'Monk', level: 5 }], 2), 38, 'Monk 5 HP');
  assertEqual(maxHitPoints([{ classId: 'class:unchained_rogue', classLabel: 'Unchained Rogue', level: 5 }], 2), 38, 'Unchained Rogue 5 HP');
  const archer = knownClass('class:arcane_archer');
  assert(archer !== undefined, 'a withheld prestige class is still known (label, hit die) for a character holding it');
  assertEqual(archer!.label, 'Arcane Archer', 'prestige label');
  assertEqual(archer!.hitDie, 10, 'Arcane Archer d10, off the withheld row');
  assertEqual(findClassOption('class:arcane_archer'), undefined, 'but it is not a creation option');
  assertEqual(knownClass('class:rogue')?.skillRanksPerLevel, 8, 'Rogue 8 skill ranks per level, off the wire');
  assertEqual(knownClass('class:arcanist')?.skillRanksPerLevel, 2, 'Arcanist 2 (the removed frontend table said 3)');
  assertEqual(catalog.known.size, wire.classes.length + wire.withheld.length, 'every census id is known: 59 offered + 78 withheld = 137');
}

/** The command failing is the ONLY route to the fallback, and it is announced. */
function verifiesAFailedCommandInstallsTheFallbackWithAVisibleNotice() {
  installClassRosterFailure('census sweep fixture not found: /nowhere/fixture.json');
  const catalog = getClassCatalog();
  assertEqual(catalog.source, 'fallback', 'source');
  assertEqual(catalog.notice, 'class roster unavailable: census sweep fixture not found: /nowhere/fixture.json', 'visible notice text');
  assertEqual(catalog.options.length, CLASS_OPTIONS_FALLBACK.length, 'fallback options');
  assertEqual(CLASS_OPTIONS_FALLBACK.length, 31, 'the fallback is the old hardcoded 31');
  assertEqual(knownClass('class:fighter')?.skillRanksPerLevel, null, 'the fallback states no skill ranks: the sheet names the gap');
}

/**
 * The fallback may not drift from what the engine serves: every fallback id is served, with the
 * same label and the same hit die (31 of 31 since SD-36 F6b: the engine states a die for every
 * offered class).
 */
function verifiesTheFallbackAgreesWithTheRosterOnEveryRowItCarries() {
  const served = new Map(wire.classes.map((row) => [row.classId, row]));
  let compared = 0;
  for (const option of CLASS_OPTIONS_FALLBACK) {
    const row = served.get(option.id);
    assert(row !== undefined, `${option.id}: in the fallback but not in the roster`);
    assertEqual(option.label, row!.label, `${option.id} label`);
    if (row!.hitPointsDie !== null) {
      assertEqual(option.hitDie, row!.hitPointsDie, `${option.id} hit die`);
      compared += 1;
    }
  }
  assertEqual(compared, 31, 'hit dice compared');
}

/** `ensureClassRosterLoaded` installs whichever outcome the command produced. */
async function verifiesEnsureInstallsTheCommandOutcome() {
  setClassCatalog(LOADING_CLASS_CATALOG);
  await ensureClassRosterLoaded(async () => wire, { reload: true });
  assertEqual(getClassCatalog().source, 'roster', 'resolved command installs the roster');

  await ensureClassRosterLoaded(async () => {
    throw new Error('Failed to load the class roster: boom');
  }, { reload: true });
  assertEqual(getClassCatalog().source, 'fallback', 'rejected command installs the fallback');
  assertEqual(getClassCatalog().notice, 'class roster unavailable: Failed to load the class roster: boom', 'notice');

  const empty: ClassCreationRosterResponse = { classes: [], withheld: [], diagnostics: [] };
  await ensureClassRosterLoaded(async () => empty, { reload: true });
  assertEqual(getClassCatalog().source, 'fallback', 'an empty roster is a failure, never an empty picker');
  assert(getClassCatalog().notice!.startsWith('class roster unavailable: '), 'and it is announced');
}

/** Level Up: advance / add a base class / add a prestige class, requirements printed, never blocking. */
function verifiesLevelUpChoiceGroupsForAFighter6() {
  const response = levelUpFighter6Wire();
  const groups = levelUpChoiceGroups(response);
  assertEqual(groups.map((group) => group.kind).join(','), 'advance,add_base,add_prestige', 'three groups in order');
  const [advance, addBase, addPrestige] = groups;
  assertEqual(advance.choices.length, 1, 'advance: the held Fighter');
  assertEqual(advance.choices[0].classId, 'class:fighter', 'advance id');
  assertEqual(advance.choices[0].nextLevel, 7, 'Fighter 6 -> 7');
  assertEqual(addBase.choices.length, 58, 'add a base class: 59 roster classes minus the held Fighter');
  assertEqual(addPrestige.choices.length, 74, 'add a prestige class: 74 of 74 census prestige classes');

  const archer = addPrestige.choices.find((choice) => choice.classId === 'class:arcane_archer');
  assert(archer !== undefined, 'Arcane Archer is offered');
  assertEqual(archer!.selectable, true, 'offered although requirements are unmet (§9.2: print, never block)');
  assertEqual(archer!.requirementsAllMet, false, 'requirements not all met');
  assertEqual(archer!.requirements.length, 4, 'four printed requirement lines (f4b-receipt §3)');
  assertEqual(archer!.requirements.map((line) => line.status).join(','), 'unmet,unmet,unmet,met', 'met/unmet notes');
  assertEqual(archer!.requirements[3].text, 'base attack bonus at least 6', 'BAB line in the rule’s words');
  // SD-36 F6c: the six FS-15 prestige classes are shown with the engine's named refusal and are
  // not selectable; every other option is.
  const blocked = addPrestige.choices.filter((choice) => !choice.selectable);
  assertEqual(
    blocked.map((choice) => choice.classId).sort().join(','),
    'class:evangelist,class:exalted,class:mammoth_rider,class:pure_legion_enforcer,class:sentinel,class:ulfen_guard',
    'the prestige options the engine refuses are the six FS-15 classes'
  );
  for (const choice of blocked) {
    assertEqual(choice.blocker?.id ?? null, 'multiclass.save_shape.unrecognized', `${choice.classId} blocker id`);
    assertEqual(
      levelUpChoiceSuffix(choice),
      ' (cannot be taken — save progression in the source data matches no PF1 form)',
      `${choice.classId} prints its blocker on the option`
    );
    assert(!canAcceptLevelUp(groups.flatMap((group) => group.choices), choice.classId), `${choice.classId}: Accept is refused before the engine`);
  }
  assertEqual(addPrestige.choices.length - blocked.length, 68, 'the other 68 of 74 prestige options are selectable');
  assert(advance.choices.every((choice) => choice.selectable), 'the held Fighter can advance');
  assertEqual(
    addBase.choices.filter((choice) => !choice.selectable).map((choice) => `${choice.classId}=${choice.blocker?.id}`).join(','),
    'class:unchained_barbarian=multiclass.save_shape.unknown,class:unchained_monk=multiclass.save_shape.unknown,' +
      'class:unchained_rogue=multiclass.save_shape.unknown,class:unchained_summoner=multiclass.save_shape.unknown',
    'the base classes a Fighter 6 cannot add: the four Unchained classes, whose saves no table or record states'
  );
  assert(canAcceptLevelUp(groups.flatMap((group) => group.choices), 'class:arcane_archer'), 'Arcane Archer can be accepted');
  assertEqual(
    levelUpChoiceSuffix(archer!),
    ' (prestige — requirements not all met)',
    'an unmet requirement is a note, not a refusal'
  );
  assert(
    addPrestige.choices.every((choice) => choice.requirements.every((line) => line.text.length > 0)),
    'no empty requirement line'
  );
}

/** SD-36 F6c: Level Up never selects an option the engine refuses, even when it is asked to. */
function verifiesTheDefaultChoiceIsNeverARefusedOption() {
  const response = levelUpFighter6Wire();
  const choices = levelUpChoiceGroups(response).flatMap((group) => group.choices);
  assertEqual(defaultLevelUpChoice(choices, ''), 'class:fighter', 'first selectable: the held Fighter');
  assertEqual(defaultLevelUpChoice(choices, 'class:ulfen_guard'), 'class:fighter', 'a refused option is not kept as the selection');
  assertEqual(defaultLevelUpChoice(choices, 'class:arcane_archer'), 'class:arcane_archer', 'a selectable option is kept');
  const onlyRefused = choices.filter((choice) => !choice.selectable);
  assertEqual(defaultLevelUpChoice(onlyRefused, ''), '', 'nothing is selected when every option is refused');
  assert(!canAcceptLevelUp(onlyRefused, ''), 'Accept is disabled with nothing selectable');
}

function verifiesTheLevelCapEmptiesEveryGroupWithTheReason() {
  const atCap: LevelUpClassOptionsResponse = {
    characterLevel: 20,
    levelCap: 20,
    atLevelCap: true,
    advance: [],
    addBase: [],
    addPrestige: [],
    diagnostics: ['character level 20 is the PF1 cap of 20: no further level can be taken'],
  };
  const groups = levelUpChoiceGroups(atCap);
  assert(groups.every((group) => group.choices.length === 0), 'nothing offered at the cap');
}

function verifiesRequirementNotesPrintTheirStatus() {
  assertEqual(describeEntryRequirement({ text: 'base attack bonus at least 6', status: 'met', condition: null }), 'base attack bonus at least 6 — met', 'met');
  assertEqual(describeEntryRequirement({ text: 'highest arcane spell level at least 1', status: 'unmet', condition: null }), 'highest arcane spell level at least 1 — unmet', 'unmet');
  assertEqual(
    describeEntryRequirement({ text: 'must be of elven descent', status: 'situational', condition: 'GM adjudicates descent' }),
    'must be of elven descent — situational: GM adjudicates descent',
    'situational prints its condition'
  );
}

/** The command failing still offers advance + add-base from the catalog, announced, capped at 20. */
function verifiesTheLevelUpFallbackIsAnnouncedAndCapped() {
  installClassRoster(wire);
  const options = getClassCatalog().options;
  const fighter6 = fallbackLevelUpResponse([{ classId: 'class:fighter', classLabel: 'Fighter', level: 6 }], options, 'boom');
  assertEqual(fighter6.diagnostics[0], 'level-up class options unavailable: boom', 'the failure is printed');
  assertEqual(fighter6.advance.map((option) => `${option.classId}:${option.nextLevel}`).join(','), 'class:fighter:7', 'advance');
  assertEqual(fighter6.addBase.length, 58, 'add a base class: catalog minus the held Fighter');
  assertEqual(fighter6.addPrestige.length, 0, 'no prestige without the command that judges its requirements');
  const capped = fallbackLevelUpResponse([{ classId: 'class:fighter', classLabel: 'Fighter', level: 20 }], options, 'boom');
  assert(capped.atLevelCap && capped.advance.length === 0 && capped.addBase.length === 0, 'the cap holds');
}

async function main() {
  verifiesEveryServedClassBecomesAnOptionVerbatim();
  verifiesTheRosterGroupsByFamily();
  verifiesTheLoadingStateOffersNothingAndConsultsNoFallback();
  verifiesAnInstalledRosterIsWhatLookupsRead();
  verifiesAFailedCommandInstallsTheFallbackWithAVisibleNotice();
  verifiesTheFallbackAgreesWithTheRosterOnEveryRowItCarries();
  await verifiesEnsureInstallsTheCommandOutcome();
  verifiesLevelUpChoiceGroupsForAFighter6();
  verifiesTheDefaultChoiceIsNeverARefusedOption();
  verifiesTheLevelCapEmptiesEveryGroupWithTheReason();
  verifiesRequirementNotesPrintTheirStatus();
  verifiesTheLevelUpFallbackIsAnnouncedAndCapped();
  console.log('classRoster: all assertions passed');
}

main().catch((error: unknown) => {
  console.error(error);
  process.exit(1);
});

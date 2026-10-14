import {
  CYBERPUNK_CREATION_METHODS,
  EMPTY_CYBERPUNK_DRAFT,
  adjustStat,
  blockingProblems,
  buildCyberpunkCreateRequest,
  buildCyberpunkPreviewRequest,
  cyberpunkDerivedTiles,
  cyberpunkStatTiles,
  rollAgain,
  selectMethod,
  selectRole,
  setCulturalLanguage,
  setLifepathCount,
  setLifepathPick,
  setLifepathSkip,
  templateRowOptions,
  withPreviewSeed,
  culturalLanguageOptions,
  culturalLanguageTile,
  lifepathRowOptions,
  lifepathTablesFor,
  addSkill,
  adjustSkill,
  dropSkill,
  setSkillSpecialty,
  skillKey,
  skillOptions,
  skillPointTiles,
  type CyberpunkCreationDraft,
} from './cyberpunkCreationModel';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  CYBERPUNK_CREATE_COLUMNS,
  CYBERPUNK_CREATE_LAYOUT,
  type CyberpunkCreateBoxId,
} from './cyberpunkCreateLayout';
import { boxesInColumn } from './sheetLayout';
import { characterCreationGate } from './characterHubRuntime';
import type { CyberpunkCreationPreview, CyberpunkSkillLine, CyberpunkStatValue } from '../boundary/cprCreation';
import type { CyberpunkRecord } from '../boundary/loadCyberpunkCatalog';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * SD-38 E3.2: New Character under Cyberpunk Red opens `CyberpunkCreateForm` (method picker +
 * STAT step) and saves through `create_cyberpunk_character` (`cpr_creation.rs`, which saves
 * with the CPR store and reads back through the envelope router to `CyberpunkAdapter`). Every
 * number the form shows is the engine's answer (`preview_cyberpunk_character`): the STATs, the
 * Complete Package points spent and total, and the derived `cpr.derived.*` rows. These tests pin
 * the pure pieces the form composes: the gate, the requests, the seed it keeps so create draws
 * the previewed dice, the method switch, the allocation buttons, the template-row options (from
 * the corpus records, not a desktop table) and the tiles (only rows the engine returned).
 */

const STAT_IDS = ['int', 'ref', 'dex', 'tech', 'cool', 'will', 'luck', 'move', 'body', 'emp'];

function stats(values: number[]): CyberpunkStatValue[] {
  return STAT_IDS.map((id, i) => ({ id, label: id.toUpperCase(), value: values[i] }));
}

function preview(partial: Partial<CyberpunkCreationPreview>): CyberpunkCreationPreview {
  return {
    role: 'Solo',
    method: 'streetrat',
    seed: 7,
    rolls: [],
    templateRoll: 1,
    stats: stats([6, 7, 7, 3, 8, 6, 5, 5, 6, 5]),
    statPointsSpent: null,
    statPointsTotal: null,
    lifepath: [],
    culturalLanguage: null,
    skills: { lines: [], rows: [] },
    explanations: [],
    problems: [],
    ...partial,
  };
}

function testNewCharacterUnderCyberpunkRedOpensTheCyberpunkForm() {
  const gate = characterCreationGate('cyberpunk-red');
  assertEqual(gate.enabled, true, 'New Character opens for Cyberpunk Red');
  assertEqual(gate.form, 'cyberpunk', 'Cyberpunk Red opens CyberpunkCreateForm, never the Pathfinder or Starfinder form');
  assertEqual(gate.disabledHint, undefined, 'an open gate carries no disabled reason');
  assertEqual(characterCreationGate('starfinder-1e').form, 'starfinder', 'Starfinder keeps its own form');
  assertEqual(characterCreationGate('pathfinder-1e').form, 'pathfinder', 'Pathfinder keeps its own form');
  assertEqual(characterCreationGate('traveller').enabled, false, 'a rule set with no adapter stays closed');
}

function testTheMethodPickerOffersTheThreeBookMethods() {
  assertEqual(
    CYBERPUNK_CREATION_METHODS.map((m) => m.id).join(','),
    'streetrat,edgerunner,completePackage',
    'the three methods, in the book order (p. 73)'
  );
  assertEqual(
    CYBERPUNK_CREATION_METHODS.map((m) => m.label).join('|'),
    'Streetrat|Edgerunner|Complete Package',
    'each method by its book name'
  );
}

function testNoRoleNoPreview() {
  assertEqual(buildCyberpunkPreviewRequest(EMPTY_CYBERPUNK_DRAFT), null, 'without a Role there is nothing to preview');
}

function testThePreviewRequestCarriesOnlyTheMethodsOwnFields() {
  const streetrat: CyberpunkCreationDraft = { ...EMPTY_CYBERPUNK_DRAFT, role: 'Solo', method: 'streetrat', templateRoll: 3, seed: 9, stats: { int: 5 } };
  const request = buildCyberpunkPreviewRequest(streetrat);
  assertEqual(request?.role, 'Solo', 'the Role');
  assertEqual(request?.method, 'streetrat', 'the method');
  assertEqual(request?.templateRoll, 3, 'Streetrat sends the picked row');
  assertEqual(request?.seed, 9, 'the kept seed');
  assertEqual(request?.stats, null, 'Streetrat sends no allocation');

  const edgerunner = buildCyberpunkPreviewRequest({ ...streetrat, method: 'edgerunner' });
  assertEqual(edgerunner?.templateRoll, null, 'Edgerunner sends no template row');
  assertEqual(edgerunner?.stats, null, 'Edgerunner sends no allocation');

  const cp = buildCyberpunkPreviewRequest({ ...streetrat, method: 'completePackage', stats: { int: 5, ref: 7 } });
  assertEqual(cp?.templateRoll, null, 'Complete Package sends no template row');
  assertEqual(JSON.stringify(cp?.stats), '{"int":5,"ref":7}', 'Complete Package sends the allocation');
}

function testTheCreateRequestCarriesAFreshIdentityAndTheSameStatStep() {
  const draft: CyberpunkCreationDraft = { ...EMPTY_CYBERPUNK_DRAFT, displayLabel: '  V  ', role: 'Solo', method: 'edgerunner', seed: 2026 };
  const request = buildCyberpunkCreateRequest(draft, { generateId: () => 'id-1', now: () => '2026-10-14T00:00:00Z' });
  assertEqual(request?.characterId, 'id-1', 'a fresh id');
  assertEqual(request?.displayLabel, 'V', 'the name, trimmed');
  assertEqual(request?.savedAt, '2026-10-14T00:00:00Z', 'the save time');
  assertEqual(
    JSON.stringify(request?.creation),
    JSON.stringify(buildCyberpunkPreviewRequest(draft)),
    'create sends exactly the previewed STAT step (same seed, so the same dice)'
  );
  assertEqual(buildCyberpunkCreateRequest(EMPTY_CYBERPUNK_DRAFT, { generateId: () => 'x', now: () => 'y' }), null, 'no Role, no request');
}

function testThePreviewsSeedIsKeptSoCreateDrawsThePreviewedDice() {
  const draft: CyberpunkCreationDraft = { ...EMPTY_CYBERPUNK_DRAFT, role: 'Solo', method: 'edgerunner', seed: null };
  const kept = withPreviewSeed(draft, preview({ method: 'edgerunner', seed: 4242 }));
  assertEqual(kept.seed, 4242, 'the seed the engine drew is kept');
  assertEqual(withPreviewSeed(kept, preview({ method: 'edgerunner', seed: 4242 })), kept, 'idempotent: the same seed keeps the same draft object (no preview loop)');
  assertEqual(rollAgain(kept).seed, null, 'Roll again drops the seed so the engine draws fresh dice');
  assertEqual(rollAgain(kept).role, 'Solo', 'Roll again keeps the other choices');
}

function testSwitchingMethodResetsTheDiceAndSeedsTheAllocationFromTheShownStats() {
  const draft: CyberpunkCreationDraft = { ...EMPTY_CYBERPUNK_DRAFT, role: 'Solo', method: 'streetrat', seed: 5, templateRoll: 2 };
  const shown = stats([6, 7, 7, 3, 8, 6, 5, 5, 6, 5]);
  const cp = selectMethod(draft, 'completePackage', shown);
  assertEqual(cp.method, 'completePackage', 'the method changes');
  assertEqual(cp.seed, null, 'a new method draws its own dice');
  assertEqual(cp.templateRoll, null, 'the template row belongs to Streetrat');
  assertEqual(cp.stats.cool, 8, 'the allocation starts from the STATs on screen');
  assertEqual(Object.keys(cp.stats).length, 10, 'one value per STAT the engine listed');
  const back = selectMethod(cp, 'edgerunner', shown);
  assertEqual(back.method, 'edgerunner', 'switching again works');
  assertEqual(back.stats.cool, 8, 'the allocation is kept for a later return to Complete Package');
  assertEqual(selectMethod(draft, 'streetrat', shown), draft, 'choosing the current method changes nothing');
}

function testTheAllocationButtonsMoveOneStatAndNeverBelowZero() {
  const next = adjustStat({ int: 2, ref: 8 }, 'ref', 1);
  assertEqual(next.ref, 9, 'the engine, not the button, refuses a 9 (shown as its problem)');
  assertEqual(next.int, 2, 'other STATs untouched');
  assertEqual(adjustStat({ int: 0 }, 'int', -1).int, 0, 'never below zero');
  assertEqual(adjustStat({}, 'emp', 1).emp, 1, 'an unset STAT starts from zero');
}

function testTemplateRowOptionsComeFromTheRolesCorpusRecords() {
  const records: CyberpunkRecord[] = [
    { kind: 'streetrat_template', name: 'Solo template roll 2', role: 'Solo', roll: 2, text: 'Roll 2: 7 8 6 3 6 6 7 5 6 6' },
    { kind: 'streetrat_template', name: 'Tech template roll 1', role: 'Tech', roll: 1, text: 'Roll 1: x' },
    { kind: 'streetrat_template', name: 'Solo template roll 1', role: 'Solo', roll: 1, text: 'Roll 1: 6 7 7 3 8 6 5 5 6 5' },
  ];
  const options = templateRowOptions(records, 'Solo');
  assertEqual(options.map((o) => o.roll).join(','), '1,2', "only the Role's rows, by roll");
  assertEqual(options[0].label, 'Roll 1: 6 7 7 3 8 6 5 5 6 5', "each row labelled by the book's own line");
  assertEqual(templateRowOptions(records, null).length, 0, 'no Role, no rows');
}

function testOnlyClaimBlockingProblemsCloseTheButton() {
  const p = preview({
    problems: [
      { id: 'cpr_creation.stat_points', message: 'spend 62', claimBlocking: true },
      { id: 'note', message: 'fyi', claimBlocking: false },
    ],
  });
  assertEqual(blockingProblems(p).map((x) => x.id).join(','), 'cpr_creation.stat_points', 'one blocking problem');
}

function testTilesComeFromTheEnginesRowsOnly() {
  const rows = [
    { id: 'cpr.stat.int', value: 6, detail: '' },
    { id: 'cpr.stat.body', value: 6, detail: '' },
    { id: 'cpr.derived.hit_points', value: 40, detail: '' },
    { id: 'cpr.derived.seriously_wounded', value: 20, detail: '' },
    { id: 'cpr.derived.humanity_max', value: 50, detail: '' },
    { id: 'pf.hit_points', value: 99, detail: '' },
  ];
  const derived = cyberpunkDerivedTiles(rows);
  assertEqual(
    derived.map((t) => `${t.label}=${t.value}`).join('|'),
    'Hit Points=40|Seriously Wounded=20|Humanity=50',
    'one tile per cpr.derived row the engine returned, in sheet order; none fabricated'
  );
  const statTiles = cyberpunkStatTiles(rows, stats([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
  assertEqual(
    statTiles.map((t) => `${t.label}=${t.value}`).join('|'),
    'INT=6|BODY=6',
    'one tile per cpr.stat row, labelled by the engine, valued by the row'
  );
}

function testTheCreateLayoutIsARegistry() {
  const ids: CyberpunkCreateBoxId[] = CYBERPUNK_CREATE_COLUMNS.flatMap((column) =>
    boxesInColumn(CYBERPUNK_CREATE_LAYOUT, column).map((box) => box.id)
  );
  assertEqual(ids.join(','), 'identity,method,stats,derived', 'every box, once, column by column in display order');
  assertEqual(new Set(CYBERPUNK_CREATE_LAYOUT.map((b) => b.id)).size, CYBERPUNK_CREATE_LAYOUT.length, 'no box registered twice');
  assert(
    CYBERPUNK_CREATE_LAYOUT.every((box) => CYBERPUNK_CREATE_COLUMNS.includes(box.column)),
    'every box lives in a registered column'
  );
}

// ---- SD-38 E3.3: the Lifepath panel ----

const CORPUS = join(dirname(fileURLToPath(import.meta.url)), '../../../../data/cyberpunk-red/corpus');

function corpusRecords(kind: string): CyberpunkRecord[] {
  return JSON.parse(readFileSync(join(CORPUS, `${kind}.json`), 'utf8')) as CyberpunkRecord[];
}

function testTheLifepathOffersEveryGeneralTableAndExactlyTheRolesTables() {
  const records = corpusRecords('lifepath_table');
  const roles = corpusRecords('role').map((r) => String(r.name));
  assertEqual(roles.length, 10, 'ten Roles');
  const general = records.filter((r) => r.group === 'general').map((r) => String(r.name));
  assertEqual(general.length, 15, 'CUI F-26: 15 general tables');
  let roleTables = 0;
  for (const role of roles) {
    const offered = lifepathTablesFor(records, role).map((r) => String(r.name));
    const expected = records.filter((r) => r.group === 'general' || r.role === role).map((r) => String(r.name));
    assertEqual(offered.join('|'), expected.join('|'), `${role}: every table offered = the lifepath_table records for that Role, corpus order`);
    assertEqual(offered.slice(0, 15).join('|'), general.join('|'), `${role}: the general tables first`);
    assert(offered.slice(15).every((name) => records.find((r) => r.name === name)?.role === role), `${role}: no other Role's table`);
    roleTables += offered.length - 15;
  }
  assertEqual(roleTables, 52, 'CUI F-27: 52 Role tables over the ten Roles');
  assertEqual(lifepathTablesFor(records, null).length, 0, 'no Role, no Lifepath yet');
}

function testLifepathRowOptionsAreTheRecordsRows() {
  const records = corpusRecords('lifepath_table');
  const revenge = records.find((r) => r.name === 'Sweet Revenge!') as CyberpunkRecord;
  const options = lifepathRowOptions(revenge, 0);
  assertEqual(options.map((o) => o.roll).join(','), '1,3,5,7,9,10', 'a "1-2" row is picked by its first roll');
  assertEqual(options[0].label, '1-2: Avoid the scum.', "the book's roll and cell");
  const dress = records.find((r) => r.name === 'Dress and Personal Style: Clothing Style and Hairstyle') as CyberpunkRecord;
  const hair = lifepathRowOptions(dress, 1);
  assertEqual(hair.length, 10, 'one option per row');
  assert(hair[0].label.startsWith('1: ') && !hair[0].label.includes(' / '), 'a per-column table offers that column only');
  const origins = records.find((r) => r.name === 'Cultural Origins') as CyberpunkRecord;
  assertEqual(lifepathRowOptions(origins, 0)[8].label.split(' / ')[0], '9: East Asian', 'a whole-row table offers the whole row');
  assertEqual(culturalLanguageOptions(origins, 9).join(','), 'Cantonese Chinese,English,Japanese,Korean,Mandarin Chinese,Mongolian', "the rolled region's languages");
  assertEqual(culturalLanguageOptions(origins, null).length, 0, 'no region, no list');
}

function testLifepathPicksCountsSkipsAndLanguageAreTheDraftsOwnChoices() {
  let draft: CyberpunkCreationDraft = { ...EMPTY_CYBERPUNK_DRAFT, role: 'Nomad', seed: 3 };
  draft = setLifepathPick(draft, 'Your Personality', 0, 0, 3);
  draft = setLifepathPick(draft, 'Your Personality', 0, 0, 4);
  assertEqual(JSON.stringify(draft.lifepath.picks), '[{"table":"Your Personality","entry":0,"column":0,"roll":4}]', 'a pick replaces the earlier pick of that answer');
  assertEqual(draft.seed, 3, 'a pick keeps the seed: the other tables keep their dice');
  draft = setLifepathPick(draft, 'Your Enemies', 2, 1, 7);
  draft = setLifepathCount(draft, 'Your Enemies', 1);
  assertEqual(draft.lifepath.counts['Your Enemies'], 1, 'the picked count');
  assert(!draft.lifepath.picks.some((p) => p.table === 'Your Enemies' && p.entry === 2), 'a smaller count drops the picks of the entries it removes');
  draft = setLifepathCount(draft, 'Your Enemies', null);
  assertEqual(draft.lifepath.counts['Your Enemies'], undefined, 'null rolls the count again');
  draft = setLifepathPick(draft, 'Your Personality', 0, 0, null);
  assertEqual(draft.lifepath.picks.length, 0, 'null rolls the answer again');
  draft = setLifepathSkip(draft, 'Nomad: If in Air, What Do They Do?', true);
  draft = setLifepathSkip(draft, 'Nomad: If in Air, What Do They Do?', true);
  assertEqual(draft.lifepath.skips.join('|'), 'Nomad: If in Air, What Do They Do?', 'marked not taken once');
  assertEqual(setLifepathSkip(draft, 'Nomad: If in Air, What Do They Do?', false).lifepath.skips.length, 0, 'taken again');
  draft = setCulturalLanguage(draft, ' Japanese ');
  assertEqual(draft.lifepath.culturalLanguage, 'Japanese', 'the language, trimmed');
  assertEqual(setCulturalLanguage(draft, '').lifepath.culturalLanguage, null, 'empty rolls it from the region');

  const request = buildCyberpunkPreviewRequest(draft);
  assertEqual(JSON.stringify(request?.lifepath), JSON.stringify(draft.lifepath), 'the preview sends the Lifepath choices');
  const create = buildCyberpunkCreateRequest({ ...draft, displayLabel: 'V' }, { generateId: () => 'id', now: () => 'now' });
  assertEqual(JSON.stringify(create?.creation.lifepath), JSON.stringify(draft.lifepath), 'create saves the previewed answers');
}

function testChangingRoleKeepsOnlyTheGeneralLifepathChoices() {
  const records = corpusRecords('lifepath_table');
  let draft: CyberpunkCreationDraft = { ...EMPTY_CYBERPUNK_DRAFT, role: 'Nomad', seed: 3, templateRoll: 2 };
  draft = setLifepathPick(draft, 'Your Personality', 0, 0, 4);
  draft = setLifepathPick(draft, 'Nomad: How Big is Your Pack?', 0, 0, 1);
  draft = setLifepathSkip(draft, 'Nomad: If at Sea, What Do They Do?', true);
  draft = setLifepathCount(draft, 'Your Friends', 2);
  const solo = selectRole(draft, 'Solo', records);
  assertEqual(solo.role, 'Solo', 'the Role changes');
  assertEqual(solo.seed, null, 'fresh dice');
  assertEqual(solo.templateRoll, null, "the template row was the old Role's");
  assertEqual(solo.lifepath.picks.map((p) => p.table).join('|'), 'Your Personality', "the Nomad table's pick is dropped");
  assertEqual(solo.lifepath.skips.length, 0, "the Nomad branch skip is dropped");
  assertEqual(solo.lifepath.counts['Your Friends'], 2, 'general counts are kept');
}

function skillLine(partial: Partial<CyberpunkSkillLine>): CyberpunkSkillLine {
  return { line: '', key: '', level: 2, basic: false, x2: false, cultural: false, adjustable: true, chooseOne: false, levelRow: '', ...partial };
}

function testTheCulturalLanguageTileIsTheEnginesRow() {
  const japanese = skillLine({ line: 'Language (Japanese)', key: 'Language (Japanese)', level: 4, cultural: true, levelRow: 'cpr.skill_level.language.japanese' });
  const streetslang = skillLine({ line: 'Language (Streetslang)', key: 'Language (Streetslang)', basic: true, levelRow: 'cpr.skill_level.language.streetslang' });
  const p = preview({
    culturalLanguage: { name: 'Japanese', picked: true, skill: 'Language (Japanese)', level: 4 },
    skills: { lines: [streetslang, japanese], rows: [] },
    explanations: [
      { id: 'cpr.skill_level.language.streetslang', value: 2, detail: '' },
      { id: 'cpr.skill_level.language.japanese', value: 4, detail: '' },
      { id: 'cpr.skill.language.japanese', value: 10, detail: '' },
    ],
  });
  assertEqual(JSON.stringify(culturalLanguageTile(p)), '{"id":"cpr.skill_level.language.japanese","label":"Language (Japanese)","value":4}', "the level is the engine's row for the Cultural Origin language, not Streetslang's");
  assertEqual(
    culturalLanguageTile(preview({ culturalLanguage: { name: 'Japanese', picked: true, skill: 'Language (Japanese)', level: 4 }, skills: { lines: [japanese], rows: [] } })),
    null,
    'no engine row, no tile'
  );
  assertEqual(culturalLanguageTile(preview({})), null, 'no language, no tile');
}

// ---- SD-38 E3.4: the skills allocator ----

function testTheSkillsRequestCarriesTheDraftsLevelsAndSpecialties() {
  let draft: CyberpunkCreationDraft = { ...EMPTY_CYBERPUNK_DRAFT, role: 'Rockerboy', method: 'edgerunner', seed: 1 };
  assertEqual(JSON.stringify(buildCyberpunkPreviewRequest(draft)?.skills), '{"levels":{},"specialties":{}}', 'nothing chosen: the method starts where the book does');
  draft = setSkillSpecialty(draft, 'Play Instrument (choose 1)', ' Guitar ');
  draft = adjustSkill(draft, skillLine({ line: 'Composition', key: 'Composition', level: 6 }), -2);
  const request = buildCyberpunkPreviewRequest(draft);
  assertEqual(JSON.stringify(request?.skills), '{"levels":{"Composition":4},"specialties":{"Play Instrument (choose 1)":"Guitar"}}', 'the level and the named instrument');
  assertEqual(draft.seed, 1, 'a skill change keeps the dice');
  const create = buildCyberpunkCreateRequest({ ...draft, displayLabel: 'V' }, { generateId: () => 'id', now: () => 'now' });
  assertEqual(JSON.stringify(create?.creation.skills), JSON.stringify(request?.skills), 'create saves the previewed allocation');
  assertEqual(setSkillSpecialty(draft, 'Play Instrument (choose 1)', '').skills.specialties['Play Instrument (choose 1)'], undefined, 'an empty name clears it');
}

function testAdjustSkillMovesFromTheEnginesLevelAndNeverBelowZero() {
  const draft: CyberpunkCreationDraft = { ...EMPTY_CYBERPUNK_DRAFT, role: 'Solo', method: 'completePackage' };
  const up = adjustSkill(draft, skillLine({ line: 'Handgun', key: 'Handgun', level: 4 }), 1);
  assertEqual(up.skills.levels.Handgun, 5, "one up from the level the engine showed; the cap of 6 is the engine's to refuse");
  assertEqual(adjustSkill(draft, skillLine({ line: 'Handgun', key: 'Handgun', level: 0 }), -1).skills.levels.Handgun, 0, 'never below zero');
}

function testCompletePackageAddsAndDropsAnyMasterListSkill() {
  let draft: CyberpunkCreationDraft = { ...EMPTY_CYBERPUNK_DRAFT, role: 'Solo', method: 'completePackage' };
  assertEqual(skillKey('Martial Arts', ' Karate '), 'Martial Arts (Karate)', 'a skill with its specialty');
  assertEqual(skillKey('Handgun', ''), 'Handgun', 'a skill without one');
  draft = addSkill(draft, 'Martial Arts (Karate)');
  draft = addSkill(draft, 'Handgun');
  assertEqual(JSON.stringify(draft.skills.levels), '{"Martial Arts (Karate)":1,"Handgun":1}', 'an added skill starts at level 1');
  assertEqual(addSkill(draft, 'Handgun').skills.levels.Handgun, 1, 'adding a held skill keeps its level');
  assertEqual(dropSkill(draft, 'Handgun').skills.levels.Handgun, 0, 'a dropped skill is sent at 0, which the engine drops');
}

function testChangingMethodOrRoleResetsTheSkillLevels() {
  let draft: CyberpunkCreationDraft = { ...EMPTY_CYBERPUNK_DRAFT, role: 'Tech', method: 'edgerunner' };
  draft = adjustSkill(draft, skillLine({ line: 'Basic Tech', key: 'Basic Tech', level: 6 }), -1);
  draft = setSkillSpecialty(draft, 'Science (choose 1)', 'Chemistry');
  const cp = selectMethod(draft, 'completePackage', []);
  assertEqual(JSON.stringify(cp.skills.levels), '{}', "the Edgerunner levels are keyed by the Role's list: a new method starts over");
  assertEqual(cp.skills.specialties['Science (choose 1)'], 'Chemistry', 'the named field is kept');
  const medtech = selectRole(draft, 'Medtech', corpusRecords('lifepath_table'));
  assertEqual(JSON.stringify(medtech.skills.levels), '{}', "a new Role's list starts over");
  assertEqual(medtech.skills.specialties['Science (choose 1)'], 'Chemistry', 'the named field is kept');
}

function testSkillOptionsAreTheMasterSkillListRecords() {
  const records = corpusRecords('skill');
  const options = skillOptions(records);
  assertEqual(options.length, 66, 'CUI F-24: the 66 skills of the Master Skill List');
  assertEqual(options.filter((o) => o.x2).length, 7, 'CUI F-41: 7 (x2) skills');
  assertEqual(
    options.filter((o) => o.specialtyRequired).map((o) => o.name).sort().join('|'),
    records.filter((r) => r.specialty_required === true).map((r) => String(r.name)).sort().join('|'),
    'a skill that needs a specialty is the record that says so'
  );
  assertEqual(options[0].name, String(records[0].name), 'record order');
}

function testTheSpentFigureIsTheEngineRow() {
  const p = preview({
    skills: {
      lines: [],
      rows: [
        { id: 'cpr.skills.creation_points_spent', value: 82, detail: '82 creation skill points spent' },
        { id: 'cpr.skills.creation_points_remaining', value: 4, detail: '4 of 86 left' },
      ],
    },
  });
  assertEqual(
    skillPointTiles(p).map((t) => `${t.id}=${t.value}`).join('|'),
    'cpr.skills.creation_points_spent=82|cpr.skills.creation_points_remaining=4',
    "the points spent and left are the engine's rows"
  );
  assertEqual(skillPointTiles(preview({})).length, 0, 'a refused allocation has no row, so no figure is shown');
}

testNewCharacterUnderCyberpunkRedOpensTheCyberpunkForm();
testTheMethodPickerOffersTheThreeBookMethods();
testNoRoleNoPreview();
testThePreviewRequestCarriesOnlyTheMethodsOwnFields();
testTheCreateRequestCarriesAFreshIdentityAndTheSameStatStep();
testThePreviewsSeedIsKeptSoCreateDrawsThePreviewedDice();
testSwitchingMethodResetsTheDiceAndSeedsTheAllocationFromTheShownStats();
testTheAllocationButtonsMoveOneStatAndNeverBelowZero();
testTemplateRowOptionsComeFromTheRolesCorpusRecords();
testOnlyClaimBlockingProblemsCloseTheButton();
testTilesComeFromTheEnginesRowsOnly();
testTheCreateLayoutIsARegistry();
testTheLifepathOffersEveryGeneralTableAndExactlyTheRolesTables();
testLifepathRowOptionsAreTheRecordsRows();
testLifepathPicksCountsSkipsAndLanguageAreTheDraftsOwnChoices();
testChangingRoleKeepsOnlyTheGeneralLifepathChoices();
testTheCulturalLanguageTileIsTheEnginesRow();
testTheSkillsRequestCarriesTheDraftsLevelsAndSpecialties();
testAdjustSkillMovesFromTheEnginesLevelAndNeverBelowZero();
testCompletePackageAddsAndDropsAnyMasterListSkill();
testChangingMethodOrRoleResetsTheSkillLevels();
testSkillOptionsAreTheMasterSkillListRecords();
testTheSpentFigureIsTheEngineRow();

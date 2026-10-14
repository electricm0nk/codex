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
  templateRowOptions,
  withPreviewSeed,
  type CyberpunkCreationDraft,
} from './cyberpunkCreationModel';
import {
  CYBERPUNK_CREATE_COLUMNS,
  CYBERPUNK_CREATE_LAYOUT,
  type CyberpunkCreateBoxId,
} from './cyberpunkCreateLayout';
import { boxesInColumn } from './sheetLayout';
import { characterCreationGate } from './characterHubRuntime';
import type { CyberpunkCreationPreview, CyberpunkStatValue } from '../boundary/cprCreation';
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

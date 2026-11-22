/**
 * SD-35 AT-35-E2-002 -- the "Rules and features" section, proven once per kind.
 *
 * Nineteen tests, one per record kind the converter writes to
 * `data/sheet_rules/` (`_report.json` `by_kind`): for a fixture character
 * whose `sheetLines` carry one held record of that kind, the record's label
 * and its value are in the DOM, inside the group for that kind. The fixture
 * ids and labels are the live package's own (each `id` is a real
 * `data/sheet_rules/<book>/<kind>/<slug>.json`); the values are the three
 * forms the evaluator produces -- a final number, dice in final form, or the
 * rule's words -- as the IPC carries them.
 */
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import {
  RULES_AND_FEATURES_ID,
  RulesAndFeaturesSection,
  groupSheetLinesByKind,
  sheetLineKindLabel,
} from './CharacterSheet';
import type { SheetLineDto } from '../boundary/loadSavedCharacterDetail';
import { assert, assertEqual } from '../testSupport/asserts';

function line(partial: Partial<SheetLineDto> & Pick<SheetLineDto, 'id' | 'kind' | 'label' | 'form' | 'value'>): SheetLineDto {
  return { also: [], prose: '', condition: null, ...partial };
}

/**
 * The fixture character's "Rules and features" payload: one held record per
 * kind, sorted by kind then label exactly as `sheet_rule::render_sheet` emits.
 */
export const FIXTURE_CHARACTER_SHEET_LINES: readonly SheetLineDto[] = [
  line({ id: 'core_rulebook:ability:1_spell_per_day__15fb2568d9d7cbde', kind: 'ability', label: '+1 Spell per Day', form: 'number', value: '1', prose: 'GM awarded PC with +1 spell per day.' }),
  line({ id: 'adventurers_guide:class:pathfinder_delver', kind: 'class', label: 'Codex-Named Unit (class_adventurers_guide_ag_classes_lst_279)', form: 'number', value: '3' }),
  line({ id: 'core_rulebook:class_feature:fighter_bravery', kind: 'class_feature', label: 'Bravery', form: 'words', value: '', prose: 'You gain a +1 bonus on Will saves against fear.' }),
  line({ id: 'core_rulebook:companion:companion_boar', kind: 'companion', label: 'Companion Boar', form: 'number', value: '1' }),
  line({ id: 'core_rulebook:deity:zon_kuthon', kind: 'deity', label: 'Codex-Named Unit (deity_core_rulebook_cr_deities_lst_22)', form: 'words', value: '', prose: 'God of envy, pain, darkness, loss' }),
  line({ id: 'core_rulebook:domain:death_pharasma', kind: 'domain', label: 'Codex-Named Unit (domain_core_rulebook_cr_domains_lst_46)', form: 'words', value: '', prose: 'You can cause the living to bleed at a touch.' }),
  line({ id: 'core_rulebook:equipment:longsword', kind: 'equipment', label: 'Longsword (Base)', form: 'dice', value: '1d8+4', prose: 'This sword is about 3-1/2 feet in length.\nCritical threat: 2-20' }),
  line({ id: 'advanced_class_guide:equipment_modifier:answering', kind: 'equipment_modifier', label: 'Answering', form: 'words', value: '', prose: 'Enhancement bonus increases by 4 (to a max of 5) for the purpose of the opportunity attack.' }),
  line({ id: 'core_rulebook:feat:acrobatic', kind: 'feat', label: 'Acrobatic', form: 'number', value: '+4', prose: 'You are skilled at leaping, jumping, and flying.' }),
  line({ id: 'core_rulebook:language:terran', kind: 'language', label: 'Terran', form: 'words', value: '' }),
  line({ id: 'bestiary:monster:crab_swarm', kind: 'monster', label: 'Crab Swarm', form: 'number', value: '12' }),
  line({ id: 'advanced_race_guide:monster_ability:grippli_toxic_skin_grippli_poison', kind: 'monster_ability', label: 'Grippli Poison', form: 'words', value: '', prose: 'Skin or weapon--contact or injury; save Fort DC 12' }),
  line({ id: 'ultimate_psionics:power:control_object', kind: 'power', label: 'Control Object', form: 'words', value: '', prose: 'Telekinetically animate a small object.' }),
  line({ id: 'advanced_race_guide:race:catfolk', kind: 'race', label: 'Catfolk', form: 'words', value: '', prose: 'Walk 30 ft.' }),
  line({ id: 'advanced_players_guide:race_trait:racial_sla_ill_omen', kind: 'race_trait', label: 'Ill Omen', form: 'number', value: '1', also: ['1/day', 'CL 1', 'DC 13'] }),
  line({ id: 'core_rulebook:skill:craft_clothing', kind: 'skill', label: 'Craft (Clothing)', form: 'number', value: '3' }),
  line({ id: 'core_rulebook:spell:repel_wood', kind: 'spell', label: 'Repel Wood', form: 'words', value: '', prose: 'Waves of energy roll forth from you.' }),
  line({ id: 'advanced_class_guide:template:starting_gold_acg_maximum', kind: 'template', label: 'Starting Gold ACG ~ Maximum', form: 'words', value: '' }),
  line({ id: 'advanced_players_guide:trait:trait_magical_knack', kind: 'trait', label: 'Magical Knack', form: 'words', value: '', prose: 'Pick a class when you gain this trait: (choice not yet made).', condition: 'when casting a spell of that class' }),
];

const KINDS = [
  'ability',
  'class',
  'class_feature',
  'companion',
  'deity',
  'domain',
  'equipment',
  'equipment_modifier',
  'feat',
  'language',
  'monster',
  'monster_ability',
  'power',
  'race',
  'race_trait',
  'skill',
  'spell',
  'template',
  'trait',
] as const;

function escapeHtml(text: string): string {
  return text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;').replace(/'/g, '&#x27;');
}

function render(lines: readonly SheetLineDto[], unavailableReason: string | null = null): string {
  return renderToStaticMarkup(createElement(RulesAndFeaturesSection, { lines, unavailableReason }));
}

/** The markup of one kind's group: from its `<section data-kind=...>` to the next section or the end. */
function groupMarkup(html: string, kind: string): string {
  const start = html.indexOf(`data-kind="${kind}"`);
  assert(start >= 0, `the ${kind} group must be in the DOM`);
  const rest = html.slice(start);
  const next = rest.indexOf('<section', 1);
  return next < 0 ? rest : rest.slice(0, next);
}

/** The per-kind proof: the held record's label and value are in the DOM, inside its kind's group. */
function verifiesKindReachesTheDom(kind: (typeof KINDS)[number]) {
  const held = FIXTURE_CHARACTER_SHEET_LINES.find((entry) => entry.kind === kind);
  if (held === undefined) {
    throw new Error(`the fixture character holds a ${kind} record`);
  }
  const html = render(FIXTURE_CHARACTER_SHEET_LINES);
  const group = groupMarkup(html, kind);
  assert(group.includes(`data-rule-id="${held.id}"`), `${kind}: the record's id keys its row`);
  assert(group.includes(escapeHtml(held.label)), `${kind}: the label "${held.label}" is in the DOM`);
  if (held.form === 'words') {
    assert(!group.includes('data-sheet-value'), `${kind}: a words line renders no number`);
    assert(group.includes(escapeHtml(held.prose)) || held.prose.length === 0, `${kind}: the rule's words are in the DOM`);
  } else {
    assert(group.includes(`>${escapeHtml(held.value)}</span>`), `${kind}: the value "${held.value}" is in the DOM`);
  }
  for (const also of held.also) {
    assert(group.includes(escapeHtml(also)), `${kind}: "${also}" is on the line`);
  }
  if (held.condition !== null) {
    assert(group.includes(escapeHtml(held.condition)), `${kind}: the condition prints on the line`);
  }
  assert(group.includes(escapeHtml(sheetLineKindLabel(kind))), `${kind}: the group is headed by its kind`);
}

function verifiesAbilityReachesTheDom() {
  verifiesKindReachesTheDom('ability');
}
function verifiesClassReachesTheDom() {
  verifiesKindReachesTheDom('class');
}
function verifiesClassFeatureReachesTheDom() {
  verifiesKindReachesTheDom('class_feature');
}
function verifiesCompanionReachesTheDom() {
  verifiesKindReachesTheDom('companion');
}
function verifiesDeityReachesTheDom() {
  verifiesKindReachesTheDom('deity');
}
function verifiesDomainReachesTheDom() {
  verifiesKindReachesTheDom('domain');
}
function verifiesEquipmentReachesTheDom() {
  verifiesKindReachesTheDom('equipment');
}
function verifiesEquipmentModifierReachesTheDom() {
  verifiesKindReachesTheDom('equipment_modifier');
}
function verifiesFeatReachesTheDom() {
  verifiesKindReachesTheDom('feat');
}
function verifiesLanguageReachesTheDom() {
  verifiesKindReachesTheDom('language');
}
function verifiesMonsterReachesTheDom() {
  verifiesKindReachesTheDom('monster');
}
function verifiesMonsterAbilityReachesTheDom() {
  verifiesKindReachesTheDom('monster_ability');
}
function verifiesPowerReachesTheDom() {
  verifiesKindReachesTheDom('power');
}
function verifiesRaceReachesTheDom() {
  verifiesKindReachesTheDom('race');
}
function verifiesRaceTraitReachesTheDom() {
  verifiesKindReachesTheDom('race_trait');
}
function verifiesSkillReachesTheDom() {
  verifiesKindReachesTheDom('skill');
}
function verifiesSpellReachesTheDom() {
  verifiesKindReachesTheDom('spell');
}
function verifiesTemplateReachesTheDom() {
  verifiesKindReachesTheDom('template');
}
function verifiesTraitReachesTheDom() {
  verifiesKindReachesTheDom('trait');
}

function verifiesTheFixtureCoversEveryKindOnce() {
  const kinds = FIXTURE_CHARACTER_SHEET_LINES.map((entry) => entry.kind);
  assertEqual(kinds.length, KINDS.length, 'one held record per kind');
  assertEqual(new Set(kinds).size, KINDS.length, 'no kind twice');
  for (const kind of KINDS) {
    assert(kinds.includes(kind), `${kind} is in the fixture`);
  }
  const groups = groupSheetLinesByKind(FIXTURE_CHARACTER_SHEET_LINES);
  assertEqual(groups.length, KINDS.length, 'grouped by kind');
  assertEqual(groups.map((group) => group.kind).join(','), [...KINDS].join(','), 'kind order is the engine emission order');
}

function verifiesAnUnavailablePackageSaysSoInsteadOfAnEmptySection() {
  const html = render([], 'no sheet rules under data/sheet_rules');
  assert(html.includes(`id="${RULES_AND_FEATURES_ID}"`), 'the section mounts to carry the reason');
  assert(html.includes('no sheet rules under data/sheet_rules'), 'the reason is the engine text, verbatim');
  assert(!html.includes('data-kind'), 'no group renders without lines');
}

function verifiesNoHeldRuleRendersNothing() {
  assertEqual(render([]), '', 'a character holding no rule gets no section, not an empty one');
}

function verifiesKindLabelsAreNamedOrHumanised() {
  assertEqual(sheetLineKindLabel('class_feature'), 'Class features', 'a named kind');
  assertEqual(sheetLineKindLabel('race_trait'), 'Racial traits', 'a named kind');
  assertEqual(sheetLineKindLabel('spell_school'), 'Spell school', 'an unnamed kind is humanised, never dropped');
}

function verifiesLinesOfOneKindShareOneGroup() {
  const two = [
    line({ id: 'core_rulebook:feat:dodge', kind: 'feat', label: 'Dodge', form: 'number', value: '+1' }),
    line({ id: 'core_rulebook:feat:power_attack', kind: 'feat', label: 'Power Attack', form: 'words', value: '' }),
  ];
  const html = render(two);
  assertEqual((html.match(/data-kind="feat"/g) ?? []).length, 1, 'one group for the kind');
  assert(html.includes('Dodge') && html.includes('Power Attack'), 'both lines in it');
}

async function main() {
  verifiesTheFixtureCoversEveryKindOnce();
  verifiesAbilityReachesTheDom();
  verifiesClassReachesTheDom();
  verifiesClassFeatureReachesTheDom();
  verifiesCompanionReachesTheDom();
  verifiesDeityReachesTheDom();
  verifiesDomainReachesTheDom();
  verifiesEquipmentReachesTheDom();
  verifiesEquipmentModifierReachesTheDom();
  verifiesFeatReachesTheDom();
  verifiesLanguageReachesTheDom();
  verifiesMonsterReachesTheDom();
  verifiesMonsterAbilityReachesTheDom();
  verifiesPowerReachesTheDom();
  verifiesRaceReachesTheDom();
  verifiesRaceTraitReachesTheDom();
  verifiesSkillReachesTheDom();
  verifiesSpellReachesTheDom();
  verifiesTemplateReachesTheDom();
  verifiesTraitReachesTheDom();
  verifiesAnUnavailablePackageSaysSoInsteadOfAnEmptySection();
  verifiesNoHeldRuleRendersNothing();
  verifiesKindLabelsAreNamedOrHumanised();
  verifiesLinesOfOneKindShareOneGroup();
  console.log('rulesAndFeaturesSection: 19 per-kind tests + 5 section tests passed');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

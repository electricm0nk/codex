import { buildSpellsPerDaySurface } from './spellsPerDayModel';
import { assert, assertEqual } from '../testSupport/asserts';
import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';

function explanation(id: string, value: number, detail = 'engine detail'): ExplanationDto {
  return { id, value, detail };
}

function verifiesAWizardsPerDayCountsComeFromTheEnginesOwnRecords() {
  const surface = buildSpellsPerDaySurface([
    explanation('class_spell.wizard.total_spells_per_day.spell_level_0', 4, 'cantrips'),
    explanation('class_spell.wizard.total_spells_per_day.spell_level_1', 3, 'first level'),
    explanation('class_spell.wizard.total_spells_per_day.spell_level_2', 2, 'second level'),
  ]);

  assertEqual(surface.isEmpty, false, 'a caster with records is not empty');
  assertEqual(
    surface.rows.map((row) => `${row.spellLevel}:${row.count}`).join(' '),
    '0:4 1:3 2:2',
    'every grounded spell level reaches the sheet with the engine count'
  );
  assertEqual(surface.rows[0].classToken, 'wizard', 'attributed to the wizard class');
  assertEqual(surface.rows[0].basis, 'total', 'a total count is labelled a total');
}

function verifiesTheTotalIsPreferredOverTheBaseForTheSameLevel() {
  const surface = buildSpellsPerDaySurface([
    explanation('class_spell.wizard.base_spells_per_day.spell_level_1', 2, 'base'),
    explanation('class_spell.wizard.total_spells_per_day.spell_level_1', 3, 'total'),
  ]);

  assertEqual(surface.rows.length, 1, 'one row per spell level, not two conflicting answers');
  assertEqual(surface.rows[0].count, 3, 'the total is what a player actually casts');
  assertEqual(surface.rows[0].basis, 'total', 'and it is labelled as the total');
}

function verifiesABaseOnlyCountIsNotPassedOffAsATotal() {
  const surface = buildSpellsPerDaySurface([
    explanation('class_spell.acg.warpriest.base_spells_per_day.spell_level_1', 1, 'base only'),
  ]);

  assertEqual(surface.rows[0].count, 1, 'the base count is real data and is shown');
  assertEqual(surface.rows[0].basis, 'base', 'but never relabelled as a total');
}

function verifiesABookNamespacedIdResolvesToTheClassNotTheBook() {
  const surface = buildSpellsPerDaySurface([
    explanation('class_spell.acg.arcanist.total_spells_per_day.spell_level_2', 4),
    explanation('class_spell.apg.witch.total_spells_per_day.spell_level_1', 3),
  ]);

  assertEqual(
    surface.rows.map((row) => row.classToken).join(' '),
    'arcanist witch',
    'the class segment wins over the source-book segment'
  );
}

function verifiesInvestigatorExtractsReadThroughTheSameSeam() {
  const surface = buildSpellsPerDaySurface([
    explanation('class_spell.acg.investigator.total_extracts_per_day.extract_level_1', 2),
  ]);

  assertEqual(surface.rows.length, 1, 'extracts are per-day counts too');
  assertEqual(surface.rows[0].classToken, 'investigator', 'attributed to the investigator');
  assertEqual(surface.rows[0].count, 2, 'with the engine count');
}

function verifiesEveryRecordThatIsNotAPerDayCountIsIgnored() {
  const surface = buildSpellsPerDaySurface([
    explanation('class_spell.wizard.spellbook_contents', 5),
    explanation('class_spell.wizard.prepared_spellbook.unsupported', 0),
    explanation('class_chassis.wizard.spell_save_dc.spell_level_1', 13),
  ]);

  assertEqual(surface.isEmpty, true, 'none of these are per-day counts');
}

function verifiesTheDetailTextIsCarriedVerbatim() {
  const detail = 'Wizard level 10 spells/day for spell level 5 from the PF1 Core Wizard table: 2';
  const surface = buildSpellsPerDaySurface([
    explanation('class_spell.wizard.total_spells_per_day.spell_level_5', 2, detail),
  ]);

  assertEqual(surface.rows[0].detail, detail, 'the engine citation crosses byte-identical');
}

function verifiesRowsAreOrderedByClassThenSpellLevel() {
  const surface = buildSpellsPerDaySurface([
    explanation('class_spell.wizard.total_spells_per_day.spell_level_2', 2),
    explanation('class_spell.cleric.total_spells_per_day.spell_level_1', 3),
    explanation('class_spell.wizard.total_spells_per_day.spell_level_0', 4),
  ]);

  assertEqual(
    surface.rows.map((row) => `${row.classToken}:${row.spellLevel}`).join(' '),
    'cleric:1 wizard:0 wizard:2',
    'a multiclass caster reads class by class, level by level'
  );
}

function verifiesANonCasterGetsACleanEmptySurface() {
  const surface = buildSpellsPerDaySurface([
    explanation('class_chassis.fighter.level_1_hit_points', 10),
  ]);

  assert(surface.isEmpty, 'a fighter casts nothing and gets no fabricated rows');
}

async function main() {
  verifiesAWizardsPerDayCountsComeFromTheEnginesOwnRecords();
  verifiesTheTotalIsPreferredOverTheBaseForTheSameLevel();
  verifiesABaseOnlyCountIsNotPassedOffAsATotal();
  verifiesABookNamespacedIdResolvesToTheClassNotTheBook();
  verifiesInvestigatorExtractsReadThroughTheSameSeam();
  verifiesEveryRecordThatIsNotAPerDayCountIsIgnored();
  verifiesTheDetailTextIsCarriedVerbatim();
  verifiesRowsAreOrderedByClassThenSpellLevel();
  verifiesANonCasterGetsACleanEmptySurface();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

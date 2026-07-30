import { buildPetsTabView, formatCompanionStatValue } from './petsTabModel';
import type { AnimalCompanionDto, PilotSnapshotDto } from '../boundary/loadCreateCharacter';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * The Pets tab rendered a generic `"Pets — coming soon."` placeholder while
 * the animal companion / mount stat block was fully computed in the engine
 * the whole time — grounded across all twenty master levels (Hit Dice, base
 * attack bonus, all three saves, hit points, armor class, the natural-armor
 * and Strength advances, and the natural attack), for Druid, Hunter and the
 * Cavalier's Mount. Same defect shape as the feats one `featsTabModel.ts`
 * fixed and the spells one `spellsTabModel.ts` fixed: data ingested and
 * correct everywhere except where a player looks.
 *
 * These tests pin the presentation decisions this tab makes. Every value
 * and every line of prose it renders comes verbatim from the engine —
 * nothing here fabricates a companion field, and the deliberately
 * ungrounded columns (bonus tricks, companion skills and feats, the
 * player-chosen stat increase, the size advance, Evasion/Devotion/
 * Multiattack) are surfaced only as the engine's own honest note.
 */

const WOLF: AnimalCompanionDto = {
  ownerClassLabel: 'Druid',
  roleLabel: 'Animal Companion',
  species: 'Wolf',
  summaryDetail:
    'Druid level 1 animal companion, Wolf: a wholly separate creature with its own combat statistics.',
  stats: [
    { label: 'Hit Points', value: 17, detail: 'maximized first Hit Die plus average for the remaining 1' },
    { label: 'Armor Class', value: 12, detail: 'base 10 + natural armor +2' },
    { label: 'Attack Bonus', value: 2, detail: 'HD*3/4 = 1 plus Strength modifier +1' },
    { label: 'Bite Damage Bonus', value: 1, detail: '1d6 + 1, plus the Trip special attack' },
    { label: 'Fortitude Save', value: 3, detail: 'classlevel/2+2 = 3' },
    { label: 'Reflex Save', value: 3, detail: 'classlevel/2+2 = 3' },
    { label: 'Will Save', value: 0, detail: 'classlevel/3 = 0' },
  ],
  notes: ['Link is vacuous: no Handle Animal check is ever computed.'],
  advancementNote: 'Deliberately NOT grounded: bonus tricks, the companion skill ranks and feats.',
};

function snapshotWith(companion: AnimalCompanionDto | undefined): PilotSnapshotDto {
  return {
    abilityModifiers: {
      strength: 0,
      dexterity: 0,
      constitution: 0,
      intelligence: 0,
      wisdom: 0,
      charisma: 0,
    },
    baseAttackBonus: 0,
    baseSaves: { fortitude: 0, reflex: 0, will: 0 },
    baselineMeleeAttackBonus: 0,
    baselineArmorClass: 10,
    totalSaves: { fortitude: 0, reflex: 0, will: 0 },
    selectedSkillModifiers: { climb: 0, intimidate: 0, swim: 0 },
    companion,
  };
}

function verifiesARealCompanionRendersItsOwnStatBlock() {
  const view = buildPetsTabView(snapshotWith(WOLF));
  assertEqual(view.kind, 'Companion', 'a snapshot carrying a companion renders the stat block');
  if (view.kind !== 'Companion') {
    return;
  }
  assertEqual(view.heading, 'Wolf', 'the species is the heading');
  assertEqual(
    view.subheading,
    'Druid Animal Companion',
    'the owning class and role read together beneath the species'
  );
  assertEqual(view.stats.length, 7, 'every grounded statistic reaches the tab');
  assertEqual(view.summaryDetail, WOLF.summaryDetail, 'the engine summary is carried verbatim');
}

/**
 * A total and a modifier must not read the same way. `+12 AC` would be
 * wrong; `2 Attack Bonus` would be wrong the other way.
 */
function verifiesTotalsRenderPlainAndModifiersRenderSigned() {
  assertEqual(formatCompanionStatValue({ label: 'Hit Points', value: 17, detail: '' }), '17', 'HP is a total');
  assertEqual(formatCompanionStatValue({ label: 'Armor Class', value: 12, detail: '' }), '12', 'AC is a total');
  assertEqual(
    formatCompanionStatValue({ label: 'Attack Bonus', value: 2, detail: '' }),
    '+2',
    'an attack bonus is a modifier and must carry its sign'
  );
  assertEqual(
    formatCompanionStatValue({ label: 'Fortitude Save', value: 3, detail: '' }),
    '+3',
    'a save is a modifier'
  );
  assertEqual(
    formatCompanionStatValue({ label: 'Bite Damage Bonus', value: 1, detail: '' }),
    '+1',
    'a damage bonus is a modifier'
  );
}

/**
 * A Wolf at 2 HD has a Will save of 2/3 = 0. That is a real computed
 * value, and `+0` is its correct rendering — it must not be hidden or
 * blanked the way a genuine absence would be.
 */
function verifiesAGenuinelyComputedZeroModifierRendersAsPlusZero() {
  assertEqual(
    formatCompanionStatValue({ label: 'Will Save', value: 0, detail: '' }),
    '+0',
    'a floor-division 0 is a real value, not an absence'
  );
  const view = buildPetsTabView(snapshotWith(WOLF));
  assert(
    view.kind === 'Companion' && view.stats.some((stat) => stat.label === 'Will Save' && stat.rendered === '+0'),
    'the zero Will save must still render'
  );
}

function verifiesNegativeModifiersKeepTheirOwnSign() {
  assertEqual(
    formatCompanionStatValue({ label: 'Attack Bonus', value: -1, detail: '' }),
    '-1',
    'a negative modifier renders with its own minus, never a doubled sign'
  );
}

/** Every row carries the engine's own derivation prose, never a rewrite. */
function verifiesEachRowCarriesItsOwnEngineDetail() {
  const view = buildPetsTabView(snapshotWith(WOLF));
  if (view.kind !== 'Companion') {
    throw new Error('expected a companion view');
  }
  for (const stat of view.stats) {
    const source = WOLF.stats.find((candidate) => candidate.label === stat.label);
    assert(source !== undefined, `row ${stat.label} must come from a real engine record`);
    assertEqual(stat.detail, source?.detail ?? '', `${stat.label} detail is carried verbatim`);
  }
}

/**
 * The honest list of what is deliberately NOT modelled is a deliverable in
 * its own right — surfacing it is fine and good, fabricating values for it
 * is not (`docs/governance/no-stub-mvp-doctrine.md`).
 */
function verifiesTheEnginesNotGroundedNoteIsSurfacedVerbatim() {
  const view = buildPetsTabView(snapshotWith(WOLF));
  if (view.kind !== 'Companion') {
    throw new Error('expected a companion view');
  }
  assertEqual(view.advancementNote, WOLF.advancementNote ?? null, 'carried verbatim, never paraphrased');
  assertEqual(view.notes.length, 1, 'the vacuous-ability notes reach the tab too');
}

function verifiesAMissingAdvancementNoteIsAbsentRatherThanInvented() {
  const view = buildPetsTabView(snapshotWith({ ...WOLF, advancementNote: undefined, notes: [] }));
  if (view.kind !== 'Companion') {
    throw new Error('expected a companion view');
  }
  assertEqual(view.advancementNote, null, 'no note is rendered when the engine emitted none');
  assertEqual(view.notes.length, 0, 'no notes are invented');
}

/**
 * A companion-less class must say so cleanly — not render an empty or
 * zeroed stat block.
 */
function verifiesACompanionlessComputedCharacterGetsACleanEmptyState() {
  const view = buildPetsTabView(snapshotWith(undefined));
  assertEqual(view.kind, 'None', 'no companion means the None state, never a zeroed block');
  if (view.kind !== 'None') {
    return;
  }
  assert(view.message.length > 0, 'the empty state says something');
  assert(
    !view.message.toLowerCase().includes('coming soon'),
    'the empty state is a real answer about this character, not a placeholder about the app'
  );
}

/**
 * A blocked build carries no snapshot at all, so the tab genuinely does
 * not know whether this character has a companion. Claiming "no companion"
 * there would be a fabrication of exactly the kind this repo's doctrine
 * forbids — the two absences must read differently.
 */
function verifiesABlockedBuildIsDistinguishedFromAGenuineAbsence() {
  const blocked = buildPetsTabView(null);
  assertEqual(blocked.kind, 'Unavailable', 'no snapshot means unknown, not absent');
  const absent = buildPetsTabView(snapshotWith(undefined));
  assertEqual(absent.kind, 'None', 'a computed build with no companion is a real absence');
  if (blocked.kind !== 'Unavailable' || absent.kind !== 'None') {
    return;
  }
  assert(
    blocked.message !== absent.message,
    'an unknown and a known-absent companion must not read identically'
  );
}

function verifiesAMountRendersUnderItsOwnRoleAndSpecies() {
  const view = buildPetsTabView(
    snapshotWith({
      ownerClassLabel: 'Cavalier',
      roleLabel: 'Mount',
      species: 'Horse',
      summaryDetail: 'Cavalier level 1 mount, Horse.',
      stats: [{ label: 'Hoof Damage Bonus', value: 4, detail: '1d4 + 4' }],
      notes: [],
    })
  );
  if (view.kind !== 'Companion') {
    throw new Error('expected a companion view');
  }
  assertEqual(view.heading, 'Horse', 'the Mount is not relabelled as an animal companion');
  assertEqual(view.subheading, 'Cavalier Mount', 'the class names its own role');
}

async function main() {
  verifiesARealCompanionRendersItsOwnStatBlock();
  verifiesTotalsRenderPlainAndModifiersRenderSigned();
  verifiesAGenuinelyComputedZeroModifierRendersAsPlusZero();
  verifiesNegativeModifiersKeepTheirOwnSign();
  verifiesEachRowCarriesItsOwnEngineDetail();
  verifiesTheEnginesNotGroundedNoteIsSurfacedVerbatim();
  verifiesAMissingAdvancementNoteIsAbsentRatherThanInvented();
  verifiesACompanionlessComputedCharacterGetsACleanEmptyState();
  verifiesABlockedBuildIsDistinguishedFromAGenuineAbsence();
  verifiesAMountRendersUnderItsOwnRoleAndSpecies();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

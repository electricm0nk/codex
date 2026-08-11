import { loadMonsterCatalog, type MonsterCatalogEntryDto } from '../boundary/loadMonsterCatalog';
import { hasTauriRuntime } from '../boundary/runtime';

/**
 * Sample data for the browser preview (no Tauri backend) — keeps the catalog
 * screen walkable without the desktop runtime, matching the
 * `spellCatalog/spellCatalogRuntime.ts` and
 * `equipmentCatalog/equipmentCatalogRuntime.ts` convention.
 *
 * Every value below is transcribed from the real ingested record in
 * `data/corpus/beastiary/monster/`, so the preview never shows a monster the
 * corpus does not contain or a number it does not state. In the desktop app
 * this branch is never taken: `list_monster_catalog` serves all 60 (46
 * Bestiary 1 + 14 Bonus Bestiary).
 */
function buildPreviewCatalog(): MonsterCatalogEntryDto[] {
  return [
    {
      key: 'beastiary1:monster:ghoul',
      book: 'B1',
      name: 'Ghoul',
      challengeRating: 1,
      size: 'M',
      speedFt: 30,
      raceType: 'Undead',
      raceSubtype: null,
      sourcePage: 'p.146',
      naturalAttacks: [
        { name: 'Claw', damageDice: '1d6', damageDiceSource: 'monsterRowToken', groundingNote: null },
        { name: 'Bite', damageDice: '1d6', damageDiceSource: 'monsterRowToken', groundingNote: null },
      ],
      speeds: [{ mode: 'Walk', feet: 30 }],
      monsterClass: null,
      abilities: [],
      externalAbilityRefs: [],
    },
    {
      key: 'beastiary1:monster:gnoll',
      book: 'B1',
      name: 'Gnoll',
      challengeRating: 1,
      size: 'M',
      speedFt: 30,
      raceType: 'Humanoid',
      raceSubtype: 'Gnoll',
      sourcePage: 'p.155',
      naturalAttacks: [],
      speeds: [{ mode: 'Walk', feet: 30 }],
      monsterClass: null,
      abilities: [],
      externalAbilityRefs: [],
    },
    {
      key: 'beastiary1:monster:wolf',
      book: 'B1',
      name: 'Wolf',
      challengeRating: 1,
      size: 'M',
      speedFt: 50,
      raceType: 'Animal',
      raceSubtype: null,
      sourcePage: 'p.278',
      naturalAttacks: [
        { name: 'Bite', damageDice: '1d6', damageDiceSource: 'monsterRowToken', groundingNote: null },
      ],
      speeds: [{ mode: 'Walk', feet: 50 }],
      monsterClass: null,
      abilities: [],
      externalAbilityRefs: [],
    },
    {
      key: 'beastiary1:monster:shark',
      book: 'B1',
      name: 'Shark',
      challengeRating: 2,
      size: 'L',
      speedFt: 0,
      raceType: 'Animal',
      raceSubtype: 'Aquatic',
      sourcePage: 'p.247',
      naturalAttacks: [
        { name: 'Bite', damageDice: '1d8', damageDiceSource: 'monsterRowToken', groundingNote: null },
      ],
      speeds: [],
      monsterClass: null,
      abilities: [],
      externalAbilityRefs: [],
    },
    {
      key: 'beastiary1:monster:ankheg',
      book: 'B1',
      name: 'Ankheg',
      challengeRating: 3,
      size: 'L',
      speedFt: 30,
      raceType: 'Magical Beast',
      raceSubtype: null,
      sourcePage: 'p.15',
      naturalAttacks: [
        {
          name: 'Bite',
          damageDice: '2d6',
          damageDiceSource: 'publishedText',
          groundingNote:
            "This monster's row names the attack with `ABILITY:Internal|AUTOMATIC|Bite` and " +
            'supplies no dice at any hop, so the dice are grounded from the published Bestiary 1 ' +
            'text ("bite +5 (2d6+4 plus 1d4 acid and grab)").',
        },
      ],
      speeds: [{ mode: 'Walk', feet: 30 }],
      monsterClass: null,
      abilities: [],
      externalAbilityRefs: [],
    },
    {
      key: 'beastiary1:monster:gelatinous_cube',
      book: 'B1',
      name: 'Gelatinous Cube',
      challengeRating: 3,
      size: 'L',
      speedFt: 15,
      raceType: 'Ooze',
      raceSubtype: null,
      sourcePage: 'p.138',
      naturalAttacks: [
        { name: 'Slam', damageDice: '1d6', damageDiceSource: 'monsterRowToken', groundingNote: null },
      ],
      speeds: [{ mode: 'Walk', feet: 15 }],
      monsterClass: null,
      abilities: [],
      externalAbilityRefs: [],
    },
    {
      // Bonus Bestiary's chassis + features, transcribed from
      // `data/corpus/bonus_bestiary/monster/allip.json` and the three ability
      // records it names. Allip is the preview's Bonus Bestiary row precisely
      // because it exercises the two things Bestiary 1's rows cannot: a
      // fly-only `MOVE:` token and ingested `monster_ability` records.
      key: 'bonus_bestiary:monster:allip',
      book: 'BB',
      name: 'Allip',
      challengeRating: 3,
      size: 'M',
      speedFt: 0,
      raceType: 'Undead',
      raceSubtype: 'Incorporeal',
      sourcePage: 'p.4',
      naturalAttacks: [
        {
          name: 'Incorporeal touch',
          damageDice: '0',
          damageDiceSource: 'monsterRowToken',
          groundingNote: null,
        },
      ],
      speeds: [{ mode: 'Fly', feet: 30 }],
      monsterClass: 'Undead:4',
      abilities: [
        {
          key: 'bonus_bestiary:monster_ability:babble',
          name: 'Babble',
          facet: 'SpecialAttack',
          delivery: 'Supernatural',
          description:
            'An allip constantly mutters and whines to itself, creating a hypnotic effect. All ' +
            'sane creatures within 60 feet of the allip must succeed on a DC %1 Will save or be ' +
            'fascinated for 2d4 rounds.',
          sourcePage: 'p.4',
        },
        {
          key: 'bonus_bestiary:monster_ability:madness',
          name: 'Madness',
          facet: 'SpecialQuality',
          delivery: 'Supernatural',
          description:
            'Anyone targeting an allip with a thought detection, mind control, or telepathic ' +
            'ability makes direct contact with its tortured mind and takes 1d4 points of Wisdom ' +
            'damage.',
          sourcePage: 'p.4',
        },
        {
          key: 'bonus_bestiary:monster_ability:touch_of_insanity',
          name: 'Touch of Insanity',
          facet: 'SpecialAttack',
          delivery: 'Supernatural',
          description:
            'The touch of an allip deals 1d4 points of Wisdom damage. A successful critical hit ' +
            'causes 1d4 points of Wisdom damage and 1 point of Wisdom drain.',
          sourcePage: 'p.4',
        },
      ],
      externalAbilityRefs: ['Channel Resistance', 'Flight Maneuverability'],
    },
  ];
}

export async function loadMonsterCatalogRuntime(): Promise<MonsterCatalogEntryDto[]> {
  if (!hasTauriRuntime()) {
    return buildPreviewCatalog();
  }
  const response = await loadMonsterCatalog();
  return response.entries;
}

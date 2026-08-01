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
 * this branch is never taken: `list_monster_catalog` serves all 41.
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

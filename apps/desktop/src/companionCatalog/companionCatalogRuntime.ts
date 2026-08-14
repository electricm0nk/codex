import {
  loadCompanionCatalog,
  type CompanionCatalogEntryDto,
} from '../boundary/loadCompanionCatalog';
import { hasTauriRuntime } from '../boundary/runtime';

/**
 * Sample data for the browser preview (no Tauri backend) — keeps the catalog
 * screen walkable without the desktop runtime, matching the
 * `monsterCatalog/monsterCatalogRuntime.ts` convention.
 *
 * Every value below is transcribed from the real ingested record in
 * `data/corpus/inner_sea_combat/companion/` and
 * `data/corpus/inner_sea_intrigue/companion/`, so the preview never shows a
 * companion the corpus does not contain or a number it does not state. In the
 * desktop app this branch is never taken: `list_companion_catalog` serves all
 * 15 registered creatures across four books. That figure is a doc comment, not
 * a pin — the count that is enforced lives in `reach_gate.rs`'s per-record
 * claims, which derive it from the corpus directories rather than from prose.
 */
function buildPreviewCatalog(): CompanionCatalogEntryDto[] {
  return [
    {
      key: 'inner_sea_combat:companion:companion_griffon',
      book: 'ISC',
      name: 'Companion (Griffon)',
      size: 'L',
      speeds: [
        { mode: 'Walk', feet: 30 },
        { mode: 'Fly', feet: 40 },
      ],
      reachFeet: null,
      raceType: 'Magical Beast',
      raceSubtype: null,
      monsterClass: 'Companion:2',
      typeSegments: [],
      naturalAttacks: [{ name: 'Bite', damageDice: null }],
      statAdjustments: [
        { ability: 'STR', amount: 6 },
        { ability: 'DEX', amount: 4 },
        { ability: 'CON', amount: 6 },
        { ability: 'INT', amount: -6 },
        { ability: 'WIS', amount: 2 },
        { ability: 'CHA', amount: -2 },
      ],
      naturalArmor: 4,
      sourcePage: null,
      abilities: [
        {
          key: 'inner_sea_combat:companion:unable_to_carry_a_rider_while_flying',
          name: 'Unable to carry a rider while flying',
          facet: 'SpecialQuality',
          delivery: null,
          typeSegments: ['SpecialQuality'],
          description: null,
          descriptionVariants: [],
          statAdjustments: [],
          sourcePage: null,
        },
        {
          key: 'inner_sea_combat:companion:companion_advancement_griffon',
          name: 'Companion Advancement (Griffon)',
          facet: 'CompanionAdvancement',
          delivery: null,
          typeSegments: ['CompanionAdvancement'],
          description: null,
          descriptionVariants: [],
          statAdjustments: [
            { ability: 'STR', amount: 2 },
            { ability: 'CON', amount: 2 },
          ],
          sourcePage: null,
        },
      ],
      externalAbilityRefs: ['Scent'],
    },
    {
      key: 'inner_sea_intrigue:companion:familiar_clockwork_spy',
      book: 'ISI',
      name: 'Familiar (Clockwork Spy)',
      size: 'T',
      speeds: [
        { mode: 'Walk', feet: 30 },
        { mode: 'Fly', feet: 30 },
      ],
      reachFeet: 0,
      raceType: 'Construct',
      raceSubtype: 'Clockwork',
      monsterClass: 'Construct:1',
      typeSegments: ['Companion', 'Familiar', 'Construct'],
      naturalAttacks: [{ name: 'Slam', damageDice: null }],
      statAdjustments: [{ ability: 'DEX', amount: 2 }],
      naturalArmor: null,
      sourcePage: 'p.47',
      abilities: [
        {
          key: 'inner_sea_intrigue:companion:clockwork_spy_tinkering',
          name: 'Tinkering',
          facet: 'SpecialQuality',
          delivery: 'Extraordinary',
          typeSegments: ['ClockworkSpyRacialAbility', 'SpecialQuality', 'Extraordinary'],
          description: null,
          descriptionVariants: [],
          statAdjustments: [],
          sourcePage: null,
        },
      ],
      externalAbilityRefs: ['Flight Maneuverability'],
    },
  ];
}

export async function loadCompanionCatalogRuntime(): Promise<CompanionCatalogEntryDto[]> {
  if (!hasTauriRuntime()) {
    return buildPreviewCatalog();
  }
  const response = await loadCompanionCatalog();
  return response.entries;
}

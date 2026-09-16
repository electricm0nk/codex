import {
  loadIntelligentItemCatalog,
  type IntelligentItemComponentDto,
} from '../boundary/loadIntelligentItemCatalog';
import { hasTauriRuntime } from '../boundary/runtime';

/**
 * Sample data for the browser preview (no Tauri backend) — keeps the catalog
 * screen walkable without the desktop runtime, matching the
 * `companionCatalog/companionCatalogRuntime.ts` convention.
 *
 * Every value below is transcribed from the real converted record in
 * `data/sheet_rules/core_rulebook/equipment_modifier/` (SD-35 AT-35-E6-003
 * cycle 11 moved the backing command off the ingest format), so the preview
 * never shows a component the package does not contain, a number it does not
 * state, a variable id it does not hold or a label it does not print. In the
 * desktop app this branch is never taken: `list_intelligent_item_catalog`
 * serves all 152 printed components across both books.
 */
function buildPreviewCatalog(): IntelligentItemComponentDto[] {
  return [
    {
      book: 'core_rulebook',
      family: 'Base',
      key: 'Intelligent Item ~ Base',
      name: 'Intelligent Magic Item Base',
      costGp: 500,
      description:
        'Empathy allows the item to encourage or discourage certain actions by communicating ' +
        'emotions and urges. It does not allow for verbal communication.',
      mechanics: [
        {
          variable: 'v027321c791a2c8bd',
          effect: 'Intelligent Item Ego',
          formula:
            'Base Ego from item price (cumulative): price ≥ 1001 gp: +1 Ego; ' +
            'price ≥ 5001 gp: +1 Ego; price ≥ 10001 gp: +1 Ego; ' +
            'price ≥ 20001 gp: +1 Ego; price ≥ 50001 gp: +2 Ego; ' +
            'price ≥ 100001 gp: +2 Ego; price ≥ 200001 gp: +4 Ego',
          condition: null,
          bonusType: null,
        },
        { variable: 'v826ec4e8a9975d96', effect: 'Int Item Stat INT', formula: '+10', condition: null, bonusType: null },
        { variable: 'v5ceaf13186f42770', effect: 'Int Item Stat WIS', formula: '+10', condition: null, bonusType: null },
        { variable: 'v39ca80604c6d98f0', effect: 'Int Item Stat CHA', formula: '+10', condition: null, bonusType: null },
      ],
      egoDelta: null,
    },
    {
      book: 'core_rulebook',
      family: 'Ability Score',
      key: 'Intelligent Item ~ Ability Score / Intelligence 14',
      name: 'Int Item / Stat Intelligence 14',
      costGp: 1000,
      description: null,
      mechanics: [
        { variable: 'v027321c791a2c8bd', effect: 'Intelligent Item Ego', formula: '+2', condition: null, bonusType: null },
        { variable: 'v826ec4e8a9975d96', effect: 'Int Item Stat INT', formula: '+4', condition: null, bonusType: null },
        {
          variable: 've2e0ccdd617a6c9d',
          effect: 'Speech Bonus Lang',
          formula: '+2',
          condition: null,
          bonusType: null,
        },
      ],
      egoDelta: 2,
    },
    {
      book: 'core_rulebook',
      family: 'Alignment',
      key: 'Intelligent Item ~ Alignment / Lawful Good',
      name: 'Int Item / Align (LG)',
      costGp: 0,
      description: null,
      mechanics: [],
      egoDelta: null,
    },
    {
      book: 'core_rulebook',
      family: 'Communication',
      key: 'Intelligent Item ~ Communication / Speech',
      name: 'Int Item / Communication Speech',
      costGp: 500,
      description:
        'An intelligent item with the capability for speech can talk using any of the languages ' +
        'it knows. It automatically knows Common.',
      mechanics: [],
      egoDelta: null,
    },
    {
      book: 'core_rulebook',
      family: 'Purpose',
      key: 'Intelligent Item ~ Purpose / Slay All',
      name: 'Int Item / Defeat/slay all (other than the item and the wielder)',
      costGp: null,
      description: 'Defeat/slay all (other than the item and the wielder)',
      mechanics: [
        {
          variable: 'v027321c791a2c8bd',
          effect: 'Intelligent Item Ego',
          formula: '+2',
          condition: null,
          bonusType: 'Purpose',
        },
      ],
      egoDelta: 2,
    },
    {
      book: 'mythic_adventures',
      family: 'Base',
      key: 'Legendary Item ~ Intelligent Item',
      name: 'Intelligent',
      costGp: 1000,
      description: null,
      mechanics: [
        {
          variable: 'v027321c791a2c8bd',
          effect: 'Intelligent Item Ego',
          formula:
            'Base Ego from item price (cumulative): price ≥ 1001 gp: +1 Ego; ' +
            'price ≥ 5001 gp: +1 Ego; price ≥ 10001 gp: +1 Ego; ' +
            'price ≥ 20001 gp: +1 Ego; price ≥ 50001 gp: +2 Ego; ' +
            'price ≥ 100001 gp: +2 Ego; price ≥ 200001 gp: +4 Ego',
          condition: null,
          bonusType: null,
        },
      ],
      egoDelta: null,
    },
  ];
}

export async function loadIntelligentItemCatalogRuntime(): Promise<IntelligentItemComponentDto[]> {
  if (!hasTauriRuntime()) {
    return buildPreviewCatalog();
  }
  return loadIntelligentItemCatalog();
}

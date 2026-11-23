import { createElement } from 'react';
import { readFileSync } from 'node:fs';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import type { HeldEquipmentDetailDto } from '../boundary/describeHeldEquipment';
import type { WeaponDamageDto } from '../boundary/loadSavedCharacterDetail';
import { ABSENT, GearDetail } from './GearDetail';
import { GearPanels } from './GearScreen';
import { gearScreenModel } from './gearScreenModel';

const copyOf = (itemId: string, over: Partial<HeldEquipmentDetailDto['copy']> = {}): HeldEquipmentDetailDto['copy'] => ({
  itemId,
  name: itemId,
  baseName: itemId,
  bonus: 0,
  activeState: 'SelectedInactive',
  slot: 'none',
  location: { kind: 'carried' },
  ordinal: 0,
  quantity: 1,
  copies: [{ itemId, bonus: 0, ordinal: 0 }],
  baseQuantity: 1,
  ...over,
});

const entryOf = (over: Partial<HeldEquipmentDetailDto['entry']> = {}): HeldEquipmentDetailDto['entry'] => ({
  key: 'Sack',
  category: 'General',
  name: 'Sack',
  costGp: 0.1,
  weightLbs: 0.5,
  book: 'CRB',
  types: ['Goods'],
  description: null,
  ...over,
});

const longsword: WeaponDamageDto = {
  weaponItemId: 'Longsword (Base)',
  weaponRecordKey: 'Longsword (Base)',
  baseDice: { count: 1, dieSize: 8 },
  strDamageModifier: 3,
  wieldCategory: 'OneHanded',
  enhancementAttackBonus: null,
  enhancementDamageBonus: null,
  criticalThreatRange: [19, 20],
  criticalMultiplier: 2,
  rangeIncrement: null,
  featEffects: [],
  inHand: false,
  unarmed: false,
  nonlethal: false,
  usesAmmunition: false,
  ammunitionCarried: 0,
};

const html = (detail: HeldEquipmentDetailDto, locationLabel = 'Carried') => renderToStaticMarkup(createElement(GearDetail, { detail, locationLabel }));
/** The text of the `<dd>` with `data-testid` `id`. */
const field = (markup: string, id: string) => markup.match(new RegExp(`data-testid="${id}"[^>]*>([^<]*)<`))?.[1];

function everyFieldRendersTheDtoValueOrADash() {
  const sword = html({
    entry: entryOf({ key: 'Longsword (Base)', name: 'Longsword', costGp: 15, weightLbs: 4, types: ['Weapon', 'Martial'], description: 'This sword is about 3-1/2 feet in length.' }),
    weapon: longsword,
    armor: null,
    container: null,
    copy: copyOf('Longsword (Base)', { name: 'Longsword', baseName: 'Longsword', slot: 'wield_main' }),
  });
  assert(sword.includes('data-testid="gear-detail"'), 'the panel mounts under its test id');
  const expected: Array<[string, string]> = [
    ['gear-detail-name', 'Longsword'],
    ['gear-detail-type', 'Weapon, Martial'],
    ['gear-detail-weight', '4 lb'],
    ['gear-detail-cost', '15 gp'],
    ['gear-detail-slot', 'Main hand'],
    ['gear-detail-damage', '1d8'],
    ['gear-detail-critical', '19–20/×2'],
    ['gear-detail-range', ABSENT],
    ['gear-detail-wield', 'OneHanded'],
    ['gear-detail-ac', ABSENT],
    ['gear-detail-max-dex', ABSENT],
    ['gear-detail-acp', ABSENT],
    ['gear-detail-asf', ABSENT],
    ['gear-detail-capacity', ABSENT],
    ['gear-detail-description', 'This sword is about 3-1/2 feet in length.'],
    ['gear-detail-book', 'CRB'],
    ['gear-detail-size', ABSENT],
    ['gear-detail-modifiers', ABSENT],
    ['gear-detail-damage-override', ABSENT],
    ['gear-detail-weight-override', ABSENT],
    ['gear-detail-cost-override', ABSENT],
    ['gear-detail-quantity', '1'],
    ['gear-detail-location', 'Carried'],
  ];
  for (const [id, text] of expected) {
    assertEqual(field(sword, id), text, id);
  }
  const shirt = html({
    entry: entryOf({ key: 'Chain Shirt (Base)', name: 'Chain Shirt', costGp: null, weightLbs: null, types: [], description: '  ' }),
    weapon: null,
    armor: { itemId: 'Chain Shirt (Base)', equipmentRecordKey: 'Chain Shirt (Base)', category: 'ArmsArmor', armorClassBonus: 4, maxDex: 4, armorCheckPenalty: -2, spellFailure: 20 },
    container: null,
    copy: copyOf('Chain Shirt (Base)', { appliedModifiers: ['+1'], abilities: ['Mithral: light'], size: 'Small', weightOverrideLbs: 12.5, costOverrideCopper: 9000, damageOverride: '1d6', properties: ['Glows'] }),
  });
  const armorExpected: Array<[string, string]> = [
    ['gear-detail-ac', '+4'],
    ['gear-detail-max-dex', '+4'],
    ['gear-detail-acp', '-2'],
    ['gear-detail-asf', '20%'],
    ['gear-detail-damage', ABSENT],
    ['gear-detail-weight', ABSENT],
    ['gear-detail-cost', ABSENT],
    ['gear-detail-type', 'General'],
    ['gear-detail-description', ABSENT],
    ['gear-detail-size', 'Small'],
    ['gear-detail-abilities', 'Mithral: light'],
    ['gear-detail-modifiers', '+1'],
    ['gear-detail-damage-override', '1d6'],
    ['gear-detail-weight-override', '12.5 lb'],
    ['gear-detail-cost-override', '9000 cp'],
    ['gear-detail-properties', 'Glows'],
  ];
  for (const [id, text] of armorExpected) {
    assertEqual(field(shirt, id), text, `armor: ${id}`);
  }
  assert(!/\b0 (lb|gp)\b/.test(shirt), 'an unstated weight or cost is never 0');
}

function aRenamedCopyShowsBothNames() {
  const markup = html({ entry: entryOf(), weapon: null, armor: null, container: null, copy: copyOf('Sack', { name: 'Camping Gear', baseName: 'Sack', quantity: 1 }) });
  assertEqual(field(markup, 'gear-detail-copy-name'), 'Camping Gear (Sack)', 'the custom name beside the base name');
  assertEqual(field(markup, 'gear-detail-name'), 'Sack', 'the item block names the record');
  assertEqual(field(html({ entry: entryOf(), weapon: null, armor: null, container: null, copy: copyOf('Sack') }), 'gear-detail-copy-name'), 'Sack', 'unnamed: one name');
}

function aWeightlessContainerShowsTheWords() {
  const bag = html({
    entry: entryOf({ key: 'Bag of Holding (Type I)', name: 'Bag of Holding (Type I)', costGp: 2500, weightLbs: 15 }),
    weapon: null,
    armor: null,
    container: { capacity: { kind: 'pounds', lbs: 250 }, contentsWeightPct: 0 },
    copy: copyOf('Bag of Holding (Type I)'),
  });
  assertEqual(field(bag, 'gear-detail-capacity'), '250 lb', 'the bag holds 250 lb');
  assertEqual(field(bag, 'gear-detail-contents'), 'Contents weigh nothing', 'and its contents weigh nothing');
  const backpack = html({ entry: entryOf(), weapon: null, armor: null, container: { capacity: { kind: 'unlimited' }, contentsWeightPct: 100 }, copy: copyOf('Backpack') });
  assertEqual(field(backpack, 'gear-detail-contents'), 'Contents weigh in full', 'a plain container');
  assertEqual(field(backpack, 'gear-detail-capacity'), 'No limit', 'an unlimited one');
}

function thePanelDoesNoArithmeticAndReusesTheCatalogDescriptionTest() {
  const source = readFileSync(new URL('./GearDetail.tsx', import.meta.url), 'utf8');
  assert(source.includes("import { hasDescription } from '../equipmentCatalog/EquipmentCatalogScreen'"), 'one description test, the catalog screen’s');
  assert(!/(weightLbs|costGp|quantity|acBonus|maxDex) *[-+*/]/.test(source), 'no arithmetic over DTO numbers');
}

function aSelectedRowIsMarkedAndItsPanelMountsUnderTheTables() {
  const noop = () => undefined;
  const model = gearScreenModel([copyOf('Sack'), copyOf('Rope')], { perItem: [], containerLoads: [] });
  const detail = createElement(GearDetail, { detail: { entry: entryOf(), weapon: null, armor: null, container: null, copy: copyOf('Sack') }, locationLabel: 'Carried' });
  const markup = renderToStaticMarkup(
    createElement(GearPanels, {
      model,
      armorClass: null,
      carriedLbs: null,
      busy: false,
      refusal: null,
      onMove: noop,
      onDrop: noop,
      onSell: noop, loadSellPrice: async () => null,
      onDiscard: noop,
      onSets: { activate: noop, create: noop, rename: noop, remove: noop },
      onCustomizeName: noop,
      onCustomizeModifier: noop,
      selectedId: 'Sack-0-0',
      detail,
    }),
  );
  assert(markup.includes('data-testid="gear-row-Sack-0-0" aria-selected="true"'), 'the selected row says so');
  assert(markup.includes('data-testid="gear-row-Rope-0-0" aria-selected="false"'), 'the other row is not selected');
  assert(markup.indexOf('data-testid="gear-detail"') > markup.indexOf('Drop keeps the item'), 'the panel sits under the tables');
  const sheet = readFileSync(new URL('./CharacterSheet.tsx', import.meta.url), 'utf8');
  assert(sheet.includes('describe={(copy) => describeHeldEquipment({ characterId: props.characterId, copy })}'), 'the sheet reads the panel through the boundary');
}

everyFieldRendersTheDtoValueOrADash();
aRenamedCopyShowsBothNames();
aWeightlessContainerShowsTheWords();
thePanelDoesNoArithmeticAndReusesTheCatalogDescriptionTest();
aSelectedRowIsMarkedAndItsPanelMountsUnderTheTables();
console.log('GearDetail.test.ts: all assertions passed');

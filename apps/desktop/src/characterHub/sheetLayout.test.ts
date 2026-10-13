import { readFileSync } from 'node:fs';
import { assert, assertEqual } from '../testSupport/asserts';
import { buildPrintCss } from './printLayout';
import {
  CREATE_LAYOUT,
  SHEET_COLUMNS,
  SHEET_COLUMN_STYLES,
  SHEET_LAYOUT,
  SQUARE_BLOCK_STYLE,
  SQUARE_BOX_STYLE,
  boxesInColumn,
  type LayoutBox,
} from './sheetLayout';

// Brief 2 L0: the sheet and the Create screen read their box placement from a declarative
// registry, so rearranging a box is an edit to one array entry, not a JSX move. These tests
// pin the configuration; no DOM is rendered (none is available on this box).

function verifiesEveryRegistryHasUniqueIdsAndAStyle(name: string, layout: readonly LayoutBox<string, string>[]) {
  const ids = layout.map((box) => box.id);
  assertEqual(new Set(ids).size, ids.length, `${name}: box ids are unique`);
  for (const box of layout) {
    assert(typeof box.column === 'string' && box.column.length > 0, `${name}: ${box.id} names a column`);
    assert(Number.isFinite(box.order), `${name}: ${box.id} has a numeric order`);
    assert(typeof box.boxStyle === 'object' && box.boxStyle !== null, `${name}: ${box.id} carries a boxStyle`);
  }
}

function verifiesAbilitiesIsOneVerticalColumn() {
  const abilities = boxesInColumn(SHEET_LAYOUT, 'abilities').map((box) => box.id);
  assertEqual(abilities.join('|'), 'abilities', 'the abilities column holds the Abilities box alone');
  assertEqual(SHEET_COLUMN_STYLES.abilities.flexDirection, 'column', 'the abilities column stacks vertically');
  const box = SHEET_LAYOUT.find((entry) => entry.id === 'abilities');
  assertEqual(box?.boxStyle.flexDirection, 'column', 'the Abilities box lists its six scores top to bottom');
}

function verifiesAcAttackSpeedSavesAreStackedOwnBoxesInOrder() {
  const combat = boxesInColumn(SHEET_LAYOUT, 'combat').map((box) => box.id);
  assertEqual(combat.join('|'), 'armorClass|attack|speed|saves', 'AC, Attack, Speed, Saves stack in that order');
  assertEqual(SHEET_COLUMN_STYLES.combat.flexDirection, 'column', 'the combat boxes stack vertically, one box each');
}

function verifiesEveryRegisteredBoxLandsInARenderedColumn() {
  assertEqual(SHEET_COLUMNS.join('|'), 'abilities|combat', 'Abilities column left of the combat stack');
  for (const box of SHEET_LAYOUT) {
    assert(SHEET_COLUMNS.includes(box.column), `${box.id} sits in a column the sheet renders`);
  }
}

function verifiesOrderNotArrayPositionDecidesPlacement() {
  // Rearranging is changing `order`; the array position must not matter.
  const reversed = [...SHEET_LAYOUT].reverse();
  assertEqual(
    boxesInColumn(reversed, 'combat').map((box) => box.id).join('|'),
    boxesInColumn(SHEET_LAYOUT, 'combat').map((box) => box.id).join('|'),
    'boxesInColumn sorts by order, not by array position',
  );
}

function verifiesTheSquareBlockIsTwoByTwoSquares() {
  assertEqual(SQUARE_BLOCK_STYLE.display, 'grid', 'the 2x2 block is a grid');
  assertEqual(SQUARE_BLOCK_STYLE.gridTemplateColumns, '1fr 1fr', 'the 2x2 block has two equal columns');
  assertEqual(SQUARE_BOX_STYLE.aspectRatio, '1', 'each box in the 2x2 block is square');
  // Brief 2 L2b: at full column width the squares are ~640px and push the stat block below the
  // fold, so the block is capped (boxes ~170px) and left-aligned under the name, sheet and Create.
  assertEqual(SQUARE_BLOCK_STYLE.maxWidth, '360px', 'the 2x2 block is capped at 360px');
  assertEqual(SQUARE_BLOCK_STYLE.width, '100%', 'up to the cap it fills its column (never shrinks to content)');
  assertEqual(SQUARE_BLOCK_STYLE.justifySelf, 'start', 'and sits at the left under the name');
}

function verifiesCreateHasItsOwnRegistry() {
  const levels = CREATE_LAYOUT.find((box) => box.id === 'levels');
  assertEqual(levels?.column, 'left', 'Create keeps the level list and HP roller far-left');
  const abilities = CREATE_LAYOUT.find((box) => box.id === 'abilities');
  const skills = CREATE_LAYOUT.find((box) => box.id === 'skills');
  assert(abilities !== undefined && skills !== undefined, 'Create registers Abilities and Skills');
  assertEqual(skills?.column, abilities?.column, 'the Skills box shares the Abilities column on Create');
  assert((skills?.order ?? 0) > (abilities?.order ?? 0), 'Skills sits below Abilities');
}

function verifiesEveryPrintColumnClassIsAppliedBySheet() {
  // printLayout.ts places page 1 by these classes; a class the sheet never applies makes its
  // print rule match nothing (`.sheet-mid` was missing before Brief 2).
  const css = buildPrintCss();
  const source = readFileSync(new URL('./CharacterSheet.tsx', import.meta.url), 'utf8');
  for (const name of ['sheet-cols', 'sheet-left', 'sheet-mid', 'sheet-right']) {
    assert(css.includes(`.${name} `), `print css targets .${name}`);
    assert(source.includes(`className="${name}"`), `CharacterSheet.tsx applies className="${name}" for the print rule`);
  }
}

verifiesEveryRegistryHasUniqueIdsAndAStyle('SHEET_LAYOUT', SHEET_LAYOUT);
verifiesEveryRegistryHasUniqueIdsAndAStyle('CREATE_LAYOUT', CREATE_LAYOUT);
verifiesAbilitiesIsOneVerticalColumn();
verifiesAcAttackSpeedSavesAreStackedOwnBoxesInOrder();
verifiesEveryRegisteredBoxLandsInARenderedColumn();
verifiesOrderNotArrayPositionDecidesPlacement();
verifiesTheSquareBlockIsTwoByTwoSquares();
verifiesCreateHasItsOwnRegistry();
verifiesEveryPrintColumnClassIsAppliedBySheet();
console.log('sheetLayout.test.ts: all assertions passed');

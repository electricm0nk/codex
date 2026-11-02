import { readFileSync } from 'node:fs';
import { assert, assertEqual } from '../testSupport/asserts';
import { buildPrintCss } from './printLayout';
import {
  COMBAT_ROW_WRAP,
  CREATE_LAYOUT,
  SHEET_REGIONS,
  SHEET_REGION_STYLES,
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

function idsIn(region: string): string {
  return boxesInColumn<string, string>(SHEET_LAYOUT, region).map((box) => box.id).join('|');
}

function verifiesTheBoxToRegionMap() {
  assertEqual(SHEET_REGIONS.join('|'), 'leftRail|combatRow|rightColumn', 'the three regions, left to right');
  assertEqual(idsIn('leftRail'), 'portrait|levelXp|classPanel|characterDetails|showProgression', 'the left rail, top to bottom');
  assertEqual(idsIn('combatRow'), 'squareBlock|combatStrip', 'the combat row holds the 2x2 and the strip');
  assertEqual(idsIn('rightColumn'), 'abilities|skills', 'Abilities sits above Skills in the right column');
}

function verifiesAbilitiesListsItsScoresVertically() {
  const box = SHEET_LAYOUT.find((entry) => entry.id === 'abilities');
  assertEqual(box?.boxStyle.flexDirection, 'column', 'the Abilities box lists its six scores top to bottom');
}

function verifiesAcAttackSpeedSavesAreOwnBoxesInOrder() {
  assertEqual(idsIn('combatStrip'), 'armorClass|attack|speed|saves', 'AC, Attack, Speed, Saves in that order');
}

function verifiesTheTwoByTwoAndTheOneByFourShareOneRowThatWraps() {
  // decisions.md §7: one row holds the 2x2 beside the 1x4; below the breakpoint the 1x4 wraps under.
  const row = SHEET_REGION_STYLES.combatRow;
  assertEqual(row.display, 'flex', 'the combat row is a flex container');
  assertEqual(row.flexDirection, 'row', 'it lays the 2x2 and the 1x4 side by side');
  assertEqual(COMBAT_ROW_WRAP, 'wrap', 'the wrap style token');
  assertEqual(row.flexWrap, COMBAT_ROW_WRAP, 'the combat row carries the wrap token');

  const strip = SHEET_REGION_STYLES.combatStrip;
  assertEqual(strip.flexDirection, 'column', 'the 1x4 lists its four boxes top to bottom');

  // A wrap needs a width to wrap at: both children must claim one, or flex-wrap never fires.
  const block = SHEET_LAYOUT.find((box) => box.id === 'squareBlock')?.boxStyle;
  const stripBox = SHEET_LAYOUT.find((box) => box.id === 'combatStrip')?.boxStyle;
  assertEqual(block?.flex, `0 1 ${SQUARE_BLOCK_STYLE.maxWidth}`, 'the 2x2 claims its capped width and takes no more');
  assertEqual(stripBox?.flex, '1 1 320px', 'the 1x4 claims the width the wrap turns on, and fills what its line has left');

  const source = readFileSync(new URL('./CharacterSheet.tsx', import.meta.url), 'utf8');
  assert(
    source.includes("<div style={SHEET_REGION_STYLES.combatRow}>{renderSheetRegion('combatRow')}</div>"),
    'the sheet puts the row style on the element that holds the two boxes',
  );
  assert(source.includes("case 'combatStrip':\n        return <div style={boxStyle}>"), 'the strip draws with its registered style');
  assert(/case 'squareBlock':\n\s*return \(\n\s*<div style=\{boxStyle\}>/.test(source), 'the 2x2 draws with its registered style');
}

function verifiesEveryRegisteredBoxLandsInARenderedStack() {
  const rendered = new Set<string>(Object.keys(SHEET_REGION_STYLES));
  for (const box of SHEET_LAYOUT) {
    assert(rendered.has(box.column), `${box.id} sits in a stack with a registered style`);
  }
  for (const region of SHEET_REGIONS) {
    assert(rendered.has(region), `${region} has a registered style`);
  }
}

function verifiesTheSheetRendersEveryRegionFromTheRegistry() {
  const source = readFileSync(new URL('./CharacterSheet.tsx', import.meta.url), 'utf8');
  for (const region of [...SHEET_REGIONS, 'combatStrip']) {
    assert(source.includes(`renderSheetRegion('${region}')`), `the sheet renders ${region} from the registry`);
  }
  for (const box of SHEET_LAYOUT) {
    assert(source.includes(`case '${box.id}':`), `renderSheetBox draws ${box.id}`);
  }
  // Each panel is drawn in one place only, its registered box: a second JSX use is a box the registry does not place.
  for (const panel of ['<PortraitUpload', '<DetailsPanel', '<AbilitiesPanel', '<SkillsPanel', '<SquareBlock', '<LevelBenefitCard']) {
    const uses = source.split(panel).length - 1;
    assert(uses === (panel === '<LevelBenefitCard' ? 2 : 1), `${panel} is drawn only from its registered box (found ${uses})`);
  }
  assert(!/SHEET_COLUMNS|SHEET_COLUMN_STYLES/.test(source), 'the old middle-only registry is gone');
}

function verifiesTheRailHeadingAndTheProgressionButton() {
  const source = readFileSync(new URL('./CharacterSheet.tsx', import.meta.url), 'utf8');
  assert(!/>\s*Progression\s*</.test(source), 'the left rail no longer carries a "Progression" heading');
  const rail = source.slice(source.indexOf('className="sheet-left"'), source.indexOf('className="sheet-mid"'));
  assert(/>\s*Character Details\s*</.test(rail), 'the left-rail heading reads Character Details');
  assert(source.includes('Show Progression'), 'the rail offers Show Progression');
  assert(/setProgressionOpen\(/.test(source), 'Show Progression changes what the rail shows; it is not a dead button');
}

function verifiesOrderNotArrayPositionDecidesPlacement() {
  // Rearranging is changing `order`; the array position must not matter.
  const reversed = [...SHEET_LAYOUT].reverse();
  assertEqual(
    boxesInColumn(reversed, 'leftRail').map((box) => box.id).join('|'),
    boxesInColumn(SHEET_LAYOUT, 'leftRail').map((box) => box.id).join('|'),
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
verifiesTheBoxToRegionMap();
verifiesAbilitiesListsItsScoresVertically();
verifiesAcAttackSpeedSavesAreOwnBoxesInOrder();
verifiesTheTwoByTwoAndTheOneByFourShareOneRowThatWraps();
verifiesEveryRegisteredBoxLandsInARenderedStack();
verifiesTheSheetRendersEveryRegionFromTheRegistry();
verifiesTheRailHeadingAndTheProgressionButton();
verifiesOrderNotArrayPositionDecidesPlacement();
verifiesTheSquareBlockIsTwoByTwoSquares();
verifiesCreateHasItsOwnRegistry();
verifiesEveryPrintColumnClassIsAppliedBySheet();
console.log('sheetLayout.test.ts: all assertions passed');

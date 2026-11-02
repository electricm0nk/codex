import type { CSSProperties } from 'react';

/**
 * Declarative box placement for the character sheet and the Create screen (Brief 2 L0).
 *
 * Each screen renders its boxes from one of these arrays: `column` says which stack a box lives
 * in, `order` its position in that stack, `boxStyle` the style its own content takes. Moving a
 * box is an edit to its entry here, not a JSX move. No drag-and-drop (operator: "easy in code").
 *
 */
export interface LayoutBox<Id extends string, Column extends string> {
  id: Id;
  column: Column;
  order: number;
  boxStyle: CSSProperties;
}

/** Sheet boxes rendered from `SHEET_LAYOUT` (see `renderSheetBox` in `CharacterSheet.tsx`). */
export type SheetBoxId =
  | 'portrait'
  | 'levelXp'
  | 'classPanel'
  | 'characterDetails'
  | 'showProgression'
  | 'squareBlock'
  | 'combatStrip'
  | 'armorClass'
  | 'attack'
  | 'speed'
  | 'saves'
  | 'abilities'
  | 'skills';

/** The sheet's regions, left to right. The tabs sit below `combatRow` and are not a box. */
export type SheetRegion = 'leftRail' | 'combatRow' | 'rightColumn';

/** A stack that boxes can sit in: a region, or the strip nested in `combatRow`. */
export type SheetStack = SheetRegion | 'combatStrip';

export const SHEET_REGIONS: readonly SheetRegion[] = ['leftRail', 'combatRow', 'rightColumn'];

const STACK: CSSProperties = { display: 'flex', flexDirection: 'column', gap: '0.6rem', minWidth: 0 };

/** The combat row's wrap: the 1x4 drops under the 2x2 when the row is narrower than both claim. */
export const COMBAT_ROW_WRAP: NonNullable<CSSProperties['flexWrap']> = 'wrap';

/** Style of each stack. `CharacterSheet.tsx` puts the rail's own width and rule on its wrapper, which collapses. */
export const SHEET_REGION_STYLES: Readonly<Record<SheetStack, CSSProperties>> = {
  leftRail: { ...STACK, gap: '0.5rem' },
  // One row: the 2x2 beside the 1x4. The two children's flex-basis is the breakpoint: the 1x4 wraps
  // under when the row is narrower than 360px + the gap + 320px. Retune it on `combatStrip`.
  combatRow: { alignItems: 'flex-start', display: 'flex', flexDirection: 'row', flexWrap: COMBAT_ROW_WRAP, gap: '0.6rem', minWidth: 0 },
  rightColumn: { ...STACK, flex: '0 0 300px' },
  // The 1x4. Holds four boxes of up to four tiles; grows to fill its line, alone or beside the 2x2.
  combatStrip: { ...STACK, flex: '1 1 320px' },
};

/** Abilities: six rows, STR to CHA, top to bottom. */
const ABILITY_ROWS: CSSProperties = { display: 'flex', flexDirection: 'column', gap: '0.3rem' };
/** A combat box: its tiles side by side. */
const TILE_ROW: CSSProperties = { display: 'flex', gap: '0.5rem' };
const RAIL_BOX: CSSProperties = { display: 'flex', flexDirection: 'column', gap: '0.5rem', minWidth: 0 };

/**
 * The 2x2 square block (Hit Points, Initiative, Encumbrance, Conditions) under the name: two equal
 * columns of square boxes. Rendered by Brief 2 L2 (sheet) and L5 (Create). Capped at 360px (boxes
 * about 170px) and left-aligned (L2b): at full column width the squares pushed the stat block below
 * the fold. Retune the cap here.
 */
export const SQUARE_BLOCK_STYLE: CSSProperties = {
  display: 'grid',
  gap: '0.6rem',
  gridTemplateColumns: '1fr 1fr',
  justifySelf: 'start',
  maxWidth: '360px',
  width: '100%',
};

export const SHEET_LAYOUT: readonly LayoutBox<SheetBoxId, SheetStack>[] = [
  { id: 'portrait', column: 'leftRail', order: 0, boxStyle: { marginBottom: '0.25rem' } },
  // Level and Caster Level, and the sheet's mutation and status messages under them.
  { id: 'levelXp', column: 'leftRail', order: 1, boxStyle: RAIL_BOX },
  // Race, any alternate racial traits, Class.
  { id: 'classPanel', column: 'leftRail', order: 2, boxStyle: RAIL_BOX },
  { id: 'characterDetails', column: 'leftRail', order: 3, boxStyle: { minWidth: 0 } },
  { id: 'showProgression', column: 'leftRail', order: 4, boxStyle: RAIL_BOX },
  { id: 'squareBlock', column: 'combatRow', order: 0, boxStyle: { flex: `0 1 ${SQUARE_BLOCK_STYLE.maxWidth}`, minWidth: 0 } },
  { id: 'combatStrip', column: 'combatRow', order: 1, boxStyle: SHEET_REGION_STYLES.combatStrip },
  { id: 'armorClass', column: 'combatStrip', order: 0, boxStyle: TILE_ROW },
  { id: 'attack', column: 'combatStrip', order: 1, boxStyle: TILE_ROW },
  { id: 'speed', column: 'combatStrip', order: 2, boxStyle: TILE_ROW },
  { id: 'saves', column: 'combatStrip', order: 3, boxStyle: TILE_ROW },
  { id: 'abilities', column: 'rightColumn', order: 0, boxStyle: ABILITY_ROWS },
  { id: 'skills', column: 'rightColumn', order: 1, boxStyle: { minWidth: 0 } },
];

export const SQUARE_BOX_STYLE: CSSProperties = { aspectRatio: '1' };

/** The four boxes of the 2x2 block, read left to right, top to bottom. */
export type SquareBoxId = 'hitPoints' | 'initiative' | 'encumbrance' | 'conditions';

export const SQUARE_BLOCK_LAYOUT: readonly LayoutBox<SquareBoxId, 'square'>[] = [
  { id: 'hitPoints', column: 'square', order: 0, boxStyle: SQUARE_BOX_STYLE },
  { id: 'initiative', column: 'square', order: 1, boxStyle: SQUARE_BOX_STYLE },
  { id: 'encumbrance', column: 'square', order: 2, boxStyle: SQUARE_BOX_STYLE },
  { id: 'conditions', column: 'square', order: 3, boxStyle: SQUARE_BOX_STYLE },
];

/** Create screen boxes (Brief 2 L3 renders Create from this). */
export type CreateBoxId = 'levels' | 'levelGains' | 'identity' | 'sheetBlock' | 'details' | 'abilities' | 'skills';
export type CreateColumn = 'left' | 'details' | 'abilities';

/** Create's columns, left to right: progression rail, identity and choices, Abilities + Skills. */
export const CREATE_COLUMNS: readonly CreateColumn[] = ['left', 'details', 'abilities'];

/** Style of each Create column (each a vertical stack; the widths the columns had before L3). */
export const CREATE_COLUMN_STYLES: Readonly<Record<CreateColumn, CSSProperties>> = {
  left: { ...STACK, flex: '2 1 380px' },
  details: { ...STACK, flex: '3 1 520px', gap: 0 },
  abilities: { ...STACK, flex: '1 1 360px', maxWidth: 560 },
};

export const CREATE_LAYOUT: readonly LayoutBox<CreateBoxId, CreateColumn>[] = [
  // Far-left: the level-add dialog and HP roller (`LevelsPanel`).
  { id: 'levels', column: 'left', order: 0, boxStyle: { display: 'flex', flexDirection: 'column', gap: '0.5rem' } },
  // Under the HP roller: the read-only per-level summary (Brief 2 L4).
  { id: 'levelGains', column: 'left', order: 1, boxStyle: { minWidth: 0 } },
  // Character and player name, race.
  { id: 'identity', column: 'details', order: 0, boxStyle: { minWidth: 0 } },
  // Under the name, as on the sheet: the 2x2 Hit Points / Initiative / Encumbrance / Conditions block (Brief 2 L5).
  { id: 'sheetBlock', column: 'details', order: 1, boxStyle: { marginBottom: '1rem', minWidth: 0 } },
  // Level, alignment, physical attributes and the Manage boxes (traits, feats, spells, equipment, custom).
  { id: 'details', column: 'details', order: 2, boxStyle: { minWidth: 0 } },
  { id: 'abilities', column: 'abilities', order: 0, boxStyle: ABILITY_ROWS },
  // Same column as Abilities: a Skills box listing known skills.
  { id: 'skills', column: 'abilities', order: 1, boxStyle: { display: 'flex', flexDirection: 'column', gap: '0.2rem' } },
];

/** The boxes of one column, in display order (by `order`, never by array position). */
export function boxesInColumn<Id extends string, Column extends string>(
  layout: readonly LayoutBox<Id, Column>[],
  column: Column,
): LayoutBox<Id, Column>[] {
  return layout.filter((box) => box.column === column).sort((a, b) => a.order - b.order);
}

/** One box's entry; throws on an id the registry does not hold (a typo is a bug, not a blank). */
export function layoutBox<Id extends string, Column extends string>(
  layout: readonly LayoutBox<Id, Column>[],
  id: Id,
): LayoutBox<Id, Column> {
  const box = layout.find((entry) => entry.id === id);
  if (!box) {
    throw new Error(`layout registry has no box "${id}"`);
  }
  return box;
}

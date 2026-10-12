import type { CSSProperties } from 'react';

/**
 * Declarative box placement for the character sheet and the Create screen (Brief 2 L0).
 *
 * Each screen renders its boxes from one of these arrays: `column` says which stack a box lives
 * in, `order` its position in that stack, `boxStyle` the style its own content takes. Moving a
 * box is an edit to its entry here, not a JSX move. No drag-and-drop (operator: "easy in code").
 *
 * Only the stat block of the sheet's middle column is registered so far (Brief 2 L1). The left
 * progression rail and the right Details/Skills column are still laid out in
 * `CharacterSheet.tsx` directly.
 */
export interface LayoutBox<Id extends string, Column extends string> {
  id: Id;
  column: Column;
  order: number;
  boxStyle: CSSProperties;
}

/** Sheet boxes rendered from `SHEET_LAYOUT` (see `CharacterSheet.tsx`, middle column). */
export type SheetBoxId = 'abilities' | 'armorClass' | 'attack' | 'speed' | 'saves';

/** The sheet's middle stat block: Abilities as one column, the combat boxes stacked beside it. */
export type SheetColumn = 'abilities' | 'combat';

/** The registered columns, left to right. */
export const SHEET_COLUMNS: readonly SheetColumn[] = ['abilities', 'combat'];

const STACK: CSSProperties = { display: 'flex', flexDirection: 'column', gap: '0.6rem', minWidth: 0 };

/** Style of each registered sheet column (a vertical stack). */
export const SHEET_COLUMN_STYLES: Readonly<Record<SheetColumn, CSSProperties>> = {
  abilities: { ...STACK, flex: '0 0 auto' },
  combat: { ...STACK, flex: 1 },
};

/** Abilities: six rows, STR to CHA, top to bottom. */
const ABILITY_ROWS: CSSProperties = { display: 'flex', flexDirection: 'column', gap: '0.3rem' };
/** A combat box: its tiles side by side. */
const TILE_ROW: CSSProperties = { display: 'flex', gap: '0.5rem' };

export const SHEET_LAYOUT: readonly LayoutBox<SheetBoxId, SheetColumn>[] = [
  { id: 'abilities', column: 'abilities', order: 0, boxStyle: ABILITY_ROWS },
  { id: 'armorClass', column: 'combat', order: 0, boxStyle: TILE_ROW },
  { id: 'attack', column: 'combat', order: 1, boxStyle: TILE_ROW },
  { id: 'speed', column: 'combat', order: 2, boxStyle: TILE_ROW },
  { id: 'saves', column: 'combat', order: 3, boxStyle: TILE_ROW },
];

/**
 * The 2x2 square block (Hit Points, Initiative, Encumbrance, Conditions) under the name: two equal
 * columns of square boxes. Rendered by Brief 2 L2 (sheet) and L5 (Create).
 */
export const SQUARE_BLOCK_STYLE: CSSProperties = { display: 'grid', gap: '0.6rem', gridTemplateColumns: '1fr 1fr' };
export const SQUARE_BOX_STYLE: CSSProperties = { aspectRatio: '1' };

/** Create screen boxes (Brief 2 L3 renders Create from this). */
export type CreateBoxId = 'levels' | 'abilities' | 'skills';
export type CreateColumn = 'left' | 'abilities';

export const CREATE_LAYOUT: readonly LayoutBox<CreateBoxId, CreateColumn>[] = [
  // Far-left: the level-add dialog and HP roller (`LevelsPanel`).
  { id: 'levels', column: 'left', order: 0, boxStyle: { display: 'flex', flexDirection: 'column', gap: '0.5rem' } },
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

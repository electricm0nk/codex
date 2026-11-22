import type { CSSProperties } from 'react';
import type { LayoutBox } from './sheetLayout';

/**
 * Declarative box placement for the Cyberpunk RED Create screen (`CyberpunkCreateForm.tsx`),
 * the registry shape of `sheetLayout.ts`: `column` says which stack a box lives in, `order` its
 * position there, `boxStyle` the style its own content takes. Moving a box is an edit to its
 * entry here, not a JSX move. The Lifepath, skills, Role Ability, kit and cyberware steps add
 * their boxes here as they land.
 */

/** The Create screen's boxes. */
export type CyberpunkCreateBoxId = 'identity' | 'method' | 'stats' | 'derived';

/** Left: who the character is and how the STATs are made. Right: the STATs and what they give. */
export type CyberpunkCreateColumn = 'choices' | 'statBlock';

/** The registered columns, left to right. */
export const CYBERPUNK_CREATE_COLUMNS: readonly CyberpunkCreateColumn[] = ['choices', 'statBlock'];

const STACK: CSSProperties = { display: 'flex', flexDirection: 'column', gap: '0.75rem', minWidth: 0 };

/** Style of each column (a vertical stack). */
export const CYBERPUNK_CREATE_COLUMN_STYLES: Readonly<Record<CyberpunkCreateColumn, CSSProperties>> = {
  choices: { ...STACK, flex: '1 1 320px' },
  statBlock: { ...STACK, flex: '2 1 420px' },
};

const TILE_GRID: CSSProperties = {
  display: 'grid',
  gap: '0.5rem',
  gridTemplateColumns: 'repeat(auto-fit, minmax(96px, 1fr))',
};

export const CYBERPUNK_CREATE_LAYOUT: readonly LayoutBox<CyberpunkCreateBoxId, CyberpunkCreateColumn>[] = [
  // Character name and Role.
  { id: 'identity', column: 'choices', order: 0, boxStyle: { minWidth: 0 } },
  // Streetrat / Edgerunner / Complete Package, and that method's own controls.
  { id: 'method', column: 'choices', order: 1, boxStyle: { display: 'flex', flexDirection: 'column', gap: '0.5rem' } },
  // The ten STATs, INT to EMP.
  { id: 'stats', column: 'statBlock', order: 0, boxStyle: TILE_GRID },
  // Hit Points, Seriously Wounded, Death Save, Humanity, EMP: the engine's derived rows.
  { id: 'derived', column: 'statBlock', order: 1, boxStyle: TILE_GRID },
];

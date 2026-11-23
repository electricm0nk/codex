import type { CSSProperties, ReactNode } from 'react';

/**
 * The character sheet's fixed frame (SD-43 items 17 + 21, decisions.md §24): the window is the sheet.
 * Row 1 is the bar (menu, close, name); row 2 is three columns. The page never scrolls; the centre is
 * the main scroller and each side column scrolls inside itself when taller than the window (Q-17.1).
 * Sized from the viewport (`100dvh`), never from an assumed pixel height, so a window a dock bar
 * shortens still shows every column's last control (Show Progression) above the edge.
 *
 * The PF sheet renders inside it (E2.1); the Starfinder and Cyberpunk RED sheets render inside it too (E2.2).
 * A sheet with one or two columns omits `left` / `right`: the frame then draws only the columns given.
 * Named SheetFrame, not SheetLayout: `sheetLayout.ts` (the box registry) already exists beside it and
 * the two names would differ only by case.
 */
/** Space under each column's last control. A sticky last box offsets by it to reach the window edge. */
export const SHEET_COLUMN_BOTTOM_PADDING = '1.5rem';

const COLUMN: CSSProperties = { display: 'flex', flexDirection: 'column', minHeight: 0, overflowY: 'auto', paddingBottom: SHEET_COLUMN_BOTTOM_PADDING };

export const SHEET_FRAME_STYLES: Readonly<Record<'root' | 'bar' | 'columns' | 'left' | 'centre' | 'right', CSSProperties>> = {
  // Pinned to the window: nothing is left in the page flow to scroll, and the old
  // `marginLeft: calc(50% - 50vw); marginTop: -3rem; width: 100vw` escape from the shell padding goes.
  root: {
    backgroundColor: 'var(--color-page)',
    display: 'grid',
    gridTemplateRows: 'auto minmax(0, 1fr)',
    height: '100dvh',
    inset: 0,
    overflow: 'hidden',
    position: 'fixed',
  },
  // No z-index: dialogs and the progression drawer (position fixed, z-index >= 60) stay above the bar.
  bar: {
    alignItems: 'center',
    backgroundColor: 'var(--color-page)',
    borderBottom: '2px solid var(--color-accent)',
    display: 'flex',
    gap: '1.5rem',
    padding: '0.95rem 4rem 0.95rem 1.5rem',
  },
  columns: { display: 'flex', gap: '0.75rem', minHeight: 0, overflow: 'hidden', padding: '1rem 1.5rem 0' },
  left: { ...COLUMN, borderRight: '1px solid var(--color-border)', paddingRight: '0.75rem' },
  centre: { ...COLUMN, flex: 1, gap: '0.6rem', minWidth: 0 },
  right: { ...COLUMN, gap: '0.6rem' },
};

export interface SheetFrameProps {
  bar: ReactNode;
  left?: ReactNode;
  /** The left column's width (a flex-basis), e.g. a collapsed rail's narrower one. Needed with `left`. */
  leftWidth?: string;
  centre: ReactNode;
  right?: ReactNode;
  rightWidth?: string;
  /** Becomes the root's `data-testid`. */
  testId?: string;
  /** Dialogs and drawers: fixed overlays, drawn above the bar. */
  children?: ReactNode;
}

export function SheetFrame(props: SheetFrameProps) {
  return (
    <div data-testid={props.testId} style={SHEET_FRAME_STYLES.root}>
      <div style={SHEET_FRAME_STYLES.bar}>{props.bar}</div>
      <div className="sheet-cols" style={SHEET_FRAME_STYLES.columns}>
        {props.left === undefined ? null : (
          <div className="sheet-left" style={{ ...SHEET_FRAME_STYLES.left, flex: `0 0 ${props.leftWidth ?? 'auto'}` }}>
            {props.left}
          </div>
        )}
        <div className="sheet-mid" style={SHEET_FRAME_STYLES.centre}>
          {props.centre}
        </div>
        {props.right === undefined ? null : (
          <div className="sheet-right" style={{ ...SHEET_FRAME_STYLES.right, flex: `0 0 ${props.rightWidth ?? 'auto'}` }}>
            {props.right}
          </div>
        )}
      </div>
      {props.children}
    </div>
  );
}

import { useEffect, useRef, type ReactNode, type RefObject } from 'react';
import { createPortal } from 'react-dom';

/**
 * Show Progression's drawer (operator R7): a modal panel on the left of the window, closed by its X,
 * a click outside it, or Escape. The sheet hands it the content and owns the state, because focus goes
 * back to the sheet's own button when the drawer closes.
 */
export interface ProgressionDrawerState {
  open: boolean;
  /** Set by a close, so the sheet puts focus back on the button that opened the drawer. */
  returnFocus: boolean;
}

export type ProgressionDrawerAction = { type: 'open' } | { type: 'close'; via: 'x' | 'click-outside' | 'escape' };

export const PROGRESSION_DRAWER_CLOSED: ProgressionDrawerState = { open: false, returnFocus: false };

export function progressionDrawerReducer(state: ProgressionDrawerState, action: ProgressionDrawerAction): ProgressionDrawerState {
  if (action.type === 'open') {
    return state.open ? state : { open: true, returnFocus: false };
  }
  return state.open ? { open: false, returnFocus: true } : state;
}

export function progressionDrawerActionForKey(key: string): ProgressionDrawerAction | null {
  return key === 'Escape' ? { type: 'close', via: 'escape' } : null;
}

/** What Tab can reach inside the drawer. */
const FOCUSABLE = 'a[href], button, input, select, textarea, [tabindex]:not([tabindex="-1"])';

/** `aria-modal` promises the page behind is out of reach, so Tab wraps inside the drawer. */
function keepTabInside(event: KeyboardEvent, dialog: HTMLElement | null) {
  const focusable = dialog ? Array.from(dialog.querySelectorAll<HTMLElement>(FOCUSABLE)) : [];
  if (focusable.length === 0) {
    return;
  }
  const first = focusable[0];
  const last = focusable[focusable.length - 1];
  const active = document.activeElement;
  if (!dialog?.contains(active) || (event.shiftKey ? active === first : active === last)) {
    event.preventDefault();
    (event.shiftKey ? last : first).focus();
  }
}

/** The drawer's markup and handlers, without the portal or the window listener. */
export function ProgressionDrawerPanel(props: {
  onAction: (action: ProgressionDrawerAction) => void;
  dialogRef: RefObject<HTMLDivElement>;
  children: ReactNode;
}) {
  return (
    <div
      role="presentation"
      onClick={() => props.onAction({ type: 'close', via: 'click-outside' })}
      style={{ backgroundColor: 'rgba(0,0,0,0.6)', display: 'flex', inset: 0, justifyContent: 'flex-start', position: 'fixed', zIndex: 60 }}
    >
      <div
        ref={props.dialogRef}
        role="dialog"
        aria-modal="true"
        aria-label="Progression"
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          borderRight: '1px solid var(--color-border)',
          boxSizing: 'border-box',
          color: 'var(--color-text)',
          height: '100%',
          maxWidth: '90vw',
          overflowY: 'auto',
          padding: '1rem',
          width: '420px',
        }}
      >
        <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '0.75rem' }}>
          <h2 style={{ fontSize: '1.1rem', margin: 0 }}>Progression</h2>
          <button
            type="button"
            aria-label="Close"
            autoFocus
            onClick={() => props.onAction({ type: 'close', via: 'x' })}
            style={{ background: 'none', border: 'none', color: 'var(--color-text-secondary)', cursor: 'pointer', fontSize: '1.2rem', lineHeight: 1 }}
          >
            ×
          </button>
        </div>
        {props.children}
      </div>
    </div>
  );
}

export function ProgressionDrawer(props: { onAction: (action: ProgressionDrawerAction) => void; children: ReactNode }) {
  const { onAction } = props;
  const dialogRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      const action = progressionDrawerActionForKey(event.key);
      if (action) {
        onAction(action);
      } else if (event.key === 'Tab') {
        keepTabInside(event, dialogRef.current);
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, [onAction]);
  return createPortal(
    <ProgressionDrawerPanel onAction={onAction} dialogRef={dialogRef}>
      {props.children}
    </ProgressionDrawerPanel>,
    document.body,
  );
}

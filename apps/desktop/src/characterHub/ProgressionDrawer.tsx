import { useEffect, useRef, useState, type ReactNode, type RefObject } from 'react';
import { createPortal } from 'react-dom';
import { listProgressionLedger, type ProgressionLedgerDto } from '../boundary/listProgressionLedger';
import { describeProgressionChoices, describeProgressionLevel, type HeldClass } from './characterProgression';

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

export type ProgressionLedgerState =
  | { kind: 'loading' }
  | { kind: 'loaded'; ledger: ProgressionLedgerDto }
  | { kind: 'failed'; reason: string };

const LEDGER_CARD = { border: '1px solid var(--color-border)', borderRadius: 6, marginBottom: '0.5rem', padding: '0.55rem 0.7rem' };
const LEDGER_LINE = { color: 'var(--color-text-secondary)', fontSize: '0.8rem', margin: '0.15rem 0 0' };

/** What `list_progression_ledger` said, or that it has not, or why it could not. */
export function ProgressionLedgerView(props: { state: ProgressionLedgerState; heldClasses: readonly HeldClass[] }) {
  const { state } = props;
  if (state.kind === 'loading') {
    return <p style={LEDGER_LINE}>Reading the level history...</p>;
  }
  if (state.kind === 'failed') {
    return (
      <p role="alert" style={{ ...LEDGER_LINE, color: 'var(--color-error)' }}>
        The level history could not be read: {state.reason}
      </p>
    );
  }
  const apart = describeProgressionChoices(state.ledger.withoutLevel);
  return (
    <div>
      {state.ledger.levels.map((level) => {
        const entry = describeProgressionLevel(level, props.heldClasses);
        return (
          <div key={level.characterLevel} style={LEDGER_CARD}>
            <p style={{ color: 'var(--color-accent)', fontSize: '0.78rem', fontWeight: 800, letterSpacing: '0.02em', margin: 0 }}>
              {entry.heading} · {entry.classLevel}
            </p>
            {level.assignedOnUpgrade ? <p style={{ ...LEDGER_LINE, color: 'var(--color-text-muted)', fontStyle: 'italic' }}>Assigned on upgrade</p> : null}
            {entry.lines.map((line) => (
              <p key={line} style={LEDGER_LINE}>
                {line}
              </p>
            ))}
          </div>
        );
      })}
      {apart.length > 0 ? (
        <div style={LEDGER_CARD}>
          <p style={{ color: 'var(--color-accent)', fontSize: '0.78rem', fontWeight: 800, letterSpacing: '0.02em', margin: 0 }}>Not placed at a level</p>
          {apart.map((line) => (
            <p key={line} style={LEDGER_LINE}>
              {line}
            </p>
          ))}
        </div>
      ) : null}
    </div>
  );
}

/** Reads the ledger when the drawer opens: the drawer is modal, so nothing changes while it is open. */
export function ProgressionLedger(props: { characterId: string; heldClasses: readonly HeldClass[] }) {
  const [state, setState] = useState<ProgressionLedgerState>({ kind: 'loading' });
  useEffect(() => {
    let cancelled = false;
    listProgressionLedger(props.characterId)
      .then((ledger) => {
        if (!cancelled) {
          setState({ kind: 'loaded', ledger });
        }
      })
      .catch((cause: unknown) => {
        if (!cancelled) {
          setState({ kind: 'failed', reason: cause instanceof Error ? cause.message : String(cause) });
        }
      });
    return () => {
      cancelled = true;
    };
  }, [props.characterId]);
  return <ProgressionLedgerView state={state} heldClasses={props.heldClasses} />;
}

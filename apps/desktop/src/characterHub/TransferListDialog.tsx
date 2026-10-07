import { useEffect, useState } from 'react';
import { createPortal } from 'react-dom';
import {
  availableItems,
  canMoveRight,
  describeFocus,
  innateItems,
  moveLeft,
  moveRight,
  remainingByGroup,
  remainingSelections,
  selectedItems,
  type TransferItem,
} from './transferListModel';

const listStyle = {
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  flex: 1,
  minHeight: 0,
  minWidth: 0,
  overflowY: 'auto',
} as const;

const arrowStyle = {
  background: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '1.1rem',
  fontWeight: 700,
  padding: '0.4rem 0.8rem',
} as const;

function Column(props: {
  title: string;
  items: TransferItem[];
  focusedId: string | null;
  onFocus: (id: string) => void;
  onActivate?: (id: string) => void;
  muted?: boolean;
}) {
  return (
    <div style={{ display: 'flex', flex: 1, flexDirection: 'column', minHeight: 0, minWidth: 0 }}>
      <p style={{ fontSize: '0.8rem', fontWeight: 700, margin: '0 0 0.3rem' }}>
        {props.title} <span style={{ color: 'var(--color-text-muted)', fontWeight: 500 }}>({props.items.length})</span>
      </p>
      <div role="listbox" aria-label={props.title} style={listStyle}>
        {props.items.map((item) => {
          const focused = props.focusedId === item.id;
          return (
            <button
              key={item.id}
              type="button"
              role="option"
              aria-selected={focused}
              onClick={() => props.onFocus(item.id)}
              onDoubleClick={() => props.onActivate?.(item.id)}
              style={{
                background: focused ? 'var(--color-surface-2)' : 'none',
                border: 'none',
                borderBottom: '1px solid var(--color-border)',
                borderLeft: focused ? '3px solid var(--color-accent)' : '3px solid transparent',
                color: props.muted || !item.qualified ? 'var(--color-text-muted)' : 'var(--color-text)',
                cursor: 'pointer',
                display: 'block',
                fontSize: '0.88rem',
                padding: '0.4rem 0.6rem',
                textAlign: 'left',
                textDecoration: item.qualified ? 'none' : 'line-through',
                width: '100%',
              }}
            >
              {item.label}
            </button>
          );
        })}
      </div>
    </div>
  );
}

/**
 * The dialog's contents, controlled by its parent so it can be rendered and tested without a DOM
 * portal: options on the left, `<` `>` between, selected on the right, an optional Innate column,
 * a Qualified filter, the remaining count on top and the clicked item's description below.
 */
export function TransferListBody(props: {
  items: TransferItem[];
  selected: string[];
  focusedId: string | null;
  qualifiedOnly: boolean;
  limit: number | null;
  /** Per-group quotas (spell levels): each group fills separately and shows its own remaining count. */
  groupLimits?: Record<string, number>;
  /** Display names for the group keys in the remaining line. */
  groupLabels?: Record<string, string>;
  /** What is being chosen, for the remaining count ("feats", "racial traits"). */
  remainingNoun: string;
  onFocus: (id: string) => void;
  onMoveRight: (id: string) => void;
  onMoveLeft: (id: string) => void;
  onQualifiedChange: (qualifiedOnly: boolean) => void;
}) {
  const available = availableItems(props.items, props.selected, props.qualifiedOnly);
  const chosen = selectedItems(props.items, props.selected);
  const innate = innateItems(props.items);
  const remaining = remainingSelections(props.limit, props.selected);
  const focusedIsSelected = props.focusedId !== null && props.selected.includes(props.focusedId);
  const rightMove = props.focusedId === null ? { ok: false } : canMoveRight(props.items, props.selected, props.focusedId, props.limit, props.groupLimits);
  const focus = describeFocus(props.items, props.focusedId);

  return (
    <div style={{ display: 'flex', flex: 1, flexDirection: 'column', gap: '0.75rem', minHeight: 0 }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between' }}>
        {props.groupLimits !== undefined ? (
          <p role="status" style={{ fontSize: '0.95rem', fontWeight: 700, margin: 0 }}>
            {remainingByGroup(props.items, props.selected, props.groupLimits)
              .map((group) => `${props.groupLabels?.[group.key] ?? group.key}: ${group.remaining} of ${group.limit}`)
              .join(' · ') || `No ${props.remainingNoun} to choose`}{' '}
            remaining
          </p>
        ) : props.limit !== null && remaining !== null ? (
          <p role="status" style={{ fontSize: '0.95rem', fontWeight: 700, margin: 0 }}>
            {remaining} of {props.limit} {props.remainingNoun} remaining
          </p>
        ) : (
          <span />
        )}
        <label style={{ alignItems: 'center', cursor: 'pointer', display: 'flex', fontSize: '0.85rem', gap: '0.4rem' }}>
          <input type="checkbox" checked={props.qualifiedOnly} onChange={(event) => props.onQualifiedChange(event.target.checked)} />
          Qualified
        </label>
      </div>

      <div style={{ display: 'flex', flex: 1, gap: '0.6rem', minHeight: 0 }}>
        <Column title="Options" items={available} focusedId={props.focusedId} onFocus={props.onFocus} onActivate={props.onMoveRight} />
        <div style={{ alignItems: 'center', display: 'flex', flexDirection: 'column', gap: '0.5rem', justifyContent: 'center' }}>
          <button
            type="button"
            aria-label="Move to selected"
            title={rightMove.ok ? 'Select the highlighted option' : (rightMove as { reason?: string }).reason ?? 'Click an option first'}
            disabled={!rightMove.ok}
            onClick={() => props.focusedId !== null && props.onMoveRight(props.focusedId)}
            style={{ ...arrowStyle, opacity: rightMove.ok ? 1 : 0.4 }}
          >
            &gt;
          </button>
          <button
            type="button"
            aria-label="Move to options"
            title="Remove the highlighted selection"
            disabled={!focusedIsSelected}
            onClick={() => props.focusedId !== null && props.onMoveLeft(props.focusedId)}
            style={{ ...arrowStyle, opacity: focusedIsSelected ? 1 : 0.4 }}
          >
            &lt;
          </button>
        </div>
        <Column title="Selected" items={chosen} focusedId={props.focusedId} onFocus={props.onFocus} onActivate={props.onMoveLeft} />
        {innate.length > 0 ? (
          <Column title="Innate" items={innate} focusedId={props.focusedId} onFocus={props.onFocus} muted />
        ) : null}
      </div>

      <div style={{ border: '1px solid var(--color-border)', borderRadius: 8, maxHeight: '32%', minHeight: 110, overflowY: 'auto', padding: '0.7rem 0.9rem' }}>
        {focus === null ? (
          <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>Click an item to read its full description.</p>
        ) : (
          <>
            <p style={{ fontWeight: 700, margin: '0 0 0.3rem' }}>{focus.title}</p>
            {focus.note ? <p style={{ color: 'var(--color-warn)', fontSize: '0.82rem', margin: '0 0 0.4rem' }}>{focus.note}</p> : null}
            <p style={{ fontSize: '0.88rem', margin: 0, whiteSpace: 'pre-wrap' }}>{focus.body}</p>
          </>
        )}
      </div>
    </div>
  );
}

/**
 * A modal "Manage" dialog over a list of options. Qualified is on by default; items the character
 * may not take are hidden then, and listed (struck through, not selectable) when it is off.
 * `limit` is how many may be chosen (`null` for no cap).
 */
export function TransferListDialog(props: {
  open: boolean;
  title: string;
  items: TransferItem[];
  /** The selection is the parent's: it changes live as items move, and the parent restores it on Cancel. */
  selected: string[];
  onSelectedChange: (selected: string[]) => void;
  limit: number | null;
  groupLimits?: Record<string, number>;
  groupLabels?: Record<string, string>;
  remainingNoun: string;
  /** A line under the title: what the list covers or what is not modelled. */
  notice?: string;
  onAccept: () => void;
  onCancel: () => void;
}) {
  const [focusedId, setFocusedId] = useState<string | null>(null);
  const [qualifiedOnly, setQualifiedOnly] = useState(true);

  useEffect(() => {
    if (!props.open) {
      return undefined;
    }
    setFocusedId(null);
    setQualifiedOnly(true);
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        props.onCancel();
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [props.open]);

  if (!props.open) {
    return null;
  }

  return createPortal(
    <div
      role="presentation"
      onClick={props.onCancel}
      style={{ alignItems: 'center', backgroundColor: 'rgba(0, 0, 0, 0.6)', display: 'flex', inset: 0, justifyContent: 'center', padding: '2rem', position: 'fixed', zIndex: 1100 }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label={props.title}
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 12,
          boxShadow: '0 24px 60px rgba(0, 0, 0, 0.55)',
          display: 'flex',
          flexDirection: 'column',
          height: 'min(820px, 92vh)',
          padding: '1rem 1.25rem',
          width: 'min(1100px, 96vw)',
        }}
      >
        <h2 style={{ fontSize: '1.15rem', margin: '0 0 0.2rem' }}>{props.title}</h2>
        {props.notice ? <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0 0 0.6rem' }}>{props.notice}</p> : null}
        <TransferListBody
          items={props.items}
          selected={props.selected}
          focusedId={focusedId}
          qualifiedOnly={qualifiedOnly}
          limit={props.limit}
          groupLimits={props.groupLimits}
          groupLabels={props.groupLabels}
          remainingNoun={props.remainingNoun}
          onFocus={setFocusedId}
          onMoveRight={(id) => props.onSelectedChange(moveRight(props.items, props.selected, id, props.limit, props.groupLimits))}
          onMoveLeft={(id) => props.onSelectedChange(moveLeft(props.selected, id))}
          onQualifiedChange={setQualifiedOnly}
        />
        <div style={{ display: 'flex', gap: '0.6rem', justifyContent: 'flex-end', marginTop: '0.8rem' }}>
          <button type="button" onClick={props.onCancel} style={{ ...arrowStyle, fontSize: '0.85rem', fontWeight: 600 }}>
            Cancel
          </button>
          <button
            type="button"
            onClick={props.onAccept}
            style={{ ...arrowStyle, backgroundColor: 'var(--color-accent)', color: 'var(--color-on-accent)', fontSize: '0.85rem', fontWeight: 600 }}
          >
            Accept
          </button>
        </div>
      </div>
    </div>,
    document.body
  );
}

import { createContext, useContext, useLayoutEffect, useState, type CSSProperties } from 'react';

/** One ☰ menu entry. */
export interface AppMenuItem {
  label: string;
  onSelect: () => void;
  dividerBefore?: boolean;
}

/**
 * What App gives every ☰ menu: `openOptions` opens the Settings modal; `claimMenu` is called by a
 * screen that draws its own ☰ menu in its top bar, so App hides its own (the release is returned).
 */
export interface AppMenuHost {
  openOptions: () => void;
  claimMenu: () => () => void;
}

export const AppMenuHostContext = createContext<AppMenuHost | null>(null);

/** A screen's entries with Options last (decisions.md §24.10). */
export function withOptions(items: ReadonlyArray<AppMenuItem>, openOptions: () => void): AppMenuItem[] {
  return [...items, { label: 'Options', onSelect: openOptions, dividerBefore: items.length > 0 }];
}

const DROPDOWN_STYLE: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
  boxShadow: '0 8px 24px rgba(0, 0, 0, 0.45)',
  minWidth: 160,
  overflow: 'hidden',
  position: 'absolute',
  top: 'calc(100% + 10px)',
  zIndex: 21,
};

/** The open dropdown's entries; `onChosen` closes the menu before the entry runs. */
export function AppMenuList(props: { items: ReadonlyArray<AppMenuItem>; onChosen: () => void }) {
  return (
    <>
      {props.items.map((item) => (
        <div key={item.label}>
          {item.dividerBefore ? <div style={{ borderTop: '1px solid var(--color-border)', margin: '0.25rem 0' }} /> : null}
          <button
            type="button"
            onClick={() => {
              props.onChosen();
              item.onSelect();
            }}
            style={{
              background: 'none',
              border: 'none',
              color: 'var(--color-text)',
              cursor: 'pointer',
              display: 'block',
              fontSize: '0.9rem',
              padding: '0.55rem 1rem',
              textAlign: 'left',
              width: '100%',
            }}
          >
            {item.label}
          </button>
        </div>
      ))}
    </>
  );
}

/**
 * The ☰ menu. In a screen's top bar it lists the screen's entries plus Options and claims the bar;
 * `fixed` is App's own copy, top-right, shown on every screen without one.
 */
export function AppMenu(props: { items: ReadonlyArray<AppMenuItem>; fixed?: boolean }) {
  const host = useContext(AppMenuHostContext);
  if (host === null) {
    throw new Error('AppMenu needs an AppMenuHostContext provider (App.tsx)');
  }
  const [open, setOpen] = useState(false);
  const { claimMenu } = host;
  useLayoutEffect(() => (props.fixed ? undefined : claimMenu()), [claimMenu, props.fixed]);
  const wrapper: CSSProperties = props.fixed ? { position: 'fixed', right: 18, top: 18, zIndex: 950 } : { position: 'relative' };
  return (
    <div style={wrapper}>
      <button
        type="button"
        onClick={() => setOpen((current) => !current)}
        style={{ background: 'none', border: 'none', color: 'var(--color-text)', cursor: 'pointer', fontSize: '1.1rem', fontWeight: 700, padding: 0 }}
      >
        ☰ Menu
      </button>
      {open ? (
        <>
          <div onClick={() => setOpen(false)} style={{ inset: 0, position: 'fixed', zIndex: 20 }} />
          <div style={{ ...DROPDOWN_STYLE, ...(props.fixed ? { right: 0 } : { left: 0 }) }}>
            <AppMenuList items={withOptions(props.items, host.openOptions)} onChosen={() => setOpen(false)} />
          </div>
        </>
      ) : null}
    </div>
  );
}

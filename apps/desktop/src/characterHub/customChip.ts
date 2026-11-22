import type { CSSProperties } from 'react';

/**
 * Style of one top-row chip in the Custom dialog. A pure function of (selected, focused) that sets the
 * same properties in every state, so deselecting restores the resting style exactly and no longhand
 * ever competes with a shorthand. Selection is a fill; keyboard focus is an outline, so the two read
 * apart. Colours are theme tokens only.
 */
export function chipStyle(state: { selected: boolean; focused: boolean }): CSSProperties {
  return {
    backgroundColor: state.selected ? 'var(--color-accent)' : 'var(--color-surface-2)',
    border: '1px solid var(--color-border)',
    borderRadius: 6,
    color: state.selected ? 'var(--color-on-accent)' : 'var(--color-text)',
    cursor: 'pointer',
    fontSize: '0.8rem',
    fontWeight: 600,
    outline: state.focused ? '2px solid var(--color-text)' : 'none',
    outlineOffset: 2,
    padding: '0.3rem 0.8rem',
  };
}

/** True for the one chip whose key is the current section. */
export function selectedChipKey(current: string, key: string): boolean {
  return current === key;
}

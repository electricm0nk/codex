import type { CSSProperties } from 'react';

/** Was 0.5rem/1.5rem padding at 1rem text (~2.2rem tall); now 2x: 1rem/3rem padding at 2rem text (~4.4rem tall). */
const marketButtonStyle: CSSProperties = {
  backgroundColor: 'var(--color-accent)',
  border: '1px solid var(--color-accent)',
  borderRadius: 8,
  color: 'var(--color-bg, #000)',
  cursor: 'pointer',
  fontSize: '2rem',
  fontWeight: 700,
  padding: '1rem 3rem',
};

/** The one Market entry point on every sheet panel, top-left. */
export function MarketButton(props: { onClick: () => void; testId?: string; disabled?: boolean }) {
  return (
    <div style={{ display: 'flex', justifyContent: 'flex-start', marginBottom: '1.25rem' }}>
      <button type="button" data-testid={props.testId} disabled={props.disabled} onClick={props.onClick} style={marketButtonStyle}>
        Market
      </button>
    </div>
  );
}

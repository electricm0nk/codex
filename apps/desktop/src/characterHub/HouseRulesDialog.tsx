import type { CSSProperties } from 'react';
import { customOverrides, type CharacterCustom, type CustomOverrides } from './customModel';
import { houseRulesFor } from './houseRulesModel';

/**
 * The ☰ House Rules dialog: this character's house rules for its game system. Each checkbox edits the
 * same `Overrides` the engine reads (NR-4), saved at once by `onChange`.
 */
export function HouseRulesDialog(props: {
  gameSystem: string;
  custom: CharacterCustom;
  error: string | null;
  onChange: (key: keyof CustomOverrides, on: boolean) => void;
  onClose: () => void;
}) {
  const rules = houseRulesFor(props.gameSystem);
  const overrides = customOverrides(props.custom);
  return (
    <div role="presentation" onClick={props.onClose} style={{ alignItems: 'center', backgroundColor: 'rgba(0,0,0,0.6)', display: 'flex', inset: 0, justifyContent: 'center', position: 'fixed', zIndex: 60 }}>
      <div role="dialog" aria-modal="true" aria-label="House Rules" onClick={(event) => event.stopPropagation()} style={dialogStyle}>
        <h2 style={{ fontSize: '1.1rem', margin: 0 }}>House Rules</h2>
        <p style={mutedStyle}>For this character only. A new character starts with every rule off.</p>
        {rules.length === 0 ? <p style={{ margin: 0 }}>No house rules for this game system.</p> : null}
        {rules.map((rule) =>
          rule.flag !== null ? (
            <label key={rule.label} style={{ alignItems: 'center', display: 'flex', gap: '0.4rem' }}>
              <input type="checkbox" aria-label={rule.label} checked={overrides[rule.flag]} onChange={(event) => props.onChange(rule.flag as keyof CustomOverrides, event.target.checked)} />
              {rule.label}
            </label>
          ) : (
            <div key={rule.label}>
              <span style={{ color: 'var(--color-text-muted)' }}>{rule.label}</span>
              <p role="note" style={mutedStyle}>{rule.notice}</p>
            </div>
          )
        )}
        {props.error !== null ? <p role="alert" style={{ color: 'var(--color-error)', fontSize: '0.85rem', margin: 0 }}>{props.error}</p> : null}
        <div style={{ display: 'flex', justifyContent: 'flex-end' }}>
          <button type="button" onClick={props.onClose} style={buttonStyle}>Close</button>
        </div>
      </div>
    </div>
  );
}

const dialogStyle: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 12,
  display: 'flex',
  flexDirection: 'column',
  gap: '0.6rem',
  padding: '1.1rem 1.4rem',
  width: 'min(480px, 95vw)',
};
const buttonStyle: CSSProperties = {
  background: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.8rem',
  fontWeight: 600,
  padding: '0.3rem 0.8rem',
};
const mutedStyle: CSSProperties = { color: 'var(--color-text-muted)', fontSize: '0.82rem', margin: 0 };

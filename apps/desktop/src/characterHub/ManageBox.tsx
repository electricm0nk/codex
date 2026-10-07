import type { ReactNode } from 'react';

/**
 * A summary box with a Manage button, the Create screen's way to offer a group of options (racial
 * traits, traits, feats, skills, spells, equipment): the box says what is chosen and what is left,
 * and Manage opens the dialog where the choosing happens.
 */
export function ManageBox(props: {
  title: string;
  /** One short line per chosen item. */
  summary: string[];
  /** What is left to pick ("1 of 3 feats remaining"), shown beside the title. */
  remaining?: string;
  /** Extra content under the summary (a warning, a per-item control). */
  children?: ReactNode;
  /** When set, Manage is disabled and this says why (loading, unavailable). */
  disabledReason?: string;
  onManage: () => void;
}) {
  return (
    <section
      aria-label={props.title}
      style={{ backgroundColor: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 10, marginBottom: '0.75rem', padding: '0.7rem 0.9rem' }}
    >
      <div style={{ alignItems: 'center', display: 'flex', gap: '0.75rem', justifyContent: 'space-between' }}>
        <p style={{ fontSize: '0.95rem', fontWeight: 700, margin: 0 }}>
          {props.title}
          {props.remaining ? (
            <span style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', fontWeight: 500 }}> · {props.remaining}</span>
          ) : null}
        </p>
        <button
          type="button"
          disabled={props.disabledReason !== undefined}
          onClick={props.onManage}
          style={{
            background: 'var(--color-surface-2)',
            border: '1px solid var(--color-border)',
            borderRadius: 6,
            color: 'var(--color-text)',
            cursor: props.disabledReason === undefined ? 'pointer' : 'default',
            fontSize: '0.8rem',
            fontWeight: 600,
            opacity: props.disabledReason === undefined ? 1 : 0.5,
            padding: '0.3rem 0.8rem',
          }}
        >
          Manage
        </button>
      </div>
      {props.disabledReason !== undefined ? (
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.4rem 0 0' }}>{props.disabledReason}</p>
      ) : props.summary.length === 0 ? (
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: '0.4rem 0 0' }}>Nothing chosen yet.</p>
      ) : (
        <ul style={{ fontSize: '0.82rem', margin: '0.4rem 0 0', paddingLeft: '1.1rem' }}>
          {props.summary.map((line) => (
            <li key={line}>{line}</li>
          ))}
        </ul>
      )}
      {props.children}
    </section>
  );
}

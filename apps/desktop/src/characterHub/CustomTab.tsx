import { CUSTOM_RECORD_KINDS, customIsEmpty, grantTargetLabel, type CharacterCustom } from './customModel';

/**
 * The sheet's Custom tab: the GM's grants (each with the reason it was given) and the custom feats,
 * equipment, spells and magic devices, with their stat lines and descriptions. Printed with the sheet
 * when there is anything to print. Editing opens the Custom dialog.
 */
export function CustomTab(props: { custom: CharacterCustom; loading: boolean; error: string | null; onEdit: () => void }) {
  const empty = customIsEmpty(props.custom);
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
      <div className="no-print" style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between' }}>
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', margin: 0 }}>
          GM grants and house rules for this character.
        </p>
        <button
          type="button"
          onClick={props.onEdit}
          style={{ background: 'var(--color-surface-2)', border: '1px solid var(--color-border)', borderRadius: 6, color: 'var(--color-text)', cursor: 'pointer', fontSize: '0.85rem', fontWeight: 600, padding: '0.35rem 0.9rem' }}
        >
          Edit Custom…
        </button>
      </div>
      {props.error !== null ? <p role="alert" style={{ color: 'var(--color-error)', margin: 0 }}>{props.error}</p> : null}
      {props.loading ? <p style={{ margin: 0 }}>Loading…</p> : null}
      {!props.loading && empty && props.error === null ? (
        <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>Nothing custom yet.</p>
      ) : null}

      {props.custom.grants.length > 0 ? (
        <section aria-label="Grants">
          <h3 style={{ fontSize: '1rem', margin: '0 0 0.4rem' }}>Grants</h3>
          <ul style={{ margin: 0, paddingLeft: '1.1rem' }}>
            {props.custom.grants.map((grant) => (
              <li key={grant.id}>
                <strong>{grant.label}</strong>: {grant.value >= 0 ? '+' : ''}
                {grant.value} {grantTargetLabel(grant.target)}
                {grant.reason ? <span style={{ color: 'var(--color-text-muted)' }}> ({grant.reason})</span> : null}
              </li>
            ))}
          </ul>
        </section>
      ) : null}

      {CUSTOM_RECORD_KINDS.map(({ kind, plural }) =>
        props.custom[kind].length === 0 ? null : (
          <section key={kind} aria-label={plural}>
            <h3 style={{ fontSize: '1rem', margin: '0 0 0.4rem' }}>{plural}</h3>
            {props.custom[kind].map((record) => (
              <article key={record.id} style={{ borderTop: '1px solid var(--color-border)', padding: '0.5rem 0' }}>
                <strong>{record.name}</strong>
                {record.stats.length > 0 ? (
                  <ul style={{ margin: '0.2rem 0', paddingLeft: '1.1rem' }}>
                    {record.stats.map((stat, index) => (
                      <li key={index}>
                        {stat.label}: {stat.value}
                      </li>
                    ))}
                  </ul>
                ) : null}
                {record.description ? <p style={{ margin: '0.2rem 0 0', whiteSpace: 'pre-wrap' }}>{record.description}</p> : null}
              </article>
            ))}
          </section>
        )
      )}
    </div>
  );
}

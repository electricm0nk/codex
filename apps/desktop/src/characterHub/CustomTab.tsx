import type { OverrideFlagDto } from '../boundary/loadSavedCharacterDetail';
import { wornLabel } from './EquipmentPanel';
import { OverrideTag } from './OverrideTag';
import { CUSTOM_RECORD_KINDS, OVERRIDE_TOGGLES, bonusLine, customIsEmpty, customItemPlace, customOverrides, grantTargetLabel, type CharacterCustom, type CustomOverrides, type CustomRecord } from './customModel';

/** `Worn, Head` or `Carried`; the Gear tab's Equipment control moves it. */
function placeLine(record: CustomRecord): string | null {
  const place = customItemPlace(record);
  if (place === null) {
    return null;
  }
  if (place === 'left_behind') {
    return 'Left behind';
  }
  const slot = record.slot && record.slot !== 'none' ? wornLabel(record.slot) : null;
  return place === 'worn' ? (slot ? `Worn, ${slot}` : 'Worn') : slot ? `Carried (takes ${slot})` : 'Carried';
}

/**
 * The sheet's Custom tab: the GM's grants (each with the reason it was given) and the custom feats,
 * equipment, spells and magic devices, with their stat lines and descriptions. Printed with the sheet
 * when there is anything to print. Editing opens the Custom dialog.
 */
export function CustomTab(props: {
  custom: CharacterCustom;
  loading: boolean;
  error: string | null;
  onEdit: () => void;
  onOverrideChange: (key: keyof CustomOverrides, on: boolean) => void;
  /** Class levels an override let in; no other tab lists a level. */
  admittedLevels: ReadonlyArray<{ label: string; levels: number; flag: OverrideFlagDto }>;
}) {
  const empty = customIsEmpty(props.custom);
  const overrides = customOverrides(props.custom);
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
      {props.loading ? null : (
        <fieldset aria-label="Overrides" className="no-print" style={{ border: '1px solid var(--color-border)', borderRadius: 6, margin: 0, padding: '0.5rem 0.75rem' }}>
          <legend style={{ fontSize: '1rem', fontWeight: 600 }}>Overrides</legend>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', margin: '0 0 0.4rem' }}>
            Lets this character do what the rules would refuse.
          </p>
          <div style={{ display: 'grid', gap: '0.2rem 1rem', gridTemplateColumns: 'repeat(auto-fit, minmax(16rem, 1fr))' }}>
            {OVERRIDE_TOGGLES.map(({ key, label }) => (
              <label key={key} style={{ alignItems: 'center', display: 'flex', gap: '0.4rem' }}>
                <input type="checkbox" aria-label={label} checked={overrides[key]} onChange={(event) => props.onOverrideChange(key, event.target.checked)} />
                {label}
              </label>
            ))}
          </div>
          {props.admittedLevels.map((line) => (
            <p key={`${line.label}::${line.flag}`} style={{ fontSize: '0.85rem', margin: '0.4rem 0 0' }}>
              {`${line.levels} ${line.levels === 1 ? 'level' : 'levels'} of ${line.label}`}
              <OverrideTag flag={line.flag} />
            </p>
          ))}
        </fieldset>
      )}
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
                {placeLine(record) !== null ? <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0.1rem 0' }}>{placeLine(record)}</p> : null}
                {(record.bonuses ?? []).length > 0 ? (
                  <ul style={{ margin: '0.2rem 0', paddingLeft: '1.1rem' }}>
                    {(record.bonuses ?? []).map((bonus, index) => (
                      <li key={index}>{bonusLine(bonus)}</li>
                    ))}
                  </ul>
                ) : null}
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

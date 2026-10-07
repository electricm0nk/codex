import { PRICE_MODE_OPTIONS, type PriceMode } from './priceMode';

/**
 * The pricing choice shown on the equipment screens: cashless, buy 100% / sell 50%, or character
 * build. Session state in the sheet; nothing here is saved on the character.
 */
export function PriceModeControl(props: { value: PriceMode; onChange: (mode: PriceMode) => void }) {
  const selected = PRICE_MODE_OPTIONS.find((option) => option.value === props.value);
  return (
    <fieldset
      aria-label="Equipment pricing"
      style={{ border: '1px solid var(--color-border)', borderRadius: 8, margin: '0 0 1rem', padding: '0.5rem 0.9rem' }}
    >
      <legend style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', padding: '0 0.4rem', textTransform: 'uppercase' }}>Pricing</legend>
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem 1.25rem', justifyContent: 'center' }}>
        {PRICE_MODE_OPTIONS.map((option) => (
          <label key={option.value} style={{ alignItems: 'center', cursor: 'pointer', display: 'flex', fontSize: '0.85rem', gap: '0.35rem' }}>
            <input
              type="radio"
              name="equipment-price-mode"
              value={option.value}
              checked={props.value === option.value}
              onChange={() => props.onChange(option.value)}
            />
            {option.label}
          </label>
        ))}
      </div>
      {selected ? (
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.4rem 0 0', textAlign: 'center' }}>{selected.hint}</p>
      ) : null}
    </fieldset>
  );
}

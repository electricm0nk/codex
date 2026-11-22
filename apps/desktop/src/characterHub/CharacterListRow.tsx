import type { CharacterHubListRowSurface } from './buildCharacterHubListSurface';

export function CharacterListRow(props: { row: CharacterHubListRowSurface }) {
  const { row } = props;
  return (
    <div
      style={{
        alignItems: 'center',
        border: '1px solid var(--color-border)',
        borderRadius: 12,
        display: 'flex',
        justifyContent: 'space-between',
        padding: '0.9rem 1.1rem',
      }}
    >
      <div>
        <p style={{ color: 'var(--color-text)', fontSize: '1rem', fontWeight: 700, margin: 0 }}>{row.displayLabel}</p>
        <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem', margin: '0.25rem 0 0' }}>
          {row.gameSystemLabel} · {row.raceLabel} {row.classSummary}
        </p>
      </div>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: 0 }}>{row.savedAtLabel}</p>
    </div>
  );
}

import type { CharacterHubListRowSurface } from './buildCharacterHubListSurface';

export function CharacterListRow(props: { row: CharacterHubListRowSurface }) {
  const { row } = props;
  return (
    <div
      style={{
        alignItems: 'center',
        border: '1px solid #cbd5e1',
        borderRadius: 12,
        display: 'flex',
        justifyContent: 'space-between',
        padding: '0.9rem 1.1rem',
      }}
    >
      <div>
        <p style={{ color: '#0f172a', fontSize: '1rem', fontWeight: 700, margin: 0 }}>{row.displayLabel}</p>
        <p style={{ color: '#475569', fontSize: '0.85rem', margin: '0.25rem 0 0' }}>
          {row.gameSystemLabel} · {row.raceLabel} {row.classSummary}
        </p>
      </div>
      <p style={{ color: '#64748b', fontSize: '0.8rem', margin: 0 }}>{row.savedAtLabel}</p>
    </div>
  );
}

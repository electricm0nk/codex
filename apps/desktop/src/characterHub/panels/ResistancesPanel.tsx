import type { ResistancesDto } from '../../boundary/loadSavedCharacterDetail';
import { resistanceTiles } from '../resistancesModel';
import { StatTile, panel } from './StatBox';

/**
 * The Defense tab's Resistances panel (`decisions.md §24.3.8`): Acid, Cold, Electricity, Fire and
 * Sonic, Damage Reduction, Spell Resistance and Immunities, from the load response's `resistances`
 * as the printed Resistances block shows them. Renders nothing until a read carries `resistances`.
 */
export function ResistancesPanel(props: { resistances: ResistancesDto | undefined }) {
  const tiles = resistanceTiles(props.resistances);
  if (tiles.length === 0) {
    return null;
  }
  return (
    <div style={{ ...panel, marginBottom: '1rem', padding: '0.75rem 1rem' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.66rem', letterSpacing: '0.06em', margin: '0 0 0.6rem', textTransform: 'uppercase' }}>
        Resistances
      </p>
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.5rem' }}>
        {tiles.slice(0, 5).map((tile) => (
          <StatTile key={tile.label} label={tile.label} value={tile.value} />
        ))}
      </div>
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem' }}>
        {tiles.slice(5).map((tile) => (
          <StatTile key={tile.label} label={tile.label} value={tile.value} />
        ))}
      </div>
    </div>
  );
}

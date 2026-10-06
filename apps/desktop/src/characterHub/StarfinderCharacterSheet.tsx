import type { CSSProperties } from 'react';
import type { LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';
import { RulesAndFeaturesSection } from './CharacterSheet';
import { buildStarfinderSheet, type StarfinderSheetCell, type StarfinderSheetSection } from './starfinderSheetModel';

/**
 * The Starfinder 1e character sheet (SD-37 E6.3): Stamina, Hit Points and
 * Resolve; EAC and KAC; saves, initiative, base attack bonus, skills, spells,
 * credits and bulk. No CMB, CMD, touch or flat-footed AC -- Starfinder has
 * none. Every number is one engine explanation row (`starfinderSheetModel.ts`);
 * the printed lines are the engine's "Rules and features", rendered verbatim.
 */
export function StarfinderCharacterSheet(props: { detail: LoadSavedCharacterResponse; onClose: () => void; onOpen: () => void }) {
  const sheet = buildStarfinderSheet(props.detail);
  return (
    <section data-testid="sf-character-sheet" style={{ marginTop: '1.5rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', gap: '0.75rem', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <div>
          <h2 data-sf-name style={{ margin: 0 }}>{sheet.name}</h2>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0.2rem 0 0' }}>
            Starfinder{sheet.raceLabel === null ? '' : ` · ${sheet.raceLabel}`}
          </p>
        </div>
        <div style={{ display: 'flex', gap: '0.5rem' }}>
          <button type="button" data-testid="sf-sheet-open" onClick={props.onOpen} style={buttonStyle}>
            Open
          </button>
          <button type="button" data-testid="sf-sheet-close" onClick={props.onClose} style={buttonStyle}>
            Close
          </button>
        </div>
      </div>

      {sheet.blocking.length === 0 ? null : (
        <div data-testid="sf-sheet-blocking" style={{ border: '1px solid var(--color-danger, #c0392b)', borderRadius: 8, marginBottom: '1rem', padding: '0.75rem' }}>
          <p style={{ fontWeight: 700, margin: '0 0 0.4rem' }}>This character did not compute:</p>
          <ul style={{ margin: 0, paddingLeft: '1.2rem' }}>
            {sheet.blocking.map((diagnostic) => (
              <li key={diagnostic.id}>{diagnostic.message}</li>
            ))}
          </ul>
        </div>
      )}

      <div style={{ display: 'grid', gap: '1rem', gridTemplateColumns: 'repeat(auto-fill, minmax(16rem, 1fr))' }}>
        {sheet.sections.map((section) => (
          <SheetSection key={section.id} section={section} />
        ))}
      </div>

      {sheet.notes.length === 0 ? null : (
        <div data-testid="sf-sheet-notes" style={{ marginTop: '1rem' }}>
          {sheet.notes.map((diagnostic) => (
            <p key={diagnostic.id} style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.2rem 0' }}>
              {diagnostic.message}
            </p>
          ))}
        </div>
      )}

      <RulesAndFeaturesSection lines={sheet.sheetLines} unavailableReason={sheet.sheetRulesUnavailableReason} />
    </section>
  );
}

const buttonStyle: CSSProperties = {
  background: 'none',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  cursor: 'pointer',
  padding: '0.5rem 1rem',
};

function SheetSection(props: { section: StarfinderSheetSection }) {
  const { section } = props;
  return (
    <div data-sf-section={section.id} style={{ border: '1px solid var(--color-border)', borderRadius: 8, padding: '0.75rem' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', fontWeight: 700, letterSpacing: '0.04em', margin: '0 0 0.5rem', textTransform: 'uppercase' }}>
        {section.title}
      </p>
      <table style={{ borderCollapse: 'collapse', width: '100%' }}>
        {section.columns === null || section.columns.length < 2 ? null : (
          <thead>
            <tr>
              <th />
              {section.columns.map((column) => (
                <th key={column} style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', textAlign: 'right' }}>
                  {column}
                </th>
              ))}
            </tr>
          </thead>
        )}
        <tbody>
          {section.rows.map((row) => {
            const ids = row.cells.flatMap((cell) => (cell === null ? [] : [cell.rowId])).join(' ');
            return (
              <tr key={`${row.label}:${ids}`}>
                <th data-sf-label-of={ids} scope="row" style={{ fontSize: '0.85rem', fontWeight: 600, padding: '0.2rem 0', textAlign: 'left' }}>
                  {row.label}
                </th>
                {row.cells.map((cell, index) => (
                  <td key={index} style={{ padding: '0.2rem 0 0.2rem 0.75rem', textAlign: 'right' }}>
                    {cell === null ? <span style={{ color: 'var(--color-text-faint)' }}>—</span> : <Value cell={cell} />}
                  </td>
                ))}
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}

function Value(props: { cell: StarfinderSheetCell }) {
  return (
    <span data-sf-row={props.cell.rowId} data-testid={props.cell.rowId} title={props.cell.detail} style={{ color: 'var(--color-accent)', fontSize: '0.95rem', fontWeight: 800 }}>
      {props.cell.value}
    </span>
  );
}

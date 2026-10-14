import { Fragment, useEffect, useRef, useState, type CSSProperties } from 'react';
import { saveCyberpunkCriticals } from '../boundary/cprSheet';
import { loadCyberpunkCatalog, type CyberpunkRecord } from '../boundary/loadCyberpunkCatalog';
import type { LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';
import { RulesAndFeaturesSection } from './CharacterSheet';
import { boxesInColumn } from './sheetLayout';
import { CYBERPUNK_SHEET_COLUMNS, CYBERPUNK_SHEET_COLUMN_STYLES, CYBERPUNK_SHEET_LAYOUT } from './cyberpunkSheetLayout';
import {
  buildCyberpunkSheet,
  cyberpunkCriticalOfRow,
  recordedCyberpunkCriticals,
  toggleCyberpunkCritical,
  type CyberpunkSheetBox,
  type CyberpunkSheetCell,
  type CyberpunkSheetRow,
} from './cyberpunkSheetModel';

/**
 * The Cyberpunk RED character sheet: the ten STATs (EMP as entered and current), the 66 skills
 * under their categories, Hit Points, Seriously Wounded, Death Save, Humanity, MOVE, LUCK, the
 * wound track, the Role Ability, armour, the weapons and their range DV rows, the cyberware
 * (each piece's Humanity Loss, option slots per container), a Netrunner's deck, Reputation (with
 * the resolved Facedown roll and every event), Improvement Points, the Lifepath answers and the
 * critical-injuries checklist. Its boxes are placed by `cyberpunkSheetLayout.ts`; every number is
 * one engine `cpr.` row (`cyberpunkSheetModel.ts`), as `load_saved_character` composed it -- this
 * file writes no number of its own. Ticking or unticking an injury saves the checklist through
 * `save_cyberpunk_criticals` and the sheet re-renders from the engine's answer (its Base Death
 * Save Penalty row moved by the engine). The engine's rule lines no box places (`cpr_sheet_print.rs`:
 * the Role Ability, cyberware effects, Humanity Loss, weapon specials, brain damage) render
 * verbatim under "Rules and features", as on the Starfinder sheet.
 */
export function CyberpunkCharacterSheet(props: {
  detail: LoadSavedCharacterResponse;
  onClose: () => void;
  onOpen: () => void;
  /** The skill catalog when the caller already holds it; otherwise the sheet loads it
   * (`list_cyberpunk_catalog('skill')`). */
  skillCatalog?: readonly CyberpunkRecord[];
  /** The Critical Injury catalog when the caller already holds it; otherwise the sheet loads it
   * (`list_cyberpunk_catalog('critical_injury')`). */
  criticalCatalog?: readonly CyberpunkRecord[];
}) {
  const given = props.skillCatalog;
  const [loaded, setLoaded] = useState<readonly CyberpunkRecord[] | null>(null);
  const [catalogError, setCatalogError] = useState<string | null>(null);
  const givenCriticals = props.criticalCatalog;
  const [loadedCriticals, setLoadedCriticals] = useState<readonly CyberpunkRecord[] | null>(null);
  const [criticalCatalogError, setCriticalCatalogError] = useState<string | null>(null);
  // The sheet as last answered by the engine: the load, then each checklist save's reload.
  const [detail, setDetail] = useState<LoadSavedCharacterResponse>(props.detail);
  const [saving, setSaving] = useState(false);
  const [saveError, setSaveError] = useState<string | null>(null);
  const mounted = useRef(true);
  useEffect(
    () => () => {
      mounted.current = false;
    },
    [],
  );
  // The skills' categories and linked STATs come from the corpus skill catalog.
  useEffect(() => {
    if (given !== undefined) {
      return;
    }
    let live = true;
    loadCyberpunkCatalog('skill')
      .then((catalog) => {
        if (live) {
          setLoaded(catalog.records);
        }
      })
      .catch((cause: unknown) => {
        if (live) {
          setCatalogError(cause instanceof Error ? cause.message : String(cause));
        }
      });
    return () => {
      live = false;
    };
  }, [given]);
  // The checklist lists both Critical Injury tables from the corpus catalog.
  useEffect(() => {
    if (givenCriticals !== undefined) {
      return;
    }
    let live = true;
    loadCyberpunkCatalog('critical_injury')
      .then((catalog) => {
        if (live) {
          setLoadedCriticals(catalog.records);
        }
      })
      .catch((cause: unknown) => {
        if (live) {
          setCriticalCatalogError(cause instanceof Error ? cause.message : String(cause));
        }
      });
    return () => {
      live = false;
    };
  }, [givenCriticals]);
  const sheet = buildCyberpunkSheet(detail, given ?? loaded, givenCriticals ?? loadedCriticals);
  const onToggleCritical = (row: CyberpunkSheetRow) => {
    const next = toggleCyberpunkCritical(recordedCyberpunkCriticals(detail), cyberpunkCriticalOfRow(row));
    setSaving(true);
    setSaveError(null);
    saveCyberpunkCriticals(detail.summary.characterId, next)
      .then((reloaded) => {
        if (mounted.current) {
          setDetail(reloaded);
        }
      })
      .catch((cause: unknown) => {
        if (mounted.current) {
          setSaveError(cause instanceof Error ? cause.message : String(cause));
        }
      })
      .finally(() => {
        if (mounted.current) {
          setSaving(false);
        }
      });
  };
  const boxById = new Map(sheet.boxes.map((box) => [box.id, box]));
  return (
    <section data-testid="cpr-character-sheet" style={{ marginTop: '1.5rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', gap: '0.75rem', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <div>
          <h2 data-cpr-name style={{ margin: 0 }}>{sheet.name}</h2>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0.2rem 0 0' }}>Cyberpunk RED</p>
        </div>
        <div style={{ display: 'flex', gap: '0.5rem' }}>
          <button type="button" data-testid="cpr-sheet-open" onClick={props.onOpen} style={buttonStyle}>
            Open
          </button>
          <button type="button" data-testid="cpr-sheet-close" onClick={props.onClose} style={buttonStyle}>
            Close
          </button>
        </div>
      </div>

      {sheet.blocking.length === 0 ? null : (
        <div data-testid="cpr-sheet-blocking" style={{ border: '1px solid var(--color-danger, #c0392b)', borderRadius: 8, marginBottom: '1rem', padding: '0.75rem' }}>
          <p style={{ fontWeight: 700, margin: '0 0 0.4rem' }}>This character did not compute:</p>
          <ul style={{ margin: 0, paddingLeft: '1.2rem' }}>
            {sheet.blocking.map((diagnostic) => (
              <li key={diagnostic.id}>{diagnostic.message}</li>
            ))}
          </ul>
        </div>
      )}

      {catalogError === null ? null : (
        <p data-testid="cpr-sheet-skill-catalog-error" style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0 0 0.75rem' }}>
          Skill categories could not be loaded, so the skills are listed ungrouped: {catalogError}
        </p>
      )}

      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '1rem' }}>
        {CYBERPUNK_SHEET_COLUMNS.map((column) => (
          <div key={column} data-cpr-column={column} style={CYBERPUNK_SHEET_COLUMN_STYLES[column]}>
            {boxesInColumn(CYBERPUNK_SHEET_LAYOUT, column).map((placement) => {
              const box = boxById.get(placement.id);
              if (box === undefined) {
                return null;
              }
              return box.id === 'criticalInjuries' ? (
                <CriticalChecklist
                  key={box.id}
                  box={box}
                  style={placement.boxStyle}
                  saving={saving}
                  saveError={saveError}
                  catalogError={criticalCatalogError}
                  onToggle={onToggleCritical}
                />
              ) : (
                <SheetBox key={box.id} box={box} style={placement.boxStyle} />
              );
            })}
          </div>
        ))}
      </div>

      {sheet.notes.length === 0 ? null : (
        <div data-testid="cpr-sheet-notes" style={{ marginTop: '1rem' }}>
          {sheet.notes.map((diagnostic) => (
            <p key={diagnostic.id} style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.2rem 0' }}>
              {diagnostic.message}
            </p>
          ))}
        </div>
      )}

      <RulesAndFeaturesSection lines={sheet.rules} unavailableReason={null} />
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

function SheetBox(props: { box: CyberpunkSheetBox; style: CSSProperties }) {
  const { box } = props;
  return (
    <div data-cpr-box={box.id} style={props.style}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', fontWeight: 700, letterSpacing: '0.04em', margin: '0 0 0.5rem', textTransform: 'uppercase' }}>
        {box.title}
      </p>
      <table style={{ borderCollapse: 'collapse', width: '100%' }}>
        {box.columns === null ? null : (
          <thead>
            <tr>
              <th />
              {box.columns.map((column) => (
                <th key={typeof column === 'string' ? column : column.rowId} style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', textAlign: 'right' }}>
                  {typeof column === 'string' ? column : <Value cell={column} />}
                </th>
              ))}
            </tr>
          </thead>
        )}
        <tbody>
          {box.rows.map((row, rowIndex) => {
            const ids = row.cells.flatMap((cell) => (cell === null ? [] : [cell.rowId])).join(' ');
            const group = row.group ?? null;
            const heading = group !== null && (rowIndex === 0 || box.rows[rowIndex - 1].group !== group);
            return (
              <Fragment key={`${row.label}:${ids}`}>
                {heading ? (
                  <tr data-cpr-skill-group={group}>
                    <th colSpan={(box.columns?.length ?? 1) + 1} scope="rowgroup" style={groupStyle}>
                      {group}
                    </th>
                  </tr>
                ) : null}
                <tr>
                  <th data-cpr-label-of={ids} scope="row" style={{ fontSize: '0.85rem', fontWeight: 600, padding: '0.2rem 0', textAlign: 'left' }}>
                    {row.labelCell === undefined ? row.label : <Value cell={row.labelCell} />}
                    {row.tag === undefined ? null : ` (${row.tag})`}
                  </th>
                  {box.columns === null ? (
                    <td style={cellStyle}>
                      {row.cells.map((cell, index) => (
                        <span key={index}>
                          {index === 0 ? null : ' / '}
                          {cell === null ? <Missing /> : <Value cell={cell} />}
                        </span>
                      ))}
                    </td>
                  ) : (
                    row.cells.map((cell, index) => (
                      <td key={index} style={cellStyle}>
                        {cell === null ? <Missing /> : <Value cell={cell} />}
                      </td>
                    ))
                  )}
                </tr>
              </Fragment>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}

/**
 * The critical-injuries checklist: one checkbox per injury under its table, ticked when the engine
 * printed it as recorded (the ticked entry shows that printed line). A change saves the whole list.
 */
function CriticalChecklist(props: {
  box: CyberpunkSheetBox;
  style: CSSProperties;
  saving: boolean;
  saveError: string | null;
  catalogError: string | null;
  onToggle: (row: CyberpunkSheetRow) => void;
}) {
  const { box } = props;
  return (
    <div data-cpr-box={box.id} style={props.style}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', fontWeight: 700, letterSpacing: '0.04em', margin: '0 0 0.5rem', textTransform: 'uppercase' }}>
        {box.title}
      </p>
      {props.catalogError === null ? null : (
        <p data-testid="cpr-sheet-critical-catalog-error" style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0 0 0.5rem' }}>
          The Critical Injury tables could not be loaded, so only the recorded injuries are listed: {props.catalogError}
        </p>
      )}
      {box.rows.map((row, index) => {
        const ticked = row.cells[0] ?? null;
        const heading = index === 0 || box.rows[index - 1].group !== row.group;
        return (
          <Fragment key={`${String(row.group)}:${row.label}`}>
            {heading ? (
              <p data-cpr-critical-table={row.group ?? ''} style={{ ...groupStyle, margin: 0 }}>
                {row.group}
              </p>
            ) : null}
            <label data-cpr-critical={`${String(row.group)}:${row.label}`} style={{ alignItems: 'center', display: 'flex', fontSize: '0.85rem', gap: '0.4rem', padding: '0.15rem 0' }}>
              <input type="checkbox" checked={ticked !== null} disabled={props.saving} onChange={() => props.onToggle(row)} />
              {ticked === null ? row.label : <Value cell={ticked} />}
            </label>
          </Fragment>
        );
      })}
      {props.saveError === null ? null : (
        <p data-testid="cpr-sheet-critical-save-error" style={{ color: 'var(--color-danger, #c0392b)', fontSize: '0.8rem', margin: '0.4rem 0 0' }}>
          {props.saveError}
        </p>
      )}
    </div>
  );
}

const groupStyle: CSSProperties = {
  borderBottom: '1px solid var(--color-border)',
  color: 'var(--color-text-muted)',
  fontSize: '0.72rem',
  fontWeight: 700,
  padding: '0.5rem 0 0.15rem',
  textAlign: 'left',
};

const cellStyle: CSSProperties = { padding: '0.2rem 0 0.2rem 0.75rem', textAlign: 'right' };

function Missing() {
  return <span style={{ color: 'var(--color-text-faint)' }}>—</span>;
}

function Value(props: { cell: CyberpunkSheetCell }) {
  if (props.cell.printed === true) {
    // A printed line: dice stay literal, words stay words; the engine's text, never a number of ours.
    return (
      <span data-cpr-printed={props.cell.rowId} title={props.cell.detail} style={{ fontSize: '0.9rem', fontWeight: 600 }}>
        {props.cell.value}
      </span>
    );
  }
  return (
    <span data-cpr-row={props.cell.rowId} data-testid={props.cell.rowId} title={props.cell.detail} style={{ color: 'var(--color-accent)', fontSize: '0.95rem', fontWeight: 800 }}>
      {props.cell.value}
    </span>
  );
}

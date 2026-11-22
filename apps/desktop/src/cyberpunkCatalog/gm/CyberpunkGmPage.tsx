import { useMemo, type CSSProperties } from 'react';
import type { CyberpunkCatalogResponse } from '../../boundary/loadCyberpunkCatalog';
import { cyberpunkCatalogTab, describeCyberpunkCatalog, filterCyberpunkRecords } from '../cyberpunkCatalogModel';
import { gmEntry, type GmEntry } from './cyberpunkGmModel';

/**
 * SD-38 E6.4 -- a GM reference page: the Cyberpunk RED corpus's `print_only` kinds (NET
 * Architecture tables, Demons, Night Market, Playstyle IP awards, trauma Humanity Loss, Mooks
 * and Grunts, Encounters, Beat Charts) as read-only tables and text. The response comes from
 * `list_cyberpunk_catalog`; nothing here edits or computes a record.
 */

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

const CELL: CSSProperties = { border: '1px solid var(--color-border)', padding: '0.25rem 0.5rem', textAlign: 'left', verticalAlign: 'top' };

function Entry(props: { kind: string; index: number; entry: GmEntry }) {
  const { entry } = props;
  return (
    <li data-cpr-gm-entry={`${props.kind}:${props.index}`} style={{ ...panel, listStyle: 'none', padding: '0.75rem 1rem' }}>
      <div style={{ alignItems: 'baseline', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', justifyContent: 'space-between' }}>
        <strong>{entry.heading}</strong>
        {entry.page !== null ? <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem' }}>{`p. ${entry.page}`}</span> : null}
      </div>
      {entry.paragraphs.map((paragraph, index) => (
        <p key={index} style={{ margin: '0.5rem 0 0', whiteSpace: 'pre-wrap' }}>
          {paragraph}
        </p>
      ))}
      {entry.facts.length > 0 ? (
        <ul style={{ margin: '0.5rem 0 0', paddingLeft: '1.1rem' }}>
          {entry.facts.map((fact) => (
            <li key={fact.label} style={{ fontSize: '0.85rem' }}>
              <span style={{ fontWeight: 600 }}>{fact.label}:</span> {fact.text}
            </li>
          ))}
        </ul>
      ) : null}
      {entry.lists.map((list) => (
        <div key={list.label} style={{ marginTop: '0.5rem' }}>
          <div style={{ fontSize: '0.85rem', fontWeight: 600 }}>{list.label}</div>
          <ul style={{ margin: 0, paddingLeft: '1.1rem' }}>
            {list.items.map((item, index) => (
              <li key={index} style={{ fontSize: '0.85rem', whiteSpace: 'pre-wrap' }}>
                {item}
              </li>
            ))}
          </ul>
        </div>
      ))}
      {entry.tables.map((table) => (
        <div key={table.caption} style={{ marginTop: '0.5rem', overflowX: 'auto' }}>
          <table style={{ borderCollapse: 'collapse', fontSize: '0.85rem', width: '100%' }}>
            <caption style={{ fontWeight: 600, textAlign: 'left' }}>{table.caption}</caption>
            <thead>
              <tr>
                {table.columns.map((column) => (
                  <th key={column} scope="col" style={CELL}>
                    {column}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {table.rows.map((row, rowIndex) => (
                <tr key={rowIndex}>
                  {row.map((text, columnIndex) => (
                    <td key={columnIndex} style={CELL}>
                      {text}
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      ))}
    </li>
  );
}

/** The GM page body for one loaded print-only kind (pure: the catalog screen loads it). */
export function CyberpunkGmPage(props: { response: CyberpunkCatalogResponse; query: string }) {
  const { response } = props;
  const filtered = useMemo(() => filterCyberpunkRecords(response.records, props.query), [response, props.query]);
  const indexOf = useMemo(() => new Map(response.records.map((record, index) => [record, index])), [response]);
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
      <p data-cpr-gm-summary style={{ color: 'var(--color-text-muted)', margin: 0 }}>
        {describeCyberpunkCatalog(cyberpunkCatalogTab(response.kind).title, response.records.length, response.expected, filtered.length)}
        {' — GM reference, read-only.'}
      </p>
      <ul style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem', margin: 0, padding: 0 }}>
        {filtered.map((record) => (
          <Entry key={indexOf.get(record)} kind={response.kind} index={indexOf.get(record) ?? 0} entry={gmEntry(record)} />
        ))}
      </ul>
    </div>
  );
}

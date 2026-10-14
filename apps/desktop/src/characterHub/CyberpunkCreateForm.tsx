import { useEffect, useRef, useState, type CSSProperties, type FormEvent, type ReactNode } from 'react';
import {
  createCyberpunkCharacter,
  previewCyberpunkCharacter,
  type CyberpunkCreationPreview,
  type CyberpunkCreationProblem,
  type CyberpunkLifepathTable,
  type CyberpunkStatValue,
} from '../boundary/cprCreation';
import { loadCyberpunkCatalog, type CyberpunkRecord } from '../boundary/loadCyberpunkCatalog';
import {
  CYBERPUNK_CREATION_METHODS,
  EMPTY_CYBERPUNK_DRAFT,
  adjustStat,
  blockingProblems,
  buildCyberpunkCreateRequest,
  buildCyberpunkPreviewRequest,
  culturalLanguageOptions,
  culturalLanguageTile,
  cyberpunkDerivedTiles,
  cyberpunkStatTiles,
  lifepathRowOptions,
  rollAgain,
  selectMethod,
  selectRole,
  setCulturalLanguage,
  setLifepathCount,
  setLifepathPick,
  setLifepathSkip,
  templateRowOptions,
  withPreviewSeed,
  type CyberpunkCreationDraft,
  type CyberpunkTile,
} from './cyberpunkCreationModel';
import {
  CYBERPUNK_CREATE_COLUMNS,
  CYBERPUNK_CREATE_COLUMN_STYLES,
  CYBERPUNK_CREATE_LAYOUT,
  type CyberpunkCreateBoxId,
} from './cyberpunkCreateLayout';
import { boxesInColumn } from './sheetLayout';

/**
 * Cyberpunk RED character creation: name and Role, then the STAT step by one of the three
 * methods (Streetrat, Edgerunner, Complete Package). Every STAT, points figure and derived value
 * on this form is the engine's answer for the current choices (`preview_cyberpunk_character`);
 * "Create character" saves through `create_cyberpunk_character`, which sends the same seed so the
 * saved character has the previewed dice, saves with the Cyberpunk RED store and reads the
 * character back through `CyberpunkAdapter`. A refused build saves nothing. The boxes are placed
 * by `CYBERPUNK_CREATE_LAYOUT`.
 *
 * The Lifepath panel (E3.3) lists every table the engine answered for the Role (the general
 * tables, then the Role's): each answer is the engine's roll until the player picks a row from
 * the `lifepath_table` record; count tables take a picked count; a table behind a printed choice
 * can be marked not taken. The Cultural Origin language is picked or rolled, and its level is
 * the engine's `cpr.skill_level.language.*` row. The answers are saved with the character.
 */

const LABEL_STYLE: CSSProperties = {
  color: 'var(--color-text-secondary)',
  display: 'block',
  fontSize: '0.875rem',
  fontWeight: 600,
  marginBottom: '0.35rem',
};
const INPUT_STYLE: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  boxSizing: 'border-box',
  color: 'var(--color-text)',
  padding: '0.5rem 0.65rem',
  width: '100%',
};
const FIELD_STYLE: CSSProperties = { marginBottom: '0.75rem', minWidth: 0 };
const SMALL_BUTTON_STYLE: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.8rem',
  padding: '0.2rem 0.5rem',
};
const FIELDSET_STYLE: CSSProperties = { border: '1px solid var(--color-border)', borderRadius: 8, margin: 0, padding: '0.75rem 1rem' };
const LEGEND_STYLE: CSSProperties = { fontSize: '0.85rem', fontWeight: 700 };
const MUTED_STYLE: CSSProperties = { color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: 0 };

type Outcome =
  | { kind: 'saved'; headline: string; stats: CyberpunkTile[]; derived: CyberpunkTile[]; language: CyberpunkTile | null }
  | { kind: 'blocked'; diagnostics: CyberpunkCreationProblem[] };

function Tile(props: { tile: CyberpunkTile }) {
  return (
    <div style={{ backgroundColor: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 8, padding: '0.5rem 0.75rem' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', margin: 0, textTransform: 'uppercase' }}>{props.tile.label}</p>
      <p style={{ color: 'var(--color-text)', fontSize: '1rem', fontWeight: 700, margin: '0.2rem 0 0' }}>{props.tile.value}</p>
    </div>
  );
}

function TileGrid(props: { tiles: CyberpunkTile[] }) {
  return (
    <div style={{ display: 'grid', gap: '0.5rem', gridTemplateColumns: 'repeat(auto-fit, minmax(96px, 1fr))' }}>
      {props.tiles.map((tile) => (
        <Tile key={tile.id} tile={tile} />
      ))}
    </div>
  );
}

export function CyberpunkCreateForm(props: { onCreated: () => void }) {
  const [draft, setDraft] = useState<CyberpunkCreationDraft>(EMPTY_CYBERPUNK_DRAFT);
  const [roles, setRoles] = useState<CyberpunkRecord[]>([]);
  const [templates, setTemplates] = useState<CyberpunkRecord[]>([]);
  const [lifepathRecords, setLifepathRecords] = useState<CyberpunkRecord[]>([]);
  const [preview, setPreview] = useState<CyberpunkCreationPreview | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [outcome, setOutcome] = useState<Outcome | null>(null);
  const previewRound = useRef(0);

  // The Roles and their Streetrat rows come from the corpus (`list_cyberpunk_catalog`).
  useEffect(() => {
    Promise.all([loadCyberpunkCatalog('role'), loadCyberpunkCatalog('streetrat_template'), loadCyberpunkCatalog('lifepath_table')])
      .then(([roleCatalog, templateCatalog, lifepathCatalog]) => {
        setRoles(roleCatalog.records);
        setTemplates(templateCatalog.records);
        setLifepathRecords(lifepathCatalog.records);
      })
      .catch((cause: unknown) => setError(cause instanceof Error ? cause.message : String(cause)));
  }, []);

  // Every choice change asks the engine for the STAT step. The name is not a rules choice.
  const request = buildCyberpunkPreviewRequest(draft);
  const previewKey = JSON.stringify(request);
  useEffect(() => {
    const round = ++previewRound.current;
    if (request === null) {
      setPreview(null);
      return;
    }
    previewCyberpunkCharacter(request)
      .then((next) => {
        if (round !== previewRound.current) {
          return; // a later choice already asked again
        }
        setPreview(next);
        setError(null);
        setDraft((current) => withPreviewSeed(current, next));
      })
      .catch((cause: unknown) => {
        if (round === previewRound.current) {
          setError(cause instanceof Error ? cause.message : String(cause));
        }
      });
    // `previewKey` is the request: the effect re-runs exactly when a rules choice changes.
  }, [previewKey]);

  const blocking = preview ? blockingProblems(preview) : [];
  const shownStats: CyberpunkStatValue[] = preview?.stats ?? [];
  const rows = templateRowOptions(templates, draft.role);
  const statTiles = preview ? cyberpunkStatTiles(preview.explanations, preview.stats) : [];
  const derivedTiles = preview ? cyberpunkDerivedTiles(preview.explanations) : [];
  const languageTile = preview ? culturalLanguageTile(preview) : null;

  function lifepathTable(table: CyberpunkLifepathTable, index: number) {
    const record = lifepathRecords.find((r) => r.name === table.table);
    const languages = record && table.answers.length > 0 && 'languages_by_roll' in record ? culturalLanguageOptions(record, table.answers[0].roll) : null;
    const base = `cpr-lifepath-${index}`;
    return (
      <div key={table.table} data-lifepath-table={table.table} style={{ borderTop: '1px solid var(--color-border)', padding: '0.5rem 0' }}>
        <p style={{ fontSize: '0.85rem', fontWeight: 700, margin: 0 }}>
          {table.table} <span style={{ ...MUTED_STYLE, display: 'inline' }}>({table.die}, {table.sourcePage})</span>
        </p>
        <p style={MUTED_STYLE}>{table.rollInstruction}</p>
        {table.skippable ? (
          <label style={{ ...MUTED_STYLE, display: 'block', marginTop: '0.25rem' }}>
            <input
              type="checkbox"
              checked={table.skipped}
              onChange={(event) => setDraft(setLifepathSkip(draft, table.table, event.target.checked))}
            />{' '}
            Not on my path
          </label>
        ) : null}
        {table.count !== null && table.countMax !== null && !table.skipped ? (
          <div style={{ ...FIELD_STYLE, marginTop: '0.35rem' }}>
            <label style={LABEL_STYLE} htmlFor={`${base}-count`}>
              How many
            </label>
            <select
              id={`${base}-count`}
              style={INPUT_STYLE}
              value={table.countPicked ? String(table.count) : ''}
              onChange={(event) => setDraft(setLifepathCount(draft, table.table, event.target.value === '' ? null : Number(event.target.value)))}
            >
              <option value="">Rolled: {table.count}</option>
              {Array.from({ length: table.countMax + 1 }, (_, n) => (
                <option key={n} value={String(n)}>
                  {n}
                </option>
              ))}
            </select>
          </div>
        ) : null}
        {table.answers.map((answer) => {
          const id = `${base}-${answer.entry}-${answer.columnIndex}`;
          const label = [answer.entry > 0 ? `#${answer.entry}` : '', table.perColumn || table.count !== null ? answer.column : ''].filter(Boolean).join(' ');
          return (
            <div key={id} style={{ ...FIELD_STYLE, marginTop: '0.35rem' }}>
              {label ? (
                <label style={LABEL_STYLE} htmlFor={id}>
                  {label}
                </label>
              ) : null}
              <select
                id={id}
                aria-label={label || table.table}
                style={INPUT_STYLE}
                value={answer.picked ? String(answer.roll) : ''}
                onChange={(event) =>
                  setDraft(setLifepathPick(draft, table.table, answer.entry, answer.columnIndex, event.target.value === '' ? null : Number(event.target.value)))
                }
              >
                <option value="">
                  {answer.picked ? 'Roll it' : `Rolled ${answer.roll}: ${[answer.text, answer.detail].filter(Boolean).join(' / ')}`}
                </option>
                {record
                  ? lifepathRowOptions(record, answer.columnIndex).map((option) => (
                      <option key={option.roll} value={String(option.roll)}>
                        {option.label}
                      </option>
                    ))
                  : null}
              </select>
            </div>
          );
        })}
        {languages !== null ? (
          <div style={{ ...FIELD_STYLE, marginTop: '0.35rem' }}>
            <label style={LABEL_STYLE} htmlFor="cpr-cultural-language">
              Cultural Origin language (pick one of the region's, or any other)
            </label>
            <input
              id="cpr-cultural-language"
              list="cpr-cultural-language-options"
              style={INPUT_STYLE}
              value={draft.lifepath.culturalLanguage ?? ''}
              onChange={(event) => setDraft(setCulturalLanguage(draft, event.target.value))}
            />
            {preview?.culturalLanguage && !preview.culturalLanguage.picked ? (
              <p style={MUTED_STYLE}>Rolled: {preview.culturalLanguage.name} (type a language to pick one)</p>
            ) : null}
            <datalist id="cpr-cultural-language-options">
              {languages.map((language) => (
                <option key={language} value={language} />
              ))}
            </datalist>
            {languageTile ? (
              <div style={{ marginTop: '0.4rem', maxWidth: 220 }}>
                <Tile tile={languageTile} />
              </div>
            ) : null}
          </div>
        ) : null}
      </div>
    );
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    const create = buildCyberpunkCreateRequest(draft, {
      generateId: () => crypto.randomUUID(),
      now: () => new Date().toISOString(),
    });
    if (create === null) {
      return;
    }
    setSubmitting(true);
    setError(null);
    try {
      const result = await createCyberpunkCharacter(create);
      if (result.kind === 'Saved') {
        setOutcome({
          kind: 'saved',
          headline: `${result.summary.displayLabel} is ready`,
          stats: cyberpunkStatTiles(result.explanations, shownStats),
          derived: cyberpunkDerivedTiles(result.explanations),
          language: culturalLanguageTile({ culturalLanguage: preview?.culturalLanguage ?? null, explanations: result.explanations }),
        });
        props.onCreated();
      } else {
        setOutcome({ kind: 'blocked', diagnostics: result.diagnostics });
      }
    } catch (cause: unknown) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setSubmitting(false);
    }
  }

  const boxes: Record<CyberpunkCreateBoxId, ReactNode> = {
    identity: (
      <div>
        <div style={FIELD_STYLE}>
          <label style={LABEL_STYLE} htmlFor="cpr-character-name">
            Character name
          </label>
          <input
            id="cpr-character-name"
            style={INPUT_STYLE}
            value={draft.displayLabel}
            onChange={(event) => setDraft({ ...draft, displayLabel: event.target.value })}
            required
          />
        </div>
        <div style={FIELD_STYLE}>
          <label style={LABEL_STYLE} htmlFor="cpr-role">
            Role
          </label>
          <select
            id="cpr-role"
            style={INPUT_STYLE}
            value={draft.role ?? ''}
            onChange={(event) => setDraft(selectRole(draft, event.target.value || null, lifepathRecords))}
          >
            <option value="">— choose a Role —</option>
            {roles.map((role) => (
              <option key={String(role.name)} value={String(role.name)}>
                {String(role.name)}
              </option>
            ))}
          </select>
        </div>
      </div>
    ),
    method: (
      <fieldset style={FIELDSET_STYLE}>
        <legend style={LEGEND_STYLE}>STATs</legend>
        <div role="group" aria-label="Creation method" style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem', marginBottom: '0.5rem' }}>
          {CYBERPUNK_CREATION_METHODS.map((method) => (
            <button
              key={method.id}
              type="button"
              aria-pressed={draft.method === method.id}
              onClick={() => setDraft(selectMethod(draft, method.id, shownStats))}
              style={{
                ...SMALL_BUTTON_STYLE,
                backgroundColor: draft.method === method.id ? 'var(--color-accent)' : SMALL_BUTTON_STYLE.backgroundColor,
                color: draft.method === method.id ? 'var(--color-on-accent)' : SMALL_BUTTON_STYLE.color,
                fontSize: '0.875rem',
                padding: '0.35rem 0.75rem',
              }}
            >
              {method.label}
            </button>
          ))}
        </div>
        <p style={MUTED_STYLE}>{CYBERPUNK_CREATION_METHODS.find((method) => method.id === draft.method)?.detail}</p>
        {draft.method === 'streetrat' ? (
          <div style={{ ...FIELD_STYLE, marginTop: '0.5rem' }}>
            <label style={LABEL_STYLE} htmlFor="cpr-template-row">
              Template row
            </label>
            <select
              id="cpr-template-row"
              style={INPUT_STYLE}
              disabled={draft.role === null}
              value={draft.templateRoll === null ? '' : String(draft.templateRoll)}
              onChange={(event) =>
                setDraft({ ...draft, templateRoll: event.target.value === '' ? null : Number(event.target.value), seed: null })
              }
            >
              <option value="">Roll the row on 1d10</option>
              {rows.map((row) => (
                <option key={row.roll} value={String(row.roll)}>
                  {row.label}
                </option>
              ))}
            </select>
          </div>
        ) : null}
        {draft.role !== null && (draft.method === 'edgerunner' || (draft.method === 'streetrat' && draft.templateRoll === null)) ? (
          <div style={{ alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.6rem', marginTop: '0.5rem' }}>
            <button type="button" style={SMALL_BUTTON_STYLE} onClick={() => setDraft(rollAgain(draft))}>
              Roll again
            </button>
            {preview && preview.rolls.length > 0 ? (
              <span style={MUTED_STYLE}>Rolled: {preview.rolls.map((roll) => `${roll.die} ${roll.result}`).join(', ')}</span>
            ) : null}
          </div>
        ) : null}
        {draft.method === 'completePackage' && preview ? (
          <div style={{ display: 'grid', gap: '0.35rem', marginTop: '0.5rem' }}>
            {preview.statPointsTotal !== null ? (
              <p style={MUTED_STYLE}>
                Points spent: {preview.statPointsSpent ?? 0} of {preview.statPointsTotal}
              </p>
            ) : null}
            {preview.stats.map((stat) => (
              <div key={stat.id} style={{ alignItems: 'center', display: 'flex', gap: '0.5rem' }}>
                <span style={{ fontWeight: 700, width: '3.5rem' }}>{stat.label}</span>
                <button type="button" style={SMALL_BUTTON_STYLE} onClick={() => setDraft({ ...draft, stats: adjustStat(draft.stats, stat.id, -1) })}>
                  -1 {stat.label}
                </button>
                <span style={{ minWidth: '1.5rem', textAlign: 'center' }}>{stat.value}</span>
                <button type="button" style={SMALL_BUTTON_STYLE} onClick={() => setDraft({ ...draft, stats: adjustStat(draft.stats, stat.id, 1) })}>
                  +1 {stat.label}
                </button>
              </div>
            ))}
          </div>
        ) : null}
      </fieldset>
    ),
    stats: statTiles.length > 0 ? statTiles.map((tile) => <Tile key={tile.id} tile={tile} />) : <p style={MUTED_STYLE}>Choose a Role to see its STATs.</p>,
    derived: derivedTiles.map((tile) => <Tile key={tile.id} tile={tile} />),
  };

  return (
    <form onSubmit={handleSubmit} style={{ border: '1px solid var(--color-border)', borderRadius: 12, padding: '1.25rem' }}>
      <p style={{ ...MUTED_STYLE, fontSize: '0.8rem', marginBottom: '1rem' }}>
        Cyberpunk RED — choose a Role, then make your STATs by one of the three methods.
      </p>
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '1.25rem' }}>
        {CYBERPUNK_CREATE_COLUMNS.map((column) => (
          <div key={column} style={CYBERPUNK_CREATE_COLUMN_STYLES[column]}>
            {boxesInColumn(CYBERPUNK_CREATE_LAYOUT, column).map((box) => (
              <div key={box.id} style={box.boxStyle}>
                {boxes[box.id]}
              </div>
            ))}
          </div>
        ))}
      </div>

      {preview && preview.lifepath.length > 0 ? (
        <fieldset id="cpr-lifepath" style={{ ...FIELDSET_STYLE, marginTop: '1rem' }}>
          <legend style={LEGEND_STYLE}>Lifepath</legend>
          <p style={MUTED_STYLE}>
            Every answer is rolled until you pick one. The answers are printed on your sheet; your Cultural Origin language is a
            Language skill.
          </p>
          {preview.lifepath.map((table, index) => lifepathTable(table, index))}
        </fieldset>
      ) : null}

      {blocking.length > 0 ? (
        <ul style={{ color: 'var(--color-warn)', fontSize: '0.85rem', margin: '0.75rem 0', paddingLeft: '1.1rem' }}>
          {blocking.map((problem) => (
            <li key={`${problem.id}:${problem.message}`}>{problem.message}</li>
          ))}
        </ul>
      ) : null}

      <button
        type="submit"
        disabled={submitting || !preview || blocking.length > 0}
        style={{
          backgroundColor: 'var(--color-accent)',
          border: 'none',
          borderRadius: 8,
          color: 'var(--color-on-accent)',
          cursor: submitting ? 'default' : 'pointer',
          marginTop: '0.75rem',
          padding: '0.6rem 1.25rem',
        }}
      >
        {submitting ? 'Creating…' : 'Create character'}
      </button>

      {error ? <p style={{ color: 'var(--color-error)', marginTop: '0.75rem' }}>{error}</p> : null}

      {outcome ? (
        <div style={{ borderTop: '1px solid var(--color-border)', marginTop: '1.25rem', paddingTop: '1rem' }}>
          {outcome.kind === 'saved' ? (
            <>
              <h3 style={{ margin: '0 0 0.35rem' }}>{outcome.headline}</h3>
              <p style={{ color: 'var(--color-text-secondary)', margin: '0 0 0.75rem' }}>Your character was computed and saved.</p>
              <div style={{ display: 'grid', gap: '0.75rem' }}>
                <TileGrid tiles={outcome.stats} />
                <TileGrid tiles={outcome.derived} />
                {outcome.language ? <TileGrid tiles={[outcome.language]} /> : null}
              </div>
            </>
          ) : (
            <>
              <h3 style={{ margin: '0 0 0.35rem' }}>Character not created</h3>
              <p style={{ color: 'var(--color-text-secondary)', margin: '0 0 0.75rem' }}>Nothing was saved. Resolve these first:</p>
              <ul style={{ color: 'var(--color-warn)', margin: 0, paddingLeft: '1.1rem' }}>
                {outcome.diagnostics.map((diagnostic) => (
                  <li key={`${diagnostic.id}:${diagnostic.message}`} style={{ marginBottom: '0.4rem' }}>
                    {diagnostic.message} <code style={{ fontSize: '0.75rem' }}>{diagnostic.id}</code>
                  </li>
                ))}
              </ul>
            </>
          )}
        </div>
      ) : null}
    </form>
  );
}

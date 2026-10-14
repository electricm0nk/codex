import { useEffect, useRef, useState, type CSSProperties, type FormEvent, type ReactNode } from 'react';
import {
  createCyberpunkCharacter,
  previewCyberpunkCharacter,
  type CyberpunkCreationPreview,
  type CyberpunkCreationProblem,
  type CyberpunkLifepathTable,
  type CyberpunkSkillLine,
  type CyberpunkStatValue,
} from '../boundary/cprCreation';
import { loadCyberpunkCatalog, type CyberpunkRecord } from '../boundary/loadCyberpunkCatalog';
import {
  CYBERPUNK_CREATION_METHODS,
  EMPTY_CYBERPUNK_DRAFT,
  addKitPurchase,
  addRolePick,
  addSkill,
  adjustRolePoints,
  adjustSkill,
  adjustStat,
  blockingProblems,
  buildCyberpunkCreateRequest,
  buildCyberpunkPreviewRequest,
  culturalLanguageOptions,
  culturalLanguageTile,
  cyberpunkDerivedTiles,
  cyberpunkStatTiles,
  dropSkill,
  kitMoneyTiles,
  kitRowTiles,
  lifepathRowOptions,
  removeKitPurchase,
  removeRolePick,
  roleAbilityTiles,
  rolePickTargets,
  rollAgain,
  selectMethod,
  selectRole,
  setCulturalLanguage,
  setKitPick,
  setLifepathCount,
  setLifepathPick,
  setLifepathSkip,
  setSkillSpecialty,
  skillKey,
  skillOptions,
  skillPointTiles,
  shopKinds,
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
 *
 * The skills panel (E3.4) lists the engine's allocation for the method: Streetrat the Role's
 * printed levels (fixed), Edgerunner the same 20 skills moved one level at a time, Complete
 * Package the Basic skills plus any skill of the Master Skill List bought or dropped. A
 * `(choose 1)` line takes the named instrument or field. The points spent and left are the
 * engine's `cpr.skills.creation_points_*` rows; a refused allocation shows the engine's reason.
 *
 * The Role Ability panel (E3.5) shows the Role Ability at its starting Rank 4 and the sub-choices
 * the engine lists for it: points moved between the abilities or specialties one allowed value at
 * a time (Combat Awareness, Maker, Medicine), and picks added or removed (Family Motorpool
 * vehicles and upgrades, the Exec's Team Member and its chart row, the Medtech's drugs). Every
 * number shown is an engine `cpr.role.*` row; a refused choice shows the engine's reason.
 *
 * The starting kit panel (E3.6) shows the Role's fixed kit for Streetrat and Edgerunner, each "or"
 * line a choice of its printed options, and for every method a shop of the engine's priced
 * records (kind, item, its location, caliber or piece, quantity) with the purchases made. The
 * budget, the spent figure, the cash left and the Complete Package fashion budget are the engine's
 * `cpr.kit.*` and `cpr.cash` rows; a refused purchase or overspend shows the engine's reason.
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
  | { kind: 'saved'; headline: string; stats: CyberpunkTile[]; derived: CyberpunkTile[]; language: CyberpunkTile | null; money: CyberpunkTile[] }
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
  const [skillRecords, setSkillRecords] = useState<CyberpunkRecord[]>([]);
  const [newSkill, setNewSkill] = useState('');
  const [newSpecialty, setNewSpecialty] = useState('');
  const [newPick, setNewPick] = useState('');
  const [newPickTarget, setNewPickTarget] = useState('');
  const [shopKind, setShopKind] = useState('');
  const [shopName, setShopName] = useState('');
  const [shopVariant, setShopVariant] = useState('');
  const [shopQuantity, setShopQuantity] = useState('1');
  const [preview, setPreview] = useState<CyberpunkCreationPreview | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [outcome, setOutcome] = useState<Outcome | null>(null);
  const previewRound = useRef(0);

  // The Roles and their Streetrat rows come from the corpus (`list_cyberpunk_catalog`).
  useEffect(() => {
    Promise.all([
      loadCyberpunkCatalog('role'),
      loadCyberpunkCatalog('streetrat_template'),
      loadCyberpunkCatalog('lifepath_table'),
      loadCyberpunkCatalog('skill'),
    ])
      .then(([roleCatalog, templateCatalog, lifepathCatalog, skillCatalog]) => {
        setRoles(roleCatalog.records);
        setTemplates(templateCatalog.records);
        setLifepathRecords(lifepathCatalog.records);
        setSkillRecords(skillCatalog.records);
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
  const pointTiles = preview ? skillPointTiles(preview) : [];
  const options = skillOptions(skillRecords);
  const chosenOption = options.find((option) => option.name === newSkill);
  const newKey = chosenOption ? skillKey(chosenOption.name, chosenOption.specialtyRequired ? newSpecialty : '') : '';
  const canAdd = chosenOption !== undefined && (!chosenOption.specialtyRequired || newSpecialty.trim() !== '');
  const roleAbility = preview?.roleAbility ?? null;
  const roleTiles = preview ? roleAbilityTiles(preview) : [];
  const pickOption = roleAbility?.menu.picks?.options.find((option) => option.name === newPick);
  const pickTargets = roleAbility && pickOption ? rolePickTargets(roleAbility.menu, pickOption, draft) : [];
  const canPick = pickOption !== undefined && (!pickOption.needsTarget || pickTargets.includes(newPickTarget));
  const kit = preview?.kit ?? null;
  const moneyTiles = preview ? kitMoneyTiles(preview) : [];
  const kinds = kit ? shopKinds(kit.shop) : [];
  const shopItems = kit ? kit.shop.filter((item) => item.kind === shopKind) : [];
  const shopItem = shopItems.find((item) => item.name === shopName);
  const quantity = Number.parseInt(shopQuantity, 10);
  const canBuy =
    shopItem !== undefined &&
    (shopItem.variantLabel === null || shopItem.variants.some((v) => v.name === shopVariant)) &&
    Number.isInteger(quantity) &&
    quantity > 0;
  const boughtItems = kit ? kit.items.filter((item) => item.section === 'bought') : [];

  function skillLine(line: CyberpunkSkillLine, index: number) {
    const id = `cpr-skill-${index}`;
    const droppable = draft.method === 'completePackage' && !line.basic && !line.cultural;
    return (
      <div key={line.key} data-skill={line.key} style={{ alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', padding: '0.2rem 0' }}>
        <span style={{ flex: '1 1 12rem', fontWeight: 600 }}>
          {line.chooseOne ? line.line : line.key}
          {line.x2 ? ' (x2)' : ''}
          {line.cultural ? <span style={{ ...MUTED_STYLE, display: 'inline' }}> Cultural Origin, 4 free</span> : null}
        </span>
        {line.chooseOne ? (
          <input
            id={`${id}-specialty`}
            aria-label={`${line.line}: name it`}
            style={{ ...INPUT_STYLE, width: '10rem' }}
            value={draft.skills.specialties[line.line] ?? ''}
            onChange={(event) => setDraft(setSkillSpecialty(draft, line.line, event.target.value))}
          />
        ) : null}
        {line.adjustable ? (
          <button type="button" style={SMALL_BUTTON_STYLE} aria-label={`-1 ${line.key}`} onClick={() => setDraft(adjustSkill(draft, line, -1))}>
            -1
          </button>
        ) : null}
        <span id={`${id}-level`} style={{ minWidth: '1.5rem', textAlign: 'center' }}>
          {line.level}
        </span>
        {line.adjustable ? (
          <button type="button" style={SMALL_BUTTON_STYLE} aria-label={`+1 ${line.key}`} onClick={() => setDraft(adjustSkill(draft, line, 1))}>
            +1
          </button>
        ) : null}
        {droppable ? (
          <button type="button" style={SMALL_BUTTON_STYLE} onClick={() => setDraft(dropSkill(draft, line.line))}>
            Drop
          </button>
        ) : null}
      </div>
    );
  }

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
          money: kitRowTiles(result.explanations),
          language: culturalLanguageTile({
            culturalLanguage: preview?.culturalLanguage ?? null,
            skills: preview?.skills ?? { lines: [], rows: [] },
            explanations: result.explanations,
          }),
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

      {preview && preview.skills.lines.length > 0 ? (
        <fieldset id="cpr-skills" style={{ ...FIELDSET_STYLE, marginTop: '1rem' }}>
          <legend style={LEGEND_STYLE}>Skills</legend>
          <p style={MUTED_STYLE}>
            {draft.method === 'streetrat'
              ? "Your Role's skills at their listed levels."
              : draft.method === 'edgerunner'
                ? "Spread 86 points over your Role's 20 skills, each from 2 to 6; (x2) skills cost 2 points a level."
                : 'Spend 86 points on any skills, none above 6, the Basic skills at 2 or more; (x2) skills cost 2 points a level.'}
          </p>
          {pointTiles.length > 0 ? (
            <div style={{ margin: '0.5rem 0', maxWidth: 360 }}>
              <TileGrid tiles={pointTiles} />
            </div>
          ) : null}
          {preview.skills.lines.map((line, index) => skillLine(line, index))}
          {draft.method === 'completePackage' ? (
            <div style={{ alignItems: 'flex-end', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginTop: '0.5rem' }}>
              <div style={{ ...FIELD_STYLE, flex: '1 1 12rem', marginBottom: 0 }}>
                <label style={LABEL_STYLE} htmlFor="cpr-skill-add">
                  Add a skill
                </label>
                <select id="cpr-skill-add" style={INPUT_STYLE} value={newSkill} onChange={(event) => setNewSkill(event.target.value)}>
                  <option value="">— choose a skill —</option>
                  {options.map((option) => (
                    <option key={option.name} value={option.name}>
                      {option.name} ({option.stat}){option.x2 ? ' (x2)' : ''}
                    </option>
                  ))}
                </select>
              </div>
              {chosenOption?.specialtyRequired ? (
                <div style={{ ...FIELD_STYLE, flex: '1 1 10rem', marginBottom: 0 }}>
                  <label style={LABEL_STYLE} htmlFor="cpr-skill-add-specialty">
                    Which one
                  </label>
                  <input id="cpr-skill-add-specialty" style={INPUT_STYLE} value={newSpecialty} onChange={(event) => setNewSpecialty(event.target.value)} />
                </div>
              ) : null}
              <button
                type="button"
                style={SMALL_BUTTON_STYLE}
                disabled={!canAdd}
                onClick={() => {
                  setDraft(addSkill(draft, newKey));
                  setNewSkill('');
                  setNewSpecialty('');
                }}
              >
                Add
              </button>
            </div>
          ) : null}
        </fieldset>
      ) : null}

      {roleAbility ? (
        <fieldset id="cpr-role-ability" style={{ ...FIELDSET_STYLE, marginTop: '1rem' }}>
          <legend style={LEGEND_STYLE}>
            Role Ability: {roleAbility.ability} (Rank {roleAbility.rank})
          </legend>
          {roleTiles.length > 0 ? (
            <div style={{ margin: '0.5rem 0' }}>
              <TileGrid tiles={roleTiles} />
            </div>
          ) : null}
          {roleAbility.menu.points ? (
            <div data-role-points={roleAbility.menu.points.label} style={{ marginTop: '0.5rem' }}>
              <p style={MUTED_STYLE}>
                {roleAbility.menu.points.label}: {roleAbility.menu.points.exact ? 'place all' : 'place up to'} {roleAbility.menu.points.total} (
                {roleAbility.menu.points.sourcePage})
              </p>
              {roleAbility.menu.points.options.map((option) => (
                <div key={option.name} style={{ alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', padding: '0.2rem 0' }}>
                  <span style={{ flex: '1 1 12rem', fontWeight: 600 }} title={option.text}>
                    {option.name}
                  </span>
                  <button
                    type="button"
                    style={SMALL_BUTTON_STYLE}
                    aria-label={`less ${option.name}`}
                    onClick={() => setDraft(adjustRolePoints(draft, option, -1))}
                  >
                    -
                  </button>
                  <span style={{ minWidth: '1.5rem', textAlign: 'center' }}>{draft.roleAbility.points[option.name] ?? 0}</span>
                  <button
                    type="button"
                    style={SMALL_BUTTON_STYLE}
                    aria-label={`more ${option.name}`}
                    onClick={() => setDraft(adjustRolePoints(draft, option, 1))}
                  >
                    +
                  </button>
                </div>
              ))}
            </div>
          ) : null}
          {roleAbility.menu.picks ? (
            <div data-role-picks={roleAbility.menu.picks.label} style={{ marginTop: '0.5rem' }}>
              <p style={MUTED_STYLE}>
                {roleAbility.menu.picks.label}: up to {roleAbility.menu.picks.count} ({roleAbility.menu.picks.sourcePage})
              </p>
              {draft.roleAbility.picks.map((pick, index) => (
                <div key={`${pick.name}@${pick.on ?? ''}#${index}`} style={{ alignItems: 'center', display: 'flex', gap: '0.5rem', padding: '0.2rem 0' }}>
                  <span style={{ flex: '1 1 12rem' }}>
                    {pick.name}
                    {pick.on ? ` @ ${pick.on}` : ''}
                  </span>
                  <button type="button" style={SMALL_BUTTON_STYLE} onClick={() => setDraft(removeRolePick(draft, index))}>
                    Remove
                  </button>
                </div>
              ))}
              <div style={{ alignItems: 'flex-end', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginTop: '0.35rem' }}>
                <div style={{ ...FIELD_STYLE, flex: '1 1 12rem', marginBottom: 0 }}>
                  <label style={LABEL_STYLE} htmlFor="cpr-role-pick">
                    Pick
                  </label>
                  <select
                    id="cpr-role-pick"
                    style={INPUT_STYLE}
                    value={newPick}
                    onChange={(event) => {
                      setNewPick(event.target.value);
                      setNewPickTarget('');
                    }}
                  >
                    <option value="">— choose —</option>
                    {roleAbility.menu.picks.options.map((option) => (
                      <option key={`${option.kind}:${option.name}`} value={option.name} title={option.text}>
                        {option.name}
                      </option>
                    ))}
                  </select>
                </div>
                {pickOption?.needsTarget ? (
                  <div style={{ ...FIELD_STYLE, flex: '1 1 10rem', marginBottom: 0 }}>
                    <label style={LABEL_STYLE} htmlFor="cpr-role-pick-target">
                      {pickOption.kind === 'team_member' ? 'Chart row' : 'On vehicle'}
                    </label>
                    <select id="cpr-role-pick-target" style={INPUT_STYLE} value={newPickTarget} onChange={(event) => setNewPickTarget(event.target.value)}>
                      <option value="">— choose —</option>
                      {pickTargets.map((target) => (
                        <option key={target} value={target}>
                          {target}
                        </option>
                      ))}
                    </select>
                  </div>
                ) : null}
                <button
                  type="button"
                  style={SMALL_BUTTON_STYLE}
                  disabled={!canPick}
                  onClick={() => {
                    setDraft(addRolePick(draft, newPick, pickOption?.needsTarget ? newPickTarget : null));
                    setNewPick('');
                    setNewPickTarget('');
                  }}
                >
                  Add
                </button>
              </div>
            </div>
          ) : null}
          {!roleAbility.menu.points && !roleAbility.menu.picks ? (
            <p style={MUTED_STYLE}>{roleAbility.ability} asks for no choice at creation.</p>
          ) : null}
        </fieldset>
      ) : null}

      {kit ? (
        <fieldset id="cpr-kit" style={{ ...FIELDSET_STYLE, marginTop: '1rem' }}>
          <legend style={LEGEND_STYLE}>Starting kit and cash</legend>
          {moneyTiles.length > 0 ? (
            <div style={{ margin: '0.5rem 0' }}>
              <TileGrid tiles={moneyTiles} />
            </div>
          ) : null}
          <p style={MUTED_STYLE}>
            {kit.method === 'completePackage'
              ? 'Complete Package: buy everything you start with (p105). Fashion is paid from its own budget first; what is left of that budget is lost (p106).'
              : "Your Role's kit (p099, p104): take one option of each \"or\" line. Spend your cash on other items or keep it."}
          </p>
          {kit.lines.map((line, index) => (
            <div
              key={`${line.section}:${index}`}
              data-kit-line={line.text}
              style={{ alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', padding: '0.2rem 0' }}
            >
              <span style={{ flex: '1 1 12rem', fontWeight: line.pickOne ? 600 : 400 }}>{line.text}</span>
              {line.pickOne ? (
                <select
                  id={`cpr-kit-line-${index}`}
                  aria-label={`Pick: ${line.text}`}
                  style={{ ...INPUT_STYLE, width: '14rem' }}
                  value={line.taken}
                  onChange={(event) => setDraft(setKitPick(draft, index, event.target.value))}
                >
                  {line.options.map((option) => (
                    <option key={option.name} value={option.name}>
                      {option.quantity > 1 ? `${option.name} x${option.quantity}` : option.name}
                    </option>
                  ))}
                </select>
              ) : null}
            </div>
          ))}
          {draft.kit.purchases.length > 0 ? (
            <div data-kit-purchases="bought" style={{ marginTop: '0.5rem' }}>
              <p style={MUTED_STYLE}>Bought</p>
              {draft.kit.purchases.map((purchase, index) => {
                const bought = boughtItems[index];
                return (
                  <div
                    key={`${purchase.kind}:${purchase.name}:${purchase.variant ?? ''}#${index}`}
                    style={{ alignItems: 'center', display: 'flex', gap: '0.5rem', padding: '0.2rem 0' }}
                  >
                    <span style={{ flex: '1 1 12rem' }}>
                      {bought ? bought.name : `${purchase.name}${purchase.variant ? ` (${purchase.variant})` : ''}`} x{purchase.quantity}
                    </span>
                    {bought ? <span style={{ minWidth: '4rem', textAlign: 'right' }}>{bought.costEb}eb</span> : null}
                    <button type="button" style={SMALL_BUTTON_STYLE} onClick={() => setDraft(removeKitPurchase(draft, index))}>
                      Remove
                    </button>
                  </div>
                );
              })}
            </div>
          ) : null}
          <div style={{ alignItems: 'flex-end', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginTop: '0.5rem' }}>
            <div style={{ ...FIELD_STYLE, flex: '1 1 9rem', marginBottom: 0 }}>
              <label style={LABEL_STYLE} htmlFor="cpr-shop-kind">
                Shop
              </label>
              <select
                id="cpr-shop-kind"
                style={INPUT_STYLE}
                value={shopKind}
                onChange={(event) => {
                  setShopKind(event.target.value);
                  setShopName('');
                  setShopVariant('');
                }}
              >
                <option value="">— choose a list —</option>
                {kinds.map((k) => (
                  <option key={k.kind} value={k.kind}>
                    {k.label}
                  </option>
                ))}
              </select>
            </div>
            {shopKind !== '' ? (
              <div style={{ ...FIELD_STYLE, flex: '2 1 14rem', marginBottom: 0 }}>
                <label style={LABEL_STYLE} htmlFor="cpr-shop-item">
                  Item
                </label>
                <select
                  id="cpr-shop-item"
                  style={INPUT_STYLE}
                  value={shopName}
                  onChange={(event) => {
                    setShopName(event.target.value);
                    setShopVariant('');
                  }}
                >
                  <option value="">— choose an item —</option>
                  {shopItems.map((item) => (
                    <option key={item.name} value={item.name}>
                      {item.variantLabel === 'Piece' ? item.name : `${item.name} (${item.costText})`}
                    </option>
                  ))}
                </select>
              </div>
            ) : null}
            {shopItem && shopItem.variantLabel !== null ? (
              <div style={{ ...FIELD_STYLE, flex: '1 1 9rem', marginBottom: 0 }}>
                <label style={LABEL_STYLE} htmlFor="cpr-shop-variant" title={shopItem.variantNote}>
                  {shopItem.variantLabel}
                </label>
                <select id="cpr-shop-variant" style={INPUT_STYLE} value={shopVariant} onChange={(event) => setShopVariant(event.target.value)}>
                  <option value="">— choose —</option>
                  {shopItem.variants.map((variant) => (
                    <option key={variant.name} value={variant.name}>
                      {variant.costEb === null ? variant.name : `${variant.name} (${variant.costEb}eb)`}
                    </option>
                  ))}
                </select>
              </div>
            ) : null}
            {shopItem ? (
              <div style={{ ...FIELD_STYLE, flex: '0 1 6rem', marginBottom: 0 }}>
                <label style={LABEL_STYLE} htmlFor="cpr-shop-quantity">
                  How many
                </label>
                <input
                  id="cpr-shop-quantity"
                  type="number"
                  min={1}
                  style={INPUT_STYLE}
                  value={shopQuantity}
                  onChange={(event) => setShopQuantity(event.target.value)}
                />
              </div>
            ) : null}
            <button
              type="button"
              style={SMALL_BUTTON_STYLE}
              disabled={!canBuy}
              onClick={() => {
                if (!shopItem) {
                  return;
                }
                setDraft(
                  addKitPurchase(draft, {
                    kind: shopItem.kind,
                    name: shopItem.name,
                    variant: shopItem.variantLabel === null ? null : shopVariant,
                    quantity,
                  })
                );
                setShopName('');
                setShopVariant('');
                setShopQuantity('1');
              }}
            >
              Buy
            </button>
          </div>
          {shopItem?.variantNote ? <p style={{ ...MUTED_STYLE, marginTop: '0.35rem' }}>{shopItem.variantNote}</p> : null}
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
                {outcome.money.length > 0 ? <TileGrid tiles={outcome.money} /> : null}
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

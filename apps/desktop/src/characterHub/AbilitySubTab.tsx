import { createContext, useContext, useEffect, useState } from 'react';
import type { AdmittedRecordDto, OverrideFlagDto } from '../boundary/loadSavedCharacterDetail';
import { OverrideTag, flagsOf } from './OverrideTag';
import { listAbilityOptions, type AbilityOptionsDto, type AbilitySubTabId } from '../boundary/abilities';
import { buildAbilitySubTabModel, formatPoolLine, type AbilitySubTabModel } from './abilitySubTabModel';

/** What a sub-tab knows about one option beyond the engine's list: prose, a grants line, choices. */
export interface AbilityOptionInfo {
  description: string | null;
  grants: string | null;
  /** The choices Add asks for first (e.g. a trait's skill); empty when none. */
  choices: { id: string; name: string }[];
}

/**
 * The character's Abilities overrides and what they let in, provided once around the sub-tabs. Off,
 * which is the default, Add stays disabled with the engine's reason and no pick carries a tag.
 */
export interface AbilityOverrides {
  abilitiesWithoutPrerequisites: boolean;
  ignoreAbilityBudget: boolean;
  admitted: readonly AdmittedRecordDto[];
}
export const AbilityOverridesContext = createContext<AbilityOverrides>({ abilitiesWithoutPrerequisites: false, ignoreAbilityBudget: false, admitted: [] });

/** Racial Abilities are the race picker's, not an `ability_budget` sub-tab: the two flags do not reach them. */
const OVERRIDABLE = (subtab: string): boolean => subtab !== 'racial_abilities';

const label = { color: 'var(--color-text-muted)', fontSize: '0.66rem', letterSpacing: '0.06em', margin: '0 0 0.4rem', textTransform: 'uppercase' } as const;
const muted = { color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.25rem 0 0' } as const;

/**
 * The generic Abilities sub-tab (SD-43 E3.3), PCGen's layout: Available (search, a Qualified
 * filter, unqualified options drawn muted and struck), an Info pane for the focused option, and
 * Selected. The budget line is the engine's rows; Add is disabled with the engine's reason.
 */
export function AbilitySubTabView(props: {
  title: string;
  model: AbilitySubTabModel;
  search: string;
  qualifiedOnly: boolean;
  focusedId: string | null;
  info: (id: string) => AbilityOptionInfo | null;
  busy: boolean;
  error: string | null;
  /** The flags that let each held pick in, by option id. */
  flagsOfPick?: (id: string) => OverrideFlagDto[];
  onSearch: (search: string) => void;
  onQualifiedOnly: (on: boolean) => void;
  onFocus: (id: string) => void;
  onAdd: (id: string, choice: string | null) => void;
  onRemove: (id: string) => void;
}) {
  const [choice, setChoice] = useState('');
  const focused = props.model.available.find((row) => row.id === props.focusedId) ?? null;
  const focusedInfo = focused ? props.info(focused.id) : null;
  const needsChoice = (focusedInfo?.choices.length ?? 0) > 0;
  const blockedBy = focused?.addBlockedBy ?? (needsChoice && choice === '' ? 'Choose one first.' : null);

  return (
    <div style={{ marginTop: '0.5rem' }}>
      <p role="status" style={{ fontSize: '0.8rem', fontVariantNumeric: 'tabular-nums', margin: '0 0 0.75rem', textAlign: 'center' }}>
        {props.model.pools.map((pool) => (
          <span key={pool.id} title={pool.detail} style={{ marginRight: '1rem' }}>
            {formatPoolLine(pool)}
          </span>
        ))}
      </p>
      {props.error ? <p role="alert" style={{ color: 'var(--color-danger, #b42318)', fontSize: '0.78rem', margin: '0 0 0.75rem' }}>{props.error}</p> : null}
      <div style={{ display: 'grid', gap: '1rem', gridTemplateColumns: 'minmax(0, 1fr) minmax(0, 1fr)' }}>
        <section aria-label={`Available ${props.title}`}>
          <p style={label}>Available</p>
          <div style={{ alignItems: 'center', display: 'flex', gap: '0.6rem', marginBottom: '0.5rem' }}>
            <input type="search" aria-label={`Search ${props.title}`} value={props.search} onChange={(event) => props.onSearch(event.target.value)} style={{ flex: 1 }} />
            <label style={{ fontSize: '0.75rem', whiteSpace: 'nowrap' }}>
              <input type="checkbox" checked={props.qualifiedOnly} onChange={(event) => props.onQualifiedOnly(event.target.checked)} /> Qualified
            </label>
          </div>
          <ul style={{ border: '1px solid var(--color-border)', listStyle: 'none', margin: 0, maxHeight: '22rem', overflowY: 'auto', padding: 0 }}>
            {props.model.available.map((row) => (
              <li key={row.id} data-qualified={String(row.qualified)}>
                <button
                  type="button"
                  aria-pressed={row.id === props.focusedId}
                  onClick={() => {
                    setChoice('');
                    props.onFocus(row.id);
                  }}
                  style={{
                    background: row.id === props.focusedId ? 'var(--color-surface-raised, rgba(127,127,127,0.15))' : 'none',
                    border: 'none',
                    color: row.qualified ? 'var(--color-text)' : 'var(--color-text-faint)',
                    cursor: 'pointer',
                    display: 'flex',
                    fontStyle: row.qualified ? 'normal' : 'italic',
                    justifyContent: 'space-between',
                    padding: '0.3rem 0.5rem',
                    textAlign: 'left',
                    textDecoration: row.qualified ? 'none' : 'line-through',
                    width: '100%',
                  }}
                >
                  <span>{row.label}</span>
                  <span style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem' }}>{row.category ?? row.pool}</span>
                </button>
              </li>
            ))}
          </ul>
        </section>
        <section aria-label={`${props.title} info`}>
          <p style={label}>Info</p>
          {focused === null ? (
            <p style={muted}>Pick an option on the left to read it.</p>
          ) : (
            <div>
              <div style={{ fontWeight: 700 }}>{focused.label}</div>
              <p style={muted}>
                {focused.category ? `${focused.category} · ` : ''}
                {focused.pool} pool · {focused.qualified ? 'qualified' : 'not qualified'}
              </p>
              {focused.unmet.map((reason) => (
                <p key={reason} style={{ ...muted, color: 'var(--color-danger, #b42318)' }}>{reason}</p>
              ))}
              {focusedInfo?.grants ? <p style={{ fontSize: '0.78rem', margin: '0.4rem 0 0' }}>{focusedInfo.grants}</p> : null}
              <p style={muted}>{focusedInfo?.description ?? focused.description ?? `The bundled roster carries no rules text for ${focused.label}; its name, pool and prerequisites above are the engine's.`}</p>
              <div style={{ alignItems: 'center', display: 'flex', gap: '0.5rem', marginTop: '0.6rem' }}>
                {needsChoice ? (
                  <select aria-label={`Choice for ${focused.label}`} value={choice} onChange={(event) => setChoice(event.target.value)}>
                    <option value="">Choose…</option>
                    {focusedInfo?.choices.map((option) => (
                      <option key={option.id} value={option.id}>{option.name}</option>
                    ))}
                  </select>
                ) : null}
                <button type="button" disabled={props.busy || blockedBy !== null} title={blockedBy ?? undefined} onClick={() => props.onAdd(focused.id, choice || null)}>
                  Add
                </button>
              </div>
              {blockedBy && focused.qualified ? <p style={muted}>{blockedBy}</p> : null}
            </div>
          )}
        </section>
      </div>
      <section aria-label={`Selected ${props.title}`} style={{ marginTop: '1rem' }}>
        <p style={label}>Selected</p>
        {props.model.selected.length === 0 ? (
          <p style={{ color: 'var(--color-text-faint)', margin: 0, textAlign: 'center' }}>No {props.title.toLowerCase()} selected.</p>
        ) : (
          props.model.selected.map((row) => {
            const info = props.info(row.id);
            return (
              <div key={row.id} style={{ alignItems: 'baseline', borderBottom: '1px solid var(--color-border)', display: 'flex', gap: '0.6rem', justifyContent: 'space-between', padding: '0.5rem 0' }}>
                <div style={{ flex: 1 }}>
                  <span style={{ fontWeight: 700 }}>{row.label}</span>
                  {row.category ? <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', marginLeft: '0.5rem' }}>{row.category}</span> : null}
                  {(props.flagsOfPick?.(row.id) ?? []).map((flag) => (
                    <OverrideTag key={flag} flag={flag} />
                  ))}
                  {info?.grants ? <p style={muted}>{info.grants}</p> : null}
                </div>
                <button type="button" disabled={props.busy} onClick={() => props.onRemove(row.id)} style={{ flexShrink: 0 }}>
                  Remove
                </button>
              </div>
            );
          })
        )}
      </section>
    </div>
  );
}

/**
 * Loads a sub-tab's engine options and budget rows (`list_ability_options`) and re-reads them
 * whenever the held picks change; Add and Remove are the sheet's, which save and republish.
 */
export function AbilitySubTab(props: {
  characterId: string;
  subtab: AbilitySubTabId;
  title: string;
  /** The held ids as the sheet knows them; when omitted, the engine's list says which are held. */
  selectedIds?: readonly string[];
  info: (id: string) => AbilityOptionInfo | null;
  onAdd: (id: string, choice: string | null) => Promise<void>;
  onRemove: (id: string) => Promise<void>;
}) {
  const [listed, setListed] = useState<AbilityOptionsDto | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState('');
  const [qualifiedOnly, setQualifiedOnly] = useState(false);
  const [focusedId, setFocusedId] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  // Bumped after every Add/Remove so a sub-tab whose held ids come from the engine re-reads them.
  const [changes, setChanges] = useState(0);
  const overrides = useContext(AbilityOverridesContext);
  const overridable = OVERRIDABLE(props.subtab);
  const heldKey = props.selectedIds?.join('\n') ?? '';

  useEffect(() => {
    let cancelled = false;
    listAbilityOptions(props.characterId, props.subtab)
      .then((result) => {
        if (!cancelled) {
          setListed(result);
          setError(null);
        }
      })
      .catch((cause: unknown) => {
        if (!cancelled) setError(cause instanceof Error ? cause.message : String(cause));
      });
    return () => {
      cancelled = true;
    };
  }, [props.characterId, props.subtab, heldKey, changes]);

  const model = buildAbilitySubTabModel({
    subtab: props.subtab,
    budget: listed?.budget ?? [],
    options: listed?.options ?? [],
    selectedIds: props.selectedIds ?? listed?.selected ?? [],
    search,
    qualifiedOnly,
    allowUnqualified: overridable && overrides.abilitiesWithoutPrerequisites,
    ignoreBudget: overridable && overrides.ignoreAbilityBudget,
  });
  const run = (action: Promise<void>) => {
    setBusy(true);
    void action.finally(() => {
      setBusy(false);
      setChanges((count) => count + 1);
    });
  };

  if (listed === null && error === null) {
    return <p style={{ color: 'var(--color-text-faint)', textAlign: 'center' }}>Loading {props.title.toLowerCase()}…</p>;
  }
  return (
    <AbilitySubTabView
      title={props.title}
      model={model}
      search={search}
      qualifiedOnly={qualifiedOnly}
      focusedId={focusedId}
      info={props.info}
      busy={busy}
      error={error}
      flagsOfPick={(id) => flagsOf(overrides.admitted, 'ability', id)}
      onSearch={setSearch}
      onQualifiedOnly={setQualifiedOnly}
      onFocus={setFocusedId}
      onAdd={(id, choice) => run(props.onAdd(id, choice))}
      onRemove={(id) => run(props.onRemove(id))}
    />
  );
}

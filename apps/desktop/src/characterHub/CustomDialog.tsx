import { useEffect, useState } from 'react';
import { createPortal } from 'react-dom';
import {
  CUSTOM_RECORD_KINDS,
  GRANT_TARGETS,
  customProblems,
  newGrant,
  newRecord,
  type CharacterCustom,
  type CustomGrant,
  type CustomRecord,
  type CustomRecordKind,
} from './customModel';

type Section = 'grants' | CustomRecordKind;

/**
 * The Custom dialog (Create and the sheet's menu): the GM's grants (a +1 Wisdom from a god, extra
 * hit points or skill points) and house-rule records (custom feats, equipment, spells and magic
 * devices, each started from a blank object and given stats and a description). The caller owns
 * saving; this component edits a draft and hands it back on Save.
 */
export function CustomDialog(props: {
  open: boolean;
  value: CharacterCustom;
  /** Shown above the buttons when the caller's save failed. */
  saveError?: string | null;
  saving?: boolean;
  onSave: (custom: CharacterCustom) => void;
  onCancel: () => void;
}) {
  const [draft, setDraft] = useState<CharacterCustom>(props.value);
  const [section, setSection] = useState<Section>('grants');
  const [showProblems, setShowProblems] = useState(false);

  useEffect(() => {
    if (!props.open) {
      return undefined;
    }
    setDraft(props.value);
    setSection('grants');
    setShowProblems(false);
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        props.onCancel();
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [props.open]);

  if (!props.open) {
    return null;
  }

  const problems = customProblems(draft);
  const newId = () => crypto.randomUUID();
  const countFor = (key: Section) => (key === 'grants' ? draft.grants.length : draft[key].length);

  function setGrant(index: number, patch: Partial<CustomGrant>) {
    setDraft({ ...draft, grants: draft.grants.map((grant, position) => (position === index ? { ...grant, ...patch } : grant)) });
  }
  function setRecord(kind: CustomRecordKind, index: number, patch: Partial<CustomRecord>) {
    setDraft({ ...draft, [kind]: draft[kind].map((record, position) => (position === index ? { ...record, ...patch } : record)) });
  }

  const kind = CUSTOM_RECORD_KINDS.find((entry) => entry.kind === section);

  return createPortal(
    <div
      role="presentation"
      onClick={props.onCancel}
      style={{ alignItems: 'center', backgroundColor: 'rgba(0,0,0,0.6)', display: 'flex', inset: 0, justifyContent: 'center', position: 'fixed', zIndex: 60 }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Custom"
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 12,
          display: 'flex',
          flexDirection: 'column',
          gap: '0.7rem',
          height: 'min(760px, 92vh)',
          padding: '1.1rem 1.4rem',
          width: 'min(820px, 95vw)',
        }}
      >
        <h2 style={{ fontSize: '1.1rem', margin: 0 }}>Custom</h2>
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: 0 }}>
          GM grants and house rules. An ability grant changes the saved score, so AC, saves, attacks, skills and hit points follow. Hit point
          and skill point grants are added to those totals. Custom feats, equipment, spells and devices are listed and printed on the sheet;
          the rules engine does not compute from them.
        </p>
        <div role="tablist" style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem' }}>
          {([{ key: 'grants' as Section, label: 'Grants' }, ...CUSTOM_RECORD_KINDS.map((entry) => ({ key: entry.kind as Section, label: entry.plural }))]).map((tab) => (
            <button
              key={tab.key}
              type="button"
              role="tab"
              aria-selected={section === tab.key}
              onClick={() => setSection(tab.key)}
              style={{ ...buttonStyle, ...(section === tab.key ? { backgroundColor: 'var(--color-accent)', color: 'var(--color-on-accent)' } : {}) }}
            >
              {tab.label} ({countFor(tab.key)})
            </button>
          ))}
        </div>

        <div style={{ flex: 1, overflowY: 'auto' }}>
          {section === 'grants' ? (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '0.6rem' }}>
              {draft.grants.length === 0 ? <p style={mutedStyle}>No grants yet. A grant is a number the GM changed: +1 Wisdom from a god, bonus hit points, extra skill points.</p> : null}
              {draft.grants.map((grant, index) => (
                <div key={grant.id} style={cardStyle}>
                  <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem' }}>
                    <input aria-label="Grant label" placeholder="What is it for? (Boon of Pharasma)" value={grant.label} onChange={(event) => setGrant(index, { label: event.target.value })} style={{ ...inputStyle, flex: 2, minWidth: 200 }} />
                    <select aria-label="Grant target" value={grant.target} onChange={(event) => setGrant(index, { target: event.target.value })} style={inputStyle}>
                      {GRANT_TARGETS.map((option) => (
                        <option key={option.value} value={option.value}>{option.label}</option>
                      ))}
                    </select>
                    <input aria-label="Grant value" type="number" value={grant.value} onChange={(event) => setGrant(index, { value: Number.parseInt(event.target.value, 10) || 0 })} style={{ ...inputStyle, width: 80 }} />
                    <button type="button" onClick={() => setDraft({ ...draft, grants: draft.grants.filter((_, position) => position !== index) })} style={buttonStyle}>Remove</button>
                  </div>
                  <input aria-label="Grant reason" placeholder="Why (the quest, the god, the house rule)" value={grant.reason} onChange={(event) => setGrant(index, { reason: event.target.value })} style={{ ...inputStyle, marginTop: '0.4rem', width: '100%' }} />
                </div>
              ))}
              <div>
                <button type="button" onClick={() => setDraft({ ...draft, grants: [...draft.grants, newGrant(newId())] })} style={buttonStyle}>Add grant</button>
              </div>
            </div>
          ) : kind !== undefined ? (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '0.6rem' }}>
              {draft[kind.kind].length === 0 ? <p style={mutedStyle}>No custom {kind.plural.toLowerCase()} yet. Start from a blank one and give it stats and a description.</p> : null}
              {draft[kind.kind].map((record, index) => (
                <div key={record.id} style={cardStyle}>
                  <div style={{ display: 'flex', gap: '0.5rem' }}>
                    <input aria-label={`Custom ${kind.singular} name`} placeholder="Name" value={record.name} onChange={(event) => setRecord(kind.kind, index, { name: event.target.value })} style={{ ...inputStyle, flex: 1 }} />
                    <button type="button" onClick={() => setDraft({ ...draft, [kind.kind]: draft[kind.kind].filter((_, position) => position !== index) })} style={buttonStyle}>Remove</button>
                  </div>
                  <textarea aria-label={`Custom ${kind.singular} description`} placeholder="Description" rows={3} value={record.description} onChange={(event) => setRecord(kind.kind, index, { description: event.target.value })} style={{ ...inputStyle, marginTop: '0.4rem', resize: 'vertical', width: '100%' }} />
                  {record.stats.map((stat, statIndex) => (
                    <div key={statIndex} style={{ display: 'flex', gap: '0.4rem', marginTop: '0.4rem' }}>
                      <input aria-label="Stat label" placeholder="Stat (Damage)" value={stat.label} onChange={(event) => setRecord(kind.kind, index, { stats: record.stats.map((entry, position) => (position === statIndex ? { ...entry, label: event.target.value } : entry)) })} style={{ ...inputStyle, flex: 1 }} />
                      <input aria-label="Stat value" placeholder="Value (1d8 fire)" value={stat.value} onChange={(event) => setRecord(kind.kind, index, { stats: record.stats.map((entry, position) => (position === statIndex ? { ...entry, value: event.target.value } : entry)) })} style={{ ...inputStyle, flex: 2 }} />
                      <button type="button" aria-label="Remove stat" onClick={() => setRecord(kind.kind, index, { stats: record.stats.filter((_, position) => position !== statIndex) })} style={buttonStyle}>×</button>
                    </div>
                  ))}
                  <button type="button" onClick={() => setRecord(kind.kind, index, { stats: [...record.stats, { label: '', value: '' }] })} style={{ ...buttonStyle, marginTop: '0.4rem' }}>Add stat</button>
                </div>
              ))}
              <div>
                <button type="button" onClick={() => setDraft({ ...draft, [kind.kind]: [...draft[kind.kind], newRecord(newId())] })} style={buttonStyle}>Add custom {kind.singular}</button>
              </div>
            </div>
          ) : null}
        </div>

        {showProblems && problems.length > 0 ? (
          <ul role="alert" style={{ color: 'var(--color-error)', fontSize: '0.8rem', margin: 0, paddingLeft: '1.1rem' }}>
            {[...new Set(problems)].map((problem) => (
              <li key={problem}>{problem}</li>
            ))}
          </ul>
        ) : null}
        {props.saveError ? <p role="alert" style={{ color: 'var(--color-error)', fontSize: '0.8rem', margin: 0 }}>{props.saveError}</p> : null}
        <div style={{ display: 'flex', gap: '0.6rem', justifyContent: 'flex-end' }}>
          <button type="button" onClick={props.onCancel} style={buttonStyle}>Cancel</button>
          <button
            type="button"
            disabled={props.saving === true}
            onClick={() => {
              if (problems.length > 0) {
                setShowProblems(true);
                return;
              }
              props.onSave(draft);
            }}
            style={{ ...buttonStyle, backgroundColor: 'var(--color-accent)', color: 'var(--color-on-accent)' }}
          >
            {props.saving === true ? 'Saving…' : 'Save'}
          </button>
        </div>
      </div>
    </div>,
    document.body
  );
}

const buttonStyle = {
  background: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.8rem',
  fontWeight: 600,
  padding: '0.3rem 0.8rem',
} as const;

const inputStyle = {
  boxSizing: 'border-box',
  background: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  fontSize: '0.85rem',
  padding: '0.3rem 0.5rem',
} as const;

const cardStyle = { border: '1px solid var(--color-border)', borderRadius: 8, padding: '0.6rem 0.7rem' } as const;
const mutedStyle = { color: 'var(--color-text-muted)', fontSize: '0.82rem', margin: 0 } as const;

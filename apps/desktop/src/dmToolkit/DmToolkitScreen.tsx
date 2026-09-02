import { useMemo, useState, type CSSProperties } from 'react';
import { getCampaigns, type Campaign } from '../campaign/campaignModel';
import { DM_KIND_FIELDS, DM_KIND_LABELS, DM_RECORD_KINDS, getDmRecords, type DmRecord, type DmRecordKind } from './dmRecordModel';
import { buildDmConsoleView, nextSelectionForKind } from './dmConsoleModel';
import { DM_LINK_LABELS, backlinksTo, createDmRecord, deleteDmRecord, linkDmRecords, unlinkDmRecords, updateDmRecord } from './dmRecordModel';
import { linkCandidates, resolveLinks } from './dmLinksModel';
import { historyFor } from './dmHistoryModel';
import { renderDmConsoleHtml } from './dmConsoleExport';
import { DESKTOP_DM_CONSOLE_EXPORT_DEPS, runDmConsoleExport } from './dmConsoleExportFlow';
import { EncounterBuilderScreen } from './EncounterBuilderScreen';
import { draftFromRecord, draftToInput, emptyDraft, validateDraft, type DmRecordDraft } from './dmRecordForm';

/**
 * DM Toolkit console (v0.8 D-2): NoDA's master/detail shape — six tabs,
 * a search box per tab, the record list on the left and one record on the
 * right — over the real per-campaign store in `dmRecordModel.ts`, in the
 * app's own theme tokens. Everything shown is read from storage; an empty
 * kind says so rather than showing sample rows.
 */

const mono: CSSProperties = { fontFamily: 'ui-monospace, monospace', fontWeight: 800, letterSpacing: '0.12em', textTransform: 'uppercase' };

const pane: CSSProperties = { border: '1px solid var(--color-border)', borderRadius: 12, display: 'flex', flexDirection: 'column', minHeight: 0 };

export function DmToolkitScreen(props: { onBack: () => void }) {
  const [campaigns] = useState<Campaign[]>(() => getCampaigns());
  const [campaignId, setCampaignId] = useState<string | null>(() => campaigns[0]?.id ?? null);
  const [kind, setKind] = useState<DmRecordKind>('World');
  const [queries, setQueries] = useState<Record<DmRecordKind, string>>({ World: '', Timeline: '', Place: '', Person: '', Clue: '', Scene: '', Rule: '' });
  const [selectedId, setSelectedId] = useState<string | null>(null);
  // Bumped after every store write so `records` re-reads storage — what
  // the console shows is always what would survive a reload.
  const [revision, setRevision] = useState(0);
  /** The open form: a new record of `kind`, or an edit of `recordId`. */
  const [editing, setEditing] = useState<{ recordId: string | null; draft: DmRecordDraft } | null>(null);
  const [formErrors, setFormErrors] = useState<string[]>([]);
  const [exporting, setExporting] = useState(false);
  /** v0.8 E-2: the encounter builder lives beside the console, same top-level screen. */
  const [showEncounters, setShowEncounters] = useState(false);
  const [exportStatus, setExportStatus] = useState<{ tone: 'ok' | 'error'; text: string } | null>(null);

  const records = useMemo(() => (campaignId ? getDmRecords(campaignId) : []), [campaignId, revision]);
  const view = buildDmConsoleView(records, kind, queries[kind], selectedId);
  const campaign = campaigns.find((candidate) => candidate.id === campaignId) ?? null;

  function switchKind(next: DmRecordKind) {
    setKind(next);
    setSelectedId(nextSelectionForKind(records, next, selectedId));
    setEditing(null);
  }

  function focusRecord(id: string) {
    const target = records.find((record) => record.id === id);
    if (!target) {
      return;
    }
    setEditing(null);
    setKind(target.kind);
    setSelectedId(target.id);
  }
  function addLink(sourceId: string, targetId: string, label: string) {
    if (!campaignId) {
      return;
    }
    linkDmRecords(campaignId, sourceId, targetId, label);
    setRevision((n) => n + 1);
  }
  function removeLink(sourceId: string, targetId: string) {
    if (!campaignId) {
      return;
    }
    unlinkDmRecords(campaignId, sourceId, targetId);
    setRevision((n) => n + 1);
  }

  async function exportConsole(audience: 'gm' | 'players') {
    if (!campaign) {
      return;
    }
    setExporting(true);
    setExportStatus(null);
    try {
      // Rendered from a fresh storage read, so the file is what a reload would show.
      const outcome = await runDmConsoleExport(
        {
          campaignName: audience === 'players' ? `${campaign.name} player handout` : `${campaign.name} GM`,
          html: renderDmConsoleHtml(campaign.name, getDmRecords(campaign.id), audience),
        },
        DESKTOP_DM_CONSOLE_EXPORT_DEPS,
      );
      if (outcome.kind === 'Exported') {
        setExportStatus({ tone: 'ok', text: outcome.message });
      } else if (outcome.kind === 'Failed') {
        setExportStatus({ tone: 'error', text: outcome.message });
      }
    } finally {
      setExporting(false);
    }
  }

  function startCreate() {
    setFormErrors([]);
    setEditing({ recordId: null, draft: emptyDraft(kind) });
  }
  function startEdit(record: DmRecord) {
    setFormErrors([]);
    setEditing({ recordId: record.id, draft: draftFromRecord(record) });
  }
  function saveForm() {
    if (!campaignId || !editing) {
      return;
    }
    const errors = validateDraft(editing.draft);
    if (errors.length > 0) {
      setFormErrors(errors);
      return;
    }
    const input = draftToInput(editing.draft);
    const saved = editing.recordId === null ? createDmRecord(campaignId, input) : updateDmRecord(campaignId, editing.recordId, input);
    setEditing(null);
    setRevision((n) => n + 1);
    if (saved) {
      setSelectedId(saved.id);
    }
  }
  function removeRecord(record: DmRecord) {
    if (!campaignId || !window.confirm(`Delete “${record.title}”? Links other records hold to it are removed too.`)) {
      return;
    }
    deleteDmRecord(campaignId, record.id);
    setEditing(null);
    setRevision((n) => n + 1);
    setSelectedId(nextSelectionForKind(getDmRecords(campaignId), kind, null));
  }

  if (showEncounters) {
    return <EncounterBuilderScreen onBack={() => setShowEncounters(false)} />;
  }

  return (
    <section style={{ display: 'flex', flexDirection: 'column', height: 'calc(100vh - 6rem)', marginTop: '1rem' }}>
      {/* Top bar: brand, tabs, campaign, back */}
      <div style={{ alignItems: 'center', borderBottom: '1px solid var(--color-border)', display: 'flex', gap: '1.5rem', paddingBottom: '0.75rem' }}>
        <div style={{ minWidth: 200 }}>
          <span style={{ ...mono, border: '2px solid var(--color-accent)', color: 'var(--color-accent)', fontSize: '1rem', padding: '0.1rem 0.5rem' }}>DM</span>
          <span style={{ ...mono, color: 'var(--color-text-muted)', fontSize: '0.62rem', marginLeft: '0.6rem' }}>Campaign console</span>
        </div>
        <nav style={{ display: 'flex', gap: '0.25rem' }}>
          {DM_RECORD_KINDS.map((tab) => {
            const active = tab === kind;
            return (
              <button
                key={tab}
                type="button"
                onClick={() => switchKind(tab)}
                style={{
                  background: 'none',
                  border: 'none',
                  borderBottom: `3px solid ${active ? 'var(--color-accent)' : 'transparent'}`,
                  color: active ? 'var(--color-text)' : 'var(--color-text-muted)',
                  cursor: 'pointer',
                  fontSize: '0.9rem',
                  fontWeight: 700,
                  padding: '0.4rem 0.9rem',
                }}
              >
                {DM_KIND_LABELS[tab].tab}
              </button>
            );
          })}
        </nav>
        <div style={{ alignItems: 'center', display: 'flex', gap: '0.6rem', marginLeft: 'auto' }}>
          {campaigns.length > 1 ? (
            <select
              aria-label="Campaign"
              value={campaignId ?? ''}
              onChange={(event) => {
                setCampaignId(event.target.value);
                setSelectedId(null);
              }}
              style={{ background: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 8, color: 'var(--color-text)', padding: '0.4rem 0.6rem' }}
            >
              {campaigns.map((candidate) => (
                <option key={candidate.id} value={candidate.id}>
                  {candidate.name}
                </option>
              ))}
            </select>
          ) : campaign ? (
            <span style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem' }}>{campaign.name}</span>
          ) : null}
          {campaign ? (
            <>
              <button type="button" onClick={() => void exportConsole('gm')} disabled={exporting} style={{ ...accentButton, fontSize: '0.85rem', padding: '0.4rem 0.9rem' }}>
                {exporting ? 'Exporting…' : 'Export GM copy'}
              </button>
              <button type="button" onClick={() => void exportConsole('players')} disabled={exporting} style={{ ...quietButton, fontSize: '0.85rem', padding: '0.4rem 0.9rem' }}>
                Export player handout
              </button>
            </>
          ) : null}
          <button type="button" onClick={() => setShowEncounters(true)} style={{ ...quietButton, fontSize: '0.85rem', padding: '0.4rem 0.9rem' }}>
            Encounter builder
          </button>
          <button type="button" onClick={props.onBack} style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, color: 'var(--color-text)', cursor: 'pointer', padding: '0.4rem 0.9rem' }}>
            Back
          </button>
        </div>
      </div>

      {exportStatus ? (
        <p style={{ color: exportStatus.tone === 'ok' ? 'var(--color-text-secondary)' : 'var(--color-error)', fontSize: '0.85rem', margin: '0.6rem 0 0' }}>{exportStatus.text}</p>
      ) : null}

      {campaign === null ? (
        <div style={{ ...pane, alignItems: 'center', justifyContent: 'center', marginTop: '1rem', padding: '2rem', flex: 1 }}>
          <p style={{ color: 'var(--color-text-secondary)', margin: 0, textAlign: 'center' }}>
            The console is scoped to a campaign, and there are no campaigns yet. Create one in Campaign Manager first.
          </p>
        </div>
      ) : (
        <div style={{ display: 'grid', flex: 1, gap: '1rem', gridTemplateColumns: '360px minmax(0, 1fr)', marginTop: '1rem', minHeight: 0 }}>
          {/* Master */}
          <div style={pane}>
            <div style={{ alignItems: 'center', borderBottom: '1px solid var(--color-border)', display: 'flex', justifyContent: 'space-between', padding: '0.7rem 1rem' }}>
              <span style={{ ...mono, color: 'var(--color-accent)', fontSize: '0.7rem' }}>{DM_KIND_LABELS[kind].tab}</span>
              <span style={{ alignItems: 'center', display: 'flex', gap: '0.6rem' }}>
                <span style={{ ...mono, color: 'var(--color-text-muted)', fontSize: '0.6rem' }}>{view.countLabel}</span>
                <button type="button" onClick={startCreate} style={accentButton}>
                  + New {DM_KIND_LABELS[kind].singular.toLowerCase()}
                </button>
              </span>
            </div>
            <div style={{ margin: '0.75rem' }}>
              <input
                aria-label={`Search ${DM_KIND_LABELS[kind].tab}`}
                placeholder={`Search ${DM_KIND_LABELS[kind].tab.toLowerCase()}…`}
                value={queries[kind]}
                onChange={(event) => setQueries((prev) => ({ ...prev, [kind]: event.target.value }))}
                style={{ background: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 8, boxSizing: 'border-box', color: 'var(--color-text)', padding: '0.55rem 0.7rem', width: '100%' }}
              />
            </div>
            <div style={{ flex: 1, overflowY: 'auto', padding: '0 0.5rem 1rem' }}>
              {view.emptyMessage !== null ? (
                <p style={{ color: 'var(--color-text-muted)', margin: '1rem', textAlign: 'center' }}>{view.emptyMessage}</p>
              ) : (
                view.list.map((record) => {
                  const active = record.id === view.selected?.id;
                  return (
                    <button
                      key={record.id}
                      type="button"
                      onClick={() => setSelectedId(record.id)}
                      style={{
                        background: active ? 'var(--color-surface)' : 'none',
                        border: '1px solid transparent',
                        borderBottom: '1px solid var(--color-border)',
                        borderLeft: `3px solid ${active ? 'var(--color-accent)' : 'transparent'}`,
                        borderRadius: 6,
                        color: 'var(--color-text)',
                        cursor: 'pointer',
                        display: 'block',
                        padding: '0.65rem 0.75rem',
                        textAlign: 'left',
                        width: '100%',
                      }}
                    >
                      <span style={{ ...mono, color: 'var(--color-text-muted)', display: 'flex', fontSize: '0.55rem', justifyContent: 'space-between', gap: '0.5rem' }}>
                        <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{record.summary || DM_KIND_LABELS[record.kind].singular}</span>
                        <VisibilityBadge visibility={record.visibility} />
                      </span>
                      <strong style={{ display: 'block', fontSize: '0.85rem', marginTop: '0.25rem' }}>{record.title}</strong>
                    </button>
                  );
                })
              )}
            </div>
          </div>

          {/* Detail */}
          <div style={{ ...pane, overflowY: 'auto', padding: '2rem 2.5rem' }}>
            {editing !== null ? (
              <RecordForm
                draft={editing.draft}
                isNew={editing.recordId === null}
                errors={formErrors}
                onChange={(draft) => setEditing({ recordId: editing.recordId, draft })}
                onSave={saveForm}
                onCancel={() => setEditing(null)}
              />
            ) : view.selected === null ? (
              <p style={{ color: 'var(--color-text-muted)', margin: 'auto', textAlign: 'center' }}>
                {view.list.length === 0 ? view.emptyMessage : 'Select a record on the left.'}
              </p>
            ) : (
              <RecordPane
                record={view.selected}
                records={records}
                onEdit={() => startEdit(view.selected as DmRecord)}
                onDelete={() => removeRecord(view.selected as DmRecord)}
                onFocus={focusRecord}
                onAddLink={(targetId, label) => addLink((view.selected as DmRecord).id, targetId, label)}
                onRemoveLink={(targetId) => removeLink((view.selected as DmRecord).id, targetId)}
              />
            )}
          </div>
        </div>
      )}
    </section>
  );
}

const accentButton: CSSProperties = {
  background: 'var(--color-accent)',
  border: 'none',
  borderRadius: 8,
  color: 'var(--color-on-accent)',
  cursor: 'pointer',
  fontSize: '0.75rem',
  fontWeight: 700,
  padding: '0.35rem 0.7rem',
};
const quietButton: CSSProperties = {
  background: 'none',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.75rem',
  fontWeight: 600,
  padding: '0.35rem 0.7rem',
};
const inputStyle: CSSProperties = {
  background: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  boxSizing: 'border-box',
  color: 'var(--color-text)',
  fontFamily: 'inherit',
  fontSize: '0.9rem',
  padding: '0.55rem 0.7rem',
  width: '100%',
};
const labelStyle: CSSProperties = { ...mono, color: 'var(--color-text-muted)', display: 'block', fontSize: '0.58rem', margin: '0 0 0.3rem' };

/** Create/edit form (v0.8 D-3). Save writes through the store; Cancel discards the draft. */
function RecordForm(props: {
  draft: DmRecordDraft;
  isNew: boolean;
  errors: string[];
  onChange: (draft: DmRecordDraft) => void;
  onSave: () => void;
  onCancel: () => void;
}) {
  const { draft } = props;
  const set = (patch: Partial<DmRecordDraft>) => props.onChange({ ...draft, ...patch });
  return (
    <form
      onSubmit={(event) => {
        event.preventDefault();
        props.onSave();
      }}
      style={{ display: 'grid', gap: '1rem', margin: '0 auto', maxWidth: 860, width: '100%' }}
    >
      <span style={{ ...mono, color: 'var(--color-accent)', fontSize: '0.62rem' }}>
        {props.isNew ? `New ${DM_KIND_LABELS[draft.kind].singular.toLowerCase()}` : `Editing ${DM_KIND_LABELS[draft.kind].singular.toLowerCase()}`}
      </span>
      <label>
        <span style={labelStyle}>Title</span>
        <input autoFocus value={draft.title} onChange={(event) => set({ title: event.target.value })} style={{ ...inputStyle, fontSize: '1.2rem', fontWeight: 700 }} />
      </label>
      <fieldset style={{ border: '1px solid var(--color-border)', borderRadius: 8, margin: 0, padding: '0.6rem 0.8rem' }}>
        <legend style={labelStyle}>Who can see this</legend>
        <label style={{ marginRight: '1.2rem' }}>
          <input type="radio" name="visibility" checked={draft.visibility === 'gm'} onChange={() => set({ visibility: 'gm' })} /> GM only
        </label>
        <label>
          <input type="radio" name="visibility" checked={draft.visibility === 'players'} onChange={() => set({ visibility: 'players' })} /> Players — included in the player handout
        </label>
      </fieldset>
      <label>
        <span style={labelStyle}>One-line summary</span>
        <input value={draft.summary} onChange={(event) => set({ summary: event.target.value })} style={inputStyle} />
      </label>
      {DM_KIND_FIELDS[draft.kind].map((spec) => (
        <label key={spec.key}>
          <span style={labelStyle}>{spec.label}</span>
          {spec.multiline ? (
            <textarea rows={5} value={draft.fields[spec.key] ?? ''} onChange={(event) => set({ fields: { ...draft.fields, [spec.key]: event.target.value } })} style={inputStyle} />
          ) : (
            <input value={draft.fields[spec.key] ?? ''} onChange={(event) => set({ fields: { ...draft.fields, [spec.key]: event.target.value } })} style={inputStyle} />
          )}
        </label>
      ))}
      <label>
        <span style={labelStyle}>Notes (Markdown)</span>
        <textarea rows={10} value={draft.body} onChange={(event) => set({ body: event.target.value })} style={inputStyle} />
      </label>
      {props.errors.map((error) => (
        <p key={error} style={{ color: 'var(--color-error)', margin: 0 }}>
          {error}
        </p>
      ))}
      <div style={{ display: 'flex', gap: '0.6rem' }}>
        <button type="submit" style={accentButton}>
          Save
        </button>
        <button type="button" onClick={props.onCancel} style={quietButton}>
          Cancel
        </button>
      </div>
    </form>
  );
}

function RecordPane(props: {
  record: DmRecord;
  records: DmRecord[];
  onEdit: () => void;
  onDelete: () => void;
  onFocus: (id: string) => void;
  onAddLink: (targetId: string, label: string) => void;
  onRemoveLink: (targetId: string) => void;
}) {
  const { record } = props;
  const links = resolveLinks(record, props.records);
  const backlinks = backlinksTo(props.records, record.id);
  const history = historyFor(record, props.records);
  const fields = DM_KIND_FIELDS[record.kind].filter((spec) => (record.fields[spec.key] ?? '').trim() !== '');
  return (
    <article style={{ margin: '0 auto', maxWidth: 860, width: '100%' }}>
      <header style={{ borderBottom: '1px solid var(--color-border)', paddingBottom: '1.25rem' }}>
        <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between' }}>
          <span style={{ alignItems: 'center', display: 'flex', gap: '0.6rem' }}>
            <span style={{ ...mono, color: 'var(--color-accent)', fontSize: '0.62rem' }}>{DM_KIND_LABELS[record.kind].singular}</span>
            <VisibilityBadge visibility={record.visibility} />
          </span>
          <span style={{ display: 'flex', gap: '0.5rem' }}>
            <button type="button" onClick={props.onEdit} style={quietButton}>
              Edit
            </button>
            <button type="button" onClick={props.onDelete} style={{ ...quietButton, color: 'var(--color-error)' }}>
              Delete
            </button>
          </span>
        </div>
        <h1 style={{ fontSize: '2.1rem', letterSpacing: '-0.03em', lineHeight: 1.1, margin: '0.4rem 0 0.6rem' }}>{record.title}</h1>
        {record.summary ? <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.95rem', lineHeight: 1.6, margin: 0 }}>{record.summary}</p> : null}
      </header>
      {fields.length > 0 ? (
        <dl style={{ margin: '1.5rem 0 0' }}>
          {fields.map((spec) => (
            <div key={spec.key} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.6rem 0' }}>
              <dt style={{ ...mono, color: 'var(--color-text-muted)', fontSize: '0.58rem' }}>{spec.label}</dt>
              <dd style={{ color: 'var(--color-text)', fontSize: '0.85rem', lineHeight: 1.6, margin: '0.3rem 0 0', whiteSpace: 'pre-wrap' }}>{record.fields[spec.key]}</dd>
            </div>
          ))}
        </dl>
      ) : null}
      {record.body.trim() !== '' ? (
        <section style={{ marginTop: '1.75rem' }}>
          <h2 style={sectionHeading}>Notes</h2>
          <p style={{ color: 'var(--color-text)', fontSize: '0.9rem', lineHeight: 1.75, margin: 0, whiteSpace: 'pre-wrap' }}>{record.body}</p>
        </section>
      ) : null}

      {/* v0.8 D-8: derived history — Timeline entries linked to this record,
          in the order they were created. `when` is a label, never parsed. */}
      {history.length > 0 ? (
        <section style={{ marginTop: '1.75rem' }}>
          <h2 style={sectionHeading}>History</h2>
          <ol style={{ borderLeft: '2px solid var(--color-border)', listStyle: 'none', margin: 0, padding: 0 }}>
            {history.map((entry) => (
              <li key={entry.id} style={{ padding: '0 0 0.6rem 1rem', position: 'relative' }}>
                <span style={{ background: 'var(--color-accent)', borderRadius: '50%', height: 8, left: -5, position: 'absolute', top: 6, width: 8 }} />
                <button type="button" onClick={() => props.onFocus(entry.id)} style={{ background: 'none', border: 'none', color: 'var(--color-text)', cursor: 'pointer', padding: 0, textAlign: 'left' }}>
                  <span style={{ ...mono, color: 'var(--color-text-muted)', display: 'block', fontSize: '0.55rem' }}>{(entry.fields.when ?? '').trim() || 'Timeline'}</span>
                  <strong style={{ display: 'block', fontSize: '0.85rem', marginTop: '0.1rem' }}>{entry.title}</strong>
                  {entry.summary ? <small style={{ color: 'var(--color-text-muted)', display: 'block', fontSize: '0.72rem' }}>{entry.summary}</small> : null}
                </button>
              </li>
            ))}
          </ol>
        </section>
      ) : null}

      {/* v0.8 D-4: cross-links. Each button focuses a real record (the
          model drops any link whose target is gone). */}
      <section style={{ marginTop: '1.75rem' }}>
        <h2 style={sectionHeading}>Links</h2>
        {links.length === 0 ? <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', margin: '0 0 0.75rem' }}>No links yet.</p> : null}
        {links.map((link) => (
          <div key={`${link.target.id}:${link.label}`} style={{ display: 'flex', gap: '0.4rem', marginBottom: '0.45rem' }}>
            <button type="button" onClick={() => props.onFocus(link.target.id)} style={linkButton}>
              <span>
                <strong style={{ display: 'block', fontSize: '0.85rem' }}>{link.target.title}</strong>
                <small style={{ color: 'var(--color-text-muted)', display: 'block', fontSize: '0.7rem', marginTop: '0.15rem' }}>
                  {link.target.summary || DM_KIND_LABELS[link.target.kind].singular}
                </small>
              </span>
              <b style={{ ...mono, color: 'var(--color-accent)', fontSize: '0.58rem', whiteSpace: 'nowrap' }}>{link.label}</b>
            </button>
            <button type="button" aria-label={`Remove link to ${link.target.title}`} onClick={() => props.onRemoveLink(link.target.id)} style={{ ...quietButton, padding: '0 0.6rem' }}>
              ×
            </button>
          </div>
        ))}
        <AddLinkControl record={record} candidates={linkCandidates(record, props.records)} onAdd={props.onAddLink} />
      </section>

      {backlinks.length > 0 ? (
        <section style={{ marginTop: '1.75rem' }}>
          <h2 style={sectionHeading}>Linked from</h2>
          {backlinks.map((source) => (
            <button key={source.id} type="button" onClick={() => props.onFocus(source.id)} style={{ ...linkButton, marginBottom: '0.45rem' }}>
              <span>
                <strong style={{ display: 'block', fontSize: '0.85rem' }}>{source.title}</strong>
                <small style={{ color: 'var(--color-text-muted)', display: 'block', fontSize: '0.7rem', marginTop: '0.15rem' }}>
                  {DM_KIND_LABELS[source.kind].singular}
                </small>
              </span>
              <b style={{ ...mono, color: 'var(--color-accent)', fontSize: '0.58rem', whiteSpace: 'nowrap' }}>
                {source.links.filter((link) => link.targetId === record.id).map((link) => link.label).join(' · ')}
              </b>
            </button>
          ))}
        </section>
      ) : null}
    </article>
  );
}

/** At-a-glance visibility (D-3a): what a player handout would carry. */
function VisibilityBadge(props: { visibility: DmRecord['visibility'] }) {
  const shared = props.visibility === 'players';
  return (
    <span
      style={{
        ...mono,
        border: `1px solid ${shared ? 'var(--color-accent)' : 'var(--color-border)'}`,
        borderRadius: 4,
        color: shared ? 'var(--color-accent)' : 'var(--color-text-muted)',
        fontSize: '0.5rem',
        padding: '0.05rem 0.3rem',
        whiteSpace: 'nowrap',
      }}
    >
      {shared ? 'Players' : 'GM only'}
    </span>
  );
}

const sectionHeading: CSSProperties = { ...mono, borderBottom: '1px solid var(--color-border)', color: 'var(--color-text-muted)', fontSize: '0.62rem', margin: '0 0 0.75rem', paddingBottom: '0.4rem' };
const linkButton: CSSProperties = {
  alignItems: 'center',
  background: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  color: 'var(--color-text)',
  cursor: 'pointer',
  display: 'flex',
  flex: 1,
  gap: '1rem',
  justifyContent: 'space-between',
  padding: '0.6rem 0.8rem',
  textAlign: 'left',
};

/** Pick any other record and a relation label; Add writes the link through the store. */
function AddLinkControl(props: { record: DmRecord; candidates: DmRecord[]; onAdd: (targetId: string, label: string) => void }) {
  const labels = DM_LINK_LABELS[props.record.kind];
  const [targetId, setTargetId] = useState('');
  const [label, setLabel] = useState(labels[0]);
  if (props.candidates.length === 0) {
    return <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0.5rem 0 0' }}>Create another record to link to.</p>;
  }
  const byKind = DM_RECORD_KINDS.map((kind) => ({ kind, items: props.candidates.filter((candidate) => candidate.kind === kind) })).filter((group) => group.items.length > 0);
  return (
    <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem', marginTop: '0.6rem' }}>
      <select aria-label="Link target" value={targetId} onChange={(event) => setTargetId(event.target.value)} style={{ ...inputStyle, flex: '1 1 220px', width: 'auto' }}>
        <option value="">Link to…</option>
        {byKind.map((group) => (
          <optgroup key={group.kind} label={DM_KIND_LABELS[group.kind].tab}>
            {group.items.map((candidate) => (
              <option key={candidate.id} value={candidate.id}>
                {candidate.title}
              </option>
            ))}
          </optgroup>
        ))}
      </select>
      <select aria-label="Link label" value={label} onChange={(event) => setLabel(event.target.value)} style={{ ...inputStyle, flex: '0 1 150px', width: 'auto' }}>
        {labels.map((option) => (
          <option key={option} value={option}>
            {option}
          </option>
        ))}
      </select>
      <button
        type="button"
        disabled={targetId === ''}
        onClick={() => {
          props.onAdd(targetId, label);
          setTargetId('');
        }}
        style={{ ...accentButton, opacity: targetId === '' ? 0.5 : 1 }}
      >
        Add link
      </button>
    </div>
  );
}

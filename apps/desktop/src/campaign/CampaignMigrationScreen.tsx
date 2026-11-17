import { useState, type CSSProperties } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { migrateCampaign, type MigratedCampaign } from '../boundary/campaignFolder/migrateCampaign';
import { clearMigrated, legacyMigrationRows } from './campaignMigrationModel';
import { readYourName, rememberYourName } from './campaignMembersModel';

/**
 * Migration of pre-SD-42 campaigns (`decisions.md §14`, OPERATOR Q13). One row per old campaign
 * (this install's localStorage records, plus an older `.config/` folder "Open folder…" picked):
 * choose where the new folder goes, then Migrate. "Later" keeps the old data untouched.
 */

interface Row {
  key: string;
  name: string;
  /** Set for a localStorage record: cleared from localStorage after a successful migration. */
  legacyId: string | null;
  source: { recordJson: string; assetsJson: string | null } | { configDir: string };
}

const button = (primary: boolean, enabled: boolean): CSSProperties => ({
  backgroundColor: primary && enabled ? 'var(--color-accent)' : 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  color: primary && enabled ? 'var(--color-on-accent)' : 'var(--color-text)',
  cursor: enabled ? 'pointer' : 'not-allowed',
  fontWeight: primary ? 700 : 500,
  opacity: enabled ? 1 : 0.5,
  padding: '0.45rem 1.1rem',
});

function initialRows(configDir: string | null): Row[] {
  const rows: Row[] = legacyMigrationRows(localStorage).map((row) => ({
    key: `ls:${row.id}`,
    name: row.name,
    legacyId: row.id,
    source: { recordJson: row.recordJson, assetsJson: row.assetsJson },
  }));
  if (configDir) {
    const name = configDir.split(/[\\/]/).filter(Boolean).pop() ?? configDir;
    rows.push({ key: `dir:${configDir}`, name, legacyId: null, source: { configDir } });
  }
  return rows;
}

export function CampaignMigrationScreen(props: { configDir: string | null; onClose: () => void }) {
  const [rows] = useState(() => initialRows(props.configDir));
  const [yourName, setYourName] = useState(() => readYourName());
  const [parents, setParents] = useState<Record<string, string>>({});
  const [done, setDone] = useState<Record<string, MigratedCampaign>>({});
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [busy, setBusy] = useState<string | null>(null);

  async function pickParent(row: Row) {
    const picked = await open({ directory: true, multiple: false, title: `Where should "${row.name}" go?` });
    if (typeof picked === 'string') {
      setParents((current) => ({ ...current, [row.key]: picked }));
    }
  }

  async function migrate(row: Row) {
    const parent = parents[row.key];
    if (!parent || !yourName.trim()) {
      return;
    }
    setBusy(row.key);
    setErrors((current) => ({ ...current, [row.key]: '' }));
    try {
      const result = await migrateCampaign(row.source, parent, yourName);
      if (row.legacyId) {
        clearMigrated(localStorage, row.legacyId);
      }
      rememberYourName(yourName);
      setDone((current) => ({ ...current, [row.key]: result }));
    } catch (cause: unknown) {
      setErrors((current) => ({ ...current, [row.key]: cause instanceof Error ? cause.message : String(cause) }));
    } finally {
      setBusy(null);
    }
  }

  const allDone = rows.every((row) => done[row.key]);

  return (
    <section data-testid="campaign-migration" style={{ marginTop: '1rem', maxWidth: 900 }}>
      <h2 style={{ margin: '0 0 0.5rem' }}>Migrate older campaigns</h2>
      <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.5, margin: '0 0 1rem' }}>
        A campaign is now a folder of Markdown files. Each older campaign below becomes a folder: pick where it
        goes and press Migrate. Every entry becomes its own file, and the old data is kept in a backup file in
        Codex's own data folder. Nothing is deleted.
      </p>
      <label style={{ display: 'block', margin: '0 0 1rem' }}>
        <span style={{ display: 'block', fontWeight: 700, marginBottom: '0.3rem' }}>Your name (member #1)</span>
        <input value={yourName} onChange={(event) => setYourName(event.target.value)} style={{ padding: '0.4rem', width: 280 }} />
      </label>
      {rows.length === 0 ? <p>No older campaigns to migrate.</p> : null}
      {rows.map((row) => {
        const parent = parents[row.key];
        const result = done[row.key];
        const ready = Boolean(parent && yourName.trim()) && !result && busy === null;
        return (
          <div key={row.key} style={{ border: '1px solid var(--color-border)', borderRadius: 10, marginBottom: '0.75rem', padding: '0.8rem 1rem' }}>
            <div style={{ alignItems: 'center', display: 'flex', gap: '0.6rem' }}>
              <strong style={{ flex: 1 }}>{row.name}</strong>
              {result ? null : (
                <>
                  <button type="button" onClick={() => void pickParent(row)} style={button(false, busy === null)} disabled={busy !== null}>
                    Choose folder…
                  </button>
                  <button type="button" onClick={() => void migrate(row)} style={button(true, ready)} disabled={!ready}>
                    {busy === row.key ? 'Migrating…' : 'Migrate'}
                  </button>
                </>
              )}
            </div>
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0.4rem 0 0', overflowWrap: 'anywhere' }}>
              {result
                ? `Migrated to ${result.campaignDir} (${result.files} files). Backup: ${result.backupFile}`
                : parent
                  ? `New folder: ${parent}/${row.name}/`
                  : 'Choose the folder the new campaign folder goes in.'}
            </p>
            {errors[row.key] ? <p role="alert" style={{ color: 'var(--color-error)', margin: '0.4rem 0 0' }}>{errors[row.key]}</p> : null}
          </div>
        );
      })}
      <div style={{ display: 'flex', gap: '0.6rem', marginTop: '1rem' }}>
        <button type="button" onClick={props.onClose} style={button(allDone, true)}>
          {allDone ? 'Done' : 'Later'}
        </button>
      </div>
    </section>
  );
}

import { useEffect, useState, type CSSProperties } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import {
  addKnownCampaignFolder,
  forgetKnownCampaignFolder,
  listKnownCampaigns,
  openCampaignFolder,
} from '../boundary/campaignFolder/knownCampaigns';
import type { KnownCampaign, OpenedCampaign } from '../boundary/campaignFolder/types';
import { notACampaignFolder } from './campaignFolderModel';
import { CampaignMigrationScreen } from './CampaignMigrationScreen';
import { CampaignHelpButton } from './CampaignHelpScreen';
import { legacyMigrationRows, olderCampaignFolder } from './campaignMigrationModel';
import { ActionBanner, CAMPAIGN_MANAGER_ART } from '../characterHub/LandingScreen';

function hasLegacyCampaigns(): boolean {
  try {
    return legacyMigrationRows(localStorage).length > 0;
  } catch {
    return false;
  }
}

/**
 * Campaign Manager: the known campaign folders (`<app data>/campaign-folders.json`) on the left,
 * each with Forget, the selected folder's `campaign.json` on the right (its typed members, the
 * creator first) under the open banner, and a bottom action bar (New campaign… / Open folder… / Edit / Cancel) —
 * patterned after the Load Character screen. A campaign is identified by its folder path. Older
 * (pre-folder) campaigns open the migration screen first (`decisions.md §14`).
 */

const barButton = (variant: 'primary' | 'default', enabled: boolean): CSSProperties => ({
  backgroundColor: variant === 'primary' && enabled ? 'var(--color-accent)' : 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  color: variant === 'primary' && enabled ? 'var(--color-on-accent)' : 'var(--color-text)',
  cursor: enabled ? 'pointer' : 'not-allowed',
  fontSize: '0.9rem',
  fontWeight: variant === 'primary' ? 700 : 500,
  opacity: enabled ? 1 : 0.5,
  padding: '0.55rem 1.4rem',
});

function formatDate(iso: string): string {
  const parsed = new Date(iso);
  return Number.isNaN(parsed.getTime()) ? iso : parsed.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
}

export const OPEN_CAMPAIGN_HINT = 'Select a campaign to open it';

/** The open button: the main screen's Campaign Manager banner, at the full width of the detail column, shorter (100 px, not 160). */
export function OpenCampaignBanner(props: { opened: OpenedCampaign | null; onOpen: (campaignDir: string) => void }) {
  const { opened } = props;
  return (
    <ActionBanner
      title={opened ? `Open ${opened.campaign.name}` : 'Open campaign'}
      art={CAMPAIGN_MANAGER_ART}
      onClick={() => opened && props.onOpen(opened.campaignDir)}
      disabled={!opened}
      disabledHint={OPEN_CAMPAIGN_HINT}
      minHeight={100}
    />
  );
}

const CAPTION: CSSProperties = { color: 'var(--color-text-muted)', fontSize: '0.7rem', letterSpacing: '0.04em', margin: 0, textTransform: 'uppercase' };

function DetailPane(props: { opened: OpenedCampaign | null; error: string | null; selected: boolean }) {
  if (props.error) {
    return <p style={{ color: 'var(--color-error)', margin: 0 }}>{props.error}</p>;
  }
  if (!props.opened) {
    return (
      <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>
        {props.selected ? 'Opening…' : 'Select a campaign to preview it, then press Open or Edit.'}
      </p>
    );
  }
  const { campaign, campaignDir } = props.opened;
  return (
    <div>
      <div style={{ alignItems: 'baseline', display: 'flex', gap: '1rem', justifyContent: 'space-between' }}>
        <h2 style={{ margin: 0 }}>{campaign.name}</h2>
        <span style={{ color: 'var(--color-text-secondary)', fontWeight: 700 }}>{campaign.ruleSetLabel}</span>
      </div>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0.6rem 0 0.9rem' }}>
        Created {formatDate(campaign.createdAt)} · Updated {formatDate(campaign.updatedAt)}
      </p>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0 0 0.9rem', overflowWrap: 'anywhere' }}>{campaignDir}</p>

      <hr style={{ border: 'none', borderTop: '1px solid var(--color-border)', margin: '0.75rem 0' }} />

      <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0 0 1rem' }}>
        {campaign.description || <span style={{ color: 'var(--color-text-faint)' }}>No description yet.</span>}
      </p>

      <p style={CAPTION}>Members</p>
      <p style={{ fontWeight: 700, margin: '0.2rem 0 0.5rem' }}>{campaign.members.length}</p>
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem' }}>
        {campaign.members.map((member, index) => (
          <span
            key={`${index}-${member.name}`}
            style={{
              backgroundColor: 'var(--color-surface-2)',
              border: '1px solid var(--color-border)',
              borderRadius: 6,
              color: 'var(--color-text-secondary)',
              fontSize: '0.78rem',
              padding: '0.15rem 0.5rem',
            }}
          >
            {member.name}
          </span>
        ))}
      </div>
    </div>
  );
}

export function CampaignManagerScreen(props: {
  onCancel: () => void;
  onCreate: () => void;
  onEdit: (campaignId: string) => void;
  onLoad: (campaignId: string) => void;
}) {
  const [campaigns, setCampaigns] = useState<KnownCampaign[]>([]);
  const [listError, setListError] = useState<string | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [opened, setOpened] = useState<OpenedCampaign | null>(null);
  const [openError, setOpenError] = useState<string | null>(null);
  /** "Open folder…" picked a folder with no campaign.json: offer New instead. */
  const [pickedNotCampaign, setPickedNotCampaign] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);
  /** The migration screen (decisions.md §14): opens by itself while older campaigns exist; "Later" closes it. */
  const [migration, setMigration] = useState<{ configDir: string | null } | null>(() =>
    hasLegacyCampaigns() ? { configDir: null } : null
  );

  function refreshList() {
    listKnownCampaigns()
      .then((known) => {
        setCampaigns(known);
        setListError(null);
      })
      .catch((cause: unknown) => setListError(cause instanceof Error ? cause.message : String(cause)));
  }

  useEffect(refreshList, []);

  async function handleOpenFolder() {
    setActionError(null);
    setPickedNotCampaign(null);
    let picked: string | string[] | null;
    try {
      picked = await open({ directory: true, multiple: false, title: 'Open a campaign folder' });
    } catch (cause: unknown) {
      setActionError(cause instanceof Error ? cause.message : String(cause));
      return;
    }
    if (typeof picked !== 'string') {
      return;
    }
    try {
      const opened = await addKnownCampaignFolder(picked);
      refreshList();
      setSelectedId(opened.campaignDir);
    } catch (cause: unknown) {
      const message = cause instanceof Error ? cause.message : String(cause);
      if (notACampaignFolder(message)) {
        setPickedNotCampaign(picked);
      } else if (olderCampaignFolder(message)) {
        setMigration({ configDir: picked });
      } else {
        setActionError(message);
      }
    }
  }

  async function handleForget(path: string) {
    setActionError(null);
    try {
      await forgetKnownCampaignFolder(path);
    } catch (cause: unknown) {
      setActionError(cause instanceof Error ? cause.message : String(cause));
      return;
    }
    if (selectedId === path) {
      setSelectedId(null);
    }
    refreshList();
  }

  useEffect(() => {
    setOpened(null);
    setOpenError(null);
    if (!selectedId) {
      return;
    }
    let live = true;
    openCampaignFolder(selectedId)
      .then((result) => live && setOpened(result))
      .catch((cause: unknown) => live && setOpenError(cause instanceof Error ? cause.message : String(cause)));
    return () => {
      live = false;
    };
  }, [selectedId]);

  const selected = opened && opened.campaignDir === selectedId ? opened : null;

  if (migration) {
    return (
      <CampaignMigrationScreen
        configDir={migration.configDir}
        onClose={() => {
          setMigration(null);
          refreshList();
        }}
      />
    );
  }

  return (
    <section style={{ display: 'flex', flexDirection: 'column', height: 'calc(100vh - 8rem)', marginTop: '1rem' }}>
      <h2 style={{ margin: '0 0 1rem' }}>Campaign Manager</h2>
      {pickedNotCampaign ? (
        <div role="alert" data-testid="campaign-not-a-folder" style={{ alignItems: 'center', display: 'flex', gap: '0.8rem', margin: '0 0 1rem' }}>
          <span style={{ color: 'var(--color-error)', overflowWrap: 'anywhere' }}>{pickedNotCampaign} is not a campaign folder.</span>
          <button type="button" onClick={props.onCreate} style={barButton('default', true)}>
            New campaign…
          </button>
        </div>
      ) : null}
      {actionError ? <p role="alert" style={{ color: 'var(--color-error)', margin: '0 0 1rem' }}>{actionError}</p> : null}
      {hasLegacyCampaigns() ? (
        <div data-testid="campaign-migration-pending" style={{ alignItems: 'center', display: 'flex', gap: '0.8rem', margin: '0 0 1rem' }}>
          <span style={{ color: 'var(--color-text-secondary)' }}>Older campaigns from before campaign folders are waiting to be migrated.</span>
          <button type="button" onClick={() => setMigration({ configDir: null })} style={barButton('default', true)}>
            Migrate…
          </button>
        </div>
      ) : null}

      <div style={{ display: 'flex', flex: 1, gap: '1rem', minHeight: 0 }}>
        {/* Campaign list */}
        <div
          style={{
            border: '1px solid var(--color-border)',
            borderRadius: 12,
            display: 'flex',
            flexDirection: 'column',
            minWidth: 280,
            overflowY: 'auto',
            width: 320,
          }}
        >
          {listError ? <p style={{ color: 'var(--color-error)', padding: '1rem' }}>{listError}</p> : null}
          {campaigns.length === 0 && !listError ? (
            <p style={{ color: 'var(--color-text-secondary)', padding: '1rem' }}>
              No campaign folders yet. Press New campaign… to make one, or Open folder… to add one you already have.
            </p>
          ) : null}
          {campaigns.map((campaign) => {
            const isSelected = campaign.path === selectedId;
            return (
              <div
                key={campaign.path}
                style={{ alignItems: 'flex-start', background: isSelected ? 'var(--color-surface-2)' : 'none', borderBottom: '1px solid var(--color-border)', display: 'flex' }}
              >
                <button
                  type="button"
                  onClick={() => setSelectedId(campaign.path)}
                  style={{ background: 'none', border: 'none', cursor: 'pointer', flex: 1, minWidth: 0, padding: '0.75rem 1rem', textAlign: 'left' }}
                >
                  <p style={{ color: isSelected ? 'var(--color-accent)' : 'var(--color-text)', fontWeight: 700, margin: 0 }}>
                    {campaign.name ?? campaign.path}
                  </p>
                  <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.78rem', margin: '0.2rem 0 0', overflowWrap: 'anywhere' }}>
                    {campaign.error ?? campaign.path}
                  </p>
                </button>
                <button
                  type="button"
                  title="Remove from this list (the folder is not touched)"
                  onClick={() => void handleForget(campaign.path)}
                  style={{ background: 'none', border: 'none', color: 'var(--color-text-muted)', cursor: 'pointer', fontSize: '0.75rem', padding: '0.8rem 0.8rem 0 0' }}
                >
                  Forget
                </button>
              </div>
            );
          })}
        </div>

        {/* Detail column: the open banner over the selected campaign's preview */}
        <div style={{ display: 'flex', flex: 1, flexDirection: 'column', gap: '1rem', minWidth: 0 }}>
          <OpenCampaignBanner opened={selected} onOpen={props.onLoad} />
          <div style={{ border: '1px solid var(--color-border)', borderRadius: 12, flex: 1, overflowY: 'auto', padding: '1.25rem' }}>
            <DetailPane opened={selected} error={openError} selected={Boolean(selectedId)} />
          </div>
        </div>
      </div>

      {/* Action bar */}
      <div style={{ borderTop: '1px solid var(--color-border)', display: 'flex', gap: '0.6rem', marginTop: '1rem', paddingTop: '1rem' }}>
        <button type="button" onClick={props.onCreate} style={barButton('default', true)}>
          New campaign…
        </button>
        <button type="button" onClick={() => void handleOpenFolder()} style={barButton('default', true)}>
          Open folder…
        </button>
        <button type="button" onClick={() => selected && props.onEdit(selected.campaignDir)} disabled={!selected} style={barButton('default', Boolean(selected))}>
          Edit
        </button>
        <CampaignHelpButton campaignDir={selected?.campaignDir ?? null} style={barButton('default', true)} />
        <div style={{ flex: 1 }} />
        <button type="button" onClick={props.onCancel} style={barButton('default', true)}>
          Cancel
        </button>
      </div>
    </section>
  );
}

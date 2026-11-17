import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import { openCampaignFolder } from '../boundary/campaignFolder/knownCampaigns';
import { listCampaignTree } from '../boundary/campaignFolder/campaignFiles';
import type { CampaignFolderSnapshot, CampaignTreeEntry } from '../boundary/campaignFolder/types';
import { CAMPAIGN_TABS, partyCharacterIds, tabFiles, type CampaignTab } from './campaignFolderModel';
import { loadCharacterHubListSurfaceRuntime } from '../characterHub/characterHubRuntime';
import type { CharacterHubListRowSurface } from '../characterHub/buildCharacterHubListSurface';
import { loadSavedCharacterDetail, type LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';
import { hasTauriRuntime } from '../boundary/runtime';
import { buildPreviewDetail } from '../characterHub/previewData';
import { formatHeldClasses } from '../characterHub/characterProgression';
import { AppMenu } from '../layout/AppMenu';
import { CampaignHelpButton } from './CampaignHelpScreen';
import { AdventureLogTab } from './tabs/AdventureLogTab';
import { PartyResourcesTab } from './tabs/PartyResourcesTab';

/**
 * Post-load campaign screen, styled like the Character Sheet: a full-bleed header, a left party
 * list, and a tabbed panel for the campaign folder's files (Party Resources / Adventure Log / Maps /
 * Wiki). Everything is read from the folder (SD-42 `decisions.md §11`): `campaign.json`, and the
 * tree for the tabs and the party (the characters copied into `Characters/` that this install
 * holds). Party Resources opens its file in the editor (E2.6); the Adventure Log lists its sessions
 * and opens one in the editor (E2.7); Maps and Wiki list their files read-only until their cards land.
 */

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

function fmt(value: number): string {
  return value >= 0 ? `+${value}` : `${value}`;
}

function MemberDetailPanel(props: { row: CharacterHubListRowSurface; onOpenFullSheet: () => void }) {
  const [detail, setDetail] = useState<LoadSavedCharacterResponse | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setDetail(null);
    const loader = hasTauriRuntime()
      ? loadSavedCharacterDetail({ characterId: props.row.characterId })
      : Promise.resolve(buildPreviewDetail());
    loader
      .then((loaded) => {
        if (!cancelled) {
          setDetail(loaded);
        }
      })
      .catch(() => {
        if (!cancelled) {
          setDetail(null);
        }
      })
      .finally(() => {
        if (!cancelled) {
          setLoading(false);
        }
      });
    return () => {
      cancelled = true;
    };
  }, [props.row.characterId]);

  const snapshot = detail?.snapshot;

  return (
    <div style={{ ...panel, padding: '1.1rem 1.25rem' }}>
      <div style={{ alignItems: 'baseline', display: 'flex', justifyContent: 'space-between' }}>
        <h3 style={{ margin: 0 }}>{props.row.displayLabel}</h3>
        <button
          type="button"
          onClick={props.onOpenFullSheet}
          style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 6, color: 'var(--color-accent)', cursor: 'pointer', fontSize: '0.8rem', padding: '0.3rem 0.7rem' }}
        >
          View full sheet →
        </button>
      </div>
      <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem', margin: '0.3rem 0 0.9rem' }}>
        {props.row.raceLabel} {formatHeldClasses(props.row.classSummary)}
      </p>
      {loading ? (
        <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>Loading…</p>
      ) : snapshot ? (
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.75rem' }}>
          {[
            ['AC', String(snapshot.baselineArmorClass)],
            ['Fort', fmt(snapshot.totalSaves.fortitude)],
            ['Ref', fmt(snapshot.totalSaves.reflex)],
            ['Will', fmt(snapshot.totalSaves.will)],
            ['BAB', fmt(snapshot.baseAttackBonus)],
          ].map(([label, value]) => (
            <div key={label} style={{ ...panel, backgroundColor: 'var(--color-surface-2)', minWidth: 64, padding: '0.4rem 0.6rem', textAlign: 'center' }}>
              <p style={{ color: 'var(--color-text-muted)', fontSize: '0.62rem', fontWeight: 700, margin: 0, textTransform: 'uppercase' }}>{label}</p>
              <p style={{ fontWeight: 800, margin: '0.1rem 0 0' }}>{value}</p>
            </div>
          ))}
        </div>
      ) : (
        <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>No computed sheet available.</p>
      )}
    </div>
  );
}

function FileList(props: { files: string[] }) {
  if (props.files.length === 0) {
    return <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', margin: 0 }}>No files here yet.</p>;
  }
  return (
    <ul data-testid="campaign-tab-files" style={{ listStyle: 'none', margin: 0, padding: 0 }}>
      {props.files.map((path) => (
        <li key={path} style={{ borderBottom: '1px solid var(--color-border)', fontSize: '0.85rem', overflowWrap: 'anywhere', padding: '0.35rem 0.2rem' }}>
          {path}
        </li>
      ))}
    </ul>
  );
}

type Loaded = { kind: 'loading' } | { kind: 'failed'; message: string } | { kind: 'open'; campaign: CampaignFolderSnapshot; tree: CampaignTreeEntry[] };

export function CampaignSheet(props: {
  campaignDir: string;
  onClose: () => void;
  onOpenCharacterSheet: (row: CharacterHubListRowSurface, detail: LoadSavedCharacterResponse | null) => void;
}) {
  const [loaded, setLoaded] = useState<Loaded>({ kind: 'loading' });
  const [savedCharacters, setSavedCharacters] = useState<CharacterHubListRowSurface[]>([]);
  const [selectedMemberId, setSelectedMemberId] = useState<string | null>(null);
  const [tab, setTab] = useState<CampaignTab>('resources');

  useEffect(() => {
    loadCharacterHubListSurfaceRuntime()
      .then((surface) => setSavedCharacters(surface.rows))
      .catch(() => setSavedCharacters([]));
  }, []);

  useEffect(() => {
    let live = true;
    Promise.all([openCampaignFolder(props.campaignDir), listCampaignTree(props.campaignDir)])
      .then(([opened, tree]) => live && setLoaded({ kind: 'open', campaign: opened.campaign, tree }))
      .catch((cause: unknown) => live && setLoaded({ kind: 'failed', message: cause instanceof Error ? cause.message : String(cause) }));
    return () => {
      live = false;
    };
  }, [props.campaignDir]);

  const reloadTree = () => {
    listCampaignTree(props.campaignDir)
      .then((fresh) => setLoaded((prev) => (prev.kind === 'open' ? { ...prev, tree: fresh } : prev)))
      .catch((cause: unknown) => setLoaded({ kind: 'failed', message: cause instanceof Error ? cause.message : String(cause) }));
  };

  const tree = loaded.kind === 'open' ? loaded.tree : null;
  const partyRows = useMemo(() => {
    if (!tree) return [];
    const ids = partyCharacterIds(tree, savedCharacters.map((row) => row.characterId));
    return savedCharacters.filter((row) => ids.includes(row.characterId));
  }, [savedCharacters, tree]);

  useEffect(() => {
    if (!selectedMemberId && partyRows.length > 0) {
      setSelectedMemberId(partyRows[0].characterId);
    }
  }, [partyRows, selectedMemberId]);

  if (loaded.kind !== 'open') {
    return (
      <div style={{ marginTop: '2rem' }}>
        <p role={loaded.kind === 'failed' ? 'alert' : undefined} style={{ color: loaded.kind === 'failed' ? 'var(--color-error)' : 'var(--color-text-muted)' }}>
          {loaded.kind === 'failed' ? loaded.message : 'Opening the campaign folder…'}
        </p>
        <button type="button" onClick={props.onClose} style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, cursor: 'pointer', padding: '0.5rem 1rem' }}>
          Back
        </button>
      </div>
    );
  }

  const { campaign } = loaded;
  const selectedMember = partyRows.find((row) => row.characterId === selectedMemberId) ?? null;

  return (
    <div style={{ marginLeft: 'calc(50% - 50vw)', marginTop: '-3rem', width: '100vw' }}>
      {/* Top bar — matches the Character Sheet's full-bleed header treatment. */}
      <div
        style={{
          alignItems: 'center',
          borderBottom: '2px solid var(--color-accent)',
          display: 'flex',
          gap: '1.5rem',
          padding: '0.95rem 4rem 0.95rem 1.5rem',
        }}
      >
        <AppMenu items={[]} />
        <button
          type="button"
          onClick={props.onClose}
          title="Close campaign"
          style={{ background: 'none', border: 'none', color: 'var(--color-text-muted)', cursor: 'pointer', fontSize: '1rem' }}
        >
          ✕
        </button>
        <span style={{ fontWeight: 600 }}>
          {campaign.name} — {campaign.ruleSetLabel}
        </span>
        <div style={{ flex: 1 }} />
        <CampaignHelpButton campaignDir={props.campaignDir} />
      </div>

      <div style={{ display: 'flex', gap: '0.75rem', padding: '1rem 1.5rem' }}>
        {/* Left: party member list */}
        <div style={{ borderRight: '1px solid var(--color-border)', flex: '0 0 240px', paddingRight: '0.75rem' }}>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', fontWeight: 700, letterSpacing: '0.06em', marginBottom: '0.6rem', textTransform: 'uppercase' }}>
            Party ({partyRows.length})
          </p>
          {partyRows.length === 0 ? (
            <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem' }}>
              No party members yet. A character joins from its sheet&apos;s Campaign dialog, which copies it into this folder&apos;s Characters/.
            </p>
          ) : (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '0.4rem' }}>
              {partyRows.map((row) => {
                const active = row.characterId === selectedMemberId;
                return (
                  <button
                    key={row.characterId}
                    type="button"
                    onClick={() => setSelectedMemberId(row.characterId)}
                    style={{
                      ...panel,
                      backgroundColor: active ? 'var(--color-accent)' : 'var(--color-surface)',
                      cursor: 'pointer',
                      padding: '0.55rem 0.7rem',
                      textAlign: 'left',
                    }}
                  >
                    <p style={{ color: active ? 'var(--color-on-accent)' : 'var(--color-text)', fontWeight: 700, margin: 0 }}>{row.displayLabel}</p>
                    <p style={{ color: active ? 'var(--color-on-accent-muted)' : 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.1rem 0 0' }}>
                      {row.raceLabel}
                    </p>
                  </button>
                );
              })}
            </div>
          )}
        </div>

        {/* Right: selected member + campaign assets */}
        <div style={{ display: 'flex', flex: 1, flexDirection: 'column', gap: '1rem', minWidth: 0 }}>
          {selectedMember ? (
            <MemberDetailPanel
              row={selectedMember}
              onOpenFullSheet={() => {
                if (hasTauriRuntime()) {
                  loadSavedCharacterDetail({ characterId: selectedMember.characterId })
                    .then((detail) => props.onOpenCharacterSheet(selectedMember, detail))
                    .catch(() => props.onOpenCharacterSheet(selectedMember, null));
                } else {
                  props.onOpenCharacterSheet(selectedMember, buildPreviewDetail());
                }
              }}
            />
          ) : null}

          <div>
            <div style={{ borderBottom: '1px solid var(--color-border)', display: 'flex', flexWrap: 'wrap', gap: '1.25rem', marginBottom: '1rem' }}>
              {CAMPAIGN_TABS.map((assetTab) => {
                const active = assetTab.tab === tab;
                return (
                  <button
                    key={assetTab.tab}
                    type="button"
                    onClick={() => setTab(assetTab.tab)}
                    style={{
                      background: 'none',
                      border: 'none',
                      borderBottom: `2px solid ${active ? 'var(--color-accent)' : 'transparent'}`,
                      color: active ? 'var(--color-text)' : 'var(--color-text-muted)',
                      cursor: 'pointer',
                      fontSize: '0.95rem',
                      fontWeight: active ? 700 : 500,
                      padding: '0.3rem 0.1rem 0.6rem',
                    }}
                  >
                    {assetTab.label}
                  </button>
                );
              })}
            </div>

            <div style={{ ...panel, padding: '1.25rem' }}>
              {tab === 'resources' ? (
                <PartyResourcesTab campaignDir={props.campaignDir} tree={loaded.tree} onTreeChanged={reloadTree} />
              ) : tab === 'adventureLog' ? (
                <AdventureLogTab campaignDir={props.campaignDir} tree={loaded.tree} onTreeChanged={reloadTree} />
              ) : (
                <FileList files={tabFiles(loaded.tree, tab)} />
              )}
            </div>
            <p style={{ color: 'var(--color-text-faint)', fontSize: '0.72rem', margin: '0.6rem 0 0', overflowWrap: 'anywhere' }}>
              The files in {props.campaignDir}. Open the folder in Obsidian or any Markdown editor to edit them.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

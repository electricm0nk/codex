import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import {
  addCampaignAsset,
  deleteCampaignAsset,
  getCampaign,
  getCampaignAssets,
  updateCampaignAsset,
  type Campaign,
  type CampaignAssetKind,
  type MarkdownAsset,
} from './campaignModel';
import { loadCharacterHubListSurfaceRuntime } from '../characterHub/characterHubRuntime';
import type { CharacterHubListRowSurface } from '../characterHub/buildCharacterHubListSurface';
import { loadSavedCharacterDetail, type LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';
import { hasTauriRuntime } from '../boundary/runtime';
import { buildPreviewDetail } from '../characterHub/previewData';
import { formatHeldClasses } from '../characterHub/characterProgression';

/**
 * Post-load campaign screen, styled like the Character Sheet: a full-bleed
 * header, a left party-member list, and a tabbed bottom panel for the
 * campaign's shared assets (Party Resources / Adventure Log / Maps / Wiki).
 *
 * These tabs hold markdown assets. localStorage (campaignModel.ts) is the
 * source of truth — every change here also gets mirrored to real `.md`
 * files under the campaign's local folder (see `syncCampaignDriveArtifacts`)
 * so the campaign stays inspectable/shareable as plain files, but there is
 * no network sync: by design, this app never talks to a cloud API.
 */

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

const ASSET_TABS: ReadonlyArray<{ kind: CampaignAssetKind; label: string }> = [
  { kind: 'resources', label: 'Party Resources' },
  { kind: 'adventureLog', label: 'Adventure Log' },
  { kind: 'maps', label: 'Maps' },
  { kind: 'wiki', label: 'Wiki' },
];

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

function AssetList(props: {
  assets: MarkdownAsset[];
  onAdd: (title: string) => void;
  onSave: (assetId: string, title: string, body: string) => void;
  onDelete: (assetId: string) => void;
  emptyLabel: string;
  addLabel: string;
}) {
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [draftTitle, setDraftTitle] = useState('');
  const [draftBody, setDraftBody] = useState('');
  const [newTitle, setNewTitle] = useState('');

  const selected = props.assets.find((asset) => asset.id === selectedId) ?? null;

  function select(asset: MarkdownAsset) {
    setSelectedId(asset.id);
    setDraftTitle(asset.title);
    setDraftBody(asset.body);
  }

  function handleAdd() {
    if (!newTitle.trim()) {
      return;
    }
    props.onAdd(newTitle.trim());
    setNewTitle('');
  }

  return (
    <div style={{ display: 'flex', gap: '1rem', minHeight: 220 }}>
      <div style={{ borderRight: '1px solid var(--color-border)', display: 'flex', flex: '0 0 220px', flexDirection: 'column', paddingRight: '1rem' }}>
        <div style={{ display: 'flex', gap: '0.4rem', marginBottom: '0.6rem' }}>
          <input
            type="text"
            placeholder={props.addLabel}
            value={newTitle}
            onChange={(event) => setNewTitle(event.target.value)}
            style={{ backgroundColor: 'var(--color-surface-2)', border: '1px solid var(--color-border)', borderRadius: 6, boxSizing: 'border-box', color: 'var(--color-text)', flex: 1, fontSize: '0.82rem', padding: '0.35rem 0.5rem' }}
          />
          <button
            type="button"
            onClick={handleAdd}
            disabled={!newTitle.trim()}
            style={{ backgroundColor: 'var(--color-accent)', border: 'none', borderRadius: 6, color: 'var(--color-on-accent)', cursor: newTitle.trim() ? 'pointer' : 'not-allowed', flexShrink: 0, fontSize: '0.8rem', fontWeight: 700, opacity: newTitle.trim() ? 1 : 0.5, padding: '0.35rem 0.6rem' }}
          >
            +
          </button>
        </div>
        {props.assets.length === 0 ? (
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.82rem' }}>{props.emptyLabel}</p>
        ) : (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '0.25rem' }}>
            {props.assets.map((asset) => (
              <button
                key={asset.id}
                type="button"
                onClick={() => select(asset)}
                style={{
                  background: selectedId === asset.id ? 'var(--color-surface-2)' : 'none',
                  border: 'none',
                  borderRadius: 6,
                  color: selectedId === asset.id ? 'var(--color-accent)' : 'var(--color-text)',
                  cursor: 'pointer',
                  fontSize: '0.85rem',
                  fontWeight: selectedId === asset.id ? 700 : 500,
                  padding: '0.4rem 0.5rem',
                  textAlign: 'left',
                }}
              >
                {asset.title}
              </button>
            ))}
          </div>
        )}
      </div>

      <div style={{ flex: 1, minWidth: 0 }}>
        {!selected ? (
          <p style={{ color: 'var(--color-text-muted)' }}>Select an entry on the left, or add a new one.</p>
        ) : (
          <div>
            <input
              type="text"
              value={draftTitle}
              onChange={(event) => setDraftTitle(event.target.value)}
              style={{ backgroundColor: 'var(--color-surface-2)', border: '1px solid var(--color-border)', borderRadius: 6, boxSizing: 'border-box', color: 'var(--color-text)', fontSize: '0.95rem', fontWeight: 700, marginBottom: '0.6rem', padding: '0.4rem 0.6rem', width: '100%' }}
            />
            <textarea
              value={draftBody}
              onChange={(event) => setDraftBody(event.target.value)}
              placeholder="Markdown content…"
              rows={8}
              style={{ backgroundColor: 'var(--color-surface-2)', border: '1px solid var(--color-border)', borderRadius: 6, boxSizing: 'border-box', color: 'var(--color-text)', fontFamily: 'inherit', fontSize: '0.85rem', padding: '0.6rem', resize: 'vertical', width: '100%' }}
            />
            <div style={{ display: 'flex', gap: '0.5rem', marginTop: '0.6rem' }}>
              <button
                type="button"
                onClick={() => props.onSave(selected.id, draftTitle.trim() || selected.title, draftBody)}
                style={{ backgroundColor: 'var(--color-accent)', border: 'none', borderRadius: 6, color: 'var(--color-on-accent)', cursor: 'pointer', fontSize: '0.82rem', fontWeight: 700, padding: '0.4rem 0.9rem' }}
              >
                Save
              </button>
              <button
                type="button"
                onClick={() => {
                  props.onDelete(selected.id);
                  setSelectedId(null);
                }}
                style={{ backgroundColor: 'var(--color-surface-2)', border: '1px solid var(--color-error-border)', borderRadius: 6, color: 'var(--color-error)', cursor: 'pointer', fontSize: '0.82rem', fontWeight: 700, padding: '0.4rem 0.9rem' }}
              >
                Delete
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export function CampaignSheet(props: {
  campaignId: string;
  onClose: () => void;
  onOpenCharacterSheet: (row: CharacterHubListRowSurface, detail: LoadSavedCharacterResponse | null) => void;
}) {
  const [campaign] = useState<Campaign | null>(() => getCampaign(props.campaignId));
  const [savedCharacters, setSavedCharacters] = useState<CharacterHubListRowSurface[]>([]);
  const [selectedMemberId, setSelectedMemberId] = useState<string | null>(null);
  const [tab, setTab] = useState<CampaignAssetKind>('resources');
  const [assets, setAssets] = useState(() => (campaign ? getCampaignAssets(campaign.id) : null));

  useEffect(() => {
    loadCharacterHubListSurfaceRuntime()
      .then((surface) => setSavedCharacters(surface.rows))
      .catch(() => setSavedCharacters([]));
  }, []);

  const partyRows = useMemo(
    () => (campaign ? savedCharacters.filter((row) => campaign.partyCharacterIds.includes(row.characterId)) : []),
    [savedCharacters, campaign]
  );

  useEffect(() => {
    if (!selectedMemberId && partyRows.length > 0) {
      setSelectedMemberId(partyRows[0].characterId);
    }
  }, [partyRows, selectedMemberId]);

  if (!campaign) {
    return (
      <div style={{ marginTop: '2rem' }}>
        <p style={{ color: 'var(--color-error)' }}>Campaign not found.</p>
        <button type="button" onClick={props.onClose} style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, cursor: 'pointer', padding: '0.5rem 1rem' }}>
          Back
        </button>
      </div>
    );
  }

  function refreshAssets() {
    setAssets(getCampaignAssets(campaign!.id));
  }

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
        <span style={{ fontSize: '1.1rem', fontWeight: 700 }}>☰ Menu</span>
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
      </div>

      <div style={{ display: 'flex', gap: '0.75rem', padding: '1rem 1.5rem' }}>
        {/* Left: party member list */}
        <div style={{ borderRight: '1px solid var(--color-border)', flex: '0 0 240px', paddingRight: '0.75rem' }}>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', fontWeight: 700, letterSpacing: '0.06em', marginBottom: '0.6rem', textTransform: 'uppercase' }}>
            Party ({partyRows.length})
          </p>
          {partyRows.length === 0 ? (
            <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem' }}>No party members yet — add some from Edit Campaign.</p>
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
              {ASSET_TABS.map((assetTab) => {
                const active = assetTab.kind === tab;
                return (
                  <button
                    key={assetTab.kind}
                    type="button"
                    onClick={() => setTab(assetTab.kind)}
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
              {assets ? (
                <AssetList
                  assets={assets[tab]}
                  emptyLabel="Nothing here yet."
                  addLabel={`New ${ASSET_TABS.find((assetTab) => assetTab.kind === tab)?.label.toLowerCase()} entry…`}
                  onAdd={(title) => {
                    addCampaignAsset(campaign.id, tab, title);
                    refreshAssets();
                  }}
                  onSave={(assetId, title, body) => {
                    updateCampaignAsset(campaign.id, tab, assetId, { title, body });
                    refreshAssets();
                  }}
                  onDelete={(assetId) => {
                    deleteCampaignAsset(campaign.id, tab, assetId);
                    refreshAssets();
                  }}
                />
              ) : null}
            </div>
            <p style={{ color: 'var(--color-text-faint)', fontSize: '0.72rem', margin: '0.6rem 0 0' }}>
              Entries are saved locally for now; syncing them as markdown files to the campaign's Drive folder is not
              yet wired.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

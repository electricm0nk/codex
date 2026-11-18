import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import { openCampaignFolder } from '../boundary/campaignFolder/knownCampaigns';
import type { CampaignFolderSnapshot, CampaignTreeEntry } from '../boundary/campaignFolder/types';
import { CAMPAIGN_TABS, type CampaignTab } from './campaignFolderModel';
import { characterRows, diffLines, IMPORT_CHOICES, type CharacterRow } from './charactersModel';
import { diffCampaignCharacter, importCampaignCharacter, listCampaignCharacters, loadCampaignCharacter, type CampaignCharacterBackup, type ImportMode } from '../boundary/campaignFolder/characters';
import { loadCharacterHubListSurfaceRuntime } from '../characterHub/characterHubRuntime';
import type { CharacterHubListRowSurface } from '../characterHub/buildCharacterHubListSurface';
import { loadSavedCharacterDetail, type LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';
import { hasTauriRuntime } from '../boundary/runtime';
import { buildPreviewDetail } from '../characterHub/previewData';
import { formatHeldClasses } from '../characterHub/characterProgression';
import { AppMenu } from '../layout/AppMenu';
import { CampaignHelpButton } from './CampaignHelpScreen';
import { AdventureLogTab } from './tabs/AdventureLogTab';
import { MapsTab } from './tabs/MapsTab';
import { listCampaignTree } from '../boundary/campaignFolder/campaignFiles';
import { PartyResourcesTab } from './tabs/PartyResourcesTab';
import { WikiTab } from './tabs/WikiTab';

/**
 * Post-load campaign screen, styled like the Character Sheet: a full-bleed header, a left Characters
 * list (the folder's `Characters/` backups, E2.10: open one read-only, import it as a stand-in or as
 * the moved master, compare this device's copy with it), and a tabbed panel for the campaign folder's files (Party Resources / Adventure Log / Maps /
 * Wiki). Everything is read from the folder (SD-42 `decisions.md §11`): `campaign.json`, and the
 * tree for the tabs, and `list_campaign_characters` for the Characters list. Party Resources opens its file in the editor (E2.6); the Adventure Log lists its sessions
 * and opens one in the editor (E2.7); Maps shows each map with its pins (E2.8), and a pin opens its
 * page on the Wiki tab (E2.9): folders, pages, links, backlinks and search.
 */

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

function fmt(value: number): string {
  return value >= 0 ? `+${value}` : `${value}`;
}

function MemberDetailPanel(props: { row: CharacterHubListRowSurface; loadKey: string; load: () => Promise<LoadSavedCharacterResponse>; onOpenFullSheet?: () => void }) {
  const [detail, setDetail] = useState<LoadSavedCharacterResponse | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setDetail(null);
    props
      .load()
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
    // `loadKey` names what `load` reads.
  }, [props.loadKey]);

  const snapshot = detail?.snapshot;

  return (
    <div style={{ ...panel, padding: '1.1rem 1.25rem' }}>
      <div style={{ alignItems: 'baseline', display: 'flex', justifyContent: 'space-between' }}>
        <h3 style={{ margin: 0 }}>{props.row.displayLabel}</h3>
        {props.onOpenFullSheet ? (
          <button
            type="button"
            onClick={props.onOpenFullSheet}
            style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 6, color: 'var(--color-accent)', cursor: 'pointer', fontSize: '0.8rem', padding: '0.3rem 0.7rem' }}
          >
            View full sheet →
          </button>
        ) : (
          <span style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem' }}>Campaign copy · read-only</span>
        )}
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

/** Import a backup onto this device, or compare this device's copy with it (decisions.md §16). */
function CharacterActions(props: { campaignDir: string; row: CharacterRow; onImported: () => void }) {
  const [mode, setMode] = useState<ImportMode>(IMPORT_CHOICES.find((choice) => choice.isDefault)!.mode);
  const [message, setMessage] = useState<string | null>(null);
  const [diff, setDiff] = useState<string[] | null>(null);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    setMessage(null);
    setDiff(null);
  }, [props.row.folder]);

  const fail = (cause: unknown) => setMessage(cause instanceof Error ? cause.message : String(cause));
  const button: CSSProperties = { background: 'var(--color-accent)', border: 'none', borderRadius: 6, color: 'var(--color-on-accent)', cursor: 'pointer', fontWeight: 600, padding: '0.4rem 0.9rem' };

  return (
    <div style={{ ...panel, padding: '0.9rem 1.25rem' }}>
      {props.row.onThisDevice ? (
        <>
          <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem', margin: '0 0 0.6rem' }}>This device holds a copy of this character. Compare it with the campaign copy (nothing is merged).</p>
          <button
            type="button"
            disabled={busy}
            style={button}
            onClick={() => {
              setBusy(true);
              diffCampaignCharacter(props.campaignDir, props.row.folder)
                .then((diffs) => setDiff(diffLines(diffs)))
                .catch(fail)
                .finally(() => setBusy(false));
            }}
          >
            Compare with the campaign copy
          </button>
          {diff ? (
            <ul style={{ fontSize: '0.85rem', margin: '0.7rem 0 0', paddingLeft: '1.2rem' }}>
              {diff.map((line) => (
                <li key={line}>{line}</li>
              ))}
            </ul>
          ) : null}
        </>
      ) : (
        <fieldset style={{ border: 'none', margin: 0, padding: 0 }}>
          <legend style={{ fontWeight: 700, marginBottom: '0.5rem' }}>Import to this device…</legend>
          {IMPORT_CHOICES.map((choice) => (
            <label key={choice.mode} style={{ display: 'flex', gap: '0.5rem', marginBottom: '0.45rem' }}>
              <input type="radio" name="import-mode" checked={mode === choice.mode} onChange={() => setMode(choice.mode)} />
              <span>
                <strong>{choice.label}</strong>
                {choice.isDefault ? ' (default)' : ''}
                <br />
                <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem' }}>{choice.help}</span>
              </span>
            </label>
          ))}
          <button
            type="button"
            disabled={busy}
            style={button}
            onClick={() => {
              setBusy(true);
              importCampaignCharacter(props.campaignDir, props.row.folder, mode)
                .then(() => {
                  setMessage(`${props.row.title} is on this device now: open it from the character list.`);
                  props.onImported();
                })
                .catch(fail)
                .finally(() => setBusy(false));
            }}
          >
            Import
          </button>
        </fieldset>
      )}
      {message ? (
        <p role="status" style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem', margin: '0.6rem 0 0' }}>
          {message}
        </p>
      ) : null}
    </div>
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
  const [backups, setBackups] = useState<CampaignCharacterBackup[]>([]);
  const [selectedFolder, setSelectedFolder] = useState<string | null>(null);
  const [tab, setTab] = useState<CampaignTab>('resources');
  // The page open on the Wiki tab (a map pin opens one there too).
  const [wikiPage, setWikiPage] = useState<string | null>(null);

  const reloadCharacters = () => {
    loadCharacterHubListSurfaceRuntime()
      .then((surface) => setSavedCharacters(surface.rows))
      .catch(() => setSavedCharacters([]));
    listCampaignCharacters(props.campaignDir)
      .then(setBackups)
      .catch(() => setBackups([]));
  };

  // Reload when the folder changes.
  useEffect(reloadCharacters, [props.campaignDir]);

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

  const rows = useMemo(() => characterRows(backups, savedCharacters.map((row) => row.characterId)), [backups, savedCharacters]);

  useEffect(() => {
    if (!selectedFolder && rows.length > 0) {
      setSelectedFolder(rows[0].folder);
    }
  }, [rows, selectedFolder]);

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
  const selected = rows.find((row) => row.folder === selectedFolder) ?? null;
  const localRow = selected?.onThisDevice ? (savedCharacters.find((row) => row.characterId === selected.characterId) ?? null) : null;

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
        {/* Left: the folder's Characters/ backups */}
        <div style={{ borderRight: '1px solid var(--color-border)', flex: '0 0 240px', paddingRight: '0.75rem' }}>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', fontWeight: 700, letterSpacing: '0.06em', marginBottom: '0.6rem', textTransform: 'uppercase' }}>
            Characters ({rows.length})
          </p>
          {rows.length === 0 ? (
            <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem' }}>
              No characters yet. A character joins from its sheet&apos;s Campaign dialog, which copies it into this folder&apos;s Characters/.
            </p>
          ) : (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '0.4rem' }}>
              {rows.map((row) => {
                const active = row.folder === selectedFolder;
                return (
                  <button
                    key={row.folder}
                    type="button"
                    onClick={() => setSelectedFolder(row.folder)}
                    style={{
                      ...panel,
                      backgroundColor: active ? 'var(--color-accent)' : 'var(--color-surface)',
                      cursor: 'pointer',
                      padding: '0.55rem 0.7rem',
                      textAlign: 'left',
                    }}
                  >
                    <p style={{ color: active ? 'var(--color-on-accent)' : 'var(--color-text)', fontWeight: 700, margin: 0 }}>{row.title}</p>
                    <p style={{ color: active ? 'var(--color-on-accent-muted)' : 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.1rem 0 0' }}>
                      {row.subtitle}
                    </p>
                    <p style={{ color: active ? 'var(--color-on-accent-muted)' : 'var(--color-text-faint)', fontSize: '0.7rem', margin: '0.1rem 0 0' }}>
                      {row.master}
                    </p>
                  </button>
                );
              })}
            </div>
          )}
        </div>

        {/* Right: selected member + campaign assets */}
        <div style={{ display: 'flex', flex: 1, flexDirection: 'column', gap: '1rem', minWidth: 0 }}>
          {selected?.readable ? (
            <MemberDetailPanel
              row={selected.surface}
              loadKey={`${props.campaignDir}/${selected.folder}`}
              load={() => (hasTauriRuntime() ? loadCampaignCharacter(props.campaignDir, selected.folder) : Promise.resolve(buildPreviewDetail()))}
              onOpenFullSheet={
                localRow
                  ? () => {
                      // This device's own copy, as the hub opens it.
                      if (hasTauriRuntime()) {
                        loadSavedCharacterDetail({ characterId: localRow.characterId })
                          .then((detail) => props.onOpenCharacterSheet(localRow, detail))
                          .catch(() => props.onOpenCharacterSheet(localRow, null));
                      } else {
                        props.onOpenCharacterSheet(localRow, buildPreviewDetail());
                      }
                    }
                  : undefined
              }
            />
          ) : null}
          {selected?.readable ? <CharacterActions campaignDir={props.campaignDir} row={selected} onImported={reloadCharacters} /> : null}

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
              ) : tab === 'maps' ? (
                <MapsTab
                  campaignDir={props.campaignDir}
                  tree={loaded.tree}
                  onTreeChanged={reloadTree}
                  onOpenPage={(path) => {
                    setWikiPage(path);
                    setTab('wiki');
                  }}
                />
              ) : (
                <WikiTab campaignDir={props.campaignDir} tree={loaded.tree} onTreeChanged={reloadTree} page={wikiPage} onOpenPage={setWikiPage} />
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

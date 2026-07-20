import { useEffect, useState, type CSSProperties } from 'react';
import { save } from '@tauri-apps/plugin-dialog';
import type { CharacterHubListSurface, CharacterHubListRowSurface } from './buildCharacterHubListSurface';
import { loadSavedCharacterDetail, type LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';
import { hasTauriRuntime } from '../boundary/runtime';
import { exportCharacterJson } from '../boundary/exportCharacterJson';
import { cloneCharacter } from '../boundary/cloneCharacter';
import { loadCharacterPortrait } from '../boundary/characterPortrait';
import { buildPreviewDetail } from './previewData';
import { formatHeldClasses } from './characterProgression';

/**
 * Load Character screen, patterned after Pathbuilder's loader: a selectable
 * character list on the left, a stat-block detail pane (portrait + summary +
 * computed stats) on the right, and a bottom action bar
 * (Load / Delete / Export / Clone / Import / Cancel).
 *
 * Storage is the database (no folders). Delete and Import are presented as
 * stubs until their persistence commands land; Load, Export, and Clone
 * operate on the real bounded detail returned by the desktop runtime.
 * Export prompts for a save location via the native file dialog (falls back
 * to the clipboard outside the Tauri runtime, e.g. the browser preview).
 * Clone duplicates the full saved build (including its portrait) under a
 * new id via the `clone_character` command, then refreshes the list.
 */

const barButton = (variant: 'primary' | 'default' | 'danger', enabled: boolean): CSSProperties => ({
  backgroundColor: variant === 'primary' && enabled ? 'var(--color-accent)' : 'var(--color-surface-2)',
  border: `1px solid ${variant === 'danger' ? 'var(--color-error-border)' : 'var(--color-border)'}`,
  borderRadius: 8,
  color:
    variant === 'primary' && enabled
      ? 'var(--color-on-accent)'
      : variant === 'danger'
        ? 'var(--color-error)'
        : 'var(--color-text)',
  cursor: enabled ? 'pointer' : 'not-allowed',
  fontSize: '0.9rem',
  fontWeight: variant === 'primary' ? 700 : 500,
  opacity: enabled ? 1 : 0.5,
  padding: '0.55rem 1.4rem',
});

function StatLine(props: { label: string; children: React.ReactNode }) {
  return (
    <p style={{ margin: '0.15rem 0' }}>
      <span style={{ fontWeight: 700 }}>{props.label}</span> {props.children}
    </p>
  );
}

function DetailPane(props: {
  row: CharacterHubListRowSurface | null;
  detail: LoadSavedCharacterResponse | null;
  detailError: string | null;
  portraitUrl: string | null;
}) {
  const { row, detail, detailError, portraitUrl } = props;

  if (!row) {
    return (
      <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>Select a character to preview, then press Load for the full sheet.</p>
    );
  }

  return (
    <div>
      <div style={{ display: 'flex', gap: '1rem' }}>
        {portraitUrl ? (
          <img
            src={portraitUrl}
            alt={`${row.displayLabel} portrait`}
            style={{ borderRadius: 8, flexShrink: 0, height: 96, objectFit: 'cover', width: 96 }}
          />
        ) : null}
        <div style={{ flex: 1, minWidth: 0 }}>
          <div style={{ alignItems: 'baseline', display: 'flex', justifyContent: 'space-between', gap: '1rem' }}>
            <h2 style={{ margin: 0 }}>{row.displayLabel}</h2>
            <span style={{ color: 'var(--color-text-secondary)', fontWeight: 700 }}>{formatHeldClasses(row.classSummary)}</span>
          </div>
          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem', margin: '0.6rem 0' }}>
            {[row.raceLabel, row.gameSystemLabel].map((tag) => (
              <span
                key={tag}
                style={{
                  backgroundColor: 'var(--color-surface-2)',
                  border: '1px solid var(--color-border)',
                  borderRadius: 6,
                  color: 'var(--color-text-secondary)',
                  fontSize: '0.78rem',
                  padding: '0.15rem 0.5rem',
                }}
              >
                {tag}
              </span>
            ))}
          </div>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: 0 }}>Last modified: {row.savedAtLabel}</p>
        </div>
      </div>

      <hr style={{ border: 'none', borderTop: '1px solid var(--color-border)', margin: '0.75rem 0' }} />

      {detailError ? (
        <p style={{ color: 'var(--color-warn)', margin: 0 }}>{detailError}</p>
      ) : detail?.snapshot ? (
        <div style={{ fontSize: '0.9rem', lineHeight: 1.5 }}>
          <StatLine label="Ability modifiers">
            Str {fmt(detail.snapshot.abilityModifiers.strength)}, Dex {fmt(detail.snapshot.abilityModifiers.dexterity)}, Con{' '}
            {fmt(detail.snapshot.abilityModifiers.constitution)}, Int {fmt(detail.snapshot.abilityModifiers.intelligence)}, Wis{' '}
            {fmt(detail.snapshot.abilityModifiers.wisdom)}, Cha {fmt(detail.snapshot.abilityModifiers.charisma)}
          </StatLine>
          <hr style={{ border: 'none', borderTop: '1px solid var(--color-border)', margin: '0.6rem 0' }} />
          <StatLine label="AC">{detail.snapshot.baselineArmorClass}</StatLine>
          <StatLine label="Saves">
            Fort {fmt(detail.snapshot.totalSaves.fortitude)}, Ref {fmt(detail.snapshot.totalSaves.reflex)}, Will{' '}
            {fmt(detail.snapshot.totalSaves.will)}
          </StatLine>
          <hr style={{ border: 'none', borderTop: '1px solid var(--color-border)', margin: '0.6rem 0' }} />
          <StatLine label="Base attack bonus">{fmt(detail.snapshot.baseAttackBonus)}</StatLine>
          <StatLine label="Melee">{fmt(detail.snapshot.baselineMeleeAttackBonus)}</StatLine>
          <StatLine label="Skills">
            Climb {fmt(detail.snapshot.selectedSkillModifiers.climb)}, Intimidate {fmt(detail.snapshot.selectedSkillModifiers.intimidate)},
            Swim {fmt(detail.snapshot.selectedSkillModifiers.swim)}
          </StatLine>
        </div>
      ) : detail && !detail.snapshot ? (
        <p style={{ color: 'var(--color-warn)', margin: 0 }}>
          This character loaded with blocking diagnostics, so no computed sheet is available.
        </p>
      ) : (
        <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>Press Load to compute and view the full sheet.</p>
      )}
    </div>
  );
}

function fmt(value: number): string {
  return value >= 0 ? `+${value}` : `${value}`;
}

export function LoadCharacterScreen(props: {
  surface: CharacterHubListSurface | null;
  error: string | null;
  onCancel: () => void;
  onOpenSheet: (row: CharacterHubListRowSurface, detail: LoadSavedCharacterResponse | null) => void;
  onRefresh: () => void;
}) {
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [detail, setDetail] = useState<LoadSavedCharacterResponse | null>(null);
  const [detailError, setDetailError] = useState<string | null>(null);
  const [status, setStatus] = useState<string | null>(null);
  const [portraitUrl, setPortraitUrl] = useState<string | null>(null);
  const [cloning, setCloning] = useState(false);

  const rows = props.surface?.rows ?? [];
  const selectedRow = rows.find((row) => row.characterId === selectedId) ?? null;

  useEffect(() => {
    if (!selectedId || !hasTauriRuntime()) {
      setPortraitUrl(null);
      return;
    }
    let cancelled = false;
    loadCharacterPortrait(selectedId)
      .then((url) => {
        if (!cancelled) {
          setPortraitUrl(url);
        }
      })
      .catch(() => {
        if (!cancelled) {
          setPortraitUrl(null);
        }
      });
    return () => {
      cancelled = true;
    };
  }, [selectedId]);

  function selectRow(characterId: string) {
    setSelectedId(characterId);
    setDetail(null);
    setDetailError(null);
    setStatus(null);
  }

  async function handleLoad() {
    if (!selectedId) {
      return;
    }
    if (!selectedRow) {
      return;
    }
    setDetailError(null);
    // Without the desktop backend (e.g. the browser preview) there is nothing to
    // fetch — open the sheet with the summary and placeholder stats so the view
    // is still reachable for UI work.
    if (!hasTauriRuntime()) {
      props.onOpenSheet(selectedRow, buildPreviewDetail());
      return;
    }
    try {
      const loaded = await loadSavedCharacterDetail({ characterId: selectedId });
      setDetail(loaded);
      setStatus(null);
      props.onOpenSheet(selectedRow, loaded);
    } catch (cause: unknown) {
      setDetailError(cause instanceof Error ? cause.message : 'Unable to load character detail.');
    }
  }

  async function handleExport() {
    if (!selectedRow) {
      return;
    }
    const payload = JSON.stringify({ summary: selectedRow, detail }, null, 2);

    // Without the Tauri runtime (e.g. the browser preview) there's no native
    // save dialog to show — fall back to the clipboard so export still does
    // something useful.
    if (!hasTauriRuntime()) {
      try {
        await navigator.clipboard.writeText(payload);
        setStatus(`Copied ${selectedRow.displayLabel} as JSON to the clipboard.`);
      } catch {
        setStatus('Could not access the clipboard to export.');
      }
      return;
    }

    const safeName = selectedRow.displayLabel.replace(/[^a-z0-9-_ ]/gi, '').trim() || 'character';
    try {
      const filePath = await save({
        defaultPath: `${safeName}.json`,
        filters: [{ name: 'Character JSON', extensions: ['json'] }],
      });
      if (!filePath) {
        return;
      }
      await exportCharacterJson(filePath, payload);
      setStatus(`Exported ${selectedRow.displayLabel} to ${filePath}.`);
    } catch (cause: unknown) {
      setStatus(cause instanceof Error ? cause.message : 'Could not export the character.');
    }
  }

  async function handleClone() {
    if (!selectedRow) {
      return;
    }
    if (!hasTauriRuntime()) {
      setStatus('Cloning requires the desktop runtime.');
      return;
    }
    setCloning(true);
    try {
      const outcome = await cloneCharacter({
        characterId: selectedRow.characterId,
        newCharacterId: crypto.randomUUID(),
        newDisplayLabel: `${selectedRow.displayLabel} (Copy)`,
        savedAt: new Date().toISOString(),
      });
      if (outcome.kind === 'Saved') {
        setStatus(`Cloned ${selectedRow.displayLabel} as "${outcome.summary.displayLabel}".`);
        props.onRefresh();
      } else {
        setStatus('Clone failed: the copy no longer computes cleanly, so nothing was saved.');
      }
    } catch (cause: unknown) {
      setStatus(cause instanceof Error ? cause.message : 'Could not clone the character.');
    } finally {
      setCloning(false);
    }
  }

  return (
    <section style={{ display: 'flex', flexDirection: 'column', height: 'calc(100vh - 8rem)', marginTop: '1rem' }}>
      <h2 style={{ margin: '0 0 1rem' }}>Load Character</h2>

      <div style={{ display: 'flex', flex: 1, gap: '1rem', minHeight: 0 }}>
        {/* Character list */}
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
          {props.error ? <p style={{ color: 'var(--color-error)', padding: '1rem' }}>{props.error}</p> : null}
          {props.surface?.isEmpty ? (
            <p style={{ color: 'var(--color-text-secondary)', padding: '1rem' }}>{props.surface.emptyStateMessage}</p>
          ) : null}
          {rows.map((row) => {
            const selected = row.characterId === selectedId;
            return (
              <button
                key={row.characterId}
                type="button"
                onClick={() => selectRow(row.characterId)}
                style={{
                  background: selected ? 'var(--color-surface-2)' : 'none',
                  border: 'none',
                  borderBottom: '1px solid var(--color-border)',
                  cursor: 'pointer',
                  padding: '0.75rem 1rem',
                  textAlign: 'left',
                }}
              >
                <p style={{ color: selected ? 'var(--color-accent)' : 'var(--color-text)', fontWeight: 700, margin: 0 }}>
                  {row.displayLabel}
                </p>
                <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.82rem', margin: '0.2rem 0 0' }}>
                  {row.raceLabel} {formatHeldClasses(row.classSummary)}
                </p>
                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.15rem 0 0' }}>
                  Last modified: {row.savedAtLabel}
                </p>
              </button>
            );
          })}
        </div>

        {/* Detail pane */}
        <div style={{ border: '1px solid var(--color-border)', borderRadius: 12, flex: 1, overflowY: 'auto', padding: '1.25rem' }}>
          <DetailPane row={selectedRow} detail={detail} detailError={detailError} portraitUrl={portraitUrl} />
        </div>
      </div>

      {status ? <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem', margin: '0.75rem 0 0' }}>{status}</p> : null}

      {/* Action bar */}
      <div style={{ borderTop: '1px solid var(--color-border)', display: 'flex', gap: '0.6rem', marginTop: '1rem', paddingTop: '1rem' }}>
        <button type="button" onClick={handleLoad} disabled={!selectedId} style={barButton('primary', Boolean(selectedId))}>
          Load
        </button>
        <button
          type="button"
          onClick={() => setStatus('Delete will be available once database storage lands.')}
          disabled={!selectedId}
          style={barButton('danger', Boolean(selectedId))}
        >
          Delete
        </button>
        <button type="button" onClick={handleExport} disabled={!selectedId} style={barButton('default', Boolean(selectedId))}>
          Export
        </button>
        <button type="button" onClick={handleClone} disabled={!selectedId || cloning} style={barButton('default', Boolean(selectedId) && !cloning)}>
          {cloning ? 'Cloning…' : 'Clone'}
        </button>
        <button
          type="button"
          onClick={() => setStatus('Import will be available once database storage lands.')}
          style={barButton('default', true)}
        >
          Import
        </button>
        <div style={{ flex: 1 }} />
        <button type="button" onClick={props.onCancel} style={barButton('default', true)}>
          Cancel
        </button>
      </div>
    </section>
  );
}

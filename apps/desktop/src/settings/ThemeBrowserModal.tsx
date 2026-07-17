import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import { createPortal } from 'react-dom';
import { loadObsidianThemeCatalog, rawFileUrl, type ObsidianThemeCatalogEntry } from './obsidianThemeCatalog';
import { getInstalledThemes, installTheme, uninstallTheme, setActiveThemeId, getActiveThemeId, type InstalledTheme } from './communityTheme';

/**
 * "Manage" popup: browses Obsidian's real community theme catalog, installs
 * a theme's actual `theme.css` from its GitHub repo, and lets it be applied.
 * Patterned after Obsidian's own community-themes browser (search + filters
 * on the left, a detail pane with screenshots and Install/Use on the right).
 *
 * Rendered via a portal straight onto `document.body` rather than in place:
 * this is opened from inside AppearancePanel, itself nested inside
 * SettingsModal's own fixed-position overlay. Two nested `position: fixed`
 * layers plus a large injected community stylesheet was enough to trip a
 * WebKitGTK compositing bug (verified — computed styles were byte-identical
 * between the broken and correct renders; only the painted pixels differed).
 * A portal escapes the nested overlay entirely, which is standard practice
 * for modals-within-modals anyway.
 */

function ThemeThumbnail(props: { repo: string; screenshot: string; alt: string }) {
  const [failed, setFailed] = useState(false);
  if (failed) {
    return (
      <div
        style={{
          alignItems: 'center',
          backgroundColor: 'var(--color-surface-2)',
          color: 'var(--color-text-faint)',
          display: 'flex',
          fontSize: '0.7rem',
          height: '100%',
          justifyContent: 'center',
          width: '100%',
        }}
      >
        No preview
      </div>
    );
  }
  return (
    <img
      src={rawFileUrl(props.repo, props.screenshot)}
      alt={props.alt}
      onError={() => setFailed(true)}
      style={{ display: 'block', height: '100%', objectFit: 'cover', width: '100%' }}
    />
  );
}

function ModeBadges(props: { modes: Array<'dark' | 'light'> }) {
  return (
    <span style={{ display: 'flex', gap: '0.3rem' }}>
      {props.modes.map((mode) => (
        <span
          key={mode}
          style={{
            backgroundColor: mode === 'dark' ? '#191919' : '#efe7d8',
            border: '1px solid var(--color-border)',
            borderRadius: 4,
            color: mode === 'dark' ? '#faf2d6' : '#2c2419',
            fontSize: '0.62rem',
            fontWeight: 700,
            padding: '0.05rem 0.35rem',
            textTransform: 'uppercase',
          }}
        >
          {mode}
        </span>
      ))}
    </span>
  );
}

const buttonStyle = (variant: 'primary' | 'default' | 'danger'): CSSProperties => ({
  backgroundColor: variant === 'primary' ? 'var(--color-accent)' : 'var(--color-surface-2)',
  border: `1px solid ${variant === 'danger' ? 'var(--color-error-border)' : 'var(--color-border)'}`,
  borderRadius: 6,
  color: variant === 'primary' ? 'var(--color-on-accent)' : variant === 'danger' ? 'var(--color-error)' : 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.85rem',
  fontWeight: 600,
  padding: '0.45rem 0.9rem',
});

export function ThemeBrowserModal(props: { open: boolean; onClose: () => void; onChange: () => void }) {
  const [catalog, setCatalog] = useState<ObsidianThemeCatalogEntry[] | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [search, setSearch] = useState('');
  const [installedOnly, setInstalledOnly] = useState(false);
  const [darkOnly, setDarkOnly] = useState(false);
  const [selected, setSelected] = useState<ObsidianThemeCatalogEntry | null>(null);
  const [installed, setInstalled] = useState<InstalledTheme[]>([]);
  const [activeId, setActiveId] = useState<string | null>(null);
  const [busyRepo, setBusyRepo] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);

  function refreshInstalled() {
    setInstalled(getInstalledThemes());
    setActiveId(getActiveThemeId());
  }

  useEffect(() => {
    if (!props.open) {
      return;
    }
    refreshInstalled();
    if (!catalog && !loadError) {
      loadObsidianThemeCatalog()
        .then(setCatalog)
        .catch((cause: unknown) => setLoadError(cause instanceof Error ? cause.message : 'Failed to load the theme catalog.'));
    }
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        props.onClose();
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [props.open]);

  const installedIds = useMemo(() => new Set(installed.map((theme) => theme.id)), [installed]);

  const filtered = useMemo(() => {
    if (!catalog) {
      return [];
    }
    const term = search.trim().toLowerCase();
    return catalog.filter((entry) => {
      if (installedOnly && !installedIds.has(entry.repo)) {
        return false;
      }
      if (darkOnly && !entry.modes.includes('dark')) {
        return false;
      }
      if (term && !entry.name.toLowerCase().includes(term) && !entry.author.toLowerCase().includes(term)) {
        return false;
      }
      return true;
    });
  }, [catalog, search, installedOnly, darkOnly, installedIds]);

  if (!props.open) {
    return null;
  }

  const selectedInstalled = selected ? installed.find((theme) => theme.id === selected.repo) : undefined;
  const isActive = Boolean(selected && activeId === selected.repo);

  async function handleInstall(entry: ObsidianThemeCatalogEntry) {
    setBusyRepo(entry.repo);
    setActionError(null);
    try {
      await installTheme(entry);
      refreshInstalled();
    } catch (cause: unknown) {
      setActionError(cause instanceof Error ? cause.message : `Failed to install "${entry.name}".`);
    } finally {
      setBusyRepo(null);
    }
  }

  function handleUninstall(id: string) {
    uninstallTheme(id);
    refreshInstalled();
    props.onChange();
  }

  function handleUse(id: string) {
    setActiveThemeId(id);
    refreshInstalled();
    props.onChange();
  }

  function handleUseWasp() {
    setActiveThemeId(null);
    refreshInstalled();
    props.onChange();
  }

  return createPortal(
    <div
      role="presentation"
      onClick={props.onClose}
      style={{
        alignItems: 'center',
        backgroundColor: 'rgba(0, 0, 0, 0.6)',
        display: 'flex',
        inset: 0,
        justifyContent: 'center',
        padding: '2rem',
        position: 'fixed',
        zIndex: 1100,
      }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Manage themes"
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 12,
          boxShadow: '0 24px 60px rgba(0, 0, 0, 0.55)',
          display: 'flex',
          flexDirection: 'column',
          height: 'min(680px, 90vh)',
          width: 'min(980px, 94vw)',
        }}
      >
        <header
          style={{
            alignItems: 'center',
            borderBottom: '1px solid var(--color-border)',
            display: 'flex',
            justifyContent: 'space-between',
            padding: '1rem 1.5rem',
          }}
        >
          <div>
            <h2 style={{ fontSize: '1.1rem', margin: 0 }}>Community themes</h2>
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: '0.15rem 0 0' }}>
              Live from Obsidian's public theme catalog — installing fetches the theme author's real theme.css.
            </p>
          </div>
          <button
            type="button"
            aria-label="Close"
            onClick={props.onClose}
            style={{ background: 'none', border: 'none', color: 'var(--color-text-muted)', cursor: 'pointer', fontSize: '1.4rem', lineHeight: 1, padding: '0.15rem 0.35rem' }}
          >
            ×
          </button>
        </header>

        <div style={{ display: 'flex', flex: 1, minHeight: 0 }}>
          {/* Left: search, filters, list */}
          <div style={{ borderRight: '1px solid var(--color-border)', display: 'flex', flex: '0 0 340px', flexDirection: 'column', minHeight: 0 }}>
            <div style={{ borderBottom: '1px solid var(--color-border)', padding: '0.75rem' }}>
              <input
                type="text"
                placeholder="Filter…"
                value={search}
                onChange={(event) => setSearch(event.target.value)}
                style={{
                  backgroundColor: 'var(--color-surface-2)',
                  border: '1px solid var(--color-border)',
                  borderRadius: 6,
                  boxSizing: 'border-box',
                  color: 'var(--color-text)',
                  padding: '0.45rem 0.6rem',
                  width: '100%',
                }}
              />
              <label style={{ alignItems: 'center', display: 'flex', fontSize: '0.82rem', gap: '0.5rem', justifyContent: 'space-between', marginTop: '0.6rem' }}>
                Show installed only
                <input type="checkbox" checked={installedOnly} onChange={(event) => setInstalledOnly(event.target.checked)} />
              </label>
              <label style={{ alignItems: 'center', display: 'flex', fontSize: '0.82rem', gap: '0.5rem', justifyContent: 'space-between', marginTop: '0.4rem' }}>
                Dark themes only
                <input type="checkbox" checked={darkOnly} onChange={(event) => setDarkOnly(event.target.checked)} />
              </label>
              <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.5rem 0 0' }}>
                {catalog ? `Showing ${filtered.length} of ${catalog.length} themes` : loadError ? 'Catalog unavailable' : 'Loading catalog…'}
              </p>
            </div>

            <div style={{ flex: 1, overflowY: 'auto' }}>
              {loadError ? (
                <p style={{ color: 'var(--color-error)', fontSize: '0.85rem', padding: '1rem' }}>{loadError}</p>
              ) : null}
              {!catalog && !loadError ? <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', padding: '1rem' }}>Loading…</p> : null}
              {filtered.map((entry) => {
                const active = selected?.repo === entry.repo;
                const isInstalled = installedIds.has(entry.repo);
                const isEntryActive = activeId === entry.repo;
                return (
                  <button
                    key={entry.repo}
                    type="button"
                    onClick={() => setSelected(entry)}
                    style={{
                      alignItems: 'center',
                      background: active ? 'var(--color-surface-2)' : 'none',
                      border: 'none',
                      borderBottom: '1px solid var(--color-border)',
                      cursor: 'pointer',
                      display: 'flex',
                      gap: '0.7rem',
                      padding: '0.55rem 0.75rem',
                      textAlign: 'left',
                      width: '100%',
                    }}
                  >
                    <div style={{ backgroundColor: 'var(--color-surface-2)', borderRadius: 6, flexShrink: 0, height: 40, overflow: 'hidden', width: 56 }}>
                      <ThemeThumbnail repo={entry.repo} screenshot={entry.screenshot} alt={entry.name} />
                    </div>
                    <div style={{ minWidth: 0 }}>
                      <p style={{ color: isEntryActive ? 'var(--color-accent)' : 'var(--color-text)', fontWeight: 700, margin: 0, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                        {entry.name}
                      </p>
                      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.1rem 0 0', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                        By {entry.author}
                      </p>
                    </div>
                    <div style={{ marginLeft: 'auto', flexShrink: 0 }}>
                      {isInstalled ? (
                        <span style={{ color: 'var(--color-accent)', fontSize: '0.68rem', fontWeight: 700, textTransform: 'uppercase' }}>Installed</span>
                      ) : null}
                    </div>
                  </button>
                );
              })}
            </div>
          </div>

          {/* Right: detail pane */}
          <div style={{ flex: 1, minWidth: 0, overflowY: 'auto', padding: '1.25rem 1.5rem' }}>
            {!selected ? (
              <p style={{ color: 'var(--color-text-muted)' }}>Select a theme on the left to see details and install it.</p>
            ) : (
              <div>
                <div style={{ alignItems: 'flex-start', display: 'flex', justifyContent: 'space-between' }}>
                  <div>
                    <h3 style={{ margin: '0 0 0.3rem' }}>
                      {selected.name} {isActive ? <span style={{ color: 'var(--color-accent)', fontSize: '0.8rem', fontWeight: 700 }}>· Active</span> : null}
                    </h3>
                    <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem', margin: '0 0 0.4rem' }}>
                      By {selected.author} ·{' '}
                      <a href={`https://github.com/${selected.repo}`} target="_blank" rel="noreferrer" style={{ color: 'var(--color-link)' }}>
                        {selected.repo}
                      </a>
                    </p>
                    <ModeBadges modes={selected.modes} />
                  </div>
                </div>

                <div style={{ backgroundColor: 'var(--color-surface-2)', borderRadius: 8, height: 220, marginTop: '1rem', overflow: 'hidden' }}>
                  <ThemeThumbnail repo={selected.repo} screenshot={selected.screenshot} alt={`${selected.name} preview`} />
                </div>

                <div style={{ display: 'flex', gap: '0.6rem', marginTop: '1rem' }}>
                  {selectedInstalled ? (
                    <>
                      <button
                        type="button"
                        onClick={() => handleUse(selectedInstalled.id)}
                        disabled={isActive}
                        style={{ ...buttonStyle(isActive ? 'default' : 'primary'), cursor: isActive ? 'default' : 'pointer', opacity: isActive ? 0.6 : 1 }}
                      >
                        {isActive ? 'Currently active' : 'Use this theme'}
                      </button>
                      <button type="button" onClick={() => handleUninstall(selectedInstalled.id)} style={buttonStyle('danger')}>
                        Uninstall
                      </button>
                    </>
                  ) : (
                    <button
                      type="button"
                      onClick={() => handleInstall(selected)}
                      disabled={busyRepo === selected.repo}
                      style={{ ...buttonStyle('primary'), opacity: busyRepo === selected.repo ? 0.6 : 1 }}
                    >
                      {busyRepo === selected.repo ? 'Installing…' : 'Install'}
                    </button>
                  )}
                </div>

                {actionError ? <p style={{ color: 'var(--color-error)', fontSize: '0.85rem', marginTop: '0.75rem' }}>{actionError}</p> : null}

                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', lineHeight: 1.6, marginTop: '1.25rem' }}>
                  Installing fetches <code>theme.css</code> directly from {selected.repo} on GitHub and applies it via the same{' '}
                  <code>.theme-dark</code> / <code>.theme-light</code> variables Obsidian itself uses, so most community themes render
                  correctly without any changes on our side.
                </p>
              </div>
            )}
          </div>
        </div>

        <footer style={{ borderTop: '1px solid var(--color-border)', display: 'flex', justifyContent: 'flex-end', padding: '0.85rem 1.5rem' }}>
          <button type="button" onClick={handleUseWasp} disabled={!activeId} style={{ ...buttonStyle('default'), opacity: activeId ? 1 : 0.5 }}>
            Revert to Wasp (built-in)
          </button>
        </footer>
      </div>
    </div>,
    document.body
  );
}

import { useEffect, useState, type ReactNode } from 'react';
import type { ThemeMode } from './themeMode';
import { getInstalledThemes, getActiveThemeId, setActiveThemeId, type InstalledTheme } from './communityTheme';
import { ThemeBrowserModal } from './ThemeBrowserModal';

/** One labelled control row, mirroring Obsidian's settings layout. */
export function SettingRow(props: { name: string; description: string; control: ReactNode }) {
  return (
    <div
      style={{
        alignItems: 'center',
        borderBottom: '1px solid var(--color-border)',
        display: 'flex',
        gap: '1.5rem',
        justifyContent: 'space-between',
        padding: '1rem 0',
      }}
    >
      <div style={{ minWidth: 0 }}>
        <p style={{ fontWeight: 600, margin: 0 }}>{props.name}</p>
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', margin: '0.2rem 0 0' }}>{props.description}</p>
      </div>
      <div style={{ flexShrink: 0 }}>{props.control}</div>
    </div>
  );
}

export const selectStyle = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.9rem',
  padding: '0.4rem 0.6rem',
} as const;

const WASP_VALUE = '';

export function AppearancePanel(props: { mode: ThemeMode; onModeChange: (mode: ThemeMode) => void }) {
  const [browserOpen, setBrowserOpen] = useState(false);
  const [installed, setInstalled] = useState<InstalledTheme[]>([]);
  const [activeId, setActiveId] = useState<string | null>(null);

  function refresh() {
    setInstalled(getInstalledThemes());
    setActiveId(getActiveThemeId());
  }

  useEffect(() => {
    refresh();
  }, []);

  const activeTheme = installed.find((theme) => theme.id === activeId);

  return (
    <div>
      <SettingRow
        name="Base color scheme"
        description="Choose the app's default light or dark appearance."
        control={
          <select
            value={props.mode}
            onChange={(event) => props.onModeChange(event.target.value as ThemeMode)}
            style={selectStyle}
          >
            <option value="dark">Dark</option>
            <option value="light">Light</option>
          </select>
        }
      />
      <SettingRow
        name="Theme"
        description="Palette applied throughout the app. Install more from Obsidian's community catalog."
        control={
          <div style={{ alignItems: 'center', display: 'flex', gap: '0.6rem' }}>
            <select
              value={activeId ?? WASP_VALUE}
              onChange={(event) => {
                setActiveThemeId(event.target.value || null);
                refresh();
              }}
              style={selectStyle}
            >
              <option value={WASP_VALUE}>Wasp (built-in)</option>
              {installed.map((theme) => (
                <option key={theme.id} value={theme.id}>
                  {theme.name}
                </option>
              ))}
            </select>
            <button
              type="button"
              onClick={() => setBrowserOpen(true)}
              style={{
                backgroundColor: 'var(--color-surface-2)',
                border: '1px solid var(--color-border)',
                borderRadius: 6,
                color: 'var(--color-text)',
                cursor: 'pointer',
                fontSize: '0.85rem',
                fontWeight: 600,
                padding: '0.4rem 0.85rem',
              }}
            >
              Manage…
            </button>
          </div>
        }
      />
      {activeTheme ? (
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0.5rem 0 0' }}>
          Using <strong style={{ color: 'var(--color-text)' }}>{activeTheme.name}</strong> by {activeTheme.author} (
          <a href={`https://github.com/${activeTheme.repo}`} target="_blank" rel="noreferrer" style={{ color: 'var(--color-link)' }}>
            {activeTheme.repo}
          </a>
          ).
        </p>
      ) : null}
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', lineHeight: 1.6, marginTop: '1.25rem' }}>
        "Manage" browses Obsidian's live community theme catalog. Installing a theme downloads that theme's real{' '}
        <code>theme.css</code> from its GitHub repo and applies it directly — the same mechanism Obsidian itself uses.
      </p>

      <ThemeBrowserModal open={browserOpen} onClose={() => setBrowserOpen(false)} onChange={refresh} />
    </div>
  );
}

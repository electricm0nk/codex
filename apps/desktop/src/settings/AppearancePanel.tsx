import type { ReactNode } from 'react';
import type { ThemeMode } from './themeMode';

/** One labelled control row, mirroring Obsidian's settings layout. */
function SettingRow(props: { name: string; description: string; control: ReactNode }) {
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

const selectStyle = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.9rem',
  padding: '0.4rem 0.6rem',
} as const;

export function AppearancePanel(props: { mode: ThemeMode; onModeChange: (mode: ThemeMode) => void }) {
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
        description="Palette applied throughout the app."
        control={
          <select value="wasp" disabled style={{ ...selectStyle, cursor: 'not-allowed', opacity: 0.8 }}>
            <option value="wasp">Wasp (built-in)</option>
          </select>
        }
      />
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', lineHeight: 1.6, marginTop: '1.25rem' }}>
        Importing Obsidian theme packs is planned here: selecting a pack will map its Obsidian CSS variables onto the
        app's palette so community themes can be applied without a rebuild.
      </p>
    </div>
  );
}

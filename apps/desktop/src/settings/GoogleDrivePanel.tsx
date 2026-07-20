import { useState } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { SettingRow } from './AppearancePanel';
import {
  clearGoogleDriveConfig,
  getGoogleDriveConfig,
  isGoogleDriveConfigured,
  saveGoogleDriveConfig,
} from './googleDrive';
import { hasTauriRuntime } from '../boundary/runtime';
import { FriendsSection } from './FriendsSection';

const inputStyle = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  boxSizing: 'border-box',
  color: 'var(--color-text)',
  fontSize: '0.9rem',
  padding: '0.4rem 0.6rem',
  width: 260,
} as const;

/**
 * Local campaign-folder setup — the gate the Campaign Manager checks before
 * it's enabled. By design this is local-only: no OAuth, no cloud API, no
 * network calls, ever. The folder is a real local path (see
 * settings/googleDrive.ts) that campaigns write real files to; sharing
 * means handing someone that folder or its files directly.
 */
export function GoogleDrivePanel(props: { onChange?: () => void }) {
  const stored = getGoogleDriveConfig();
  const [accountEmail, setAccountEmail] = useState(stored?.accountEmail ?? '');
  const [driveFolderPath, setDriveFolderPath] = useState(stored?.driveFolderPath ?? '');
  const [configured, setConfigured] = useState(isGoogleDriveConfigured());

  function handleSave() {
    saveGoogleDriveConfig({ accountEmail: accountEmail.trim(), driveFolderPath: driveFolderPath.trim() });
    setConfigured(isGoogleDriveConfigured());
    props.onChange?.();
  }

  function handleDisconnect() {
    clearGoogleDriveConfig();
    setAccountEmail('');
    setDriveFolderPath('');
    setConfigured(false);
    props.onChange?.();
  }

  async function handleBrowseFolder() {
    if (!hasTauriRuntime()) {
      return;
    }
    const picked = await open({ directory: true, multiple: false });
    if (typeof picked === 'string') {
      setDriveFolderPath(picked);
    }
  }

  return (
    <div>
      <div
        style={{
          alignItems: 'center',
          backgroundColor: configured ? 'var(--color-warn-bg)' : 'var(--color-surface-2)',
          border: `1px solid ${configured ? 'var(--color-warn-border)' : 'var(--color-border)'}`,
          borderRadius: 8,
          display: 'flex',
          gap: '0.6rem',
          marginBottom: '1rem',
          padding: '0.65rem 0.9rem',
        }}
      >
        <span
          aria-hidden
          style={{
            backgroundColor: configured ? 'var(--color-accent)' : 'var(--color-text-faint)',
            borderRadius: '50%',
            flexShrink: 0,
            height: 8,
            width: 8,
          }}
        />
        <span style={{ fontSize: '0.85rem' }}>
          {configured ? (
            <>
              Connected as <strong>{stored?.accountEmail}</strong> — campaigns will be created under{' '}
              <strong>{stored?.driveFolderPath}</strong>.
            </>
          ) : (
            'Not connected. Fill in an account and a destination folder below to enable the Campaign Manager.'
          )}
        </span>
      </div>

      <SettingRow
        name="Google account"
        description="Reference only — whose campaign folder this is. Never used to sign in or send anything."
        control={
          <input
            type="email"
            placeholder="you@example.com"
            value={accountEmail}
            onChange={(event) => setAccountEmail(event.target.value)}
            style={inputStyle}
          />
        }
      />
      <SettingRow
        name="Campaign Drive folder"
        description="Destination folder for campaign subfolders, e.g. My Drive/TTRPG Campaigns."
        control={
          <div style={{ alignItems: 'center', display: 'flex', gap: '0.6rem' }}>
            <input
              type="text"
              placeholder="My Drive/TTRPG Campaigns"
              value={driveFolderPath}
              onChange={(event) => setDriveFolderPath(event.target.value)}
              style={inputStyle}
            />
            <button
              type="button"
              onClick={handleBrowseFolder}
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
              Browse…
            </button>
          </div>
        }
      />

      <div style={{ display: 'flex', gap: '0.6rem', marginTop: '1rem' }}>
        <button
          type="button"
          onClick={handleSave}
          disabled={!accountEmail.trim() || !driveFolderPath.trim()}
          style={{
            backgroundColor: 'var(--color-accent)',
            border: 'none',
            borderRadius: 6,
            color: 'var(--color-on-accent)',
            cursor: accountEmail.trim() && driveFolderPath.trim() ? 'pointer' : 'not-allowed',
            fontSize: '0.85rem',
            fontWeight: 600,
            opacity: accountEmail.trim() && driveFolderPath.trim() ? 1 : 0.5,
            padding: '0.45rem 0.9rem',
          }}
        >
          Save
        </button>
        <button
          type="button"
          onClick={handleDisconnect}
          disabled={!configured}
          style={{
            backgroundColor: 'var(--color-surface-2)',
            border: '1px solid var(--color-error-border)',
            borderRadius: 6,
            color: 'var(--color-error)',
            cursor: configured ? 'pointer' : 'not-allowed',
            fontSize: '0.85rem',
            fontWeight: 600,
            opacity: configured ? 1 : 0.5,
            padding: '0.45rem 0.9rem',
          }}
        >
          Disconnect
        </button>
      </div>

      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', lineHeight: 1.6, marginTop: '1.25rem' }}>
        Codex runs entirely on your machine — there's no cloud sync, no account sign-in, and no network calls.
        "Drive folder" just means a local folder (point it at a Drive/Dropbox/Syncthing-synced folder if you want
        campaigns to sync between devices that way). To share a campaign, hand someone its folder or files directly.
      </p>

      <div style={{ borderTop: '1px solid var(--color-border)', marginTop: '1.75rem', paddingTop: '1.5rem' }}>
        <FriendsSection />
      </div>
    </div>
  );
}

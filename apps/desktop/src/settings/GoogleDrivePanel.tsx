import { useState } from 'react';
import { SettingRow } from './AppearancePanel';
import {
  clearGoogleDriveConfig,
  getGoogleDriveConfig,
  isGoogleDriveConfigured,
  saveGoogleDriveConfig,
} from './googleDrive';

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
 * Google Drive setup — the gate the Campaign Manager checks before it's
 * enabled. There's no real OAuth/Drive API integration yet; this captures
 * the account + destination folder and treats both being filled in as
 * "setup completed," which is enough to build the rest of the Campaign
 * Manager flow against a real (if locally-persisted) gate.
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
        description="The account campaign folders and invites will be sent from."
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
          <input
            type="text"
            placeholder="My Drive/TTRPG Campaigns"
            value={driveFolderPath}
            onChange={(event) => setDriveFolderPath(event.target.value)}
            style={inputStyle}
          />
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
        This does not yet perform real Google OAuth or talk to the Drive API — that requires backend work (an OAuth
        flow, token storage, and Drive API calls). Saving here only records the account and folder so the rest of
        the Campaign Manager has a real setup gate to build against.
      </p>
    </div>
  );
}

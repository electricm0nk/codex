/**
 * Local campaign-folder config — the gate for the Campaign Manager.
 *
 * By design, this app never talks to Google's (or anyone's) cloud API — no
 * OAuth, no token storage, no network calls. "Drive folder" just means a
 * local path (typically a Drive/Dropbox/Syncthing desktop-sync-client
 * mirror, or any plain folder). Campaigns write real files there (see
 * campaignModel.ts's `writeCampaignLocalFolderArtifacts`); sharing a campaign means
 * handing someone that folder or its files directly. `accountEmail` is
 * reference metadata only (whose folder this is) — it is never used to
 * authenticate or contact anyone. Persisted to localStorage.
 */

export interface GoogleDriveConfig {
  accountEmail: string;
  driveFolderPath: string;
}

const STORAGE_KEY = 'codex.googleDrive.config';

export function getGoogleDriveConfig(): GoogleDriveConfig | null {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    return raw ? (JSON.parse(raw) as GoogleDriveConfig) : null;
  } catch {
    return null;
  }
}

export function saveGoogleDriveConfig(config: GoogleDriveConfig): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(config));
  } catch {
    /* non-persistent environments still get the config applied this session */
  }
}

export function clearGoogleDriveConfig(): void {
  try {
    localStorage.removeItem(STORAGE_KEY);
  } catch {
    /* ignore */
  }
}

export function isGoogleDriveConfigured(): boolean {
  const config = getGoogleDriveConfig();
  return Boolean(config?.accountEmail.trim() && config?.driveFolderPath.trim());
}

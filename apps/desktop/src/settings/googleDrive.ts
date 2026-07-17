/**
 * Google Drive connection config — the gate for the Campaign Manager.
 *
 * There is no real Google OAuth or Drive API integration yet (that's
 * backend work: an OAuth flow, token storage, and Drive API calls from
 * Rust). This captures the two pieces of information that integration will
 * need (the account and the destination folder) and treats "both filled in"
 * as "setup completed" so the rest of the UI has a real gate to build
 * against. Persisted to localStorage today, same as other not-yet-backed
 * settings in this app.
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

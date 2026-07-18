/**
 * Campaign Manager's first-run access gate on the character-hub landing
 * page.
 *
 * The Campaign Manager needs a configured local folder (see
 * `settings/googleDrive.ts`) before it's meaningful to open. There is no
 * real Google OAuth / Drive API integration — that's descoped for SD-21
 * (see the Epic 2 engine-shape addendum, 2026-07-18) — so the landing
 * page's gating copy must say "local folder", not imply an
 * account-authorization flow the app doesn't actually have.
 */

export interface CampaignManagerAccessGate {
  enabled: boolean;
  /** Empty once `enabled` — there is nothing to explain. */
  disabledHint: string;
}

export function computeCampaignManagerAccessGate(localFolderConfigured: boolean): CampaignManagerAccessGate {
  if (localFolderConfigured) {
    return { enabled: true, disabledHint: '' };
  }
  return {
    enabled: false,
    disabledHint: 'Choose a local Drive-synced folder under ⚙ Settings → Google Drive to enable the Campaign Manager.',
  };
}

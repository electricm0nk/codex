/**
 * SD-16-E6 shared update-UI model.
 *
 * This module is the contract surface consumed by the F3c shell-update-UI
 * slice. It declares the type shapes the UI renders against, plus the
 * permissive defaults the UI must surface when its dependencies (F3a
 * fetch/index parser and F3b eligibility/diagnostics rules) have not yet
 * landed on `origin/develop`.
 *
 * The F1 closure
 * (`programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/artifacts/SD16-E6-execution-readiness-closure-2026-07-03.md`)
 * places F3a (fetch + validate) and F3b (eligibility + diagnostics model)
 * as sibling slices that F3c wires into the shell. F3c is therefore the
 * last of the three to land on `develop`; the UI must be reviewable on
 * its own without forcing the reviewer to chase F3a / F3b branches.
 *
 * Three guarantees this module honors:
 *
 *   1. **Type fidelity.** The shape names (channel order, eight installed
 *      fields, eight last_check fields, five pending/rollback fields) are
 *      exact. AV-* ids in the canonical map enforce them.
 *   2. **Lazy runtime.** Runtime values cross the slice boundary through
 *      `Sd16UpdateController`, an opaque controller object F3a and F3b
 *      will produce and F3c will receive. The UI never reaches into a
 *      fetcher directly; the controller is the seam.
 *   3. **Deterministic defaults.** Until the controller is wired by the
 *      runtime, the UI surfaces honest "unknown" / "not-loaded" states
 *      (not fabricated eligibility). The `#install-disabled-reason` hook
 *      is populated with a real reason string in every non-eligible
 *      state, including the freshly-mounted "no controller wired yet"
 *      state.
 */

import type { CSSProperties } from 'react';

/**
 * The pinned channel selector option order.
 *
 * AV-UI-1 pins the rendered option list to exactly this array, in this
 * order. Different ordering is wrong even if it would befriend a human
 * reviewer's eye: the order is the **release-promotion order**, not the
 * release-stability order.
 */
export const SD16_UPDATE_CHANNEL_OPTIONS = ['alpha', 'beta', 'stable'] as const;

export type Sd16UpdateChannelLabel = (typeof SD16_UPDATE_CHANNEL_OPTIONS)[number];

/**
 * Eligibility verdict surfaced by F3b's rules engine. The UI must treat
 * anything that is not `eligible` as Install-disabled.
 */
export type Sd16EligibilityResult = 'eligible' | 'ineligible' | 'unknown';

/**
 * Categorical install provenance reported by the shell runtime. The
 * eligibility table from the F1 closure matches on these values.
 */
export type Sd16InstallKind =
  | 'appimage'
  | 'tarball'
  | 'dev'
  | 'unknown';

export interface Sd16InstalledState {
  channel: Sd16UpdateChannelLabel;
  version: string;
  sourceCommit: string | null;
  artifactSha256: string | null;
  installKind: Sd16InstallKind;
  managedExecutablePath: string | null;
  updateEligible: boolean;
  ineligibleReason: string | null;
}

export type Sd16LastCheckFetchStatus =
  | 'not-loaded'
  | 'in-progress'
  | 'ok'
  | 'failed';

export interface Sd16LastCheckState {
  selectedChannel: Sd16UpdateChannelLabel;
  indexUrl: string;
  indexStatus: Sd16LastCheckFetchStatus;
  manifestStatus: Sd16LastCheckFetchStatus;
  releaseVersion: string | null;
  releaseNotesStatus: 'not-loaded' | 'loaded' | 'unavailable';
  eligibilityResult: Sd16EligibilityResult;
  installDisabledReason: string | null;
}

export interface Sd16PendingRollbackState {
  pendingUpdateState: 'idle' | 'pending-relaunch' | 'unknown';
  previousVersionAvailable: boolean;
  rollbackState: 'none' | 'available' | 'unknown';
  backupCount: number;
  retainedUpdateStorageBytes: number;
}

/**
 * Release-notes content the UI renders after a successful Check.
 *
 * The `body` is plain-text Markdown rendered without a third-party
 * Markdown library: the diagnostic surface is meant to be readable,
 * not feature-rich. If `body` is null the UI surfaces an explicit
 * "no release notes available" placeholder rather than an empty pane.
 */
export interface Sd16ReleaseNotes {
  releaseVersion: string;
  body: string | null;
}

/**
 * An opaque connection object that the F3c UI talks to for fetch and
 * eligibility work. The runtime in `Ui.tsx` constructs a default
 * implementation that surfaces the deterministic "not wired" state; a
 * later wiring (F3a + F3b's loader hook) supplies the real one.
 *
 * Tests and downstream integration both pass their own implementation
 * through this interface. The UI never reaches past this boundary.
 */
export interface Sd16UpdateController {
  /** Run a bounded Check against the selected channel. */
  runCheck(channel: Sd16UpdateChannelLabel): Promise<void>;
  /** Compute current eligibility verdict for the selected channel. */
  computeEligibility(
    installed: Sd16InstalledState,
    lastCheck: Sd16LastCheckState,
  ): Sd16EligibilityResult;
  /** The human-readable disabled reason pinned by F3b's decision table. */
  disabledReason(
    installed: Sd16InstalledState,
    lastCheck: Sd16LastCheckState,
  ): string | null;
  /** Resolved release notes after a successful check, or null. */
  releaseNotes(): Sd16ReleaseNotes | null;
}

export interface Sd16UpdateControllerDeps {
  installed: Sd16InstalledState;
  lastCheck: Sd16LastCheckState;
  pendingRollback: Sd16PendingRollbackState;
  releaseNotes: Sd16ReleaseNotes | null;
  controller: Sd16UpdateController;
}

/**
 * The "controller not yet wired" surface — used until F3a + F3b land.
 *
 * `runCheck` resolves without mutating; `computeEligibility` returns
 * `'unknown'`; `disabledReason` is an explicit "controller not wired"
 * reason so AV-UI-5 always passes and the Install button stays
 * disabled for honest reasons rather than fabricated eligibility.
 */
export const SD16_UNWIRED_CONTROLLER: Sd16UpdateController = {
  async runCheck(_channel: Sd16UpdateChannelLabel): Promise<void> {
    return;
  },
  computeEligibility(
    _installed: Sd16InstalledState,
    _lastCheck: Sd16LastCheckState,
  ): Sd16EligibilityResult {
    return 'unknown';
  },
  disabledReason(
    _installed: Sd16InstalledState,
    _lastCheck: Sd16LastCheckState,
  ): string | null {
    return 'update controller not wired (F3a/F3b pending)';
  },
  releaseNotes(): Sd16ReleaseNotes | null {
    return null;
  },
};

export function buildUnwiredUpdateDeps(): Sd16UpdateControllerDeps {
  return {
    installed: emptyInstalledState(),
    lastCheck: emptyLastCheckState(),
    pendingRollback: emptyPendingRollbackState(),
    releaseNotes: null,
    controller: SD16_UNWIRED_CONTROLLER,
  };
}

export function emptyInstalledState(
  channel: Sd16UpdateChannelLabel = 'alpha',
): Sd16InstalledState {
  return {
    channel,
    version: 'unknown',
    sourceCommit: null,
    artifactSha256: null,
    installKind: 'unknown',
    managedExecutablePath: null,
    updateEligible: false,
    ineligibleReason: 'installed state not wired',
  };
}

export function emptyLastCheckState(
  channel: Sd16UpdateChannelLabel = 'alpha',
): Sd16LastCheckState {
  return {
    selectedChannel: channel,
    indexUrl: '',
    indexStatus: 'not-loaded',
    manifestStatus: 'not-loaded',
    releaseVersion: null,
    releaseNotesStatus: 'not-loaded',
    eligibilityResult: 'unknown',
    installDisabledReason: 'check has not been run yet',
  };
}

export function emptyPendingRollbackState(): Sd16PendingRollbackState {
  return {
    pendingUpdateState: 'unknown',
    previousVersionAvailable: false,
    rollbackState: 'unknown',
    backupCount: 0,
    retainedUpdateStorageBytes: 0,
  };
}

/**
 * Inline styles shared by every component — keeps the visual surface
 * reviewable without pulling in a CSS-in-JS dependency this slice is
 * forbidden from introducing.
 */
export const SD16_UI_CONTAINER_STYLE: CSSProperties = {
  fontFamily:
    '-apple-system, BlinkMacSystemFont, "Segoe UI", "Helvetica Neue", Arial, sans-serif',
  fontSize: '14px',
  lineHeight: 1.45,
  color: '#1f2328',
  maxWidth: '720px',
  margin: '0 auto',
  padding: '16px',
};

export const SD16_UI_PANEL_STYLE: CSSProperties = {
  border: '1px solid #d0d7de',
  borderRadius: '6px',
  padding: '12px',
  marginBottom: '12px',
  backgroundColor: '#ffffff',
};

export const SD16_UI_PANEL_TITLE_STYLE: CSSProperties = {
  fontSize: '13px',
  fontWeight: 600,
  textTransform: 'uppercase',
  letterSpacing: '0.04em',
  color: '#57606a',
  margin: '0 0 8px 0',
};

export const SD16_UI_FIELD_LABEL_STYLE: CSSProperties = {
  display: 'inline-block',
  width: '180px',
  color: '#57606a',
  fontVariantNumeric: 'tabular-nums',
};

export const SD16_UI_BADGE_DISABLED_STYLE: CSSProperties = {
  display: 'inline-block',
  padding: '2px 8px',
  borderRadius: '999px',
  backgroundColor: '#fff8c5',
  color: '#7d4e00',
  border: '1px solid #d4a72c',
  fontSize: '12px',
  fontWeight: 500,
  marginLeft: '8px',
};

export const SD16_UI_BUTTON_BASE_STYLE: CSSProperties = {
  padding: '6px 14px',
  borderRadius: '6px',
  fontSize: '14px',
  fontWeight: 500,
  cursor: 'pointer',
  border: '1px solid #1f2328',
};

export const SD16_UI_BUTTON_DISABLED_STYLE: CSSProperties = {
  ...SD16_UI_BUTTON_BASE_STYLE,
  backgroundColor: '#eaeef2',
  color: '#57606a',
  border: '1px solid #d0d7de',
  cursor: 'not-allowed',
};

export const SD16_UI_BUTTON_ENABLED_STYLE: CSSProperties = {
  ...SD16_UI_BUTTON_BASE_STYLE,
  backgroundColor: '#1f883d',
  color: '#ffffff',
  border: '1px solid #1a7f37',
};

export const SD16_UI_NOTES_BLOCK_STYLE: CSSProperties = {
  whiteSpace: 'pre-wrap',
  fontFamily:
    'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace',
  fontSize: '12px',
  lineHeight: 1.45,
  backgroundColor: '#f6f8fa',
  border: '1px solid #d0d7de',
  borderRadius: '6px',
  padding: '8px 10px',
  maxHeight: '180px',
  overflowY: 'auto',
};

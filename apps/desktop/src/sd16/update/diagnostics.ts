/**
 * SD16-E6-F3b — shell update diagnostics model.
 *
 * Three pure typed shapes whose field names match `technical-requirements.md`
 * §"Diagnostics Requirements" verbatim, plus deterministic default factories and
 * render helpers. F3b owns the data-model surface; F3c renders it into the
 * `#installed-panel`, `#last-check-panel`, and `#pending-rollback-panel` DOM
 * hooks. The `pending/rollback.*` group has a soft dependency on E7 (E6 -> E8
 * gate): the contract is that every key is surfaced with a deterministic
 * placeholder, not that the values are non-zero.
 */

import type { ChannelLabel, EligibilityResult, FetchStatus, InstallKind } from './eligibility';

export type { ChannelLabel, EligibilityResult, FetchStatus, InstallKind } from './eligibility';

/** State of a staged/pending update (E7 owns the real transitions). */
export type PendingUpdateState =
  | 'none'
  | 'pending-relaunch'
  | 'verification-failed'
  | 'rollback-failed';

/** Outcome of the most recent rollback attempt (E7 owns the real transitions). */
export type RollbackState = 'ok' | 'rollback-failed' | 'unknown';

/** Deterministic placeholder rendered when a diagnostics value is not yet known. */
export const DIAGNOSTIC_PLACEHOLDER = 'unknown';

// Installed-state group — eight fields.
export interface InstalledDiagnostics {
  channel: ChannelLabel;
  version: string;
  source_commit: string;
  artifact_sha256: string;
  install_kind: InstallKind;
  managed_executable_path: string | null;
  update_eligible: boolean;
  ineligible_reason: string | null;
}

// Last-check group — eight fields.
export interface LastCheckDiagnostics {
  selected_channel: ChannelLabel;
  index_url: string;
  index_status: FetchStatus;
  manifest_status: FetchStatus;
  release_version: string | null;
  release_notes_status: FetchStatus;
  eligibility_result: EligibilityResult;
  install_disabled_reason: string | null;
}

// Pending / rollback group — five fields.
export interface PendingRollbackDiagnostics {
  pending_update_state: PendingUpdateState;
  previous_version_available: boolean;
  rollback_state: RollbackState;
  backup_count: number;
  retained_update_storage_bytes: number;
}

/** Installed-state defaults: fail-closed sentinels until E7/E8 supplies truth. */
export function defaultInstalledDiagnostics(): InstalledDiagnostics {
  return {
    channel: 'alpha',
    version: '',
    source_commit: '',
    artifact_sha256: '',
    install_kind: 'unknown',
    managed_executable_path: null,
    update_eligible: false,
    ineligible_reason: null,
  };
}

/** Last-check defaults: no check has run yet, so fetch statuses fail-closed. */
export function defaultLastCheckDiagnostics(): LastCheckDiagnostics {
  return {
    selected_channel: 'alpha',
    index_url: '',
    index_status: 'failed',
    manifest_status: 'failed',
    release_version: null,
    release_notes_status: 'failed',
    eligibility_result: 'unknown',
    install_disabled_reason: null,
  };
}

/** Pending/rollback defaults: zeros and `none`/`unknown` sentinels (E6 -> E8 gate). */
export function defaultPendingRollbackDiagnostics(): PendingRollbackDiagnostics {
  return {
    pending_update_state: 'none',
    previous_version_available: false,
    rollback_state: 'unknown',
    backup_count: 0,
    retained_update_storage_bytes: 0,
  };
}

/** Render-ready installed view — every key a non-empty deterministic string. */
export interface InstalledDiagnosticsView {
  channel: string;
  version: string;
  source_commit: string;
  artifact_sha256: string;
  install_kind: string;
  managed_executable_path: string;
  update_eligible: string;
  ineligible_reason: string;
}

/** Render-ready last-check view. */
export interface LastCheckDiagnosticsView {
  selected_channel: string;
  index_url: string;
  index_status: string;
  manifest_status: string;
  release_version: string;
  release_notes_status: string;
  eligibility_result: string;
  install_disabled_reason: string;
}

/** Render-ready pending/rollback view. */
export interface PendingRollbackDiagnosticsView {
  pending_update_state: string;
  previous_version_available: string;
  rollback_state: string;
  backup_count: string;
  retained_update_storage_bytes: string;
}

/**
 * Map an installed model to a deterministic render-ready object whose keys match
 * the `installed.*` field set exactly. Empty strings and `null` collapse to the
 * placeholder; booleans render as `yes`/`no`. No value is ever `undefined`.
 */
export function renderInstalledDiagnostics(model: InstalledDiagnostics): InstalledDiagnosticsView {
  return {
    channel: text(model.channel),
    version: text(model.version),
    source_commit: text(model.source_commit),
    artifact_sha256: text(model.artifact_sha256),
    install_kind: text(model.install_kind),
    managed_executable_path: text(model.managed_executable_path),
    update_eligible: yesNo(model.update_eligible),
    ineligible_reason: text(model.ineligible_reason),
  };
}

/** Map a last-check model to a deterministic render-ready object. */
export function renderLastCheckDiagnostics(model: LastCheckDiagnostics): LastCheckDiagnosticsView {
  return {
    selected_channel: text(model.selected_channel),
    index_url: text(model.index_url),
    index_status: text(model.index_status),
    manifest_status: text(model.manifest_status),
    release_version: text(model.release_version),
    release_notes_status: text(model.release_notes_status),
    eligibility_result: text(model.eligibility_result),
    install_disabled_reason: text(model.install_disabled_reason),
  };
}

/** Map a pending/rollback model to a deterministic render-ready object. */
export function renderPendingRollbackDiagnostics(
  model: PendingRollbackDiagnostics
): PendingRollbackDiagnosticsView {
  return {
    pending_update_state: text(model.pending_update_state),
    previous_version_available: yesNo(model.previous_version_available),
    rollback_state: text(model.rollback_state),
    backup_count: String(model.backup_count),
    retained_update_storage_bytes: String(model.retained_update_storage_bytes),
  };
}

function text(value: string | null): string {
  return value === null || value === '' ? DIAGNOSTIC_PLACEHOLDER : value;
}

function yesNo(value: boolean): string {
  return value ? 'yes' : 'no';
}

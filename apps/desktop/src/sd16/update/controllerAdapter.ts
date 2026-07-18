/**
 * Character Hub Phase 3 — real `UpdateController` adapter.
 *
 * Bridges the sd16/update UI to the parts of the update system that are
 * genuinely real today: `fetch.ts`'s discovery fetch/validate, `eligibility.ts`'s
 * pure decision table, and the two Tauri commands that have real, tested
 * bodies (`verify_relaunch_artifact`, `perform_restore_previous`).
 *
 * `is_install_eligible` and `perform_install` remain honest stubs (deferred
 * to a future slice — see the Phase 3 plan), so this module never calls
 * them and never fabricates local install-eligibility truth it cannot
 * verify: whenever a real fetch check succeeds but no trustworthy local
 * installed-state exists, eligibility stays `'unknown'` with an honest
 * reason rather than guessing install-kind, writability, or a version
 * comparison.
 */

import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from '../../boundary/runtime';
import {
  fetchChannelIndex,
  fetchUpdateManifest,
  type FetchLike,
  type FetchResult,
} from './fetch';
import { decideEligibility, type EligibilityInput } from './eligibility';
import type { ReloadVerifyOutcome } from './installAction';
import {
  emptyInstalledState,
  emptyLastCheckState,
  emptyPendingRollbackState,
  type EligibilityResult,
  type InstalledState,
  type PendingRollbackState,
  type UpdateChannelLabel,
  type UpdateController,
  type UpdateControllerDeps,
} from './updateModel';

// ---------- Tauri invoke indirection ----------
//
// Mirrors `fetch.ts`'s `fetchImpl` injection tenet: every Tauri call goes
// through this seam so tests never touch a real Tauri runtime.

export interface InvokeLike {
  <T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}

async function callInvoke<T>(
  cmd: string,
  invokeImpl: InvokeLike | undefined,
  args?: Record<string, unknown>,
): Promise<T | null> {
  if (invokeImpl) {
    return invokeImpl<T>(cmd, args);
  }
  if (!hasTauriRuntime()) {
    return null;
  }
  return invoke<T>(cmd, args);
}

// ---------- fetch-result -> eligibility-input classification ----------

interface FetchClassification {
  status: EligibilityInput['fetchOutcomes']['indexStatus'];
  fetchError: string | null;
  schemaError: string | null;
}

/** Strips the schema-name prefix `fetch.ts` already added, so `decideEligibility`'s own prefix doesn't double up. */
function stripSchemaPrefix(reason: string): string {
  return reason.replace(/^(channel-index|update-manifest)\.schema\.json:\s*/, '');
}

function classifyFetchResult<T>(result: FetchResult<T>): FetchClassification {
  if (result.ok) {
    return { status: 'ok', fetchError: null, schemaError: null };
  }
  const failure = result.failure;
  switch (failure.kind) {
    case 'http-error':
      return {
        status: 'failed',
        fetchError: `HTTP ${failure.status} fetching ${failure.url}`,
        schemaError: null,
      };
    case 'unsupported-channel':
      return {
        status: 'failed',
        fetchError: `unsupported channel: ${failure.channel}`,
        schemaError: null,
      };
    case 'invalid-json':
      return {
        status: 'schema-invalid',
        fetchError: null,
        schemaError: `${failure.reason} at ${failure.url}`,
      };
    case 'invalid-channel-index':
    case 'invalid-manifest':
      return {
        status: 'schema-invalid',
        fetchError: null,
        schemaError: stripSchemaPrefix(failure.reason),
      };
  }
}

// ---------- runCheck + eligibility controller ----------

function emptyFetchOutcomes(): EligibilityInput['fetchOutcomes'] {
  return {
    indexStatus: 'failed',
    manifestStatus: 'failed',
    indexSchemaError: null,
    manifestSchemaError: null,
    indexFetchError: null,
    manifestFetchError: null,
  };
}

const LOCAL_STATE_UNAVAILABLE_REASON =
  'local installed-state is not available yet — is_install_eligible / perform_install remain deferred; real install-kind, writability, and version comparison cannot be verified';

/**
 * Build a real `UpdateControllerDeps`. `mountTimeState` supplies the
 * `installed`/`pendingRollback` fields (from `loadMountTimeState`); this
 * function owns `lastCheck`/`releaseNotes`/`controller`, whose methods
 * mutate `lastCheck`/`releaseNotes` on the returned object in place — the
 * contract `Ui.tsx` already expects (see its `handleCheck` comment).
 */
export function createUpdateControllerDeps(
  mountTimeState: Pick<MountTimeState, 'installed' | 'pendingRollback'>,
  defaultChannel: UpdateChannelLabel = 'alpha',
  options: { fetchImpl?: FetchLike } = {},
): UpdateControllerDeps {
  const deps: UpdateControllerDeps = {
    installed: mountTimeState.installed,
    lastCheck: emptyLastCheckState(defaultChannel),
    pendingRollback: mountTimeState.pendingRollback,
    releaseNotes: null,
    controller: undefined as unknown as UpdateController,
  };

  let hasRun = false;
  let checkedChannel: UpdateChannelLabel | null = null;
  let fetchOutcomes = emptyFetchOutcomes();

  function computeDecision(currentChannel: UpdateChannelLabel): {
    result: EligibilityResult;
    reason: string;
  } {
    if (!hasRun || checkedChannel !== currentChannel) {
      return { result: 'unknown', reason: 'check has not been run yet for this channel' };
    }
    if (fetchOutcomes.indexStatus === 'ok' && fetchOutcomes.manifestStatus === 'ok') {
      // The real check succeeded, but nothing in this slice can honestly
      // resolve local install-kind/writability/version — decideEligibility's
      // remaining rows all read installedState, which we do not have real
      // data for, so we must not call it with placeholder values here.
      return { result: 'unknown', reason: LOCAL_STATE_UNAVAILABLE_REASON };
    }
    // The fetch itself did not fully succeed — decideEligibility's first
    // four rows resolve this purely from fetchOutcomes, never touching
    // installedState/manifest, so passing placeholders below is safe.
    const decision = decideEligibility({
      selectedChannel: currentChannel,
      manifest: { version: '', artifact_sha256: '' },
      installedState: {
        version: '',
        artifact_sha256: '',
        install_kind: 'unknown',
        managed_executable_path: null,
        isManagedPathWritable: true,
      },
      fetchOutcomes,
    });
    return { result: decision.result, reason: decision.install_disabled_reason ?? 'ineligible' };
  }

  /**
   * `LastCheckPanel` reads `lastCheck.eligibilityResult`/`installDisabledReason`
   * directly (it does not call the controller), so `runCheck` must keep these
   * two fields in sync with the same decision `computeEligibility`/
   * `disabledReason` would return — otherwise the panel shows a stale
   * pre-check placeholder forever after a real check completes.
   */
  function syncLastCheckEligibility(channel: UpdateChannelLabel): void {
    const decision = computeDecision(channel);
    deps.lastCheck.eligibilityResult = decision.result;
    deps.lastCheck.installDisabledReason = decision.reason;
  }

  deps.controller = {
    async runCheck(channel: UpdateChannelLabel): Promise<void> {
      deps.lastCheck.selectedChannel = channel;
      deps.lastCheck.indexUrl = `https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels/${channel}.json`;
      deps.lastCheck.indexStatus = 'in-progress';
      deps.lastCheck.manifestStatus = 'not-loaded';
      deps.lastCheck.releaseVersion = null;
      deps.lastCheck.releaseNotesStatus = 'not-loaded';
      fetchOutcomes = emptyFetchOutcomes();

      const indexResult = await fetchChannelIndex(channel, { fetchImpl: options.fetchImpl });
      const indexClass = classifyFetchResult(indexResult);
      fetchOutcomes = {
        ...fetchOutcomes,
        indexStatus: indexClass.status,
        indexFetchError: indexClass.fetchError,
        indexSchemaError: indexClass.schemaError,
      };
      deps.lastCheck.indexStatus = indexClass.status === 'ok' ? 'ok' : 'failed';
      if (!indexResult.ok) {
        deps.lastCheck.manifestStatus = 'not-loaded';
        hasRun = true;
        checkedChannel = channel;
        syncLastCheckEligibility(channel);
        return;
      }

      const manifestResult = await fetchUpdateManifest(indexResult.value.manifest_url, {
        fetchImpl: options.fetchImpl,
      });
      const manifestClass = classifyFetchResult(manifestResult);
      fetchOutcomes = {
        ...fetchOutcomes,
        manifestStatus: manifestClass.status,
        manifestFetchError: manifestClass.fetchError,
        manifestSchemaError: manifestClass.schemaError,
      };
      deps.lastCheck.manifestStatus = manifestClass.status === 'ok' ? 'ok' : 'failed';

      if (manifestResult.ok) {
        deps.lastCheck.releaseVersion = manifestResult.value.version;
        // This slice fetches and validates the manifest but does not fetch
        // release-notes body content — that is a distinct, not-yet-built
        // fetch path. Reporting anything but 'unavailable' would imply
        // notes we never loaded.
        deps.lastCheck.releaseNotesStatus = 'unavailable';
      }

      hasRun = true;
      checkedChannel = channel;
      syncLastCheckEligibility(channel);
    },
    computeEligibility(_installed, lastCheck) {
      return computeDecision(lastCheck.selectedChannel).result;
    },
    disabledReason(_installed, lastCheck) {
      return computeDecision(lastCheck.selectedChannel).reason;
    },
    releaseNotes() {
      return deps.releaseNotes;
    },
  };

  return deps;
}

// ---------- mount-time state: verify_relaunch_artifact ----------

export interface RestoreOfferState {
  priorVersion: string;
  restoreAvailable: boolean;
}

export interface MountTimeState {
  installed: InstalledState;
  pendingRollback: PendingRollbackState;
  restoreOffer: RestoreOfferState | null;
}

/**
 * Load the real mount-time state via `verify_relaunch_artifact` (already
 * real, already tested Rust command). This is the one point in the app
 * where a pending relaunch verification is checked and, on success,
 * promoted into `installed-state.json` server-side.
 */
export async function loadMountTimeState(
  options: { invokeImpl?: InvokeLike } = {},
): Promise<MountTimeState> {
  let outcome: ReloadVerifyOutcome;
  try {
    outcome =
      (await callInvoke<ReloadVerifyOutcome>('verify_relaunch_artifact', options.invokeImpl)) ??
      { kind: 'no-pending-update' };
  } catch (cause) {
    throw new Error(`verify_relaunch_artifact failed: ${formatError(cause)}`);
  }

  switch (outcome.kind) {
    case 'no-pending-update':
      return {
        installed: emptyInstalledState(),
        pendingRollback: emptyPendingRollbackState(),
        restoreOffer: null,
      };
    case 'verification-failed':
      return {
        installed: emptyInstalledState(),
        pendingRollback: {
          pendingUpdateState: 'pending-relaunch',
          previousVersionAvailable: true,
          rollbackState: 'available',
          backupCount: 0,
          retainedUpdateStorageBytes: 0,
        },
        // The exact prior version lives in pending-update.json, which no
        // command exposes to the frontend in this slice — degrade honestly
        // to 'unknown' rather than guess. `restoreAvailable` is still a real
        // fact: `perform_restore_previous` genuinely works from here.
        restoreOffer: { priorVersion: 'unknown', restoreAvailable: true },
      };
    case 'promoted':
      return {
        installed: {
          ...emptyInstalledState(),
          version: outcome.promotedVersion,
          // Safe, code-grounded inference, not a guess: the only Rust path
          // that ever promotes (`verify_relaunch_artifact_impl`) always
          // writes `InstallKind::AppImage`.
          installKind: 'appimage',
          updateEligible: false,
          ineligibleReason:
            'local eligibility probe not available yet (is_install_eligible deferred)',
        },
        pendingRollback: emptyPendingRollbackState(),
        restoreOffer: null,
      };
  }
}

// ---------- restore action: perform_restore_previous ----------

export type RollbackOutcome =
  | { kind: 'promoted'; restoredVersion: string }
  | { kind: 'auto-restored'; restoredVersion: string }
  | { kind: 'rollback-failed'; reason: string }
  | { kind: 'no-backup'; reason: string }
  | { kind: 'no-pending' };

interface RustRollbackOutcome {
  kind: 'promoted' | 'auto-restored' | 'rollback-failed' | 'no-backup' | 'no-pending';
  restoredVersion?: string;
  reason?: string;
}

/** Call the real, already-tested `perform_restore_previous` Tauri command. */
export async function restorePreviousVersion(
  options: { invokeImpl?: InvokeLike } = {},
): Promise<RollbackOutcome> {
  let raw: RustRollbackOutcome;
  try {
    raw =
      (await callInvoke<RustRollbackOutcome>('perform_restore_previous', options.invokeImpl)) ?? {
        kind: 'no-pending',
      };
  } catch (cause) {
    throw new Error(`perform_restore_previous failed: ${formatError(cause)}`);
  }
  switch (raw.kind) {
    case 'promoted':
    case 'auto-restored':
      return { kind: raw.kind, restoredVersion: raw.restoredVersion ?? 'unknown' };
    case 'rollback-failed':
      return { kind: 'rollback-failed', reason: raw.reason ?? 'unknown failure' };
    case 'no-backup':
      return { kind: 'no-backup', reason: raw.reason ?? 'no backup available' };
    case 'no-pending':
      return { kind: 'no-pending' };
  }
}

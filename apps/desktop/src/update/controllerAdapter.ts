/**
 * Character Hub Phase 3 — real `UpdateController` adapter.
 *
 * Bridges the update UI to the parts of the update system that are
 * genuinely real today: `fetch.ts`'s discovery fetch/validate,
 * `eligibility.ts`'s pure decision table, and the Tauri commands that have
 * real, tested bodies (`verify_relaunch_artifact`, `perform_restore_previous`,
 * and — as of E3.13/E3.14 — `is_install_eligible`).
 *
 * `is_install_eligible` reports real local install-state facts (install
 * kind, version, artifact hash, managed-path writability) without itself
 * rendering a verdict; this module calls it during `runCheck` and feeds the
 * result into `decideEligibility`, so a successful fetch check now resolves
 * to a genuine `eligible`/`ineligible` verdict whenever a local
 * installed-state record exists, instead of the permanent `'unknown'` this
 * module used to return. When no local record exists yet (first run, or a
 * run that never completed `verify_relaunch_artifact`), or the probe itself
 * fails, eligibility still degrades honestly to `'unknown'` with a reason
 * naming exactly why — never a fabricated verdict. `perform_install` remains
 * an honest deferred stub (it needs an HTTP client this crate does not
 * carry yet), so this module never calls it.
 */

import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from '../boundary/runtime';
import {
  fetchChannelIndex,
  fetchReleaseNotesBody,
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
  type InstallResult,
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

// ---------- is_install_eligible probe wire shape + mapping ----------

/**
 * Wire shape of `is_install_eligible`'s `Ok` response — mirrors Rust's
 * `InstallEligibilityProbe` (camelCase per the module-wide wire convention).
 * `installed` is `null` when there is no `installed-state.json` on disk yet;
 * `decideEligibility` must never be called with a fabricated version/hash
 * in that case.
 */
interface RustInstalledStateWire {
  managedExecutablePath: string;
  installKind: 'app-image' | 'deb' | 'dev-local';
  channel: string;
  version: string;
  sourceCommit: string;
  releaseTag: string;
  manifestHash: string;
  artifactSha256: string;
  installedAt: string;
  updateEligible: boolean;
  ineligibleReason: string | null;
}

interface PerformInstallResponseWire {
  fromVersion: string;
  toVersion: string;
}

interface RustInstallEligibilityProbe {
  installed: RustInstalledStateWire | null;
  isManagedPathWritable: boolean;
}

/**
 * Maps the Rust `InstallKind` wire enum (kebab-case) onto
 * `decideEligibility`'s `install_kind` union. `deb` self-updates through the package
 * manager and has its own eligibility rows.
 */
function mapRustInstallKind(
  kind: RustInstalledStateWire['installKind'],
): EligibilityInput['installedState']['install_kind'] {
  switch (kind) {
    case 'app-image':
      return 'appimage';
    case 'dev-local':
      return 'dev';
    case 'deb':
      return 'deb';
  }
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

const NO_LOCAL_PROBE_YET_REASON =
  'local install-state probe has not run yet for this check';
const NO_LOCAL_RECORD_REASON =
  'no local installed-state record yet — is_install_eligible has nothing to compare the fetched manifest against';

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
  options: { fetchImpl?: FetchLike; invokeImpl?: InvokeLike } = {},
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
  let lastManifest: {
    version: string;
    artifactSha256: string;
    debArtifactSha256: string | null;
  } | null = null;
  // The validated manifest exactly as fetched; the backend re-reads the artifact block from it.
  let lastManifestRaw: unknown = null;
  let localProbe: RustInstallEligibilityProbe | null = null;
  let localProbeError: string | null = null;

  function computeDecision(currentChannel: UpdateChannelLabel): {
    result: EligibilityResult;
    reason: string;
  } {
    if (!hasRun || checkedChannel !== currentChannel) {
      return { result: 'unknown', reason: 'check has not been run yet for this channel' };
    }
    if (fetchOutcomes.indexStatus === 'ok' && fetchOutcomes.manifestStatus === 'ok') {
      // E3.14: the real check succeeded — resolve the real verdict via
      // decideEligibility, fed by the real is_install_eligible probe result
      // `runCheck` already fetched. Every branch below degrades honestly to
      // 'unknown' rather than fabricating eligible/ineligible when a piece
      // of real data is missing.
      if (localProbeError !== null) {
        return { result: 'unknown', reason: `local install-state probe failed: ${localProbeError}` };
      }
      if (localProbe === null) {
        return { result: 'unknown', reason: NO_LOCAL_PROBE_YET_REASON };
      }
      if (localProbe.installed === null) {
        return { result: 'unknown', reason: NO_LOCAL_RECORD_REASON };
      }
      if (!lastManifest) {
        return { result: 'unknown', reason: NO_LOCAL_PROBE_YET_REASON };
      }
      const installKind = mapRustInstallKind(localProbe.installed.installKind);
      const decision = decideEligibility({
        selectedChannel: currentChannel,
        manifest: {
          version: lastManifest.version,
          artifact_sha256:
            installKind === 'deb' ? lastManifest.debArtifactSha256 : lastManifest.artifactSha256,
        },
        installedState: {
          version: localProbe.installed.version,
          artifact_sha256: localProbe.installed.artifactSha256,
          install_kind: installKind,
          managed_executable_path: localProbe.installed.managedExecutablePath,
          isManagedPathWritable: localProbe.isManagedPathWritable,
        },
        fetchOutcomes,
      });
      return { result: decision.result, reason: decision.install_disabled_reason ?? 'ineligible' };
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
      lastManifest = null;
      lastManifestRaw = null;
      localProbe = null;
      localProbeError = null;

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
        lastManifestRaw = manifestResult.value;
        lastManifest = {
          version: manifestResult.value.version,
          artifactSha256: manifestResult.value.linux_appimage.sha256,
          debArtifactSha256: manifestResult.value.linux_deb?.sha256 ?? null,
        };
        // E3.12: the manifest names a release-notes body (`release_notes_url`
        // + `release_notes_hash`) but does not carry the prose itself — fetch
        // and hash-verify it now. A fetch/hash failure here must never
        // fabricate notes; `releaseNotesStatus` stays 'unavailable' and
        // `deps.releaseNotes` stays null, same as before this slice existed.
        const notesResult = await fetchReleaseNotesBody(
          manifestResult.value.release_notes_url,
          manifestResult.value.release_notes_hash,
          { fetchImpl: options.fetchImpl },
        );
        if (notesResult.ok) {
          deps.releaseNotes = {
            releaseVersion: manifestResult.value.version,
            body: notesResult.value.body,
          };
          deps.lastCheck.releaseNotesStatus = 'loaded';
        } else {
          deps.releaseNotes = null;
          deps.lastCheck.releaseNotesStatus = 'unavailable';
        }

        // E3.14: real local install-state probe. A failure here must never
        // fabricate eligibility — `computeDecision` degrades honestly to
        // 'unknown' whenever `localProbeError` is set or `localProbe` stays
        // null.
        try {
          localProbe = await callInvoke<RustInstallEligibilityProbe>(
            'is_install_eligible',
            options.invokeImpl,
          );
        } catch (cause) {
          localProbe = null;
          localProbeError = formatError(cause);
        }
      } else {
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
    async install(): Promise<InstallResult> {
      if (lastManifestRaw === null || checkedChannel === null) {
        throw new Error('nothing to install: run Check first');
      }
      const decision = computeDecision(checkedChannel);
      if (decision.result !== 'eligible') {
        throw new Error(`install is not available: ${decision.reason}`);
      }
      let response: PerformInstallResponseWire | null;
      try {
        response = await callInvoke<PerformInstallResponseWire>('perform_install', options.invokeImpl, {
          manifest: lastManifestRaw,
          indexUrl: deps.lastCheck.indexUrl,
        });
      } catch (cause) {
        // Tauri rejects with the Rust `Err(String)`; keep that text, it is the actual reason.
        throw new Error(formatError(cause));
      }
      if (response === null) {
        throw new Error('install is only available in the desktop app');
      }
      return { fromVersion: response.fromVersion, toVersion: response.toVersion };
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

  // The Installed panel describes what is installed, so it comes from the real
  // `installed-state.json` (via the same probe Check uses) whatever the verifier outcome was.
  const recorded = await readRecordedInstall(options.invokeImpl);

  switch (outcome.kind) {
    case 'no-pending-update':
      return {
        installed: recorded.installed ?? unknownInstalled(recorded.error),
        pendingRollback: emptyPendingRollbackState(),
        restoreOffer: null,
      };
    case 'verification-failed':
      return {
        installed: recorded.installed ?? unknownInstalled(recorded.error),
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
        // The verifier just wrote the record; prefer it. If it cannot be read back, fall back to
        // the one fact the outcome carries (the version) and the one kind that path writes.
        installed: recorded.installed ?? {
          ...emptyInstalledState(),
          version: outcome.promotedVersion,
          installKind: 'appimage',
          updateEligible: false,
          ineligibleReason:
            recorded.error ??
            'eligibility for a freshly-promoted install is determined by the next Check, not this mount-time snapshot',
        },
        pendingRollback: emptyPendingRollbackState(),
        restoreOffer: null,
      };
  }
}

function unknownInstalled(probeError: string | null): InstalledState {
  const base = emptyInstalledState();
  return probeError === null ? base : { ...base, ineligibleReason: probeError };
}

/** Read `installed-state.json` through `is_install_eligible`, mapping it onto the panel model. */
async function readRecordedInstall(
  invokeImpl: InvokeLike | undefined,
): Promise<{ installed: InstalledState | null; error: string | null }> {
  let probe: RustInstallEligibilityProbe | null;
  try {
    probe = await callInvoke<RustInstallEligibilityProbe>('is_install_eligible', invokeImpl);
  } catch (cause) {
    return { installed: null, error: `local install-state probe failed: ${formatError(cause)}` };
  }
  const record = probe?.installed;
  if (!record) {
    return { installed: null, error: null };
  }
  return {
    installed: {
      channel: record.channel as InstalledState['channel'],
      version: record.version,
      sourceCommit: record.sourceCommit,
      artifactSha256: record.artifactSha256,
      installKind: mapRustInstallKind(record.installKind),
      managedExecutablePath: record.managedExecutablePath,
      updateEligible: record.updateEligible,
      ineligibleReason: record.ineligibleReason ?? null,
    },
    error: null,
  };
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

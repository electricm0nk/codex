import { assert, assertEqual } from '../../testSupport/asserts';
import type { FetchLike } from './fetch';
import {
  createUpdateControllerDeps,
  loadMountTimeState,
  restorePreviousVersion,
  type InvokeLike,
} from './controllerAdapter';

// ---------- fixtures ----------

const MANIFEST_URL =
  'https://raw.githubusercontent.com/electricm0nk/codex/update-index/manifests/alpha/v0.1.0-abc12345/update-manifest.json';

const SHA40 = 'abcdef0123456789abcdef0123456789abcdef01';
const SHA64 = 'abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789';

const VALID_CHANNEL_WIRE = {
  schema_version: '1.0.0',
  channel: 'alpha' as const,
  version: '0.1.0',
  tag: 'alpha/v0.1.0-abc12345',
  release_url: 'https://github.com/electricm0nk/codex/releases/tag/alpha-v0.1.0-abc12345',
  manifest_url: MANIFEST_URL,
  publication_timestamp: '2026-07-03T12:00:00Z',
  tranche_id: 'STC-CODEX-SD-16',
  signature: null,
};

const VALID_MANIFEST_WIRE = {
  schema_version: '1.0.0',
  channel: 'alpha' as const,
  version: '0.1.0',
  tag: 'alpha/v0.1.0-abc12345',
  tranche_id: 'STC-CODEX-SD-16',
  source_branch: 'develop',
  source_commit: SHA40,
  release_notes_path:
    'programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/release-notes.md',
  release_notes_url: 'https://github.com/electricm0nk/codex/releases/tag/alpha-v0.1.0-abc12345',
  release_notes_hash: SHA64,
  linux_appimage: {
    name: 'Codex.Desktop.Shell.Scaffold_0.1.0_amd64.AppImage',
    url: 'https://github.com/electricm0nk/codex/releases/download/alpha-v0.1.0-abc12345/Codex.Desktop.Shell.Scaffold_0.1.0_amd64.AppImage',
    sha256: SHA64,
    size_bytes: 77662712,
  },
  workflow_provenance: {
    workflow: '.github/workflows/publish-tester-release.yml',
    run_id: 28808170752,
    run_attempt: 1,
  },
  eligibility: {
    min_supported_version: '0.0.0',
    appimage_install: true,
    required_install_kind: 'appimage' as const,
  },
  promotion_lineage: {
    source_branch: 'develop',
    source_commit: SHA40,
    promoted_at: '2026-07-03T12:00:00Z',
  },
  signature: null,
};

function channelText(overrides: Record<string, unknown> = {}): string {
  return JSON.stringify({ ...VALID_CHANNEL_WIRE, ...overrides });
}

function manifestText(overrides: Record<string, unknown> = {}): string {
  return JSON.stringify({ ...VALID_MANIFEST_WIRE, ...overrides });
}

const CHANNEL_INDEX_URL =
  'https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels/alpha.json';

interface Stub {
  url: string;
  status: number;
  responded: string;
}

function makeFetchImpl(stubs: Stub[]): FetchLike {
  const byUrl = new Map(stubs.map((s) => [s.url, s]));
  return async (input) => {
    const url = typeof input === 'string' ? input : String(input);
    const stub = byUrl.get(url);
    if (!stub) {
      throw new Error(`no stub registered for ${url}`);
    }
    return {
      ok: stub.status >= 200 && stub.status < 300,
      status: stub.status,
      text: async () => stub.responded,
    };
  };
}

function emptyMountTimeState() {
  return loadMountTimeState({
    invokeImpl: (async () => ({ kind: 'no-pending-update' })) as InvokeLike,
  });
}

// ---------- runCheck: success path ----------

async function verifiesRunCheckOnSuccessPopulatesLastCheckHonestly() {
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText() },
      { url: MANIFEST_URL, status: 200, responded: manifestText() },
    ]),
  });

  await deps.controller.runCheck('alpha');

  assertEqual(deps.lastCheck.indexStatus, 'ok', 'index status after successful check');
  assertEqual(deps.lastCheck.manifestStatus, 'ok', 'manifest status after successful check');
  assertEqual(deps.lastCheck.releaseVersion, '0.1.0', 'release version captured from manifest');

  // Real fetch succeeded, but no code path in this slice yields trustworthy
  // local installed-state (is_install_eligible / perform_install remain
  // deferred) — eligibility must stay honestly 'unknown', never fabricated.
  const eligibility = deps.controller.computeEligibility(deps.installed, deps.lastCheck);
  assertEqual(eligibility, 'unknown', 'eligibility stays unknown without a real local-state source');
  const reason = deps.controller.disabledReason(deps.installed, deps.lastCheck);
  assert(
    typeof reason === 'string' && reason.length > 0,
    'disabledReason must be a real, non-empty explanation',
  );
  assert(
    !/not wired \(F3a\/F3b pending\)/.test(reason ?? ''),
    'disabledReason must not be the old canned unwired-controller string',
  );

  // Regression: LastCheckPanel reads lastCheck.eligibilityResult /
  // installDisabledReason directly rather than calling the controller — these
  // must be kept in sync by runCheck itself, or the panel shows a stale
  // pre-check placeholder forever after a real, completed check.
  assertEqual(
    deps.lastCheck.eligibilityResult,
    eligibility,
    'lastCheck.eligibilityResult must match what the controller itself computes',
  );
  assertEqual(
    deps.lastCheck.installDisabledReason,
    reason,
    'lastCheck.installDisabledReason must match what the controller itself computes',
  );
}

// ---------- runCheck: index fetch network failure ----------

async function verifiesRunCheckSurfacesRealIndexFetchFailure() {
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([{ url: CHANNEL_INDEX_URL, status: 404, responded: 'not found' }]),
  });

  await deps.controller.runCheck('alpha');

  assertEqual(deps.lastCheck.indexStatus, 'failed', 'index status after 404');
  assertEqual(deps.lastCheck.manifestStatus, 'not-loaded', 'manifest never attempted after index failure');

  const decision = deps.controller.disabledReason(deps.installed, deps.lastCheck);
  assert(
    (decision ?? '').includes('channel index fetch failed'),
    `disabledReason must name the real index-fetch failure, got: ${decision}`,
  );
  assertEqual(
    deps.controller.computeEligibility(deps.installed, deps.lastCheck),
    'unknown',
    'eligibility unknown on fetch failure',
  );
}

// ---------- runCheck: index schema-invalid ----------

async function verifiesRunCheckSurfacesRealSchemaInvalidReason() {
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText({ schema_version: 'v9' }) },
    ]),
  });

  await deps.controller.runCheck('alpha');

  assertEqual(deps.lastCheck.indexStatus, 'failed', 'schema-invalid folds into failed at the UI level');
  const reason = deps.controller.disabledReason(deps.installed, deps.lastCheck);
  assert(
    (reason ?? '').includes('channel-index.schema.json'),
    `disabledReason must name the real schema failure, got: ${reason}`,
  );
}

// ---------- runCheck: manifest fetch failure after index ok ----------

async function verifiesRunCheckSurfacesRealManifestFetchFailure() {
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText() },
      { url: MANIFEST_URL, status: 500, responded: 'server error' },
    ]),
  });

  await deps.controller.runCheck('alpha');

  assertEqual(deps.lastCheck.indexStatus, 'ok', 'index ok');
  assertEqual(deps.lastCheck.manifestStatus, 'failed', 'manifest failed');
  const reason = deps.controller.disabledReason(deps.installed, deps.lastCheck);
  assert(
    (reason ?? '').includes('manifest fetch failed'),
    `disabledReason must name the real manifest-fetch failure, got: ${reason}`,
  );
}

// ---------- before any check has run ----------

async function verifiesEligibilityUnknownBeforeAnyCheck() {
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha');
  assertEqual(
    deps.controller.computeEligibility(deps.installed, deps.lastCheck),
    'unknown',
    'eligibility unknown before any check has run',
  );
  const reason = deps.controller.disabledReason(deps.installed, deps.lastCheck);
  assert(typeof reason === 'string' && reason.length > 0, 'a real reason must be present pre-check');
}

// ---------- mount-time verify_relaunch_artifact mapping ----------

async function verifiesNoPendingUpdateMapsToEmptyState() {
  const state = await loadMountTimeState({
    invokeImpl: (async () => ({ kind: 'no-pending-update' })) as InvokeLike,
  });
  assertEqual(state.installed.version, 'unknown', 'no installed record yet');
  assertEqual(state.restoreOffer, null, 'no restore offer when nothing is pending');
  assertEqual(state.pendingRollback.rollbackState, 'unknown', 'rollback state unknown with no pending record');
}

async function verifiesVerificationFailedOffersRestoreHonestly() {
  const state = await loadMountTimeState({
    invokeImpl: (async () => ({
      kind: 'verification-failed',
      expected: SHA64,
      actual: 'deadbeef',
    })) as InvokeLike,
  });
  assert(state.restoreOffer !== null, 'a restore offer must be surfaced on verification failure');
  assertEqual(state.restoreOffer?.restoreAvailable, true, 'restore is genuinely available via perform_restore_previous');
  // The exact prior version is not known without a dedicated installed-state
  // reader (out of scope this slice) — must degrade honestly, not fabricate.
  assertEqual(state.restoreOffer?.priorVersion, 'unknown', 'prior version honestly unknown without a real reader');
  assertEqual(state.pendingRollback.pendingUpdateState, 'pending-relaunch', 'pending state reflects the mismatch');
  assertEqual(state.pendingRollback.rollbackState, 'available', 'rollback state reflects real availability');
}

async function verifiesPromotedMapsKnownFieldsOnlyWithoutFabricating() {
  const state = await loadMountTimeState({
    invokeImpl: (async () => ({
      kind: 'promoted',
      installedStatePath: '/home/ubuntu/.config/codex/update/installed-state.json',
      promotedVersion: '0.1.0',
    })) as InvokeLike,
  });
  assertEqual(state.installed.version, '0.1.0', 'promoted version is real, from the outcome');
  // installKind='appimage' is a safe, code-grounded inference: the Rust
  // promotion path (verify_relaunch_artifact_impl) only ever writes
  // InstallKind::AppImage — never a guess.
  assertEqual(state.installed.installKind, 'appimage', 'install kind inferred from the only promotion path that exists');
  assertEqual(state.installed.artifactSha256, null, 'artifact hash honestly unknown — not returned by this outcome');
  assertEqual(state.restoreOffer, null, 'no restore offer once promoted cleanly');
}

// ---------- restorePreviousVersion ----------

async function verifiesRestorePreviousVersionCallsRealCommand() {
  let calledCmd: string | null = null;
  const outcome = await restorePreviousVersion({
    invokeImpl: (async (cmd: string) => {
      calledCmd = cmd;
      return {
        kind: 'promoted',
        restoredFrom: '/home/ubuntu/.config/codex/update/backups/prev.AppImage',
        managedExecutablePath: '/opt/codex/app.AppImage',
        restoredVersion: '0.0.9',
        restoredArtifactSha256: SHA64,
      };
    }) as InvokeLike,
  });
  assertEqual(calledCmd, 'perform_restore_previous', 'must call the real Rust command');
  assertEqual(outcome.kind, 'promoted', 'outcome kind mapped from Rust response');
  if (outcome.kind === 'promoted') {
    assertEqual(outcome.restoredVersion, '0.0.9', 'restored version passed through');
  }
}

async function verifiesRestorePreviousVersionSurfacesFailureHonestly() {
  const outcome = await restorePreviousVersion({
    invokeImpl: (async () => ({
      kind: 'rollback-failed',
      reason: 'backup checksum mismatch',
      pendingUpdatePath: '/home/ubuntu/.config/codex/update/pending-update.json',
    })) as InvokeLike,
  });
  assertEqual(outcome.kind, 'rollback-failed', 'failure surfaced honestly, not fabricated as success');
  if (outcome.kind === 'rollback-failed') {
    assertEqual(outcome.reason, 'backup checksum mismatch', 'real failure reason passed through');
  }
}

async function main() {
  await verifiesRunCheckOnSuccessPopulatesLastCheckHonestly();
  await verifiesRunCheckSurfacesRealIndexFetchFailure();
  await verifiesRunCheckSurfacesRealSchemaInvalidReason();
  await verifiesRunCheckSurfacesRealManifestFetchFailure();
  await verifiesEligibilityUnknownBeforeAnyCheck();
  await verifiesNoPendingUpdateMapsToEmptyState();
  await verifiesVerificationFailedOffersRestoreHonestly();
  await verifiesPromotedMapsKnownFieldsOnlyWithoutFabricating();
  await verifiesRestorePreviousVersionCallsRealCommand();
  await verifiesRestorePreviousVersionSurfacesFailureHonestly();
  console.log('controllerAdapter.test.ts: all assertions passed');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

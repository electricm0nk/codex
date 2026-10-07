import { assert, assertEqual } from '../testSupport/asserts';
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
    'docs/release/SD-16/release-notes.md',
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

async function sha256Hex(text: string): Promise<string> {
  const digest = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(text));
  return Array.from(new Uint8Array(digest))
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('');
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

// ---------- runCheck: release-notes-body fetch (E3.12) ----------

async function verifiesRunCheckFetchesAndVerifiesReleaseNotesOnSuccess() {
  const body = '## v0.1.0\n\n- Real, verified release notes.\n';
  const hash = await sha256Hex(body);
  const notesUrl =
    'https://raw.githubusercontent.com/electricm0nk/codex/update-index/manifests/alpha/v0.1.0-abc12345/release-notes.md';
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText() },
      {
        url: MANIFEST_URL,
        status: 200,
        responded: manifestText({ release_notes_url: notesUrl, release_notes_hash: hash }),
      },
      { url: notesUrl, status: 200, responded: body },
    ]),
  });

  await deps.controller.runCheck('alpha');

  assertEqual(
    deps.lastCheck.releaseNotesStatus,
    'loaded',
    'releaseNotesStatus reflects a real, hash-verified fetch',
  );
  assert(deps.releaseNotes !== null, 'deps.releaseNotes must be populated after a successful check');
  assertEqual(deps.releaseNotes?.body, body, 'release notes body preserved verbatim');
  assertEqual(deps.releaseNotes?.releaseVersion, '0.1.0', 'release notes tagged with the manifest version');
  assertEqual(
    deps.controller.releaseNotes()?.body,
    body,
    'controller.releaseNotes() must mirror deps.releaseNotes',
  );
}

async function verifiesRunCheckLeavesReleaseNotesUnavailableOnHashMismatch() {
  const notesUrl =
    'https://raw.githubusercontent.com/electricm0nk/codex/update-index/manifests/alpha/v0.1.0-abc12345/release-notes.md';
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText() },
      {
        url: MANIFEST_URL,
        status: 200,
        responded: manifestText({ release_notes_url: notesUrl }), // uses SHA64 fixture hash
      },
      { url: notesUrl, status: 200, responded: 'body that does not match the pinned hash' },
    ]),
  });

  await deps.controller.runCheck('alpha');

  assertEqual(
    deps.lastCheck.releaseNotesStatus,
    'unavailable',
    'a hash-mismatched body must never be surfaced as loaded',
  );
  assertEqual(deps.releaseNotes, null, 'deps.releaseNotes stays null on hash mismatch');
}

// ---------- computeDecision: real decideEligibility rewiring (E3.14) ----------

async function verifiesEligibleWhenRealLocalProbeIsOlderThanFetchedManifest() {
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText() },
      {
        url: MANIFEST_URL,
        status: 200,
        responded: manifestText({
          version: '0.2.0',
          linux_appimage: { ...VALID_MANIFEST_WIRE.linux_appimage, sha256: 'a'.repeat(64) },
        }),
      },
    ]),
    invokeImpl: (async (cmd: string) => {
      if (cmd === 'is_install_eligible') {
        return {
          installed: {
            managedExecutablePath: '/opt/codex/codex.AppImage',
            installKind: 'app-image',
            channel: 'alpha',
            version: '0.1.0',
            sourceCommit: 'deadbeef',
            releaseTag: 'alpha/v0.1.0',
            manifestHash: 'manifest-hash',
            artifactSha256: 'b'.repeat(64),
            installedAt: '2026-07-03T00:00:00Z',
            updateEligible: true,
            ineligibleReason: null,
          },
          isManagedPathWritable: true,
        };
      }
      throw new Error(`unexpected invoke ${cmd}`);
    }) as InvokeLike,
  });

  await deps.controller.runCheck('alpha');

  const eligibility = deps.controller.computeEligibility(deps.installed, deps.lastCheck);
  assertEqual(
    eligibility,
    'eligible',
    'a real, older local AppImage install plus a real, newer fetched manifest must resolve eligible via decideEligibility',
  );
  assertEqual(
    deps.lastCheck.eligibilityResult,
    'eligible',
    'lastCheck.eligibilityResult must mirror the controller-computed verdict',
  );
}

async function verifiesIneligibleWhenRealLocalProbeReportsDevLocalInstall() {
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText() },
      { url: MANIFEST_URL, status: 200, responded: manifestText({ version: '0.2.0' }) },
    ]),
    invokeImpl: (async (cmd: string) => {
      if (cmd === 'is_install_eligible') {
        return {
          installed: {
            managedExecutablePath: '/home/dev/codex/target/debug/codex-desktop',
            installKind: 'dev-local',
            channel: 'alpha',
            version: '0.1.0',
            sourceCommit: 'deadbeef',
            releaseTag: 'alpha/v0.1.0',
            manifestHash: 'manifest-hash',
            artifactSha256: SHA64,
            installedAt: '2026-07-03T00:00:00Z',
            updateEligible: false,
            ineligibleReason: 'dev-local build is not update-eligible',
          },
          isManagedPathWritable: true,
        };
      }
      throw new Error(`unexpected invoke ${cmd}`);
    }) as InvokeLike,
  });

  await deps.controller.runCheck('alpha');

  const eligibility = deps.controller.computeEligibility(deps.installed, deps.lastCheck);
  assertEqual(eligibility, 'ineligible', 'a real dev-local local install must resolve ineligible via decideEligibility');
  const reason = deps.controller.disabledReason(deps.installed, deps.lastCheck);
  assert((reason ?? '').length > 0, 'a real, non-empty ineligible reason must be surfaced');
}

async function verifiesUnknownHonestlyWhenNoLocalInstalledStateRecordExists() {
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText() },
      { url: MANIFEST_URL, status: 200, responded: manifestText() },
    ]),
    invokeImpl: (async (cmd: string) => {
      if (cmd === 'is_install_eligible') {
        return { installed: null, isManagedPathWritable: false };
      }
      throw new Error(`unexpected invoke ${cmd}`);
    }) as InvokeLike,
  });

  await deps.controller.runCheck('alpha');

  const eligibility = deps.controller.computeEligibility(deps.installed, deps.lastCheck);
  assertEqual(eligibility, 'unknown', 'no local installed-state record must stay honestly unknown, never fabricated');
  const reason = deps.controller.disabledReason(deps.installed, deps.lastCheck);
  assert(
    (reason ?? '').includes('installed-state'),
    `reason must honestly name the missing local record, got: ${reason}`,
  );
}

async function verifiesUnknownHonestlyWhenLocalProbeInvokeFails() {
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText() },
      { url: MANIFEST_URL, status: 200, responded: manifestText() },
    ]),
    invokeImpl: (async (cmd: string) => {
      if (cmd === 'is_install_eligible') {
        throw new Error('installed-state.json is unreadable: corrupt JSON');
      }
      throw new Error(`unexpected invoke ${cmd}`);
    }) as InvokeLike,
  });

  await deps.controller.runCheck('alpha');

  const eligibility = deps.controller.computeEligibility(deps.installed, deps.lastCheck);
  assertEqual(eligibility, 'unknown', 'a failed local probe must stay honestly unknown, never fabricated');
  const reason = deps.controller.disabledReason(deps.installed, deps.lastCheck);
  assert(
    (reason ?? '').includes('corrupt JSON'),
    `reason must surface the real probe failure verbatim, got: ${reason}`,
  );
}

// ---------- E3.15: additional per-probe/per-decision outcome coverage ----------

async function verifiesIneligibleWhenRealLocalProbeReportsUnwritableManagedPath() {
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText() },
      { url: MANIFEST_URL, status: 200, responded: manifestText({ version: '0.2.0' }) },
    ]),
    invokeImpl: (async (cmd: string) => {
      if (cmd === 'is_install_eligible') {
        return {
          installed: {
            managedExecutablePath: '/opt/codex/codex.AppImage',
            installKind: 'app-image',
            channel: 'alpha',
            version: '0.1.0',
            sourceCommit: 'deadbeef',
            releaseTag: 'alpha/v0.1.0',
            manifestHash: 'manifest-hash',
            artifactSha256: 'b'.repeat(64),
            installedAt: '2026-07-03T00:00:00Z',
            updateEligible: false,
            ineligibleReason: null,
          },
          isManagedPathWritable: false,
        };
      }
      throw new Error(`unexpected invoke ${cmd}`);
    }) as InvokeLike,
  });

  await deps.controller.runCheck('alpha');

  const eligibility = deps.controller.computeEligibility(deps.installed, deps.lastCheck);
  assertEqual(eligibility, 'ineligible', 'an unwritable managed path must resolve ineligible via decideEligibility');
  const reason = deps.controller.disabledReason(deps.installed, deps.lastCheck);
  assert((reason ?? '').includes('not writable'), `reason must name the writability gate, got: ${reason}`);
}

const DEB_PROBE = {
  installed: {
    managedExecutablePath: '/usr/bin/codex-desktop',
    installKind: 'deb',
    channel: 'alpha',
    version: '0.1.0',
    sourceCommit: 'deadbeef',
    releaseTag: 'alpha/v0.1.0',
    manifestHash: '',
    artifactSha256: SHA64,
    installedAt: '2026-07-03T00:00:00Z',
    updateEligible: true,
    ineligibleReason: null,
  },
  // /usr/bin is root-owned: the deb path installs through pkexec, so this must not gate it.
  isManagedPathWritable: false,
};

const LINUX_DEB_BLOCK = {
  name: 'Codex_0.2.0_amd64.deb',
  url: 'https://github.com/electricm0nk/codex/releases/download/alpha-v0.2.0/Codex_0.2.0_amd64.deb',
  sha256: 'd'.repeat(64),
  size_bytes: 10,
};

async function checkDebInstallAgainst(manifestOverrides: Record<string, unknown>) {
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText() },
      { url: MANIFEST_URL, status: 200, responded: manifestText({ version: '0.2.0', ...manifestOverrides }) },
    ]),
    invokeImpl: (async (cmd: string) => {
      if (cmd === 'is_install_eligible') return DEB_PROBE;
      throw new Error(`unexpected invoke ${cmd}`);
    }) as InvokeLike,
  });
  await deps.controller.runCheck('alpha');
  return deps;
}

async function verifiesEligibleWhenDebInstallSeesNewerManifestWithDebArtifact() {
  const deps = await checkDebInstallAgainst({ schema_version: '1.2.0', linux_deb: LINUX_DEB_BLOCK });
  assertEqual(
    deps.controller.computeEligibility(deps.installed, deps.lastCheck),
    'eligible',
    'a deb install with a newer manifest that carries linux_deb must be eligible, regardless of path writability',
  );
}

async function verifiesIneligibleWhenDebInstallSeesManifestWithoutDebArtifact() {
  const deps = await checkDebInstallAgainst({});
  assertEqual(
    deps.controller.computeEligibility(deps.installed, deps.lastCheck),
    'ineligible',
    'a deb install must not be offered an AppImage-only release',
  );
  const reason = deps.controller.disabledReason(deps.installed, deps.lastCheck);
  assert((reason ?? '').includes('.deb'), `reason must name the missing .deb artifact, got: ${reason}`);
}

async function verifiesIneligibleWhenFetchedManifestMatchesAlreadyInstalledVersion() {
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText() },
      {
        url: MANIFEST_URL,
        status: 200,
        responded: manifestText({
          version: '0.1.0',
          linux_appimage: { ...VALID_MANIFEST_WIRE.linux_appimage, sha256: 'c'.repeat(64) },
        }),
      },
    ]),
    invokeImpl: (async (cmd: string) => {
      if (cmd === 'is_install_eligible') {
        return {
          installed: {
            managedExecutablePath: '/opt/codex/codex.AppImage',
            installKind: 'app-image',
            channel: 'alpha',
            version: '0.1.0',
            sourceCommit: 'deadbeef',
            releaseTag: 'alpha/v0.1.0',
            manifestHash: 'manifest-hash',
            artifactSha256: 'c'.repeat(64),
            installedAt: '2026-07-03T00:00:00Z',
            updateEligible: true,
            ineligibleReason: null,
          },
          isManagedPathWritable: true,
        };
      }
      throw new Error(`unexpected invoke ${cmd}`);
    }) as InvokeLike,
  });

  await deps.controller.runCheck('alpha');

  const eligibility = deps.controller.computeEligibility(deps.installed, deps.lastCheck);
  assertEqual(
    eligibility,
    'ineligible',
    'a fetched manifest identical to what is already installed must resolve ineligible, never eligible',
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

// ---------- mount-time installed-state record ----------

const MOUNT_DEB_RECORD = {
  installed: {
    managedExecutablePath: '/usr/bin/codex-desktop',
    installKind: 'deb',
    channel: 'alpha',
    version: '0.16.140',
    sourceCommit: '157873a67e80',
    releaseTag: 'alpha/v0.16.140-157873a6',
    manifestHash: '',
    artifactSha256: SHA64,
    installedAt: '2026-10-07T00:00:00Z',
    updateEligible: true,
    ineligibleReason: null,
  },
  isManagedPathWritable: false,
};

async function verifiesMountTimeInstalledPanelReadsTheRealRecord() {
  const state = await loadMountTimeState({
    invokeImpl: (async (cmd: string) => {
      if (cmd === 'verify_relaunch_artifact') return { kind: 'no-pending-update' };
      if (cmd === 'is_install_eligible') return MOUNT_DEB_RECORD;
      throw new Error(`unexpected invoke ${cmd}`);
    }) as InvokeLike,
  });
  assertEqual(state.installed.version, '0.16.140', 'version comes from installed-state.json');
  assertEqual(state.installed.sourceCommit, '157873a67e80', 'source commit comes from the record');
  assertEqual(state.installed.artifactSha256, SHA64, 'artifact hash comes from the record');
  assertEqual(state.installed.installKind, 'deb', 'install kind comes from the record');
  assertEqual(state.installed.managedExecutablePath, '/usr/bin/codex-desktop', 'managed path comes from the record');
  assertEqual(state.installed.updateEligible, true, 'eligibility flag comes from the record');
  assertEqual(state.installed.channel, 'alpha', 'channel comes from the record');
}

async function verifiesMountTimeProbeFailureIsReportedNotHidden() {
  const state = await loadMountTimeState({
    invokeImpl: (async (cmd: string) => {
      if (cmd === 'verify_relaunch_artifact') return { kind: 'no-pending-update' };
      throw new Error('installed-state.json at /x is unreadable: expected value');
    }) as InvokeLike,
  });
  assertEqual(state.installed.version, 'unknown', 'no record means the version is genuinely unknown');
  assert(
    (state.installed.ineligibleReason ?? '').includes('unreadable'),
    `the probe failure must be shown, got: ${state.installed.ineligibleReason}`,
  );
}

async function verifiesPromotedStatePrefersTheWrittenRecord() {
  const state = await loadMountTimeState({
    invokeImpl: (async (cmd: string) => {
      if (cmd === 'verify_relaunch_artifact') {
        return { kind: 'promoted', installedStatePath: '/x/installed-state.json', promotedVersion: '0.16.140' };
      }
      if (cmd === 'is_install_eligible') return MOUNT_DEB_RECORD;
      throw new Error(`unexpected invoke ${cmd}`);
    }) as InvokeLike,
  });
  assertEqual(state.installed.artifactSha256, SHA64, 'the freshly written record supplies the hash');
}

// ---------- install(): the Install button's real action ----------

async function checkedDebDeps(performInstall: (args: Record<string, unknown> | undefined) => unknown) {
  const calls: Array<{ cmd: string; args: Record<string, unknown> | undefined }> = [];
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText() },
      {
        url: MANIFEST_URL,
        status: 200,
        responded: manifestText({ version: '0.2.0', schema_version: '1.2.0', linux_deb: LINUX_DEB_BLOCK }),
      },
    ]),
    invokeImpl: (async (cmd: string, args?: Record<string, unknown>) => {
      calls.push({ cmd, args });
      if (cmd === 'is_install_eligible') return DEB_PROBE;
      if (cmd === 'perform_install') return performInstall(args);
      throw new Error(`unexpected invoke ${cmd}`);
    }) as InvokeLike,
  });
  return { deps, calls };
}

async function verifiesInstallHandsTheFetchedManifestToPerformInstall() {
  const { deps, calls } = await checkedDebDeps(() => ({
    pendingUpdatePath: '/home/u/.config/codex/update/installed-state.json',
    managedExecutablePath: '/usr/bin/codex-desktop',
    fromVersion: '0.1.0',
    toVersion: '0.2.0',
    artifactSha256: 'd'.repeat(64),
  }));
  await deps.controller.runCheck('alpha');
  const response = await deps.controller.install();
  const call = calls.find((c) => c.cmd === 'perform_install');
  assert(call !== undefined, 'install() must invoke perform_install');
  const sent = call?.args as { manifest: { version: string; linux_deb: { sha256: string } }; indexUrl: string };
  assertEqual(sent.manifest.version, '0.2.0', 'the fetched manifest is what gets installed');
  assertEqual(sent.manifest.linux_deb.sha256, 'd'.repeat(64), 'the deb block reaches the backend untouched');
  assertEqual(sent.indexUrl, CHANNEL_INDEX_URL, 'the index url is passed through');
  assertEqual(response.toVersion, '0.2.0', 'the backend response is returned');
}

// ---------- Windows and AppImage installs ----------

const WINDOWS_PROBE = {
  installed: {
    managedExecutablePath: 'C:\\Users\\u\\AppData\\Local\\Codex\\codex-desktop.exe',
    installKind: 'windows-nsis',
    channel: 'alpha',
    version: '0.1.0',
    sourceCommit: 'deadbeef',
    releaseTag: 'alpha/v0.1.0',
    manifestHash: '',
    artifactSha256: SHA64,
    installedAt: '2026-07-03T00:00:00Z',
    updateEligible: true,
    ineligibleReason: null,
  },
  isManagedPathWritable: true,
};

const WINDOWS_NSIS_BLOCK = {
  name: 'Codex_0.2.0_x64-setup.exe',
  url: 'https://github.com/electricm0nk/codex/releases/download/alpha-v0.2.0/Codex_0.2.0_x64-setup.exe',
  sha256: 'e'.repeat(64),
  size_bytes: 10,
};

async function checkedDeps(probe: unknown, manifestOverrides: Record<string, unknown>, performInstall: () => unknown = () => ({
  pendingUpdatePath: 'p', managedExecutablePath: 'm', fromVersion: '0.1.0', toVersion: '0.2.0', artifactSha256: 'e'.repeat(64),
})) {
  const mountTimeState = await emptyMountTimeState();
  const deps = createUpdateControllerDeps(mountTimeState, 'alpha', {
    fetchImpl: makeFetchImpl([
      { url: CHANNEL_INDEX_URL, status: 200, responded: channelText() },
      { url: MANIFEST_URL, status: 200, responded: manifestText({ version: '0.2.0', ...manifestOverrides }) },
    ]),
    invokeImpl: (async (cmd: string) => {
      if (cmd === 'is_install_eligible') return probe;
      if (cmd === 'perform_install') return performInstall();
      throw new Error(`unexpected invoke ${cmd}`);
    }) as InvokeLike,
  });
  await deps.controller.runCheck('alpha');
  return deps;
}

async function verifiesAWindowsInstallIsEligibleWhenTheReleaseCarriesTheInstaller() {
  const deps = await checkedDeps(WINDOWS_PROBE, { schema_version: '1.3.0', windows_nsis: WINDOWS_NSIS_BLOCK });
  assertEqual(deps.controller.computeEligibility(deps.installed, deps.lastCheck), 'eligible', 'a windows install with a newer release and an installer');
  const result = await deps.controller.install();
  assertEqual(result.closesToFinish, true, 'a Windows update finishes by closing and reopening Codex');
}

async function verifiesAWindowsInstallIsNotOfferedARelease_WithoutTheInstaller() {
  const deps = await checkedDeps(WINDOWS_PROBE, {});
  assertEqual(deps.controller.computeEligibility(deps.installed, deps.lastCheck), 'ineligible', 'an AppImage-only release is not a Windows update');
  const reason = deps.controller.disabledReason(deps.installed, deps.lastCheck);
  assert((reason ?? '').includes('Windows installer'), `reason must name the missing installer, got: ${reason}`);
}

async function verifiesAnAppImageInstallStillChecksTheAppImageHashAndDoesNotCloseTheApp() {
  const appimageProbe = { ...DEB_PROBE, installed: { ...DEB_PROBE.installed, installKind: 'app-image', managedExecutablePath: '/home/u/Codex.AppImage', artifactSha256: 'f'.repeat(64) }, isManagedPathWritable: true };
  const deps = await checkedDeps(appimageProbe, {});
  assertEqual(deps.controller.computeEligibility(deps.installed, deps.lastCheck), 'eligible', 'a newer AppImage release is installable');
  const result = await deps.controller.install();
  assertEqual(result.closesToFinish ?? false, false, 'an AppImage update asks for a restart, it does not close the app');
}

async function verifiesInstallBeforeAnyCheckFailsLoudly() {
  const { deps } = await checkedDebDeps(() => {
    throw new Error('must not be reached');
  });
  let message = '';
  try {
    await deps.controller.install();
  } catch (cause) {
    message = cause instanceof Error ? cause.message : String(cause);
  }
  assert(message.includes('Check'), `install without a checked manifest must say to run Check, got: ${message}`);
}

async function verifiesInstallSurfacesTheBackendFailure() {
  const { deps } = await checkedDebDeps(() => {
    throw new Error('installation was cancelled at the authorization prompt');
  });
  await deps.controller.runCheck('alpha');
  let message = '';
  try {
    await deps.controller.install();
  } catch (cause) {
    message = cause instanceof Error ? cause.message : String(cause);
  }
  assert(message.includes('cancelled'), `the backend reason must reach the UI, got: ${message}`);
}

async function verifiesInstallRefusesWhenNotEligible() {
  const { deps, calls } = await checkedDebDeps(() => {
    throw new Error('must not be reached');
  });
  // No Check has run: eligibility is unknown, so the backend must not be asked to install.
  try {
    await deps.controller.install();
  } catch {
    /* expected */
  }
  assert(!calls.some((c) => c.cmd === 'perform_install'), 'perform_install must not be called when not eligible');
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
  await verifiesRunCheckFetchesAndVerifiesReleaseNotesOnSuccess();
  await verifiesRunCheckLeavesReleaseNotesUnavailableOnHashMismatch();
  await verifiesRunCheckSurfacesRealIndexFetchFailure();
  await verifiesRunCheckSurfacesRealSchemaInvalidReason();
  await verifiesRunCheckSurfacesRealManifestFetchFailure();
  await verifiesEligibleWhenRealLocalProbeIsOlderThanFetchedManifest();
  await verifiesIneligibleWhenRealLocalProbeReportsDevLocalInstall();
  await verifiesUnknownHonestlyWhenNoLocalInstalledStateRecordExists();
  await verifiesUnknownHonestlyWhenLocalProbeInvokeFails();
  await verifiesIneligibleWhenRealLocalProbeReportsUnwritableManagedPath();
  await verifiesEligibleWhenDebInstallSeesNewerManifestWithDebArtifact();
  await verifiesIneligibleWhenDebInstallSeesManifestWithoutDebArtifact();
  await verifiesIneligibleWhenFetchedManifestMatchesAlreadyInstalledVersion();
  await verifiesEligibilityUnknownBeforeAnyCheck();
  await verifiesNoPendingUpdateMapsToEmptyState();
  await verifiesVerificationFailedOffersRestoreHonestly();
  await verifiesPromotedMapsKnownFieldsOnlyWithoutFabricating();
  await verifiesMountTimeInstalledPanelReadsTheRealRecord();
  await verifiesMountTimeProbeFailureIsReportedNotHidden();
  await verifiesPromotedStatePrefersTheWrittenRecord();
  await verifiesInstallHandsTheFetchedManifestToPerformInstall();
  await verifiesAWindowsInstallIsEligibleWhenTheReleaseCarriesTheInstaller();
  await verifiesAWindowsInstallIsNotOfferedARelease_WithoutTheInstaller();
  await verifiesAnAppImageInstallStillChecksTheAppImageHashAndDoesNotCloseTheApp();
  await verifiesInstallBeforeAnyCheckFailsLoudly();
  await verifiesInstallSurfacesTheBackendFailure();
  await verifiesInstallRefusesWhenNotEligible();
  await verifiesRestorePreviousVersionCallsRealCommand();
  await verifiesRestorePreviousVersionSurfacesFailureHonestly();
  console.log('controllerAdapter.test.ts: all assertions passed');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

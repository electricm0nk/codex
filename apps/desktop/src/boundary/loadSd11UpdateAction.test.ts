import { loadSd11UpdateAction, type Sd11UpdateActionRequest } from './loadSd11UpdateAction';
import type { Sd11TesterChannelLabel } from '../sd11/update/updateActionModel';
import { channelIndexUrl } from '../sd16/update/fetch';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * `boundary/runtime.ts` decides "Tauri is present" by checking
 * `typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window || ...)`.
 * In node tsx that is false, so without this stub `loadSd11UpdateAction`
 * short-circuits to the no-runtime sentinel before ever calling `fetch`.
 */
function withTauriRuntime(): { restore: () => void } {
  const win = (globalThis as Record<string, unknown>).window as
    | Record<string, unknown>
    | undefined;
  const hadWindow = win !== undefined;
  const previousWindow = win;
  const stubWindow: Record<string, unknown> = hadWindow ? { ...previousWindow } : {};
  stubWindow.__TAURI_INTERNALS__ = {};
  (globalThis as Record<string, unknown>).window = stubWindow;
  return {
    restore: () => {
      if (hadWindow && previousWindow) {
        (globalThis as Record<string, unknown>).window = previousWindow;
      } else {
        delete (globalThis as Record<string, unknown>).window;
      }
    },
  };
}

interface StubResponse {
  status: number;
  body: string;
}

/** Swap `globalThis.fetch` for the duration of a call so `fetch.ts`'s `defaultFetchImpl` is exercised deterministically. */
function withStubFetch(responses: StubResponse[]): { restore: () => void } {
  const previousFetch = (globalThis as Record<string, unknown>).fetch;
  let i = 0;
  (globalThis as Record<string, unknown>).fetch = async () => {
    if (i >= responses.length) {
      throw new Error(`stub fetch called more times than expected (${responses.length})`);
    }
    const next = responses[i++];
    return {
      ok: next.status >= 200 && next.status < 300,
      status: next.status,
      text: async () => next.body,
    };
  };
  return {
    restore: () => {
      (globalThis as Record<string, unknown>).fetch = previousFetch;
    },
  };
}

// Canonical wire fixtures per schemas/update/channel-index.schema.json and
// schemas/update/update-manifest.schema.json — the same contract the E4
// release lane validates before publishing to `update-index`.
const SHA40 = 'abcdef0123456789abcdef0123456789abcdef01';
const SHA64 = 'abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789';

const MANIFEST_URL =
  'https://raw.githubusercontent.com/electricm0nk/codex/update-index/manifests/alpha/v0.1.0-abc12345/update-manifest.json';

const VALID_CHANNEL_WIRE_JSON = JSON.stringify({
  schema_version: '1.0.0',
  channel: 'alpha',
  version: '0.1.0',
  tag: 'alpha/v0.1.0-abc12345',
  release_url: 'https://github.com/electricm0nk/codex/releases/tag/alpha-v0.1.0-abc12345',
  manifest_url: MANIFEST_URL,
  publication_timestamp: '2026-07-03T12:00:00Z',
  tranche_id: 'STC-CODEX-SD-16',
  signature: null,
});

const VALID_MANIFEST_WIRE = {
  schema_version: '1.0.0',
  channel: 'alpha',
  version: '0.1.0',
  tag: 'alpha/v0.1.0-abc12345',
  tranche_id: 'STC-CODEX-SD-16',
  source_branch: 'develop',
  source_commit: SHA40,
  release_notes_path:
    'programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/release-notes.md',
  release_notes_url:
    'https://github.com/electricm0nk/codex/releases/tag/alpha-v0.1.0-abc12345',
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
    required_install_kind: 'appimage',
  },
  promotion_lineage: {
    source_branch: 'develop',
    source_commit: SHA40,
    promoted_at: '2026-07-03T12:00:00Z',
  },
  signature: null,
};

const VALID_MANIFEST_WIRE_JSON = JSON.stringify(VALID_MANIFEST_WIRE);

// Valid JSON, but schema-invalid (missing the required `linux_appimage`
// object) so it exercises the `invalid-manifest` failure branch rather
// than the `invalid-json` branch.
const STRUCTURALLY_INVALID_MANIFEST_WIRE_JSON = JSON.stringify(
  Object.fromEntries(
    Object.entries(VALID_MANIFEST_WIRE).filter(([key]) => key !== 'linux_appimage')
  )
);

function request(overrides: Partial<Sd11UpdateActionRequest> = {}): Sd11UpdateActionRequest {
  return {
    buildVersion: '0.0.0',
    buildLabel: 'codex@0.0.0',
    platformLabel: 'Linux',
    testerChannelLabel: 'alpha' as Sd11TesterChannelLabel,
    ...overrides,
  };
}

async function main() {
  await testHappyPathAlphaReturnsGovernedRelease();
  await testUpToDateBuildCollapsesLatestToCurrent();
  await testBuildBelowVersionFloorIsManualOnly();
  await testHttpErrorOnChannelIndexReturnsCheckFailed();
  await testInvalidManifestReturnsCheckFailed();
  await testUnsupportedChannelReturnsCheckFailed();
  await testNoRuntimeReturnsCheckFailed();
}

async function testHappyPathAlphaReturnsGovernedRelease() {
  const runtime = withTauriRuntime();
  const fetchStub = withStubFetch([
    { status: 200, body: VALID_CHANNEL_WIRE_JSON },
    { status: 200, body: VALID_MANIFEST_WIRE_JSON },
  ]);
  try {
    const result = await loadSd11UpdateAction(request());
    assertEqual(result.kind, 'governed-release', 'happy path resolves governed-release');
    if (result.kind !== 'governed-release') {
      throw new Error('unreachable');
    }
    assertEqual(result.manifest.channel, 'alpha', 'translated channel');
    // The current build is the RUNNING build, not the manifest's release —
    // otherwise no update could ever be detected.
    assertEqual(
      result.manifest.currentBuild.releaseId,
      'codex@0.0.0',
      'currentBuild identity comes from the running build'
    );
    assertEqual(result.manifest.currentBuild.version, '0.0.0', 'running version');
    // The manifest offers 0.1.0 > 0.0.0, so the latest eligible build is
    // the manifest's release identity.
    assertEqual(
      result.manifest.latestEligibleBuild.releaseId,
      'alpha/v0.1.0-abc12345',
      'latestEligibleBuild is the manifest release when newer'
    );
    assertEqual(result.manifest.latestEligibleBuild.version, '0.1.0', 'manifest version');
    assertEqual(
      result.manifest.latestEligibleBuild.commitOrProvenanceHandle,
      SHA40,
      'manifest source commit carried as provenance handle'
    );
    assertEqual(result.manifest.eligibilityState, 'automatic', 'translated eligibility');
    assertEqual(result.manifest.integrity.checksumAvailable, true, 'checksum available');
    assertEqual(result.manifest.integrity.provenanceAvailable, true, 'provenance available');
    assertEqual(result.manifest.integrity.linuxArtifactPresent, true, 'linux artifact present');
    assertEqual(
      result.manifest.integrity.recoveryPostureDefined,
      true,
      'recovery posture defined from release_notes_url'
    );
    assert(result.manifest.notes.length === 1, 'notes carries the release_notes_url');
  } finally {
    fetchStub.restore();
    runtime.restore();
  }
}

async function testUpToDateBuildCollapsesLatestToCurrent() {
  const runtime = withTauriRuntime();
  const fetchStub = withStubFetch([
    { status: 200, body: VALID_CHANNEL_WIRE_JSON },
    { status: 200, body: VALID_MANIFEST_WIRE_JSON },
  ]);
  try {
    const result = await loadSd11UpdateAction(
      request({
        buildVersion: '0.1.0',
        buildLabel: 'codex@0.1.0',
      })
    );
    assertEqual(result.kind, 'governed-release', 'up-to-date build still governed-release');
    if (result.kind !== 'governed-release') {
      throw new Error('unreachable');
    }
    // The manifest offers nothing newer than the running build, so the
    // latest eligible build IS the current build (deriveSd11UpdateAction
    // classifies equal identities as up-to-date).
    assertEqual(
      result.manifest.latestEligibleBuild.releaseId,
      result.manifest.currentBuild.releaseId,
      'latestEligibleBuild collapses to currentBuild when manifest is not newer'
    );
  } finally {
    fetchStub.restore();
    runtime.restore();
  }
}

async function testBuildBelowVersionFloorIsManualOnly() {
  const runtime = withTauriRuntime();
  const flooredManifest = JSON.stringify({
    ...VALID_MANIFEST_WIRE,
    eligibility: {
      ...VALID_MANIFEST_WIRE.eligibility,
      min_supported_version: '0.0.5',
    },
  });
  const fetchStub = withStubFetch([
    { status: 200, body: VALID_CHANNEL_WIRE_JSON },
    { status: 200, body: flooredManifest },
  ]);
  try {
    const result = await loadSd11UpdateAction(request({ buildVersion: '0.0.1' }));
    assertEqual(result.kind, 'governed-release', 'floored build still governed-release');
    if (result.kind !== 'governed-release') {
      throw new Error('unreachable');
    }
    assertEqual(
      result.manifest.eligibilityState,
      'manual-only',
      'a running build below min_supported_version is not automatic-eligible'
    );
  } finally {
    fetchStub.restore();
    runtime.restore();
  }
}

async function testHttpErrorOnChannelIndexReturnsCheckFailed() {
  const runtime = withTauriRuntime();
  const fetchStub = withStubFetch([{ status: 404, body: '' }]);
  try {
    const result = await loadSd11UpdateAction(request());
    assertEqual(result.kind, 'check-failed', 'http error resolves check-failed');
    if (result.kind !== 'check-failed') {
      throw new Error('unreachable');
    }
    assertEqual(
      result.reason,
      `HTTP 404 when fetching ${channelIndexUrl('alpha')}`,
      'http error reason is verbatim'
    );
    assertEqual(result.buildLabel, request().buildLabel, 'buildLabel carried through');
    assertEqual(result.version, request().buildVersion, 'version carried through');
  } finally {
    fetchStub.restore();
    runtime.restore();
  }
}

async function testInvalidManifestReturnsCheckFailed() {
  const runtime = withTauriRuntime();
  const fetchStub = withStubFetch([
    { status: 200, body: VALID_CHANNEL_WIRE_JSON },
    { status: 200, body: STRUCTURALLY_INVALID_MANIFEST_WIRE_JSON },
  ]);
  try {
    const result = await loadSd11UpdateAction(request());
    assertEqual(result.kind, 'check-failed', 'invalid manifest resolves check-failed');
    if (result.kind !== 'check-failed') {
      throw new Error('unreachable');
    }
    assert(
      result.reason.startsWith(`Update-manifest validation failed at ${MANIFEST_URL}:`),
      `invalid manifest reason must be the verbatim invalid-manifest translation, got: ${result.reason}`
    );
  } finally {
    fetchStub.restore();
    runtime.restore();
  }
}

async function testUnsupportedChannelReturnsCheckFailed() {
  const runtime = withTauriRuntime();
  try {
    const result = await loadSd11UpdateAction(
      request({ testerChannelLabel: 'omega-channel' as unknown as Sd11TesterChannelLabel })
    );
    assertEqual(result.kind, 'check-failed', 'unsupported channel resolves check-failed');
    if (result.kind !== 'check-failed') {
      throw new Error('unreachable');
    }
    assertEqual(
      result.reason,
      'Channel "omega-channel" is not supported in this tranche',
      'unsupported channel reason is verbatim'
    );
  } finally {
    runtime.restore();
  }
}

async function testNoRuntimeReturnsCheckFailed() {
  const result = await loadSd11UpdateAction(request());
  assertEqual(result.kind, 'check-failed', 'no runtime resolves check-failed');
  if (result.kind !== 'check-failed') {
    throw new Error('unreachable');
  }
  assertEqual(
    result.reason,
    'Desktop runtime boundary is unavailable, so governed SD-12 release truth cannot be proven from this context.',
    'no runtime reason is verbatim'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

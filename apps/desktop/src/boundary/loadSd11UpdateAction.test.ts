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

const MANIFEST_URL =
  'https://github.com/electricm0nk/codex/releases/download/alpha-v0.1.0-abc1234/update-manifest.json';

const VALID_CHANNEL_WIRE_JSON = JSON.stringify({
  schema_version: 'v1',
  channel: 'alpha',
  release: {
    tag: 'alpha-v0.1.0-abc1234',
    published_at: '2026-07-03T12:00:00Z',
    manifest_url: MANIFEST_URL,
    tranche_id: 'codex-tranche-2-5',
  },
  signature: null,
});

const VALID_MANIFEST_WIRE_JSON = JSON.stringify({
  schema_version: 'v1',
  channel: 'alpha',
  eligibility: 'automatic',
  artifact: {
    release_id: 'rel-001',
    version: '0.1.0',
    build_label: 'codex-desktop-shell-scaffold@0.1.0',
    commit_or_provenance_handle: 'abc1234',
    published_at: '2026-07-03T12:00:00Z',
    artifact_sha256:
      '0000000000000000000000000000000000000000000000000000000000000000',
    path: 'Codex-0.1.0-x86_64.AppImage',
  },
  notes_url: 'https://example.invalid/alpha/notes.md',
  signature: null,
});

// Valid JSON, but structurally invalid per `validateManifestShape` (missing
// the required `artifact.path` field) so it exercises the `invalid-manifest`
// failure branch rather than the `invalid-json` branch.
const STRUCTURALLY_INVALID_MANIFEST_WIRE_JSON = JSON.stringify({
  schema_version: 'v1',
  channel: 'alpha',
  eligibility: 'automatic',
  artifact: {
    release_id: 'rel-001',
    version: '0.1.0',
    build_label: 'codex-desktop-shell-scaffold@0.1.0',
    commit_or_provenance_handle: 'abc1234',
    published_at: '2026-07-03T12:00:00Z',
    artifact_sha256:
      '0000000000000000000000000000000000000000000000000000000000000000',
  },
  signature: null,
});

function request(overrides: Partial<Sd11UpdateActionRequest> = {}): Sd11UpdateActionRequest {
  return {
    buildVersion: '0.1.0',
    buildLabel: 'codex-desktop-shell-scaffold@0.1.0',
    platformLabel: 'Linux',
    testerChannelLabel: 'alpha' as Sd11TesterChannelLabel,
    ...overrides,
  };
}

async function main() {
  await testHappyPathAlphaReturnsGovernedRelease();
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
    assertEqual(result.manifest.currentBuild.releaseId, 'Codex-0.1.0-x86_64.AppImage', 'releaseId derived from artifact.path');
    assertEqual(result.manifest.currentBuild.version, '0.1.0', 'translated version');
    assertEqual(result.manifest.eligibilityState, 'automatic', 'translated eligibility');
    assertEqual(result.manifest.integrity.checksumAvailable, true, 'checksum available');
    assertEqual(result.manifest.integrity.provenanceAvailable, true, 'provenance available');
    assertEqual(result.manifest.integrity.linuxArtifactPresent, true, 'linux artifact present');
    assertEqual(result.manifest.integrity.recoveryPostureDefined, true, 'recovery posture defined from notesUrl');
    assert(result.manifest.notes.length === 1, 'notes carries the notesUrl');
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

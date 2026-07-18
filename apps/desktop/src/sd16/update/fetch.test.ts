import { assertEqual, assert } from '../../testSupport/asserts';
import {
  channelIndexUrl,
  fetchChannelIndex,
  fetchUpdateManifest,
  validateChannelIndexShape,
  validateManifestShape,
  type FetchFailure,
  type FetchLike,
  type FetchResult,
  type ChannelIndexFile,
} from './fetch';

// ---------- helpers ----------

interface Stub {
  url: string;
  responded: string;
  status: number;
}

/**
 * Build a `FetchLike` whose each call consumes the matching stub in order.
 * Throws on URL mismatch (defense against accidental fan-out — discovery
 * never calls the network twice for the same purpose).
 */
function makeFetchImpl(stubs: Stub[]): FetchLike {
  let i = 0;
  return async (input) => {
    if (i >= stubs.length) {
      throw new Error(
        `stub fetchImpl called more times than expected (${stubs.length}); url=${String(input)}`
      );
    }
    const expected = stubs[i++];
    const url = typeof input === 'string' ? input : String(input);
    if (url !== expected.url) {
      throw new Error(`stub mismatch: expected ${expected.url}, got ${url}`);
    }
    return {
      ok: expected.status >= 200 && expected.status < 300,
      status: expected.status,
      text: async () => expected.responded,
    };
  };
}

function expectOk<T>(result: FetchResult<T>, label: string): T {
  if (!result.ok) {
    throw new Error(`${label}: expected ok, got failure ${JSON.stringify(result.failure)}`);
  }
  return result.value;
}

function expectFailure<T>(
  result: FetchResult<T>,
  kind: FetchFailure['kind'],
  label: string
): FetchFailure {
  if (result.ok) {
    throw new Error(`${label}: expected failure ${kind}, got ok ${JSON.stringify(result.value)}`);
  }
  assertEqual(result.failure.kind, kind, `${label}: failure kind`);
  return result.failure;
}

// The fetcher/validator operates on the canonical wire format defined by
// `schemas/update/channel-index.schema.json` and
// `schemas/update/update-manifest.schema.json` — the same contract the E4
// release lane validates before publishing to the `update-index` branch.
const SHA40 = 'abcdef0123456789abcdef0123456789abcdef01';
const SHA64 = 'abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789';

const MANIFEST_URL =
  'https://raw.githubusercontent.com/electricm0nk/codex/update-index/manifests/alpha/v0.1.0-abc12345/update-manifest.json';

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

// ---------- URL + pure-validator tests ----------

function verifiesChannelIndexUrlShape() {
  assertEqual(
    channelIndexUrl('alpha'),
    'https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels/alpha.json',
    'alpha URL'
  );
  assertEqual(
    channelIndexUrl('beta'),
    'https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels/beta.json',
    'beta URL'
  );
  assertEqual(
    channelIndexUrl('stable'),
    'https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels/stable.json',
    'stable URL'
  );
}

function verifiesValidChannelIndexShapeAccepted() {
  const result = validateChannelIndexShape(channelText(), channelIndexUrl('alpha'));
  const value = expectOk(result, 'valid channel should pass validator');
  assertEqual(value.channel, 'alpha', 'channel preserved');
  assertEqual(value.tag, 'alpha/v0.1.0-abc12345', 'tag preserved');
  assertEqual(value.manifest_url, MANIFEST_URL, 'manifest_url preserved');
  assertEqual(value.tranche_id, 'STC-CODEX-SD-16', 'tranche_id preserved');
  assertEqual(value.version, '0.1.0', 'version preserved');
}

function verifiesChannelIndexRejectsLegacySchemaVersion() {
  // "v1" was the pre-E3 placeholder literal; the canonical contract is the
  // semver string "1.0.0". A legacy index must fail closed.
  const result = validateChannelIndexShape(
    channelText({ schema_version: 'v1' }),
    channelIndexUrl('alpha')
  );
  expectFailure(result, 'invalid-channel-index', 'legacy schema_version');
}

function verifiesChannelIndexRejectsLegacyNestedShape() {
  // The pre-F4 wire format nested the pointer under `release`; the canonical
  // channel-index contract is flat. The legacy shape must fail closed.
  const legacy = JSON.stringify({
    schema_version: 'v1',
    channel: 'alpha',
    release: {
      tag: 'alpha-v0.1.0-abc12345',
      published_at: '2026-07-03T12:00:00Z',
      manifest_url: MANIFEST_URL,
      tranche_id: 'codex-tranche-2-5',
    },
    signature: null,
  });
  const result = validateChannelIndexShape(legacy, channelIndexUrl('alpha'));
  expectFailure(result, 'invalid-channel-index', 'legacy nested shape');
}

function verifiesInvalidChannelIndexRejectsMissingManifestUrl() {
  const wire = { ...VALID_CHANNEL_WIRE } as Record<string, unknown>;
  delete wire.manifest_url;
  const result = validateChannelIndexShape(JSON.stringify(wire), channelIndexUrl('alpha'));
  expectFailure(result, 'invalid-channel-index', 'missing manifest_url');
}

function verifiesInvalidChannelIndexRejectsUnknownChannel() {
  const result = validateChannelIndexShape(
    channelText({ channel: 'rc' }),
    channelIndexUrl('alpha')
  );
  expectFailure(result, 'invalid-channel-index', 'unknown channel');
}

function verifiesInvalidChannelIndexRejectsBadSignatureType() {
  const result = validateChannelIndexShape(
    channelText({ signature: 42 }),
    channelIndexUrl('alpha')
  );
  expectFailure(result, 'invalid-channel-index', 'bad signature type');
}

function verifiesValidManifestShapeAccepted() {
  const result = validateManifestShape(manifestText(), MANIFEST_URL);
  const value = expectOk(result, 'valid manifest should pass validator');
  assertEqual(value.version, '0.1.0', 'version preserved');
  assertEqual(value.linux_appimage.sha256.length, 64, 'sha256 length');
  assertEqual(value.eligibility.appimage_install, true, 'eligibility preserved');
  assertEqual(value.eligibility.min_supported_version, '0.0.0', 'min_supported_version preserved');
  assertEqual(value.source_commit, SHA40, 'source_commit preserved');
}

function verifiesManifestRejectsLegacySchemaVersion() {
  const result = validateManifestShape(
    manifestText({ schema_version: 'v1' }),
    MANIFEST_URL
  );
  expectFailure(result, 'invalid-manifest', 'legacy schema_version');
}

function verifiesInvalidManifestRejectsMissingLinuxAppimage() {
  const wire = { ...VALID_MANIFEST_WIRE } as Record<string, unknown>;
  delete wire.linux_appimage;
  const result = validateManifestShape(JSON.stringify(wire), MANIFEST_URL);
  expectFailure(result, 'invalid-manifest', 'missing linux_appimage');
}

function verifiesInvalidManifestRejectsMalformedSha256() {
  const result = validateManifestShape(
    manifestText({
      linux_appimage: { ...VALID_MANIFEST_WIRE.linux_appimage, sha256: '' },
    }),
    MANIFEST_URL
  );
  expectFailure(result, 'invalid-manifest', 'empty sha256');
}

function verifiesInvalidManifestRejectsChannelTagMismatch() {
  // Cross-field rule from the canonical schema: tag prefix must equal channel.
  const result = validateManifestShape(
    manifestText({ tag: 'beta/v0.1.0-abc12345' }),
    MANIFEST_URL
  );
  expectFailure(result, 'invalid-manifest', 'channel/tag prefix mismatch');
}

// ---------- fetcher tests (network stubbed) ----------

async function verifiesFetcherSucceedsOnValidChannel() {
  const result = await fetchChannelIndex('alpha', {
    fetchImpl: makeFetchImpl([
      {
        url: channelIndexUrl('alpha'),
        responded: channelText(),
        status: 200,
      },
    ]),
  });
  const value = expectOk(result, 'channel fetcher should succeed');
  assertEqual(value.channel, 'alpha', 'channel in result');
  assertEqual(value.tranche_id, 'STC-CODEX-SD-16', 'tranche_id in result');
}

async function verifiesFetcherRejectsHttpError() {
  const result = await fetchChannelIndex('beta', {
    fetchImpl: makeFetchImpl([
      { url: channelIndexUrl('beta'), responded: 'not found body', status: 404 },
    ]),
  });
  const failure = expectFailure(result, 'http-error', '404 should fail closed');
  // http-error is the only FetchFailure variant carrying .status
  if (failure.kind !== 'http-error') {
    throw new Error(`expected http-error variant, got ${failure.kind}`);
  }
  assertEqual(failure.status, 404, 'http-error status');
}

async function verifiesFetcherRejectsInvalidJson() {
  const result = await fetchChannelIndex('alpha', {
    fetchImpl: makeFetchImpl([
      { url: channelIndexUrl('alpha'), responded: '{"oops":', status: 200 },
    ]),
  });
  expectFailure(result, 'invalid-json', 'invalid JSON should fail');
}

async function verifiesFetcherRejectsInvalidShape() {
  const result = await fetchChannelIndex('alpha', {
    fetchImpl: makeFetchImpl([
      {
        url: channelIndexUrl('alpha'),
        responded: channelText({ schema_version: 'v9' }),
        status: 200,
      },
    ]),
  });
  expectFailure(result, 'invalid-channel-index', 'bad shape should fail');
}

async function verifiesFetcherRejectsUnsupportedChannel() {
  // The type system already constrains this, but mirror the upstream guard
  // at runtime so a caller cannot smuggle in 'rc' via string coercion.
  const result = await fetchChannelIndex(
    'rc' as unknown as ChannelIndexFile['channel']
  );
  expectFailure(result, 'unsupported-channel', 'unsupported channel');
}

async function verifiesManifestFetcherSucceedsOnValidManifest() {
  const result = await fetchUpdateManifest(MANIFEST_URL, {
    fetchImpl: makeFetchImpl([
      {
        url: MANIFEST_URL,
        responded: manifestText(),
        status: 200,
      },
    ]),
  });
  const value = expectOk(result, 'manifest fetcher should succeed');
  assertEqual(value.version, '0.1.0', 'manifest version preserved');
}

async function verifiesManifestFetcherRejectsInvalidShape() {
  const result = await fetchUpdateManifest(MANIFEST_URL, {
    fetchImpl: makeFetchImpl([
      {
        url: MANIFEST_URL,
        responded: manifestText({ linux_appimage: null }),
        status: 200,
      },
    ]),
  });
  expectFailure(result, 'invalid-manifest', 'bad manifest shape');
}

async function verifiesManifestFetcherRejectsEmptyUrl() {
  const result = await fetchUpdateManifest('');
  expectFailure(result, 'invalid-manifest', 'empty url should fail closed');
}

async function verifiesManifestFetchesOnlyManifestUrl() {
  // Defense-in-depth: the shell must NOT make GitHub Releases scanning
  // calls during discovery. We pin this via the URL contract — fetchChannelIndex
  // only ever asks for the channel index on the update-index branch, and
  // fetchUpdateManifest only ever asks for the URL the index already named
  // (the F6 CORS-friendly mirror, also on the update-index branch).
  const channelCalls: string[] = [];
  const manifestCalls: string[] = [];
  const routingFetch: FetchLike = async (input) => {
    const url = typeof input === 'string' ? input : String(input);
    if (url.includes('/update-index/channels/')) {
      channelCalls.push(url);
      return {
        ok: true,
        status: 200,
        text: async () => channelText(),
      };
    }
    manifestCalls.push(url);
    return {
      ok: true,
      status: 200,
      text: async () => manifestText(),
    };
  };

  const channelResult = await fetchChannelIndex('alpha', { fetchImpl: routingFetch });
  const channelValue = expectOk(channelResult, 'channel fetcher ok in fan-out test');
  const manifestResult = await fetchUpdateManifest(channelValue.manifest_url, {
    fetchImpl: routingFetch,
  });
  expectOk(manifestResult, 'manifest fetcher ok in fan-out test');

  assertEqual(channelCalls.length, 1, 'exactly one channel-index call');
  assertEqual(manifestCalls.length, 1, 'exactly one manifest call');
  assertEqual(
    manifestCalls[0],
    MANIFEST_URL,
    'manifest call resolved via the index.manifest_url'
  );
  // Guard against GitHub Releases API scanning creeping in.
  const allCalls = [...channelCalls, ...manifestCalls];
  assert(
    !allCalls.some((u) => u.includes('/releases/')),
    'discovery must not call the GitHub Releases API'
  );
  assert(
    !allCalls.some((u) => u.includes('/repos/')),
    'discovery must not call GitHub repo APIs'
  );
}

async function main() {
  // pure-validator tests
  verifiesChannelIndexUrlShape();
  verifiesValidChannelIndexShapeAccepted();
  verifiesChannelIndexRejectsLegacySchemaVersion();
  verifiesChannelIndexRejectsLegacyNestedShape();
  verifiesInvalidChannelIndexRejectsMissingManifestUrl();
  verifiesInvalidChannelIndexRejectsUnknownChannel();
  verifiesInvalidChannelIndexRejectsBadSignatureType();
  verifiesValidManifestShapeAccepted();
  verifiesManifestRejectsLegacySchemaVersion();
  verifiesInvalidManifestRejectsMissingLinuxAppimage();
  verifiesInvalidManifestRejectsMalformedSha256();
  verifiesInvalidManifestRejectsChannelTagMismatch();

  // fetcher tests
  await verifiesFetcherSucceedsOnValidChannel();
  await verifiesFetcherRejectsHttpError();
  await verifiesFetcherRejectsInvalidJson();
  await verifiesFetcherRejectsInvalidShape();
  await verifiesFetcherRejectsUnsupportedChannel();
  await verifiesManifestFetcherSucceedsOnValidManifest();
  await verifiesManifestFetcherRejectsInvalidShape();
  await verifiesManifestFetcherRejectsEmptyUrl();
  await verifiesManifestFetchesOnlyManifestUrl();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

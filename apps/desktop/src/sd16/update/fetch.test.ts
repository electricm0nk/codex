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
  type Sd16ChannelIndexFile,
  type Sd16UpdateManifestFile,
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

// The fetcher/validator operates on the wire format (snake_case) that the
// E4 release lane writes onto the `update-index` branch; the parsed result
// surfaces camelCase TS types.
const VALID_CHANNEL_WIRE = {
  schema_version: 'v1',
  channel: 'alpha' as const,
  release: {
    tag: 'alpha-v0.1.0-abc1234',
    published_at: '2026-07-03T12:00:00Z',
    manifest_url:
      'https://github.com/electricm0nk/codex/releases/download/alpha-v0.1.0-abc1234/update-manifest.json',
    tranche_id: 'codex-tranche-2-5',
  },
  signature: null,
};

const VALID_MANIFEST_WIRE = {
  schema_version: 'v1',
  channel: 'alpha' as const,
  eligibility: 'automatic' as const,
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
  signature: null,
};

const MANIFEST_URL =
  'https://github.com/electricm0nk/codex/releases/download/alpha-v0.1.0-abc1234/update-manifest.json';

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
  const result = validateChannelIndexShape(VALID_CHANNEL_WIRE, channelIndexUrl('alpha'));
  const value = expectOk(result, 'valid channel should pass validator');
  assertEqual(value.channel, 'alpha', 'channel preserved');
  assertEqual(value.release.tag, 'alpha-v0.1.0-abc1234', 'tag preserved');
  assertEqual(value.release.manifestUrl, MANIFEST_URL, 'manifestUrl preserved');
  assertEqual(value.release.trancheId, 'codex-tranche-2-5', 'trancheId preserved');
}

function verifiesInvalidChannelIndexRejectsBadSchemaVersion() {
  const result = validateChannelIndexShape(
    { ...VALID_CHANNEL_WIRE, schema_version: 'v2' },
    channelIndexUrl('alpha')
  );
  expectFailure(result, 'invalid-channel-index', 'bad schema_version');
}

function verifiesInvalidChannelIndexRejectsMissingManifestUrl() {
  const raw = {
    schema_version: 'v1',
    channel: 'alpha',
    release: {
      tag: 'alpha-v0.1.0-abc1234',
      published_at: '2026-07-03T12:00:00Z',
      tranche_id: 'codex-tranche-2-5',
    },
    signature: null,
  };
  const result = validateChannelIndexShape(raw, channelIndexUrl('alpha'));
  expectFailure(result, 'invalid-channel-index', 'missing manifest_url');
}

function verifiesInvalidChannelIndexRejectsUnknownChannel() {
  const result = validateChannelIndexShape(
    { ...VALID_CHANNEL_WIRE, channel: 'rc' },
    channelIndexUrl('alpha')
  );
  expectFailure(result, 'invalid-channel-index', 'unknown channel');
}

function verifiesInvalidChannelIndexRejectsBadSignatureType() {
  const result = validateChannelIndexShape(
    { ...VALID_CHANNEL_WIRE, signature: 42 },
    channelIndexUrl('alpha')
  );
  expectFailure(result, 'invalid-channel-index', 'bad signature type');
}

function verifiesValidManifestShapeAccepted() {
  const result = validateManifestShape(VALID_MANIFEST_WIRE, MANIFEST_URL);
  const value = expectOk(result, 'valid manifest should pass validator');
  assertEqual(value.eligibility, 'automatic', 'eligibility preserved');
  assertEqual(value.artifact.artifactSha256.length, 64, 'sha256 length');
}

function verifiesInvalidManifestRejectsBadEligibility() {
  const result = validateManifestShape(
    { ...VALID_MANIFEST_WIRE, eligibility: 'definitely-yes' },
    MANIFEST_URL
  );
  expectFailure(result, 'invalid-manifest', 'bad eligibility');
}

function verifiesInvalidManifestRejectsEmptyArtifactSha256() {
  const result = validateManifestShape(
    {
      ...VALID_MANIFEST_WIRE,
      artifact: { ...VALID_MANIFEST_WIRE.artifact, artifact_sha256: '' },
    },
    MANIFEST_URL
  );
  expectFailure(result, 'invalid-manifest', 'empty sha256');
}

function verifiesInvalidManifestRejectsMissingArtifactSha256() {
  const raw = {
    schema_version: 'v1',
    channel: 'alpha',
    eligibility: 'automatic',
    artifact: {
      release_id: 'rel-001',
      version: '0.1.0',
      build_label: 'codex-desktop-shell-scaffold@0.1.0',
      commit_or_provenance_handle: 'abc1234',
      published_at: '2026-07-03T12:00:00Z',
      path: 'Codex-0.1.0-x86_64.AppImage',
    },
    signature: null,
  };
  const result = validateManifestShape(raw, MANIFEST_URL);
  expectFailure(result, 'invalid-manifest', 'missing sha256');
}

// ---------- fetcher tests (network stubbed) ----------

async function verifiesFetcherSucceedsOnValidChannel() {
  const result = await fetchChannelIndex('alpha', {
    fetchImpl: makeFetchImpl([
      {
        url: channelIndexUrl('alpha'),
        responded: JSON.stringify(VALID_CHANNEL_WIRE),
        status: 200,
      },
    ]),
  });
  const value = expectOk(result, 'channel fetcher should succeed');
  assertEqual(value.channel, 'alpha', 'channel in result');
  assertEqual(value.release.trancheId, 'codex-tranche-2-5', 'trancheId in result');
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
        responded: JSON.stringify({
          schema_version: 'v9',
          channel: 'alpha',
          release: null,
          signature: null,
        }),
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
    'rc' as unknown as Sd16ChannelIndexFile['channel']
  );
  expectFailure(result, 'unsupported-channel', 'unsupported channel');
}

async function verifiesManifestFetcherSucceedsOnValidManifest() {
  const result = await fetchUpdateManifest(MANIFEST_URL, {
    fetchImpl: makeFetchImpl([
      {
        url: MANIFEST_URL,
        responded: JSON.stringify(VALID_MANIFEST_WIRE),
        status: 200,
      },
    ]),
  });
  const value = expectOk(result, 'manifest fetcher should succeed');
  assertEqual(value.artifact.version, '0.1.0', 'manifest version preserved');
}

async function verifiesManifestFetcherRejectsInvalidShape() {
  const result = await fetchUpdateManifest(MANIFEST_URL, {
    fetchImpl: makeFetchImpl([
      {
        url: MANIFEST_URL,
        responded: JSON.stringify({
          schema_version: 'v1',
          channel: 'alpha',
          artifact: null,
          signature: null,
        }),
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
  // fetchUpdateManifest only ever asks for the URL the index already named.
  const updateIndexCalls: string[] = [];
  const manifestCalls: string[] = [];
  const routingFetch: FetchLike = async (input) => {
    const url = typeof input === 'string' ? input : String(input);
    if (url.startsWith('https://raw.githubusercontent.com/electricm0nk/codex/update-index/')) {
      updateIndexCalls.push(url);
      return {
        ok: true,
        status: 200,
        text: async () => JSON.stringify(VALID_CHANNEL_WIRE),
      };
    }
    manifestCalls.push(url);
    return {
      ok: true,
      status: 200,
      text: async () => JSON.stringify(VALID_MANIFEST_WIRE),
    };
  };

  const channelResult = await fetchChannelIndex('alpha', { fetchImpl: routingFetch });
  const channelValue = expectOk(channelResult, 'channel fetcher ok in fan-out test');
  const manifestResult = await fetchUpdateManifest(channelValue.release.manifestUrl, {
    fetchImpl: routingFetch,
  });
  expectOk(manifestResult, 'manifest fetcher ok in fan-out test');

  assertEqual(updateIndexCalls.length, 1, 'exactly one update-index call');
  assertEqual(manifestCalls.length, 1, 'exactly one manifest call');
  assertEqual(
    manifestCalls[0],
    MANIFEST_URL,
    'manifest call resolved via the index.manifest_url'
  );
  // Guard against GitHub Releases API scanning creeping in.
  assert(
    !updateIndexCalls.some((u) => u.includes('/releases/')),
    'discovery must not call the GitHub Releases API'
  );
  assert(
    !manifestCalls.some((u) => u.includes('/repos/')),
    'manifest fetch must not call GitHub repo APIs'
  );
}

async function main() {
  // pure-validator tests
  verifiesChannelIndexUrlShape();
  verifiesValidChannelIndexShapeAccepted();
  verifiesInvalidChannelIndexRejectsBadSchemaVersion();
  verifiesInvalidChannelIndexRejectsMissingManifestUrl();
  verifiesInvalidChannelIndexRejectsUnknownChannel();
  verifiesInvalidChannelIndexRejectsBadSignatureType();
  verifiesValidManifestShapeAccepted();
  verifiesInvalidManifestRejectsBadEligibility();
  verifiesInvalidManifestRejectsEmptyArtifactSha256();
  verifiesInvalidManifestRejectsMissingArtifactSha256();

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

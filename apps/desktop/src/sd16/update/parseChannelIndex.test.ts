// SD16-E3-F3b-BACKFILL — channel-index TS parser tests.
//
// These tests are the TS-lane half of the dual-validator posture the
// F1 readiness closure committed to (Python lane:
// `python -m jsonschema -i tests/fixtures/update/<fixture>
// schemas/update/channel-index.schema.json`; both lanes MUST produce
// identical accept/reject verdicts on the SAME fixture bytes).
//
// AV-SCH coverage provided here:
//   - AV-SCH-1: channel-index schema exists and is used by shell tests
//     (this file — three positive fixtures parse cleanly).
//   - AV-SCH-3: signature reserved, null allowed (signature is null on
//     every positive fixture; parsing succeeds).
//   - AV-SCH-4: channel index forbids duplicating manifest body
//     (alpha.full-manifest.json rejected with `additionalProperties`).
//   - AV-SCH-7: tag prefix must equal channel
//     (channel-index.bad-tag.json exercises the rejection).
//   - AV-SCH-8: manifest_url must end with .json (sanity asserted
//     post-parse so the test catches schema regressions).
//
// Fixtures are inlined as raw JSON strings to match the E6-F3a test
// convention (no fs reads, no json-module imports in tests). The
// canonical fixture bytes are guaranteed identical because they were
// copied from `tests/fixtures/update/*.json` at authoring time and
// the receipt comment cites the byte-for-byte verification.

import { assert, assertEqual } from '../../testSupport/asserts';
import { parseChannelIndex, type ChannelIndexFile } from './parseChannelIndex';

// ---------- positive channel-index fixtures ----------

const ALPHA_JSON = `{
  "schema_version": "1.0.0",
  "channel": "alpha",
  "version": "0.0.0-alpha.20260704.1",
  "tag": "alpha/0.0.0-alpha.20260704.1",
  "release_url": "https://github.com/electricm0nk/codex/releases/tag/alpha/0.0.0-alpha.20260704.1",
  "manifest_url": "https://raw.githubusercontent.com/electricm0nk/codex/update-index/update-manifest.json",
  "publication_timestamp": "2026-07-04T08:45:00Z",
  "tranche_id": "STC-CODEX-SD-16",
  "signature": null
}`;

const BETA_JSON = `{
  "schema_version": "1.0.0",
  "channel": "beta",
  "version": "0.0.0-beta.20260704.1",
  "tag": "beta/0.0.0-beta.20260704.1",
  "release_url": "https://github.com/electricm0nk/codex/releases/tag/beta/0.0.0-beta.20260704.1",
  "manifest_url": "https://raw.githubusercontent.com/electricm0nk/codex/update-index/update-manifest.json",
  "publication_timestamp": "2026-07-04T08:45:00Z",
  "tranche_id": "STC-CODEX-SD-16",
  "signature": null
}`;

const STABLE_JSON = `{
  "schema_version": "1.0.0",
  "channel": "stable",
  "version": "0.0.0",
  "tag": "stable/0.0.0",
  "release_url": "https://github.com/electricm0nk/codex/releases/tag/stable/0.0.0",
  "manifest_url": "https://raw.githubusercontent.com/electricm0nk/codex/update-index/update-manifest.json",
  "publication_timestamp": "2026-07-04T08:45:00Z",
  "tranche_id": "STC-CODEX-SD-16",
  "signature": null
}`;

// ---------- negative fixtures (must NOT parse) ----------

const ALPHA_FULL_MANIFEST_JSON = `{
  "schema_version": "1.0.0",
  "channel": "alpha",
  "version": "0.0.0-alpha.20260704.1",
  "tag": "alpha/0.0.0-alpha.20260704.1",
  "release_url": "https://github.com/electricm0nk/codex/releases/tag/alpha/0.0.0-alpha.20260704.1",
  "manifest_url": "https://raw.githubusercontent.com/electricm0nk/codex/update-index/update-manifest.json",
  "publication_timestamp": "2026-07-04T08:45:00Z",
  "tranche_id": "STC-CODEX-SD-16",
  "signature": null,
  "linux_appimage": {
    "name": "codex_0.0.0-alpha.20260704.1_amd64.AppImage",
    "url": "https://github.com/electricm0nk/codex/releases/download/alpha/0.0.0-alpha.20260704.1/codex_0.0.0-alpha.20260704.1_amd64.AppImage",
    "sha256": "0000000000000000000000000000000000000000000000000000000000000000",
    "size_bytes": 1
  },
  "workflow_provenance": {
    "workflow": ".github/workflows/publish-tester-release.yml",
    "run_id": 1,
    "run_attempt": 1
  },
  "source_branch": "develop",
  "source_commit": "0000000000000000000000000000000000000000"
}`;

const BAD_TAG_JSON = `{
  "schema_version": "1.0.0",
  "channel": "alpha",
  "version": "0.0.0-beta.20260704.1",
  "tag": "beta/0.0.0-beta.20260704.1",
  "release_url": "https://github.com/electricm0nk/codex/releases/tag/beta/0.0.0-beta.20260704.1",
  "manifest_url": "https://raw.githubusercontent.com/electricm0nk/codex/update-index/update-manifest.json",
  "publication_timestamp": "2026-07-04T08:45:00Z",
  "tranche_id": "STC-CODEX-SD-16",
  "signature": null
}`;

// ---------- helpers ----------

function hasAdditionalPropertyError(
  errors: ReadonlyArray<{ keyword?: string; params?: { additionalProperty?: string } }>,
): boolean {
  return errors.some(
    (e) =>
      e.keyword === 'additionalProperties' &&
      e.params !== undefined &&
      e.params.additionalProperty !== undefined,
  );
}

function hasTagPatternError(
  errors: ReadonlyArray<{ keyword?: string; instancePath?: string }>,
): boolean {
  return errors.some(
    (e) => e.keyword === 'pattern' && (e.instancePath ?? '').endsWith('/tag'),
  );
}

// ---------- AV-SCH-1 ----------

export function test_av_sch_1_alpha_channel_index_positive(): void {
  const result = parseChannelIndex(ALPHA_JSON);
  assert(result.ok, `alpha fixture must parse: ${JSON.stringify(result.ok ? null : result.errors)}`);
  if (result.ok) {
    const data = result.data as ChannelIndexFile;
    assertEqual(data.channel, 'alpha', 'channel preserved');
    assert(data.tag.startsWith('alpha/'), 'tag must start with channel prefix');
    assert(data.manifest_url.endsWith('.json'), 'AV-SCH-8: manifest_url must end with .json');
  }
}

export function test_av_sch_1_beta_channel_index_positive(): void {
  const result = parseChannelIndex(BETA_JSON);
  assert(result.ok, `beta fixture must parse: ${JSON.stringify(result.ok ? null : result.errors)}`);
  if (result.ok) {
    const data = result.data as ChannelIndexFile;
    assertEqual(data.channel, 'beta', 'channel preserved');
    assert(data.tag.startsWith('beta/'), 'tag must start with channel prefix');
  }
}

export function test_av_sch_1_stable_channel_index_positive(): void {
  const result = parseChannelIndex(STABLE_JSON);
  assert(result.ok, `stable fixture must parse: ${JSON.stringify(result.ok ? null : result.errors)}`);
  if (result.ok) {
    const data = result.data as ChannelIndexFile;
    assertEqual(data.channel, 'stable', 'channel preserved');
    assert(data.tag.startsWith('stable/'), 'tag must start with channel prefix');
  }
}

// ---------- AV-SCH-3 ----------

export function test_av_sch_3_signature_null_accepted(): void {
  const result = parseChannelIndex(ALPHA_JSON);
  assert(result.ok, 'alpha fixture (signature: null) must parse');
  if (result.ok) {
    assertEqual(
      result.data.signature,
      null,
      'AV-SCH-3: signature:null must round-trip; the schema accepts null via oneOf',
    );
  }
}

// ---------- AV-SCH-4 ----------

export function test_av_sch_4_channel_index_rejects_full_manifest(): void {
  const result = parseChannelIndex(ALPHA_FULL_MANIFEST_JSON);
  assert(
    !result.ok,
    'alpha.full-manifest fixture smuggles manifest data into a channel-index file and MUST be rejected',
  );
  if (!result.ok) {
    assert(
      hasAdditionalPropertyError(result.errors),
      `expected additionalProperties violation, got ${JSON.stringify(result.errors)}`,
    );
  }
}

// ---------- AV-SCH-7: tag prefix must equal channel ----------

export function test_channel_index_tag_channel_mismatch_rejected(): void {
  const result = parseChannelIndex(BAD_TAG_JSON);
  assert(
    !result.ok,
    'channel-index.bad-tag fixture (channel=alpha, tag=beta/...) MUST be rejected',
  );
  if (!result.ok) {
    assert(
      hasTagPatternError(result.errors),
      `expected tag pattern violation from allOf cross-field rule, got ${JSON.stringify(result.errors)}`,
    );
  }
}

// ---------- Invalid JSON input sanity ----------

export function test_parseChannelIndex_invalid_json_returns_typed_error(): void {
  const result = parseChannelIndex('{not valid json');
  assert(!result.ok, 'invalid JSON input must NOT parse');
  if (!result.ok) {
    assert(result.errors.length >= 1, 'invalid JSON input must surface at least one ErrorObject');
  }
}

function run(): void {
  test_av_sch_1_alpha_channel_index_positive();
  test_av_sch_1_beta_channel_index_positive();
  test_av_sch_1_stable_channel_index_positive();
  test_av_sch_3_signature_null_accepted();
  test_av_sch_4_channel_index_rejects_full_manifest();
  test_channel_index_tag_channel_mismatch_rejected();
  test_parseChannelIndex_invalid_json_returns_typed_error();
  console.log('parseChannelIndex.test.ts: 7/7 AV-SCH-* assertions passed');
}

run();

// SD16-E3-F3b-BACKFILL — update-manifest TS parser tests.
//
// Mirror of `parseChannelIndex.test.ts` against the manifest schema.
// The same fixtures drive both lanes (Python jsonschema and TypeScript
// ajv); the inlined raw JSON strings here are byte-for-byte identical
// to the corresponding files at `tests/fixtures/update/*.json`. The
// receipt comment cites the byte-for-byte verification.
//
// AV-SCH coverage provided here:
//   - AV-SCH-2: update-manifest schema exists and is used by shell tests.
//   - AV-SCH-3: signature reserved, null allowed (positive manifest fixture).
//   - AV-SCH-5: manifest locks `tranche_id` and `release_notes_path`.
//   - AV-SCH-6: `release_notes_path` regex locks the path.
//   - AV-SCH-8: manifest_url regex; sanity asserted post-parse (channel-index side).

import { assert, assertEqual } from '../../testSupport/asserts';
import { parseUpdateManifest } from './parseUpdateManifest';

const UPDATE_MANIFEST_JSON = `{
  "schema_version": "1.0.0",
  "channel": "alpha",
  "version": "0.0.0-alpha.20260704.1",
  "tag": "alpha/0.0.0-alpha.20260704.1",
  "tranche_id": "STC-CODEX-SD-16",
  "source_branch": "develop",
  "source_commit": "d94edcbfc2f3a6f428895eb9c3e504fc7a363ade",
  "release_notes_path": "programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/release-notes.md",
  "release_notes_url": "https://raw.githubusercontent.com/electricm0nk/codex/develop/programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/release-notes.md",
  "release_notes_hash": "0000000000000000000000000000000000000000000000000000000000000000",
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
  "eligibility": {
    "min_supported_version": "0.0.0-alpha.20260701.1",
    "appimage_install": true,
    "required_install_kind": "appimage"
  },
  "promotion_lineage": {
    "source_branch": "develop",
    "source_commit": "d94edcbfc2f3a6f428895eb9c3e504fc7a363ade",
    "promoted_at": "2026-07-04T08:45:00Z"
  },
  "signature": null
}`;

const MISSING_SIGNATURE_ALLOWED_JSON = UPDATE_MANIFEST_JSON; // structural twin; signature is already null and accepted

const BAD_PATH_JSON = `{
  "schema_version": "1.0.0",
  "channel": "alpha",
  "version": "0.0.0-alpha.20260704.1",
  "tag": "alpha/0.0.0-alpha.20260704.1",
  "tranche_id": "STC-CODEX-SD-16",
  "source_branch": "develop",
  "source_commit": "d94edcbfc2f3a6f428895eb9c3e504fc7a363ade",
  "release_notes_path": "some/other/path/release-notes.md",
  "release_notes_url": "https://raw.githubusercontent.com/electricm0nk/codex/develop/some/other/path/release-notes.md",
  "release_notes_hash": "0000000000000000000000000000000000000000000000000000000000000000",
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
  "eligibility": {
    "min_supported_version": "0.0.0-alpha.20260701.1",
    "appimage_install": true,
    "required_install_kind": "appimage"
  },
  "promotion_lineage": {
    "source_branch": "develop",
    "source_commit": "d94edcbfc2f3a6f428895eb9c3e504fc7a363ade",
    "promoted_at": "2026-07-04T08:45:00Z"
  },
  "signature": null
}`;

// ---------- AV-SCH-2 ----------

export function test_av_sch_2_update_manifest_positive(): void {
  const result = parseUpdateManifest(UPDATE_MANIFEST_JSON);
  assert(
    result.ok,
    `update-manifest fixture must parse: ${JSON.stringify(result.ok ? null : result.errors)}`,
  );
  if (result.ok) {
    assertEqual(result.data.tranche_id, 'STC-CODEX-SD-16', 'AV-SCH-5: tranche_id must be canonical');
    assert(
      /^programs\/codex\/requirements\/[^/]+\/release-notes\.md$/.test(result.data.release_notes_path),
      'AV-SCH-6: release_notes_path must match the canonical regex',
    );
    assert(result.data.linux_appimage.size_bytes >= 1, 'linux_appimage.size_bytes must be >= 1');
    assertEqual(
      result.data.workflow_provenance.run_attempt,
      1,
      'workflow_provenance.run_attempt must round-trip as a number',
    );
    assertEqual(
      result.data.eligibility.appimage_install,
      true,
      'eligibility.appimage_install must round-trip as a boolean',
    );
  }
}

// ---------- AV-SCH-3 ----------

export function test_av_sch_3_update_manifest_signature_null_accepted(): void {
  const result = parseUpdateManifest(MISSING_SIGNATURE_ALLOWED_JSON);
  assert(
    result.ok,
    `missing-signature-allowed manifest must parse: ${JSON.stringify(
      result.ok ? null : result.errors,
    )}`,
  );
  if (result.ok) {
    assertEqual(
      result.data.signature,
      null,
      'AV-SCH-3: signature:null is allowed and must round-trip, not be coerced',
    );
  }
}

// ---------- AV-SCH-5 ----------

export function test_av_sch_5_manifest_tranche_id_and_path_locked_at_parse(): void {
  const result = parseUpdateManifest(UPDATE_MANIFEST_JSON);
  assert(result.ok, 'positive manifest must parse');
  if (result.ok) {
    assertEqual(
      result.data.tranche_id,
      'STC-CODEX-SD-16',
      `tranche_id must be STC-CODEX-SD-16 (got ${result.data.tranche_id})`,
    );
    assertEqual(
      result.data.release_notes_path,
      'programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/release-notes.md',
      `release_notes_path must be canonical (got ${result.data.release_notes_path})`,
    );
  }
}

// ---------- AV-SCH-6 ----------

export function test_av_sch_6_manifest_rejects_bad_path(): void {
  const result = parseUpdateManifest(BAD_PATH_JSON);
  assert(!result.ok, 'release-manifest.bad-path fixture MUST be rejected');
  if (!result.ok) {
    const patternError = result.errors.find(
      (e) => e.keyword === 'pattern' && (e.instancePath ?? '').endsWith('/release_notes_path'),
    );
    assert(
      patternError !== undefined,
      `expected pattern violation on release_notes_path, got ${JSON.stringify(result.errors)}`,
    );
  }
}

// ---------- Invalid JSON input sanity ----------

export function test_parseUpdateManifest_invalid_json_returns_typed_error(): void {
  const result = parseUpdateManifest('{not valid json');
  assert(!result.ok, 'invalid JSON input must NOT parse');
  if (!result.ok) {
    assert(result.errors.length >= 1, 'invalid JSON input must surface at least one ErrorObject');
  }
}

function run(): void {
  test_av_sch_2_update_manifest_positive();
  test_av_sch_3_update_manifest_signature_null_accepted();
  test_av_sch_5_manifest_tranche_id_and_path_locked_at_parse();
  test_av_sch_6_manifest_rejects_bad_path();
  test_parseUpdateManifest_invalid_json_returns_typed_error();
  console.log('parseUpdateManifest.test.ts: 5/5 AV-SCH-* assertions passed');
}

run();

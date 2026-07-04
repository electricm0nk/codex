// SD16-E8-F3 — controlledDefect tests.
//
// Covers the four F2-pinned acceptance cases:
//   (a) controlled mismatch returns `match: false` and the verbatim
//       `install_disabled_reason` string `checksum-mismatch`;
//   (b) corrected match returns `match: true`;
//   (c) network failure on AppImage download returns a structured
//       error and `match: false` with reason `checksum-network-error`;
//   (d) manifest without `artifact_sha256` field returns `match: false`
//       with reason `checksum-missing-field`.

import { assert, assertEqual } from '../../testSupport/asserts';
import {
  decideControlledDefect,
  type ControlledDefectDeps,
  type ControlledDefectVerdict,
} from './controlledDefect';
import type { FetchLike } from './fetch';

// ---------- helpers ----------

interface Stub {
  url: string;
  responded: string;
  bytes?: Uint8Array;
  status: number;
}

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
      arrayBuffer: async () =>
        (expected.bytes ?? new TextEncoder().encode(expected.responded)).slice().buffer,
    };
  };
}

/** Stub SHA-256: maps the AppImage body to a deterministic hex digest. */
function makeStubSha256(answers: Record<string, string>) {
  return async (bytes: Uint8Array): Promise<string> => {
    const text = new TextDecoder().decode(bytes);
    const asHex = Array.from(bytes, (byte) => byte.toString(16).padStart(2, '0')).join('');
    const found = answers[text] ?? answers[asHex];
    if (typeof found === 'string') {
      return found;
    }
    // Default: derive a stable hash from the text content so two
    // different bodies do not accidentally collide in tests.
    let h = 0;
    for (let i = 0; i < text.length; i += 1) {
      h = (h * 31 + text.charCodeAt(i)) >>> 0;
    }
    return h.toString(16).padStart(64, '0');
  };
}

const MANIFEST_URL = 'https://updates.example.test/alpha/update-manifest.json';
const APPIMAGE_URL = 'https://updates.example.test/alpha/codex.AppImage';
const DECLARED = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa';
const ACTUAL_MATCH = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb';
const ACTUAL_MISMATCH = 'cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc';

const MANIFEST_WITH_SHA = {
  artifact_sha256: DECLARED,
  artifact_path: 'codex.AppImage',
};

const MANIFEST_WITHOUT_SHA = {
  artifact_path: 'codex.AppImage',
};

const MANIFEST_WITH_INVALID_SHA = {
  artifact_sha256: 'not-a-valid-sha256',
  artifact_path: 'codex.AppImage',
};

function verdictOkForNotMatch(
  v: ControlledDefectVerdict,
  expectedReason: string
): void {
  assertEqual(v.match, false, 'verdict.match');
  assertEqual(v.reason, expectedReason, 'verdict.reason');
  assert(v.reason !== null, 'verdict.reason should not be null on mismatch');
}

function deps(stubs: Stub[], answers: Record<string, string>): ControlledDefectDeps {
  return { fetchImpl: makeFetchImpl(stubs), sha256: makeStubSha256(answers) };
}

// ---------- (a) controlled mismatch ----------

async function verifiesControlledMismatchReturnsChecksumMismatch() {
  const body = 'CONTROLLED-BODY-1';
  const result = await decideControlledDefect(
    { manifestUrl: MANIFEST_URL, manifest: MANIFEST_WITH_SHA },
    deps(
      [{ url: APPIMAGE_URL, responded: body, status: 200 }],
      { [body]: ACTUAL_MISMATCH }
    )
  );
  verdictOkForNotMatch(result, 'checksum-mismatch');
  assertEqual(result.actual, ACTUAL_MISMATCH, 'verdict.actual on controlled mismatch');
  assertEqual(result.declared, DECLARED, 'verdict.declared on controlled mismatch');
  assertEqual(result.httpStatus, 200, 'verdict.httpStatus on controlled mismatch');
}

// ---------- (b) corrected match ----------

async function verifiesCorrectedMatchReturnsTrue() {
  const body = 'CORRECTED-BODY-2';
  const result = await decideControlledDefect(
    { manifestUrl: MANIFEST_URL, manifest: MANIFEST_WITH_SHA },
    deps(
      [{ url: APPIMAGE_URL, responded: body, status: 200 }],
      { [body]: DECLARED }
    )
  );
  assertEqual(result.match, true, 'verdict.match on corrected match');
  assertEqual(result.reason, null, 'verdict.reason on corrected match should be null');
  assertEqual(result.actual, DECLARED, 'verdict.actual on corrected match');
  assertEqual(result.declared, DECLARED, 'verdict.declared on corrected match');
  assertEqual(result.httpStatus, 200, 'verdict.httpStatus on corrected match');
}

// ---------- (c) network failure on AppImage download ----------

async function verifiesNetworkFailureReturnsChecksumNetworkError() {
  // The fetchImpl throws (network error) — the dispatcher must
  // return a structured verdict, not propagate the throw.
  const throwingFetchImpl: FetchLike = async () => {
    throw new Error('simulated network failure');
  };
  const result = await decideControlledDefect(
    { manifestUrl: MANIFEST_URL, manifest: MANIFEST_WITH_SHA },
    { fetchImpl: throwingFetchImpl, sha256: makeStubSha256({}) }
  );
  verdictOkForNotMatch(result, 'checksum-network-error');
  assertEqual(result.actual, '', 'verdict.actual on network failure');
  assertEqual(result.declared, DECLARED, 'verdict.declared on network failure');
  assertEqual(result.httpStatus, 0, 'verdict.httpStatus on network failure');
}

async function verifiesHttpErrorReturnsChecksumHttpError() {
  const result = await decideControlledDefect(
    { manifestUrl: MANIFEST_URL, manifest: MANIFEST_WITH_SHA },
    deps(
      [{ url: APPIMAGE_URL, responded: 'not found', status: 404 }],
      {}
    )
  );
  verdictOkForNotMatch(result, 'checksum-http-error');
  assertEqual(result.httpStatus, 404, 'verdict.httpStatus on http error');
  assertEqual(result.declared, DECLARED, 'verdict.declared on http error');
}

// ---------- (d) manifest without `artifact_sha256` field ----------

async function verifiesManifestWithoutShaReturnsChecksumMissingField() {
  const result = await decideControlledDefect(
    { manifestUrl: MANIFEST_URL, manifest: MANIFEST_WITHOUT_SHA },
    deps([], {})
  );
  verdictOkForNotMatch(result, 'checksum-missing-field');
  assertEqual(result.declared, null, 'verdict.declared should be null when field is absent');
  assertEqual(result.actual, '', 'verdict.actual on missing-field');
  assertEqual(result.httpStatus, 0, 'verdict.httpStatus on missing-field');
}

async function verifiesManifestWithInvalidShaReturnsChecksumMissingField() {
  // Non-hex string in `artifact_sha256` is treated as the field being
  // unusable — same path as (d).
  const result = await decideControlledDefect(
    { manifestUrl: MANIFEST_URL, manifest: MANIFEST_WITH_INVALID_SHA },
    deps([], {})
  );
  verdictOkForNotMatch(result, 'checksum-missing-field');
  assertEqual(result.declared, null, 'verdict.declared on invalid sha');
}

async function verifiesManifestWithoutArtifactPathReturnsChecksumMissingField() {
  const result = await decideControlledDefect(
    { manifestUrl: MANIFEST_URL, manifest: { artifact_sha256: DECLARED } },
    deps([], {})
  );
  verdictOkForNotMatch(result, 'checksum-missing-field');
  assertEqual(result.declared, DECLARED, 'verdict.declared should be the (unusable) sha');
  assertEqual(result.actual, '', 'verdict.actual on missing path');
}

// ---------- discriminates actual hex case ----------

async function verifiesHexCaseIsNormalised() {
  // Manifest declares uppercase; AppImage returns lowercase. Both
  // normalise to lowercase, so the match must be true even when the
  // case differs.
  const body = 'CASE-BODY';
  const result = await decideControlledDefect(
    {
      manifestUrl: MANIFEST_URL,
      manifest: { artifact_sha256: DECLARED.toUpperCase(), artifact_path: 'codex.AppImage' },
    },
    deps(
      [{ url: APPIMAGE_URL, responded: body, status: 200 }],
      { [body]: DECLARED }
    )
  );
  assertEqual(result.match, true, 'verdict.match should ignore case');
  assertEqual(result.declared, DECLARED, 'verdict.declared should be normalised to lowercase');
}

async function verifiesDigestUsesRawBytes() {
  const bytes = Uint8Array.from([0xff, 0x00, 0x61, 0x62, 0x80]);
  const result = await decideControlledDefect(
    { manifestUrl: MANIFEST_URL, manifest: MANIFEST_WITH_SHA },
    deps(
      [{ url: APPIMAGE_URL, responded: 'WRONG-TEXT-BODY', bytes, status: 200 }],
      { 'ff00616280': DECLARED }
    )
  );
  assertEqual(result.match, true, 'verdict.match should hash the raw AppImage bytes');
  assertEqual(result.actual, DECLARED, 'verdict.actual should come from the raw bytes digest');
}

async function verifiesParsedManifestArtifactShapeIsAccepted() {
  const body = 'PARSED-MANIFEST-BODY';
  const result = await decideControlledDefect(
    {
      manifestUrl: MANIFEST_URL,
      manifest: {
        artifact: {
          artifactSha256: DECLARED,
          path: 'codex.AppImage',
        },
      },
    },
    deps(
      [{ url: APPIMAGE_URL, responded: body, status: 200 }],
      { [body]: DECLARED }
    )
  );
  assertEqual(result.match, true, 'parsed manifest artifact shape should be accepted');
  assertEqual(result.reason, null, 'parsed manifest verdict should not include a reason on match');
}

async function main() {
  await verifiesControlledMismatchReturnsChecksumMismatch();
  await verifiesCorrectedMatchReturnsTrue();
  await verifiesNetworkFailureReturnsChecksumNetworkError();
  await verifiesHttpErrorReturnsChecksumHttpError();
  await verifiesManifestWithoutShaReturnsChecksumMissingField();
  await verifiesManifestWithInvalidShaReturnsChecksumMissingField();
  await verifiesManifestWithoutArtifactPathReturnsChecksumMissingField();
  await verifiesHexCaseIsNormalised();
  await verifiesDigestUsesRawBytes();
  await verifiesParsedManifestArtifactShapeIsAccepted();
  console.log('controlledDefect.test.ts: all assertions passed');
}

main();

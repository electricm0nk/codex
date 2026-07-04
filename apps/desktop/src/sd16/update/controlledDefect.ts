// SD16-E8-F3 — controlled-defect dispatcher.
//
// This module is the in-tranche controlled-defect SHA-256 surface for
// the SD-16 E8 final-acceptance run. It pairs with the F1-pinned
// "primary scenario: SHA-256 pre-replacement mismatch" controlled
// defect and is the seam that lets the shell prove the E7 staged
// transaction's pre-replacement SHA-256 guard is discriminating, not
// broken.
//
// Design tenets (carry into the wider E8 surface):
//   - Pure and testable: every network call goes through a `fetchImpl`
//     indirection (we reuse the E6 FetchLike type). Tests inject a stub
//     so the network is never touched under `npm test`. The actual
//     AppImage bytes stream through the same `fetchImpl` and the
//     SHA-256 is computed via Web Crypto (`crypto.subtle.digest`) in
//     the browser/Tauri webview and `node:crypto.webcrypto.subtle`
//     under `npm test` (Node 22 exposes the same global).
//   - Fail closed with a discriminated result: any non-200 fetch, any
//     fetchImpl error, any manifest parse failure, any manifest without
//     `artifact_sha256`, or any network failure on the AppImage
//     download returns a typed result with a verbatim `reason` string
//     that maps 1:1 to an `install_disabled_reason` slot the shell UI
//     can surface. NEVER throw across the module boundary — the F3a
//     fetch module is the reference for that contract.
//   - The two-state pair contract: a controlled (mismatching) alpha
//     and a corrected (matching) alpha share the same code path; the
//     only thing that changes is whether the AppImage bytes the
//     fetcher streams back match the manifest's `artifact_sha256`.
//     The shell calls the same function for both releases; the
//     function reports `match: true` only on the corrected state. This
//     is what makes the "defect-fixed" claim mechanical, not
//     interpretive.
//   - No reliance on the GitHub Releases API: the manifest URL is the
//     input; the AppImage URL is read from the parsed manifest's
//     `artifact.path` plus the same origin as the manifest. E4 is the
//     release lane; E6 owns discovery; E8-F3 is the verifier.
//   - No secrets, no tokens, no raw full logs in the result. The
//     `reason` strings are the only operator-visible surface; they
//     are literal constants, not user input.

// ---------- shared types ----------

import type { FetchLike } from './fetch';

/**
 * Minimal manifest shape this module needs. We accept a structural
 * subset (not the full E3 schema) so the module does not depend on a
 * JSON-Schema runtime. `artifact_sha256` is the field that must be
 * present; if it is missing or non-hex, the dispatcher reports
 * `checksum-missing-field` per F3 acceptance case (d).
 */
export interface ControlledDefectManifest {
  artifact_sha256?: string;
  artifact_path?: string;
}

export type ControlledDefectReason =
  | 'checksum-mismatch'
  | 'checksum-network-error'
  | 'checksum-missing-field'
  | 'checksum-http-error'
  | 'checksum-invalid-json';

export interface ControlledDefectVerdict {
  /** Whether the AppImage bytes' SHA-256 matches the manifest. */
  match: boolean;
  /** SHA-256 of the AppImage bytes actually streamed back, lowercase hex. */
  actual: string;
  /** SHA-256 declared in the manifest, lowercase hex; null when absent. */
  declared: string | null;
  /**
   * When `match` is false, the verbatim reason string the shell UI
   * surfaces as `install_disabled_reason`. When `match` is true, null.
   */
  reason: null | ControlledDefectReason;
  /**
   * HTTP status of the AppImage fetch when the failure is an HTTP
   * error. 0 when not applicable (e.g. network error or success).
   */
  httpStatus: number;
}

export interface ControlledDefectInput {
  manifestUrl: string;
  manifest: ControlledDefectManifest;
  /**
   * Base URL used to resolve a relative `artifact_path` against the
   * manifest URL. When omitted, the manifest URL itself is the base.
   */
  appimageBaseUrl?: string;
}

export interface ControlledDefectDeps {
  fetchImpl: FetchLike;
  /**
   * SHA-256 of the given bytes, lowercase hex. Injected so tests can
   * stub the digest without touching the Web Crypto shim.
   */
  sha256: (bytes: Uint8Array) => Promise<string>;
}

const HEX_64_RE = /^[0-9a-f]{64}$/;

function normaliseHex(s: string): string {
  return s.trim().toLowerCase();
}

function isHex64(s: string): boolean {
  return HEX_64_RE.test(s);
}

function resolveAppImageUrl(
  manifest: ControlledDefectManifest,
  manifestUrl: string,
  appimageBaseUrl: string | undefined
): string | null {
  const path = manifest.artifact_path;
  if (typeof path !== 'string' || path.length === 0) {
    return null;
  }
  // Absolute URL: take as-is.
  if (/^https?:\/\//i.test(path)) {
    return path;
  }
  // Relative path: resolve against the base URL. We do this with the
  // URL constructor in environments that have it (browser / Node 22+);
  // for the rare environment without URL, fall back to a literal
  // base+path join so the seam is testable.
  const base = appimageBaseUrl ?? manifestUrl;
  try {
    return new URL(path, base).toString();
  } catch {
    const trimmedBase = base.replace(/\/[^/]*$/, '');
    return `${trimmedBase}/${path}`;
  }
}

/**
 * Compute the controlled-defect verdict.
 *
 * Pure function over its inputs and injected deps. Returns a
 * discriminated `ControlledDefectVerdict`; never throws.
 */
export async function decideControlledDefect(
  input: ControlledDefectInput,
  deps: ControlledDefectDeps
): Promise<ControlledDefectVerdict> {
  const declaredRaw = input.manifest.artifact_sha256;
  if (typeof declaredRaw !== 'string' || !isHex64(normaliseHex(declaredRaw))) {
    return {
      match: false,
      actual: '',
      declared: null,
      reason: 'checksum-missing-field',
      httpStatus: 0,
    };
  }
  const declared = normaliseHex(declaredRaw);

  const appimageUrl = resolveAppImageUrl(input.manifest, input.manifestUrl, input.appimageBaseUrl);
  if (appimageUrl === null) {
    return {
      match: false,
      actual: '',
      declared,
      reason: 'checksum-missing-field',
      httpStatus: 0,
    };
  }

  let response: { ok: boolean; status: number; text(): Promise<string> };
  try {
    response = await deps.fetchImpl(appimageUrl);
  } catch {
    return {
      match: false,
      actual: '',
      declared,
      reason: 'checksum-network-error',
      httpStatus: 0,
    };
  }
  if (!response.ok) {
    return {
      match: false,
      actual: '',
      declared,
      reason: 'checksum-http-error',
      httpStatus: response.status,
    };
  }

  let actual: string;
  try {
    const body = await response.text();
    const bytes = new TextEncoder().encode(body);
    actual = await deps.sha256(bytes);
  } catch {
    return {
      match: false,
      actual: '',
      declared,
      reason: 'checksum-network-error',
      httpStatus: response.status,
    };
  }
  actual = normaliseHex(actual);

  if (actual === declared) {
    return {
      match: true,
      actual,
      declared,
      reason: null,
      httpStatus: response.status,
    };
  }
  return {
    match: false,
    actual,
    declared,
    reason: 'checksum-mismatch',
    httpStatus: response.status,
  };
}

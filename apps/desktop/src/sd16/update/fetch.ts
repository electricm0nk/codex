// Channel index + update manifest fetch with canonical
// JSON-Schema validation.
//
// This module is the shell-side update discovery path for Tranche 2.5. It
// fetches (1) the channel index pointer file from the protected
// `update-index` branch on origin, and (2) the update manifest that index
// points at (the F6 CORS-friendly mirror, also on `update-index`), then
// validates both against the canonical contracts in `schemas/update/`:
//   - `channel-index.schema.json`
//   - `update-manifest.schema.json`
// It does NOT issue any `fetch("/repos/.../releases")` style GitHub
// Releases API call (E6 is the shell's discovery surface; the release lane
// — E4 — publishes the release and writes the channel index).
//
// Validation is delegated to the E3-backfill ajv parsers
// (`parseChannelIndex.ts` / `parseUpdateManifest.ts`), which compile the
// canonical schemas directly. This module was originally shipped with a
// self-contained pure-TS shape validator mirroring a pre-E3 draft of the
// contract; that placeholder is gone — the schemas are the single source
// of truth, and the E4 release lane validates against the same documents
// before publishing, so producer and consumer cannot drift apart again.
//
// Design tenets (carry into the wider E6 surface):
//   - Fail closed: any non-200 response, malformed JSON, or schema
//     violation returns a discriminated `FetchFailure` rather than
//     throwing. UI/Install gating uses the failure reason verbatim to
//     render a deterministic reason to the user.
//   - Pure and testable: every fetch goes through a `fetchImpl` indirection
//     defaulting to the platform `fetch`. Tests inject a stub so the network
//     is never touched under `npm test`.
//   - Explicit Result type: no exceptions on the surface. Internal failures
//     during JSON parsing become typed results, not thrown `SyntaxError`s.
//   - No reliance on Node-only globals: this module is also valid in the
//     Tauri webview context (the browser `fetch` is a superset of what we
//     need; if a host has no `fetch`, the injected `fetchImpl` is the
//     escape hatch).

import type { ErrorObject } from 'ajv/dist/2020';
import {
  parseChannelIndex,
  type ChannelIndexFile,
  type ChannelLabel,
} from './parseChannelIndex';
import { parseUpdateManifest, type UpdateManifestFile } from './parseUpdateManifest';

// ---------- channel + manifest field contracts ----------
//
// The wire types are the canonical snake_case shapes pinned by the ajv
// parsers. Downstream slices (F3b eligibility + diagnostics, F3c UI
// integration, the SD-11 boundary) consume these re-exported aliases so
// the schema contract has exactly one TS surface.

export type ChannelLabel = ChannelLabel;
export type ChannelIndexFile = ChannelIndexFile;
export type UpdateManifestFile = UpdateManifestFile;

// ---------- discriminated results ----------

export type FetchFailure =
  | { kind: 'http-error'; status: number; url: string }
  | { kind: 'invalid-json'; reason: string; url: string }
  | { kind: 'invalid-channel-index'; reason: string; url: string }
  | { kind: 'invalid-manifest'; reason: string; url: string }
  | { kind: 'unsupported-channel'; channel: string };

export type FetchResult<T> =
  | { ok: true; value: T }
  | { ok: false; failure: FetchFailure };

// ---------- network indirection ----------

export type FetchLike = (
  input: string,
  init?: { method?: string; headers?: Record<string, string> }
) => Promise<{
  ok: boolean;
  status: number;
  text(): Promise<string>;
  arrayBuffer?(): Promise<ArrayBuffer>;
}>;

const defaultFetchImpl: FetchLike = async (input, init) => {
  // Runtime guard: Tauri webview always has `fetch`; Node 22 also exposes
  // the global. If neither is available, the caller can inject `fetchImpl`
  // to keep discovery testable.
  const f = (globalThis as { fetch?: typeof fetch }).fetch;
  if (typeof f !== 'function') {
    throw new Error(
      'sd16/update/fetch: no global fetch available; pass `fetchImpl` explicitly'
    );
  }
  const res = await f(input, init);
  return {
    ok: res.ok,
    status: res.status,
    text: () => Promise.resolve(res.text()),
    arrayBuffer: () => Promise.resolve(res.arrayBuffer()),
  };
};

// ---------- channel index ----------

const RAW_BASE_URL =
  'https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels';

/**
 * Resolve the raw `update-index` URL for a given channel pointer. Kept
 * exported so tests can pin the URL shape without depending on the fetcher.
 */
export function channelIndexUrl(channel: ChannelLabel): string {
  return `${RAW_BASE_URL}/${channel}.json`;
}

function isChannelLabel(value: unknown): value is ChannelLabel {
  return value === 'alpha' || value === 'beta' || value === 'stable';
}

function isNonEmptyString(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0;
}

/**
 * Render the first ajv error into the deterministic human-readable reason
 * UI surfaces show verbatim (disabled-reason badges, check-failed detail).
 */
function renderSchemaErrors(errors: ReadonlyArray<ErrorObject>): string {
  const first = errors[0];
  if (!first) {
    return 'schema validation failed';
  }
  const where = first.instancePath === '' ? '(root)' : first.instancePath;
  return `${where} ${first.message ?? 'schema validation failed'}`;
}

/** Extract the synthetic json-parse error the ajv parsers emit on bad JSON. */
function jsonParseError(errors: ReadonlyArray<ErrorObject>): ErrorObject | undefined {
  return errors.find((e) => e.keyword === 'json-parse');
}

/**
 * Validate raw channel-index response text against the canonical
 * `channel-index.schema.json`. Returns a human-readable reason on failure
 * (used by UI surfaces for deterministic disabled-reason badges).
 */
export function validateChannelIndexShape(
  rawText: string,
  atUrl: string
): FetchResult<ChannelIndexFile> {
  const parsed = parseChannelIndex(rawText);
  if (!parsed.ok) {
    const parseFailure = jsonParseError(parsed.errors);
    if (parseFailure) {
      return {
        ok: false,
        failure: {
          kind: 'invalid-json',
          reason: parseFailure.message ?? 'invalid JSON',
          url: atUrl,
        },
      };
    }
    return {
      ok: false,
      failure: {
        kind: 'invalid-channel-index',
        reason: `channel-index.schema.json: ${renderSchemaErrors(parsed.errors)} at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  return { ok: true, value: parsed.data };
}

/**
 * Fetch the channel index pointer for the given channel from the protected
 * `update-index` branch. Returns a discriminated result; never throws.
 */
export async function fetchChannelIndex(
  channel: ChannelLabel,
  options: { fetchImpl?: FetchLike; baseUrl?: string } = {}
): Promise<FetchResult<ChannelIndexFile>> {
  if (!isChannelLabel(channel)) {
    return {
      ok: false,
      failure: {
        kind: 'unsupported-channel',
        channel: String(channel),
      },
    };
  }
  const url =
    options.baseUrl !== undefined
      ? `${options.baseUrl.replace(/\/$/, '')}/${channel}.json`
      : channelIndexUrl(channel);
  return fetchAndValidate<ChannelIndexFile>(url, validateChannelIndexShape, options.fetchImpl);
}

/**
 * Fetch the update manifest at the given URL (typically the
 * `manifest_url` resolved from the channel index). Returns a discriminated
 * result; never throws. The URL MUST be exactly what the validated channel
 * index already proved trustworthy — callers must not pass arbitrary URLs.
 */
export async function fetchUpdateManifest(
  manifestUrl: string,
  options: { fetchImpl?: FetchLike } = {}
): Promise<FetchResult<UpdateManifestFile>> {
  if (!isNonEmptyString(manifestUrl)) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-manifest',
        reason: 'manifest_url must be a non-empty string',
        url: manifestUrl,
      },
    };
  }
  return fetchAndValidate<UpdateManifestFile>(manifestUrl, validateManifestShape, options.fetchImpl);
}

// ---------- update manifest ----------

/**
 * Validate raw update-manifest response text against the canonical
 * `update-manifest.schema.json`.
 */
export function validateManifestShape(
  rawText: string,
  atUrl: string
): FetchResult<UpdateManifestFile> {
  const parsed = parseUpdateManifest(rawText);
  if (!parsed.ok) {
    const parseFailure = jsonParseError(parsed.errors);
    if (parseFailure) {
      return {
        ok: false,
        failure: {
          kind: 'invalid-json',
          reason: parseFailure.message ?? 'invalid JSON',
          url: atUrl,
        },
      };
    }
    return {
      ok: false,
      failure: {
        kind: 'invalid-manifest',
        reason: `update-manifest.schema.json: ${renderSchemaErrors(parsed.errors)} at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  return { ok: true, value: parsed.data };
}

// ---------- release-notes body fetch (E3.12) ----------
//
// The channel-index/manifest fetch above proves *identity* (schema-valid,
// signed-shape wire contracts); it never fetches the release-notes prose
// body itself — `release_notes_url`/`release_notes_hash` are manifest
// fields, not fetched content. This is the distinct fetch path E3.12 adds:
// given a manifest's `release_notes_url` + `release_notes_hash`, fetch the
// body text and verify it hashes to the pinned value before ever handing
// it to the UI. A body that fails to hash-match is exactly as untrustworthy
// as one that 404s — both fail closed rather than rendering unverified
// prose.

export type ReleaseNotesFetchFailure =
  | { kind: 'http-error'; status: number; url: string }
  | { kind: 'hash-mismatch'; url: string; expected: string; actual: string };

export type ReleaseNotesFetchResult =
  | { ok: true; value: { body: string } }
  | { ok: false; failure: ReleaseNotesFetchFailure };

/** SHA-256 of the given text, lowercase hex. Uses the platform Web Crypto API. */
async function defaultSha256Text(text: string): Promise<string> {
  const digest = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(text));
  return Array.from(new Uint8Array(digest))
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('');
}

function normaliseHex(s: string): string {
  return s.trim().toLowerCase();
}

/**
 * Fetch the release-notes body at `releaseNotesUrl` and verify it hashes to
 * `releaseNotesHash` (both sourced from an already schema-validated
 * `UpdateManifestFile`). Returns a discriminated result; never throws.
 * A hash mismatch fails closed exactly like an HTTP error — the caller must
 * never surface a body it could not verify.
 */
export async function fetchReleaseNotesBody(
  releaseNotesUrl: string,
  releaseNotesHash: string,
  options: { fetchImpl?: FetchLike; sha256?: (text: string) => Promise<string> } = {}
): Promise<ReleaseNotesFetchResult> {
  const fetchImpl = options.fetchImpl ?? defaultFetchImpl;
  const sha256 = options.sha256 ?? defaultSha256Text;

  let res: Awaited<ReturnType<FetchLike>>;
  try {
    res = await fetchImpl(releaseNotesUrl, { method: 'GET' });
  } catch {
    return { ok: false, failure: { kind: 'http-error', status: 0, url: releaseNotesUrl } };
  }
  if (!res.ok) {
    return {
      ok: false,
      failure: { kind: 'http-error', status: res.status, url: releaseNotesUrl },
    };
  }

  const body = await res.text();
  const actual = await sha256(body);
  const expected = normaliseHex(releaseNotesHash);
  if (actual !== expected) {
    return {
      ok: false,
      failure: { kind: 'hash-mismatch', url: releaseNotesUrl, expected, actual },
    };
  }
  return { ok: true, value: { body } };
}

// ---------- shared HTTP + JSON + validate plumbing ----------

async function fetchAndValidate<T>(
  url: string,
  validator: (rawText: string, atUrl: string) => FetchResult<T>,
  fetchImpl: FetchLike = defaultFetchImpl
): Promise<FetchResult<T>> {
  let res: Awaited<ReturnType<FetchLike>>;
  try {
    res = await fetchImpl(url, {
      method: 'GET',
      headers: { Accept: 'application/json' },
    });
  } catch (err) {
    return {
      ok: false,
      failure: {
        kind: 'http-error',
        status: 0,
        url,
      },
    };
  }
  if (!res.ok) {
    return {
      ok: false,
      failure: {
        kind: 'http-error',
        status: res.status,
        url,
      },
    };
  }
  const text = await res.text();
  return validator(text, url);
}

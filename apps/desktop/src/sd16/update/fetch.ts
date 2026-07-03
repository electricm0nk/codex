// SD16-E6-F3a — channel index + update manifest fetch with schema-shape validation.
//
// This module is the FIRST slice of the shell-side update discovery path for
// Tranche 2.5. It is intentionally narrow: it fetches (1) the channel index
// pointer file from the protected `update-index` branch on origin, and (2) the
// update manifest that index points at, then validates that the JSON returned
// matches the field-level contract documented in
// `programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/`
// (the README and the E6 readiness closure). It does NOT issue any
// `fetch("/repos/.../releases")` style GitHub Releases API call (E6 is the
// shell's discovery surface; the release lane — E4 — publishes the release
// and writes the channel index).
//
// The schema authoring and the JSON-Schema -> validator wiring live on the
// E3 epic and have not yet landed in origin/develop at the time this slice
// was authored. To keep the contract honest and to give downstream slices
// (F3b eligibility + diagnostics, F3c UI integration) a stable typed
// surface to build on, this module ships a self-contained pure-TS shape
// validator that mirrors the documented field-level contract. When E3 lands
// the actual JSON Schemas, this module is the single seam to swap to ajv
// without touching F3b/F3c — they consume the parsed types, not the
// validation internals.
//
// Design tenets (carry into the wider E6 surface):
//   - Fail closed: any non-200 response, malformed JSON, or shape mismatch
//     returns a discriminated `FetchFailure` rather than throwing. UI/Install
//     gating uses the failure reason verbatim to render a deterministic
//     reason to the user.
//   - Pure and testable: every fetch goes through a `fetchImpl` indirection
//     defaulting to the platform `fetch`. Tests inject a stub so the network
//     is never touched under `npm test`.
//   - Explicit Result type: no exceptions on the surface. Internal failures
//     during JSON parsing become typed results, not thrown `SyntaxError`s.
//   - No `any` and no runtime JSON Schema engine — only structural checks
//     against typed field shapes. Reserved signature field is allowed to be
//     null or absent (signing is deferred per Non-Goals).
//   - No reliance on Node-only globals: this module is also valid in the
//     Tauri webview context (the browser `fetch` is a superset of what we
//     need; if a host has no `fetch`, the injected `fetchImpl` is the
//     escape hatch).

// ---------- channel + manifest field contracts ----------

export type Sd16ChannelLabel = 'alpha' | 'beta' | 'stable';

export interface Sd16ChannelReleasePointer {
  tag: string;
  publishedAt: string; // ISO-8601 UTC
  manifestUrl: string;
  trancheId: string;
  notesUrl?: string;
}

export interface Sd16ChannelIndexFile {
  schemaVersion: 'v1';
  channel: Sd16ChannelLabel;
  release: Sd16ChannelReleasePointer;
  signature: null | unknown; // reserved; signing deferred per Non-Goals
}

export interface Sd16UpdateManifestArtifact {
  releaseId: string;
  version: string;
  buildLabel: string;
  commitOrProvenanceHandle: string;
  publishedAt: string; // ISO-8601 UTC
  artifactSha256: string;
  path: string;
}

export interface Sd16UpdateManifestFile {
  schemaVersion: 'v1';
  channel: Sd16ChannelLabel;
  artifact: Sd16UpdateManifestArtifact;
  eligibility:
    | 'automatic'
    | 'manual-only'
    | 'unsupported'
    | 'withdrawn'
    | 'blocked';
  notesUrl?: string;
  signature: null | unknown; // reserved; signing deferred per Non-Goals
}

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
  };
};

// ---------- channel index ----------

const RAW_BASE_URL =
  'https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels';

/**
 * Resolve the raw `update-index` URL for a given channel pointer. Kept
 * exported so tests can pin the URL shape without depending on the fetcher.
 */
export function channelIndexUrl(channel: Sd16ChannelLabel): string {
  return `${RAW_BASE_URL}/${channel}.json`;
}

const SUPPORTED_CHANNELS: ReadonlySet<Sd16ChannelLabel> = new Set<Sd16ChannelLabel>([
  'alpha',
  'beta',
  'stable',
]);

function isSd16ChannelLabel(value: unknown): value is Sd16ChannelLabel {
  return (
    typeof value === 'string' &&
    (SUPPORTED_CHANNELS as Set<string>).has(value)
  );
}

function isNonEmptyString(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0;
}

/**
 * Validate the structural shape of a fetched channel index. Returns a
 * human-readable reason on failure (used by UI surfaces for deterministic
 * disabled-reason badges). Mirrors the documented channel-index shape from
 * the E6 readiness closure + the E4 channel-index implementation, minus
 * the JSON Schema engine that E3 has not yet wired.
 */
export function validateChannelIndexShape(raw: unknown, atUrl: string): FetchResult<Sd16ChannelIndexFile> {
  if (raw === null || typeof raw !== 'object' || Array.isArray(raw)) {
    const got = raw === null ? 'null' : Array.isArray(raw) ? 'array' : typeof raw;
    return {
      ok: false,
      failure: {
        kind: 'invalid-channel-index',
        reason: `expected a JSON object at ${atUrl}, got ${got}`,
        url: atUrl,
      },
    };
  }
  const obj = raw as Record<string, unknown>;

  if (obj.schema_version !== 'v1') {
    return {
      ok: false,
      failure: {
        kind: 'invalid-channel-index',
        reason: `schema_version must be the literal string "v1" at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  if (!isSd16ChannelLabel(obj.channel)) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-channel-index',
        reason: `channel must be one of "alpha" | "beta" | "stable" at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  if (obj.release === null || typeof obj.release !== 'object' || Array.isArray(obj.release)) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-channel-index',
        reason: `release must be an object at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  const rel = obj.release as Record<string, unknown>;
  if (!isNonEmptyString(rel.tag)) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-channel-index',
        reason: `release.tag must be a non-empty string at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  if (!isNonEmptyString(rel.published_at)) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-channel-index',
        reason: `release.published_at must be a non-empty ISO-8601 string at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  if (!isNonEmptyString(rel.manifest_url)) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-channel-index',
        reason: `release.manifest_url must be a non-empty string at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  if (!isNonEmptyString(rel.tranche_id)) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-channel-index',
        reason: `release.tranche_id must be a non-empty string at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  if (
    rel.notes_url !== undefined &&
    typeof rel.notes_url !== 'string'
  ) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-channel-index',
        reason: `release.notes_url, if present, must be a string at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  // signature is reserved (signing deferred); must be null or absent.
  if (
    obj.signature !== undefined &&
    obj.signature !== null &&
    typeof obj.signature !== 'object'
  ) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-channel-index',
        reason: `signature, if present, must be null or an object (signing reserved) at ${atUrl}`,
        url: atUrl,
      },
    };
  }

  const parsed: Sd16ChannelIndexFile = {
    schemaVersion: 'v1',
    channel: obj.channel,
    release: {
      tag: rel.tag,
      publishedAt: rel.published_at,
      manifestUrl: rel.manifest_url,
      trancheId: rel.tranche_id,
      ...(rel.notes_url !== undefined ? { notesUrl: rel.notes_url } : {}),
    },
    signature: (obj.signature ?? null) as null,
  };
  return { ok: true, value: parsed };
}

/**
 * Fetch the channel index pointer for the given channel from the protected
 * `update-index` branch. Returns a discriminated result; never throws.
 */
export async function fetchChannelIndex(
  channel: Sd16ChannelLabel,
  options: { fetchImpl?: FetchLike; baseUrl?: string } = {}
): Promise<FetchResult<Sd16ChannelIndexFile>> {
  if (!isSd16ChannelLabel(channel)) {
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
  return fetchAndValidate<Sd16ChannelIndexFile>(url, validateChannelIndexShape, options.fetchImpl);
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
): Promise<FetchResult<Sd16UpdateManifestFile>> {
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
  return fetchAndValidate<Sd16UpdateManifestFile>(manifestUrl, validateManifestShape, options.fetchImpl);
}

// ---------- update manifest ----------

/**
 * Validate the structural shape of a fetched update manifest. Mirrors the
 * documented update-manifest shape from the E6 readiness closure (release
 * identity, eligibility, optional notes, reserved signature).
 */
export function validateManifestShape(raw: unknown, atUrl: string): FetchResult<Sd16UpdateManifestFile> {
  if (raw === null || typeof raw !== 'object') {
    return {
      ok: false,
      failure: {
        kind: 'invalid-manifest',
        reason: `expected a JSON object at ${atUrl}, got ${raw === null ? 'null' : typeof raw}`,
        url: atUrl,
      },
    };
  }
  const obj = raw as Record<string, unknown>;

  if (obj.schema_version !== 'v1') {
    return {
      ok: false,
      failure: {
        kind: 'invalid-manifest',
        reason: `schema_version must be the literal string "v1" at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  if (!isSd16ChannelLabel(obj.channel)) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-manifest',
        reason: `channel must be one of "alpha" | "beta" | "stable" at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  const allowedEligibility = new Set<string>([
    'automatic',
    'manual-only',
    'unsupported',
    'withdrawn',
    'blocked',
  ]);
  if (typeof obj.eligibility !== 'string' || !allowedEligibility.has(obj.eligibility)) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-manifest',
        reason: `eligibility must be one of "automatic" | "manual-only" | "unsupported" | "withdrawn" | "blocked" at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  if (obj.artifact === null || typeof obj.artifact !== 'object') {
    return {
      ok: false,
      failure: {
        kind: 'invalid-manifest',
        reason: `artifact must be an object at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  const art = obj.artifact as Record<string, unknown>;
  const artifactStrings: Record<string, string> = {};
  for (const key of ['release_id', 'version', 'build_label', 'commit_or_provenance_handle', 'published_at', 'artifact_sha256', 'path']) {
    const v = art[key];
    if (!isNonEmptyString(v)) {
      return {
        ok: false,
        failure: {
          kind: 'invalid-manifest',
          reason: `artifact.${key} must be a non-empty string at ${atUrl}`,
          url: atUrl,
        },
      };
    }
    artifactStrings[key] = v;
  }
  if (
    obj.notes_url !== undefined &&
    typeof obj.notes_url !== 'string'
  ) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-manifest',
        reason: `notes_url, if present, must be a string at ${atUrl}`,
        url: atUrl,
      },
    };
  }
  if (
    obj.signature !== undefined &&
    obj.signature !== null &&
    typeof obj.signature !== 'object'
  ) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-manifest',
        reason: `signature, if present, must be null or an object (signing reserved) at ${atUrl}`,
        url: atUrl,
      },
    };
  }

  const parsed: Sd16UpdateManifestFile = {
    schemaVersion: 'v1',
    channel: obj.channel,
    eligibility: obj.eligibility as Sd16UpdateManifestFile['eligibility'],
    artifact: {
      releaseId: artifactStrings.release_id,
      version: artifactStrings.version,
      buildLabel: artifactStrings.build_label,
      commitOrProvenanceHandle: artifactStrings.commit_or_provenance_handle,
      publishedAt: artifactStrings.published_at,
      artifactSha256: artifactStrings.artifact_sha256,
      path: artifactStrings.path,
    },
    ...(obj.notes_url !== undefined ? { notesUrl: obj.notes_url } : {}),
    signature: (obj.signature ?? null) as null,
  };
  return { ok: true, value: parsed };
}

// ---------- shared HTTP + JSON + validate plumbing ----------

async function fetchAndValidate<T>(
  url: string,
  validator: (raw: unknown, atUrl: string) => FetchResult<T>,
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
  let raw: unknown;
  try {
    raw = JSON.parse(text);
  } catch (err) {
    return {
      ok: false,
      failure: {
        kind: 'invalid-json',
        reason: err instanceof Error ? err.message : String(err),
        url,
      },
    };
  }
  return validator(raw, url);
}

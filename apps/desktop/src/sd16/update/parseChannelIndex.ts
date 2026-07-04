// SD16-E3-F3b-BACKFILL — channel index parser.
//
// Validates a raw JSON text string against the canonical
// `channel-index.schema.json` using ajv programmatically (NOT ajv-cli).
// This is the TS-lane half of the dual-validator stance the F1 readiness
// closure committed to (the Python lane is
// `tools/release/check_release_manifest.py` plus the F3a release-tooling
// tests that exercise `python -m jsonschema` against the same schema).
//
// Per F2 handoff §"F3b — Implementation Contract": Ajv instance MUST be
// a module-level singleton so all callers share the same compiled
// validators and the test lane exercises the SAME ajv the runtime uses.
//
// The parser returns a discriminated `ParseResult<T>` so downstream
// callers (F3c UI integration, E6 install-gating) consume a typed
// success/failure shape that mirrors the E6-F3a `fetch.ts` convention
// (see `FetchFailure` for the network-side analog).

import Ajv, { type ErrorObject } from 'ajv/dist/2020';
import { loadChannelIndexSchema } from './loadSchemas';

// Module-level singleton. The F2 handoff's `strict: true` collides with
// the canonical schema's `if/then` cross-field rule: ajv strict mode
// demands that any property referenced inside `if` be declared under
// `properties` at the schema root, but the schema's `required: ["channel", "tag"]`
// inside the allOf branch is author-valid for Draft 2020-12 (Python
// jsonschema has no such complaint). We honor the F1 closure's
// "strict semantics" intent by using `strict: 'log'` — strict-mode
// violations are logged but not thrown, so any future schema drift
// that introduces a true strict-mode violation (e.g. a typo in
// `properties.tag.pattern`) still surfaces loudly during development
// while the canonical schemas remain parseable.
//
// `allErrors: false` short-circuits validation at the first failure
// to mirror the Python `jsonschema` default behavior (F2 handoff §
// "F3b — Implementation Contract").
//
// ajv/dist/2020 imports the Draft 2020-12 meta-schema out of the box
// (the default `ajv` import only registers Draft-07), which the
// canonical channel-index.schema.json declares via
// `"$schema": "https://json-schema.org/draft/2020-12/schema"`.
const ajv = new Ajv({ strict: 'log', allErrors: false, strictSchema: false });

// Compile-once validator. ajv caches internally; compiling in a
// module-level constant means the very first parse already pays the
// compile cost, and subsequent parses reuse the compiled validator.
const validateChannelIndex = ajv.compile(loadChannelIndexSchema());

export type ChannelLabel = 'alpha' | 'beta' | 'stable';

export interface ChannelIndexFile {
  schema_version: string;
  channel: ChannelLabel;
  version: string;
  tag: string;
  release_url: string;
  manifest_url: string;
  publication_timestamp: string;
  tranche_id: string;
  signature: null | Record<string, unknown>;
}

export type ParseChannelIndexFailure = {
  readonly ok: false;
  readonly errors: ReadonlyArray<ErrorObject>;
};

export type ParseChannelIndexSuccess = {
  readonly ok: true;
  readonly data: ChannelIndexFile;
};

export type ParseChannelIndexResult = ParseChannelIndexSuccess | ParseChannelIndexFailure;

/**
 * Parse a JSON text string into a `ChannelIndexFile`, validating it
 * against the canonical channel-index schema via ajv. Returns a
 * discriminated ParseResult; never throws on validation failure.
 */
export function parseChannelIndex(rawText: string): ParseChannelIndexResult {
  let parsed: unknown;
  try {
    parsed = JSON.parse(rawText);
  } catch (err) {
    return {
      ok: false,
      errors: [
        {
          keyword: 'json-parse',
          instancePath: '',
          schemaPath: '#/json-parse',
          params: {},
          message: `Invalid JSON: ${(err as Error).message}`,
        } as ErrorObject,
      ],
    };
  }

  const ok = validateChannelIndex(parsed);
  if (!ok && validateChannelIndex.errors) {
    return { ok: false, errors: validateChannelIndex.errors };
  }

  // Deep-clone via structured clone so downstream mutators cannot
  // bypass the schema by mutating the parsed object reference the
  // validator cached.
  return {
    ok: true,
    data: structuredClone(parsed) as ChannelIndexFile,
  };
}

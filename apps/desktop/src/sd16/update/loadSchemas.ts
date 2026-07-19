// Schema loader.
//
// Loads the two JSON Schema documents authored by the F3a slice:
//   - `schemas/update/channel-index.schema.json`
//   - `schemas/update/update-manifest.schema.json`
//
// Loader contract (F2 handoff §"F3b — Implementation Contract"):
//
//   - Default-exports `loadChannelIndexSchema()` and
//     `loadUpdateManifestSchema()` returning the parsed JSON Schema
//     documents, importing each file as a JSON module via the
//     `resolveJsonModule` TypeScript option (already enabled in
//     `apps/desktop/tsconfig.json`).
//   - The import paths assume the desktop app's tsconfig resolves the
//     repo-root `schemas/` directory. `apps/desktop/tsconfig.json`'s
//     `include` key carries the additive entries `"../../schemas/update/*.json"`
//     and `"../../tests/fixtures/update/**/*.json"` (the only additive
//     tsconfig changes F3b makes).
//   - The schemas-guard fixture runs once at module evaluation to ensure
//     the resolved modules are the canonical JSON documents. Any future
//     divergence surfaces at first import, not at first parse failure.

import channelIndexSchema from '../../../../../schemas/update/channel-index.schema.json';
import updateManifestSchema from '../../../../../schemas/update/update-manifest.schema.json';

// JSON-module imports produce a typed `JsonSchema` shape. Ajv's
// `Schema` type is structurally compatible (it is the same JSON object)
// so we re-export with ajv's expected type in the parser modules.
export interface LoadedJsonSchema {
  readonly $id?: string;
  readonly $schema?: string;
  readonly title?: string;
  readonly description?: string;
  readonly type?: string;
  readonly properties?: Record<string, LoadedJsonSchema>;
  readonly required?: ReadonlyArray<string>;
  readonly additionalProperties?: boolean | LoadedJsonSchema;
  readonly allOf?: ReadonlyArray<LoadedJsonSchema>;
  readonly oneOf?: ReadonlyArray<LoadedJsonSchema>;
  readonly anyOf?: ReadonlyArray<LoadedJsonSchema>;
  readonly [key: string]: unknown;
}

export function loadChannelIndexSchema(): LoadedJsonSchema {
  return channelIndexSchema as LoadedJsonSchema;
}

export function loadUpdateManifestSchema(): LoadedJsonSchema {
  return updateManifestSchema as LoadedJsonSchema;
}

import { assertCanonicalSchemasLoaded } from './__fixtures__/schemas-guard';

assertCanonicalSchemasLoaded();

export { assertCanonicalSchemasLoaded };

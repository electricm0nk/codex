// Schemas guard.
//
// Runtime defensive assertion that the JSON Schema files imported by
// `loadSchemas.ts` are the canonical documents hosted at the repo paths
// `schemas/update/channel-index.schema.json` and
// `schemas/update/update-manifest.schema.json`.
//
// Why this exists:
//
//   The F1 readiness closure §"Validator decision — confirmed" commits to
//   dual-validator posture: both lanes load the SAME schema files. If a
//   future refactor rewrites one schema file but a divergent copy is left
//   at the import path, the dual-validator premise collapses silently:
//   ajv and jsonschema would both pass on a bundle that no longer reflects
//   the bundle's declared contract.
//
//   This guard runs at the first import of `loadSchemas.ts` and throws
//   loudly if the resolved module exports are not the canonical JSON. It
//   catches (a) accidental shadowing by an intermediate build/cache, and
//   (b) any vendored copy of these schemas that drifts from the canonical
//   source.
//
//   The guard is intentionally minimal: it inspects `$id`, `title`,
//   `version`, and key required top-level keys for each schema. It does
//   NOT execute the validator itself — that is `parseChannelIndex.ts` and
//   `parseUpdateManifest.ts`.

import { loadChannelIndexSchema, loadUpdateManifestSchema } from '../loadSchemas';

interface CanonicalChannelIndexSchemaShape {
  readonly expectedId: string;
  readonly expectedTitle: string;
  // The array-of-accepted-consts shape is uniform
  // across both schemas. Channel-index stays at the historical
  // ['1.0.0']; update-manifest carries ['1.0.0', '1.1.0'] because the
  // bump was additive-only.
  readonly expectedVersionConsts: ReadonlyArray<string>;
  readonly requiredTopLevelKeys: ReadonlyArray<string>;
}

interface CanonicalUpdateManifestSchemaShape {
  readonly expectedId: string;
  readonly expectedTitle: string;
  readonly expectedVersionConsts: ReadonlyArray<string>;
  readonly requiredTopLevelKeys: ReadonlyArray<string>;
}

const CHANNEL_INDEX_CANON: CanonicalChannelIndexSchemaShape = {
  expectedId: 'https://codex.electricm0nk/schemas/update/channel-index.schema.json',
  expectedTitle: 'Codex Channel Index',
  expectedVersionConsts: ['1.0.0'],
  requiredTopLevelKeys: [
    'schema_version',
    'channel',
    'version',
    'tag',
    'release_url',
    'manifest_url',
    'publication_timestamp',
    'tranche_id',
  ],
};

const UPDATE_MANIFEST_CANON: CanonicalUpdateManifestSchemaShape = {
  expectedId: 'https://codex.electricm0nk/schemas/update/update-manifest.schema.json',
  expectedTitle: 'Codex Update Manifest',
  expectedVersionConsts: ['1.0.0', '1.1.0'],
  requiredTopLevelKeys: [
    'schema_version',
    'channel',
    'version',
    'tag',
    'tranche_id',
    'source_branch',
    'source_commit',
    'release_notes_path',
    'release_notes_url',
    'release_notes_hash',
    'linux_appimage',
    'workflow_provenance',
    'eligibility',
    'promotion_lineage',
  ],
};

function assertCanonical(
  label: string,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  schema: any,
  expectation: CanonicalChannelIndexSchemaShape | CanonicalUpdateManifestSchemaShape,
): void {
  if (!schema || typeof schema !== 'object') {
    throw new Error(
      `[sd16/update/__fixtures__/schemas-guard] ${label} schema import did not resolve to a JSON object. ` +
        `The schema file path may be missing or the import shadowed by a stub.`,
    );
  }
  if (schema.$id !== expectation.expectedId) {
    throw new Error(
      `[sd16/update/__fixtures__/schemas-guard] ${label} schema $id "${schema.$id}" does not match canonical "${
        expectation.expectedId
      }". The import path may be pointing at a divergent or vendored copy.`,
    );
  }
  if (schema.title !== expectation.expectedTitle) {
    throw new Error(
      `[sd16/update/__fixtures__/schemas-guard] ${label} schema title "${schema.title}" does not match canonical "${
        expectation.expectedTitle
      }". The import path may be pointing at a stale or rewritten copy.`,
    );
  }
  const versionProperty = schema.properties?.schema_version;
  if (!versionProperty) {
    throw new Error(
      `[sd16/update/__fixtures__/schemas-guard] ${label} schema is missing properties.schema_version. The canonical contract must declare schema_version with an enum / oneOf / const.`,
    );
  }
  // The pin accepts either a single `const` (channel-index shape)
  // or a `oneOf` array of `const` entries (update-manifest v1.1.0+
  // additive pattern). Extract the accepted const values uniformly.
  const acceptedConsts: ReadonlyArray<string> = (() => {
    if (typeof versionProperty.const === 'string') {
      return [versionProperty.const];
    }
    if (Array.isArray(versionProperty.oneOf)) {
      const out: string[] = [];
      for (const e of versionProperty.oneOf) {
        if (e && typeof e.const === 'string') out.push(e.const);
      }
      return out;
    }
    return [];
  })();
  if (acceptedConsts.length === 0) {
    throw new Error(
      `[sd16/update/__fixtures__/schemas-guard] ${label} schema properties.schema_version has neither a ` +
        `string const nor a oneOf of const entries. Expected: const="..." or oneOf=[{const:"a"}, {const:"b"}].`,
    );
  }
  const expectedConsts = expectation.expectedVersionConsts;
  for (const expected of expectedConsts) {
    if (!acceptedConsts.includes(expected)) {
      throw new Error(
        `[sd16/update/__fixtures__/schemas-guard] ${label} schema version const mismatch. ` +
          `Expected const="${expected}" to be accepted by properties.schema_version; ` +
          `got accepted consts=[${acceptedConsts.map((s) => `"${s}"`).join(',')}] from ${JSON.stringify(versionProperty)}. ` +
          `The canonical schema contract has bumped without a guard refresh.`,
      );
    }
  }
  const required: ReadonlyArray<string> = Array.isArray(schema.required) ? schema.required : [];
  const missing = expectation.requiredTopLevelKeys.filter((k) => !required.includes(k));
  if (missing.length > 0) {
    throw new Error(
      `[sd16/update/__fixtures__/schemas-guard] ${label} schema is missing required top-level keys: ${missing.join(
        ', ',
      )}. The canonical contract has extended without a guard refresh.`,
    );
  }
  if (schema.additionalProperties !== false) {
    throw new Error(
      `[sd16/update/__fixtures__/schemas-guard] ${label} schema must declare additionalProperties: false at root ` +
        `(this is the AV-SCH-4 contract that forbids manifest data on the channel index and vice versa). ` +
        `Got additionalProperties=${JSON.stringify(schema.additionalProperties)}.`,
    );
  }
}

/**
 * Synchronously verify both schemas on module load. Called by
 * `loadSchemas.ts` after it has resolved its two imports so that any
 * divergence surfaces immediately at first import, not at first user
 * visible parse failure.
 */
export function assertCanonicalSchemasLoaded(): void {
  const channelIndexSchema = loadChannelIndexSchema();
  const updateManifestSchema = loadUpdateManifestSchema();
  assertCanonical('channel-index', channelIndexSchema, CHANNEL_INDEX_CANON);
  assertCanonical('update-manifest', updateManifestSchema, UPDATE_MANIFEST_CANON);
}

// SD16-E3-F3b-BACKFILL — update manifest parser.
//
// Validates a raw JSON text string against the canonical
// `update-manifest.schema.json` using the same module-level ajv
// singleton that `parseChannelIndex.ts` uses. See that module for the
// design rationale; this module is its twin against the manifest schema.
//
// Sub-object shapes (linux_appimage, workflow_provenance, eligibility,
// promotion_lineage) are pinned here at the type level so downstream
// UI code (E6 install-gating, F3c release-notes display) consumes a
// typed shape rather than re-discovering fields on every read.

import type { ErrorObject } from 'ajv/dist/2020';
import Ajv from 'ajv/dist/2020';
import { loadUpdateManifestSchema } from './loadSchemas';

// Fresh Ajv instance so the manifest validator compiles independently
// of the channel-index validator. ajv requires a fresh instance per
// schema set when schema ids overlap, and keeping the two parsers
// independent makes single-mode tests (parse just the channel index
// without the manifest registered) trivial.
//
// `strict: 'log'` mirrors the channel-index parser: F1 closure's strict
// semantics intent is honored without forcing ajv to reject the
// canonical schema's `if/then` cross-field rule. `allErrors: false`
// short-circuits at the first failure for parity with Python jsonschema.
//
// ajv/dist/2020 imports the Draft 2020-12 meta-schema out of the box
// (the default `ajv` import only registers Draft-07), which the
// canonical update-manifest.schema.json declares via
// `"$schema": "https://json-schema.org/draft/2020-12/schema"`.
const manifestAjv = new Ajv({ strict: 'log', allErrors: false, strictSchema: false });
const validateUpdateManifest = manifestAjv.compile(loadUpdateManifestSchema());

export interface LinuxAppImageArtifact {
  name: string;
  url: string;
  sha256: string;
  size_bytes: number;
}

// SD16-F-WINDOWS: parallel to LinuxAppImageArtifact, for the Windows MSI
// the publish-tester-release-windows job produces. Optional because
// schema 1.0.0 manifests and Linux-only builds do not set it. Same
// required-key shape as LinuxAppImageArtifact so the discovery path can
// be generalized later without contract churn.
export interface WindowsMsiArtifact {
  name: string;
  url: string;
  sha256: string;
  size_bytes: number;
}

// SD16-F-WINDOWS: parallel to LinuxAppImageArtifact + WindowsMsiArtifact,
// for the macOS DMG the publish-tester-release-macos job produces.
// Optional and same-shape; optional because schema 1.0.0 manifests and
// Linux-only / Windows-only builds do not set it. Three-platform
// discoverability later happens in the shell by iterating over whichever
// blocks the schema validates.
export interface MacosDmgArtifact {
  name: string;
  url: string;
  sha256: string;
  size_bytes: number;
}

export interface WorkflowProvenance {
  workflow: string;
  run_id: number | string;
  run_attempt: number;
}

export interface Eligibility {
  min_supported_version: string;
  appimage_install: boolean;
  required_install_kind: 'appimage' | 'dev' | 'any';
}

export interface PromotionLineage {
  source_branch: string;
  source_commit: string;
  promoted_at: string;
}

export interface UpdateManifestFile {
  schema_version: string;
  channel: 'alpha' | 'beta' | 'stable';
  version: string;
  tag: string;
  tranche_id: string;
  source_branch: string;
  source_commit: string;
  release_notes_path: string;
  release_notes_url: string;
  release_notes_hash: string;
  linux_appimage: LinuxAppImageArtifact;
  windows_msi?: WindowsMsiArtifact;
  macos_dmg?: MacosDmgArtifact;
  workflow_provenance: WorkflowProvenance;
  eligibility: Eligibility;
  promotion_lineage: PromotionLineage;
  signature?: null | Record<string, never>;
}

export type ParseUpdateManifestFailure = {
  readonly ok: false;
  readonly errors: ReadonlyArray<ErrorObject>;
};

export type ParseUpdateManifestSuccess = {
  readonly ok: true;
  readonly data: UpdateManifestFile;
};

export type ParseUpdateManifestResult = ParseUpdateManifestSuccess | ParseUpdateManifestFailure;

/**
 * Parse a JSON text string into an `UpdateManifestFile`, validating it
 * against the canonical update-manifest schema. Returns a discriminated
 * ParseResult; never throws on validation failure.
 */
export function parseUpdateManifest(rawText: string): ParseUpdateManifestResult {
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

  const ok = validateUpdateManifest(parsed);
  if (!ok) {
    return { ok: false, errors: validateUpdateManifest.errors ?? [] };
  }

  return {
    ok: true,
    data: structuredClone(parsed) as UpdateManifestFile,
  };
}

/**
 * SD16-E6-F3b — shell update eligibility rules.
 *
 * A pure decision function over installed-state, manifest identity, and fetch
 * outcomes. It never performs I/O; F3a owns fetching/parsing and passes the
 * outcomes in, F3c owns rendering the decision. The rows are evaluated in a
 * fixed order (first match wins) so the `install_disabled_reason` is
 * deterministic — see the F1 closure eligibility decision table.
 */

export type ChannelLabel = 'alpha' | 'beta' | 'stable';

/** Install provenance. `unknown` is the sentinel used before E7 records it. */
export type InstallKind = 'appimage' | 'dev' | 'tarball' | 'unknown';

/** Outcome of a single fetch+validate step. */
export type FetchStatus = 'ok' | 'failed' | 'schema-invalid';

export type EligibilityResult = 'eligible' | 'ineligible' | 'unknown';

export interface EligibilityInput {
  selectedChannel: ChannelLabel;
  manifest: {
    version: string;
    artifact_sha256: string;
  };
  installedState: {
    version: string;
    artifact_sha256: string;
    install_kind: InstallKind;
    managed_executable_path: string | null;
    /** Result of the startup writability probe on the managed path. */
    isManagedPathWritable: boolean;
  };
  fetchOutcomes: {
    indexStatus: FetchStatus;
    manifestStatus: FetchStatus;
    indexSchemaError: string | null;
    manifestSchemaError: string | null;
    indexFetchError: string | null;
    manifestFetchError: string | null;
  };
}

export interface EligibilityDecision {
  result: EligibilityResult;
  install_disabled_reason: string | null;
}

/**
 * Minimal semver-like comparison: split on `.`, compare each segment
 * numerically when both sides are numbers, otherwise lexically. Missing
 * trailing segments compare as `0`, so `1.0` equals `1.0.0`. Returns a
 * negative number when `a < b`, `0` when equal, positive when `a > b`.
 */
export function compareVersions(a: string, b: string): number {
  const aSegments = a.split('.');
  const bSegments = b.split('.');
  const length = Math.max(aSegments.length, bSegments.length);
  for (let i = 0; i < length; i += 1) {
    const aRaw = aSegments[i] ?? '';
    const bRaw = bSegments[i] ?? '';
    const aNum = Number(aRaw);
    const bNum = Number(bRaw);
    if (!Number.isNaN(aNum) && !Number.isNaN(bNum)) {
      if (aNum !== bNum) {
        return aNum < bNum ? -1 : 1;
      }
    } else if (aRaw !== bRaw) {
      return aRaw < bRaw ? -1 : 1;
    }
  }
  return 0;
}

/**
 * Evaluate the eligibility decision table. `unknown` (fetch/validation could not
 * be proven) outranks `ineligible` (proven not-eligible), which outranks
 * `eligible`; the UI must not enable Install unless the result is `eligible`.
 */
export function decideEligibility(input: EligibilityInput): EligibilityDecision {
  const { installedState, manifest, fetchOutcomes } = input;

  // 1..2 — the fetch must succeed before anything downstream can be trusted.
  if (fetchOutcomes.indexStatus !== 'ok') {
    return unknown(`channel index fetch failed: ${fetchOutcomes.indexStatus}`);
  }
  if (fetchOutcomes.manifestStatus !== 'ok') {
    return unknown(`manifest fetch failed: ${fetchOutcomes.manifestStatus}`);
  }

  // 3..4 — schema validation must pass before the parsed values are trusted.
  if (fetchOutcomes.indexSchemaError !== null) {
    return unknown(`channel-index.schema.json: ${fetchOutcomes.indexSchemaError}`);
  }
  if (fetchOutcomes.manifestSchemaError !== null) {
    return unknown(`update-manifest.schema.json: ${fetchOutcomes.manifestSchemaError}`);
  }

  // 5..7 — install provenance / writability gates (fail-closed).
  if (installedState.install_kind === 'dev') {
    return ineligible('dev build is not update-eligible');
  }
  if (installedState.install_kind === 'tarball') {
    return ineligible('tarball install is not update-eligible');
  }
  if (!installedState.isManagedPathWritable) {
    return ineligible('managed executable path is not writable');
  }

  // 8..9 — the manifest must offer something newer and distinct.
  if (compareVersions(manifest.version, installedState.version) <= 0) {
    return ineligible('installed version is at or above manifest version');
  }
  if (manifest.artifact_sha256 === installedState.artifact_sha256) {
    return ineligible('installed artifact hash already matches manifest');
  }

  // 10 — all checks pass; Install is enabled.
  return { result: 'eligible', install_disabled_reason: null };
}

function unknown(reason: string): EligibilityDecision {
  return { result: 'unknown', install_disabled_reason: reason };
}

function ineligible(reason: string): EligibilityDecision {
  return { result: 'ineligible', install_disabled_reason: reason };
}

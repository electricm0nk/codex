import { fetchChannelIndex, fetchUpdateManifest } from '../sd16/update/fetch';
import type {
  UpdateManifestFile,
  FetchFailure,
} from '../sd16/update/fetch';
import { compareVersions } from '../sd16/update/eligibility';
import { formatError, hasTauriRuntime } from './runtime';
import type {
  Sd11TesterChannelLabel,
  Sd11UpdateReleaseTruth,
  Sd11UpdateManifestView,
  Sd11UpdateBuildIdentity,
  Sd11UpdateIntegrityMaterial,
  Sd11ManifestEligibilityState,
  Sd11ReleaseLifecycleState,
} from '../sd11/update/updateActionModel';

export interface Sd11UpdateActionRequest {
  buildVersion: string;
  buildLabel: string;
  platformLabel: string;
  testerChannelLabel: Sd11TesterChannelLabel;
}

export async function loadSd11UpdateAction(
  request: Sd11UpdateActionRequest
): Promise<Sd11UpdateReleaseTruth> {
  if (!hasTauriRuntime()) {
    return {
      kind: 'check-failed',
      reason:
        'Desktop runtime boundary is unavailable, so governed SD-12 release truth cannot be proven from this context.',
      buildLabel: request.buildLabel,
      version: request.buildVersion,
    };
  }

  try {
    const channel = request.testerChannelLabel;
    const indexResult = await fetchChannelIndex(channel);
    if (!indexResult.ok) {
      return translateFailure(indexResult.failure, request);
    }
    const manifestResult = await fetchUpdateManifest(
      indexResult.value.manifest_url
    );
    if (!manifestResult.ok) {
      return translateFailure(manifestResult.failure, request);
    }
    return {
      kind: 'governed-release',
      manifest: translateManifest(manifestResult.value, request),
    };
  } catch (cause: unknown) {
    return {
      kind: 'check-failed',
      reason: `SD-11 update-action command failed: ${formatError(cause)}`,
      buildLabel: request.buildLabel,
      version: request.buildVersion,
    };
  }
}

function translateFailure(
  failure: FetchFailure,
  request: Sd11UpdateActionRequest
): Sd11UpdateReleaseTruth {
  const reason = renderFetchFailure(failure);
  return {
    kind: 'check-failed',
    reason,
    buildLabel: request.buildLabel,
    version: request.buildVersion,
  };
}

function renderFetchFailure(failure: FetchFailure): string {
  switch (failure.kind) {
    case 'http-error':
      return `HTTP ${failure.status} when fetching ${failure.url}`;
    case 'invalid-json':
      return `Invalid JSON when fetching ${failure.url}: ${failure.reason}`;
    case 'invalid-channel-index':
      return `Channel-index validation failed at ${failure.url}: ${failure.reason}`;
    case 'invalid-manifest':
      return `Update-manifest validation failed at ${failure.url}: ${failure.reason}`;
    case 'unsupported-channel':
      return `Channel "${failure.channel}" is not supported in this tranche`;
  }
}

function translateManifest(
  manifest: UpdateManifestFile,
  request: Sd11UpdateActionRequest
): Sd11UpdateManifestView {
  // Map the canonical schema-valid update manifest into the consumer-side
  // Sd11UpdateManifestView. The current build is the RUNNING build (from the
  // request), and the latest eligible build is the manifest's release —
  // collapsed back onto the current build when the manifest offers nothing
  // newer, so deriveSd11UpdateAction classifies equal identities as
  // up-to-date and distinct identities as update-available.
  const currentBuild: Sd11UpdateBuildIdentity = {
    releaseId: request.buildLabel,
    version: request.buildVersion,
    buildLabel: request.buildLabel,
    commitOrProvenanceHandle: 'running-build',
    publishedAt: 'running-build',
  };
  const manifestBuild: Sd11UpdateBuildIdentity = {
    releaseId: manifest.tag,
    version: manifest.version,
    buildLabel: manifest.linux_appimage.name,
    commitOrProvenanceHandle: manifest.source_commit,
    publishedAt: manifest.promotion_lineage.promoted_at,
  };
  const manifestIsNewer = compareVersions(manifest.version, request.buildVersion) > 0;
  const latestEligibleBuild = manifestIsNewer ? manifestBuild : currentBuild;

  // The manifest's eligibility policy: AppImage self-update must be enabled
  // and the running build must be at or above the manifest's version floor.
  const meetsVersionFloor =
    compareVersions(request.buildVersion, manifest.eligibility.min_supported_version) >= 0;
  const eligibilityState: Sd11ManifestEligibilityState =
    manifest.eligibility.appimage_install && meetsVersionFloor ? 'automatic' : 'manual-only';

  const integrity: Sd11UpdateIntegrityMaterial = {
    checksumAvailable: isNonEmptyString(manifest.linux_appimage.sha256),
    provenanceAvailable:
      isNonEmptyString(manifest.source_commit) &&
      isNonEmptyString(manifest.workflow_provenance.workflow),
    manifestAssetResolved: true, // fetch.ts only returns ok=true after a successful, schema-valid manifest fetch
    linuxArtifactPresent: isNonEmptyString(manifest.linux_appimage.url),
    recoveryPostureDefined: isNonEmptyString(manifest.release_notes_url),
  };
  return {
    manifestVersion: manifest.schema_version,
    channel: manifest.channel,
    operatorPromotionPathReference: `${manifest.promotion_lineage.source_branch} -> ${manifest.channel}`,
    platform: 'linux', // the canonical manifest is Linux-first-class per its contract; UI handles tier
    supportTier: 'first-class',
    currentBuild,
    latestEligibleBuild,
    lifecycleState: 'active' as Sd11ReleaseLifecycleState, // the manifest does not carry lifecycle; default active
    eligibilityState,
    integrity,
    replacementReleaseId: null,
    recoveryTarget: null,
    notes: [manifest.release_notes_url],
  };
}

function isNonEmptyString(value: string | null | undefined): boolean {
  return typeof value === 'string' && value.length > 0;
}

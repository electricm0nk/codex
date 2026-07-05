import { fetchChannelIndex, fetchUpdateManifest } from '../sd16/update/fetch';
import type {
  Sd16UpdateManifestFile,
  FetchFailure,
} from '../sd16/update/fetch';
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
      indexResult.value.release.manifestUrl
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
  manifest: Sd16UpdateManifestFile,
  request: Sd11UpdateActionRequest
): Sd11UpdateManifestView {
  // Map F3a's Sd16UpdateManifestFile into the consumer-side Sd11UpdateManifestView.
  // F3a already produced schema-valid typed output; this translation is a
  // pure field rename + integrity-derivation step.
  const currentBuild: Sd11UpdateBuildIdentity = {
    releaseId: manifest.artifact.path, // F3a uses `path` as the release id
    version: manifest.artifact.version,
    buildLabel: manifest.artifact.buildLabel,
    commitOrProvenanceHandle: manifest.artifact.commitOrProvenanceHandle,
    publishedAt: manifest.artifact.publishedAt,
  };
  const latestEligibleBuild: Sd11UpdateBuildIdentity = currentBuild;
  const integrity: Sd11UpdateIntegrityMaterial = {
    checksumAvailable: isNonEmptyString(manifest.artifact.artifactSha256),
    provenanceAvailable: isNonEmptyString(manifest.artifact.commitOrProvenanceHandle),
    manifestAssetResolved: true, // F3a only returns ok=true after successful manifest fetch
    linuxArtifactPresent: isNonEmptyString(manifest.artifact.path),
    recoveryPostureDefined: isNonEmptyString(manifest.notesUrl ?? null),
  };
  return {
    manifestVersion: manifest.schemaVersion,
    channel: manifest.channel,
    operatorPromotionPathReference: 'develop -> main', // per F3a's hard-coded operator path
    platform: 'linux', // F3a is Linux-first-class per F3a contract; UI handles tier
    supportTier: 'first-class', // F3a manifest is Linux-only; tier matches the support matrix
    currentBuild,
    latestEligibleBuild,
    lifecycleState: 'active' as Sd11ReleaseLifecycleState, // F3a manifest does not carry lifecycle; default active
    eligibilityState: manifest.eligibility as Sd11ManifestEligibilityState,
    integrity,
    replacementReleaseId: null,
    recoveryTarget: null,
    notes: manifest.notesUrl ? [manifest.notesUrl] : [],
  };
}

function isNonEmptyString(value: string | null | undefined): boolean {
  return typeof value === 'string' && value.length > 0;
}

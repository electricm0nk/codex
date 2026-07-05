import {
  loadSd11UpdateAction,
  type Sd11UpdateActionRequest,
} from './loadSd11UpdateAction';
import { deriveSd11UpdateAction } from '../sd11/update/deriveSd11UpdateAction';
import type {
  Sd11SupportTier,
} from '../sd11/status/createSd11WorkbenchStatus';
import type {
  Sd11UpdateActionResult,
  Sd11UpdateReleaseTruth,
} from '../sd11/update/updateActionModel';

export interface Sd12ReleaseTruthIssueCapture {
  releaseUnitId: string | null;
  sourceRevision: string | null;
  manifestPath: string | null;
  updateEligibilityState: string | null;
  trustGateStatus: string | null;
  replacementReleaseId: string | null;
  officialSurface: string;
  localBuildAuthority: string;
}

export interface Sd12ReleaseTruthSnapshot {
  truth: Sd11UpdateReleaseTruth;
  updateAction: Sd11UpdateActionResult;
  issueCapture: Sd12ReleaseTruthIssueCapture;
}

export type Sd12ReleaseTruthRequest = Sd11UpdateActionRequest;

const OFFICIAL_SURFACE =
  'GitHub release assets published by .github/workflows/publish-tester-release.yml and consumed via apps/desktop/src/boundary/loadSd11UpdateAction.ts over the F3a fetch pipeline';

export async function loadSd12ReleaseTruth(
  request: Sd12ReleaseTruthRequest
): Promise<Sd12ReleaseTruthSnapshot> {
  const truth = await loadSd11UpdateAction(request);
  const updateAction = deriveSd11UpdateAction(
    {
      buildLabel: request.buildLabel,
      buildVersion: request.buildVersion,
      platformLabel: request.platformLabel,
      platformTier: derivePlatformTier(request.platformLabel),
      testerChannelLabel: request.testerChannelLabel,
    },
    truth
  );

  return {
    truth,
    updateAction,
    issueCapture: buildIssueCapture(truth, updateAction),
  };
}

function buildIssueCapture(
  truth: Sd11UpdateReleaseTruth,
  updateAction: Sd11UpdateActionResult
): Sd12ReleaseTruthIssueCapture {
  if (truth.kind === 'governed-release') {
    return {
      releaseUnitId: truth.manifest.currentBuild.releaseId,
      sourceRevision: truth.manifest.currentBuild.commitOrProvenanceHandle,
      manifestPath: truth.manifest.integrity.manifestAssetResolved
        ? 'GitHub release asset: update-manifest-stub.json'
        : null,
      updateEligibilityState: truth.manifest.eligibilityState,
      trustGateStatus: deriveTrustGateStatus(updateAction),
      replacementReleaseId: truth.manifest.replacementReleaseId,
      officialSurface: OFFICIAL_SURFACE,
      localBuildAuthority: 'governed-release-unit',
    };
  }

  if (truth.kind === 'no-official-release') {
    return {
      releaseUnitId: null,
      sourceRevision: null,
      manifestPath: null,
      updateEligibilityState: 'no-official-release',
      trustGateStatus: 'not-applicable-no-governed-release',
      replacementReleaseId: null,
      officialSurface: OFFICIAL_SURFACE,
      localBuildAuthority: truth.reason,
    };
  }

  return {
    releaseUnitId: null,
    sourceRevision: null,
    manifestPath: null,
    updateEligibilityState: 'check-failed',
    trustGateStatus: 'unverified-runtime-check-failed',
    replacementReleaseId: null,
    officialSurface: OFFICIAL_SURFACE,
    localBuildAuthority: truth.reason,
  };
}

function deriveTrustGateStatus(updateAction: Sd11UpdateActionResult): string {
  switch (updateAction.state) {
    case 'update-available':
      return updateAction.automaticEligible ? 'automatic-eligible' : 'update-available-manual';
    case 'manual-only':
      return 'governed-manual-only';
    case 'blocked':
      return 'governed-blocked';
    case 'withdrawn':
      return 'governed-withdrawn';
    case 'unsupported':
      return 'governed-unsupported';
    case 'up-to-date':
      return updateAction.automaticEligible ? 'automatic-eligible-no-newer-release' : 'governed-current';
    case 'check-failed':
      return 'unverified-runtime-check-failed';
    case 'no-official-release-for-this-build':
      return 'not-applicable-no-governed-release';
  }
}

function derivePlatformTier(platformLabel: string): Sd11SupportTier {
  const normalised = platformLabel.trim().toLowerCase();

  if (normalised === 'linux') {
    return 'first-class';
  }

  if (normalised === 'macos') {
    return 'second-class';
  }

  if (normalised === 'windows') {
    return 'third-class';
  }

  return 'unknown';
}

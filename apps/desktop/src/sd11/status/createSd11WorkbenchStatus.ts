import type { Sd12ReleaseTruthIssueCapture, Sd12ReleaseTruthSnapshot } from '../../boundary/loadSd12ReleaseTruth';
import type { Sd11UpdateResultState } from '../update/updateActionModel';

export type Sd11SupportTier = 'first-class' | 'second-class' | 'third-class' | 'unknown';
// `check-ready` / `not-yet-supported` are retained only so adjacent fixtures keep typechecking;
// live status should prefer the normalized SD-12 release-truth bridge states.
export type Sd11UpdateState = Sd11UpdateResultState | 'check-ready' | 'not-yet-supported';
// Accepted SD-12 operator promotion truth is `develop -> main`. The legacy `develop -> uat -> main`
// literal is retained as a type member only so adjacent feedback-slice fixtures keep typechecking.
export type Sd11OperatorPromotionPath = 'develop -> main' | 'develop -> uat -> main';

export interface Sd11WorkbenchStatusContext {
  buildVersion: string;
  platformLabel: string;
}

export interface Sd11WorkbenchBuildStatus {
  label: string;
  version: string;
}

export interface Sd11WorkbenchChannelStatus {
  testerFacingLabel: 'alpha';
  operatorBranch: 'develop';
  operatorPromotionPath: Sd11OperatorPromotionPath;
  audience: string;
  detail: string;
}

export interface Sd11WorkbenchSupportStatus {
  platformLabel: string;
  platformTier: Sd11SupportTier;
  currentPlatformSupportLabel: string;
  tierMatrixLabel: string;
  platformSupportDetail: string;
}

export interface Sd11WorkbenchUpdateStatus {
  state: Sd11UpdateState;
  label: string;
  detail: string;
}

export interface Sd11WorkbenchIssueCaptureStatus {
  testerFacingChannelSupportLabel: string;
  operatorBranch: 'develop';
  operatorPromotionPath: Sd11OperatorPromotionPath;
  platformLabel: string;
  platformTier: Sd11SupportTier;
  releaseTruth?: Sd12ReleaseTruthIssueCapture;
}

export interface Sd11WorkbenchStatus {
  build: Sd11WorkbenchBuildStatus;
  channel: Sd11WorkbenchChannelStatus;
  support: Sd11WorkbenchSupportStatus;
  update: Sd11WorkbenchUpdateStatus;
  issueCapture: Sd11WorkbenchIssueCaptureStatus;
}

const BUILD_PREFIX = 'codex';
const TESTER_CHANNEL: Sd11WorkbenchChannelStatus['testerFacingLabel'] = 'alpha';
const OPERATOR_BRANCH: Sd11WorkbenchChannelStatus['operatorBranch'] = 'develop';
const OPERATOR_PROMOTION_PATH: Sd11WorkbenchChannelStatus['operatorPromotionPath'] = 'develop -> main';
const CHANNEL_AUDIENCE = 'fastest-moving tester track; highest churn; acceptable for close/internal testers';
const CHANNEL_DETAIL = 'Tester-facing channel language over the develop → main operator path.';
const SUPPORT_TIER_MATRIX_LABEL = 'Linux first-class · macOS second-class · Windows third-class';
const DEFAULT_UPDATE_LABEL = 'Bounded update check available';
const DEFAULT_UPDATE_DETAIL =
  'Run a bounded check against governed SD-12 release truth. SD-11 consumes release truth as a client; it never authors releases, applies installers, or surfaces raw branch names as the primary tester UX. Outcomes stay honest and platform-aware: up-to-date, update-available, manual-only, blocked, withdrawn, unsupported, check-failed, or no official release for this build.';

export function formatSd11WorkbenchBuildLabel(buildVersion: string): string {
  return `${BUILD_PREFIX}@${buildVersion}`;
}

export function createSd11WorkbenchStatus(
  context: Sd11WorkbenchStatusContext,
  releaseTruth?: Sd12ReleaseTruthSnapshot
): Sd11WorkbenchStatus {
  const platformTier = derivePlatformTier(context.platformLabel);
  const currentPlatformSupportLabel = formatCurrentPlatformSupportLabel(context.platformLabel, platformTier);
  const update = releaseTruth?.updateAction;

  return {
    build: {
      label: formatSd11WorkbenchBuildLabel(context.buildVersion),
      version: context.buildVersion,
    },
    channel: {
      testerFacingLabel: TESTER_CHANNEL,
      operatorBranch: OPERATOR_BRANCH,
      operatorPromotionPath: resolveOperatorPromotionPath(update?.operatorPromotionPathReference),
      audience: CHANNEL_AUDIENCE,
      detail: CHANNEL_DETAIL,
    },
    support: {
      platformLabel: context.platformLabel,
      platformTier,
      currentPlatformSupportLabel,
      tierMatrixLabel: SUPPORT_TIER_MATRIX_LABEL,
      platformSupportDetail: describePlatformSupport(context.platformLabel, platformTier),
    },
    update: {
      state: update?.state ?? 'check-ready',
      label: update?.headline ?? DEFAULT_UPDATE_LABEL,
      detail: update?.detail ?? DEFAULT_UPDATE_DETAIL,
    },
    issueCapture: {
      testerFacingChannelSupportLabel: `${TESTER_CHANNEL} · ${currentPlatformSupportLabel}`,
      operatorBranch: OPERATOR_BRANCH,
      operatorPromotionPath: resolveOperatorPromotionPath(update?.operatorPromotionPathReference),
      platformLabel: context.platformLabel,
      platformTier,
      releaseTruth: releaseTruth?.issueCapture ?? defaultReleaseTruthIssueCapture(),
    },
  };
}

function defaultReleaseTruthIssueCapture(): Sd12ReleaseTruthIssueCapture {
  return {
    releaseUnitId: null,
    sourceRevision: null,
    manifestPath: null,
    updateEligibilityState: 'not-captured',
    trustGateStatus: 'not-captured',
    replacementReleaseId: null,
    officialSurface:
      'GitHub release assets published by .github/workflows/publish-tester-release.yml and consumed via apps/desktop/src/boundary/loadSd11UpdateAction.ts over the F3a fetch pipeline',
    localBuildAuthority: 'release-truth bridge not yet loaded',
  };
}

function resolveOperatorPromotionPath(
  value: string | null | undefined
): Sd11WorkbenchChannelStatus['operatorPromotionPath'] {
  if (value === 'develop -> main' || value === 'develop -> uat -> main') {
    return value;
  }

  return OPERATOR_PROMOTION_PATH;
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

function formatCurrentPlatformSupportLabel(platformLabel: string, platformTier: Sd11SupportTier): string {
  if (platformTier === 'unknown') {
    return `${platformLabel} outside named support matrix`;
  }

  return `${platformLabel} ${platformTier}`;
}

function describePlatformSupport(platformLabel: string, platformTier: Sd11SupportTier): string {
  if (platformTier === 'first-class') {
    return 'Linux is the first-class tester surface in this tranche and the strongest candidate for later self-update coverage.';
  }

  if (platformTier === 'second-class') {
    return 'macOS is a real but less mature tester surface in this tranche.';
  }

  if (platformTier === 'third-class') {
    return 'Windows remains explicitly bounded in this tranche. No fake parity claims.';
  }

  return `${platformLabel} is outside the named support matrix for this tranche.`;
}

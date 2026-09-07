import type { TesterWorkbenchSurface } from '../testerWorkbench/loadTesterWorkbenchSurface';

/**
 * Canonical SD-11 tester workbench surface fixture for tests.
 *
 * This is the single source of truth for the test surface shape. Several test
 * files previously carried their own copies; the copies drifted when the SD-12
 * release-truth bridge added new required auto-captured evidence fields, which
 * silently broke the submit-flow tests. Keep additions here so every consumer
 * sees the same, complete surface.
 *
 * Overrides are spread shallowly — pass a whole replacement `status` (or other
 * nested object) if a test needs to vary nested fields.
 */
export function makeSurface(
  overrides: Partial<TesterWorkbenchSurface> = {}
): TesterWorkbenchSurface {
  return {
    surfaceLabel: 'Developer diagnostics',
    headline: 'Connected to the app backend',
    lead: 'lead',
    buildLabel: 'Codex 0.14.0-test',
    channelLabel: 'alpha',
    platformLabel: 'Linux',
    supportTierLabel: 'Linux first-class · macOS second-class · Windows third-class',
    workflowName: 'Character-preview authoring check',
    workflowState: 'Authored / Computed',
    dataTruthLabel: 'Live backend data',
    fallbackNotice: null,
    boundedScopeNotice: 'bounded',
    feedbackStatusNotice: 'feedback',
    updateStatusLabel: 'alpha tester track on Linux first-class',
    summaryRows: [],
    diagnostics: [
      {
        classLabel: 'GuardStance',
        severity: 'warning',
        severityLabel: 'Warning',
        message: 'A claim is blocked.',
        subjectRef: 'node:1',
        claimBlocking: true,
      },
    ],
    blockedClaims: ['Baseline AC blocked'],
    explanationRefs: [{ label: 'Rule:1', detail: 'detail', machineRef: 'Rule:1' }],
    provenanceRefs: [],
    notes: ['note'],
    status: {
      build: { label: 'Codex 0.14.0-test', version: '0.14.0-test' },
      channel: {
        testerFacingLabel: 'alpha',
        operatorBranch: 'develop',
        operatorPromotionPath: 'develop -> main',
        audience: 'audience',
        detail: 'detail',
      },
      support: {
        platformLabel: 'Linux',
        platformTier: 'first-class',
        currentPlatformSupportLabel: 'Linux first-class',
        tierMatrixLabel: 'Linux first-class · macOS second-class · Windows third-class',
        platformSupportDetail: 'detail',
      },
      update: {
        state: 'not-yet-supported',
        label: 'Update checks not yet wired in this slice',
        detail: 'detail',
      },
      issueCapture: {
        testerFacingChannelSupportLabel: 'alpha · Linux first-class',
        operatorBranch: 'develop',
        operatorPromotionPath: 'develop -> main',
        platformLabel: 'Linux',
        platformTier: 'first-class',
        releaseTruth: {
          releaseUnitId: 'alpha-v0.0.0-test-1234abcd',
          sourceRevision: '1234abcd',
          manifestPath: 'release asset: update-manifest-stub.json',
          updateEligibilityState: 'manual-only',
          trustGateStatus: 'governed-manual-only',
          replacementReleaseId: 'alpha-v0.0.1-test-9876fedc',
          officialSurface:
            'GitHub release assets published by .github/workflows/publish-tester-release.yml and consumed via apps/desktop/src/boundary/loadUpdateAction.ts over the F3a fetch pipeline',
          localBuildAuthority: 'governed-release-unit',
        },
      },
    },
    ...overrides,
  };
}

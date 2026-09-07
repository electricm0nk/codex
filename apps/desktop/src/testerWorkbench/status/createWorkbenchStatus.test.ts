import { createWorkbenchStatus } from './createWorkbenchStatus';
import { assertEqual } from '../../testSupport/asserts';

async function main() {
  verifiesLinuxAlphaStatusTruth();
  verifiesWindowsSupportAsymmetry();
}

function noOfficialReleaseTruth() {
  return {
    truth: {
      kind: 'no-official-release' as const,
      reason: 'Feature/local builds are not governed tester release units.',
      buildLabel: 'Codex 0.14.0-test',
      version: '0.0.0-test',
    },
    updateAction: {
      state: 'no-official-release-for-this-build' as const,
      headline: 'No official tester release for this build',
      detail: 'This is a local, non-governed build, so no governed update outcome is claimed.',
      platformLabel: 'Linux',
      platformTier: 'first-class' as const,
      testerChannelLabel: 'alpha' as const,
      automaticEligible: false,
      manualReason: null,
      replacementTarget: null,
      recoveryDirection: null,
      checkedBuildLabel: 'Codex 0.14.0-test',
      checkedVersion: '0.0.0-test',
      operatorPromotionPathReference: null,
      evidenceNotes: [],
    },
    issueCapture: {
      releaseUnitId: null,
      sourceRevision: null,
      manifestPath: null,
      updateEligibilityState: 'no-official-release',
      trustGateStatus: 'not-applicable-no-governed-release',
      replacementReleaseId: null,
      officialSurface:
        'GitHub release assets published by .github/workflows/publish-tester-release.yml and consumed via apps/desktop/src/boundary/loadUpdateAction.ts over the F3a fetch pipeline',
      localBuildAuthority: 'Feature/local build — no governed release unit was proven.',
    },
  };
}

function verifiesLinuxAlphaStatusTruth() {
  const status = createWorkbenchStatus({
    buildVersion: '0.0.0-test',
    platformLabel: 'Linux',
  }, noOfficialReleaseTruth());

  assertEqual(status.build.label, 'Codex 0.0.0-test', 'build label');
  assertEqual(status.channel.testerFacingLabel, 'alpha', 'tester-facing channel');
  assertEqual(status.channel.operatorBranch, 'develop', 'operator branch');
  assertEqual(status.channel.operatorPromotionPath, 'develop -> main', 'operator promotion path');
  assertEqual(
    status.channel.audience,
    'Fastest-moving build track — changes often, meant for close/internal testers.',
    'channel audience'
  );
  assertEqual(status.support.platformTier, 'first-class', 'linux support tier');
  assertEqual(status.support.currentPlatformSupportLabel, 'Linux first-class', 'linux support label');
  assertEqual(
    status.support.tierMatrixLabel,
    'Linux first-class · macOS second-class · Windows third-class',
    'support tier matrix'
  );
  assertEqual(status.update.state, 'no-official-release-for-this-build', 'update state');
  assertEqual(status.update.label, 'No official tester release for this build', 'update label');
  assertEqual(status.issueCapture.testerFacingChannelSupportLabel, 'alpha · Linux first-class', 'issue capture label');
  assertEqual(
    status.issueCapture.releaseTruth?.updateEligibilityState,
    'no-official-release',
    'issue-capture release eligibility state'
  );
}

function verifiesWindowsSupportAsymmetry() {
  const status = createWorkbenchStatus({
    buildVersion: '0.0.0-test',
    platformLabel: 'Windows',
  });

  assertEqual(status.support.platformTier, 'third-class', 'windows support tier');
  assertEqual(status.support.currentPlatformSupportLabel, 'Windows third-class', 'windows support label');
  assertEqual(
    status.support.platformSupportDetail,
    'Windows support is still limited — some features may not work as well here yet.',
    'windows support detail'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

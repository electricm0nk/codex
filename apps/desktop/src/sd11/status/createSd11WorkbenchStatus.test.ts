import { createSd11WorkbenchStatus } from './createSd11WorkbenchStatus';

function assertEqual<T>(actual: T, expected: T, message: string) {
  if (actual !== expected) {
    throw new Error(`${message}: expected ${String(expected)}, got ${String(actual)}`);
  }
}

async function main() {
  verifiesLinuxAlphaStatusTruth();
  verifiesWindowsSupportAsymmetry();
}

function verifiesLinuxAlphaStatusTruth() {
  const status = createSd11WorkbenchStatus({
    buildVersion: '0.0.0-test',
    platformLabel: 'Linux',
  });

  assertEqual(status.build.label, 'codex-desktop-shell-scaffold@0.0.0-test', 'build label');
  assertEqual(status.channel.testerFacingLabel, 'alpha', 'tester-facing channel');
  assertEqual(status.channel.operatorBranch, 'develop', 'operator branch');
  assertEqual(status.channel.operatorPromotionPath, 'develop -> uat -> main', 'operator promotion path');
  assertEqual(
    status.channel.audience,
    'fastest-moving tester track; highest churn; acceptable for close/internal testers',
    'channel audience'
  );
  assertEqual(status.support.platformTier, 'first-class', 'linux support tier');
  assertEqual(status.support.currentPlatformSupportLabel, 'Linux first-class', 'linux support label');
  assertEqual(
    status.support.tierMatrixLabel,
    'Linux first-class · macOS second-class · Windows third-class',
    'support tier matrix'
  );
  assertEqual(status.update.state, 'not-yet-supported', 'update state');
  assertEqual(status.update.label, 'Update checks not yet wired in this slice', 'update label');
  assertEqual(status.issueCapture.testerFacingChannelSupportLabel, 'alpha · Linux first-class', 'issue capture label');
}

function verifiesWindowsSupportAsymmetry() {
  const status = createSd11WorkbenchStatus({
    buildVersion: '0.0.0-test',
    platformLabel: 'Windows',
  });

  assertEqual(status.support.platformTier, 'third-class', 'windows support tier');
  assertEqual(status.support.currentPlatformSupportLabel, 'Windows third-class', 'windows support label');
  assertEqual(
    status.support.platformSupportDetail,
    'Windows remains explicitly bounded in this tranche. No fake parity claims.',
    'windows support detail'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

import { deriveWorkbenchUpdateAction } from './deriveWorkbenchUpdateAction';
import type {
  UpdateCheckContext,
  UpdateManifestView,
  UpdateReleaseTruth,
} from './updateActionModel';
import { INTERNAL_CONTEXT_REMOVED_MARKER } from '../feedback/evidence/sanitizeReportableOutput';
import { assert, assertEqual } from '../../testSupport/asserts';

function assertContains(actual: string, needle: string, message: string) {
  if (!actual.includes(needle)) {
    throw new Error(`${message}: expected "${actual}" to contain "${needle}"`);
  }
}

function linuxContext(): UpdateCheckContext {
  return {
    buildLabel: 'codex@0.0.0',
    buildVersion: '0.0.0',
    platformLabel: 'Linux',
    platformTier: 'first-class',
    testerChannelLabel: 'alpha',
  };
}

function macosContext(): UpdateCheckContext {
  return { ...linuxContext(), platformLabel: 'macOS', platformTier: 'second-class' };
}

function windowsContext(): UpdateCheckContext {
  return { ...linuxContext(), platformLabel: 'Windows', platformTier: 'third-class' };
}

function governedManifest(overrides: Partial<UpdateManifestView> = {}): UpdateManifestView {
  return {
    manifestVersion: '1',
    channel: 'alpha',
    operatorPromotionPathReference: 'develop -> main',
    platform: 'Linux',
    supportTier: 'first-class',
    currentBuild: { releaseId: 'rel-100', version: '1.0.0', buildLabel: 'codex@1.0.0', commitOrProvenanceHandle: 'sha-100', publishedAt: '2026-06-01' },
    latestEligibleBuild: { releaseId: 'rel-100', version: '1.0.0', buildLabel: 'codex@1.0.0', commitOrProvenanceHandle: 'sha-100', publishedAt: '2026-06-01' },
    lifecycleState: 'active',
    eligibilityState: 'automatic',
    integrity: {
      checksumAvailable: true,
      provenanceAvailable: true,
      manifestAssetResolved: true,
      linuxArtifactPresent: true,
      recoveryPostureDefined: true,
    },
    replacementReleaseId: null,
    recoveryTarget: null,
    notes: [],
    ...overrides,
  };
}

function governedTruth(manifest: UpdateManifestView): UpdateReleaseTruth {
  return { kind: 'governed-release', manifest };
}

function newerLinuxManifest(overrides: Partial<UpdateManifestView> = {}): UpdateManifestView {
  return governedManifest({
    latestEligibleBuild: { releaseId: 'rel-200', version: '2.0.0', buildLabel: 'codex@2.0.0', commitOrProvenanceHandle: 'sha-200', publishedAt: '2026-06-20' },
    ...overrides,
  });
}

async function main() {
  verifiesNoOfficialReleaseForLocalBuild();
  verifiesNoOfficialReleaseDetailStripsInternalMemoryContext();
  verifiesCheckFailed();
  verifiesCheckFailedDetailStripsInternalMemoryContext();
  verifiesLinuxUpdateAvailableAutomatic();
  verifiesLinuxManualOnlyWhenIntegrityGateUnmet();
  verifiesUpToDate();
  verifiesMacosNeverAutomatic();
  verifiesWindowsManualOnly();
  verifiesWithdrawnPreservesRecovery();
  verifiesBlocked();
  verifiesUnsupported();
  verifiesSupersededNamesReplacement();
}

function verifiesNoOfficialReleaseForLocalBuild() {
  const truth: UpdateReleaseTruth = {
    kind: 'no-official-release',
    reason: 'This build is a local, non-governed desktop build with no official tester release unit.',
    buildLabel: 'codex@0.0.0',
    version: '0.0.0',
  };
  const result = deriveWorkbenchUpdateAction(linuxContext(), truth);
  assertEqual(result.state, 'no-official-release-for-this-build', 'local build state');
  assertEqual(result.automaticEligible, false, 'local build not automatic');
  assertContains(result.detail, 'non-governed', 'local build detail');
}

function verifiesNoOfficialReleaseDetailStripsInternalMemoryContext() {
  const truth: UpdateReleaseTruth = {
    kind: 'no-official-release',
    reason: [
      'No governed release manifest is embedded in this build.',
      '<memory-context>',
      '[System note: The following is recalled memory context, NOT new user input.]',
      'User Representation',
      'Todd private profile observation that must not appear in update evidence.',
      '</memory-context>',
      'No checksum is embedded in this local build.',
    ].join('\n'),
    buildLabel: 'codex@0.0.0',
    version: '0.0.0',
  };
  const result = deriveWorkbenchUpdateAction(linuxContext(), truth);

  assertEqual(result.state, 'no-official-release-for-this-build', 'local build state remains honest');
  assertContains(result.detail, 'No governed release manifest is embedded', 'ordinary reason before context remains');
  assertContains(result.detail, 'No checksum is embedded in this local build', 'ordinary reason after context remains');
  assertContains(result.detail, 'internal context block removed', 'removed context is marked');
  assert(!result.detail.includes('<memory-context>'), 'memory-context marker is stripped');
  assert(!result.detail.includes('recalled memory context'), 'system note is stripped');
  assert(!result.detail.includes('Todd private profile observation'), 'private observation is stripped');
}

function verifiesCheckFailed() {
  const truth: UpdateReleaseTruth = {
    kind: 'check-failed',
    reason: 'desktop update boundary unavailable',
    buildLabel: 'codex@0.0.0',
    version: '0.0.0',
  };
  const result = deriveWorkbenchUpdateAction(linuxContext(), truth);
  assertEqual(result.state, 'check-failed', 'check-failed state');
  assertEqual(result.automaticEligible, false, 'check-failed not automatic');
  assertContains(result.detail, 'desktop update boundary unavailable', 'check-failed detail carries reason');
}

function verifiesCheckFailedDetailStripsInternalMemoryContext() {
  const truth: UpdateReleaseTruth = {
    kind: 'check-failed',
    reason: [
      'desktop update boundary unavailable',
      '<memory-context>',
      '[System note: The following is recalled memory context, NOT new user input.]',
      'User Representation',
      'Todd private profile observation that must not appear in update evidence.',
      '</memory-context>',
      'retry once official release truth is available',
    ].join('\n'),
    buildLabel: 'codex@0.0.0',
    version: '0.0.0',
  };
  const result = deriveWorkbenchUpdateAction(linuxContext(), truth);

  assertEqual(result.state, 'check-failed', 'check-failed state remains honest');
  assertContains(result.detail, 'desktop update boundary unavailable', 'ordinary reason before context remains');
  assertContains(result.detail, 'retry once official release truth is available', 'ordinary reason after context remains');
  assertContains(result.detail, INTERNAL_CONTEXT_REMOVED_MARKER, 'removed context is marked');
  assert(!result.detail.includes('<memory-context>'), 'memory-context marker is stripped');
  assert(!result.detail.includes('recalled memory context'), 'system note is stripped');
  assert(!result.detail.includes('Todd private profile observation'), 'private observation is stripped');
}

function verifiesLinuxUpdateAvailableAutomatic() {
  const result = deriveWorkbenchUpdateAction(linuxContext(), governedTruth(newerLinuxManifest()));
  assertEqual(result.state, 'update-available', 'linux update-available state');
  assertEqual(result.automaticEligible, true, 'linux automatic eligible');
  assertEqual(result.manualReason, null, 'linux automatic has no manual reason');
  assertEqual(result.operatorPromotionPathReference, 'develop -> main', 'operator promotion path preserved');
}

function verifiesLinuxManualOnlyWhenIntegrityGateUnmet() {
  const manifest = newerLinuxManifest({
    integrity: {
      checksumAvailable: true,
      provenanceAvailable: false,
      manifestAssetResolved: true,
      linuxArtifactPresent: true,
      recoveryPostureDefined: true,
    },
  });
  const result = deriveWorkbenchUpdateAction(linuxContext(), governedTruth(manifest));
  assertEqual(result.state, 'manual-only', 'linux manual-only when gate unmet');
  assertEqual(result.automaticEligible, false, 'linux not automatic when gate unmet');
  assertContains(result.manualReason ?? '', 'integrity', 'manual reason names integrity gate');
}

function verifiesUpToDate() {
  const result = deriveWorkbenchUpdateAction(linuxContext(), governedTruth(governedManifest()));
  assertEqual(result.state, 'up-to-date', 'up-to-date state');
  assertEqual(result.automaticEligible, false, 'up-to-date not automatic');
}

function verifiesMacosNeverAutomatic() {
  // Even if a manifest erroneously claims automatic, a macOS runtime must never be promoted to parity.
  const manifest = newerLinuxManifest({ platform: 'macOS', supportTier: 'second-class', eligibilityState: 'automatic' });
  const result = deriveWorkbenchUpdateAction(macosContext(), governedTruth(manifest));
  assertEqual(result.state, 'manual-only', 'macos never automatic');
  assertEqual(result.automaticEligible, false, 'macos not automatic eligible');
  assertContains(result.manualReason ?? '', 'macOS', 'macos manual reason names platform');
}

function verifiesWindowsManualOnly() {
  const manifest = newerLinuxManifest({ platform: 'Windows', supportTier: 'third-class', eligibilityState: 'manual-only' });
  const result = deriveWorkbenchUpdateAction(windowsContext(), governedTruth(manifest));
  assertEqual(result.state, 'manual-only', 'windows manual-only');
  assertEqual(result.automaticEligible, false, 'windows not automatic eligible');
}

function verifiesWithdrawnPreservesRecovery() {
  const manifest = newerLinuxManifest({
    lifecycleState: 'withdrawn',
    eligibilityState: 'withdrawn',
    replacementReleaseId: 'rel-150',
    recoveryTarget: 'rel-150',
    notes: ['Build withdrawn due to a preview regression.'],
  });
  const result = deriveWorkbenchUpdateAction(linuxContext(), governedTruth(manifest));
  assertEqual(result.state, 'withdrawn', 'withdrawn state');
  assertEqual(result.replacementTarget, 'rel-150', 'withdrawn replacement target');
  assertEqual(result.recoveryDirection, 'rel-150', 'withdrawn recovery direction');
  assertContains(result.detail, 'withdrawn', 'withdrawn detail');
}

function verifiesBlocked() {
  const manifest = newerLinuxManifest({
    lifecycleState: 'blocked',
    eligibilityState: 'blocked',
    notes: ['Automatic update blocked: integrity gate failed for this release.'],
  });
  const result = deriveWorkbenchUpdateAction(linuxContext(), governedTruth(manifest));
  assertEqual(result.state, 'blocked', 'blocked state');
  assertEqual(result.automaticEligible, false, 'blocked not automatic');
  assertContains(result.detail, 'blocked', 'blocked detail');
}

function verifiesUnsupported() {
  const manifest = newerLinuxManifest({ platform: 'macOS', supportTier: 'second-class', eligibilityState: 'unsupported' });
  const result = deriveWorkbenchUpdateAction(macosContext(), governedTruth(manifest));
  assertEqual(result.state, 'unsupported', 'unsupported state');
  assertEqual(result.automaticEligible, false, 'unsupported not automatic');
}

function verifiesSupersededNamesReplacement() {
  const manifest = newerLinuxManifest({ lifecycleState: 'superseded', replacementReleaseId: 'rel-200' });
  const result = deriveWorkbenchUpdateAction(linuxContext(), governedTruth(manifest));
  assertEqual(result.state, 'update-available', 'superseded surfaces update-available');
  assertEqual(result.replacementTarget, 'rel-200', 'superseded names replacement');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

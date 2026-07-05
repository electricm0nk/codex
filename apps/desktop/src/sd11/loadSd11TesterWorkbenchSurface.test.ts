import { loadSd11TesterWorkbenchSurface } from './loadSd11TesterWorkbenchSurface';
import { assertEqual } from '../testSupport/asserts';

async function main() {
  await verifiesRealGe08SnapshotSurface();
  await verifiesExplicitFallbackSurface();
}

function noOfficialReleaseTruth() {
  return {
    truth: {
      kind: 'no-official-release' as const,
      reason: 'Feature/local builds are not governed tester release units.',
      buildLabel: 'codex-desktop-shell-scaffold@0.0.0-test',
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
      checkedBuildLabel: 'codex-desktop-shell-scaffold@0.0.0-test',
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
        'GitHub release assets published by .github/workflows/publish-tester-release.yml and consumed via apps/desktop/src/boundary/loadSd11UpdateAction.ts over the F3a fetch pipeline',
      localBuildAuthority: 'Feature/local build — no governed release unit was proven.',
    },
  };
}

async function verifiesRealGe08SnapshotSurface() {
  let requestedPackageRoot: string | null = null;
  const model = await loadSd11TesterWorkbenchSurface(
    {
      buildVersion: '0.0.0-test',
      platformLabel: 'Linux',
    },
    {
      loadSd12ReleaseTruth: async () => noOfficialReleaseTruth(),
      loadGe08AuthoringWorkbench: async (request) => {
        requestedPackageRoot = request.packageRoot;
        return {
          packageRoot: request.packageRoot,
          packageState: 'valid',
          packageManifest: {
            packageId: 'guard-stance',
            packageTitle: 'Guard Stance',
            packageVersion: '0.1.0',
            dependsOn: [],
            supportedObjectKinds: ['feat', 'effect', 'prerequisite'],
          },
          activeRecordRef: null,
          authoredRecords: {
            feat: null,
            effect: null,
            prerequisite: null,
          },
          preview: {
            caseId: 'guard-stance-preview',
            previewStatus: 'success',
            selectedSlotResolution: {
              slot: 'human_bonus_feat',
              removed: 'Skill Focus',
              added: 'Guard Stance',
              resolvedFeatId: 'feat.guard_stance',
            },
            baselineArmorClass: {
              kind: 'Computed',
              value: 17,
            },
            diagnostics: [
              {
                class: 'Advisory',
                severity: 'Warning',
                message: 'Preview is bounded to the current GE08 package.',
                subjectRef: 'preview.guard_stance',
                claimBlocking: false,
              },
            ],
            provenanceRefs: [
              {
                stableId: 'feat.guard_stance',
                sourcePackageId: 'guard-stance',
                authoredPath: 'feat/guard-stance.json',
              },
            ],
            explanationRefs: [
              {
                nodeKind: 'rule',
                refId: 'guard-stance-ac',
                detail: 'Baseline armor class derives from the guarded stance package preview.',
              },
            ],
            oracleDimensionStatus: [
              {
                dimension: 'armor_class',
                status: 'bounded-success',
              },
            ],
            blockedClaims: [],
          },
          lifecycleGateState: {
            saveAllowed: true,
            previewAllowed: true,
            exportAllowed: true,
            diffMode: 'deferred',
          },
          dataSource: 'ge08-headless-preview-bridge',
          note: 'Real GE08 authoring workbench snapshot from headless substrate.',
        };
      },
      loadPilotShellSnapshot: async () => {
        throw new Error('fallback should not be used when the GE08 workbench is available');
      },
    }
  );

  assertEqual(
    requestedPackageRoot,
    'resources/ge08/guard-stance-package',
    'default GE08 package root uses the packaged resource path instead of source checkout fixtures'
  );
  assertEqual(model.surfaceLabel, 'SD-11 tester workbench', 'surface label');
  assertEqual(model.workflowName, 'GE08 Guard Stance authoring workbench', 'workflow name');
  assertEqual(model.workflowState, 'valid / success', 'workflow state');
  assertEqual(model.channelLabel, 'alpha', 'channel label');
  assertEqual(
    model.supportTierLabel,
    'Linux first-class · macOS second-class · Windows third-class',
    'support tier label'
  );
  assertEqual(model.dataTruthLabel, 'Real Tauri command snapshot', 'data truth label');
  assertEqual(model.summaryRows[0]?.label, 'Package', 'summary row label');
  assertEqual(model.summaryRows[0]?.value, 'guard-stance', 'summary row value');
  assertEqual(
    model.diagnostics[0]?.message,
    'Preview is bounded to the current GE08 package.',
    'diagnostic message'
  );
  assertEqual(model.diagnostics[0]?.classLabel, 'Advisory', 'diagnostic class label');
  assertEqual(model.diagnostics[0]?.severityLabel, 'Warning', 'diagnostic severity label');
  assertEqual(model.diagnostics[0]?.subjectRef, 'preview.guard_stance', 'diagnostic subject ref');
  assertEqual(model.diagnostics[0]?.claimBlocking, false, 'diagnostic claim-blocking flag');
  assertEqual(model.explanationRefs[0]?.label, 'rule:guard-stance-ac', 'explanation ref label');
  assertEqual(
    model.explanationRefs[0]?.detail,
    'Baseline armor class derives from the guarded stance package preview.',
    'explanation ref detail'
  );
  assertEqual(model.provenanceRefs[0]?.label, 'feat.guard_stance', 'provenance ref label');
  assertEqual(
    model.provenanceRefs[0]?.detail,
    'guard-stance · feat/guard-stance.json',
    'provenance ref detail'
  );
  assertEqual(
    model.updateStatusLabel,
    'No official tester release for this build',
    'update status label'
  );
  assertEqual(model.status.update.state, 'no-official-release-for-this-build', 'status update state');
  assertEqual(model.status.channel.operatorBranch, 'develop', 'status operator branch');
  assertEqual(model.status.channel.operatorPromotionPath, 'develop -> main', 'status promotion path');
  assertEqual(model.status.support.platformTier, 'first-class', 'status platform tier');
  assertEqual(
    model.status.issueCapture.testerFacingChannelSupportLabel,
    'alpha · Linux first-class',
    'status issue-capture label'
  );
  assertEqual(
    model.status.issueCapture.releaseTruth?.officialSurface,
    'GitHub release assets published by .github/workflows/publish-tester-release.yml and consumed via apps/desktop/src/boundary/loadSd11UpdateAction.ts over the F3a fetch pipeline',
    'status issue-capture official surface'
  );
}

async function verifiesExplicitFallbackSurface() {
  const model = await loadSd11TesterWorkbenchSurface(
    {
      buildVersion: '0.0.0-test',
      platformLabel: 'Linux',
    },
    {
      loadSd12ReleaseTruth: async () => noOfficialReleaseTruth(),
      loadGe08AuthoringWorkbench: async () => {
        throw new Error('Tauri runtime not available for GE08 authoring workbench');
      },
      loadPilotShellSnapshot: async () => ({
        caseId: 'ge07-e1-scaffold-placeholder',
        sourcePackageId: 'pending-real-ge06-source-package',
        receiptStatus: 'Unknown/Unavailable',
        summaryValues: null,
        diagnostics: ['Real GE08 data could not load in this slice.'],
        explanationRefs: ['future/load_pilot_shell_snapshot'],
        dataSource: 'scaffold-placeholder',
        note: 'Placeholder fallback until a real bounded tester snapshot is available.',
      }),
    }
  );

  assertEqual(model.workflowName, 'GE07 pilot snapshot seam', 'fallback workflow name');
  assertEqual(model.dataTruthLabel, 'Explicit fallback placeholder', 'fallback data truth');
  assertEqual(model.summaryRows[0]?.label, 'Case', 'fallback summary row label');
  assertEqual(model.summaryRows[0]?.value, 'ge07-e1-scaffold-placeholder', 'fallback summary row value');
  assertEqual(model.diagnostics[0]?.classLabel, 'Fallback', 'fallback diagnostic class label');
  assertEqual(model.diagnostics[0]?.severityLabel, 'Warning', 'fallback diagnostic severity label');
  assertEqual(model.diagnostics[0]?.message, 'Real GE08 data could not load in this slice.', 'fallback diagnostic message');
  assertEqual(model.explanationRefs[0]?.label, 'future/load_pilot_shell_snapshot', 'fallback explanation ref label');
  assertEqual(
    model.explanationRefs[0]?.detail,
    'Fallback explanation reference preserved for later evidence capture.',
    'fallback explanation ref detail'
  );
  assertEqual(model.provenanceRefs.length, 0, 'fallback provenance refs');
  assertEqual(model.status.support.platformTier, 'first-class', 'fallback status platform tier');
  assertEqual(
    model.status.issueCapture.testerFacingChannelSupportLabel,
    'alpha · Linux first-class',
    'fallback status issue-capture label'
  );
  assertEqual(
    model.fallbackNotice,
    'GE08 authoring workbench unavailable: Tauri runtime not available for GE08 authoring workbench. This fallback exists because the real bounded snapshot could not load and the UI must not counterfeit product truth.',
    'fallback notice'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

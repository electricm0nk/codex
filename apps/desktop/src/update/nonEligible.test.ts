import { renderToStaticMarkup } from 'react-dom/server';
import { CheckPanel } from './CheckPanel';
import { InstallControl } from './InstallControl';
import {
  buildUnwiredUpdateDeps,
  emptyInstalledState,
  emptyLastCheckState,
  type EligibilityResult,
  type UpdateController,
  type UpdateControllerDeps,
  type InstallKind,
} from './updateModel';
import { assertEqual } from '../../testSupport/asserts';

function assertContains(actual: string, needle: string, message: string) {
  if (!actual.includes(needle)) {
    throw new Error(`${message}: expected markup to contain "${needle}"`);
  }
}

function fixedController(
  result: EligibilityResult,
  reason: string | null,
): UpdateController {
  return {
    async runCheck() {
      return;
    },
    computeEligibility() {
      return result;
    },
    disabledReason() {
      return reason;
    },
    releaseNotes() {
      return null;
    },
  };
}

function buildDepsForInstallKind(
  installKind: InstallKind,
  reason: string,
): UpdateControllerDeps {
  return {
    installed: { ...emptyInstalledState(), installKind },
    lastCheck: {
      ...emptyLastCheckState(),
      indexStatus: 'ok',
      manifestStatus: 'ok',
    },
    pendingRollback: {
      pendingUpdateState: 'idle',
      previousVersionAvailable: false,
      rollbackState: 'none',
      backupCount: 0,
      retainedUpdateStorageBytes: 0,
    },
    releaseNotes: null,
    controller: fixedController('ineligible', reason),
  };
}

function testDevBuildCannotInstallButCheckStillRuns() {
  const deps = buildDepsForInstallKind(
    'dev',
    'dev build is not update-eligible',
  );
  // Check button must remain enabled — the AV-UI-6 contract is that
  // non-eligible builds may still Check; only Install is gated.
  const checkHtml = renderToStaticMarkup(
    CheckPanel({ deps, checkInProgress: false, onCheck: () => undefined }),
  );
  assertEqual(
    checkHtml.includes('disabled=""'),
    false,
    'AV-UI-6: Check button must remain enabled for dev builds',
  );
  // Install must be disabled with the F1 closure's pinned reason string.
  const installHtml = renderToStaticMarkup(
    InstallControl({
      deps,
      installInProgress: false,
      onInstall: () => undefined,
    }),
  );
  assertContains(
    installHtml,
    'id="install-disabled-reason"',
    'AV-UI-6: dev build must surface #install-disabled-reason',
  );
  assertContains(
    installHtml,
    'dev build is not update-eligible',
    'AV-UI-6: dev build reason must match the F1 closure decision table',
  );
  assertContains(
    installHtml,
    'disabled=""',
    'AV-UI-6: dev build must keep Install disabled',
  );
}

function testTarballBuildCannotInstallButCheckStillRuns() {
  const deps = buildDepsForInstallKind(
    'tarball',
    'tarball install is not update-eligible',
  );
  const checkHtml = renderToStaticMarkup(
    CheckPanel({ deps, checkInProgress: false, onCheck: () => undefined }),
  );
  assertEqual(
    checkHtml.includes('disabled=""'),
    false,
    'AV-UI-6: Check button must remain enabled for tarball builds',
  );
  const installHtml = renderToStaticMarkup(
    InstallControl({
      deps,
      installInProgress: false,
      onInstall: () => undefined,
    }),
  );
  assertContains(
    installHtml,
    'tarball install is not update-eligible',
    'AV-UI-6: tarball build reason must match the F1 closure decision table',
  );
}

function testNonEligibleWiredUnwiredStillSurfacesReason() {
  // Even with the unwired default controller, Install is gated; the
  // reason surfaces the explicit "not wired yet" posture so reviewers
  // can confirm the gate is honest rather than fabricated eligibility.
  const html = renderToStaticMarkup(
    InstallControl({
      deps: { ...buildUnwiredUpdateDeps(), installed: { ...buildUnwiredUpdateDeps().installed, installKind: 'appimage' } },
      installInProgress: false,
      onInstall: () => undefined,
    }),
  );
  assertContains(
    html,
    'id="install-disabled-reason"',
    'AV-UI-6: even the unwired controller must surface a disabled reason',
  );
}

testDevBuildCannotInstallButCheckStillRuns();
testTarballBuildCannotInstallButCheckStillRuns();
testNonEligibleWiredUnwiredStillSurfacesReason();

console.log('nonEligible.test.ts: 3/3 assertions passed');

<<<<<<< HEAD
import { assertEqual } from '../../testSupport/asserts';
import { decideEligibility } from './eligibility';
import type { EligibilityInput } from './eligibility';

/**
 * Umbrella cross-cite for AV-UI-6: a single fixture that walks all five
 * ineligible context classes and asserts each resolves to `ineligible` with the
 * decision-table reason string verbatim. Non-update-eligible contexts may Check
 * but must never enable Install (fail-closed). F3b proves the rule surface here;
 * F3c proves the DOM gating cross-cite in its own test.
 */

function baseInput(): EligibilityInput {
  return {
    selectedChannel: 'alpha',
    manifest: { version: '2.0.0', artifact_sha256: 'sha-manifest' },
    installedState: {
      version: '1.0.0',
      artifact_sha256: 'sha-installed',
      install_kind: 'appimage',
      managed_executable_path: '/opt/codex/codex.AppImage',
      isManagedPathWritable: true,
    },
    fetchOutcomes: {
      indexStatus: 'ok',
      manifestStatus: 'ok',
      indexSchemaError: null,
      manifestSchemaError: null,
      indexFetchError: null,
      manifestFetchError: null,
=======
import { renderToStaticMarkup } from 'react-dom/server';
import { Sd16CheckPanel } from './CheckPanel';
import { Sd16InstallControl } from './InstallControl';
import {
  buildUnwiredUpdateDeps,
  emptyInstalledState,
  emptyLastCheckState,
  type Sd16EligibilityResult,
  type Sd16UpdateController,
  type Sd16UpdateControllerDeps,
  type Sd16InstallKind,
} from './updateModel';
import { assertEqual } from '../../testSupport/asserts';

function assertContains(actual: string, needle: string, message: string) {
  if (!actual.includes(needle)) {
    throw new Error(`${message}: expected markup to contain "${needle}"`);
  }
}

function fixedController(
  result: Sd16EligibilityResult,
  reason: string | null,
): Sd16UpdateController {
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
>>>>>>> origin/develop
    },
  };
}

<<<<<<< HEAD
interface IneligibleClass {
  label: string;
  mutate: (input: EligibilityInput) => void;
  reason: string;
}

const INELIGIBLE_CLASSES: IneligibleClass[] = [
  {
    label: 'dev/local build',
    mutate: (input) => {
      input.installedState.install_kind = 'dev';
    },
    reason: 'dev build is not update-eligible',
  },
  {
    label: 'non-AppImage (tarball) install',
    mutate: (input) => {
      input.installedState.install_kind = 'tarball';
    },
    reason: 'tarball install is not update-eligible',
  },
  {
    label: 'non-writable managed path',
    mutate: (input) => {
      input.installedState.isManagedPathWritable = false;
    },
    reason: 'managed executable path is not writable',
  },
  {
    label: 'manifest version not greater than installed',
    mutate: (input) => {
      input.manifest.version = '1.0.0';
    },
    reason: 'installed version is at or above manifest version',
  },
  {
    label: 'artifact hash already matches installed',
    mutate: (input) => {
      input.manifest.artifact_sha256 = 'sha-shared';
      input.installedState.artifact_sha256 = 'sha-shared';
    },
    reason: 'installed artifact hash already matches manifest',
  },
];

function main() {
  assertEqual(INELIGIBLE_CLASSES.length, 5, 'exactly five ineligible context classes');
  for (const klass of INELIGIBLE_CLASSES) {
    const input = baseInput();
    klass.mutate(input);
    const decision = decideEligibility(input);
    assertEqual(decision.result, 'ineligible', `${klass.label} is ineligible`);
    assertEqual(
      decision.install_disabled_reason,
      klass.reason,
      `${klass.label} reason string`
    );
  }
  console.log('nonEligible.test.ts: all five ineligible classes passed');
}

main();
=======
function buildDepsForInstallKind(
  installKind: Sd16InstallKind,
  reason: string,
): Sd16UpdateControllerDeps {
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
    Sd16CheckPanel({ deps, checkInProgress: false, onCheck: () => undefined }),
  );
  assertEqual(
    checkHtml.includes('disabled=""'),
    false,
    'AV-UI-6: Check button must remain enabled for dev builds',
  );
  // Install must be disabled with the F1 closure's pinned reason string.
  const installHtml = renderToStaticMarkup(
    Sd16InstallControl({
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
    Sd16CheckPanel({ deps, checkInProgress: false, onCheck: () => undefined }),
  );
  assertEqual(
    checkHtml.includes('disabled=""'),
    false,
    'AV-UI-6: Check button must remain enabled for tarball builds',
  );
  const installHtml = renderToStaticMarkup(
    Sd16InstallControl({
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
    Sd16InstallControl({
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
>>>>>>> origin/develop

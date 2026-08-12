import { renderToStaticMarkup } from 'react-dom/server';
import { InstallControl } from './InstallControl';
import {
  buildUnwiredUpdateDeps,
  emptyLastCheckState,
  UNWIRED_CONTROLLER,
} from './updateModel';
import type {
  EligibilityResult,
  InstalledState,
  LastCheckState,
  UpdateController,
  UpdateControllerDeps,
} from './updateModel';
import { assert, assertEqual } from '../testSupport/asserts';

function assertContains(actual: string, needle: string, message: string) {
  if (!actual.includes(needle)) {
    throw new Error(`${message}: expected markup to contain "${needle}"`);
  }
}

function render(props: {
  deps: UpdateControllerDeps;
  installInProgress?: boolean;
}): string {
  return renderToStaticMarkup(
    InstallControl({
      deps: props.deps,
      installInProgress: props.installInProgress ?? false,
      onInstall: () => undefined,
    }),
  );
}

function eligibilityController(
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

function testInstallButtonDisabledWhenEligibilityUnknown() {
  const html = render({ deps: buildUnwiredUpdateDeps() });
  assertContains(
    html,
    'id="install-button"',
    'AV-UI-4: Install button must have id="install-button"',
  );
  assertContains(
    html,
    'disabled=""',
    'AV-UI-4: Install button must be disabled when eligibility is unknown',
  );
}

function testDisabledReasonPopulatedWhenIneligible() {
  const deps: UpdateControllerDeps = {
    ...buildUnwiredUpdateDeps(),
    controller: eligibilityController(
      'ineligible',
      'dev build is not update-eligible',
    ),
    installed: { ...buildUnwiredUpdateDeps().installed, installKind: 'dev' },
  };
  const html = render({ deps });
  assertContains(
    html,
    'id="install-disabled-reason"',
    'AV-UI-5: disabled-reason badge must have id="install-disabled-reason"',
  );
  assertContains(
    html,
    'dev build is not update-eligible',
    'AV-UI-5: disabled reason must be populated with the F1 closure decision-table string',
  );
}

function testDisabledReasonPopulatedEvenWhenReasonIsNull() {
  const deps: UpdateControllerDeps = {
    ...buildUnwiredUpdateDeps(),
    controller: eligibilityController('ineligible', null),
  };
  const html = render({ deps });
  assertContains(
    html,
    'id="install-disabled-reason"',
    'AV-UI-5: disabled reason hook must exist even when reason is null',
  );
  assertContains(
    html,
    'Install is unavailable for this build',
    'AV-UI-5: a deterministic fallback reason must populate the hook when controller returns null',
  );
}

function testInstallButtonEnabledOnlyWhenEligible() {
  const deps: UpdateControllerDeps = {
    ...buildUnwiredUpdateDeps(),
    controller: eligibilityController('eligible', null),
    installed: {
      ...buildUnwiredUpdateDeps().installed,
      updateEligible: true,
      ineligibleReason: null,
    },
    lastCheck: {
      ...emptyLastCheckState(),
      eligibilityResult: 'eligible',
      installDisabledReason: null,
    },
  };
  const html = render({ deps });
  assertEqual(
    html.includes('disabled=""'),
    false,
    'AV-UI-4: Install button must NOT be disabled when eligibility_result is "eligible"',
  );
  assertContains(
    html,
    'data-eligible="true"',
    'AV-UI-4: Install button must carry data-eligible="true" when eligible',
  );
}

function testInstallButtonDisabledWhenInstallInProgress() {
  const deps: UpdateControllerDeps = {
    ...buildUnwiredUpdateDeps(),
    controller: eligibilityController('eligible', null),
  };
  const html = render({ deps, installInProgress: true });
  assertContains(
    html,
    'disabled=""',
    'AV-UI-4: Install button must be disabled while install is in progress',
  );
  assertContains(
    html,
    'Installing…',
    'AV-UI-4: button label must read "Installing…" while busy',
  );
}

testInstallButtonDisabledWhenEligibilityUnknown();
testDisabledReasonPopulatedWhenIneligible();
testDisabledReasonPopulatedEvenWhenReasonIsNull();
testInstallButtonEnabledOnlyWhenEligible();
testInstallButtonDisabledWhenInstallInProgress();

console.log('installEnablement.test.ts: 5/5 assertions passed');

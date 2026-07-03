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
    },
  };
}

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

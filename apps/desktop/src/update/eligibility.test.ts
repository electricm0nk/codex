import { assert, assertEqual } from '../testSupport/asserts';
import { compareVersions, decideEligibility } from './eligibility';
import type { EligibilityInput } from './eligibility';

/**
 * Exercises the eligibility decision table verbatim (F1 closure). Each
 * row asserts both the `result` and the exact `install_disabled_reason` string,
 * and the ordering guarantee (first match wins) is asserted explicitly.
 */

function eligibleInput(): EligibilityInput {
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

function main() {
  verifiesHappyPathEligible();
  verifiesDevIneligible();
  verifiesTarballIneligible();
  verifiesNonWritablePathIneligible();
  verifiesDebEligibleDespiteRootOwnedPath();
  verifiesDebWithoutManifestArtifactIneligible();
  verifiesWindowsInstallsUpdateFromTheInstallerNotByHash();
  verifiesVersionNotGreaterIneligible();
  verifiesVersionEqualIneligible();
  verifiesHashMatchIneligible();
  verifiesIndexFetchFailedUnknown();
  verifiesManifestFetchFailedUnknown();
  verifiesIndexSchemaInvalidUnknown();
  verifiesManifestSchemaInvalidUnknown();
  verifiesDevBeatsStaleVersion();
  verifiesFetchFailureBeatsDev();
  verifiesCompareVersions();
  console.log('eligibility.test.ts: all assertions passed');
}

function verifiesHappyPathEligible() {
  const decision = decideEligibility(eligibleInput());
  assertEqual(decision.result, 'eligible', 'happy path is eligible');
  assertEqual(decision.install_disabled_reason, null, 'happy path has no disabled reason');
}

function verifiesDebEligibleDespiteRootOwnedPath() {
  const input = eligibleInput();
  input.installedState.install_kind = 'deb';
  input.installedState.managed_executable_path = '/usr/bin/codex-desktop';
  input.installedState.isManagedPathWritable = false;
  // The installed hash is the binary's, the manifest's is the .deb's; they are not comparable.
  input.installedState.artifact_sha256 = input.manifest.artifact_sha256 as string;
  const decision = decideEligibility(input);
  assertEqual(decision.result, 'eligible', 'deb installs update through the package manager, not a writable path');
}

function verifiesDebWithoutManifestArtifactIneligible() {
  const input = eligibleInput();
  input.installedState.install_kind = 'deb';
  input.manifest.artifact_sha256 = null;
  const decision = decideEligibility(input);
  assertEqual(decision.result, 'ineligible', 'no .deb in the release is ineligible');
  assertEqual(
    decision.install_disabled_reason,
    'this release publishes no .deb artifact for a deb install',
    'missing deb artifact reason string',
  );
}

function verifiesWindowsInstallsUpdateFromTheInstallerNotByHash() {
  const windows = (): EligibilityInput => {
    const input = eligibleInput();
    input.installedState.install_kind = 'windows';
    input.installedState.managed_executable_path = 'C:\\Users\\u\\AppData\\Local\\Codex\\codex-desktop.exe';
    return input;
  };
  const eligible = windows();
  // The installed hash is the .exe's, the manifest's is the installer's; they are not comparable.
  eligible.installedState.artifact_sha256 = eligible.manifest.artifact_sha256 as string;
  assertEqual(decideEligibility(eligible).result, 'eligible', 'a newer release with an installer is installable');

  const unwritable = windows();
  unwritable.installedState.isManagedPathWritable = false;
  assertEqual(decideEligibility(unwritable).install_disabled_reason, 'managed executable path is not writable', 'the installer must be able to replace the install');

  const none = windows();
  none.manifest.artifact_sha256 = null;
  assertEqual(decideEligibility(none).result, 'ineligible', 'no installer in the release');
  assertEqual(decideEligibility(none).install_disabled_reason, 'this release publishes no Windows installer for a Windows install', 'names the missing installer');

  const same = windows();
  same.manifest.version = same.installedState.version;
  assertEqual(decideEligibility(same).result, 'ineligible', 'the version must be newer');
}

function verifiesDevIneligible() {
  const input = eligibleInput();
  input.installedState.install_kind = 'dev';
  const decision = decideEligibility(input);
  assertEqual(decision.result, 'ineligible', 'dev build ineligible');
  assertEqual(
    decision.install_disabled_reason,
    'dev build is not update-eligible',
    'dev reason string'
  );
}

function verifiesTarballIneligible() {
  const input = eligibleInput();
  input.installedState.install_kind = 'tarball';
  const decision = decideEligibility(input);
  assertEqual(decision.result, 'ineligible', 'tarball install ineligible');
  assertEqual(
    decision.install_disabled_reason,
    'tarball install is not update-eligible',
    'tarball reason string'
  );
}

function verifiesNonWritablePathIneligible() {
  const input = eligibleInput();
  input.installedState.isManagedPathWritable = false;
  const decision = decideEligibility(input);
  assertEqual(decision.result, 'ineligible', 'non-writable path ineligible');
  assertEqual(
    decision.install_disabled_reason,
    'managed executable path is not writable',
    'non-writable reason string'
  );
}

function verifiesVersionNotGreaterIneligible() {
  const input = eligibleInput();
  input.manifest.version = '0.9.0';
  const decision = decideEligibility(input);
  assertEqual(decision.result, 'ineligible', 'older manifest ineligible');
  assertEqual(
    decision.install_disabled_reason,
    'installed version is at or above manifest version',
    'version reason string'
  );
}

function verifiesVersionEqualIneligible() {
  const input = eligibleInput();
  input.manifest.version = '1.0.0';
  const decision = decideEligibility(input);
  assertEqual(decision.result, 'ineligible', 'equal manifest ineligible');
  assertEqual(
    decision.install_disabled_reason,
    'installed version is at or above manifest version',
    'equal version reason string'
  );
}

function verifiesHashMatchIneligible() {
  const input = eligibleInput();
  input.manifest.artifact_sha256 = 'sha-shared';
  input.installedState.artifact_sha256 = 'sha-shared';
  const decision = decideEligibility(input);
  assertEqual(decision.result, 'ineligible', 'hash match ineligible');
  assertEqual(
    decision.install_disabled_reason,
    'installed artifact hash already matches manifest',
    'hash reason string'
  );
}

function verifiesIndexFetchFailedUnknown() {
  const input = eligibleInput();
  input.fetchOutcomes.indexStatus = 'failed';
  const decision = decideEligibility(input);
  assertEqual(decision.result, 'unknown', 'index fetch failure is unknown');
  assertEqual(
    decision.install_disabled_reason,
    'channel index fetch failed: failed',
    'index fetch reason string'
  );
}

function verifiesManifestFetchFailedUnknown() {
  const input = eligibleInput();
  input.fetchOutcomes.manifestStatus = 'failed';
  const decision = decideEligibility(input);
  assertEqual(decision.result, 'unknown', 'manifest fetch failure is unknown');
  assertEqual(
    decision.install_disabled_reason,
    'manifest fetch failed: failed',
    'manifest fetch reason string'
  );
}

function verifiesIndexSchemaInvalidUnknown() {
  const input = eligibleInput();
  input.fetchOutcomes.indexSchemaError = 'missing manifest_url';
  const decision = decideEligibility(input);
  assertEqual(decision.result, 'unknown', 'index schema error is unknown');
  assertEqual(
    decision.install_disabled_reason,
    'channel-index.schema.json: missing manifest_url',
    'index schema reason string'
  );
}

function verifiesManifestSchemaInvalidUnknown() {
  const input = eligibleInput();
  input.fetchOutcomes.manifestSchemaError = 'version is not a string';
  const decision = decideEligibility(input);
  assertEqual(decision.result, 'unknown', 'manifest schema error is unknown');
  assertEqual(
    decision.install_disabled_reason,
    'update-manifest.schema.json: version is not a string',
    'manifest schema reason string'
  );
}

function verifiesDevBeatsStaleVersion() {
  // Rule 5 (dev) must fire before rule 8 (version) — ordering guarantee.
  const input = eligibleInput();
  input.installedState.install_kind = 'dev';
  input.manifest.version = '1.0.0'; // also a stale-version condition
  const decision = decideEligibility(input);
  assertEqual(
    decision.install_disabled_reason,
    'dev build is not update-eligible',
    'dev reason beats stale-version reason'
  );
}

function verifiesFetchFailureBeatsDev() {
  // Rule 1 (index fetch) must fire before rule 5 (dev) — unknown outranks ineligible.
  const input = eligibleInput();
  input.installedState.install_kind = 'dev';
  input.fetchOutcomes.indexStatus = 'failed';
  const decision = decideEligibility(input);
  assertEqual(decision.result, 'unknown', 'fetch failure outranks dev');
  assertEqual(
    decision.install_disabled_reason,
    'channel index fetch failed: failed',
    'fetch reason beats dev reason'
  );
}

function verifiesCompareVersions() {
  assert(compareVersions('2.0.0', '1.0.0') > 0, '2.0.0 > 1.0.0');
  assert(compareVersions('1.0.0', '2.0.0') < 0, '1.0.0 < 2.0.0');
  assertEqual(compareVersions('1.0.0', '1.0.0'), 0, '1.0.0 == 1.0.0');
  assertEqual(compareVersions('1.0', '1.0.0'), 0, '1.0 == 1.0.0 (trailing zero)');
  assert(compareVersions('1.10.0', '1.9.0') > 0, '1.10.0 > 1.9.0 numerically, not lexically');
}

main();

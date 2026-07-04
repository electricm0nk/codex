<<<<<<< HEAD
import { assert, assertEqual } from '../../testSupport/asserts';
import { defaultInstalledDiagnostics } from './diagnostics';
import type { InstalledDiagnostics } from './diagnostics';

/**
 * AV-DIAG-1 cross-cite (F3b owns the `installed.*` model shape). Field-presence
 * only: every documented key exists with the documented type, defaulting to a
 * deterministic sentinel. F3c renders these keys into `#installed-panel`.
 */

const EXPECTED_KEYS = [
  'channel',
  'version',
  'source_commit',
  'artifact_sha256',
  'install_kind',
  'managed_executable_path',
  'update_eligible',
  'ineligible_reason',
];

function main() {
  const model: InstalledDiagnostics = defaultInstalledDiagnostics();

  assertEqual(
    Object.keys(model).sort().join(','),
    [...EXPECTED_KEYS].sort().join(','),
    'installed diagnostics exposes exactly the eight documented keys'
  );

  assertEqual(typeof model.channel, 'string', 'channel is a string label');
  assertEqual(typeof model.version, 'string', 'version is a string');
  assertEqual(typeof model.source_commit, 'string', 'source_commit is a string');
  assertEqual(typeof model.artifact_sha256, 'string', 'artifact_sha256 is a string');
  assertEqual(typeof model.install_kind, 'string', 'install_kind is a string label');
  assert(
    model.managed_executable_path === null || typeof model.managed_executable_path === 'string',
    'managed_executable_path is string | null'
  );
  assertEqual(typeof model.update_eligible, 'boolean', 'update_eligible is a boolean');
  assert(
    model.ineligible_reason === null || typeof model.ineligible_reason === 'string',
    'ineligible_reason is string | null'
  );

  // Deterministic sentinels when E7 has not landed.
  assertEqual(model.version, '', 'version defaults to empty string');
  assertEqual(model.install_kind, 'unknown', 'install_kind defaults to the unknown sentinel');
  assertEqual(model.managed_executable_path, null, 'managed_executable_path defaults to null');
  assertEqual(model.update_eligible, false, 'update_eligible defaults to false (fail-closed)');
  assertEqual(model.ineligible_reason, null, 'ineligible_reason defaults to null');

  console.log('installedPanel.test.ts: installed.* model shape verified');
}

main();
=======
import { renderToStaticMarkup } from 'react-dom/server';
import { Sd16InstalledPanel, INSTALLED_PANEL_ID } from './installedPanel';
import {
  buildUnwiredUpdateDeps,
  emptyInstalledState,
  type Sd16UpdateControllerDeps,
} from './updateModel';
import type { Sd16InstallKind } from './updateModel';
import { assert, assertEqual } from '../../testSupport/asserts';

function assertContains(actual: string, needle: string, message: string) {
  if (!actual.includes(needle)) {
    throw new Error(`${message}: expected markup to contain "${needle}"`);
  }
}

function render(deps: Sd16UpdateControllerDeps): string {
  return renderToStaticMarkup(Sd16InstalledPanel({ deps }));
}

function testPanelRendersWithCanonicalId() {
  const html = render(buildUnwiredUpdateDeps());
  assertContains(
    html,
    `id="${INSTALLED_PANEL_ID}"`,
    'AV-DIAG-1: installed panel must render with the canonical id="#installed-panel"',
  );
}

function testAllEightInstalledFieldsExposed() {
  const html = render(buildUnwiredUpdateDeps());
  const expectedRows = [
    'channel',
    'version',
    'source-commit',
    'artifact-sha-256',
    'install-kind',
    'managed-path',
    'update-eligible',
    'ineligible-reason',
  ];
  for (const needle of expectedRows) {
    assertContains(
      html,
      `data-installed-row="${needle}"`,
      `AV-DIAG-1: installed panel must expose row for "${needle}"`,
    );
  }
}

function testFieldValuesReflectSuppliedState() {
  const deps: Sd16UpdateControllerDeps = {
    ...buildUnwiredUpdateDeps(),
    installed: {
      channel: 'beta',
      version: '1.4.0',
      sourceCommit: 'abcdef1234567890',
      artifactSha256:
        '0000000000000000000000000000000000000000000000000000000000000000',
      installKind: 'appimage' as Sd16InstallKind,
      managedExecutablePath: '/opt/codex/codex.AppImage',
      updateEligible: true,
      ineligibleReason: null,
    },
  };
  const html = render(deps);
  assertContains(html, '>beta<', 'AV-DIAG-1: channel value is rendered');
  assertContains(html, '>1.4.0<', 'AV-DIAG-1: version value is rendered');
  assertContains(
    html,
    'abcdef1234567890',
    'AV-DIAG-1: source commit value is rendered',
  );
  assertContains(
    html,
    '0000000000000000000000000000000000000000000000000000000000000000',
    'AV-DIAG-1: artifact SHA-256 value is rendered',
  );
  assertContains(html, '>appimage<', 'AV-DIAG-1: install kind value is rendered');
  assertContains(
    html,
    '/opt/codex/codex.AppImage',
    'AV-DIAG-1: managed executable path is rendered',
  );
}

function testNullFieldsFallBackToUnknown() {
  const deps: Sd16UpdateControllerDeps = {
    ...buildUnwiredUpdateDeps(),
    installed: emptyInstalledState(),
  };
  const html = render(deps);
  assertEqual(
    html.includes('>unknown<'),
    true,
    'AV-DIAG-1: null fields must surface the "unknown" placeholder',
  );
}

testPanelRendersWithCanonicalId();
testAllEightInstalledFieldsExposed();
testFieldValuesReflectSuppliedState();
testNullFieldsFallBackToUnknown();

console.log('installedPanel.test.ts: 4/4 assertions passed');
>>>>>>> origin/develop

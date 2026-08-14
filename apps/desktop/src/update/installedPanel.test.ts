import { renderToStaticMarkup } from 'react-dom/server';
import { InstalledPanel, INSTALLED_PANEL_ID } from './installedPanel';
import {
  buildUnwiredUpdateDeps,
  emptyInstalledState,
  type UpdateControllerDeps,
} from './updateModel';
import type { InstallKind } from './updateModel';
import { assert, assertEqual } from '../testSupport/asserts';

function assertContains(actual: string, needle: string, message: string) {
  if (!actual.includes(needle)) {
    throw new Error(`${message}: expected markup to contain "${needle}"`);
  }
}

function render(deps: UpdateControllerDeps): string {
  return renderToStaticMarkup(InstalledPanel({ deps }));
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
  const deps: UpdateControllerDeps = {
    ...buildUnwiredUpdateDeps(),
    installed: {
      channel: 'beta',
      version: '1.4.0',
      sourceCommit: 'abcdef1234567890',
      artifactSha256:
        '0000000000000000000000000000000000000000000000000000000000000000',
      installKind: 'appimage' as InstallKind,
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
  const deps: UpdateControllerDeps = {
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

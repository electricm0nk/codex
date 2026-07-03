import { renderToStaticMarkup } from 'react-dom/server';
import {
  Sd16PendingRollbackPanel,
  PENDING_ROLLBACK_PANEL_ID,
} from './pendingRollbackPanel';
import {
  buildUnwiredUpdateDeps,
  emptyPendingRollbackState,
  type Sd16UpdateControllerDeps,
  type Sd16PendingRollbackState,
} from './updateModel';
import { assert, assertEqual } from '../../testSupport/asserts';

function assertContains(actual: string, needle: string, message: string) {
  if (!actual.includes(needle)) {
    throw new Error(`${message}: expected markup to contain "${needle}"`);
  }
}

function render(deps: Sd16UpdateControllerDeps): string {
  return renderToStaticMarkup(Sd16PendingRollbackPanel({ deps }));
}

function testPanelRendersWithCanonicalId() {
  const html = render(buildUnwiredUpdateDeps());
  assertContains(
    html,
    `id="${PENDING_ROLLBACK_PANEL_ID}"`,
    'AV-DIAG-3: pending-rollback panel must render with id="#pending-rollback-panel"',
  );
}

function testAllFivePendingRollbackFieldsExposed() {
  const html = render(buildUnwiredUpdateDeps());
  const expectedRows = [
    'pending-update-state',
    'previous-version-available',
    'rollback-state',
    'backup-count',
    'retained-update-bytes',
  ];
  for (const needle of expectedRows) {
    assertContains(
      html,
      `data-pending-rollback-row="${needle}"`,
      `AV-DIAG-3: pending-rollback panel must expose row for "${needle}"`,
    );
  }
}

function testE7PlaceholderNoteIsVisibleUntilE7Lands() {
  const html = render(buildUnwiredUpdateDeps());
  assertContains(
    html,
    'id="sd16-pending-rollback-source-note"',
    'AV-DIAG-3: panel must surface the E7 carryover note anchor',
  );
  assertContains(
    html,
    'populated by the SD-16-E7 staged-transaction module',
    'AV-DIAG-3: carryover note must mention the E7 staged-transaction module',
  );
}

function testSuppliedStateRendersValues() {
  const pendingRollback: Sd16PendingRollbackState = {
    pendingUpdateState: 'pending-relaunch',
    previousVersionAvailable: true,
    rollbackState: 'available',
    backupCount: 2,
    retainedUpdateStorageBytes: 134217728,
  };
  const deps: Sd16UpdateControllerDeps = {
    ...buildUnwiredUpdateDeps(),
    pendingRollback,
  };
  const html = render(deps);
  assertContains(
    html,
    '>pending-relaunch<',
    'AV-DIAG-3: pending update state value is rendered',
  );
  assertContains(
    html,
    '>yes<',
    'AV-DIAG-3: previous-version-available value is rendered as yes',
  );
  assertContains(
    html,
    '>available<',
    'AV-DIAG-3: rollback state value is rendered',
  );
  assertContains(html, '>2<', 'AV-DIAG-3: backup count value is rendered');
  assertContains(
    html,
    '>134217728<',
    'AV-DIAG-3: retained update bytes value is rendered',
  );
}

function testEmptyStateSurfacesDeterministicPlaceholders() {
  const html = render({
    ...buildUnwiredUpdateDeps(),
    pendingRollback: emptyPendingRollbackState(),
  });
  assertEqual(
    html.includes('>unknown<'),
    true,
    'AV-DIAG-3: empty pending/rollback state must surface "unknown" placeholders',
  );
  assertEqual(
    html.includes('>0<'),
    true,
    'AV-DIAG-3: empty counts must surface numeric zero placeholders',
  );
}

testPanelRendersWithCanonicalId();
testAllFivePendingRollbackFieldsExposed();
testE7PlaceholderNoteIsVisibleUntilE7Lands();
testSuppliedStateRendersValues();
testEmptyStateSurfacesDeterministicPlaceholders();

console.log('pendingRollbackPanel.test.ts: 5/5 assertions passed');

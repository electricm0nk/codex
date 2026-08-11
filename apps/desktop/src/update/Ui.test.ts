import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { UpdateUi, UPDATE_UI_ID } from './Ui';
import { RESTORE_OFFER_ID } from './restoreOffer';
import { buildUnwiredUpdateDeps } from './updateModel';
import { assert } from '../../testSupport/asserts';

function assertContains(actual: string, needle: string, message: string) {
  if (!actual.includes(needle)) {
    throw new Error(`${message}: expected markup to contain "${needle}"`);
  }
}

function assertNotContains(actual: string, needle: string, message: string) {
  if (actual.includes(needle)) {
    throw new Error(`${message}: expected markup NOT to contain "${needle}"`);
  }
}

function render(props: Parameters<typeof UpdateUi>[0]): string {
  return renderToStaticMarkup(createElement(UpdateUi, props));
}

function testMountsWithCanonicalIdAndAllSubPanels() {
  const html = render({ initialDeps: buildUnwiredUpdateDeps() });
  assertContains(html, `id="${UPDATE_UI_ID}"`, 'page must render with the canonical UpdateUi id');
  assertContains(html, 'id="check-panel"', 'check panel must be mounted');
  assertContains(html, 'id="install-panel"', 'install panel must be mounted');
  assertContains(html, 'id="installed-panel"', 'installed panel must be mounted');
  assertContains(html, 'id="pending-rollback-panel"', 'pending/rollback panel must be mounted');
}

function testNoRestoreOfferRenderedWhenNotSupplied() {
  const html = render({ initialDeps: buildUnwiredUpdateDeps() });
  assertNotContains(html, `id="${RESTORE_OFFER_ID}"`, 'restore offer must not render unless supplied');
}

function testRestoreOfferAndButtonRenderWhenAvailable() {
  const html = render({
    initialDeps: buildUnwiredUpdateDeps(),
    restoreOffer: {
      priorVersion: 'unknown',
      restoreAvailable: true,
      onRestore: async () => {},
    },
  });
  assertContains(html, `id="${RESTORE_OFFER_ID}"`, 'restore offer must render when supplied');
  assertContains(
    html,
    'id="restore-previous-button"',
    'an actionable restore button must render when restoreAvailable is true',
  );
}

function testRestoreOfferWithoutButtonWhenUnavailable() {
  const html = render({
    initialDeps: buildUnwiredUpdateDeps(),
    restoreOffer: {
      priorVersion: 'unknown',
      restoreAvailable: false,
      onRestore: async () => {},
    },
  });
  assertContains(html, `id="${RESTORE_OFFER_ID}"`, 'restore offer must still render its notice');
  assertNotContains(
    html,
    'id="restore-previous-button"',
    'no actionable button when restoreAvailable is false',
  );
}

function main() {
  testMountsWithCanonicalIdAndAllSubPanels();
  testNoRestoreOfferRenderedWhenNotSupplied();
  testRestoreOfferAndButtonRenderWhenAvailable();
  testRestoreOfferWithoutButtonWhenUnavailable();
  assert(true, 'all Ui.tsx mount assertions passed');
  console.log('Ui.test.ts: all assertions passed');
}

main();

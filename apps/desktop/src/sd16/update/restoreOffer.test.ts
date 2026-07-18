import { renderToStaticMarkup } from 'react-dom/server';
import {
  RestoreOffer,
  RESTORE_OFFER_ID,
  buildRestoreOfferMarkup,
} from './restoreOffer';
import { assert } from '../../testSupport/asserts';

function assertContains(actual: string, needle: string, message: string) {
  if (!actual.includes(needle)) {
    throw new Error(`${message}: expected markup to contain "${needle}"`);
  }
}

function renderRestoreOffer(priorVersion: string, restoreAvailable: boolean): string {
  return renderToStaticMarkup(
    RestoreOffer({ priorVersion, restoreAvailable }),
  );
}

function testRendersWithCanonicalRestorePriorId() {
  const html = renderRestoreOffer('0.0.0', true);
  assertContains(
    html,
    `id="${RESTORE_OFFER_ID}"`,
    'AV-RB-2: restore-offer panel must render with id="#restore-prior"',
  );
}

function testExposesPriorVersionAsDataAttribute() {
  const html = renderRestoreOffer('0.0.0', true);
  assertContains(
    html,
    'data-prior-version="0.0.0"',
    'restore-offer panel must expose data-prior-version for F3b integration',
  );
}

function testMarksRestoreAvailableWhenWired() {
  const html = renderRestoreOffer('0.0.0', true);
  assertContains(
    html,
    'data-restore-available="true"',
    'restore-offer panel must mark restoreAvailable=true when wired',
  );
  assertContains(
    html,
    'Restore previous version: 0.0.0',
    'restore-offer panel must show the wired restore copy when wired',
  );
}

function testMarksRestoreUnavailableWhenF3bNotYetWired() {
  const html = renderRestoreOffer('0.0.0', false);
  assertContains(
    html,
    'data-restore-available="false"',
    'restore-offer panel must mark restoreAvailable=false until F3b lands',
  );
  assertContains(
    html,
    'restore mechanism not yet wired by F3b',
    'restore-offer panel must explain why the restore is not wired',
  );
}

function testBuildRestoreOfferMarkupMatchesReactRenderContract() {
  const props = { priorVersion: '0.0.7', restoreAvailable: true };
  const reactHtml = renderToStaticMarkup(RestoreOffer(props));
  const pureHtml = buildRestoreOfferMarkup(props);
  // The pure renderer mirrors the React renderer's *contract* (id, data-testid,
  // data-prior-version, data-restore-available, visible text). React adds a
  // serialized `style="..."` attribute that the pure renderer intentionally
  // omits to keep the markup readable; we assert on the contract seam, not
  // on byte-for-byte equality.
  const contractFragments = [
    `id="${RESTORE_OFFER_ID}"`,
    'data-testid="sd16-restore-offer"',
    'data-prior-version="0.0.7"',
    'data-restore-available="true"',
    'Restore previous version: 0.0.7',
  ];
  for (const fragment of contractFragments) {
    assert(
      reactHtml.includes(fragment),
      `React render must carry contract fragment \`${fragment}\` — got: ${reactHtml}`,
    );
    assert(
      pureHtml.includes(fragment),
      `pure renderer must carry contract fragment \`${fragment}\` — got: ${pureHtml}`,
    );
  }
}

function testHandlesEmptyPriorVersionWithoutCrashing() {
  const html = renderRestoreOffer('', false);
  assertContains(
    html,
    `id="${RESTORE_OFFER_ID}"`,
    'restore-offer panel must render even with an empty priorVersion',
  );
}

function main(): void {
  testRendersWithCanonicalRestorePriorId();
  testExposesPriorVersionAsDataAttribute();
  testMarksRestoreAvailableWhenWired();
  testMarksRestoreUnavailableWhenF3bNotYetWired();
  testBuildRestoreOfferMarkupMatchesReactRenderContract();
  testHandlesEmptyPriorVersionWithoutCrashing();
  assert(true, 'all restoreOffer assertions passed');
  console.log('restoreOffer.test.ts: all assertions passed');
}

main();
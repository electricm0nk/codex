/**
 * The shell must never claim submission without proof.
 *
 * Self-executing tsx test (auto-discovered by scripts/run-tests.mjs). Uses the
 * shared testSupport/asserts helpers (the repo idiom; node builtins are not in
 * the tsconfig lib set) plus a local structural deep-equal for the
 * discriminated-union states. The load-bearing invariant under test: `confirmed`
 * is reachable ONLY via `BROWSER_OPENED` carrying a non-empty URL, and
 * `canClaimSubmitted` returns `true` ONLY on that `confirmed(url)` state.
 */
import { assert, assertEqual } from '../testSupport/asserts';
import {
  canClaimSubmitted,
  reduceSubmissionState,
  type SubmissionEvent,
  type SubmissionState,
} from './submissionState';

const CONFIRMED_URL =
  'https://github.com/electricm0nk/codex/issues/new?title=Shell%20defect';

function isDeepEqual(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (typeof a !== 'object' || a === null || typeof b !== 'object' || b === null) {
    return false;
  }
  const aKeys = Object.keys(a as Record<string, unknown>);
  const bKeys = Object.keys(b as Record<string, unknown>);
  if (aKeys.length !== bKeys.length) return false;
  return aKeys.every(
    (k) =>
      Object.prototype.hasOwnProperty.call(b, k) &&
      isDeepEqual((a as Record<string, unknown>)[k], (b as Record<string, unknown>)[k]),
  );
}

function assertStateEqual(actual: SubmissionState, expected: SubmissionState, message: string) {
  assert(
    isDeepEqual(actual, expected),
    `${message}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`,
  );
}

// Positive case: idle → opening → awaiting-issue-url → confirmed, the
// ONLY path that reaches confirmed, driven by BROWSER_OPENED with a real URL.
function happyPathReachesConfirmedViaBrowserOpened() {
  let s: SubmissionState = { kind: 'idle' };
  s = reduceSubmissionState(s, { type: 'OPEN', title: 'Shell defect' });
  assertStateEqual(s, { kind: 'opening', title: 'Shell defect' }, 'OPEN from idle');
  s = reduceSubmissionState(s, { type: 'URL_BUILT', title: 'Shell defect' });
  assertStateEqual(s, { kind: 'awaiting-issue-url', title: 'Shell defect' }, 'URL_BUILT from opening');
  s = reduceSubmissionState(s, { type: 'BROWSER_OPENED', url: CONFIRMED_URL });
  assertStateEqual(s, { kind: 'confirmed', url: CONFIRMED_URL }, 'BROWSER_OPENED from awaiting');
}

// Negative case: from awaiting-issue-url, no event other than a
// non-empty BROWSER_OPENED may reach confirmed.
function noPathToConfirmedWithoutBrowserOpened() {
  let s: SubmissionState = { kind: 'idle' };
  s = reduceSubmissionState(s, { type: 'OPEN', title: 'x' });
  s = reduceSubmissionState(s, { type: 'URL_BUILT', title: 'x' });
  assertEqual(s.kind, 'awaiting-issue-url', 'setup must land in awaiting-issue-url');

  const nonConfirming: SubmissionEvent[] = [
    { type: 'OPEN', title: 'x' },
    { type: 'URL_BUILT', title: 'x' },
    { type: 'BROWSER_FAILED', reason: 'os-shell-error' },
    { type: 'BROWSER_OPENED', url: '' },
    { type: 'RESET' },
  ];
  for (const ev of nonConfirming) {
    const next = reduceSubmissionState({ kind: 'awaiting-issue-url', title: 'x' }, ev);
    assert(next.kind !== 'confirmed', `event ${ev.type} must not reach confirmed`);
  }
}

// Negative case: an empty URL is not proof — it lands in failed(empty-url).
function browserOpenedWithEmptyUrlLandsInFailed() {
  const s = reduceSubmissionState(
    { kind: 'awaiting-issue-url', title: 'x' },
    { type: 'BROWSER_OPENED', url: '' },
  );
  assertStateEqual(s, { kind: 'failed', reason: 'empty-url' }, 'empty-url BROWSER_OPENED');
}

// Negative case: a browser failure from opening lands in failed.
function browserFailedFromOpeningLandsInFailed() {
  const s = reduceSubmissionState(
    { kind: 'opening', title: 'x' },
    { type: 'BROWSER_FAILED', reason: 'os-shell-error' },
  );
  assertStateEqual(s, { kind: 'failed', reason: 'os-shell-error' }, 'BROWSER_FAILED from opening');
}

// Negative case: a browser failure from awaiting lands in failed.
function browserFailedFromAwaitingLandsInFailed() {
  const s = reduceSubmissionState(
    { kind: 'awaiting-issue-url', title: 'x' },
    { type: 'BROWSER_FAILED', reason: 'os-shell-error' },
  );
  assertStateEqual(s, { kind: 'failed', reason: 'os-shell-error' }, 'BROWSER_FAILED from awaiting');
}

// Positive case: RESET from any state returns the reducer to idle.
function resetFromAnyStateReturnsToIdle() {
  const states: SubmissionState[] = [
    { kind: 'idle' },
    { kind: 'opening', title: 'x' },
    { kind: 'awaiting-issue-url', title: 'x' },
    { kind: 'confirmed', url: CONFIRMED_URL },
    { kind: 'failed', reason: 'x' },
  ];
  for (const st of states) {
    assertStateEqual(
      reduceSubmissionState(st, { type: 'RESET' }),
      { kind: 'idle' },
      `RESET from ${st.kind}`,
    );
  }
}

// Negative case: the submit gate is false on every non-confirmed state
// AND on confirmed carrying an empty URL.
function canClaimSubmittedIsFalseOnEveryNonClaimableState() {
  const falseStates: SubmissionState[] = [
    { kind: 'idle' },
    { kind: 'opening', title: 'x' },
    { kind: 'awaiting-issue-url', title: 'x' },
    { kind: 'confirmed', url: '' },
    { kind: 'failed', reason: 'x' },
  ];
  for (const st of falseStates) {
    assertEqual(canClaimSubmitted(st), false, `canClaimSubmitted must be false on ${st.kind}`);
  }
}

// Positive case: the submit gate is true ONLY on confirmed(non-empty url).
function canClaimSubmittedIsTrueOnlyOnConfirmedWithNonEmptyUrl() {
  assertEqual(
    canClaimSubmitted({ kind: 'confirmed', url: CONFIRMED_URL }),
    true,
    'canClaimSubmitted true on confirmed(non-empty url)',
  );
}

happyPathReachesConfirmedViaBrowserOpened();
noPathToConfirmedWithoutBrowserOpened();
browserOpenedWithEmptyUrlLandsInFailed();
browserFailedFromOpeningLandsInFailed();
browserFailedFromAwaitingLandsInFailed();
resetFromAnyStateReturnsToIdle();
canClaimSubmittedIsFalseOnEveryNonClaimableState();
canClaimSubmittedIsTrueOnlyOnConfirmedWithNonEmptyUrl();

console.log('submissionState.test.ts: 8/8 assertions passed');

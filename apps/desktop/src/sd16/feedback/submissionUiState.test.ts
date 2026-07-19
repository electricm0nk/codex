/**
 * React-agnostic adapter over the submission reducer state.
 *
 * Self-executing tsx test (auto-discovered by scripts/run-tests.mjs). Uses the
 * shared testSupport/asserts helpers plus a local structural deep-equal.
 * Enumerates every variant of SubmissionState and asserts the exact
 * SubmissionUiState the React layer (F4) will consume, plus the load-bearing
 * invariant `deriveSubmissionUiState(s).canClaimSubmitted === canClaimSubmitted(s)`.
 */
import { assert, assertEqual } from '../../testSupport/asserts';
import { canClaimSubmitted, type SubmissionState } from './submissionState';
import { deriveSubmissionUiState, type SubmissionUiState } from './submissionUiState';

const ISSUE_URL = 'https://github.com/electricm0nk/codex/issues/new?title=x';

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

function assertUiEqual(actual: SubmissionUiState, expected: SubmissionUiState, message: string) {
  assert(
    isDeepEqual(actual, expected),
    `${message}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`,
  );
}

function idleMaps() {
  assertUiEqual(
    deriveSubmissionUiState({ kind: 'idle' }),
    { status: 'idle', canClaimSubmitted: false },
    'idle',
  );
}

function openingMaps() {
  assertUiEqual(
    deriveSubmissionUiState({ kind: 'opening', title: 'x' }),
    { status: 'opening', canClaimSubmitted: false },
    'opening',
  );
}

function awaitingMaps() {
  assertUiEqual(
    deriveSubmissionUiState({ kind: 'awaiting-issue-url', title: 'x' }),
    { status: 'awaiting', canClaimSubmitted: false },
    'awaiting-issue-url',
  );
}

function confirmedWithUrlMaps() {
  assertUiEqual(
    deriveSubmissionUiState({ kind: 'confirmed', url: ISSUE_URL }),
    { status: 'confirmed', canClaimSubmitted: true, url: ISSUE_URL },
    'confirmed(non-empty url)',
  );
}

function confirmedWithEmptyUrlMaps() {
  // Empty-url confirmed is NOT claimable and carries no `url` key.
  assertUiEqual(
    deriveSubmissionUiState({ kind: 'confirmed', url: '' }),
    { status: 'confirmed', canClaimSubmitted: false },
    'confirmed(empty url)',
  );
}

function failedMaps() {
  assertUiEqual(
    deriveSubmissionUiState({ kind: 'failed', reason: 'os-shell-error' }),
    { status: 'failed', canClaimSubmitted: false, reason: 'os-shell-error' },
    'failed',
  );
}

// Load-bearing invariant for F4: the adapter's claim flag must agree with the
// reducer's single source of truth for every reachable state.
function canClaimSubmittedInvariantHoldsForEveryState() {
  const states: SubmissionState[] = [
    { kind: 'idle' },
    { kind: 'opening', title: 'x' },
    { kind: 'awaiting-issue-url', title: 'x' },
    { kind: 'confirmed', url: ISSUE_URL },
    { kind: 'confirmed', url: '' },
    { kind: 'failed', reason: 'x' },
  ];
  for (const s of states) {
    assertEqual(
      deriveSubmissionUiState(s).canClaimSubmitted,
      canClaimSubmitted(s),
      `derived.canClaimSubmitted must agree with canClaimSubmitted(${s.kind})`,
    );
  }
}

idleMaps();
openingMaps();
awaitingMaps();
confirmedWithUrlMaps();
confirmedWithEmptyUrlMaps();
failedMaps();
canClaimSubmittedInvariantHoldsForEveryState();

console.log('submissionUiState.test.ts: 7/7 assertions passed');

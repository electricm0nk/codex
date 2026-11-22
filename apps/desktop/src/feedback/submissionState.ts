/**
 * Defect-submission state machine for the shell's feedback flow.
 *
 * Pure reducer — no React, no DOM, no IPC. The single load-bearing rule the
 * shell must honour: it may NEVER claim submission without proof. `confirmed`
 * is therefore reachable ONLY via `BROWSER_OPENED` carrying a non-empty URL;
 * every other transition that would otherwise land in `confirmed` lands in
 * `failed(reason)` instead. `canClaimSubmitted` is the single source of truth
 * for the UI "submit" gate.
 *
 * There is deliberately NO `submitted` state: the shell hands the OS to the
 * browser and confirms only that the browser was opened with a real URL. Proof
 * that a GitHub issue actually exists is E8 acceptance territory, not a state
 * this reducer may fabricate. The `BROWSER_OPENED` event is emitted by the F4
 * wiring slice after the actual OS-level browser open succeeds.
 */

export type SubmissionState =
  | { kind: 'idle' }
  | { kind: 'opening'; title: string }
  | { kind: 'awaiting-issue-url'; title: string }
  | { kind: 'confirmed'; url: string }
  | { kind: 'failed'; reason: string };

export type SubmissionEvent =
  | { type: 'OPEN'; title: string }
  | { type: 'URL_BUILT'; title: string }
  | { type: 'BROWSER_OPENED'; url: string }
  | { type: 'BROWSER_FAILED'; reason: string }
  | { type: 'RESET' };

/**
 * Pure reducer. `confirmed` is reachable ONLY by `BROWSER_OPENED` carrying a
 * non-empty URL. Every event that does not apply to the current state leaves
 * the state unchanged (no accidental transitions).
 */
export function reduceSubmissionState(
  state: SubmissionState,
  event: SubmissionEvent,
): SubmissionState {
  switch (event.type) {
    case 'RESET':
      return { kind: 'idle' };
    case 'OPEN':
      return state.kind === 'idle'
        ? { kind: 'opening', title: event.title }
        : state;
    case 'URL_BUILT':
      return state.kind === 'opening'
        ? { kind: 'awaiting-issue-url', title: state.title }
        : state;
    case 'BROWSER_OPENED':
      if (state.kind !== 'awaiting-issue-url') {
        return state;
      }
      return event.url.length > 0
        ? { kind: 'confirmed', url: event.url }
        : { kind: 'failed', reason: 'empty-url' };
    case 'BROWSER_FAILED':
      return state.kind === 'opening' || state.kind === 'awaiting-issue-url'
        ? { kind: 'failed', reason: event.reason }
        : state;
  }
}

/**
 * Single source of truth for the UI "submit" gate. Returns `false` on every
 * non-`confirmed` state, and `false` on `confirmed` if the URL is empty.
 */
export function canClaimSubmitted(state: SubmissionState): boolean {
  return state.kind === 'confirmed' && state.url.length > 0;
}

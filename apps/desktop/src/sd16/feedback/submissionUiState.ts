/**
 * React-agnostic view of the SD-16 submission reducer state (AV-PAY-5).
 *
 * `status` is a flattened enum so the React layer (F4) does not have to walk
 * the discriminated union, and `canClaimSubmitted` is computed via the reducer's
 * single source of truth. The load-bearing invariant, tested exhaustively in
 * submissionUiState.test.ts, is:
 *
 *   deriveSubmissionUiState(state).canClaimSubmitted === canClaimSubmitted(state)
 *
 * for every state. `url` is present only when it is safe to surface (non-empty
 * confirmed); `reason` is present only on `failed`.
 */
import { canClaimSubmitted, type SubmissionState } from './submissionState';

export type SubmissionUiState = {
  status: 'idle' | 'opening' | 'awaiting' | 'confirmed' | 'failed';
  canClaimSubmitted: boolean;
  url?: string;
  reason?: string;
};

export function deriveSubmissionUiState(state: SubmissionState): SubmissionUiState {
  switch (state.kind) {
    case 'idle':
      return { status: 'idle', canClaimSubmitted: false };
    case 'opening':
      return { status: 'opening', canClaimSubmitted: false };
    case 'awaiting-issue-url':
      return { status: 'awaiting', canClaimSubmitted: false };
    case 'confirmed':
      // canClaimSubmitted(state) is the single source of truth: a non-empty
      // URL is claimable and surfaces `url`; an empty URL is neither.
      return canClaimSubmitted(state)
        ? { status: 'confirmed', canClaimSubmitted: true, url: state.url }
        : { status: 'confirmed', canClaimSubmitted: false };
    case 'failed':
      return { status: 'failed', canClaimSubmitted: false, reason: state.reason };
  }
}

/**
 * Governed SD-11 GitHub bug-report flow.
 *
 * Bug-only composition (`composeBugReport`) over the shared feedback evidence
 * substrate, plus governed submission (`submitBugReport`) that never claims
 * success without a real issue handle and always preserves a copyable draft.
 */

export * from './composeBugReport';
export * from './submitBugReport';

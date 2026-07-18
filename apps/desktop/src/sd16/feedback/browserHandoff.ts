/**
 * SD-16 F4 — browser-handoff wiring for defect/enhancement submission.
 *
 * Drives the AV-PAY-5 submission reducer against the real Rust boundary:
 * the `handoff_defect_report_to_browser` Tauri command builds + re-validates the
 * prefilled GitHub "new issue" URL and performs the OS-level browser open
 * (tauri-plugin-opener). This module owns the event choreography the
 * reducer's contract demands:
 *
 *   OPEN → URL_BUILT → BROWSER_OPENED(url)   only after the OS open succeeded
 *                    → BROWSER_FAILED(reason) on every other outcome
 *
 * The single load-bearing rule carries through from the reducer: the shell
 * NEVER claims the issue was submitted. `confirmed` means exactly "a
 * prefilled GitHub issue form was opened in the tester's browser with a
 * validated URL" — filing happens in the browser, and closed-loop proof of
 * an existing issue is E8 acceptance territory.
 *
 * When the OS open fails after the URL was validated, the validated URL is
 * preserved (`manualUrl`) so the tester can open it by hand without losing
 * the prepared handoff.
 */

import { invoke } from '@tauri-apps/api/core';
import { hasTauriRuntime } from '../../boundary/runtime';
import { reduceSubmissionState, type SubmissionState } from './submissionState';
import { deriveSubmissionUiState, type SubmissionUiState } from './submissionUiState';

/** Governed target repository for shell-filed issues. */
export const GITHUB_ISSUE_OWNER = 'electricm0nk';
export const GITHUB_ISSUE_REPO = 'codex';

/** The composed-draft fields the handoff needs (bug and enhancement drafts both satisfy this). */
export interface BrowserHandoffDraft {
  title: string;
  markdownBody: string;
  labels: string[];
}

export type InvokeLike = (
  command: string,
  args: Record<string, unknown>
) => Promise<unknown>;

export interface BrowserHandoffOptions {
  /** Injected IPC. Defaults to the Tauri `invoke`; tests stub it. */
  invokeImpl?: InvokeLike;
  /** Injected runtime probe. Defaults to the boundary's Tauri detection. */
  hasRuntimeImpl?: () => boolean;
}

export interface BrowserHandoffOutcome {
  state: SubmissionState;
  ui: SubmissionUiState;
  /**
   * The validated issue URL whenever one was produced — present on success
   * AND on a failed OS open, so the UI can always offer a manual link.
   * `null` when the failure happened before a URL was validated.
   */
  manualUrl: string | null;
}

interface IssueUrlResponseWire {
  url?: unknown;
  opened?: unknown;
}

/** Render the Rust `IssueUrlError` rejection payload into a stable reason string. */
function renderHandoffError(error: unknown): { reason: string; url: string | null } {
  if (error !== null && typeof error === 'object') {
    const record = error as Record<string, unknown>;
    const kind = typeof record.kind === 'string' ? record.kind : 'unknown-error';
    const detail = typeof record.reason === 'string' ? `: ${record.reason}` : '';
    const url = typeof record.url === 'string' && record.url.length > 0 ? record.url : null;
    return { reason: `${kind}${detail}`, url };
  }
  return { reason: String(error), url: null };
}

/**
 * Run the full browser handoff for a composed issue draft. Never throws;
 * every outcome is a reducer state plus its UI projection.
 */
export async function runBrowserHandoff(
  draft: BrowserHandoffDraft,
  options: BrowserHandoffOptions = {}
): Promise<BrowserHandoffOutcome> {
  const invokeImpl = options.invokeImpl ?? (invoke as InvokeLike);
  const hasRuntimeImpl = options.hasRuntimeImpl ?? hasTauriRuntime;

  let state: SubmissionState = { kind: 'idle' };
  const dispatch = (event: Parameters<typeof reduceSubmissionState>[1]) => {
    state = reduceSubmissionState(state, event);
  };
  const finish = (manualUrl: string | null): BrowserHandoffOutcome => ({
    state,
    ui: deriveSubmissionUiState(state),
    manualUrl,
  });

  dispatch({ type: 'OPEN', title: draft.title });

  if (!hasRuntimeImpl()) {
    dispatch({
      type: 'BROWSER_FAILED',
      reason:
        'desktop runtime unavailable: the browser handoff needs the Tauri boundary to open the prefilled GitHub issue form',
    });
    return finish(null);
  }

  let response: IssueUrlResponseWire;
  try {
    response = (await invokeImpl('handoff_defect_report_to_browser', {
      req: {
        owner: GITHUB_ISSUE_OWNER,
        repo: GITHUB_ISSUE_REPO,
        title: draft.title,
        body: draft.markdownBody,
        labels: draft.labels,
      },
    })) as IssueUrlResponseWire;
  } catch (error: unknown) {
    const rendered = renderHandoffError(error);
    dispatch({ type: 'BROWSER_FAILED', reason: rendered.reason });
    return finish(rendered.url);
  }

  const url = typeof response.url === 'string' ? response.url : '';
  dispatch({ type: 'URL_BUILT', title: draft.title });

  if (response.opened !== true) {
    dispatch({
      type: 'BROWSER_FAILED',
      reason: 'the boundary did not report a successful browser open',
    });
    return finish(url.length > 0 ? url : null);
  }

  // BROWSER_OPENED with an empty URL lands in failed(empty-url) per the
  // reducer contract — no claim without a real URL.
  dispatch({ type: 'BROWSER_OPENED', url });
  return finish(url.length > 0 ? url : null);
}

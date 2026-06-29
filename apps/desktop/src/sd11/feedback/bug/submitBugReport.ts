/**
 * Governed submission for SD-11 GitHub bug reports.
 *
 * The single non-negotiable rule from the intake contract: the app must never
 * claim a bug report was filed unless a real transport confirms it with an actual
 * issue handle. When submission is unavailable, fails, or returns no confirmed
 * handle, the structured draft is preserved as a copyable payload so no evidence
 * is ever discarded. This module deliberately ships no transport and no auth — a
 * real transport is injected by the caller; its absence is an honest
 * draft-preserved outcome, not a failure to hide.
 */

import type { ComposedBugReport, GithubBugIssueDraft } from './composeBugReport';

/** Result returned by an injected GitHub bug-report transport. */
export interface BugReportTransportResult {
  ok: boolean;
  /** Real, confirmed issue URL. Success is only claimed when ok is true and this is present. */
  issueUrl?: string | null;
  issueNumber?: number | null;
  error?: string | null;
}

export type BugReportTransport = (
  draft: GithubBugIssueDraft
) => Promise<BugReportTransportResult>;

export type BugReportSubmissionStatus = 'submitted' | 'draft-preserved' | 'blocked-incomplete';

export interface BugReportResultHandle {
  issueUrl: string;
  issueNumber: number | null;
}

export interface BugReportSubmissionOutcome {
  status: BugReportSubmissionStatus;
  /** True only when a real issue handle was returned by the transport. */
  claimedSubmitted: boolean;
  resultHandle: BugReportResultHandle | null;
  draft: GithubBugIssueDraft;
  /** Always populated so evidence survives a missing/failed transport. */
  copyablePayload: string;
  message: string;
  problems: string[];
}

export interface SubmitBugReportInput {
  composed: ComposedBugReport;
  /** Injected real transport. When absent, the draft is preserved, not faked. */
  transport?: BugReportTransport | null;
}

/** Render a copyable plain-text payload that preserves the structured report. */
export function renderCopyableBugPayload(draft: GithubBugIssueDraft): string {
  return [
    `Title: ${draft.title}`,
    `Issue type: ${draft.issueType}`,
    `Labels: ${draft.labels.join(', ')}`,
    '',
    draft.markdownBody,
  ].join('\n');
}

/**
 * Attempt to submit a composed bug report under governance. Never returns
 * `submitted` without a real result handle from the transport.
 */
export async function submitBugReport(
  input: SubmitBugReportInput
): Promise<BugReportSubmissionOutcome> {
  const { composed } = input;
  const copyablePayload = renderCopyableBugPayload(composed.draft);
  const base = {
    resultHandle: null as BugReportResultHandle | null,
    draft: composed.draft,
    copyablePayload,
    claimedSubmitted: false,
  };

  if (!composed.submittable) {
    return {
      ...base,
      status: 'blocked-incomplete',
      message:
        'Bug report is not submittable yet: required evidence or title is incomplete. ' +
        'The structured draft below is preserved so nothing is lost.',
      problems: composed.problems,
    };
  }

  const transport = input.transport ?? null;
  if (!transport) {
    return {
      ...base,
      status: 'draft-preserved',
      message:
        'No GitHub submission transport is configured in this build. The complete structured ' +
        'draft is preserved for manual filing or copy — no submission was performed and none is claimed.',
      problems: [],
    };
  }

  let result: BugReportTransportResult;
  try {
    result = await transport(composed.draft);
  } catch (cause: unknown) {
    return {
      ...base,
      status: 'draft-preserved',
      message:
        `Bug report submission failed: ${formatError(cause)}. The structured draft is preserved ` +
        'so no evidence is lost; nothing was filed.',
      problems: [],
    };
  }

  const rawIssueUrl = result.issueUrl?.trim() ?? '';
  let issueUrl = '';
  if (rawIssueUrl.length > 0) {
    try {
      const parsed = new URL(rawIssueUrl);
      if (parsed.protocol === 'http:' || parsed.protocol === 'https:') {
        issueUrl = parsed.toString();
      }
    } catch {
      issueUrl = '';
    }
  }
  if (!result.ok || issueUrl.length === 0) {
    const detail = result.error
      ? `: ${result.error}`
      : rawIssueUrl.length
        ? ': invalid issueUrl returned by transport'
        : '';
    return {
      ...base,
      status: 'draft-preserved',
      message:
        `Bug report submission did not return a confirmed issue handle${detail}. The structured ` +
        'draft is preserved; submission is not claimed.',
      problems: [],
    };
  }

  return {
    ...base,
    status: 'submitted',
    claimedSubmitted: true,
    resultHandle: { issueUrl, issueNumber: result.issueNumber ?? null },
    message: `Bug report filed: ${issueUrl}`,
    problems: [],
  };
}

function formatError(cause: unknown): string {
  return cause instanceof Error ? cause.message : String(cause);
}

/**
 * Which path a bug or enhancement report takes (`decisions.md §23`, item 15).
 *
 *   Anonymous on  → the codex-web report API through the `send_report` Tauri command,
 *                   with the optional Handle as the body's first line.
 *   Anonymous off → the GitHub browser handoff (unchanged). When that fails the user is told
 *                   to turn on Anonymous; the API is never called as a silent fallback.
 */

import {
  loadReportAvailability,
  sendReport,
  type ReportAvailability,
  type ReportKind,
  type ReportOutcome,
  type ReportRequest,
} from '../boundary/sendReport';
import { formatError } from '../boundary/runtime';
import { runBrowserHandoff, type BrowserHandoffDraft, type BrowserHandoffOutcome } from './browserHandoff';

export interface ReportRouteDeps {
  availability: () => Promise<ReportAvailability>;
  send: (request: ReportRequest) => Promise<ReportOutcome>;
  handoff: (draft: BrowserHandoffDraft) => Promise<BrowserHandoffOutcome>;
}

const DEFAULT_DEPS: ReportRouteDeps = {
  availability: loadReportAvailability,
  send: sendReport,
  handoff: (draft) => runBrowserHandoff(draft),
};

export interface ReportRouteInput {
  kind: ReportKind;
  anonymous: boolean;
  handle: string;
  draft: BrowserHandoffDraft;
}

export type ReportRouteOutcome =
  | { route: 'api'; sent: boolean; message: string }
  | { route: 'browser'; handoff: BrowserHandoffOutcome; githubUnavailable: string | null };

/** The Handle, collapsed to one line, becomes the body's first line; a blank Handle adds nothing. */
export function bodyWithHandle(handle: string, body: string): string {
  const line = handle.replace(/\s+/g, ' ').trim();
  return line.length > 0 ? `${line}\n${body}` : body;
}

export async function routeReport(
  input: ReportRouteInput,
  deps: ReportRouteDeps = DEFAULT_DEPS,
): Promise<ReportRouteOutcome> {
  if (!input.anonymous) {
    const handoff = await deps.handoff(input.draft);
    const githubUnavailable =
      handoff.ui.status === 'failed'
        ? `GitHub is unavailable (${handoff.ui.reason ?? 'the browser handoff failed'}). Nothing was sent — turn on Anonymous to send this report for review instead.`
        : null;
    return { route: 'browser', handoff, githubUnavailable };
  }

  try {
    const availability = await deps.availability();
    if (!availability.available) {
      return {
        route: 'api',
        sent: false,
        message: availability.reason ?? 'Anonymous reports are not available in this build. Nothing was sent.',
      };
    }
    const outcome = await deps.send({
      kind: input.kind,
      title: input.draft.title,
      body: bodyWithHandle(input.handle, input.draft.markdownBody),
    });
    return outcome.outcome === 'sent'
      ? { route: 'api', sent: true, message: outcome.message }
      : { route: 'api', sent: false, message: outcome.reason };
  } catch (cause: unknown) {
    return { route: 'api', sent: false, message: `${formatError(cause)}. Nothing was sent.` };
  }
}

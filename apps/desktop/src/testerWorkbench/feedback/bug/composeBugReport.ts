/**
 * Governed GitHub bug-report composition for SD-11.
 *
 * Turns an assembled `bug`-flow feedback evidence payload plus a tester-supplied
 * title into a structured GitHub issue draft that follows the bug-report intake
 * contract:
 *   programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/
 *     artifacts/github-bug-report-intake-contract.md
 *
 * The composer is bug-only by construction. It never composes enhancement
 * requests, never improvises auth/transport, and always emits a structured draft
 * (so evidence is preserved even when the report is not yet submittable). It keeps
 * observed and expected behavior in distinct sections and never omits the
 * auto-captured build/channel/platform/workflow metadata when it is available.
 */

import type {
  AutoCapturedEvidence,
  CapturedEvidenceField,
  FeedbackEvidencePayload,
} from '../evidence';
import { sanitizeReportableOutput } from '../evidence/sanitizeReportableOutput';

/** The minimum-required GitHub label for every bug report. */
export const BUG_ISSUE_LABEL = 'bug';

export interface BugReportComposeInput {
  /** Tester-supplied title; must summarize the observable failure, not a guessed cause. */
  title: string;
  /** An assembled feedback evidence payload whose flow is `bug`. */
  payload: FeedbackEvidencePayload;
}

export interface GithubIssueSection {
  heading: string;
  body: string;
}

export interface GithubBugIssueDraft {
  issueType: 'bug';
  title: string;
  labels: string[];
  /** Structured sections, preserved even when serialized to markdown. */
  sections: GithubIssueSection[];
  /** The same sections rendered as a markdown issue body. */
  markdownBody: string;
}

export interface ComposedBugReport {
  draft: GithubBugIssueDraft;
  /** True only when the evidence is complete and a real title is present. */
  submittable: boolean;
  problems: string[];
}

const REDACTED_MARKER = '_[redacted by tester before inclusion]_';
const NOT_PROVIDED_MARKER = '_Not provided._';

/**
 * Compose a governed GitHub bug-report draft. Throws if handed a non-bug payload —
 * this composer deliberately does not handle enhancement requests.
 */
export function composeBugReport(input: BugReportComposeInput): ComposedBugReport {
  if (input.payload.flow !== 'bug') {
    throw new Error(
      `composeBugReport only accepts a 'bug' flow payload, received '${input.payload.flow}'.`
    );
  }

  const title = sanitizeReportableOutput(input.title.trim());
  const problems: string[] = [...input.payload.problems];

  if (title.length === 0) {
    problems.push('A bug report title is required and must summarize the observable failure.');
  }

  const draft: GithubBugIssueDraft = buildDraft(title, input.payload);
  const submittable = input.payload.complete && title.length > 0;

  return { draft, submittable, problems };
}

function buildDraft(title: string, payload: FeedbackEvidencePayload): GithubBugIssueDraft {
  const labels = buildLabels(payload.auto);
  const sections = buildSections(title, payload);
  const markdownBody = renderMarkdownBody(sections);

  return {
    issueType: 'bug',
    title,
    labels,
    sections,
    markdownBody,
  };
}

function buildLabels(auto: AutoCapturedEvidence): string[] {
  const labels: string[] = [BUG_ISSUE_LABEL];

  // channel/build tier label when available — channelSupportLabel is
  // `${channel} · ${platform support}`, so the leading token is the channel.
  const channel = slug(auto.channelSupportLabel.split('·')[0] ?? '');
  if (channel) {
    labels.push(`channel:${channel}`);
  }

  const platform = slug(auto.platformLabel);
  if (platform) {
    labels.push(`platform:${platform}`);
  }

  // affected-surface label when determinable — the workflow name (the portion
  // before the ` / ` state suffix) identifies the affected surface.
  const surface = slug((auto.currentWorkflow.split('/')[0] ?? '').trim());
  if (surface) {
    labels.push(`surface:${surface}`);
  }

  return dedupe(labels);
}

function buildSections(title: string, payload: FeedbackEvidencePayload): GithubIssueSection[] {
  const auto = payload.auto;

  return sanitizeSections([
    {
      heading: 'Summary',
      body: title.length > 0 ? title : '_No summary title provided yet._',
    },
    {
      heading: 'Current build / channel / platform / workflow',
      body: renderMetadataList(auto),
    },
    {
      heading: 'Observed behavior',
      body: renderField(payload, 'observedBehavior'),
    },
    {
      heading: 'Expected behavior',
      body: renderField(payload, 'expectedBehavior'),
    },
    {
      heading: 'Reproduction steps',
      body: renderField(payload, 'reproductionSteps'),
    },
    {
      heading: 'Diagnostics / explanation context',
      body: renderDiagnostics(auto),
    },
    {
      heading: 'Attachments / redactions',
      body: renderAttachments(payload),
    },
  ]);
}

function renderMetadataList(auto: AutoCapturedEvidence): string {
  return [
    `- Build label: ${auto.buildLabel}`,
    `- Tester channel / support label: ${auto.channelSupportLabel}`,
    `- Platform: ${auto.platformLabel}`,
    `- Current bounded workflow: ${auto.currentWorkflow}`,
    `- Data-source identity: ${auto.dataSourceIdentity}`,
    `- Governed release unit id: ${auto.releaseUnitId ?? '_Not recorded_'}`,
    `- Source revision / provenance handle: ${auto.sourceRevision ?? '_Not recorded_'}`,
    `- Manifest surface / asset path: ${auto.manifestPath ?? '_Not recorded_'}`,
    `- Update eligibility state: ${auto.updateEligibilityState ?? '_Not recorded_'}`,
    `- Trust-gate status: ${auto.trustGateStatus ?? '_Not recorded_'}`,
    `- Replacement release id: ${auto.replacementReleaseId ?? '_Not recorded_'}`,
    `- Official release-truth surface: ${auto.officialSurface ?? '_Not recorded_'}`,
    `- Local build authority posture: ${auto.localBuildAuthority ?? '_Not recorded_'}`,
  ].join('\n');
}

function renderField(payload: FeedbackEvidencePayload, key: CapturedEvidenceField['key']): string {
  const field = payload.fields.find((candidate) => candidate.key === key);
  if (!field) {
    return NOT_PROVIDED_MARKER;
  }

  if (field.state === 'redacted') {
    return REDACTED_MARKER;
  }

  if (field.present && field.value) {
    return field.value;
  }

  return NOT_PROVIDED_MARKER;
}

function renderDiagnostics(auto: AutoCapturedEvidence): string {
  const parts: string[] = [];

  if (auto.diagnostics.length) {
    parts.push('Diagnostics:');
    for (const diagnostic of auto.diagnostics) {
      const blocking = diagnostic.claimBlocking ? ' (claim-blocking)' : '';
      const subject = diagnostic.subjectRef ? ` [${diagnostic.subjectRef}]` : '';
      parts.push(`- ${diagnostic.severityLabel} · ${diagnostic.classLabel}${blocking}: ${diagnostic.message}${subject}`);
    }
  }

  if (auto.blockedClaims.length) {
    parts.push('', 'Blocked claims:');
    for (const claim of auto.blockedClaims) {
      parts.push(`- ${claim}`);
    }
  }

  if (auto.explanationRefs.length) {
    parts.push('', 'Explanation references:');
    for (const reference of auto.explanationRefs) {
      parts.push(`- ${reference.label}: ${reference.detail} (\`${reference.machineRef}\`)`);
    }
  }

  if (auto.provenanceRefs.length) {
    parts.push('', 'Provenance references:');
    for (const reference of auto.provenanceRefs) {
      parts.push(`- ${reference.label}: ${reference.detail} (\`${reference.machineRef}\`)`);
    }
  }

  if (parts.length === 0) {
    return 'No diagnostics, blocked claims, or explanation context recorded in the current bounded snapshot.';
  }

  return parts.join('\n');
}

function renderAttachments(payload: FeedbackEvidencePayload): string {
  const parts: string[] = [];

  if (payload.attachments.length === 0) {
    parts.push('No attachments included. Nothing was captured silently from logs, save files, or screenshots.');
  } else {
    parts.push('Attachments:');
    for (const decision of payload.attachments) {
      parts.push(`- ${decision.attachment.label} (${decision.attachment.kind}) — ${decision.outcome}: ${decision.reason}`);
    }
  }

  parts.push('');
  if (payload.redactionRequired) {
    const statement = payload.redaction.statement?.trim();
    parts.push(
      payload.redaction.declared && statement
        ? `Redaction declaration: ${statement}`
        : 'Redaction declaration: required but not yet provided for the included attachments.'
    );
  } else {
    parts.push('Redaction declaration: not required (no attachments included).');
  }

  return parts.join('\n');
}

function renderMarkdownBody(sections: GithubIssueSection[]): string {
  return sanitizeReportableOutput(sections.map((section) => `## ${section.heading}\n\n${section.body}`).join('\n\n'));
}

function sanitizeSections(sections: GithubIssueSection[]): GithubIssueSection[] {
  return sections.map((section) => ({
    heading: sanitizeReportableOutput(section.heading),
    body: sanitizeReportableOutput(section.body),
  }));
}

function slug(value: string): string {
  return value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

function dedupe(values: string[]): string[] {
  return Array.from(new Set(values));
}

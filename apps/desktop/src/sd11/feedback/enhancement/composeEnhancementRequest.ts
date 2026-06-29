/**
 * Governed GitHub enhancement-request composition for SD-11.
 *
 * Turns an assembled `enhancement`-flow feedback evidence payload plus a
 * tester-supplied title into a structured GitHub issue draft that follows the
 * enhancement-request intake contract:
 *   programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/
 *     artifacts/github-enhancement-request-intake-contract.md
 *
 * The composer is enhancement-only by construction. It never composes bug
 * reports, never improvises auth/transport, and always emits a structured draft
 * (so evidence is preserved even when the request is not yet submittable). It
 * keeps tester goal, current friction, requested capability, and affected surface
 * as four distinct sections and never omits the auto-captured
 * build/channel/platform/workflow metadata when it is available. It reuses the
 * shared evidence substrate rather than duplicating the bug-report schema, so the
 * two flows cannot drift apart.
 */

import type {
  AutoCapturedEvidence,
  CapturedEvidenceField,
  FeedbackEvidencePayload,
} from '../evidence';

/** The minimum-required GitHub label for every enhancement request. */
export const ENHANCEMENT_ISSUE_LABEL = 'enhancement';

export interface EnhancementRequestComposeInput {
  /** Tester-supplied title; must summarize the missing capability or blocked workflow. */
  title: string;
  /** An assembled feedback evidence payload whose flow is `enhancement`. */
  payload: FeedbackEvidencePayload;
}

export interface GithubIssueSection {
  heading: string;
  body: string;
}

export interface GithubEnhancementIssueDraft {
  issueType: 'enhancement';
  title: string;
  labels: string[];
  /** Structured sections, preserved even when serialized to markdown. */
  sections: GithubIssueSection[];
  /** The same sections rendered as a markdown issue body. */
  markdownBody: string;
}

export interface ComposedEnhancementRequest {
  draft: GithubEnhancementIssueDraft;
  /** True only when the evidence is complete and a real title is present. */
  submittable: boolean;
  problems: string[];
}

const REDACTED_MARKER = '_[redacted by tester before inclusion]_';
const NOT_PROVIDED_MARKER = '_Not provided._';

/**
 * Compose a governed GitHub enhancement-request draft. Throws if handed a
 * non-enhancement payload — this composer deliberately does not handle bug
 * reports, keeping the two flows separate even though they share transport and
 * the evidence substrate.
 */
export function composeEnhancementRequest(
  input: EnhancementRequestComposeInput
): ComposedEnhancementRequest {
  if (input.payload.flow !== 'enhancement') {
    throw new Error(
      `composeEnhancementRequest only accepts an 'enhancement' flow payload, received '${input.payload.flow}'.`
    );
  }

  const title = input.title.trim();
  const problems: string[] = [...input.payload.problems];

  if (title.length === 0) {
    problems.push(
      'An enhancement request title is required and must summarize the missing capability or blocked workflow.'
    );
  }

  const draft = buildDraft(title, input.payload);
  const submittable = input.payload.complete && title.length > 0;

  return { draft, submittable, problems };
}

function buildDraft(title: string, payload: FeedbackEvidencePayload): GithubEnhancementIssueDraft {
  const labels = buildLabels(payload);
  const sections = buildSections(title, payload);
  const markdownBody = renderMarkdownBody(sections);

  return {
    issueType: 'enhancement',
    title,
    labels,
    sections,
    markdownBody,
  };
}

function buildLabels(payload: FeedbackEvidencePayload): string[] {
  const auto = payload.auto;
  const labels: string[] = [ENHANCEMENT_ISSUE_LABEL];

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

  // affected-surface label when determinable — for the enhancement flow the
  // affected surface is a tester-entered field; fall back to the workflow name
  // (the portion before the ` / ` state suffix) when the tester left it empty or
  // it was redacted.
  const surface = slug(affectedSurfaceLabelSource(payload));
  if (surface) {
    labels.push(`surface:${surface}`);
  }

  return dedupe(labels);
}

/**
 * Determine the source string for the affected-surface label. Prefers the
 * tester-entered affected surface; falls back to the auto-captured workflow name
 * when that field is absent or redacted, so labels stay determinable without
 * leaking redacted content.
 */
function affectedSurfaceLabelSource(payload: FeedbackEvidencePayload): string {
  const affected = payload.fields.find((field) => field.key === 'affectedSurface');
  if (affected && affected.state !== 'redacted' && affected.value) {
    return affected.value;
  }
  return (payload.auto.currentWorkflow.split('/')[0] ?? '').trim();
}

function buildSections(title: string, payload: FeedbackEvidencePayload): GithubIssueSection[] {
  const auto = payload.auto;

  return [
    {
      heading: 'Summary',
      body: title.length > 0 ? title : '_No summary title provided yet._',
    },
    {
      heading: 'Current build / channel / platform / workflow',
      body: renderMetadataList(auto),
    },
    {
      heading: 'Tester goal',
      body: renderField(payload, 'testerGoal'),
    },
    {
      heading: 'Current friction or limitation',
      body: renderField(payload, 'currentFriction'),
    },
    {
      heading: 'Requested capability or improvement',
      body: renderField(payload, 'requestedCapability'),
    },
    {
      heading: 'Affected surface',
      body: renderField(payload, 'affectedSurface'),
    },
    {
      heading: 'Supporting evidence / examples',
      body: renderSupportingEvidence(payload),
    },
  ];
}

function renderMetadataList(auto: AutoCapturedEvidence): string {
  return [
    `- Build label: ${auto.buildLabel}`,
    `- Tester channel / support label: ${auto.channelSupportLabel}`,
    `- Platform: ${auto.platformLabel}`,
    `- Current bounded workflow: ${auto.currentWorkflow}`,
    `- Data-source identity: ${auto.dataSourceIdentity}`,
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

/**
 * Render the supporting-evidence section. Diagnostics, explanation, and
 * provenance context are optional for enhancement requests, so they are included
 * only when the bounded snapshot actually carries them (never fabricated).
 * Attachment and redaction decisions are folded in here so the contract's
 * seven-section body stays intact while redaction governance remains visible.
 */
function renderSupportingEvidence(payload: FeedbackEvidencePayload): string {
  const auto = payload.auto;
  const parts: string[] = [];

  if (auto.diagnostics.length) {
    parts.push('Diagnostics:');
    for (const diagnostic of auto.diagnostics) {
      const blocking = diagnostic.claimBlocking ? ' (claim-blocking)' : '';
      const subject = diagnostic.subjectRef ? ` [${diagnostic.subjectRef}]` : '';
      parts.push(
        `- ${diagnostic.severityLabel} · ${diagnostic.classLabel}${blocking}: ${diagnostic.message}${subject}`
      );
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
    parts.push('No diagnostics, blocked claims, or explanation context recorded in the current bounded snapshot.');
  }

  parts.push('', renderAttachments(payload));

  return parts.join('\n');
}

function renderAttachments(payload: FeedbackEvidencePayload): string {
  const parts: string[] = [];

  if (payload.attachments.length === 0) {
    parts.push('No attachments included. Nothing was captured silently from logs, save files, or screenshots.');
  } else {
    parts.push('Attachments:');
    for (const decision of payload.attachments) {
      parts.push(
        `- ${decision.attachment.label} (${decision.attachment.kind}) — ${decision.outcome}: ${decision.reason}`
      );
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
  return sections.map((section) => `## ${section.heading}\n\n${section.body}`).join('\n\n');
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

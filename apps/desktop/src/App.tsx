import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import desktopPackage from '../package.json';
import {
  loadSd11TesterWorkbenchSurfaceRuntime,
} from './sd11/loadSd11TesterWorkbenchSurfaceRuntime';
import type { Sd11TesterWorkbenchSurface } from './sd11/loadSd11TesterWorkbenchSurface';
import { createReferenceListKey } from './referenceListKey';
import {
  assembleFeedbackEvidence,
  captureAutoEvidence,
  sharedAutoCapturedFields,
  REDACTION_POLICY_NOTICE,
} from './sd11/feedback/evidence';
import {
  composeBugReport,
  renderCopyableBugPayload,
  submitBugReport,
  type BugReportSubmissionOutcome,
} from './sd11/feedback/bug';
import {
  composeEnhancementRequest,
  renderCopyableEnhancementPayload,
  submitEnhancementRequest,
  type EnhancementRequestSubmissionOutcome,
} from './sd11/feedback/enhancement';
import {
  runBrowserHandoff,
  type BrowserHandoffOutcome,
} from './sd16/feedback/browserHandoff';
import { CharacterHubPage } from './characterHub/CharacterHubPage';
import { UpdateUi } from './sd16/update/Ui';
import {
  createUpdateControllerDeps,
  loadMountTimeState,
  restorePreviousVersion,
  type MountTimeState,
} from './sd16/update/controllerAdapter';
import type { UpdateControllerDeps } from './sd16/update/updateModel';
import { SettingsModal, type SettingsTab } from './settings/SettingsModal';
import { AppearancePanel } from './settings/AppearancePanel';
import { GoogleDrivePanel } from './settings/GoogleDrivePanel';
import { applyThemeMode, getStoredThemeMode, type ThemeMode } from './settings/themeMode';
import { applyActiveTheme, applyObsidianModeClass, seedBuiltinThemes } from './settings/communityTheme';

function derivePlatformLabel(): string {
  if (typeof navigator === 'undefined') {
    return 'Unknown platform';
  }

  const rawPlatform = navigator.platform || 'Unknown platform';

  if (/mac/i.test(rawPlatform)) {
    return 'macOS';
  }

  if (/win/i.test(rawPlatform)) {
    return 'Windows';
  }

  if (/linux/i.test(rawPlatform)) {
    return 'Linux';
  }

  return rawPlatform;
}

function toneColor(tone: 'info' | 'warning' | 'error'): string {
  if (tone === 'error') {
    return 'var(--color-error)';
  }

  if (tone === 'warning') {
    return 'var(--color-warn)';
  }

  return 'var(--color-accent)';
}

function AppCard(props: { label: string; value: string; detail?: string }) {
  return (
    <div
      style={{
        backgroundColor: 'var(--color-surface)',
        border: '1px solid var(--color-border)',
        borderRadius: 12,
        padding: '0.9rem 1rem',
      }}
    >
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
        {props.label}
      </p>
      <p style={{ color: 'var(--color-text)', fontSize: '1rem', fontWeight: 700, margin: '0.35rem 0 0' }}>{props.value}</p>
      {props.detail ? <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.875rem', margin: '0.4rem 0 0' }}>{props.detail}</p> : null}
    </div>
  );
}

function EvidenceList(props: {
  emptyMessage: string;
  items: Array<{ label: string; detail: string; machineRef: string }>;
}) {
  if (!props.items.length) {
    return <p style={{ color: 'var(--color-text-secondary)', margin: 0 }}>{props.emptyMessage}</p>;
  }

  return (
    <ul style={{ margin: 0, paddingLeft: '1.2rem' }}>
      {props.items.map((reference, index) => (
        // GE08 refs can share label and machineRef while differing in detail,
        // so the list index keeps sibling keys unique.
        <li key={createReferenceListKey(reference.label, `${reference.machineRef}#${index}`)} style={{ marginBottom: '0.6rem' }}>
          <strong>{reference.label}</strong>
          <div style={{ color: 'var(--color-text-secondary)', marginTop: '0.2rem' }}>{reference.detail}</div>
          <code style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem' }}>{reference.machineRef}</code>
        </li>
      ))}
    </ul>
  );
}

function FeedbackEvidencePanel(props: { surface: Sd11TesterWorkbenchSurface }) {
  const auto = captureAutoEvidence(props.surface);
  const autoValueByKey: Record<string, string> = {
    buildLabel: auto.buildLabel,
    channelSupportLabel: auto.channelSupportLabel,
    platformLabel: auto.platformLabel,
    currentWorkflow: auto.currentWorkflow,
    dataSourceIdentity: auto.dataSourceIdentity,
    releaseUnitId: auto.releaseUnitId ?? '—',
    sourceRevision: auto.sourceRevision ?? '—',
    manifestPath: auto.manifestPath ?? '—',
    updateEligibilityState: auto.updateEligibilityState ?? '—',
    trustGateStatus: auto.trustGateStatus ?? '—',
    replacementReleaseId: auto.replacementReleaseId ?? '—',
    officialSurface: auto.officialSurface ?? '—',
    localBuildAuthority: auto.localBuildAuthority ?? '—',
    diagnostics:
      auto.diagnostics.length || auto.blockedClaims.length
        ? `${auto.diagnostics.length} diagnostic(s) · ${auto.blockedClaims.length} blocked claim(s)`
        : '—',
    explanationRefs: auto.explanationRefs.length ? `${auto.explanationRefs.length} explanation ref(s)` : '—',
    provenanceRefs: auto.provenanceRefs.length ? `${auto.provenanceRefs.length} provenance ref(s)` : '—',
  };

  return (
    <section style={{ border: '1px solid var(--color-border)', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
      <h2 style={{ marginTop: 0 }}>What gets attached to bug reports &amp; enhancement requests</h2>
      <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6 }}>
        These fields are captured automatically and included with every bug report or enhancement request you
        submit, on top of whatever you write yourself. Attachments (screenshots, logs, save files) are never
        captured without your explicit confirmation.
      </p>

      <div style={{ display: 'grid', gap: '0.75rem', gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))', marginTop: '1rem' }}>
        {sharedAutoCapturedFields().map((field) => (
          <AppCard
            key={field.key}
            label={`${field.label} · auto-captured`}
            value={autoValueByKey[field.key] ?? '—'}
            detail={field.note}
          />
        ))}
      </div>

      <div style={{ backgroundColor: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 12, marginTop: '1rem', padding: '0.9rem 1rem' }}>
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
          Attachment &amp; redaction policy
        </p>
        <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>{REDACTION_POLICY_NOTICE}</p>
        <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
          No attachments have been captured yet. Nothing is collected without your explicit confirmation.
        </p>
      </div>
    </section>
  );
}

function bugFieldStyle(): CSSProperties {
  return {
    border: '1px solid var(--color-border)',
    borderRadius: 8,
    fontFamily: 'inherit',
    fontSize: '0.9rem',
    marginTop: '0.35rem',
    padding: '0.55rem 0.65rem',
    width: '100%',
  };
}

function BugReportField(props: {
  label: string;
  value: string;
  placeholder: string;
  onChange: (value: string) => void;
  multiline?: boolean;
}) {
  const sharedProps = {
    value: props.value,
    placeholder: props.placeholder,
    onChange: (event: { target: { value: string } }) => props.onChange(event.target.value),
    style: bugFieldStyle(),
  };

  return (
    <label style={{ display: 'block', marginTop: '0.9rem' }}>
      <span style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem', fontWeight: 700, letterSpacing: '0.04em', textTransform: 'uppercase' }}>
        {props.label}
      </span>
      {props.multiline ? <textarea rows={3} {...sharedProps} /> : <input type="text" {...sharedProps} />}
    </label>
  );
}

function outcomeTone(status: BugReportSubmissionOutcome['status']): string {
  if (status === 'submitted') {
    return 'var(--color-accent)';
  }

  if (status === 'blocked-incomplete') {
    return 'var(--color-error)';
  }

  return 'var(--color-warn)';
}

/**
 * Honest browser-handoff outcome surface, shared by the bug and enhancement
 * composers. `confirmed` means exactly "a prefilled GitHub issue form was
 * opened in the tester's browser" — never that an issue was submitted. Every
 * outcome keeps the governed draft copyable so no evidence is lost.
 */
function BrowserHandoffResultPanel(props: { handoff: BrowserHandoffOutcome; copyablePayload: string }) {
  const [copied, setCopied] = useState(false);
  const confirmed = props.handoff.ui.status === 'confirmed';
  const tone = confirmed ? 'var(--color-accent)' : 'var(--color-warn)';

  async function onCopyPayload() {
    try {
      if (typeof navigator !== 'undefined' && navigator.clipboard) {
        await navigator.clipboard.writeText(props.copyablePayload);
        setCopied(true);
      }
    } catch {
      setCopied(false);
    }
  }

  return (
    <div
      style={{
        backgroundColor: 'var(--color-surface)',
        border: `1px solid ${tone}`,
        borderRadius: 12,
        marginTop: '1rem',
        padding: '0.9rem 1rem',
      }}
    >
      <p style={{ color: tone, fontWeight: 700, margin: 0 }}>
        Browser handoff: {confirmed ? 'prefilled issue form opened' : 'failed'}
      </p>
      {confirmed ? (
        <>
          <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
            A prefilled GitHub issue form was opened in your browser. Review it and press
            &ldquo;Create&rdquo; there to file the issue — the shell only confirms the form was
            opened; it does not claim the issue was submitted.
          </p>
          {props.handoff.ui.url ? (
            <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0.45rem 0 0', wordBreak: 'break-all' }}>
              Opened URL: <code>{props.handoff.ui.url}</code>
            </p>
          ) : null}
        </>
      ) : (
        <>
          <p style={{ color: 'var(--color-warn)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
            The prefilled GitHub issue form could not be opened
            {props.handoff.ui.reason ? ` (${props.handoff.ui.reason})` : ''}. Nothing was filed and
            no submission is claimed. Your structured draft is preserved below.
          </p>
          {props.handoff.manualUrl ? (
            <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0.45rem 0 0', wordBreak: 'break-all' }}>
              Open the validated prefilled form manually: <code>{props.handoff.manualUrl}</code>
            </p>
          ) : null}
        </>
      )}
      <button
        type="button"
        onClick={onCopyPayload}
        style={{
          backgroundColor: 'var(--color-accent)',
          border: 'none',
          borderRadius: 8,
          color: 'var(--color-on-accent)',
          cursor: 'pointer',
          fontWeight: 700,
          marginTop: '0.6rem',
          padding: '0.45rem 0.85rem',
        }}
      >
        {copied ? 'Copied governed draft' : 'Copy governed draft'}
      </button>
      <textarea
        readOnly
        value={props.copyablePayload}
        rows={10}
        style={{ ...bugFieldStyle(), fontFamily: 'monospace', marginTop: '0.6rem' }}
      />
    </div>
  );
}

function BugReportComposer(props: { surface: Sd11TesterWorkbenchSurface }) {
  const [title, setTitle] = useState('');
  const [observedBehavior, setObservedBehavior] = useState('');
  const [expectedBehavior, setExpectedBehavior] = useState('');
  const [reproductionSteps, setReproductionSteps] = useState('');
  const [outcome, setOutcome] = useState<BugReportSubmissionOutcome | null>(null);
  const [handoff, setHandoff] = useState<BrowserHandoffOutcome | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [copied, setCopied] = useState(false);

  const composed = useMemo(() => {
    const payload = assembleFeedbackEvidence({
      flow: 'bug',
      surface: props.surface,
      testerInput: { observedBehavior, expectedBehavior, reproductionSteps },
    });
    return composeBugReport({ title, payload });
  }, [props.surface, title, observedBehavior, expectedBehavior, reproductionSteps]);

  // Submittable drafts go through the governed browser handoff (SD-16 F4):
  // the Rust boundary builds + validates a prefilled GitHub issue URL and
  // opens it in the OS browser. No secret-bearing auth is ever improvised,
  // and no submission is claimed — filing completes in the browser.
  // Non-submittable drafts keep the honest blocked-incomplete preservation.
  async function onPrepareSubmission() {
    setSubmitting(true);
    setCopied(false);
    try {
      if (composed.submittable) {
        setOutcome(null);
        setHandoff(await runBrowserHandoff(composed.draft));
      } else {
        setHandoff(null);
        setOutcome(await submitBugReport({ composed, transport: null }));
      }
    } finally {
      setSubmitting(false);
    }
  }

  async function onCopyPayload() {
    if (!outcome) {
      return;
    }
    try {
      if (typeof navigator !== 'undefined' && navigator.clipboard) {
        await navigator.clipboard.writeText(outcome.copyablePayload);
        setCopied(true);
      }
    } catch {
      setCopied(false);
    }
  }

  return (
    <section style={{ border: '1px solid var(--color-border)', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
      <h2 style={{ marginTop: 0 }}>Report a bug</h2>
      <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6 }}>
        This is the governed <strong>bug-report</strong> flow: it captures an observable failure with structured,
        required fields and composes a GitHub issue per the SD-11 bug-report intake contract. It is deliberately
        separate from enhancement requests, which are not handled here. Observed and expected behavior are kept
        distinct so triage can tell what happened from what the tester believed should happen.
      </p>

      <BugReportField
        label="Issue title (summarize the observable failure, not a guessed cause)"
        value={title}
        placeholder="e.g. Preview fails to render baseline AC after loading a package"
        onChange={(value) => setTitle(value)}
      />
      <BugReportField
        label="Observed behavior (what happened)"
        value={observedBehavior}
        placeholder="Describe exactly what the app did."
        onChange={(value) => setObservedBehavior(value)}
        multiline
      />
      <BugReportField
        label="Expected behavior (what you believed should happen)"
        value={expectedBehavior}
        placeholder="Describe what you expected instead."
        onChange={(value) => setExpectedBehavior(value)}
        multiline
      />
      <BugReportField
        label="Reproduction steps (step-ordered when reproducible)"
        value={reproductionSteps}
        placeholder={'1. …\n2. …\n3. …'}
        onChange={(value) => setReproductionSteps(value)}
        multiline
      />

      <div style={{ backgroundColor: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 12, marginTop: '1rem', padding: '0.9rem 1rem' }}>
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
          Auto-captured metadata · attached to every report
        </p>
        <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
          Labels: {composed.draft.labels.join(', ')}
        </p>
        <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>{REDACTION_POLICY_NOTICE}</p>
      </div>

      {composed.problems.length ? (
        <div style={{ backgroundColor: 'var(--color-warn-bg)', border: '1px solid var(--color-warn-border)', borderRadius: 12, marginTop: '1rem', padding: '0.9rem 1rem' }}>
          <p style={{ color: 'var(--color-warn)', fontWeight: 700, margin: 0 }}>Still required before this report can be submitted</p>
          <ul style={{ color: 'var(--color-warn)', margin: '0.45rem 0 0', paddingLeft: '1.2rem' }}>
            {composed.problems.map((problem) => (
              <li key={problem} style={{ marginBottom: '0.3rem' }}>{problem}</li>
            ))}
          </ul>
        </div>
      ) : null}

      <button
        type="button"
        onClick={onPrepareSubmission}
        disabled={submitting}
        style={{
          backgroundColor: composed.submittable ? 'var(--color-accent)' : 'var(--color-text-muted)',
          border: 'none',
          borderRadius: 8,
          color: 'var(--color-on-accent)',
          cursor: submitting ? 'wait' : 'pointer',
          fontWeight: 700,
          marginTop: '1rem',
          padding: '0.6rem 1rem',
        }}
      >
        {composed.submittable ? 'Open prefilled GitHub issue form' : 'Preserve bug draft (not yet submittable)'}
      </button>

      <h3 style={{ marginBottom: '0.4rem', marginTop: '1.25rem' }}>Composed issue preview</h3>
      <pre
        style={{
          backgroundColor: 'var(--color-accent)',
          borderRadius: 12,
          color: 'var(--color-on-accent-muted)',
          fontSize: '0.8rem',
          lineHeight: 1.5,
          margin: 0,
          overflowX: 'auto',
          padding: '1rem',
          whiteSpace: 'pre-wrap',
        }}
      >
        {composed.draft.markdownBody}
      </pre>

      {outcome ? (
        <div
          style={{
            backgroundColor: 'var(--color-surface)',
            border: `1px solid ${outcomeTone(outcome.status)}`,
            borderRadius: 12,
            marginTop: '1rem',
            padding: '0.9rem 1rem',
          }}
        >
          <p style={{ color: outcomeTone(outcome.status), fontWeight: 700, margin: 0 }}>
            Submission status: {outcome.status}
          </p>
          <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>{outcome.message}</p>
          {outcome.resultHandle ? (
            <p style={{ color: 'var(--color-accent)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
              Filed issue: <a href={outcome.resultHandle.issueUrl}>{outcome.resultHandle.issueUrl}</a>
            </p>
          ) : (
            <p style={{ color: 'var(--color-warn)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
              No issue handle was returned, so no successful submission is claimed. Your structured report is
              preserved below — copy it to file the issue manually without losing any evidence.
            </p>
          )}
          <button
            type="button"
            onClick={onCopyPayload}
            style={{
              backgroundColor: 'var(--color-accent)',
              border: 'none',
              borderRadius: 8,
              color: 'var(--color-on-accent)',
              cursor: 'pointer',
              fontWeight: 700,
              marginTop: '0.6rem',
              padding: '0.45rem 0.85rem',
            }}
          >
            {copied ? 'Copied governed draft' : 'Copy governed draft'}
          </button>
          <textarea
            readOnly
            value={outcome.copyablePayload}
            rows={10}
            style={{ ...bugFieldStyle(), fontFamily: 'monospace', marginTop: '0.6rem' }}
          />
        </div>
      ) : null}

      {handoff ? (
        <BrowserHandoffResultPanel
          handoff={handoff}
          copyablePayload={renderCopyableBugPayload(composed.draft)}
        />
      ) : null}
    </section>
  );
}

function EnhancementRequestComposer(props: { surface: Sd11TesterWorkbenchSurface }) {
  const [title, setTitle] = useState('');
  const [testerGoal, setTesterGoal] = useState('');
  const [currentFriction, setCurrentFriction] = useState('');
  const [requestedCapability, setRequestedCapability] = useState('');
  const [affectedSurface, setAffectedSurface] = useState('');
  const [outcome, setOutcome] = useState<EnhancementRequestSubmissionOutcome | null>(null);
  const [handoff, setHandoff] = useState<BrowserHandoffOutcome | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [copied, setCopied] = useState(false);

  const composed = useMemo(() => {
    const payload = assembleFeedbackEvidence({
      flow: 'enhancement',
      surface: props.surface,
      testerInput: { testerGoal, currentFriction, requestedCapability, affectedSurface },
    });
    return composeEnhancementRequest({ title, payload });
  }, [props.surface, title, testerGoal, currentFriction, requestedCapability, affectedSurface]);

  // Submittable drafts go through the governed browser handoff (SD-16 F4):
  // the Rust boundary builds + validates a prefilled GitHub issue URL and
  // opens it in the OS browser. No secret-bearing auth is ever improvised,
  // and no submission is claimed — filing completes in the browser.
  // Non-submittable drafts keep the honest blocked-incomplete preservation.
  async function onPrepareSubmission() {
    setSubmitting(true);
    setCopied(false);
    try {
      if (composed.submittable) {
        setOutcome(null);
        setHandoff(await runBrowserHandoff(composed.draft));
      } else {
        setHandoff(null);
        setOutcome(await submitEnhancementRequest({ composed, transport: null }));
      }
    } finally {
      setSubmitting(false);
    }
  }

  async function onCopyPayload() {
    if (!outcome) {
      return;
    }
    try {
      if (typeof navigator !== 'undefined' && navigator.clipboard) {
        await navigator.clipboard.writeText(outcome.copyablePayload);
        setCopied(true);
      }
    } catch {
      setCopied(false);
    }
  }

  return (
    <section style={{ border: '1px solid var(--color-border)', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
      <h2 style={{ marginTop: 0 }}>Request an enhancement</h2>
      <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6 }}>
        This is the governed <strong>enhancement-request</strong> flow: it captures a real blocked workflow or
        missing capability with structured, required fields and composes a GitHub issue per the SD-11
        enhancement-request intake contract. It is deliberately separate from bug reports, which are not handled
        here, and it is bounded structured intake — not a general roadmap idea box. Goal, friction, requested
        capability, and affected surface are kept distinct so triage can tell what the tester wanted from what
        blocked them and what should change.
      </p>

      <BugReportField
        label="Issue title (summarize the missing capability or blocked workflow)"
        value={title}
        placeholder="e.g. Add duplicate-and-edit so repeated package authoring is not a full wizard each time"
        onChange={(value) => setTitle(value)}
      />
      <BugReportField
        label="Tester goal (what you were trying to accomplish)"
        value={testerGoal}
        placeholder="Describe the outcome you were working toward."
        onChange={(value) => setTesterGoal(value)}
        multiline
      />
      <BugReportField
        label="Current friction or limitation (what blocked or slowed that goal)"
        value={currentFriction}
        placeholder="Describe what got in the way."
        onChange={(value) => setCurrentFriction(value)}
        multiline
      />
      <BugReportField
        label="Requested capability or improvement (what should exist or work differently)"
        value={requestedCapability}
        placeholder="Describe the change you are asking for."
        onChange={(value) => setRequestedCapability(value)}
        multiline
      />
      <BugReportField
        label="Affected surface (workbench area, workflow slice, or update/support surface)"
        value={affectedSurface}
        placeholder="e.g. GE08 authoring workbench"
        onChange={(value) => setAffectedSurface(value)}
      />

      <div style={{ backgroundColor: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 12, marginTop: '1rem', padding: '0.9rem 1rem' }}>
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
          Auto-captured metadata · attached to every request
        </p>
        <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
          Labels: {composed.draft.labels.join(', ')}
        </p>
        <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>{REDACTION_POLICY_NOTICE}</p>
      </div>

      {composed.problems.length ? (
        <div style={{ backgroundColor: 'var(--color-warn-bg)', border: '1px solid var(--color-warn-border)', borderRadius: 12, marginTop: '1rem', padding: '0.9rem 1rem' }}>
          <p style={{ color: 'var(--color-warn)', fontWeight: 700, margin: 0 }}>Still required before this request can be submitted</p>
          <ul style={{ color: 'var(--color-warn)', margin: '0.45rem 0 0', paddingLeft: '1.2rem' }}>
            {composed.problems.map((problem) => (
              <li key={problem} style={{ marginBottom: '0.3rem' }}>{problem}</li>
            ))}
          </ul>
        </div>
      ) : null}

      <button
        type="button"
        onClick={onPrepareSubmission}
        disabled={submitting}
        style={{
          backgroundColor: composed.submittable ? 'var(--color-accent)' : 'var(--color-text-muted)',
          border: 'none',
          borderRadius: 8,
          color: 'var(--color-on-accent)',
          cursor: submitting ? 'wait' : 'pointer',
          fontWeight: 700,
          marginTop: '1rem',
          padding: '0.6rem 1rem',
        }}
      >
        {composed.submittable
          ? 'Open prefilled GitHub issue form'
          : 'Preserve enhancement draft (not yet submittable)'}
      </button>

      <h3 style={{ marginBottom: '0.4rem', marginTop: '1.25rem' }}>Composed issue preview</h3>
      <pre
        style={{
          backgroundColor: 'var(--color-accent)',
          borderRadius: 12,
          color: 'var(--color-on-accent-muted)',
          fontSize: '0.8rem',
          lineHeight: 1.5,
          margin: 0,
          overflowX: 'auto',
          padding: '1rem',
          whiteSpace: 'pre-wrap',
        }}
      >
        {composed.draft.markdownBody}
      </pre>

      {outcome ? (
        <div
          style={{
            backgroundColor: 'var(--color-surface)',
            border: `1px solid ${outcomeTone(outcome.status)}`,
            borderRadius: 12,
            marginTop: '1rem',
            padding: '0.9rem 1rem',
          }}
        >
          <p style={{ color: outcomeTone(outcome.status), fontWeight: 700, margin: 0 }}>
            Submission status: {outcome.status}
          </p>
          <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>{outcome.message}</p>
          {outcome.resultHandle ? (
            <p style={{ color: 'var(--color-accent)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
              Filed issue: <a href={outcome.resultHandle.issueUrl}>{outcome.resultHandle.issueUrl}</a>
            </p>
          ) : (
            <p style={{ color: 'var(--color-warn)', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
              No issue handle was returned, so no successful submission is claimed. Your structured request is
              preserved below — copy it to file the issue manually without losing any evidence.
            </p>
          )}
          <button
            type="button"
            onClick={onCopyPayload}
            style={{
              backgroundColor: 'var(--color-accent)',
              border: 'none',
              borderRadius: 8,
              color: 'var(--color-on-accent)',
              cursor: 'pointer',
              fontWeight: 700,
              marginTop: '0.6rem',
              padding: '0.45rem 0.85rem',
            }}
          >
            {copied ? 'Copied governed draft' : 'Copy governed draft'}
          </button>
          <textarea
            readOnly
            value={outcome.copyablePayload}
            rows={10}
            style={{ ...bugFieldStyle(), fontFamily: 'monospace', marginTop: '0.6rem' }}
          />
        </div>
      ) : null}

      {handoff ? (
        <BrowserHandoffResultPanel
          handoff={handoff}
          copyablePayload={renderCopyableEnhancementPayload(composed.draft)}
        />
      ) : null}
    </section>
  );
}

/**
 * Character Hub Phase 3 — real `sd16/update` mount. Replaces `UpdateActionPanel`
 * now that a genuine controller adapter exists (real check/rollback wiring;
 * install stays honestly disabled pending a future slice). Loads mount-time
 * state via the real, already-tested `verify_relaunch_artifact` command
 * before rendering, so the page never opens with a stale placeholder state.
 */
function UpdateSection() {
  const [mounted, setMounted] = useState<{
    deps: UpdateControllerDeps;
    restoreOffer: MountTimeState['restoreOffer'];
  } | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);

  const refresh = useMemo(
    () => async () => {
      const mountTimeState = await loadMountTimeState();
      setMounted({
        deps: createUpdateControllerDeps(mountTimeState),
        restoreOffer: mountTimeState.restoreOffer,
      });
    },
    [],
  );

  useEffect(() => {
    refresh().catch((cause: unknown) => {
      setLoadError(cause instanceof Error ? cause.message : 'Unknown update-status load failure');
    });
  }, [refresh]);

  if (loadError) {
    return (
      <section style={{ backgroundColor: 'var(--color-error-bg)', border: '1px solid var(--color-error-border)', borderRadius: 12, padding: '1rem 1.25rem' }}>
        <h2 style={{ color: 'var(--color-error)', marginTop: 0 }}>Update status load failure</h2>
        <p style={{ color: 'var(--color-error)', marginBottom: 0, whiteSpace: 'pre-wrap' }}>{loadError}</p>
      </section>
    );
  }

  if (!mounted) {
    return <p style={{ color: 'var(--color-text-muted)', fontSize: '0.875rem' }}>Loading update status…</p>;
  }

  return (
    <UpdateUi
      initialDeps={mounted.deps}
      restoreOffer={
        mounted.restoreOffer
          ? {
              ...mounted.restoreOffer,
              onRestore: async () => {
                try {
                  setLoadError(null);
                  await restorePreviousVersion();
                  await refresh();
                } catch (cause: unknown) {
                  setLoadError(cause instanceof Error ? cause.message : 'Unknown restore failure');
                }
              },
            }
          : undefined
      }
    />
  );
}

function supportStateTone(state: string): 'info' | 'warning' | 'error' {
  if (state === 'supported') {
    return 'info';
  }
  if (state === 'blocked') {
    return 'error';
  }
  // partial, lossy, unverified, and any unexpected token all surface as debt.
  return 'warning';
}

function freshnessLabel(evidenceFreshness: string): string {
  if (evidenceFreshness === 'refreshable-from-live-proof') {
    return 'can be refreshed from live data';
  }
  if (evidenceFreshness === 'awaiting-initial-evidence') {
    return 'awaiting first check';
  }
  return evidenceFreshness;
}

const SUBJECT_TYPE_LABELS: Record<string, string> = {
  race: 'Race',
  class: 'Class',
  interaction: 'Rule interaction',
  school: 'Spell school',
  equipment: 'Equipment',
};

function subjectTypeLabel(subjectType: string): string {
  return SUBJECT_TYPE_LABELS[subjectType] ?? subjectType;
}

// Row subjectIds are namespaced like "race:human" or "category:arms_armor" —
// show only the human-readable part.
function humanizeSubjectId(subjectId: string): string {
  const colonIndex = subjectId.indexOf(':');
  const raw = colonIndex >= 0 ? subjectId.slice(colonIndex + 1) : subjectId;
  return raw
    .replace(/[-_]/g, ' ')
    .replace(/\b\w/g, (char) => char.toUpperCase());
}

// Other Pathfinder 1e sourcebooks with no support data at all yet — listed
// honestly as not started rather than fabricating a per-item breakdown for
// content that doesn't exist in the app.
const NOT_STARTED_SOURCEBOOKS = [
  "Advanced Player's Guide",
  'Advanced Class Guide',
  'Ultimate Combat',
  'Ultimate Magic',
  'Ultimate Equipment',
];

function SupportDebtPanel(props: { surface: Sd11TesterWorkbenchSurface }) {
  const debt = props.surface.supportDebt;

  if (!debt) {
    return null;
  }

  const blockingStates = new Set(['partial', 'lossy', 'blocked', 'unverified']);
  const coreRulebookIsFullySupported =
    debt.rows.length > 0 && debt.rows.every((row) => !blockingStates.has(row.supportState));

  return (
    <section style={{ border: '1px solid var(--color-border)', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
      <h2 style={{ marginTop: 0 }}>{debt.sectionLabel}</h2>
      <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6 }}>{debt.lead}</p>

      {debt.unavailableNotice ? (
        <div style={{ backgroundColor: 'var(--color-warn-bg)', border: '1px solid var(--color-warn-border)', borderRadius: 12, padding: '0.9rem 1rem' }}>
          <p style={{ color: 'var(--color-warn)', margin: 0 }}>{debt.unavailableNotice}</p>
        </div>
      ) : (
        <>
          <div style={{ alignItems: 'baseline', display: 'flex', gap: '0.6rem', marginBottom: '0.75rem', marginTop: '1.25rem' }}>
            <h3 style={{ margin: 0 }}>Core Rulebook</h3>
            <span style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem' }}>
              {coreRulebookIsFullySupported ? 'Fully supported' : 'Partially supported — see breakdown below'}
            </span>
          </div>

          {debt.stateCounts.length ? (
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '1rem' }}>
              {debt.stateCounts.map((tally) => (
                <span
                  key={tally.supportState}
                  style={{
                    backgroundColor: 'var(--color-surface-2)',
                    border: `1px solid ${toneColor(supportStateTone(tally.supportState))}`,
                    borderRadius: 999,
                    color: toneColor(supportStateTone(tally.supportState)),
                    fontSize: '0.75rem',
                    fontWeight: 700,
                    letterSpacing: '0.04em',
                    padding: '0.2rem 0.6rem',
                    textTransform: 'uppercase',
                  }}
                >
                  {tally.supportState}: {tally.count}
                </span>
              ))}
            </div>
          ) : null}

          <div style={{ display: 'grid', gap: '0.75rem' }}>
            {debt.rows.map((row) => (
              <div
                key={row.rowId}
                style={{
                  backgroundColor: 'var(--color-surface)',
                  border: `1px solid ${toneColor(supportStateTone(row.supportState))}`,
                  borderRadius: 12,
                  padding: '0.9rem 1rem',
                }}
              >
                <p
                  style={{
                    color: toneColor(supportStateTone(row.supportState)),
                    fontSize: '0.75rem',
                    fontWeight: 700,
                    letterSpacing: '0.08em',
                    margin: 0,
                    textTransform: 'uppercase',
                  }}
                >
                  {row.supportState} · evidence: {row.evidenceTier}
                </p>
                <p style={{ color: 'var(--color-text)', fontSize: '1rem', fontWeight: 700, margin: '0.35rem 0 0' }}>
                  {humanizeSubjectId(row.subjectId)}{' '}
                  <span style={{ color: 'var(--color-text-muted)', fontWeight: 400 }}>({subjectTypeLabel(row.subjectType)})</span>
                </p>
                <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0.5rem 0 0' }}>{row.testerFacingStateLabel}</p>
              </div>
            ))}
          </div>

          <div style={{ marginTop: '1.75rem' }}>
            <h3 style={{ marginBottom: '0.5rem' }}>Other sourcebooks</h3>
            <div style={{ display: 'grid', gap: '0.5rem' }}>
              {NOT_STARTED_SOURCEBOOKS.map((book) => (
                <div
                  key={book}
                  style={{
                    alignItems: 'center',
                    backgroundColor: 'var(--color-surface)',
                    border: '1px solid var(--color-border)',
                    borderRadius: 10,
                    display: 'flex',
                    justifyContent: 'space-between',
                    padding: '0.6rem 0.9rem',
                  }}
                >
                  <span>{book}</span>
                  <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', fontWeight: 700, letterSpacing: '0.04em', textTransform: 'uppercase' }}>
                    Not started
                  </span>
                </div>
              ))}
            </div>
          </div>
        </>
      )}
    </section>
  );
}

function BreadthClaimAuditPanel(props: { surface: Sd11TesterWorkbenchSurface }) {
  const audit = props.surface.breadthClaimAudit;

  if (!audit) {
    return null;
  }

  const postureTone: 'info' | 'warning' = audit.overallPosture === 'refresh-backed' ? 'info' : 'warning';

  return (
    <section style={{ border: '1px dashed var(--color-text-faint)', borderRadius: 12, marginTop: '1rem', padding: '1.25rem' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
        Checks the support status above
      </p>
      <h3 style={{ margin: '0.35rem 0 0' }}>{audit.sectionLabel}</h3>
      <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6 }}>{audit.lead}</p>

      {audit.unavailableNotice ? (
        <div style={{ backgroundColor: 'var(--color-warn-bg)', border: '1px solid var(--color-warn-border)', borderRadius: 12, padding: '0.9rem 1rem' }}>
          <p style={{ color: 'var(--color-warn)', margin: 0 }}>{audit.unavailableNotice}</p>
        </div>
      ) : (
        <>
          <div
            style={{
              backgroundColor: 'var(--color-surface)',
              border: `1px solid ${toneColor(postureTone)}`,
              borderRadius: 12,
              padding: '0.9rem 1rem',
            }}
          >
            <p style={{ color: toneColor(postureTone), fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
              {audit.overallPosture === 'refresh-backed' ? 'Freshness-verified' : 'Needs a freshness check'}
            </p>
            <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0.35rem 0 0' }}>{audit.overallLabel}</p>
            <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem', margin: '0.45rem 0 0' }}>
              Rows needing a refresh: {audit.refreshRequiredCount} · freshness-verified rows: {audit.positiveBreadthClaimCount}
            </p>
          </div>

          {audit.freshnessCounts.length ? (
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', margin: '1rem 0' }}>
              {audit.freshnessCounts.map((tally) => (
                <span
                  key={tally.evidenceFreshness}
                  style={{
                    backgroundColor: 'var(--color-surface-2)',
                    border: '1px solid var(--color-text-faint)',
                    borderRadius: 999,
                    color: 'var(--color-text-secondary)',
                    fontSize: '0.75rem',
                    fontWeight: 700,
                    letterSpacing: '0.04em',
                    padding: '0.2rem 0.6rem',
                    textTransform: 'uppercase',
                  }}
                >
                  {freshnessLabel(tally.evidenceFreshness)}: {tally.count}
                </span>
              ))}
            </div>
          ) : null}

          <div style={{ display: 'grid', gap: '0.6rem' }}>
            {audit.rows.map((row) => (
              <div
                key={row.rowId}
                style={{
                  backgroundColor: 'var(--color-surface)',
                  border: '1px solid var(--color-border)',
                  borderRadius: 10,
                  padding: '0.75rem 0.9rem',
                }}
              >
                <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.06em', margin: 0, textTransform: 'uppercase' }}>
                  {freshnessLabel(row.evidenceFreshness)} · {row.supportState} · evidence: {row.evidenceTier}
                </p>
                <p style={{ color: 'var(--color-text)', fontSize: '0.95rem', fontWeight: 700, margin: '0.3rem 0 0' }}>
                  {humanizeSubjectId(row.subjectId)}{' '}
                  <span style={{ color: 'var(--color-text-muted)', fontWeight: 400 }}>({subjectTypeLabel(row.subjectType)})</span>
                </p>
                <p style={{ color: 'var(--color-warn)', lineHeight: 1.6, margin: '0.4rem 0 0' }}>{row.refreshAuditLabel}</p>
              </div>
            ))}
          </div>
        </>
      )}
    </section>
  );
}

export default function App() {
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [settingsTab, setSettingsTab] = useState<SettingsTab>('appearance');
  const [themeMode, setThemeMode] = useState<ThemeMode>(getStoredThemeMode);
  const [surface, setSurface] = useState<Sd11TesterWorkbenchSurface | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    applyThemeMode(themeMode);
    applyObsidianModeClass(themeMode);
  }, [themeMode]);

  // Re-inject any persisted community theme's CSS on boot — localStorage
  // survives reloads, but the injected <style> tags themselves do not.
  useEffect(() => {
    applyActiveTheme();
  }, []);

  // One-time fetch of featured community themes (e.g. Ayu Mirage) so they
  // show up pre-installed in the Theme dropdown without a manual browse step.
  // Does not change the active theme — it only makes the option available.
  useEffect(() => {
    seedBuiltinThemes();
  }, []);


  useEffect(() => {
    loadSd11TesterWorkbenchSurfaceRuntime({
      buildVersion: desktopPackage.version,
      platformLabel: derivePlatformLabel(),
    })
      .then(setSurface)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown developer diagnostics failure');
      });
  }, []);

  return (
    <main style={{ fontFamily: 'Inter, system-ui, sans-serif', margin: '0 auto', maxWidth: 1100, padding: '3rem 1.5rem' }}>
      {/* Settings gear — fixed to the viewport top-right on every screen. */}
      <button
        type="button"
        aria-label="Open settings"
        title="Settings"
        onClick={() => setSettingsOpen(true)}
        style={{
          alignItems: 'center',
          background: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 8,
          color: 'var(--color-text-secondary)',
          cursor: 'pointer',
          display: 'flex',
          fontSize: '1.2rem',
          height: 38,
          justifyContent: 'center',
          position: 'fixed',
          right: 14,
          top: 8,
          width: 38,
          zIndex: 950,
        }}
      >
        ⚙
      </button>

      <CharacterHubPage />

      <SettingsModal
        open={settingsOpen}
        activeTab={settingsTab}
        onTabChange={setSettingsTab}
        onClose={() => setSettingsOpen(false)}
        panels={{
          appearance: <AppearancePanel mode={themeMode} onModeChange={setThemeMode} />,
          'google-drive': <GoogleDrivePanel />,
          update: <UpdateSection />,
          bug: surface ? (
            <BugReportComposer surface={surface} />
          ) : (
            <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>Workbench data is unavailable, so a bug report cannot be composed right now.</p>
          ),
          enhancement: surface ? (
            <EnhancementRequestComposer surface={surface} />
          ) : (
            <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>Workbench data is unavailable, so an enhancement request cannot be composed right now.</p>
          ),
          developer: (
            <>
          <header>
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.875rem', letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
              {surface?.surfaceLabel ?? 'Developer diagnostics'}
            </p>
            <h1 style={{ marginBottom: '0.5rem' }}>{surface?.headline ?? 'Connecting to the app backend…'}</h1>
            <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, marginBottom: 0 }}>
              {surface?.lead ?? 'Loading backend diagnostics.'}
            </p>
          </header>

          {error ? (
        <section style={{ backgroundColor: 'var(--color-error-bg)', border: '1px solid var(--color-error-border)', borderRadius: 12, marginTop: '1.5rem', padding: '1rem 1.25rem' }}>
          <h2 style={{ color: 'var(--color-error)', marginTop: 0 }}>Failed to load diagnostics</h2>
          <p style={{ color: 'var(--color-error)', marginBottom: 0, whiteSpace: 'pre-wrap' }}>{error}</p>
        </section>
      ) : null}

      {surface ? (
        <>
          <section style={{ display: 'grid', gap: '0.9rem', gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))', marginTop: '2rem' }}>
            <AppCard label="Build" value={surface.status.build.label} detail={surface.status.update.label} />
            <AppCard label="Platform" value={surface.platformLabel} detail={surface.status.support.currentPlatformSupportLabel} />
            <AppCard
              label="Backend"
              value={surface.backendHealth?.reachable ? `v${surface.backendHealth.version} · ${surface.backendHealth.gitCommit}` : 'Unreachable'}
              detail={
                surface.backendHealth?.reachable
                  ? 'Rust backend version and the commit this build was compiled from.'
                  : surface.backendHealth?.unavailableNotice ?? 'Backend health has not been checked yet.'
              }
            />
            <AppCard label="Feature check" value={surface.workflowName} detail={surface.workflowState} />
            <AppCard label="Data source" value={surface.dataTruthLabel} detail="Live backend data when reachable, clearly-labeled fallback data when it isn't." />
          </section>

          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', lineHeight: 1.6, marginTop: '0.75rem' }}>
            There is no database in this app. Rule content (races, classes, spells, equipment) is built directly
            into this version of the app; your saved characters are stored as local files on this device.
          </p>

          {surface.fallbackNotice ? (
            <section style={{ backgroundColor: 'var(--color-warn-bg)', border: '1px solid var(--color-warn-border)', borderRadius: 12, marginTop: '1.5rem', padding: '1rem 1.25rem' }}>
              <h2 style={{ color: 'var(--color-warn)', marginTop: 0 }}>Showing fallback data</h2>
              <p style={{ color: 'var(--color-warn)', marginBottom: 0 }}>{surface.fallbackNotice}</p>
            </section>
          ) : null}

          {surface.summaryRows.length ? (
            <section style={{ border: '1px solid var(--color-border)', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
              <h2 style={{ marginTop: 0 }}>Backend feature check</h2>
              <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6 }}>{surface.boundedScopeNotice}</p>
              <div style={{ display: 'grid', gap: '0.75rem', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', marginTop: '1rem' }}>
                {surface.summaryRows.map((row) => (
                  <AppCard key={row.label} label={row.label} value={row.value} />
                ))}
              </div>
            </section>
          ) : null}

          <section style={{ border: '1px solid var(--color-border)', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
            <h2 style={{ marginTop: 0 }}>Diagnostics</h2>
            <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6 }}>{surface.feedbackStatusNotice}</p>

            <div style={{ display: 'grid', gap: '0.75rem', marginTop: '1rem' }}>
              {surface.diagnostics.length ? (
                surface.diagnostics.map((diagnostic, index) => (
                  <div
                    key={`${diagnostic.classLabel}-${index}`}
                    style={{
                      backgroundColor: 'var(--color-surface)',
                      border: '1px solid var(--color-border)',
                      borderRadius: 12,
                      padding: '0.9rem 1rem',
                    }}
                  >
                    <p style={{ color: toneColor(diagnostic.severity), fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
                      {diagnostic.severityLabel} · {diagnostic.classLabel}
                    </p>
                    <p style={{ margin: '0.45rem 0 0' }}>{diagnostic.message}</p>
                    {diagnostic.subjectRef ? (
                      <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.875rem', margin: '0.45rem 0 0' }}>
                        Subject reference: <code>{diagnostic.subjectRef}</code>
                      </p>
                    ) : null}
                    {diagnostic.claimBlocking ? (
                      <p style={{ color: 'var(--color-warn)', fontSize: '0.875rem', margin: '0.45rem 0 0' }}>
                        This diagnostic blocks at least one claim in the bounded snapshot.
                      </p>
                    ) : null}
                  </div>
                ))
              ) : (
                <p style={{ color: 'var(--color-text-secondary)', margin: 0 }}>No diagnostics for this check.</p>
              )}
            </div>

            <div style={{ display: 'grid', gap: '1rem', gridTemplateColumns: 'repeat(auto-fit, minmax(260px, 1fr))', marginTop: '1rem' }}>
              <div>
                <h3 style={{ marginBottom: '0.5rem' }}>Blocked calculations</h3>
                <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.875rem', lineHeight: 1.5, marginTop: 0 }}>
                  Things this check tried to compute but couldn't — distinct from features that are simply out of scope for this check.
                </p>
                {surface.blockedClaims.length ? (
                  <ul style={{ margin: 0, paddingLeft: '1.2rem' }}>
                    {surface.blockedClaims.map((claim) => (
                      <li key={claim} style={{ marginBottom: '0.45rem' }}>{claim}</li>
                    ))}
                  </ul>
                ) : (
                  <p style={{ color: 'var(--color-text-secondary)', margin: 0 }}>Nothing was blocked in this check.</p>
                )}
              </div>
              <div>
                <h3 style={{ marginBottom: '0.5rem' }}>Explanations</h3>
                <EvidenceList emptyMessage="No explanations for this check." items={surface.explanationRefs} />
              </div>
              <div>
                <h3 style={{ marginBottom: '0.5rem' }}>Sources</h3>
                <EvidenceList emptyMessage="No sources for this check." items={surface.provenanceRefs} />
              </div>
            </div>
          </section>

          <section style={{ border: '1px solid var(--color-border)', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
            <h2 style={{ marginTop: 0 }}>Update &amp; platform support</h2>
            <div style={{ display: 'grid', gap: '0.9rem', gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))' }}>
              <AppCard label="Build track" value={surface.status.channel.testerFacingLabel} detail={surface.status.channel.detail} />
              <AppCard label="Platform support" value={surface.status.support.tierMatrixLabel} detail={surface.status.support.currentPlatformSupportLabel} />
              <AppCard label="Update status" value={surface.status.update.label} detail={`${surface.status.channel.testerFacingLabel} build on ${surface.status.support.currentPlatformSupportLabel}`} />
            </div>
            <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, marginBottom: '0.75rem', marginTop: '1rem' }}>
              {surface.status.support.platformSupportDetail}
            </p>
            <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, marginBottom: 0 }}>{surface.status.update.detail}</p>
          </section>

          <SupportDebtPanel surface={surface} />

          <BreadthClaimAuditPanel surface={surface} />

          <FeedbackEvidencePanel surface={surface} />

          {surface.notes.length ? (
            <section style={{ border: '1px solid var(--color-border)', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
              <h2 style={{ marginTop: 0 }}>Notes</h2>
              <ul style={{ marginBottom: 0, paddingLeft: '1.2rem' }}>
                {surface.notes.map((note) => (
                  <li key={note} style={{ marginBottom: '0.45rem' }}>{note}</li>
                ))}
              </ul>
            </section>
          ) : null}
            </>
          ) : null}
            </>
          ),
        }}
      />
    </main>
  );
}

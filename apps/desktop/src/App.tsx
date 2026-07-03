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
  submitBugReport,
  type BugReportSubmissionOutcome,
} from './sd11/feedback/bug';
import {
  composeEnhancementRequest,
  submitEnhancementRequest,
  type EnhancementRequestSubmissionOutcome,
} from './sd11/feedback/enhancement';
import { loadSd11UpdateAction } from './boundary/loadSd11UpdateAction';
import {
  deriveSd11UpdateAction,
  type Sd11UpdateActionResult,
  type Sd11UpdateCheckContext,
  type Sd11UpdateResultState,
} from './sd11/update';

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
    return '#b91c1c';
  }

  if (tone === 'warning') {
    return '#b45309';
  }

  return '#0f766e';
}

function AppCard(props: { label: string; value: string; detail?: string }) {
  return (
    <div
      style={{
        backgroundColor: '#f8fafc',
        border: '1px solid #cbd5e1',
        borderRadius: 12,
        padding: '0.9rem 1rem',
      }}
    >
      <p style={{ color: '#64748b', fontSize: '0.75rem', letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
        {props.label}
      </p>
      <p style={{ color: '#0f172a', fontSize: '1rem', fontWeight: 700, margin: '0.35rem 0 0' }}>{props.value}</p>
      {props.detail ? <p style={{ color: '#475569', fontSize: '0.875rem', margin: '0.4rem 0 0' }}>{props.detail}</p> : null}
    </div>
  );
}

function EvidenceList(props: {
  emptyMessage: string;
  items: Array<{ label: string; detail: string; machineRef: string }>;
}) {
  if (!props.items.length) {
    return <p style={{ color: '#475569', margin: 0 }}>{props.emptyMessage}</p>;
  }

  return (
    <ul style={{ margin: 0, paddingLeft: '1.2rem' }}>
      {props.items.map((reference, index) => (
        // GE08 refs can share label and machineRef while differing in detail,
        // so the list index keeps sibling keys unique.
        <li key={createReferenceListKey(reference.label, `${reference.machineRef}#${index}`)} style={{ marginBottom: '0.6rem' }}>
          <strong>{reference.label}</strong>
          <div style={{ color: '#475569', marginTop: '0.2rem' }}>{reference.detail}</div>
          <code style={{ color: '#334155', fontSize: '0.8rem' }}>{reference.machineRef}</code>
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
    <section style={{ border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
      <h2 style={{ marginTop: 0 }}>Feedback evidence capture &amp; redaction</h2>
      <p style={{ color: '#475569', lineHeight: 1.6 }}>
        Shared substrate for the upcoming GitHub bug-report and enhancement-request flows. The auto-captured
        backbone below is reused by both flows without schema drift; tester-entered narrative fields are added
        by each composer, and attachments are never captured silently.
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

      <div style={{ backgroundColor: '#f8fafc', border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '1rem', padding: '0.9rem 1rem' }}>
        <p style={{ color: '#64748b', fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
          Attachment &amp; redaction policy
        </p>
        <p style={{ color: '#475569', lineHeight: 1.6, margin: '0.45rem 0 0' }}>{REDACTION_POLICY_NOTICE}</p>
        <p style={{ color: '#475569', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
          Attachments captured in this substrate slice: none. No screenshot, log, or save file is collected without
          explicit tester confirmation and a recorded redaction declaration.
        </p>
      </div>
    </section>
  );
}

function bugFieldStyle(): CSSProperties {
  return {
    border: '1px solid #cbd5e1',
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
      <span style={{ color: '#334155', fontSize: '0.8rem', fontWeight: 700, letterSpacing: '0.04em', textTransform: 'uppercase' }}>
        {props.label}
      </span>
      {props.multiline ? <textarea rows={3} {...sharedProps} /> : <input type="text" {...sharedProps} />}
    </label>
  );
}

function outcomeTone(status: BugReportSubmissionOutcome['status']): string {
  if (status === 'submitted') {
    return '#0f766e';
  }

  if (status === 'blocked-incomplete') {
    return '#b91c1c';
  }

  return '#b45309';
}

function BugReportComposer(props: { surface: Sd11TesterWorkbenchSurface }) {
  const [title, setTitle] = useState('');
  const [observedBehavior, setObservedBehavior] = useState('');
  const [expectedBehavior, setExpectedBehavior] = useState('');
  const [reproductionSteps, setReproductionSteps] = useState('');
  const [outcome, setOutcome] = useState<BugReportSubmissionOutcome | null>(null);
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

  // No GitHub transport is wired in this slice. We never improvise secret-bearing
  // auth, so submission preserves a governed draft instead of counterfeiting success.
  async function onPrepareSubmission() {
    setSubmitting(true);
    setCopied(false);
    try {
      const result = await submitBugReport({ composed, transport: null });
      setOutcome(result);
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
    <section style={{ border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
      <h2 style={{ marginTop: 0 }}>Report a bug</h2>
      <p style={{ color: '#475569', lineHeight: 1.6 }}>
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

      <div style={{ backgroundColor: '#f8fafc', border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '1rem', padding: '0.9rem 1rem' }}>
        <p style={{ color: '#64748b', fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
          Auto-captured metadata · attached to every report
        </p>
        <p style={{ color: '#475569', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
          Labels: {composed.draft.labels.join(', ')}
        </p>
        <p style={{ color: '#475569', lineHeight: 1.6, margin: '0.45rem 0 0' }}>{REDACTION_POLICY_NOTICE}</p>
      </div>

      {composed.problems.length ? (
        <div style={{ backgroundColor: '#fffbeb', border: '1px solid #fde68a', borderRadius: 12, marginTop: '1rem', padding: '0.9rem 1rem' }}>
          <p style={{ color: '#92400e', fontWeight: 700, margin: 0 }}>Still required before this report can be submitted</p>
          <ul style={{ color: '#92400e', margin: '0.45rem 0 0', paddingLeft: '1.2rem' }}>
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
          backgroundColor: composed.submittable ? '#0f766e' : '#64748b',
          border: 'none',
          borderRadius: 8,
          color: '#ffffff',
          cursor: submitting ? 'wait' : 'pointer',
          fontWeight: 700,
          marginTop: '1rem',
          padding: '0.6rem 1rem',
        }}
      >
        {composed.submittable ? 'Prepare bug draft for manual filing' : 'Preserve bug draft (not yet submittable)'}
      </button>

      <h3 style={{ marginBottom: '0.4rem', marginTop: '1.25rem' }}>Composed issue preview</h3>
      <pre
        style={{
          backgroundColor: '#0f172a',
          borderRadius: 12,
          color: '#e2e8f0',
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
            backgroundColor: '#f8fafc',
            border: `1px solid ${outcomeTone(outcome.status)}`,
            borderRadius: 12,
            marginTop: '1rem',
            padding: '0.9rem 1rem',
          }}
        >
          <p style={{ color: outcomeTone(outcome.status), fontWeight: 700, margin: 0 }}>
            Submission status: {outcome.status}
          </p>
          <p style={{ color: '#475569', lineHeight: 1.6, margin: '0.45rem 0 0' }}>{outcome.message}</p>
          {outcome.resultHandle ? (
            <p style={{ color: '#0f766e', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
              Filed issue: <a href={outcome.resultHandle.issueUrl}>{outcome.resultHandle.issueUrl}</a>
            </p>
          ) : (
            <p style={{ color: '#7c2d12', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
              No issue handle was returned, so no successful submission is claimed. Your structured report is
              preserved below — copy it to file the issue manually without losing any evidence.
            </p>
          )}
          <button
            type="button"
            onClick={onCopyPayload}
            style={{
              backgroundColor: '#334155',
              border: 'none',
              borderRadius: 8,
              color: '#ffffff',
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

  // No GitHub transport is wired in this slice. We never improvise secret-bearing
  // auth, so submission preserves a governed draft instead of counterfeiting success.
  async function onPrepareSubmission() {
    setSubmitting(true);
    setCopied(false);
    try {
      const result = await submitEnhancementRequest({ composed, transport: null });
      setOutcome(result);
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
    <section style={{ border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
      <h2 style={{ marginTop: 0 }}>Request an enhancement</h2>
      <p style={{ color: '#475569', lineHeight: 1.6 }}>
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

      <div style={{ backgroundColor: '#f8fafc', border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '1rem', padding: '0.9rem 1rem' }}>
        <p style={{ color: '#64748b', fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
          Auto-captured metadata · attached to every request
        </p>
        <p style={{ color: '#475569', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
          Labels: {composed.draft.labels.join(', ')}
        </p>
        <p style={{ color: '#475569', lineHeight: 1.6, margin: '0.45rem 0 0' }}>{REDACTION_POLICY_NOTICE}</p>
      </div>

      {composed.problems.length ? (
        <div style={{ backgroundColor: '#fffbeb', border: '1px solid #fde68a', borderRadius: 12, marginTop: '1rem', padding: '0.9rem 1rem' }}>
          <p style={{ color: '#92400e', fontWeight: 700, margin: 0 }}>Still required before this request can be submitted</p>
          <ul style={{ color: '#92400e', margin: '0.45rem 0 0', paddingLeft: '1.2rem' }}>
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
          backgroundColor: composed.submittable ? '#0f766e' : '#64748b',
          border: 'none',
          borderRadius: 8,
          color: '#ffffff',
          cursor: submitting ? 'wait' : 'pointer',
          fontWeight: 700,
          marginTop: '1rem',
          padding: '0.6rem 1rem',
        }}
      >
        {composed.submittable
          ? 'Prepare enhancement draft for manual filing'
          : 'Preserve enhancement draft (not yet submittable)'}
      </button>

      <h3 style={{ marginBottom: '0.4rem', marginTop: '1.25rem' }}>Composed issue preview</h3>
      <pre
        style={{
          backgroundColor: '#0f172a',
          borderRadius: 12,
          color: '#e2e8f0',
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
            backgroundColor: '#f8fafc',
            border: `1px solid ${outcomeTone(outcome.status)}`,
            borderRadius: 12,
            marginTop: '1rem',
            padding: '0.9rem 1rem',
          }}
        >
          <p style={{ color: outcomeTone(outcome.status), fontWeight: 700, margin: 0 }}>
            Submission status: {outcome.status}
          </p>
          <p style={{ color: '#475569', lineHeight: 1.6, margin: '0.45rem 0 0' }}>{outcome.message}</p>
          {outcome.resultHandle ? (
            <p style={{ color: '#0f766e', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
              Filed issue: <a href={outcome.resultHandle.issueUrl}>{outcome.resultHandle.issueUrl}</a>
            </p>
          ) : (
            <p style={{ color: '#7c2d12', lineHeight: 1.6, margin: '0.45rem 0 0' }}>
              No issue handle was returned, so no successful submission is claimed. Your structured request is
              preserved below — copy it to file the issue manually without losing any evidence.
            </p>
          )}
          <button
            type="button"
            onClick={onCopyPayload}
            style={{
              backgroundColor: '#334155',
              border: 'none',
              borderRadius: 8,
              color: '#ffffff',
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
    </section>
  );
}

function updateStateTone(state: Sd11UpdateResultState): 'info' | 'warning' | 'error' {
  if (state === 'up-to-date' || state === 'update-available') {
    return 'info';
  }
  if (state === 'manual-only' || state === 'unsupported' || state === 'no-official-release-for-this-build') {
    return 'warning';
  }
  return 'error';
}

function UpdateActionPanel(props: { surface: Sd11TesterWorkbenchSurface }) {
  const [result, setResult] = useState<Sd11UpdateActionResult | null>(null);
  const [checking, setChecking] = useState(false);

  const context = useMemo<Sd11UpdateCheckContext>(
    () => ({
      buildLabel: props.surface.status.build.label,
      buildVersion: props.surface.status.build.version,
      platformLabel: props.surface.status.support.platformLabel,
      platformTier: props.surface.status.support.platformTier,
      testerChannelLabel: props.surface.status.channel.testerFacingLabel,
    }),
    [props.surface]
  );

  // Read-only check only. This slice classifies honest update state over governed SD-12 truth; it
  // never downloads, applies, or mutates a release, and never counterfeits success.
  async function onCheck() {
    setChecking(true);
    try {
      const truth = await loadSd11UpdateAction({
        buildVersion: context.buildVersion,
        buildLabel: context.buildLabel,
        platformLabel: context.platformLabel,
        testerChannelLabel: context.testerChannelLabel,
      });
      setResult(deriveSd11UpdateAction(context, truth));
    } finally {
      setChecking(false);
    }
  }

  const tone = result ? updateStateTone(result.state) : 'info';

  return (
    <section style={{ border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
      <h2 style={{ marginTop: 0 }}>Check for updates</h2>
      <p style={{ color: '#475569', lineHeight: 1.6 }}>
        Run a bounded, read-only update check against governed SD-12 release truth. SD-11 consumes release
        truth as a client; it never publishes, applies, or mutates releases, and it never promotes macOS or
        Windows to automatic-update parity. Outcomes stay explicit and platform-aware.
      </p>

      <button
        type="button"
        onClick={onCheck}
        disabled={checking}
        style={{
          backgroundColor: '#0f766e',
          border: 'none',
          borderRadius: 8,
          color: '#ffffff',
          cursor: checking ? 'wait' : 'pointer',
          fontWeight: 700,
          marginTop: '0.25rem',
          padding: '0.6rem 1rem',
        }}
      >
        {checking ? 'Checking…' : 'Check for updates'}
      </button>

      {result ? (
        <div
          style={{
            backgroundColor: '#f8fafc',
            border: `1px solid ${toneColor(tone)}`,
            borderRadius: 12,
            marginTop: '1rem',
            padding: '0.9rem 1rem',
          }}
        >
          <p style={{ color: toneColor(tone), fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
            Result: {result.state}
          </p>
          <p style={{ color: '#0f172a', fontSize: '1rem', fontWeight: 700, margin: '0.35rem 0 0' }}>{result.headline}</p>
          <p style={{ color: '#475569', lineHeight: 1.6, margin: '0.45rem 0 0' }}>{result.detail}</p>
          <p style={{ color: '#475569', fontSize: '0.875rem', margin: '0.45rem 0 0' }}>
            Checked build: <code>{result.checkedBuildLabel}</code> · platform {result.platformLabel} ({result.platformTier}) ·{' '}
            {result.testerChannelLabel} track
          </p>
          <p style={{ color: result.automaticEligible ? '#0f766e' : '#b45309', fontSize: '0.875rem', margin: '0.45rem 0 0' }}>
            {result.automaticEligible
              ? 'Automatic-update eligible (Linux first-class, SD-12 integrity gate satisfied).'
              : 'Not automatic-update eligible in this context.'}
          </p>
          {result.manualReason ? (
            <p style={{ color: '#7c2d12', fontSize: '0.875rem', margin: '0.45rem 0 0' }}>{result.manualReason}</p>
          ) : null}
          {result.replacementTarget ? (
            <p style={{ color: '#7c2d12', fontSize: '0.875rem', margin: '0.45rem 0 0' }}>
              Preferred replacement release: <code>{result.replacementTarget}</code>
            </p>
          ) : null}
          {result.recoveryDirection ? (
            <p style={{ color: '#7c2d12', fontSize: '0.875rem', margin: '0.45rem 0 0' }}>
              Recovery target: <code>{result.recoveryDirection}</code>
            </p>
          ) : null}
          {result.operatorPromotionPathReference ? (
            <p style={{ color: '#64748b', fontSize: '0.8rem', margin: '0.45rem 0 0' }}>
              Operator provenance: {result.operatorPromotionPathReference}
            </p>
          ) : null}
          {result.evidenceNotes.length ? (
            <ul style={{ color: '#475569', fontSize: '0.85rem', margin: '0.45rem 0 0', paddingLeft: '1.2rem' }}>
              {result.evidenceNotes.map((note) => (
                <li key={note}>{note}</li>
              ))}
            </ul>
          ) : null}
        </div>
      ) : (
        <p style={{ color: '#64748b', fontSize: '0.875rem', marginTop: '0.75rem' }}>
          No check has been run yet. Results are reported explicitly and are never silently assumed successful.
        </p>
      )}
    </section>
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

function SupportDebtPanel(props: { surface: Sd11TesterWorkbenchSurface }) {
  const debt = props.surface.supportDebt;

  if (!debt) {
    return null;
  }

  return (
    <section style={{ border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
      <h2 style={{ marginTop: 0 }}>{debt.sectionLabel}</h2>
      <p style={{ color: '#475569', lineHeight: 1.6 }}>{debt.lead}</p>

      {debt.unavailableNotice ? (
        <div style={{ backgroundColor: '#fff7ed', border: '1px solid #fdba74', borderRadius: 12, padding: '0.9rem 1rem' }}>
          <p style={{ color: '#7c2d12', margin: 0 }}>{debt.unavailableNotice}</p>
        </div>
      ) : (
        <>
          {debt.stateCounts.length ? (
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '1rem' }}>
              {debt.stateCounts.map((tally) => (
                <span
                  key={tally.supportState}
                  style={{
                    backgroundColor: '#f1f5f9',
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
                  backgroundColor: '#f8fafc',
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
                <p style={{ color: '#0f172a', fontSize: '1rem', fontWeight: 700, margin: '0.35rem 0 0' }}>
                  {row.subjectId} <span style={{ color: '#64748b', fontWeight: 400 }}>({row.subjectType})</span>
                </p>
                <p style={{ color: '#475569', margin: '0.3rem 0 0' }}>{row.dimension}</p>
                <p style={{ color: '#334155', lineHeight: 1.6, margin: '0.5rem 0 0' }}>{row.testerFacingStateLabel}</p>
                {row.hasDebtNote ? (
                  <p style={{ color: '#7c2d12', lineHeight: 1.6, margin: '0.5rem 0 0' }}>
                    <strong>Blocker / lossiness:</strong> {row.blockerOrLossinessNote}
                  </p>
                ) : null}
                <p style={{ color: '#475569', fontSize: '0.85rem', margin: '0.5rem 0 0' }}>
                  Grounding: <code style={{ color: '#334155' }}>{row.groundingRef}</code>
                </p>
                <p style={{ color: '#475569', fontSize: '0.85rem', margin: '0.3rem 0 0' }}>
                  Next uplift: {row.nextRequiredUplift}
                </p>
                <p style={{ margin: '0.4rem 0 0' }}>
                  <code style={{ color: '#94a3b8', fontSize: '0.75rem' }}>{row.rowId}</code>
                </p>
              </div>
            ))}
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
    <section style={{ border: '1px dashed #94a3b8', borderRadius: 12, marginTop: '1rem', padding: '1.25rem' }}>
      <p style={{ color: '#64748b', fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
        Audit surface · subordinate to the support and debt truth above
      </p>
      <h3 style={{ margin: '0.35rem 0 0' }}>{audit.sectionLabel}</h3>
      <p style={{ color: '#475569', lineHeight: 1.6 }}>{audit.lead}</p>

      {audit.unavailableNotice ? (
        <div style={{ backgroundColor: '#fff7ed', border: '1px solid #fdba74', borderRadius: 12, padding: '0.9rem 1rem' }}>
          <p style={{ color: '#7c2d12', margin: 0 }}>{audit.unavailableNotice}</p>
        </div>
      ) : (
        <>
          <div
            style={{
              backgroundColor: '#f8fafc',
              border: `1px solid ${toneColor(postureTone)}`,
              borderRadius: 12,
              padding: '0.9rem 1rem',
            }}
          >
            <p style={{ color: toneColor(postureTone), fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
              Overall posture: {audit.overallPosture}
            </p>
            <p style={{ color: '#334155', lineHeight: 1.6, margin: '0.35rem 0 0' }}>{audit.overallLabel}</p>
            <p style={{ color: '#475569', fontSize: '0.85rem', margin: '0.45rem 0 0' }}>
              Refresh-required rows: {audit.refreshRequiredCount} · promoted breadth claims: {audit.positiveBreadthClaimCount}
            </p>
          </div>

          {audit.freshnessCounts.length ? (
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', margin: '1rem 0' }}>
              {audit.freshnessCounts.map((tally) => (
                <span
                  key={tally.evidenceFreshness}
                  style={{
                    backgroundColor: '#f1f5f9',
                    border: '1px solid #94a3b8',
                    borderRadius: 999,
                    color: '#334155',
                    fontSize: '0.75rem',
                    fontWeight: 700,
                    letterSpacing: '0.04em',
                    padding: '0.2rem 0.6rem',
                    textTransform: 'uppercase',
                  }}
                >
                  {tally.evidenceFreshness}: {tally.count}
                </span>
              ))}
            </div>
          ) : null}

          <div style={{ display: 'grid', gap: '0.6rem' }}>
            {audit.rows.map((row) => (
              <div
                key={row.rowId}
                style={{
                  backgroundColor: '#ffffff',
                  border: '1px solid #cbd5e1',
                  borderRadius: 10,
                  padding: '0.75rem 0.9rem',
                }}
              >
                <p style={{ color: '#475569', fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.06em', margin: 0, textTransform: 'uppercase' }}>
                  {row.evidenceFreshness} · {row.supportState} · evidence: {row.evidenceTier}
                </p>
                <p style={{ color: '#0f172a', fontSize: '0.95rem', fontWeight: 700, margin: '0.3rem 0 0' }}>
                  {row.subjectId} <span style={{ color: '#64748b', fontWeight: 400 }}>({row.subjectType})</span>
                </p>
                <p style={{ color: '#7c2d12', lineHeight: 1.6, margin: '0.4rem 0 0' }}>{row.refreshAuditLabel}</p>
                <p style={{ color: '#475569', fontSize: '0.85rem', margin: '0.4rem 0 0' }}>
                  Grounding: <code style={{ color: '#334155' }}>{row.groundingRef}</code>
                </p>
                {row.blockerOrLossinessNote ? (
                  <p style={{ color: '#7c2d12', fontSize: '0.85rem', lineHeight: 1.6, margin: '0.3rem 0 0' }}>
                    <strong>Blocker / lossiness:</strong> {row.blockerOrLossinessNote}
                  </p>
                ) : null}
                <p style={{ margin: '0.35rem 0 0' }}>
                  <code style={{ color: '#94a3b8', fontSize: '0.75rem' }}>{row.rowId}</code>
                </p>
              </div>
            ))}
          </div>
        </>
      )}
    </section>
  );
}

export default function App() {
  const [surface, setSurface] = useState<Sd11TesterWorkbenchSurface | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadSd11TesterWorkbenchSurfaceRuntime({
      buildVersion: desktopPackage.version,
      platformLabel: derivePlatformLabel(),
    })
      .then(setSurface)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown SD-11 tester workbench failure');
      });
  }, []);

  return (
    <main style={{ fontFamily: 'Inter, system-ui, sans-serif', margin: '0 auto', maxWidth: 1100, padding: '3rem 1.5rem' }}>
      <header>
        <p style={{ color: '#64748b', fontSize: '0.875rem', letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
          {surface?.surfaceLabel ?? 'SD-11 tester workbench'}
        </p>
        <h1 style={{ marginBottom: '0.5rem' }}>{surface?.headline ?? 'Loading bounded tester workbench frame…'}</h1>
        <p style={{ color: '#334155', lineHeight: 1.6, marginBottom: 0 }}>
          {surface?.lead ??
            'Loading the first bounded tester-facing workbench frame over the current desktop runtime seam.'}
        </p>
      </header>

      {error ? (
        <section style={{ backgroundColor: '#fef2f2', border: '1px solid #fecaca', borderRadius: 12, marginTop: '1.5rem', padding: '1rem 1.25rem' }}>
          <h2 style={{ color: '#991b1b', marginTop: 0 }}>Workbench load failure</h2>
          <p style={{ color: '#7f1d1d', marginBottom: 0, whiteSpace: 'pre-wrap' }}>{error}</p>
        </section>
      ) : null}

      {surface ? (
        <>
          <section style={{ display: 'grid', gap: '0.9rem', gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))', marginTop: '2rem' }}>
            <AppCard label="Build" value={surface.status.build.label} detail={surface.status.update.label} />
            <AppCard label="Channel" value={surface.status.channel.testerFacingLabel} detail={surface.status.channel.audience} />
            <AppCard label="Platform" value={surface.platformLabel} detail={surface.status.support.currentPlatformSupportLabel} />
            <AppCard label="Workflow" value={surface.workflowName} detail={surface.workflowState} />
            <AppCard label="Data truth" value={surface.dataTruthLabel} detail="Real bounded snapshot when available, explicit fallback when it is not." />
          </section>

          {surface.fallbackNotice ? (
            <section style={{ backgroundColor: '#fff7ed', border: '1px solid #fdba74', borderRadius: 12, marginTop: '1.5rem', padding: '1rem 1.25rem' }}>
              <h2 style={{ color: '#9a3412', marginTop: 0 }}>Explicit fallback</h2>
              <p style={{ color: '#7c2d12', marginBottom: 0 }}>{surface.fallbackNotice}</p>
            </section>
          ) : null}

          <section style={{ border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
            <h2 style={{ marginTop: 0 }}>Current bounded workflow</h2>
            <p style={{ color: '#475569', lineHeight: 1.6 }}>{surface.boundedScopeNotice}</p>
            <div style={{ display: 'grid', gap: '0.75rem', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', marginTop: '1rem' }}>
              {surface.summaryRows.map((row) => (
                <AppCard key={row.label} label={row.label} value={row.value} />
              ))}
            </div>
          </section>

          <section style={{ border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
            <h2 style={{ marginTop: 0 }}>Diagnostics and explanation visibility</h2>
            <p style={{ color: '#475569', lineHeight: 1.6 }}>{surface.feedbackStatusNotice}</p>

            <div style={{ display: 'grid', gap: '0.75rem', marginTop: '1rem' }}>
              {surface.diagnostics.length ? (
                surface.diagnostics.map((diagnostic, index) => (
                  <div
                    key={`${diagnostic.classLabel}-${index}`}
                    style={{
                      backgroundColor: '#f8fafc',
                      border: '1px solid #cbd5e1',
                      borderRadius: 12,
                      padding: '0.9rem 1rem',
                    }}
                  >
                    <p style={{ color: toneColor(diagnostic.severity), fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.08em', margin: 0, textTransform: 'uppercase' }}>
                      {diagnostic.severityLabel} · {diagnostic.classLabel}
                    </p>
                    <p style={{ margin: '0.45rem 0 0' }}>{diagnostic.message}</p>
                    {diagnostic.subjectRef ? (
                      <p style={{ color: '#475569', fontSize: '0.875rem', margin: '0.45rem 0 0' }}>
                        Subject reference: <code>{diagnostic.subjectRef}</code>
                      </p>
                    ) : null}
                    {diagnostic.claimBlocking ? (
                      <p style={{ color: '#7c2d12', fontSize: '0.875rem', margin: '0.45rem 0 0' }}>
                        This diagnostic blocks at least one claim in the bounded snapshot.
                      </p>
                    ) : null}
                  </div>
                ))
              ) : (
                <p style={{ color: '#475569', margin: 0 }}>No diagnostics were returned for the current bounded snapshot.</p>
              )}
            </div>

            <div style={{ display: 'grid', gap: '1rem', gridTemplateColumns: 'repeat(auto-fit, minmax(260px, 1fr))', marginTop: '1rem' }}>
              <div>
                <h3 style={{ marginBottom: '0.5rem' }}>Blocked claims</h3>
                <p style={{ color: '#475569', fontSize: '0.875rem', lineHeight: 1.5, marginTop: 0 }}>
                  Runtime-blocked claims stay separate from broad unsupported-scope messaging so testers can tell a bounded workflow failure from an out-of-scope request.
                </p>
                {surface.blockedClaims.length ? (
                  <ul style={{ margin: 0, paddingLeft: '1.2rem' }}>
                    {surface.blockedClaims.map((claim) => (
                      <li key={claim} style={{ marginBottom: '0.45rem' }}>{claim}</li>
                    ))}
                  </ul>
                ) : (
                  <p style={{ color: '#475569', margin: 0 }}>No blocked claims surfaced in the current bounded snapshot.</p>
                )}
              </div>
              <div>
                <h3 style={{ marginBottom: '0.5rem' }}>Explanation references</h3>
                <EvidenceList
                  emptyMessage="No explanation references were returned for the current bounded snapshot."
                  items={surface.explanationRefs}
                />
              </div>
              <div>
                <h3 style={{ marginBottom: '0.5rem' }}>Provenance references</h3>
                <EvidenceList
                  emptyMessage="No provenance references were returned for the current bounded snapshot."
                  items={surface.provenanceRefs}
                />
              </div>
            </div>
          </section>

          <section style={{ border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
            <h2 style={{ marginTop: 0 }}>Update and support posture</h2>
            <div style={{ display: 'grid', gap: '0.9rem', gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))' }}>
              <AppCard label="Tester track" value={surface.status.channel.testerFacingLabel} detail={surface.status.channel.detail} />
              <AppCard label="Support matrix" value={surface.status.support.tierMatrixLabel} detail={surface.status.support.currentPlatformSupportLabel} />
              <AppCard label="Update status" value={surface.status.update.label} detail={`${surface.status.channel.testerFacingLabel} tester track on ${surface.status.support.currentPlatformSupportLabel}`} />
              <AppCard
                label="Issue payload status"
                value={surface.status.issueCapture.testerFacingChannelSupportLabel}
                detail="Structured status object reused for later evidence capture."
              />
            </div>
            <p style={{ color: '#475569', lineHeight: 1.6, marginBottom: '0.75rem', marginTop: '1rem' }}>
              {surface.status.support.platformSupportDetail}
            </p>
            <p style={{ color: '#475569', lineHeight: 1.6, marginBottom: '0.75rem' }}>{surface.status.update.detail}</p>
            <p style={{ color: '#64748b', lineHeight: 1.6, marginBottom: 0 }}>
              Operator provenance remains {surface.status.channel.operatorPromotionPath}.
            </p>
          </section>

          <SupportDebtPanel surface={surface} />

          <BreadthClaimAuditPanel surface={surface} />

          <UpdateActionPanel surface={surface} />

          <FeedbackEvidencePanel surface={surface} />

          <BugReportComposer surface={surface} />

          <EnhancementRequestComposer surface={surface} />

          <section style={{ border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '1.5rem', padding: '1.25rem' }}>
            <h2 style={{ marginTop: 0 }}>Bounded truth and next surface</h2>
            <p style={{ color: '#475569', lineHeight: 1.6 }}>{surface.boundedScopeNotice}</p>
            <ul style={{ marginBottom: 0, paddingLeft: '1.2rem' }}>
              {surface.notes.map((note) => (
                <li key={note} style={{ marginBottom: '0.45rem' }}>{note}</li>
              ))}
            </ul>
          </section>
        </>
      ) : null}
    </main>
  );
}

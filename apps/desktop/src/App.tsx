import { useEffect, useState } from 'react';
import desktopPackage from '../package.json';
import {
  loadSd11TesterWorkbenchSurfaceRuntime,
} from './sd11/loadSd11TesterWorkbenchSurfaceRuntime';
import type { Sd11TesterWorkbenchSurface } from './sd11/loadSd11TesterWorkbenchSurface';

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
                      {diagnostic.classLabel}
                    </p>
                    <p style={{ color: '#64748b', fontSize: '0.875rem', margin: '0.35rem 0 0' }}>
                      {diagnostic.severityLabel}
                      {diagnostic.subjectRef ? ` · ${diagnostic.subjectRef}` : ''}
                      {diagnostic.claimBlocking ? ' · claim-blocking' : ''}
                    </p>
                    <p style={{ margin: '0.45rem 0 0' }}>{diagnostic.message}</p>
                  </div>
                ))
              ) : (
                <p style={{ color: '#475569', margin: 0 }}>No diagnostics were returned for the current bounded snapshot.</p>
              )}
            </div>

            <div style={{ display: 'grid', gap: '1rem', gridTemplateColumns: 'repeat(auto-fit, minmax(260px, 1fr))', marginTop: '1rem' }}>
              <div>
                <h3 style={{ marginBottom: '0.5rem' }}>Blocked claims</h3>
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
                {surface.explanationRefs.length ? (
                  <ul style={{ margin: 0, paddingLeft: '1.2rem' }}>
                    {surface.explanationRefs.map((reference) => (
                      <li key={reference.label} style={{ marginBottom: '0.45rem' }}>
                        <strong>{reference.label}</strong>
                        <span style={{ color: '#475569' }}> — {reference.detail}</span>
                      </li>
                    ))}
                  </ul>
                ) : (
                  <p style={{ color: '#475569', margin: 0 }}>No explanation references were returned for the current bounded snapshot.</p>
                )}
              </div>
            </div>

            <div style={{ marginTop: '1rem' }}>
              <h3 style={{ marginBottom: '0.5rem' }}>Provenance references</h3>
              {surface.provenanceRefs.length ? (
                  {surface.provenanceRefs.map((reference, index) => (
                    <li key={`${reference.label}-${index}`} style={{ marginBottom: '0.45rem' }}>
                      <strong>{reference.label}</strong>
                      <span style={{ color: '#475569' }}> — {reference.detail}</span>
                    </li>
                </ul>
              ) : (
                <p style={{ color: '#475569', margin: 0 }}>No provenance references were returned for the current bounded snapshot.</p>
              )}
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

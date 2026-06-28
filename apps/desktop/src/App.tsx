import { useEffect, useState } from 'react';
import { loadGe08AuthoringWorkbench, type Ge08AuthoringWorkbenchSnapshot } from './boundary/loadGe08AuthoringWorkbench';

export default function App() {
  const [snapshot, setSnapshot] = useState<Ge08AuthoringWorkbenchSnapshot | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadGe08AuthoringWorkbench({
      packageRoot: 'tests/fixtures/ge08/guard-stance-package',
    })
      .then(setSnapshot)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown shell boundary failure');
      });
  }, []);

  return (
    <main style={{ fontFamily: 'Inter, system-ui, sans-serif', margin: '0 auto', maxWidth: 960, padding: '3rem 1.5rem' }}>
      <header>
        <p style={{ color: '#64748b', fontSize: '0.875rem', letterSpacing: '0.08em', textTransform: 'uppercase' }}>
          GE08-E5 desktop authoring workbench
        </p>
        <h1 style={{ marginBottom: '0.5rem' }}>Guard Stance Package Authoring Workbench</h1>
        <p style={{ color: '#334155', lineHeight: 1.6 }}>
          This workbench loads the first bounded GE-08 proof package from the headless authoring substrate through the
          Tauri command boundary, surfacing validation state, preview results, provenance, and explanation without
          counterfeit success.
        </p>
      </header>

      <section style={{ border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '2rem', padding: '1.25rem' }}>
        <h2 style={{ marginTop: 0 }}>GE08 Authoring Workbench Snapshot</h2>
        {error ? (
          <p style={{ color: '#b91c1c', whiteSpace: 'pre-wrap', fontFamily: 'monospace', fontSize: '0.85rem' }}>
            Error: {error}
          </p>
        ) : null}
        {snapshot ? (
          <div>
            <div style={{ marginBottom: '1rem', padding: '0.75rem', backgroundColor: '#f1f5f9', borderRadius: 8 }}>
              <p style={{ margin: 0, fontWeight: 'bold', marginBottom: '0.25rem' }}>Package: {snapshot.packageManifest.packageId}</p>
              <p style={{ margin: 0, fontSize: '0.875rem', color: '#475569' }}>
                State: <span style={{ fontWeight: 'bold' }}>{snapshot.packageState}</span> | Preview:{' '}
                <span style={{ fontWeight: 'bold' }}>{snapshot.preview.previewStatus}</span>
              </p>
            </div>
            <details style={{ marginBottom: '1rem' }}>
              <summary style={{ cursor: 'pointer', fontWeight: 'bold', marginBottom: '0.5rem' }}>
                Full Snapshot (click to expand)
              </summary>
              <pre
                style={{
                  backgroundColor: '#f8fafc',
                  padding: '1rem',
                  borderRadius: 8,
                  fontSize: '0.75rem',
                  overflow: 'auto',
                  maxHeight: '60vh',
                }}
              >
                {JSON.stringify(snapshot, null, 2)}
              </pre>
            </details>
          </div>
        ) : (
          <p>Loading GE08 authoring workbench…</p>
        )}
      </section>
    </main>
  );
}

import { useEffect, useState } from 'react';
import { loadPilotShellSnapshot, type PilotShellSnapshot } from './boundary/loadPilotShellSnapshot';

export default function App() {
  const [snapshot, setSnapshot] = useState<PilotShellSnapshot | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadPilotShellSnapshot()
      .then(setSnapshot)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown shell boundary failure');
      });
  }, []);

  return (
    <main style={{ fontFamily: 'Inter, system-ui, sans-serif', margin: '0 auto', maxWidth: 960, padding: '3rem 1.5rem' }}>
      <header>
        <p style={{ color: '#64748b', fontSize: '0.875rem', letterSpacing: '0.08em', textTransform: 'uppercase' }}>
          GE07-E1 additive scaffold
        </p>
        <h1 style={{ marginBottom: '0.5rem' }}>Codex desktop shell scaffold</h1>
        <p style={{ color: '#334155', lineHeight: 1.6 }}>
          This shell is a non-production boundary spike. It exists to prove that GE-07 can host a desktop subtree
          without claiming UI-complete product truth.
        </p>
      </header>

      <section style={{ border: '1px solid #cbd5e1', borderRadius: 12, marginTop: '2rem', padding: '1.25rem' }}>
        <h2 style={{ marginTop: 0 }}>Pilot shell boundary</h2>
        <p style={{ color: '#475569', lineHeight: 1.6 }}>
          The first shell contract will eventually read from a Tauri command boundary instead of recomputing rules data
          in the frontend.
        </p>
        {error ? <p style={{ color: '#b91c1c' }}>{error}</p> : null}
        {snapshot ? <pre>{JSON.stringify(snapshot, null, 2)}</pre> : <p>Loading scaffold placeholder…</p>}
      </section>
    </main>
  );
}

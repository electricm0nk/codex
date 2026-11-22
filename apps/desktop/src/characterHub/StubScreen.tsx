/**
 * Placeholder screen for features that have a landing entry point but no
 * build yet (e.g. Manage Party, DM Toolkit) — keeps the button honest about
 * what exists rather than doing nothing when clicked.
 */
export function StubScreen(props: { title: string; description: string; onBack: () => void }) {
  return (
    <section style={{ marginTop: '2rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <h2 style={{ margin: 0 }}>{props.title}</h2>
        <button
          type="button"
          onClick={props.onBack}
          style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, cursor: 'pointer', padding: '0.5rem 1rem' }}
        >
          Back
        </button>
      </div>
      <div style={{ border: '1px dashed var(--color-border)', borderRadius: 12, padding: '2rem', textAlign: 'center' }}>
        <p style={{ color: 'var(--color-text-secondary)', margin: 0 }}>{props.description}</p>
      </div>
    </section>
  );
}

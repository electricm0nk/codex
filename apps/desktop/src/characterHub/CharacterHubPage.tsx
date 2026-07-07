import { useEffect, useState } from 'react';
import { loadCharacterHubListSurfaceRuntime } from './characterHubRuntime';
import type { CharacterHubListSurface } from './buildCharacterHubListSurface';
import { CharacterListRow } from './CharacterListRow';
import { CreateCharacterForm } from './CreateCharacterForm';

export function CharacterHubPage() {
  const [mode, setMode] = useState<'list' | 'create'>('list');
  const [surface, setSurface] = useState<CharacterHubListSurface | null>(null);
  const [error, setError] = useState<string | null>(null);

  function reload() {
    loadCharacterHubListSurfaceRuntime()
      .then(setSurface)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown character hub failure');
      });
  }

  useEffect(() => {
    reload();
  }, []);

  return (
    <section style={{ marginTop: '2rem' }}>
      {mode === 'list' ? (
        <>
          <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
            <h2 style={{ margin: 0 }}>Your characters</h2>
            <button
              type="button"
              onClick={() => setMode('create')}
              style={{
                backgroundColor: '#0f172a',
                border: 'none',
                borderRadius: 8,
                color: 'white',
                cursor: 'pointer',
                padding: '0.55rem 1.1rem',
              }}
            >
              Create new character
            </button>
          </div>

          {error ? <p style={{ color: '#991b1b' }}>{error}</p> : null}

          {surface?.unreadableNotice ? (
            <p style={{ color: '#9a3412', fontSize: '0.875rem' }}>{surface.unreadableNotice}</p>
          ) : null}

          {surface?.isEmpty ? <p style={{ color: '#475569' }}>{surface.emptyStateMessage}</p> : null}

          <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
            {surface?.rows.map((row) => (
              <CharacterListRow key={row.characterId} row={row} />
            ))}
          </div>
        </>
      ) : (
        <>
          <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
            <h2 style={{ margin: 0 }}>Create a character</h2>
            <button
              type="button"
              onClick={() => setMode('list')}
              style={{ background: 'none', border: '1px solid #cbd5e1', borderRadius: 8, cursor: 'pointer', padding: '0.5rem 1rem' }}
            >
              Back to characters
            </button>
          </div>
          {/* Refresh the list data in the background so it's current whenever the
              user chooses to go back — but stay on the form so they can see the
              computed character sheet (or blocked diagnostics) the submit produced. */}
          <CreateCharacterForm onCreated={reload} />
        </>
      )}
    </section>
  );
}

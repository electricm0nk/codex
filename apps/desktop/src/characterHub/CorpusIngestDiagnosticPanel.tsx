import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { hasTauriRuntime, formatError } from '../boundary/runtime';

/**
 * One book's real ingested-corpus status, mirroring the Rust
 * `corpus_ingest_diagnostic.rs`'s `BookIngestStatus` DTO verbatim.
 *
 * Sketch-only boundary (Criterion 5.1, `cycles/5_1.md`): exactly these
 * four fields. SD-26 fans out the full status table + flags + ETA once
 * the JSON ingest cache lands.
 */
export interface BookIngestStatusDto {
  bookId: string;
  status: string;
  lastIngestedAt: string | null;
  contentKindCounts: Record<string, number>;
}

/**
 * Boundary wrapper over the `corpus_ingest_diagnostic` Tauri command — the
 * real ingested state of every populated `rules_tables` book, never
 * fixture/sample data. Mirrors `boundary/loadClassCatalog.ts`'s
 * runtime-detection + error-formatting shape.
 */
async function loadCorpusIngestDiagnostic(): Promise<BookIngestStatusDto[]> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading the corpus ingest diagnostic');
  }

  try {
    return await invoke<BookIngestStatusDto[]>('corpus_ingest_diagnostic');
  } catch (cause: unknown) {
    throw new Error(`Failed to load corpus ingest diagnostic: ${formatError(cause)}`);
  }
}

function formatCounts(counts: Record<string, number>): string {
  const kinds = Object.keys(counts).sort();
  return kinds.map((kind) => `${kind}: ${counts[kind]}`).join(' · ');
}

/**
 * Corpus ingest diagnostic panel — a developer/operator-facing sketch
 * surfacing each ingested rule book's real status (Epic 5, Criterion 5.1).
 * Full status table + flags + ETA is SD-26 scope; this panel shows only
 * `bookId` / `status` / `lastIngestedAt` / `contentKindCounts`.
 */
export function CorpusIngestDiagnosticPanel(props: { onClose: () => void }) {
  const [books, setBooks] = useState<BookIngestStatusDto[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadCorpusIngestDiagnostic()
      .then(setBooks)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown corpus ingest diagnostic failure');
      });
  }, []);

  return (
    <section style={{ marginTop: '1rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <h2 style={{ margin: 0 }}>Corpus Ingest Diagnostic</h2>
        <button
          type="button"
          onClick={props.onClose}
          style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, cursor: 'pointer', padding: '0.5rem 1rem' }}
        >
          Back
        </button>
      </div>

      {error ? (
        <p style={{ color: 'var(--color-danger, #d33)', margin: 0 }}>{error}</p>
      ) : !books ? (
        <p style={{ color: 'var(--color-text-faint)', margin: 0 }}>Loading corpus ingest diagnostic…</p>
      ) : (
        <>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0 0 1rem' }}>
            Real ingested state of every rule book landed in <code>rules_tables</code> — not a sample. Sketch
            scope: {books.length} book{books.length === 1 ? '' : 's'}.
          </p>

          <div
            style={{
              backgroundColor: 'var(--color-surface)',
              border: '1px solid var(--color-border)',
              borderRadius: 10,
            }}
          >
            {books.map((book) => (
              <div
                key={book.bookId}
                style={{
                  borderBottom: '1px solid var(--color-border)',
                  padding: '0.75rem 1rem',
                }}
              >
                <div style={{ alignItems: 'baseline', display: 'flex', gap: '0.75rem', justifyContent: 'space-between' }}>
                  <span style={{ fontWeight: 700, textTransform: 'uppercase' }}>{book.bookId}</span>
                  <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem' }}>{book.status}</span>
                </div>
                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.25rem 0 0' }}>
                  {formatCounts(book.contentKindCounts)}
                </p>
                <p style={{ color: 'var(--color-text-faint)', fontSize: '0.7rem', margin: '0.25rem 0 0' }}>
                  Last ingested: {book.lastIngestedAt ?? 'unknown'}
                </p>
              </div>
            ))}
          </div>
        </>
      )}
    </section>
  );
}

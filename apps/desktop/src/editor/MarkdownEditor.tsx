// The campaign-folder Markdown editor (decisions.md §18). Every file access goes through props,
// so the component imports no boundary and the campaign tabs supply the folder commands.
import { useEffect, useRef, useState } from 'react';
import { EditorState } from '@codemirror/state';
import { EditorView, keymap } from '@codemirror/view';
import { defaultKeymap, history, historyKeymap } from '@codemirror/commands';
import { searchKeymap } from '@codemirror/search';
import { markdown, markdownLanguage } from '@codemirror/lang-markdown';
import { decode, encode, type DecodedFile } from './roundTrip';
import { livePreview, type ResolveImage } from './livePreview';
import { renderReading } from './readingView';

export interface MarkdownEditorProps {
  campaignDir: string;
  /** Path inside the campaign folder, e.g. `Party Resources.md`. */
  path: string;
  readFile: (campaignDir: string, path: string) => Promise<Uint8Array>;
  writeFile: (campaignDir: string, path: string, bytes: Uint8Array) => Promise<void>;
  statFile: (campaignDir: string, path: string) => Promise<{ mtimeMs: number }>;
  resolveImage: ResolveImage;
  pages: () => Promise<string[]>;
}

/** SD-s: a file this large opens read-only. */
export const MAX_EDITABLE_BYTES = 2 * 1024 * 1024;

type Status = { kind: 'loading' } | { kind: 'ready'; readOnlyReason?: string; message?: string } | { kind: 'error'; message: string };

export function MarkdownEditor(props: MarkdownEditorProps) {
  const { campaignDir, path } = props;
  // IO callbacks are read through a ref so a parent's inline closures never re-open the file mid-edit.
  const io = useRef(props);
  io.current = props;
  const host = useRef<HTMLDivElement>(null);
  const view = useRef<EditorView | null>(null);
  const meta = useRef<DecodedFile | null>(null);
  const [status, setStatus] = useState<Status>({ kind: 'loading' });
  const [reading, setReading] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    setStatus({ kind: 'loading' });
    setReading(null);

    const save = (target: EditorView) => {
      const loaded = meta.current;
      if (!loaded || target.state.readOnly) return false;
      const text = target.state.doc.toString();
      const bytes = encode(text, loaded);
      io.current
        .writeFile(campaignDir, path, bytes)
        .then(() => {
          meta.current = { ...loaded, sourceText: text, original: bytes };
          setStatus({ kind: 'ready', message: 'Saved' });
        })
        .catch((err: unknown) => setStatus({ kind: 'ready', message: `Save failed: ${String(err)}` }));
      return true;
    };

    io.current
      .readFile(campaignDir, path)
      .then((bytes) => {
        if (cancelled || !host.current) return;
        let decoded: DecodedFile;
        let readOnlyReason: string | undefined;
        try {
          decoded = decode(bytes);
        } catch {
          setStatus({ kind: 'error', message: 'This file is not UTF-8 text. Edit it in another editor.' });
          return;
        }
        if (bytes.length > MAX_EDITABLE_BYTES) readOnlyReason = 'This file is larger than 2 MB. Edit it in another editor.';
        meta.current = decoded;
        view.current = new EditorView({
          parent: host.current,
          state: EditorState.create({
            doc: decoded.text,
            extensions: [
              history(),
              keymap.of([{ key: 'Mod-s', run: save, preventDefault: true }, ...defaultKeymap, ...historyKeymap, ...searchKeymap]),
              markdown({ base: markdownLanguage }),
              livePreview((image) => io.current.resolveImage(image)),
              EditorView.lineWrapping,
              EditorState.readOnly.of(readOnlyReason !== undefined),
              EditorView.editable.of(readOnlyReason === undefined),
            ],
          }),
        });
        setStatus({ kind: 'ready', readOnlyReason });
      })
      .catch((err: unknown) => {
        if (!cancelled) setStatus({ kind: 'error', message: `Could not open ${path}: ${String(err)}` });
      });

    return () => {
      cancelled = true;
      view.current?.destroy();
      view.current = null;
      meta.current = null;
    };
  }, [campaignDir, path]);

  const toggleReading = () => {
    setReading((current) => (current === null && view.current ? renderReading(view.current.state.doc.toString()) : null));
  };

  return (
    <div className="markdown-editor">
      <div className="markdown-editor__bar">
        <button type="button" onClick={toggleReading} disabled={status.kind !== 'ready'} aria-pressed={reading !== null}>
          {reading === null ? 'Reading' : 'Edit'}
        </button>
        {status.kind === 'loading' && <span>Loading {path}…</span>}
        {status.kind === 'error' && <span role="alert">{status.message}</span>}
        {status.kind === 'ready' && status.readOnlyReason && <span role="status">{status.readOnlyReason}</span>}
        {status.kind === 'ready' && status.message && <span role="status">{status.message}</span>}
      </div>
      <div ref={host} className="markdown-editor__source" hidden={reading !== null} />
      {reading !== null && (
        // renderReading escapes every raw-HTML token and drops script URLs (readingView.ts).
        <div className="markdown-editor__reading" dangerouslySetInnerHTML={{ __html: reading }} />
      )}
    </div>
  );
}

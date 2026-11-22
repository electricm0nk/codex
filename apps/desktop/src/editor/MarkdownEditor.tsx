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
import { wikilinks, type OpenLink } from './wikilinks';
import { Toolbar } from './Toolbar';
import { createAutosave, type Autosave, type AutosaveState } from './autosave';
import { imageDrop } from './imageDrop';

export interface MarkdownEditorProps {
  campaignDir: string;
  /** Path inside the campaign folder, e.g. `Party Resources.md`. */
  path: string;
  // The shapes of `boundary/campaignFolder/campaignFiles.ts`, so a tab passes those functions as they are.
  readFile: (campaignDir: string, path: string) => Promise<{ bytes: Uint8Array; modifiedMs: number }>;
  /** Resolves to the file's new mtime. */
  writeFile: (campaignDir: string, path: string, bytes: Uint8Array) => Promise<{ modifiedMs: number }>;
  statFile: (campaignDir: string, path: string) => Promise<{ modifiedMs: number }>;
  /** Copies a dropped image into `Attachments/` (`boundary/campaignFolder/attachments.ts`); resolves to its path. */
  saveAttachment: (campaignDir: string, name: string, bytes: Uint8Array) => Promise<string>;
  resolveImage: ResolveImage;
  /** Every wiki page title and filename stem, for `[[` completion. */
  pages: () => Promise<string[]>;
  /** A `[[link]]` was clicked (or Ctrl-Enter'd): its page name, alias and heading dropped. */
  onOpenLink: OpenLink;
}

/** SD-s: a file this large opens read-only. */
export const MAX_EDITABLE_BYTES = 2 * 1024 * 1024;

type Status = { kind: 'loading' } | { kind: 'ready'; readOnlyReason?: string } | { kind: 'error'; message: string };

export function MarkdownEditor(props: MarkdownEditorProps) {
  const { campaignDir, path } = props;
  // IO callbacks are read through a ref so a parent's inline closures never re-open the file mid-edit.
  const io = useRef(props);
  io.current = props;
  const host = useRef<HTMLDivElement>(null);
  const view = useRef<EditorView | null>(null);
  const [status, setStatus] = useState<Status>({ kind: 'loading' });
  const [reading, setReading] = useState<string | null>(null);
  const [save, setSave] = useState<AutosaveState>({ kind: 'idle' });
  const saver = useRef<Autosave | null>(null);

  useEffect(() => {
    let cancelled = false;
    setStatus({ kind: 'loading' });
    setReading(null);

    io.current
      .readFile(campaignDir, path)
      .then(({ bytes, modifiedMs }) => {
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
        // Local to this open file, so a write finishing after the file closes still encodes it right.
        let loadedFile: DecodedFile = decoded;
        const editable = readOnlyReason === undefined;
        const flush = () => {
          void saver.current?.flush();
          return true;
        };
        const editor = (view.current = new EditorView({
          parent: host.current,
          state: EditorState.create({
            doc: decoded.text,
            extensions: [
              history(),
              keymap.of([{ key: 'Mod-s', run: flush, preventDefault: true }, ...defaultKeymap, ...historyKeymap, ...searchKeymap]),
              markdown({ base: markdownLanguage }),
              livePreview((image) => io.current.resolveImage(image)),
              wikilinks(
                () => io.current.pages(),
                (target) => io.current.onOpenLink(target),
              ),
              imageDrop(
                (name, data) => io.current.saveAttachment(campaignDir, name, data),
                (message) => setSave({ kind: 'error', message }),
              ),
              EditorView.updateListener.of((update) => {
                if (update.docChanged) saver.current?.changed();
              }),
              EditorView.domEventHandlers({ blur: () => void flush() }),
              EditorView.lineWrapping,
              EditorState.readOnly.of(!editable),
              EditorView.editable.of(editable),
            ],
          }),
        }));
        if (editable) {
          saver.current = createAutosave(
            {
              getText: () => editor.state.doc.toString(),
              write: async (text) => {
                const out = encode(text, loadedFile);
                const written = await io.current.writeFile(campaignDir, path, out);
                loadedFile = { ...loadedFile, sourceText: text, original: out };
                return written.modifiedMs;
              },
              stat: async () => (await io.current.statFile(campaignDir, path)).modifiedMs,
              read: async () => {
                const file = await io.current.readFile(campaignDir, path);
                const next = decode(file.bytes);
                loadedFile = next;
                return { text: next.text, modifiedMs: file.modifiedMs };
              },
              replaceDoc: (text) => editor.dispatch({ changes: { from: 0, to: editor.state.doc.length, insert: text } }),
              onState: (state) => {
                if (!cancelled) setSave(state);
              },
            },
            modifiedMs,
          );
        }
        setStatus({ kind: 'ready', readOnlyReason });
      })
      .catch((err: unknown) => {
        if (!cancelled) setStatus({ kind: 'error', message: `Could not open ${path}: ${String(err)}` });
      });

    return () => {
      // An edit inside the last second is written before the file closes (unless the banner is up).
      void saver.current?.flush();
      saver.current?.dispose();
      saver.current = null;
      cancelled = true;
      setSave({ kind: 'idle' });
      view.current?.destroy();
      view.current = null;
    };
  }, [campaignDir, path]);

  const toggleReading = () => {
    setReading((current) => (current === null && view.current ? renderReading(view.current.state.doc.toString()) : null));
  };

  return (
    <div className="markdown-editor">
      <div className="markdown-editor__bar">
        <Toolbar getView={() => view.current} disabled={status.kind !== 'ready' || !!status.readOnlyReason || reading !== null} />
        <button type="button" onClick={toggleReading} disabled={status.kind !== 'ready'} aria-pressed={reading !== null}>
          {reading === null ? 'Reading' : 'Edit'}
        </button>
        {status.kind === 'loading' && <span>Loading {path}…</span>}
        {status.kind === 'error' && <span role="alert">{status.message}</span>}
        {status.kind === 'ready' && status.readOnlyReason && <span role="status">{status.readOnlyReason}</span>}
        {save.kind === 'saved' && <span role="status">Saved</span>}
        {save.kind === 'error' && <span role="alert">{save.message}</span>}
      </div>
      {save.kind === 'conflict' && (
        <div className="markdown-editor__banner" role="alert">
          Changed on disk. Your edits are not saved while this shows.
          <button type="button" onClick={() => void saver.current?.reload()}>
            Reload
          </button>
          <button type="button" onClick={() => void saver.current?.keepMine()}>
            Keep mine
          </button>
        </div>
      )}
      <div ref={host} className="markdown-editor__source" hidden={reading !== null} />
      {reading !== null && (
        // renderReading escapes every raw-HTML token and drops script URLs (readingView.ts).
        <div className="markdown-editor__reading" dangerouslySetInnerHTML={{ __html: reading }} />
      )}
    </div>
  );
}

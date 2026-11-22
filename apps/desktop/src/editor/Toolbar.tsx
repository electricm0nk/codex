// The editor toolbar (decisions.md §18, Q25): the usual Markdown edits on the selection.
import { EditorSelection, type EditorState, type TransactionSpec } from '@codemirror/state';
import type { EditorView } from '@codemirror/view';

export type ToolbarAction = 'bold' | 'italic' | 'heading' | 'list' | 'link' | 'table';

/** Wraps each selection in `marker`, or unwraps it when the marker already surrounds it. */
function wrap(marker: string) {
  return (state: EditorState): TransactionSpec =>
    state.changeByRange((range) => {
      const n = marker.length;
      const before = state.sliceDoc(range.from - n, range.from);
      const after = state.sliceDoc(range.to, range.to + n);
      // A `*` that is half of bold's `**` is not italic's marker.
      const boldEdge = marker === '*' && state.sliceDoc(range.from - 2, range.from - 1) === '*';
      if (before === marker && after === marker && !boldEdge) {
        return {
          changes: [{ from: range.from - n, to: range.from }, { from: range.to, to: range.to + n }],
          range: EditorSelection.range(range.from - n, range.to - n),
        };
      }
      return {
        changes: [{ from: range.from, insert: marker }, { from: range.to, insert: marker }],
        range: EditorSelection.range(range.from + n, range.to + n),
      };
    });
}

/** The lines any selection touches, once each. */
function selectedLines(state: EditorState) {
  const seen = new Map<number, { from: number; text: string }>();
  for (const range of state.selection.ranges) {
    for (let pos = range.from; ; ) {
      const line = state.doc.lineAt(pos);
      seen.set(line.number, { from: line.from, text: line.text });
      if (line.to >= range.to || line.number === state.doc.lines) break;
      pos = line.to + 1;
    }
  }
  return [...seen.values()];
}

function cycleHeading(state: EditorState): TransactionSpec {
  return {
    changes: selectedLines(state).map(({ from, text }) => {
      const level = /^(#{1,6}) /.exec(text)?.[1].length ?? 0;
      const next = level >= 3 ? '' : `${'#'.repeat(level + 1)} `;
      return { from, to: from + (level ? level + 1 : 0), insert: next };
    }),
  };
}

function toggleList(state: EditorState): TransactionSpec {
  const lines = selectedLines(state);
  const all = lines.every(({ text }) => text.startsWith('- '));
  return {
    changes: lines
      .filter(({ text }) => all || !text.startsWith('- '))
      .map(({ from }) => (all ? { from, to: from + 2 } : { from, insert: '- ' })),
  };
}

function link(state: EditorState): TransactionSpec {
  return state.changeByRange((range) => {
    const text = range.empty ? 'text' : state.sliceDoc(range.from, range.to);
    const insert = `[${text}](url)`;
    const urlFrom = range.from + text.length + 3;
    return { changes: { from: range.from, to: range.to, insert }, range: EditorSelection.range(urlFrom, urlFrom + 3) };
  });
}

const TABLE = '| Column 1 | Column 2 |\n| --- | --- |\n|  |  |\n|  |  |';

/** A 2x2 table under a header row, after the cursor's line and set off by blank lines. */
function table(state: EditorState): TransactionSpec {
  const line = state.doc.lineAt(state.selection.main.head);
  const blank = line.text.trim() === '';
  const before = blank ? '' : '\n\n';
  // The line break that follows the line stays; one more makes the blank line after the table.
  const after = line.number < state.doc.lines ? '\n' : '';
  const at = blank ? line.from : line.to;
  const cell = at + before.length + TABLE.indexOf('|  |') + 2;
  return { changes: { from: at, to: line.to, insert: `${before}${TABLE}${after}` }, selection: EditorSelection.cursor(cell) };
}

export const toolbarEdits: Record<ToolbarAction, (state: EditorState) => TransactionSpec> = {
  bold: wrap('**'),
  italic: wrap('*'),
  heading: cycleHeading,
  list: toggleList,
  link,
  table,
};

const BUTTONS: Array<[ToolbarAction, string, string]> = [
  ['bold', 'B', 'Bold'],
  ['italic', 'I', 'Italic'],
  ['heading', 'H', 'Heading (cycles 1–3)'],
  ['list', '•', 'List'],
  ['link', 'Link', 'Link'],
  ['table', 'Table', 'Table'],
];

export function Toolbar({ getView, disabled }: { getView: () => EditorView | null; disabled: boolean }) {
  const run = (action: ToolbarAction) => {
    const view = getView();
    if (!view || view.state.readOnly) return;
    view.dispatch(toolbarEdits[action](view.state));
    view.focus();
  };
  return (
    <span className="markdown-editor__toolbar" role="toolbar" aria-label="Formatting">
      {BUTTONS.map(([action, label, title]) => (
        <button
          key={action}
          type="button"
          title={title}
          aria-label={title}
          disabled={disabled}
          // Keeps the editor's selection: a toolbar press must not blur the text first.
          onMouseDown={(event) => event.preventDefault()}
          onClick={() => run(action)}
        >
          {label}
        </button>
      ))}
    </span>
  );
}

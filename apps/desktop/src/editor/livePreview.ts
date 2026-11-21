// Obsidian-style live preview (decisions.md §18): decorations over the Lezer Markdown tree.
// Markup is hidden or drawn as a widget everywhere except on the lines a cursor or selection touches.
import { type EditorState, type Extension, type Range } from '@codemirror/state';
import { Decoration, type DecorationSet, EditorView, ViewPlugin, type ViewUpdate, WidgetType } from '@codemirror/view';
import { ensureSyntaxTree, syntaxTree } from '@codemirror/language';

export type ResolveImage = (path: string) => Promise<string>;

class BulletWidget extends WidgetType {
  readonly kind = 'bullet';
  eq() {
    return true;
  }
  toDOM() {
    const span = document.createElement('span');
    span.className = 'cm-lp-bullet';
    span.textContent = '•';
    return span;
  }
}

class TaskWidget extends WidgetType {
  readonly kind = 'task';
  constructor(readonly checked: boolean, readonly markerFrom: number) {
    super();
  }
  eq(other: TaskWidget) {
    return other.checked === this.checked && other.markerFrom === this.markerFrom;
  }
  toDOM(view: EditorView) {
    const box = document.createElement('input');
    box.type = 'checkbox';
    box.className = 'cm-lp-task';
    box.checked = this.checked;
    box.addEventListener('mousedown', (event) => {
      event.preventDefault();
      // Toggle the character between the brackets: "[ ]" <-> "[x]".
      view.dispatch({ changes: { from: this.markerFrom + 1, to: this.markerFrom + 2, insert: this.checked ? ' ' : 'x' } });
    });
    return box;
  }
}

class ImageWidget extends WidgetType {
  readonly kind = 'image';
  constructor(readonly path: string, readonly resolveImage: ResolveImage) {
    super();
  }
  eq(other: ImageWidget) {
    return other.path === this.path;
  }
  toDOM() {
    const img = document.createElement('img');
    img.className = 'cm-lp-image';
    img.alt = this.path;
    this.resolveImage(this.path)
      .then((url) => {
        img.src = url;
      })
      .catch(() => {
        img.alt = `Image not found: ${this.path}`;
      });
    return img;
  }
}

const hide = Decoration.replace({});
const lineClass = (cls: string) => Decoration.line({ class: cls });
const markClass = (cls: string) => Decoration.mark({ class: cls });
const WIKILINK = /\[\[([^[\]\n]+?)\]\]/g;

export function buildDecorations(state: EditorState, resolveImage: ResolveImage): DecorationSet {
  const tree = ensureSyntaxTree(state, state.doc.length, 50) ?? syntaxTree(state);
  const doc = state.doc;
  const cursorLines = new Set<number>();
  for (const range of state.selection.ranges) {
    for (let n = doc.lineAt(range.from).number; n <= doc.lineAt(range.to).number; n += 1) cursorLines.add(n);
  }
  const raw = (pos: number) => cursorLines.has(doc.lineAt(pos).number);
  const out: Range<Decoration>[] = [];
  const codeRanges: Array<[number, number]> = [];
  const eachLine = (from: number, to: number, cls: string) => {
    for (let n = doc.lineAt(from).number; n <= doc.lineAt(to).number; n += 1) out.push(lineClass(cls).range(doc.line(n).from));
  };

  tree.iterate({
    enter: (node) => {
      const { name, from, to } = node;
      const heading = /^ATXHeading(\d)$/.exec(name);
      if (heading) {
        out.push(lineClass(`cm-lp-h${heading[1]}`).range(doc.lineAt(from).from));
      } else if (name === 'HeaderMark' && node.node.parent?.name.startsWith('ATXHeading')) {
        const end = doc.sliceString(to, to + 1) === ' ' ? to + 1 : to;
        if (!raw(from)) out.push(hide.range(from, end));
      } else if (name === 'Emphasis') {
        out.push(markClass('cm-lp-em').range(from, to));
      } else if (name === 'StrongEmphasis') {
        out.push(markClass('cm-lp-strong').range(from, to));
      } else if (name === 'EmphasisMark') {
        if (!raw(from)) out.push(hide.range(from, to));
      } else if (name === 'InlineCode') {
        codeRanges.push([from, to]);
        out.push(markClass('cm-lp-code').range(from, to));
      } else if (name === 'CodeMark' && node.node.parent?.name === 'InlineCode') {
        if (!raw(from)) out.push(hide.range(from, to));
      } else if (name === 'ListMark' && node.node.parent?.parent?.name === 'BulletList') {
        if (!raw(from)) out.push(Decoration.replace({ widget: new BulletWidget() }).range(from, to));
      } else if (name === 'TaskMarker') {
        const checked = /x/i.test(doc.sliceString(from + 1, from + 2));
        if (!raw(from)) out.push(Decoration.replace({ widget: new TaskWidget(checked, from) }).range(from, to));
      } else if (name === 'Image') {
        const url = node.node.getChild('URL');
        const path = url ? doc.sliceString(url.from, url.to) : '';
        if (path.startsWith('Attachments/') && !raw(from) && doc.lineAt(from).number === doc.lineAt(to).number) {
          out.push(Decoration.replace({ widget: new ImageWidget(path, resolveImage) }).range(from, to));
        }
      } else if (name === 'FencedCode' || name === 'CodeBlock') {
        codeRanges.push([from, to]);
        eachLine(from, to, 'cm-lp-fence');
        return false;
      } else if (name === 'Table') {
        eachLine(from, to, 'cm-lp-table');
        return false;
      }
      return undefined;
    },
  });

  // Lezer's Markdown has no [[wikilink]] node; find them in the text, outside code.
  const text = doc.toString();
  for (const match of text.matchAll(WIKILINK)) {
    const from = match.index ?? 0;
    const to = from + match[0].length;
    if (codeRanges.some(([a, b]) => from < b && to > a)) continue;
    const target = match[1].split('|')[0].trim();
    out.push(Decoration.mark({ class: 'cm-lp-wikilink', attributes: { 'data-wikilink': target } }).range(from, to));
  }

  return Decoration.set(out, true);
}

const theme = EditorView.baseTheme({
  '.cm-lp-h1': { fontSize: '1.6em', fontWeight: '700' },
  '.cm-lp-h2': { fontSize: '1.4em', fontWeight: '700' },
  '.cm-lp-h3': { fontSize: '1.2em', fontWeight: '700' },
  '.cm-lp-h4, .cm-lp-h5, .cm-lp-h6': { fontWeight: '700' },
  '.cm-lp-em': { fontStyle: 'italic' },
  '.cm-lp-strong': { fontWeight: '700' },
  '.cm-lp-code': { fontFamily: 'monospace', backgroundColor: 'rgba(127,127,127,0.15)', borderRadius: '3px' },
  '.cm-lp-fence': { fontFamily: 'monospace', backgroundColor: 'rgba(127,127,127,0.10)' },
  '.cm-lp-table': { fontFamily: 'monospace', whiteSpace: 'pre' },
  '.cm-lp-wikilink': { color: 'var(--wasp-color-link)', textDecoration: 'underline', cursor: 'pointer' },
  '.cm-lp-bullet': { padding: '0 0.25em' },
  '.cm-lp-image': { maxWidth: '100%', display: 'block' },
});

export function livePreview(resolveImage: ResolveImage): Extension {
  const plugin = ViewPlugin.fromClass(
    class {
      decorations: DecorationSet;
      constructor(view: EditorView) {
        this.decorations = buildDecorations(view.state, resolveImage);
      }
      update(update: ViewUpdate) {
        if (update.docChanged || update.selectionSet || syntaxTree(update.startState) !== syntaxTree(update.state)) {
          this.decorations = buildDecorations(update.state, resolveImage);
        }
      }
    },
    { decorations: (value) => value.decorations },
  );
  return [plugin, theme];
}

// [[wikilink]] completion and link opening (decisions.md §18, §15.4). The page list and the
// resolution of a target to a file belong to the caller: this module only reads the text.
import { Prec, type EditorState, type Extension } from '@codemirror/state';
import { EditorView, keymap, type Command } from '@codemirror/view';
import { autocompletion, type CompletionContext, type CompletionResult, type CompletionSource } from '@codemirror/autocomplete';
import { ensureSyntaxTree, syntaxTree } from '@codemirror/language';

export type OpenLink = (target: string) => void;

const CODE_NODES = new Set(['InlineCode', 'FencedCode', 'CodeBlock']);

function inCode(state: EditorState, pos: number): boolean {
  const tree = ensureSyntaxTree(state, pos, 50) ?? syntaxTree(state);
  for (let node: ReturnType<typeof tree.resolveInner> | null = tree.resolveInner(pos, -1); node; node = node.parent) {
    if (CODE_NODES.has(node.name)) return true;
  }
  return false;
}

/** Opens on `[[`, lists every distinct title and stem from `pages()`, inserts `[[Title]]`. */
export function wikilinkCompletion(pages: () => Promise<string[]>): CompletionSource {
  return async (context: CompletionContext): Promise<CompletionResult | null> => {
    const before = context.matchBefore(/\[\[[^[\]|#\n]*$/);
    if (!before || inCode(context.state, context.pos)) return null;
    const from = before.from + 2;
    // An existing `]]` right after the cursor is replaced, so the link is never closed twice.
    const closed = context.state.sliceDoc(context.pos, context.pos + 2) === ']]';
    const names = [...new Set(await pages())];
    return {
      from,
      to: closed ? context.pos + 2 : context.pos,
      options: names.map((name) => ({ label: name, type: 'text', apply: `${name}]]` })),
      validFor: /^[^[\]|#\n]*$/,
    };
  };
}

/** The page a `[[link]]` at `pos` names, with `|alias` and `#heading` dropped; null off a link or in code. */
export function linkTargetAt(state: EditorState, pos: number): string | null {
  const line = state.doc.lineAt(pos);
  for (const match of line.text.matchAll(/\[\[([^[\]\n]+?)\]\]/g)) {
    const from = line.from + (match.index ?? 0);
    const to = from + match[0].length;
    if (pos < from || pos > to) continue;
    if (inCode(state, from + 2)) return null;
    const page = match[1].split('|')[0].split('#')[0].trim();
    return page === '' ? null : page;
  }
  return null;
}

/** Ctrl/Cmd-Enter: opens the link under the cursor, else falls through to the next binding. */
export function openLinkAtCursor(onOpenLink: OpenLink): Command {
  return (view) => {
    const target = linkTargetAt(view.state, view.state.selection.main.head);
    if (target === null) return false;
    onOpenLink(target);
    return true;
  };
}

/** Completion, Ctrl/Cmd-Enter and clicks. A plain click opens a link off the cursor line (where it is
 *  previewed); on the cursor line, where its markup is being edited, Ctrl/Cmd-click opens it. */
export function wikilinks(pages: () => Promise<string[]>, onOpenLink: OpenLink): Extension {
  return [
    autocompletion({ override: [wikilinkCompletion(pages)] }),
    Prec.high(keymap.of([{ key: 'Mod-Enter', run: openLinkAtCursor(onOpenLink) }])),
    EditorView.domEventHandlers({
      mousedown(event, view) {
        if (event.button !== 0) return false;
        const pos = view.posAtCoords({ x: event.clientX, y: event.clientY });
        if (pos === null) return false;
        const target = linkTargetAt(view.state, pos);
        if (target === null) return false;
        const line = view.state.doc.lineAt(pos).number;
        const editing = view.state.selection.ranges.some(
          (r) => view.state.doc.lineAt(r.from).number <= line && line <= view.state.doc.lineAt(r.to).number,
        );
        if (editing && !(event.ctrlKey || event.metaKey)) return false;
        event.preventDefault();
        onOpenLink(target);
        return true;
      },
    }),
  ];
}

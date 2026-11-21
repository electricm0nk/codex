import { EditorSelection, EditorState } from '@codemirror/state';
import { CompletionContext, insertCompletionText, type CompletionResult } from '@codemirror/autocomplete';
import { markdown, markdownLanguage } from '@codemirror/lang-markdown';
import type { EditorView } from '@codemirror/view';
import { linkTargetAt, openLinkAtCursor, wikilinkCompletion } from './wikilinks';
import { assert, assertEqual } from '../testSupport/asserts';

const state = (doc: string, cursor = doc.length) =>
  EditorState.create({ doc, selection: EditorSelection.cursor(cursor), extensions: [markdown({ base: markdownLanguage })] });

async function complete(doc: string, cursor = doc.length, pages: string[] = ['Old Tam', 'old-tam', 'Riverside Ferry']) {
  const st = state(doc, cursor);
  const result = await wikilinkCompletion(async () => pages)(new CompletionContext(st, cursor, false));
  return { st, result: result as CompletionResult | null };
}

// Applies an option the way the autocomplete plugin does for a string `apply`.
function apply(st: EditorState, result: CompletionResult, label: string) {
  const option = result.options.find((o) => o.label === label);
  assert(option !== undefined, `option ${label} listed`);
  const text = typeof option!.apply === 'string' ? option!.apply : option!.label;
  return st.update(insertCompletionText(st, text, result.from, result.to ?? st.selection.main.head)).state;
}

const cases: Array<[string, () => Promise<void>]> = [
  ['opens on double bracket', async () => {
    const open = await complete('See [[');
    assert(open.result !== null, 'a result after [[');
    assertEqual(open.result!.from, 6, 'the query starts after [[');
    assertEqual((await complete('See [')).result, null, 'no result after a single [');
    assertEqual((await complete('See [[Old Tam]] and more')).result, null, 'no result after a closed link');
    assertEqual((await complete('`[[`', 3)).result, null, 'no result inside inline code');
  }],
  ['lists titles and stems', async () => {
    const { result } = await complete('[[o', 3, ['Old Tam', 'old-tam', 'Old Tam', 'Riverside Ferry']);
    const labels = result!.options.map((o) => o.label);
    assertEqual(labels.join(','), 'Old Tam,old-tam,Riverside Ferry', 'every distinct title and stem, once');
  }],
  ['inserts closed link', async () => {
    const typed = await complete('Met [[Ol');
    assertEqual(apply(typed.st, typed.result!, 'Old Tam').doc.toString(), 'Met [[Old Tam]]', 'closes ]]');
    const closed = await complete('Met [[Ol]] today', 8);
    const after = apply(closed.st, closed.result!, 'Old Tam');
    assertEqual(after.doc.toString(), 'Met [[Old Tam]] today', 'an existing ]] is not doubled');
    assertEqual(after.selection.main.head, 15, 'the cursor lands after ]]');
  }],
  ['click resolves the target text incl. alias and heading', async () => {
    const doc = 'A [[Old Tam]], [[Old Tam|him]], [[Ferry#History]], [[Ferry#History|the ferry]] `[[Code]]`';
    const st = state(doc, 0);
    const at = (needle: string) => linkTargetAt(st, doc.indexOf(needle) + 2);
    assertEqual(at('[[Old Tam]]'), 'Old Tam', 'plain link');
    assertEqual(at('[[Old Tam|him]]'), 'Old Tam', 'alias stripped');
    assertEqual(at('[[Ferry#History]]'), 'Ferry', 'heading stripped');
    assertEqual(at('[[Ferry#History|'), 'Ferry', 'heading and alias stripped');
    assertEqual(at('[[Code]]'), null, 'not inside code');
    assertEqual(linkTargetAt(st, 0), null, 'plain text is no link');

    const opened: string[] = [];
    const cursorOnLink = state(doc, doc.indexOf('[[Old Tam|him]]') + 5);
    const ran = openLinkAtCursor((t) => opened.push(t))({ state: cursorOnLink } as EditorView);
    assert(ran && opened.join() === 'Old Tam', `Ctrl-Enter opens the link under the cursor: ${opened.join()}`);
    assert(!openLinkAtCursor((t) => opened.push(t))({ state: state(doc, 0) } as EditorView), 'Ctrl-Enter off a link falls through');
  }],
];

async function main() {
  let passed = 0;
  for (const [name, run] of cases) {
    await run();
    passed += 1;
    console.log(`  ok ${name}`);
  }
  console.log(`wikilinks: ${passed} cases passed`);
}

main().catch((err: unknown) => {
  console.error(err);
  process.exit(1);
});

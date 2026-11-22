import { EditorSelection, EditorState } from '@codemirror/state';
import { toolbarEdits, type ToolbarAction } from './Toolbar';
import { assertEqual } from '../testSupport/asserts';

// Applies one toolbar button to `doc` with the selection [from, to]; returns the text and the new selection.
function press(action: ToolbarAction, doc: string, from: number, to = from) {
  const state = EditorState.create({ doc, selection: EditorSelection.range(from, to), extensions: [EditorState.allowMultipleSelections.of(true)] });
  const next = state.update(toolbarEdits[action](state)).state;
  const sel = next.selection.main;
  return { text: next.doc.toString(), selected: next.sliceDoc(sel.from, sel.to), head: sel.head, state: next };
}

const cases: Array<[string, () => void]> = [
  ['bold', () => {
    const on = press('bold', 'a word here', 2, 6);
    assertEqual(on.text, 'a **word** here', 'wraps the selection in **');
    assertEqual(on.selected, 'word', 'the word stays selected');
    assertEqual(press('bold', on.text, 4, 8).text, 'a word here', 'a second press unwraps');
    const empty = press('bold', 'ab', 1);
    assertEqual(empty.text, 'a****b', 'an empty selection inserts the pair');
    assertEqual(empty.head, 3, 'the caret sits between the markers');
  }],
  ['italic', () => {
    assertEqual(press('italic', 'a word', 2, 6).text, 'a *word*', 'wraps the selection in *');
    assertEqual(press('italic', 'a *word*', 3, 7).text, 'a word', 'a second press unwraps');
    assertEqual(press('italic', 'a **word**', 4, 8).text, 'a ***word***', 'bold markers are not mistaken for italic');
  }],
  ['heading', () => {
    let doc = 'Title\nbody';
    const seen: string[] = [];
    for (let i = 0; i < 4; i += 1) {
      doc = press('heading', doc, 2).text;
      seen.push(doc.split('\n')[0]);
    }
    assertEqual(seen.join('|'), '# Title|## Title|### Title|Title', 'cycles # -> ## -> ### -> none');
    assertEqual(doc.split('\n')[1], 'body', 'other lines are untouched');
  }],
  ['list', () => {
    const on = press('list', 'one\ntwo\nthree', 0, 7);
    assertEqual(on.text, '- one\n- two\nthree', 'every selected line gains "- "');
    assertEqual(press('list', on.text, 0, 11).text, 'one\ntwo\nthree', 'a second press removes them');
  }],
  ['link', () => {
    const on = press('link', 'see the docs now', 8, 12);
    assertEqual(on.text, 'see the [docs](url) now', 'the selection becomes the link text');
    assertEqual(on.selected, 'url', 'the url is selected for typing');
    const empty = press('link', 'x', 1);
    assertEqual(empty.text, 'x[text](url)', 'an empty selection inserts a skeleton');
  }],
  ['table', () => {
    const skeleton = '| Column 1 | Column 2 |\n| --- | --- |\n|  |  |\n|  |  |';
    assertEqual(press('table', '', 0).text, skeleton, 'a 2x2 skeleton under a header row');
    assertEqual(press('table', 'para', 4).text, `para\n\n${skeleton}`, 'starts on its own line after a blank line');
    assertEqual(press('table', 'para\nnext', 2).text, `para\n\n${skeleton}\n\nnext`, 'inserted after the current line, never splitting it');
  }],
];

let passed = 0;
for (const [name, run] of cases) {
  run();
  passed += 1;
  console.log(`  ok ${name}`);
}
console.log(`toolbar: ${passed} cases passed`);

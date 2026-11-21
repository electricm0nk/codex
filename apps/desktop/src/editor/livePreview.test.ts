import { EditorSelection, EditorState } from '@codemirror/state';
import { markdown, markdownLanguage } from '@codemirror/lang-markdown';
import { buildDecorations, type ResolveImage } from './livePreview';
import { assert, assertEqual } from '../testSupport/asserts';

interface Seen {
  from: number;
  to: number;
  kind: 'line' | 'mark' | 'hide' | 'widget';
  name: string;
}

const noImage: ResolveImage = async () => '';

// The decoration set as data: line/mark classes, hidden ranges, and widget kinds.
function decorate(doc: string, cursor = doc.length, resolveImage = noImage): Seen[] {
  const state = EditorState.create({
    doc,
    selection: EditorSelection.cursor(cursor),
    extensions: [markdown({ base: markdownLanguage })],
  });
  const seen: Seen[] = [];
  buildDecorations(state, resolveImage).between(0, doc.length, (from, to, value) => {
    const spec = value.spec as { class?: string; widget?: { kind?: string } };
    if (spec.widget) seen.push({ from, to, kind: 'widget', name: spec.widget.kind ?? '?' });
    else if (from === to && spec.class) seen.push({ from, to, kind: 'line', name: spec.class });
    else if (value.point) seen.push({ from, to, kind: 'hide', name: doc.slice(from, to) });
    else seen.push({ from, to, kind: 'mark', name: spec.class ?? '' });
  });
  return seen;
}

const has = (seen: Seen[], kind: Seen['kind'], name: string) => seen.some((d) => d.kind === kind && d.name === name);
const off = 'Away from the cursor.\n\n'; // documents put the cursor at the end, on this other line

const cases: Array<[string, () => void]> = [
  ['heading: sized, # hidden off the cursor line', () => {
    const seen = decorate('## Two\n\n' + off);
    assert(has(seen, 'line', 'cm-lp-h2'), JSON.stringify(seen));
    assert(has(seen, 'hide', '## '), 'header mark hidden');
  }],
  ['emphasis and strong: styled, markers hidden', () => {
    const seen = decorate('*em* and **strong**\n\n' + off);
    assert(has(seen, 'mark', 'cm-lp-em') && has(seen, 'mark', 'cm-lp-strong'), JSON.stringify(seen));
    assertEqual(seen.filter((d) => d.kind === 'hide' && /^\*+$/.test(d.name)).length, 4, 'four emphasis markers hidden');
  }],
  ['inline code: styled, backticks hidden', () => {
    const seen = decorate('a `b` c\n\n' + off);
    assert(has(seen, 'mark', 'cm-lp-code'), JSON.stringify(seen));
    assertEqual(seen.filter((d) => d.kind === 'hide' && d.name === '`').length, 2, 'two backticks hidden');
  }],
  ['list bullet: drawn as a bullet', () => {
    const seen = decorate('- item\n\n' + off);
    assert(has(seen, 'widget', 'bullet'), JSON.stringify(seen));
  }],
  ['task box: drawn as a checkbox', () => {
    const seen = decorate('- [x] done\n- [ ] todo\n\n' + off);
    assertEqual(seen.filter((d) => d.kind === 'widget' && d.name === 'task').length, 2, JSON.stringify(seen));
  }],
  ['wikilinks: styled spans for [[Page]], [[Page|alias]], [[Page#heading]]; not inside code', () => {
    const seen = decorate('See [[Old Tam]], [[Old Tam|him]] and [[Ferry#History]]. `[[not a link]]`\n\n' + off);
    assertEqual(seen.filter((d) => d.kind === 'mark' && d.name === 'cm-lp-wikilink').length, 3, JSON.stringify(seen));
  }],
  ['image from Attachments/: rendered inline through resolveImage', () => {
    const asked: string[] = [];
    const seen = decorate('![](Attachments/ferry.png)\n\n' + off, undefined, async (path: string) => {
      asked.push(path);
      return 'data:image/png;base64,';
    });
    assert(has(seen, 'widget', 'image'), JSON.stringify(seen));
    assertEqual(asked.length, 0, 'the image is resolved when drawn, not while decorating');
  }],
  ['fenced code: every line styled as code', () => {
    const seen = decorate('```js\nlet a = 1;\n```\n\n' + off);
    assertEqual(seen.filter((d) => d.kind === 'line' && d.name === 'cm-lp-fence').length, 3, JSON.stringify(seen));
  }],
  ['table: every line monospace', () => {
    const seen = decorate('| a | b |\n|---|---|\n| 1 | 2 |\n\n' + off);
    assertEqual(seen.filter((d) => d.kind === 'line' && d.name === 'cm-lp-table').length, 3, JSON.stringify(seen));
  }],
  ['the cursor line shows its raw markup', () => {
    const doc = '## Two *em* `c` [[Page]]\n- [ ] task ![](Attachments/a.png)\n';
    const onFirst = decorate(doc, 3);
    assert(!onFirst.some((d) => d.kind === 'hide' && d.from < 24), `no marker hidden on the cursor line: ${JSON.stringify(onFirst)}`);
    assert(has(onFirst, 'line', 'cm-lp-h2'), 'styling stays on the cursor line');
    assert(onFirst.some((d) => d.kind === 'widget' && d.from > 24), 'the other line is still previewed');
    const onSecond = decorate(doc, 30);
    assert(!onSecond.some((d) => d.kind === 'widget'), `no widget replaces markup on the cursor line: ${JSON.stringify(onSecond)}`);
    assert(has(onSecond, 'hide', '## '), 'the first line is previewed again');
  }],
];

let passed = 0;
for (const [name, run] of cases) {
  run();
  passed += 1;
  console.log(`  ok ${name}`);
}
console.log(`livePreview: ${passed} cases passed`);

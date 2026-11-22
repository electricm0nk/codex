import { renderReading } from './readingView';
import { assert } from '../testSupport/asserts';

function main() {
  const block = renderReading('# Title\n\n<script>alert(1)</script>\n');
  assert(block.includes('<h1'), 'headings render');
  assert(!block.includes('<script>'), `raw <script> block is escaped: ${block}`);
  assert(block.includes('&lt;script&gt;'), 'the escaped markup is shown as text');

  const inline = renderReading('Hello <img src=x onerror=alert(1)> **bold**');
  assert(!inline.includes('<img'), `inline raw HTML is escaped: ${inline}`);
  assert(inline.includes('<strong>bold</strong>'), 'Markdown still renders');

  const link = renderReading('[bad](javascript:alert(1)) [good](https://obsidian.md) [rel](Wiki/Old%20Tam.md)');
  assert(!link.toLowerCase().includes('javascript:'), `script URLs are dropped: ${link}`);
  assert(link.includes('href="https://obsidian.md"'), 'https links keep their href');
  assert(link.includes('href="Wiki/Old%20Tam.md"'), 'relative links keep their href');

  // A URL parser strips tab, LF and CR anywhere and C0/space at the ends, so `java<TAB>script:` is `javascript:`.
  const hidden = renderReading('[t](<java\tscript:alert(1)>) [n](<\u0001javascript:alert(2)>) ![i](<java\tscript:alert(3)>)');
  assert(!/<a |<img /.test(hidden), `scheme hidden by a tab or control char is dropped: ${JSON.stringify(hidden)}`);

  const table = renderReading('| a | b |\n|---|---|\n| 1 | 2 |\n');
  assert(table.includes('<table>'), 'GFM tables render');
  console.log('readingView: 5 cases passed');
}

main();

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

  const table = renderReading('| a | b |\n|---|---|\n| 1 | 2 |\n');
  assert(table.includes('<table>'), 'GFM tables render');
  console.log('readingView: 4 cases passed');
}

main();

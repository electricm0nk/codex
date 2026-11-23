import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert } from '../testSupport/asserts';
import { makeSurface } from '../testSupport/makeSurface';
import { DeveloperPanel } from '../App';

/**
 * Options > Developer (operator 16 + 19, decisions.md §25): build, platform, backend version +
 * commit, update status and the Feedback evidence panel are kept; the Corpus Ingest Diagnostic is
 * embedded as a section (no Back button); everything else on the old screen is gone.
 */
const html = renderToStaticMarkup(createElement(DeveloperPanel, { surface: makeSurface(), error: null }));

for (const kept of ['>Build<', '>Platform<', '>Backend<', '>Update status<', 'Corpus Ingest Diagnostic']) {
  assert(html.includes(kept), `the Developer tab keeps ${kept}`);
}
for (const removed of ['Feature check', 'Data source', 'Backend feature check', 'Diagnostics</h2>', 'Blocked calculations', 'Explanations', 'Sources', 'Showing fallback data', '>Notes<']) {
  assert(!html.includes(removed), `the Developer tab no longer shows ${removed}`);
}
assert(!html.includes('>Back<'), 'the embedded diagnostic has no Back button');

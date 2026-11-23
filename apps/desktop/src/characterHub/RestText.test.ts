import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { RestTextDto } from '../boundary/loadRestText';
import { assert } from '../testSupport/asserts';
import { RestTextView } from './RestText';

const row = { id: 'r1', system: 'pf1', applies: ['all'], rule: 'Natural healing', text: 'A night of rest restores 1 hit point per level.', book: 'Core Rulebook', page: 'p.181' };
const rows: RestTextDto = { rows: [row], overrideText: null, gap: null };
const html = renderToStaticMarkup(createElement(RestTextView, { rest: rows }));
assert(html.includes('Natural healing') && html.includes('A night of rest restores') && html.includes('Core Rulebook') && html.includes('Core Rulebook p.181') && !html.includes('p. p.'), 'a row shows its rule, words, book and page');
assert(!html.includes('<textarea'), 'the rest text is read-only: no field');
const gm = renderToStaticMarkup(createElement(RestTextView, { rest: { rows: [row], overrideText: 'The GM says: a night at the inn.', gap: null } }));
assert(gm.includes('The GM says') && !gm.includes('Natural healing'), 'the GM override replaces the rows, as the print does');
const gap = renderToStaticMarkup(createElement(RestTextView, { rest: { rows: [], overrideText: null, gap: 'The table lacks rest rules for this system.' } }));
assert(gap.includes('The table lacks rest rules'), 'a gap shows its sentence');

// The Biography branch: the rest text is a sibling below the field and never part of what blur saves.
const sheet = readFileSync(new URL('./CharacterSheet.tsx', import.meta.url), 'utf8');
const branch = sheet.slice(sheet.indexOf("name === 'Biography'"), sheet.indexOf("name === 'Notes'"));
assert(branch.indexOf('<FreeTextTab') >= 0 && branch.indexOf('<RestTextPanel') > branch.indexOf('<FreeTextTab'), 'the rest text renders below the Biography field');
assert(!/updateBio\(\{[^}]*rest/i.test(branch) && !/bio\.biography\s*\+/.test(branch), 'the rest text is never merged into bio.biography');

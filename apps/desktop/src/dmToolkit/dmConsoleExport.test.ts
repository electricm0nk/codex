import { renderDmConsoleHtml } from './dmConsoleExport';
import type { DmRecord } from './dmRecordModel';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * v0.8 D-5: the standalone export. One self-contained HTML file a DM can
 * open on any device: inline CSS, inline script, no external fetches, every
 * record present, every cross-link a working in-page focus. Pinned here on
 * the rendered string, since the file is what ships.
 */
function rec(id: string, kind: DmRecord['kind'], title: string, extra: Partial<DmRecord> = {}): DmRecord {
  return { id, kind, title, summary: '', body: '', fields: {}, links: [], visibility: 'gm', createdAt: '', updatedAt: '', ...extra };
}
const records = [
  rec('bar', 'Place', 'The <Afterlife>', { summary: 'Bar & grill', fields: { gridRef: 'C4' } }),
  rec('rogue', 'Person', 'Rogue', { body: 'Line one\n\nLine "two"', links: [{ targetId: 'bar', label: 'Home base' }, { targetId: 'gone', label: 'Ally' }] }),
];
const html = renderDmConsoleHtml('Night City <Test>', records, 'gm');

assert(html.startsWith('<!doctype html>'), 'a complete document');
assert(html.includes('<title>Night City &lt;Test&gt; — GM copy</title>'), 'campaign name and audience in the title, escaped');
assert(html.includes('The &lt;Afterlife&gt;'), 'record titles are HTML-escaped');
assert(html.includes('Bar &amp; grill'), 'summaries escaped');
assert(html.includes('Line &quot;two&quot;'), 'body escaped');
assert(!html.includes('The <Afterlife>'), 'no raw markup from record text reaches the page');

assert(!/https?:\/\//.test(html), 'no external URLs: the file must work offline');
assert(!/<link\b/i.test(html) && !/\bsrc=/i.test(html), 'no external stylesheet, script or image references');
assert(/<style>/.test(html) && /<script>/.test(html), 'CSS and script are inline');

assert(html.includes('data-record="rogue"'), 'each record renders as an addressable section');
assert(html.includes("focusRecord('bar')"), 'a link becomes an in-page focus call to its target');
assert(!html.includes("focusRecord('gone')"), 'a link to a record that does not exist is not rendered as a dead button');
assert(html.includes('Home base'), 'the relation label is shown');
assert(html.includes('Map grid ref') && html.includes('C4'), 'per-kind fields are rendered with their labels');
assert(html.includes('data-tab="Person"') && html.includes('data-tab="Clue"') && html.includes('data-tab="Rule"'), 'all seven tabs exist even when a kind is empty');
assert(html.includes('No rules yet.'), 'an empty kind says so in the export too');


// D-3a/D-5: audience. The player handout carries only shared records, and a
// link to a GM-only record is absent — not greyed, not a dead button.
const secretBar = rec('bar', 'Place', 'Afterlife', { visibility: 'gm' });
const publicRogue = rec('rogue', 'Person', 'Rogue', { visibility: 'players', links: [{ targetId: 'bar', label: 'Home base' }, { targetId: 'clue1', label: 'Reveals clue' }, { targetId: 'clue2', label: 'Reveals clue' }] });
const secretClue = rec('clue1', 'Clue', 'The owner is alive', { visibility: 'gm', body: 'He was seen in Pacifica.', fields: { trigger: 'Bribe over 500eb' }, links: [{ targetId: 'rogue', label: 'Revealed by' }] });
const sharedClue = rec('clue2', 'Clue', 'The bar changed hands', { visibility: 'players', body: 'Public record.' });
const set = [secretBar, publicRogue, secretClue, sharedClue];

const gm = renderDmConsoleHtml('NC', set, 'gm');
assert(gm.includes('data-record="bar"') && gm.includes('data-record="clue1"'), 'GM copy has everything');
assert(gm.includes('He was seen in Pacifica.'), 'GM copy inlines a linked clue’s content into the record that links to it');
assert(gm.includes('Bribe over 500eb'), 'with its trigger');
const rogueSection = gm.slice(gm.indexOf('data-record="rogue"'), gm.indexOf('data-record="clue1"'));
assert(rogueSection.includes('The owner is alive'), 'the clue appears inside Rogue’s own section, not only under Clues');
assert(gm.includes('data-tab="Clue"') && gm.includes('data-row="clue1"'), 'the flat Clues tab stays in the GM copy');
assert(gm.includes('GM copy'), 'the GM copy says what it is');

const players = renderDmConsoleHtml('NC', set, 'players');
assert(players.includes('data-record="rogue"') && players.includes('data-record="clue2"'), 'shared records are in the handout');
assert(!players.includes('data-record="bar"') && !players.includes('Afterlife'), 'a GM-only record is absent from the handout entirely');
assert(!players.includes("focusRecord('bar')"), 'a link to a GM-only record is omitted, not rendered dead');
assert(!players.includes('The owner is alive') && !players.includes('Pacifica'), 'a GM-only clue is absent — no trace that a secret exists');
assert(players.includes('Public record.'), 'a shared clue is inlined into the shared record that links to it');
assert(players.includes('Player handout'), 'the handout says what it is');

// Backlink leak rule: a GM-only record that links TO a shared one must not
// surface in the shared record's "Linked from" — the handout would otherwise
// reveal that a secret exists and roughly where.
const hideout = rec('hideout', 'Place', 'Secret hideout', { visibility: 'gm', links: [{ targetId: 'rogue', label: 'Resident' }] });
const leakCheck = renderDmConsoleHtml('NC', [publicRogue, hideout], 'players');
assert(!leakCheck.includes('Secret hideout') && !leakCheck.includes("focusRecord('hideout')") && !leakCheck.includes('Linked from'), 'a GM-only source never appears as a backlink in the handout');
const gmSide = renderDmConsoleHtml('NC', [publicRogue, hideout], 'gm');
assert(gmSide.includes('Linked from') && gmSide.includes('Secret hideout'), 'the same backlink is present in the GM copy');

// D-7: a kind with nothing visible has no tab in the handout — "No clues yet."
// would be false there (the clue exists; it was stripped), and the tab's
// presence alone would tell the reader what category was removed.
assert(!players.includes('data-tab="Clue"') || players.includes('data-record="clue2"'), 'a kind whose only records are GM-only has no tab in the handout');
const stripped = renderDmConsoleHtml('NC', [secretBar, publicRogue, secretClue], 'players');
assert(!stripped.includes('data-tab="Clue"') && !stripped.includes('No clues yet.') && !stripped.includes('>Clues<'), 'no Clues tab, list or empty message when every clue is GM-only');
assert(!stripped.includes('data-tab="World"') && !stripped.includes('data-tab="Place"'), 'the same rule for every other kind with nothing visible');
assert(stripped.includes('data-tab="Person"'), 'kinds with visible records keep their tab');
assert(stripped.includes("showTab('Person')"), 'the handout opens on the first kind that is present, not on an absent World tab');
assert(!stripped.includes('.clue{'), 'the clue styling ships only when a clue is rendered, so the source does not reveal the kind either');
assert(gm.includes('data-tab="Rule"') && gm.includes('No rules yet.'), 'the GM copy keeps empty tabs — there the statement is true and a prompt to write');
assert(renderDmConsoleHtml('NC', [], 'players').includes('Nothing has been shared with players yet.'), 'a handout with nothing visible says so once, instead of seven empty tabs');

// D-8: derived history in both exports, same filtering rule as everything else.
const ev1 = rec('ev1', 'Timeline', 'Night 1 — 22:00', { visibility: 'players', fields: { when: 'Night 1, 22:00' }, links: [{ targetId: 'rogue', label: 'Involves' }] });
const ev2 = rec('ev2', 'Timeline', 'The secret meeting', { visibility: 'gm', fields: { when: 'later that night' }, links: [{ targetId: 'rogue', label: 'Involves' }] });
const histGm = renderDmConsoleHtml('NC', [publicRogue, ev1, ev2], 'gm');
const rogueGm = histGm.slice(histGm.indexOf('data-record="rogue"'), histGm.indexOf('data-record="ev1"'));
assert(rogueGm.includes('History') && rogueGm.includes('Night 1 — 22:00') && rogueGm.includes('The secret meeting'), 'GM copy: linked timeline entries listed inside the record');
assert(rogueGm.indexOf('Night 1') < rogueGm.indexOf('secret meeting'), 'in insertion order');
const histPl = renderDmConsoleHtml('NC', [publicRogue, ev1, ev2], 'players');
const roguePl = histPl.slice(histPl.indexOf('data-record="rogue"'), histPl.indexOf('data-record="ev1"'));
assert(roguePl.includes('Night 1 — 22:00') && !histPl.includes('secret meeting'), 'handout: a GM-only entry is absent from a shared record’s history');

console.log('dmConsoleExport tests passed');

import { DM_KIND_FIELDS, DM_KIND_LABELS, DM_RECORD_KINDS, type DmRecord, type DmRecordKind, type DmVisibility } from './dmRecordModel';
import { kindPlural } from './dmConsoleModel';
import { resolveLinks } from './dmLinksModel';

/**
 * Renders the whole campaign console as ONE self-contained HTML file
 * (v0.8 D-5, Q-DM3): inline CSS, inline script, no images, no external
 * fetches, so a DM can open it on any phone/tablet/laptop at the table.
 * Layout mirrors the live console — six tabs, list left, record right —
 * and every cross-link is an in-page focus, the way NoDA's export works.
 * Read-only: authoring stays in the app.
 *
 * Two audiences from the same records (D-3a, the World Anvil secrets
 * model): the GM copy holds everything; the player handout holds only
 * `visibility: 'players'` records, rendered against that subset alone, so
 * a link to a GM-only record — and a GM-only clue — is simply absent, with
 * no trace that a secret exists. Clues linked to a Person / Place / Scene
 * are inlined into that record's page as well as listed under Clues, the
 * way NoDA reads at the table.
 *
 * Everything the DM typed is HTML-escaped on the way in; record ids are
 * UUIDs from the store and are additionally escaped where they land in
 * attributes and script literals.
 */

export function escapeHtml(text: string): string {
  return text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;').replace(/'/g, '&#39;');
}

function jsString(text: string): string {
  return `'${text.replace(/\\/g, '\\\\').replace(/'/g, "\\'").replace(/</g, '\\x3c')}'`;
}

function paragraphs(text: string): string {
  return text
    .split(/\n{2,}/)
    .map((block) => block.trim())
    .filter((block) => block !== '')
    .map((block) => `<p>${escapeHtml(block).replace(/\n/g, '<br>')}</p>`)
    .join('');
}

/** Clues attached to this record in either direction — linked from it, or linking to it. */
function attachedClues(record: DmRecord, records: readonly DmRecord[]): DmRecord[] {
  if (record.kind === 'Clue') {
    return [];
  }
  const outgoing = resolveLinks(record, records).map((link) => link.target);
  const incoming = records.filter((source) => source.links.some((link) => link.targetId === record.id));
  const seen = new Set<string>();
  return [...outgoing, ...incoming].filter((candidate) => candidate.kind === 'Clue' && !seen.has(candidate.id) && seen.add(candidate.id));
}

function renderClueBlock(clue: DmRecord): string {
  const trigger = (clue.fields.trigger ?? '').trim();
  return `<div class="clue"><div><span>Clue</span>${trigger ? `<small>${escapeHtml(trigger)}</small>` : ''}</div><button type="button" onclick="focusRecord(${jsString(clue.id)})">${escapeHtml(clue.title)}</button>${clue.summary ? `<p>${escapeHtml(clue.summary)}</p>` : ''}${paragraphs(clue.body)}</div>`;
}

function renderRecord(record: DmRecord, records: readonly DmRecord[]): string {
  const fields = DM_KIND_FIELDS[record.kind].filter((spec) => (record.fields[spec.key] ?? '').trim() !== '');
  const links = resolveLinks(record, records);
  const backlinks = records.filter((source) => source.links.some((link) => link.targetId === record.id));
  const clues = attachedClues(record, records);
  const linkButton = (target: DmRecord, label: string) =>
    `<button class="link" type="button" onclick="focusRecord(${jsString(target.id)})"><span><strong>${escapeHtml(target.title)}</strong><small>${escapeHtml(target.summary || DM_KIND_LABELS[target.kind].singular)}</small></span><b>${escapeHtml(label)}</b></button>`;
  return [
    `<article class="record" data-record="${escapeHtml(record.id)}" data-kind="${record.kind}" hidden>`,
    `<header><span>${DM_KIND_LABELS[record.kind].singular}</span><h1>${escapeHtml(record.title)}</h1>${record.summary ? `<p>${escapeHtml(record.summary)}</p>` : ''}</header>`,
    fields.length > 0
      ? `<dl>${fields.map((spec) => `<div><dt>${spec.label}</dt><dd>${escapeHtml(record.fields[spec.key]).replace(/\n/g, '<br>')}</dd></div>`).join('')}</dl>`
      : '',
    record.body.trim() !== '' ? `<section><h2>Notes</h2>${paragraphs(record.body)}</section>` : '',
    clues.length > 0 ? `<section><h2>Clues</h2>${clues.map(renderClueBlock).join('')}</section>` : '',
    links.length > 0 ? `<section><h2>Links</h2>${links.map((link) => linkButton(link.target, link.label)).join('')}</section>` : '',
    backlinks.length > 0
      ? `<section><h2>Linked from</h2>${backlinks
          .map((source) =>
            linkButton(
              source,
              source.links
                .filter((link) => link.targetId === record.id)
                .map((link) => link.label)
                .join(' · '),
            ),
          )
          .join('')}</section>`
      : '',
    `</article>`,
  ].join('');
}

function renderList(kind: DmRecordKind, records: readonly DmRecord[]): string {
  const items = records.filter((record) => record.kind === kind);
  if (items.length === 0) {
    return `<p class="empty">No ${kindPlural(kind)} yet.</p>`;
  }
  return items
    .map(
      (record) =>
        `<button type="button" class="row" data-row="${escapeHtml(record.id)}" data-search="${escapeHtml([record.title, record.summary, record.body, ...Object.values(record.fields)].join('\n').toLowerCase())}" onclick="focusRecord(${jsString(record.id)})"><span>${escapeHtml(record.summary || DM_KIND_LABELS[record.kind].singular)}</span><strong>${escapeHtml(record.title)}</strong></button>`,
    )
    .join('');
}

const CSS = `
*{box-sizing:border-box}html,body{margin:0;min-height:100%;background:#0f1216;color:#e8ebef;font-family:system-ui,Segoe UI,Helvetica,Arial,sans-serif}
button{font:inherit}
.console{height:100vh;display:flex;flex-direction:column}
.top{display:flex;align-items:center;gap:20px;padding:12px 20px;border-bottom:1px solid #2a3139;flex-wrap:wrap}
.brand{font:800 12px ui-monospace,monospace;letter-spacing:.14em;text-transform:uppercase;color:#8ab4ff;border:2px solid #8ab4ff;padding:2px 8px}
.top h1{font-size:14px;margin:0;font-weight:700}
nav{display:flex;flex-wrap:wrap}nav button{border:0;border-bottom:3px solid transparent;background:none;color:#98a2ad;font-weight:700;padding:8px 14px;cursor:pointer}nav button.active{color:#fff;border-bottom-color:#8ab4ff}
.panes{flex:1;min-height:0;display:grid;grid-template-columns:340px minmax(0,1fr)}
.master{min-height:0;display:flex;flex-direction:column;border-right:1px solid #2a3139}
.master .head{display:flex;justify-content:space-between;padding:10px 14px;border-bottom:1px solid #2a3139;font:800 10px ui-monospace,monospace;letter-spacing:.12em;text-transform:uppercase;color:#8ab4ff}.master .head b{color:#6d7885}
.master input{margin:10px;padding:9px 10px;border:1px solid #2a3139;border-radius:8px;background:#161b21;color:#fff}
.list{overflow:auto;padding:0 6px 20px}.list[hidden]{display:none}
.row{width:100%;display:block;text-align:left;padding:10px 12px;border:1px solid transparent;border-bottom-color:#232a32;border-left:3px solid transparent;border-radius:6px;background:none;color:#e8ebef;cursor:pointer}.row:hover{background:#161b21}.row.selected{background:#161b21;border-left-color:#8ab4ff}
.row span{display:block;color:#6d7885;font:700 9px ui-monospace,monospace;text-transform:uppercase;letter-spacing:.07em;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}.row strong{display:block;font-size:13px;margin-top:4px}
.empty{color:#6d7885;text-align:center;padding:16px}
.detail{overflow:auto;padding:32px 40px 80px}
.record{max-width:860px;margin:0 auto}.record[hidden]{display:none}
.record header{border-bottom:1px solid #2a3139;padding-bottom:18px}.record header span,.record h2,dt{color:#8ab4ff;font:800 10px ui-monospace,monospace;text-transform:uppercase;letter-spacing:.14em}
.record h1{font-size:34px;line-height:1.05;letter-spacing:-.03em;margin:8px 0 10px}.record header p{color:#aab4bf;font-size:15px;line-height:1.6;margin:0}
dl{margin:20px 0 0}dl>div{border-bottom:1px solid #2a3139;padding:9px 0}dt{color:#6d7885}dd{margin:4px 0 0;font-size:14px;line-height:1.6}
.record section{margin-top:26px}.record h2{color:#6d7885;border-bottom:1px solid #2a3139;padding-bottom:6px;margin:0 0 10px}.record p{font-size:14px;line-height:1.75;margin:0 0 12px}
.link{width:100%;display:flex;justify-content:space-between;align-items:center;gap:16px;text-align:left;border:1px solid #2a3139;background:#161b21;color:#fff;padding:10px 12px;margin-bottom:7px;border-radius:8px;cursor:pointer}.link:hover{border-color:#8ab4ff}
.audience{margin-left:auto;color:#98a2ad;font:800 10px ui-monospace,monospace;letter-spacing:.12em;text-transform:uppercase}
.link strong{display:block;font-size:13px}.link small{display:block;color:#98a2ad;font-size:11px;margin-top:3px}.link b{color:#8ab4ff;font:800 9px ui-monospace,monospace;white-space:nowrap;text-transform:uppercase}
@media(max-width:760px){.panes{grid-template-columns:1fr;grid-template-rows:40vh 1fr}.master{border-right:0;border-bottom:1px solid #2a3139}.detail{padding:20px}}
`;

const CLUE_CSS = `.clue{border-left:3px solid #e6b450;background:#161b21;padding:10px 12px;margin-bottom:8px;border-radius:0 8px 8px 0}.clue>div{display:flex;justify-content:space-between;gap:12px}.clue span,.clue small{color:#e6b450;font:700 9px ui-monospace,monospace;text-transform:uppercase;letter-spacing:.1em}.clue button{border:0;background:none;color:#fff;padding:0;margin:6px 0 4px;font-weight:800;font-size:13px;cursor:pointer;text-align:left}.clue button:hover{color:#e6b450}.clue p{margin:4px 0 0;font-size:13px}`;

const SCRIPT = `
var current=null;
function showTab(kind){document.querySelectorAll('nav button').forEach(function(b){b.classList.toggle('active',b.dataset.tab===kind)});document.querySelectorAll('.list').forEach(function(l){l.hidden=l.dataset.tab!==kind});document.getElementById('search').value='';filterList('');document.querySelector('.master .head span').textContent=document.querySelector('nav button[data-tab="'+kind+'"]').textContent;var first=document.querySelector('.list[data-tab="'+kind+'"] .row');if(!current||document.querySelector('[data-record="'+current+'"]').dataset.kind!==kind){if(first){focusRecord(first.dataset.row)}else{hideAll()}}}
function hideAll(){document.querySelectorAll('.record').forEach(function(r){r.hidden=true});document.querySelectorAll('.row').forEach(function(r){r.classList.remove('selected')});current=null}
function focusRecord(id){var rec=document.querySelector('[data-record="'+id+'"]');if(!rec)return;var kind=rec.dataset.kind;document.querySelectorAll('nav button').forEach(function(b){b.classList.toggle('active',b.dataset.tab===kind)});document.querySelectorAll('.list').forEach(function(l){l.hidden=l.dataset.tab!==kind});document.querySelector('.master .head span').textContent=document.querySelector('nav button[data-tab="'+kind+'"]').textContent;document.querySelectorAll('.record').forEach(function(r){r.hidden=r!==rec});document.querySelectorAll('.row').forEach(function(r){r.classList.toggle('selected',r.dataset.row===id)});current=id;document.querySelector('.detail').scrollTop=0}
function filterList(q){q=q.trim().toLowerCase();document.querySelectorAll('.list:not([hidden]) .row').forEach(function(r){r.hidden=q!==''&&r.dataset.search.indexOf(q)===-1})}

`;

export type DmExportAudience = DmVisibility;

export function renderDmConsoleHtml(campaignName: string, allRecords: readonly DmRecord[], audience: DmExportAudience): string {
  // The handout is rendered against the shared subset ONLY, so every link,
  // backlink and inlined clue resolves inside that subset or not at all.
  const records = audience === 'players' ? allRecords.filter((record) => record.visibility === 'players') : allRecords;
  const audienceLabel = audience === 'players' ? 'Player handout' : 'GM copy';
  const title = `${escapeHtml(campaignName)} — ${audienceLabel}`;
  // D-7: the GM copy shows every kind, including empty ones — "No rules
  // yet." is true there and a prompt to write. The handout shows only kinds
  // with something visible: an empty Clues tab would assert a falsehood
  // (the clues exist; they were stripped) and reveal what was removed.
  const kinds = audience === 'players' ? DM_RECORD_KINDS.filter((kind) => records.some((record) => record.kind === kind)) : [...DM_RECORD_KINDS];
  const firstKind = kinds[0] ?? null;
  const hasClues = records.some((record) => record.kind === 'Clue');
  if (firstKind === null) {
    return [
      '<!doctype html>',
      '<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">',
      `<title>${title}</title><style>${CSS}</style></head><body><div class="console"><div class="top"><span class="brand">DM</span><h1>${escapeHtml(campaignName)}</h1><span class="audience">${audienceLabel}</span></div>`,
      '<p class="empty">Nothing has been shared with players yet.</p></div></body></html>',
    ].join('');
  }
  return [
    '<!doctype html>',
    '<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">',
    `<title>${title}</title><style>${CSS}${hasClues ? CLUE_CSS : ''}</style></head><body><div class="console">`,
    `<div class="top"><span class="brand">DM</span><h1>${escapeHtml(campaignName)}</h1><nav>${kinds.map((kind) => `<button type="button" data-tab="${kind}" onclick="showTab('${kind}')">${DM_KIND_LABELS[kind].tab}</button>`).join('')}</nav><span class="audience">${audienceLabel}</span></div>`,
    `<div class="panes"><div class="master"><div class="head"><span>${DM_KIND_LABELS[firstKind].tab}</span><b>${records.length} records</b></div><input id="search" placeholder="Search this tab…" oninput="filterList(this.value)">`,
    kinds.map((kind) => `<div class="list" data-tab="${kind}" hidden>${renderList(kind, records)}</div>`).join(''),
    `</div><div class="detail">`,
    records.map((record) => renderRecord(record, records)).join(''),
    `</div></div></div><script>${SCRIPT}document.addEventListener('DOMContentLoaded',function(){showTab('${firstKind}')});</script></body></html>`,
  ].join('');
}

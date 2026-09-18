#!/usr/bin/env node
// Renders docs/testing/ui-smoke-inventory.md FROM spec.json. spec.json is
// the source of truth; this script's only job is to make it readable
// without opening JSON. Re-run after every spec.json edit — nothing
// verifies the doc is in sync, so a stale copy is a real risk this script
// exists specifically to avoid by never being hand-edited.
import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const HERE = dirname(fileURLToPath(import.meta.url));
const APP_ROOT = resolve(HERE, '..', '..');
const SPEC_PATH = join(HERE, 'spec.json');
const OUT_PATH = join(APP_ROOT, '..', '..', 'docs', 'testing', 'ui-smoke-inventory.md');

function escapeCell(value) {
  return String(value ?? '').replace(/\|/g, '\\|').replace(/\n/g, '<br>');
}

function describeSteps(row) {
  const parts = [];
  for (const setupId of row.setup ?? []) {
    parts.push(`_(setup: ${setupId})_`);
  }
  for (const step of row.steps ?? []) {
    if (step.op === 'click') parts.push(`click "${step.target}"`);
    else if (step.op === 'type') parts.push(`type "${step.text}"`);
    else if (step.op === 'key') parts.push(`key ${step.key}`);
    else if (step.op === 'scroll') parts.push(`scroll (${step.ticks ?? 5} ticks)`);
    else if (step.op === 'wait') parts.push(`wait ${step.ms ?? 500}ms`);
    else parts.push(step.op);
  }
  return parts.length ? parts.join('; ') : '_(none)_';
}

function describeMarkerExpected(row) {
  const bits = [];
  if (row.marker) bits.push(`marker: \`${row.marker}\``);
  if (row.expect?.length) bits.push(`expect: ${row.expect.map((s) => `\`${s}\``).join(', ')}`);
  if (row.selectsNonEmpty?.length) bits.push(`selects non-empty: ${row.selectsNonEmpty.map((s) => `\`${s}\``).join(', ')}`);
  return bits.length ? bits.join('<br>') : '_(none — manual row)_';
}

function describeRedCondition(row) {
  const bits = ['marker absent', 'a global forbid string present', "'Loading' still present after the wait budget"];
  if (row.expect?.length) bits.push('any expect string missing');
  if (row.forbid?.length) bits.push(`row forbid string present (${row.forbid.map((s) => `\`${s}\``).join(', ')})`);
  if (row.selectsNonEmpty?.length) bits.push('a listed select has zero options');
  bits.push('a click target not found on screen (layout drift)');
  return bits.join('; ');
}

function main() {
  const spec = JSON.parse(readFileSync(SPEC_PATH, 'utf8'));
  const rows = spec.rows;

  const byScreen = new Map();
  for (const row of rows) {
    if (!byScreen.has(row.screen)) byScreen.set(row.screen, []);
    byScreen.get(row.screen).push(row);
  }

  const manualCount = rows.filter((r) => r.manual).length;

  const lines = [];
  lines.push('# UI smoke inventory');
  lines.push('');
  lines.push(
    '**Generated from `apps/desktop/scripts/ui-smoke/spec.json` by `render-inventory.mjs` — do not hand-edit this file. ' +
      'Edit `spec.json` and re-run `npm run ui-smoke:doc`.**',
  );
  lines.push('');
  lines.push(
    `${rows.length} rows across ${byScreen.size} screens (${manualCount} manual — see each row's own reason). ` +
      'Run with `npm run ui-smoke` (requires `RUN_DESKTOP_AGENT` set to a value other than `default`; see ' +
      '`apps/desktop/.claude/skills/run-desktop/SKILL.md`).',
  );
  lines.push('');
  lines.push(
    '**Denominator discipline**: `results.json` always carries one entry per selected row, written as `not-run` ' +
      "before any row executes and replaced in place as each finishes, so a killed/interrupted run's row count " +
      'still matches its own denominator instead of a shorter file being mistaken for a complete run. ' +
      '**`--resume`** (with `--out <dir>` pointed at a directory already holding a `results.json`): skips ' +
      "re-running any row whose entry there is already `green` or `manual`, and re-runs every other row " +
      "(`not-run`, `red`, `blocked`, or missing entirely). **Auto-recover**: two consecutive command-channel " +
      'stalls trigger one app relaunch + retry of the current row (capped at 3 relaunches per run); a retry ' +
      "that then passes is logged with reason `auto-relaunch`. See `spec.json`'s own top-level `$comment` for " +
      'the full contract.',
  );
  lines.push('');

  for (const [screen, screenRows] of byScreen) {
    lines.push(`## ${screen}`);
    lines.push('');
    lines.push('| id | steps | marker / expected | red condition | manual reason | notes |');
    lines.push('|---|---|---|---|---|---|');
    for (const row of screenRows) {
      lines.push(
        `| \`${row.id}\` | ${escapeCell(describeSteps(row))} | ${escapeCell(describeMarkerExpected(row))} | ${escapeCell(
          row.manual ? '_(not run automatically)_' : describeRedCondition(row),
        )} | ${escapeCell(row.manual ?? '')} | ${escapeCell(row.notes ?? '')} |`,
      );
    }
    lines.push('');
  }

  writeFileSync(OUT_PATH, `${lines.join('\n')}\n`);
  console.log(`Wrote ${OUT_PATH} (${rows.length} rows, ${byScreen.size} screens).`);
}

main();

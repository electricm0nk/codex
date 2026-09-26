// Self-executing test for commandChannel.mjs's sendUntilOk. Run directly:
// `node commandChannel.test.mjs` (exits non-zero on the first failed assertion).
//
// A fake frontend runs as a child process (sendCommand polls synchronously, so the
// acknowledging side cannot live in this process): it claims each command file the
// way ui_probe.rs does (rename), records the command, and writes a probe report whose
// `lastCommand` echoes it -- after a configurable delay, and ok or not-found by script.
//
// The defect this pins (SD-36 F4d, live row create-character-kineticist): a `click Load`
// whose acknowledgement arrived after the 2s ack timeout was SENT AGAIN. The first one
// had already run and opened the sheet, so every resend reported "no element named
// 'Load'" for the whole 150s budget and the row went red with the sheet on screen.
import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import { sendUntilOk } from './commandChannel.mjs';

const FAKE_FRONTEND = `
const fs = require('node:fs');
const [cmdPath, probePath, logPath, script] = process.argv.slice(1);
const plan = JSON.parse(script); // [{delayMs, ok}] per received command; last entry repeats
let n = 0;
const tick = () => {
  let raw = null;
  try { fs.renameSync(cmdPath, cmdPath + '.claimed'); raw = fs.readFileSync(cmdPath + '.claimed', 'utf8'); } catch {}
  if (raw) {
    const cmd = JSON.parse(raw);
    fs.appendFileSync(logPath, cmd.id + '\\n');
    const step = plan[Math.min(n, plan.length - 1)]; n += 1;
    setTimeout(() => {
      const lastCommand = step.ok ? { id: cmd.id, ok: true, matchedName: cmd.target } : { id: cmd.id, ok: false, error: "no element named '" + cmd.target + "'" };
      fs.writeFileSync(probePath + '.tmp', JSON.stringify({ ts: Date.now(), lastCommand }));
      fs.renameSync(probePath + '.tmp', probePath);
    }, step.delayMs);
  }
};
setInterval(tick, 50);
setTimeout(() => process.exit(0), 20000);
`;

function withFakeFrontend(plan, body) {
  const dir = mkdtempSync(join(tmpdir(), 'ui-smoke-channel-'));
  const cmdPath = join(dir, 'cmd.json');
  const probePath = join(dir, 'probe.json');
  const logPath = join(dir, 'received.log');
  writeFileSync(logPath, '');
  const child = spawn(process.execPath, ['-e', FAKE_FRONTEND, cmdPath, probePath, logPath, JSON.stringify(plan)], { stdio: 'ignore' });
  try {
    const result = body({ cmdPath, probePath });
    const received = readFileSync(logPath, 'utf8').split('\n').filter(Boolean);
    return { result, received };
  } finally {
    child.kill();
  }
}

// 1. A late acknowledgement is waited for, never answered with a second send.
{
  const { result, received } = withFakeFrontend([{ delayMs: 3500, ok: true }], ({ cmdPath, probePath }) =>
    sendUntilOk(cmdPath, probePath, { op: 'click', target: 'Load' }, { ackTimeoutMs: 2000, deadlineMs: 12000 }),
  );
  assert.equal(result.ok, true, 'the late acknowledgement is the result');
  assert.equal(received.length, 1, `the command is sent exactly once, got ${received.length} sends`);
}

// 2. A target that is not there yet IS re-sent until it appears (a screen mounting late).
{
  const { result, received } = withFakeFrontend(
    [{ delayMs: 10, ok: false }, { delayMs: 10, ok: false }, { delayMs: 10, ok: true }],
    ({ cmdPath, probePath }) => sendUntilOk(cmdPath, probePath, { op: 'click', target: 'Search' }, { ackTimeoutMs: 2000, deadlineMs: 12000, retryDelayMs: 50 }),
  );
  assert.equal(result.ok, true, 'the third send finds the target');
  assert.equal(received.length, 3, 'a not-found answer is retried');
}

// 3. The deadline bounds the whole call and the last answer is returned.
{
  const started = Date.now();
  const { result } = withFakeFrontend([{ delayMs: 10, ok: false }], ({ cmdPath, probePath }) =>
    sendUntilOk(cmdPath, probePath, { op: 'click', target: 'Nowhere' }, { ackTimeoutMs: 1000, deadlineMs: 1500, retryDelayMs: 50 }),
  );
  assert.equal(result.ok, false);
  assert.match(result.error, /no element named 'Nowhere'/);
  assert.ok(Date.now() - started < 5000, 'returns near the deadline');
}

console.log('commandChannel.test.mjs: all assertions passed');

// Runner-side half of the DOM command channel: writes one queued command
// (click/type/select/key/scroll) to CODEX_UI_PROBE_CMD_FILE, then polls the
// probe file for a report whose `lastCommand.id` echoes back the command it
// just sent. The Rust side (`poll_ui_probe_command` in ui_probe.rs) and the
// frontend (`uiProbe.ts`'s `startCommandChannel`/`executeCommand`) are the
// other two legs — see either's own doc comment for the full picture.
//
// This replaces driving the app via `xdotool` click/type/key: those are
// synthetic OS-level input events racing WebKitGTK's own input pipeline
// under Xvfb, which `run-desktop`'s SKILL.md documents as having
// unpredictable dead stretches. A command sent through this channel instead
// makes the WEBVIEW ITSELF call `el.click()` / dispatch a real
// `KeyboardEvent` — there is no synthetic-input step to race at all.
import { mkdirSync, renameSync, writeFileSync } from 'node:fs';
import { dirname } from 'node:path';

import { readProbeFile, sleepMs } from './probe.mjs';

let counter = 0;
function nextCommandId() {
  counter += 1;
  return `cmd-${process.pid}-${Date.now()}-${counter}`;
}

/**
 * Writes `command` (an object with `op` and whichever of `target`/`text`/
 * `key`/`index` that op needs) to `cmdPath`, write-temp-then-rename so the
 * Rust side's poll (itself a claim-by-rename) never observes a half-written
 * file, then polls `probePath` until a snapshot's `lastCommand.id` matches
 * the id this call generated, or `timeoutMs` elapses.
 *
 * Returns the `UiProbeLastCommand` the frontend reported — `{id, ok,
 * error?, matchedName?}` — or a synthetic `{ok: false}` result of the same
 * shape on a timeout, so callers never need a separate timeout branch.
 */
export function sendCommand(cmdPath, probePath, command, { timeoutMs = 5000, pollIntervalMs = 100 } = {}) {
  const id = nextCommandId();
  const payload = JSON.stringify({ id, ...command });
  mkdirSync(dirname(cmdPath), { recursive: true });
  const tmpPath = `${cmdPath}.tmp-${process.pid}-${Date.now()}`;
  writeFileSync(tmpPath, payload);
  renameSync(tmpPath, cmdPath);

  const deadline = Date.now() + timeoutMs;
  for (;;) {
    const snapshot = readProbeFile(probePath);
    if (snapshot?.lastCommand?.id === id) {
      return snapshot.lastCommand;
    }
    if (Date.now() >= deadline) {
      return { id, ok: false, error: `no probe report acknowledged command '${id}' within ${timeoutMs}ms` };
    }
    sleepMs(pollIntervalMs);
  }
}

/**
 * Polls `probePath` for the acknowledgement of command `id` until `deadline` (epoch ms).
 * Returns the acknowledgement, or `null` if none arrived in time.
 */
function awaitAck(probePath, id, deadline, pollIntervalMs = 100) {
  for (;;) {
    const snapshot = readProbeFile(probePath);
    if (snapshot?.lastCommand?.id === id) {
      return snapshot.lastCommand;
    }
    if (Date.now() >= deadline) {
      return null;
    }
    sleepMs(pollIntervalMs);
  }
}

/**
 * Sends `command` and retries until the frontend answers `ok`, or `deadlineMs` runs out.
 *
 * Two different "not yet" answers, handled differently:
 *  - the frontend ANSWERED `ok: false` (the target is not on screen yet — a screen the
 *    previous step navigated to can mount its content a beat later): re-send after
 *    `retryDelayMs`;
 *  - NO answer within `ackTimeoutMs`: the command may already be running (the webview
 *    drains the channel between `invoke()` round trips, and a heavy screen can hold it for
 *    seconds). Keep waiting for THAT command's answer; never send it again. A re-send of a
 *    click that already ran acts twice, or — the SD-36 F4d case — targets a control the
 *    first click already navigated away from: `click Load` opened the Kineticist sheet
 *    after its 2s ack window, and every re-send answered "no element named 'Load'" for the
 *    whole budget while the sheet was on screen.
 */
export function sendUntilOk(cmdPath, probePath, command, { ackTimeoutMs = 2000, deadlineMs = 150000, retryDelayMs = 150 } = {}) {
  const deadline = Date.now() + deadlineMs;
  let result;
  for (;;) {
    result = sendCommand(cmdPath, probePath, command, { timeoutMs: ackTimeoutMs });
    if (!result.ok && /^no probe report acknowledged/.test(result.error ?? '')) {
      const late = awaitAck(probePath, result.id, deadline);
      if (late) {
        result = late;
      }
    }
    if (result.ok || Date.now() >= deadline) {
      return result;
    }
    sleepMs(retryDelayMs);
  }
}

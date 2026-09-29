// Reads the JSON snapshot `uiProbe.ts` writes (via `record_ui_probe` /
// `ui_probe.rs`) and answers "where is the thing named X" so the runner can
// click by DOM truth instead of a hard-coded pixel coordinate. Pure
// functions over a snapshot object plus a small polling loop around the
// probe file itself — no process/state of its own.
import { existsSync, readFileSync } from 'node:fs';

/**
 * Reads and parses the probe file. Returns `null` (never throws) on a
 * missing file, an empty file, or invalid JSON — all of which are normal
 * transient states while the app is mid-write (the Rust side writes
 * temp-then-rename, but a poll can still land between launches or before
 * the very first report).
 */
export function readProbeFile(path) {
  if (!existsSync(path)) {
    return null;
  }
  let raw;
  try {
    raw = readFileSync(path, 'utf8');
  } catch {
    return null;
  }
  if (!raw || !raw.trim()) {
    return null;
  }
  try {
    return JSON.parse(raw);
  } catch {
    return null;
  }
}

/**
 * Polls `path` until a snapshot appears whose `ts` is strictly newer than
 * `sinceTs` (pass `0` to accept the first snapshot seen at all), or until
 * `timeoutMs` elapses. Returns the freshest snapshot read either way (which
 * may still be stale on a timeout) so callers can decide whether stale data
 * is good enough for their purpose.
 */
export function pollForFreshProbe(path, sinceTs, timeoutMs = 3000, intervalMs = 100) {
  const deadline = Date.now() + timeoutMs;
  let latest = null;
  for (;;) {
    const snapshot = readProbeFile(path);
    if (snapshot) {
      latest = snapshot;
      if ((snapshot.ts ?? 0) > sinceTs) {
        return { snapshot, fresh: true };
      }
    }
    if (Date.now() >= deadline) {
      return { snapshot: latest, fresh: false };
    }
    busyWaitMs(intervalMs);
  }
}

// Node has no synchronous sleep primitive without a native add-on; this
// runner is a short-lived CLI script (not a server), so a tight
// Atomics.wait busy-loop on a scratch SharedArrayBuffer is an acceptable,
// dependency-free way to block synchronously between polls. (Node's main
// thread, unlike a browser/DOM main thread, does permit `Atomics.wait`.)
export function sleepMs(ms) {
  const sab = new SharedArrayBuffer(4);
  const view = new Int32Array(sab);
  Atomics.wait(view, 0, 0, ms);
}
function busyWaitMs(ms) {
  sleepMs(ms);
}

/**
 * Every target (button/a/input/select/textarea/[role=tab]/[role=menuitem])
 * whose derived `name` exactly matches `name`. Order is DOM document order,
 * as emitted by uiProbe.ts.
 */
export function findTargets(snapshot, name) {
  if (!snapshot || !Array.isArray(snapshot.targets)) {
    return [];
  }
  return snapshot.targets.filter((t) => t.name === name);
}

/**
 * The single best target match for `name`: prefers a non-disabled match
 * (a screen can render both a disabled and an enabled control that happen
 * to share a name across a re-render boundary), falling back to the first
 * match of any state. Returns `undefined` when nothing matches at all.
 */
export function findTarget(snapshot, name) {
  const matches = findTargets(snapshot, name);
  if (matches.length === 0) {
    return undefined;
  }
  return matches.find((t) => !t.disabled) ?? matches[0];
}

export function findSelect(snapshot, name) {
  if (!snapshot || !Array.isArray(snapshot.selects)) {
    return undefined;
  }
  return snapshot.selects.find((s) => s.name === name);
}

export function centerOf(rect) {
  return { x: rect.x + rect.w / 2, y: rect.y + rect.h / 2 };
}

export function bodyTextOf(snapshot) {
  return snapshot?.bodyText ?? '';
}

export function headingsOf(snapshot) {
  return Array.isArray(snapshot?.headings) ? snapshot.headings : [];
}

/** True when `needle` appears in the snapshot's headings, body text, or open-dialog labels. */
export function screenContains(snapshot, needle) {
  if (!snapshot) {
    return false;
  }
  if (bodyTextOf(snapshot).includes(needle)) {
    return true;
  }
  if (headingsOf(snapshot).some((h) => h.includes(needle))) {
    return true;
  }
  if (Array.isArray(snapshot.dialogs) && snapshot.dialogs.some((d) => d.includes(needle))) {
    return true;
  }
  return false;
}

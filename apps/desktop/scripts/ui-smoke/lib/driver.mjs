// Thin wrapper around .claude/skills/run-desktop/driver.sh for the UI smoke
// runner. Every call is a short-lived `spawnSync` (matching driver.sh's own
// "no persistent process" design — see its own header comment) so this
// module carries no state of its own beyond the resolved script path.
import { spawnSync } from 'node:child_process';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const HERE = dirname(fileURLToPath(import.meta.url));
// apps/desktop/scripts/ui-smoke/lib -> apps/desktop is three levels up.
export const APP_ROOT = resolve(HERE, '..', '..', '..');
export const DRIVER_SH = join(APP_ROOT, '.claude', 'skills', 'run-desktop', 'driver.sh');
export const READ_CLIPBOARD_PY = join(APP_ROOT, '.claude', 'skills', 'run-desktop', 'read-clipboard.py');

function run(args, opts = {}) {
  const result = spawnSync(DRIVER_SH, args, {
    encoding: 'utf8',
    env: process.env,
    ...opts,
  });
  return {
    status: result.status,
    stdout: result.stdout ?? '',
    stderr: result.stderr ?? '',
  };
}

export function launch() {
  return run(['launch']);
}

export function stop() {
  return run(['stop']);
}

export function isAlive() {
  const result = run(['_app_pid']);
  return result.status === 0 && result.stdout.trim().length > 0;
}

export function stateFilePath() {
  const agent = process.env.RUN_DESKTOP_AGENT || 'default';
  return `/tmp/run-desktop-driver-${agent}.state`;
}

export function click(x, y) {
  return run(['click', String(Math.round(x)), String(Math.round(y))]);
}

export function scroll(x, y, ticks = 5, direction = 'down') {
  return run(['scroll', String(Math.round(x)), String(Math.round(y)), String(ticks), direction]);
}

export function type(text) {
  return run(['type', text]);
}

export function key(keyName) {
  return run(['key', keyName]);
}

export function screenshot(path) {
  return run(['screenshot', path]);
}

export function diagnose() {
  return run(['diagnose']);
}

export function displayNum() {
  const result = run(['_display_num']);
  return result.stdout.trim();
}

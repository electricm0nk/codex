// Keeps every ui-smoke run off the operator's real character store.
//
// SD-36 F6d (operator 2026-09-27: "a lot of test characters littering the
// default database. We only want the one Ironhands character"). The app keeps
// its characters under Tauri's `app_data_dir()`, which on Linux resolves to
// `dirs::data_dir()/<identifier>` (tauri 2.11.5 `src/path/desktop.rs`,
// `PathResolver::app_data_dir`), and `dirs::data_dir()` is `$XDG_DATA_HOME`
// when it is set to an ABSOLUTE path, else `$HOME/.local/share` (dirs 6.0.0
// `src/lin.rs`, `data_dir`; the absolute-path filter is dirs-sys 0.5.0
// `is_absolute_path`). `app_config_dir`/`app_cache_dir` follow
// `$XDG_CONFIG_HOME`/`$XDG_CACHE_HOME` the same way. So a run that hands the
// app its own three XDG roots gets a fresh, empty store (the app seeds Aldric
// Ironhand itself on first run -- `seed_default_character_if_needed`), and
// nothing it creates can land in `~/.local/share/io.electricm0nk.codex`.
//
// The layout (`<root>/data`, `<root>/config`, `<root>/cache`) is shared with
// driver.sh, which derives the same three from `RUN_DESKTOP_DATA_ROOT`; the
// test asks driver.sh itself (`_data_env`) rather than trusting this file's
// copy of the layout.
import { mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { isAbsolute, join, relative, resolve } from 'node:path';

import { APP_ROOT } from './driver.mjs';

const TAURI_CONF = join(APP_ROOT, 'src-tauri', 'tauri.conf.json');

/** The bundle identifier Tauri joins onto data_dir() -- read from tauri.conf.json, never restated. */
export const APP_IDENTIFIER = JSON.parse(readFileSync(TAURI_CONF, 'utf8')).identifier;

/** Mirrors the character_hub.rs `CHARACTERS_ROOT_DIR_NAME` subdirectory of app_data_dir. */
const CHARACTERS_DIR_NAME = 'characters';

/** `dirs::data_dir()` on Linux, for a given environment. */
function dataHome(env) {
  const xdg = env.XDG_DATA_HOME;
  if (xdg && isAbsolute(xdg)) {
    return xdg;
  }
  return join(env.HOME ?? '', '.local', 'share');
}

/** Tauri's `app_data_dir()` as the app would resolve it under `env`. */
export function appDataDirFor(env, identifier = APP_IDENTIFIER) {
  return resolve(dataHome(env), identifier);
}

function isSameOrInside(candidate, container) {
  const rel = relative(resolve(container), resolve(candidate));
  return rel === '' || (!rel.startsWith('..') && !isAbsolute(rel));
}

/**
 * THE GUARD. Throws unless `isolatedAppDataDir` is an absolute path that is
 * neither the real app-data dir nor anywhere inside it. run.mjs calls this
 * before it launches anything; a throw there means no row runs.
 */
export function assertIsolated(isolatedAppDataDir, realAppDataDir) {
  if (!isolatedAppDataDir || !isAbsolute(isolatedAppDataDir)) {
    throw new Error(`ui-smoke app-data dir must be an absolute path, got '${isolatedAppDataDir}'`);
  }
  const iso = resolve(isolatedAppDataDir);
  const real = resolve(realAppDataDir);
  if (iso === real) {
    throw new Error(`ui-smoke refuses to run against the REAL character store (${real})`);
  }
  if (isSameOrInside(iso, real)) {
    throw new Error(`ui-smoke app-data dir ${iso} is inside the REAL character store (${real})`);
  }
}

/** Creates a fresh per-run root with empty data/config/cache dirs under `parent`. */
export function createIsolatedDataRoot({ parent = tmpdir(), label = 'run' } = {}) {
  mkdirSync(parent, { recursive: true });
  const root = mkdtempSync(join(parent, `codex-ui-smoke-${label.replace(/[^A-Za-z0-9_-]/g, '_')}-`));
  for (const dir of Object.values(isolatedXdg(root))) {
    mkdirSync(dir, { recursive: true });
  }
  return root;
}

/** The three XDG roots a run's app sees -- the same layout driver.sh derives. */
export function isolatedXdg(root) {
  return {
    XDG_DATA_HOME: join(root, 'data'),
    XDG_CONFIG_HOME: join(root, 'config'),
    XDG_CACHE_HOME: join(root, 'cache'),
  };
}

/** The environment run.mjs hands driver.sh: `baseEnv` plus the run's data root. */
export function launchEnv(baseEnv, root) {
  return { ...baseEnv, RUN_DESKTOP_DATA_ROOT: root };
}

/** A live process's environment (`/proc/<pid>/environ`), or null if unreadable. */
export function readProcessEnv(pid) {
  try {
    const raw = readFileSync(`/proc/${pid}/environ`, 'utf8');
    const env = {};
    for (const entry of raw.split('\0')) {
      const eq = entry.indexOf('=');
      if (eq > 0) env[entry.slice(0, eq)] = entry.slice(eq + 1);
    }
    return env;
  } catch {
    return null;
  }
}

/** Character ids (directory names) saved under `<appDataDir>/characters`, sorted; [] if none. */
export function characterIds(appDataDir) {
  try {
    return readdirSync(join(appDataDir, CHARACTERS_DIR_NAME), { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => entry.name)
      .sort();
  } catch {
    return [];
  }
}

/** Removes a run's root -- refusing outright if the real app-data dir lives anywhere under it. */
export function removeDataRoot(root, { realAppDataDir }) {
  if (isSameOrInside(realAppDataDir, root)) {
    throw new Error(`refusing to remove ${root}: it contains the REAL character store (${realAppDataDir})`);
  }
  rmSync(root, { recursive: true, force: true });
}

/**
 * Per-row cleanup: deletes (via `deleteById`, which run.mjs routes through the
 * app's own `delete_character` command) every character id `listIds()` now
 * reports that was not in `before`, then re-lists to see what is really gone.
 * Returns `{created, deleted, leftover, errors}` -- `deleted` is what the
 * re-list no longer shows, not what a delete call claimed.
 */
export function cleanupCreatedCharacters({ before, listIds, deleteById }) {
  const beforeSet = new Set(before);
  const created = listIds().filter((id) => !beforeSet.has(id));
  const errors = {};
  for (const id of created) {
    const answer = deleteById(id);
    if (!answer?.ok) {
      errors[id] = answer?.error ?? 'delete failed';
    }
  }
  const remaining = new Set(created.length ? listIds() : []);
  return {
    created,
    deleted: created.filter((id) => !remaining.has(id)),
    leftover: created.filter((id) => remaining.has(id)),
    errors,
  };
}

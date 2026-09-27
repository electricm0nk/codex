// Self-executing test for appDataIsolation.mjs. Run directly:
// `node appDataIsolation.test.mjs` (exits non-zero on the first failed assertion).
//
// Why this exists (SD-36 F6d, operator 2026-09-27: "a lot of test characters
// littering the default database. We only want the one Ironhands character"):
// every ui-smoke run used to launch the real Tauri app against the operator's
// own app_data_dir, so each create/level-up row left a character behind in
// ~/.local/share/io.electricm0nk.codex/characters (287 of 378 dirs there were
// ui-smoke litter). These cases pin the three legs of the fix: the data root a
// run uses is never the real one (the guard), the isolation actually reaches
// the process driver.sh spawns (propagation through the REAL driver.sh, not a
// re-implementation of its layout), and the scratch root is removable without
// ever reaching the real store.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import {
  APP_IDENTIFIER,
  appDataDirFor,
  assertIsolated,
  characterIds,
  cleanupCreatedCharacters,
  createIsolatedDataRoot,
  isolatedXdg,
  launchEnv,
  removeDataRoot,
} from './appDataIsolation.mjs';
import { DRIVER_SH } from './driver.mjs';

const scratch = mkdtempSync(join(tmpdir(), 'appDataIsolation-test-'));
try {
  // --- the identifier is the one tauri.conf.json declares --------------------
  assert.equal(APP_IDENTIFIER, 'io.electricm0nk.codex', 'identifier is read from tauri.conf.json');

  // --- appDataDirFor mirrors dirs::data_dir() + identifier (tauri 2 path resolver) ---
  assert.equal(
    appDataDirFor({ HOME: '/home/op' }),
    '/home/op/.local/share/io.electricm0nk.codex',
    'no XDG_DATA_HOME: $HOME/.local/share/<identifier>',
  );
  assert.equal(
    appDataDirFor({ HOME: '/home/op', XDG_DATA_HOME: '/x/data' }),
    '/x/data/io.electricm0nk.codex',
    'absolute XDG_DATA_HOME wins',
  );
  assert.equal(
    appDataDirFor({ HOME: '/home/op', XDG_DATA_HOME: 'relative/data' }),
    '/home/op/.local/share/io.electricm0nk.codex',
    'a relative XDG_DATA_HOME is ignored, exactly as dirs-sys is_absolute_path does',
  );

  // --- the guard ---------------------------------------------------------------
  const real = '/home/op/.local/share/io.electricm0nk.codex';
  assert.throws(() => assertIsolated(real, real), /REAL character store/, 'the real app-data dir is refused');
  assert.throws(
    () => assertIsolated(`${real}/`, real),
    /REAL character store/,
    'a trailing-slash spelling of the real dir is still refused',
  );
  assert.throws(
    () => assertIsolated(`${real}/scratch/io.electricm0nk.codex`, real),
    /inside the REAL/,
    'a scratch root nested inside the real store is refused',
  );
  assert.throws(() => assertIsolated('relative/dir', real), /absolute/, 'a relative isolated dir is refused');
  assert.doesNotThrow(() => assertIsolated('/tmp/x/data/io.electricm0nk.codex', real), 'a /tmp scratch dir passes');

  // --- a created root has the XDG layout and passes the guard -----------------
  const root = createIsolatedDataRoot({ parent: scratch, label: 'unit' });
  const xdg = isolatedXdg(root);
  assert.deepEqual(xdg, {
    XDG_DATA_HOME: join(root, 'data'),
    XDG_CONFIG_HOME: join(root, 'config'),
    XDG_CACHE_HOME: join(root, 'cache'),
  });
  for (const dir of Object.values(xdg)) {
    assert.ok(existsSync(dir), `${dir} is created fresh`);
  }
  assert.doesNotThrow(() => assertIsolated(appDataDirFor(xdg), appDataDirFor(process.env)));

  // --- propagation: the env run.mjs hands driver.sh reaches the child it spawns ---
  // `_data_env` is driver.sh's own introspection of the exact XDG_* values its
  // `launch` exports to `npx tauri dev` (and so to the app binary). Asking the
  // real script, not re-deriving its layout here, is what keeps this test
  // failing if driver.sh ever stops honouring RUN_DESKTOP_DATA_ROOT.
  const env = launchEnv({ ...process.env, RUN_DESKTOP_AGENT: 'isolation-test' }, root);
  assert.equal(env.RUN_DESKTOP_DATA_ROOT, root, 'launchEnv sets RUN_DESKTOP_DATA_ROOT');
  const probe = spawnSync(DRIVER_SH, ['_data_env'], { encoding: 'utf8', env });
  assert.equal(probe.status, 0, `driver.sh _data_env exits 0 (stderr: ${probe.stderr})`);
  const reported = Object.fromEntries(
    probe.stdout
      .trim()
      .split('\n')
      .map((line) => line.split('=', 2)),
  );
  assert.equal(reported.XDG_DATA_HOME, xdg.XDG_DATA_HOME, 'the spawned child sees the isolated XDG_DATA_HOME');
  assert.equal(reported.XDG_CONFIG_HOME, xdg.XDG_CONFIG_HOME, 'the spawned child sees the isolated XDG_CONFIG_HOME');
  assert.equal(reported.XDG_CACHE_HOME, xdg.XDG_CACHE_HOME, 'the spawned child sees the isolated XDG_CACHE_HOME');

  // --- driver.sh's own guard refuses a data root that IS the real one --------
  const fakeHome = join(scratch, 'home');
  mkdirSync(fakeHome, { recursive: true });
  const refused = spawnSync(DRIVER_SH, ['_data_env'], {
    encoding: 'utf8',
    env: { ...process.env, RUN_DESKTOP_AGENT: 'isolation-test', HOME: fakeHome, XDG_DATA_HOME: join(fakeHome, 'data'), RUN_DESKTOP_DATA_ROOT: fakeHome },
  });
  assert.notEqual(refused.status, 0, 'driver.sh refuses a data root whose data dir is the real one');
  assert.match(refused.stderr, /real app-data/i);

  // --- characterIds lists what the app wrote under <appDataDir>/characters ---
  const appDir = appDataDirFor(xdg);
  assert.deepEqual(characterIds(appDir), [], 'no characters dir yet: empty list, not a throw');
  mkdirSync(join(appDir, 'characters', 'character:b'), { recursive: true });
  mkdirSync(join(appDir, 'characters', 'character:a'), { recursive: true });
  writeFileSync(join(appDir, 'characters', 'stray-file'), 'x');
  assert.deepEqual(characterIds(appDir), ['character:a', 'character:b'], 'directories only, sorted');

  // --- per-row cleanup deletes exactly what the row created ----------------
  {
    const store = new Set(['character:aldric-ironhand']);
    const before = [...store].sort();
    store.add('character:smoke-a');
    store.add('character:smoke-b');
    const deleted = [];
    const outcome = cleanupCreatedCharacters({
      before,
      listIds: () => [...store].sort(),
      deleteById: (id) => {
        deleted.push(id);
        store.delete(id);
        return { ok: true };
      },
    });
    assert.deepEqual(deleted, ['character:smoke-a', 'character:smoke-b'], 'only the created ids are deleted');
    assert.deepEqual(outcome, { created: ['character:smoke-a', 'character:smoke-b'], deleted: ['character:smoke-a', 'character:smoke-b'], leftover: [], errors: {} });
    assert.deepEqual([...store], ['character:aldric-ironhand'], 'the seed survives');
  }
  {
    // A delete that fails (or answers ok but leaves the dir) is reported as leftover, never as deleted.
    const store = new Set(['seed', 'made']);
    const outcome = cleanupCreatedCharacters({
      before: ['seed'],
      listIds: () => [...store].sort(),
      deleteById: () => ({ ok: false, error: 'no acknowledgement' }),
    });
    assert.deepEqual(outcome, { created: ['made'], deleted: [], leftover: ['made'], errors: { made: 'no acknowledgement' } });
  }
  {
    const outcome = cleanupCreatedCharacters({ before: ['seed'], listIds: () => ['seed'], deleteById: () => assert.fail('nothing to delete') });
    assert.deepEqual(outcome.created, [], 'a row that creates nothing deletes nothing');
  }

  // --- removeDataRoot never reaches the real store -----------------------------
  assert.throws(
    () => removeDataRoot(fakeHome, { realAppDataDir: join(fakeHome, '.local', 'share', APP_IDENTIFIER) }),
    /contains the REAL/,
    'a root that contains the real app-data dir is never removed',
  );
  assert.ok(existsSync(fakeHome), 'the refused root is still there');
  removeDataRoot(root, { realAppDataDir: appDataDirFor(process.env) });
  assert.ok(!existsSync(root), 'the isolated root is removed');
} finally {
  rmSync(scratch, { recursive: true, force: true });
}

console.log('appDataIsolation.test.mjs: all assertions passed');

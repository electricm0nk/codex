# SD-36 Epic F · Stage F6d receipt — ui-smoke runs against an isolated app-data root

**Operator, 2026-09-27:** "a lot of test characters littering the default database. We only want the one Ironhands character."
Measured by the orchestrator before this step: `~/.local/share/io.electricm0nk.codex/characters` held 378 character dirs, 287 of them dated 2026-09-17/18/26/27 — ui-smoke rows (create-character-fill-and-submit, F4d, F6c) writing through the real Tauri `app_data_dir`. The orchestrator cleaned the real store separately; this step did not touch its contents.

## Mechanism (fixed at the root, generically — no per-row special cases)

1. **Where the store lives.** Tauri 2.11.5 `src/path/desktop.rs` `PathResolver::app_data_dir` = `dirs::data_dir()/<identifier>`; dirs 6.0.0 `src/lin.rs` `data_dir` = `$XDG_DATA_HOME` if absolute (dirs-sys 0.5.0 `is_absolute_path`), else `$HOME/.local/share`. `app_config_dir`/`app_cache_dir` follow `$XDG_CONFIG_HOME`/`$XDG_CACHE_HOME`. Identifier `io.electricm0nk.codex` is read from `apps/desktop/src-tauri/tauri.conf.json`, not restated.
2. **driver.sh** (`apps/desktop/.claude/skills/run-desktop/driver.sh`): every `launch` exports `XDG_DATA_HOME=<root>/data`, `XDG_CONFIG_HOME=<root>/config`, `XDG_CACHE_HOME=<root>/cache` to `npx tauri dev`, `<root>` = `$RUN_DESKTOP_DATA_ROOT` (default `/tmp/run-desktop-driver-<agent>.appdata`). Refuses (exit 3) a relative root or one whose data dir is, or sits inside, the real one. No opt-out. `_data_env` prints exactly what `launch` exports.
3. **run.mjs** (`apps/desktop/scripts/ui-smoke/run.mjs` + `lib/appDataIsolation.mjs` + `lib/driver.mjs`): creates a fresh `mkdtemp` root under `os.tmpdir()` per run, hands it to driver.sh as `RUN_DESKTOP_DATA_ROOT`; **guard** `assertIsolated` refuses to start (exit 2, no row runs) if the resolved app-data dir equals or is inside the real one; after launch (and after any auto-relaunch) reads the LIVE app process's `/proc/<pid>/environ` and stops if it does not resolve to the isolated dir; a reused `--keep` app is checked the same way. The root is removed at exit unless `--keep-data` (or `--keep`, which leaves the app running on it); removal itself refuses a root that contains the real store.
4. **Rows clean up regardless.** Each executed row lists `<isolated>/characters` before and after; any new id is deleted, once the app is back on the landing screen, through the app's own `delete_character` command via a new DOM-command-channel op `deleteCharacter` (`src/testSupport/uiProbe.ts`, DEV-only). What counts as deleted is what a re-list no longer shows. Each row's `results.json` entry records `app_data_root` and `cleanup {created, deleted, leftover, errors}`.
5. **Docs.** `docs/testing/ui-smoke-inventory.md` regenerated (`npm run ui-smoke:doc`; +2 lines stating the isolation — the rest was already in sync); SKILL.md gains "Isolated app data — the operator's real store is never touched".

## Proof — one real launch, the 4 create/level-up rows

Command (agent `f6d`, one app, stopped at the end):
`RUN_DESKTOP_AGENT=f6d node scripts/ui-smoke/run.mjs --only create-character-fill-and-submit,level-up-fighter6-into-arcane-archer,level-up-fighter6-into-arcane-archer-accept,level-up-fighter6-blockers-and-labels --out <scratch>/f6d-out`

| measure | before (09:44:34) | after (09:49:10) |
|---|---|---|
| `ls ~/.local/share/io.electricm0nk.codex/characters \| wc -l` | 1 | 1 |
| `find ~/.local/share/io.electricm0nk.codex \| wc -l` | 15,380 | 15,380 |
| sha256 of `find … -printf '%p %s %T@'` (every path, size, mtime) | `38446d8e…dbdda2` | `38446d8e…dbdda2` |
| `characters/` mtime | 2026-09-27 09:16:58 | 2026-09-27 09:16:58 |

Real store: **unchanged** — denominator: every entry under `~/.local/share/io.electricm0nk.codex` (15,380), compared by path+size+mtime (`f6d/f6d-real-before.txt`, `f6d/f6d-real-after.txt`).

Run (`f6d/f6d-smoke.log`, `f6d/f6d-results.json`): **4/4 green** (M = 4 rows). Live app environ during the run: `XDG_DATA_HOME=/tmp/codex-ui-smoke-f6d-5Ystq6/data` (and config/cache under the same root). The scratch store held the created characters: 4 created (1 per row: `efa7332a…`, `e2739912…`, `2e7f3281…`, `7b18065d…`), 4 deleted via `delete_character`, 0 leftover; at the end it held only the seed `00000000-0000-0000-0000-000000000001` (Aldric Ironhand, seeded by the app on first run). `Removed isolated app-data root /tmp/codex-ui-smoke-f6d-5Ystq6`; `ls -d /tmp/codex-ui-smoke-f6d-*` afterwards: no such file; no `codex-desktop` process; no state file.

## Tests

- `cd apps/desktop && npm test` → **131/131 test files passed** (`f6d/f6d-npm-test.log`). `run-tests.mjs` now also runs `scripts/ui-smoke/lib/*.test.mjs` (4 files), including the new `appDataIsolation.test.mjs`: identifier from tauri.conf.json; `appDataDirFor` mirrors dirs/tauri (absolute vs relative `XDG_DATA_HOME`); the guard refuses the real dir, a trailing-slash spelling, a nested dir, a relative dir; **propagation through the real driver.sh** (`_data_env` with the env run.mjs builds reports the isolated XDG trio); driver.sh refuses a root onto the real one; cleanup deletes exactly the created ids, reports a failed delete as leftover; `removeDataRoot` refuses a root containing the real store. `uiSmokeCommandChannel.test.ts` adds the `deleteCharacter` op cases.
- `npm run typecheck` → exit 0 (`f6d/f6d-typecheck.log`).
- `scripts/tests/test_run_desktop_driver.sh` → 9 passed, 0 failed (cases 7–8 new; `f6d/f6d-driver-selftest.log`).
- Planted mutations FAIL (`f6d/f6d-mutations.log`): guard made a no-op → test fails; cleanup claiming every id deleted → test fails; driver.sh `_data_env` reporting the caller's real data home → self-test 7 passed, 2 failed.

## Remainder, by mechanism

- The default driver.sh root (`/tmp/run-desktop-driver-<agent>.appdata`) is kept between manual launches by design; a manual session removes it itself. ui-smoke runs never use it (they pass their own root).
- A root kept by `--keep` / `--keep-data` stays in `/tmp` until removed by hand; the run prints its path.

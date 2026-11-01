# Cycle C1 — C / cut tranche-19 and 0.19.0 on the version surfaces

- **Card ID:** C1   **Model:** haiku   **RETRO_ACTOR:** sd39-c1 (not exported in this session; no retro event emitted)
- **Commit SHA:** 49d598c13a1bd19ffd02106e23fa0f7671083a55   **Base SHA:** b48bff2ee8380109e125964dd7ea003225d9a438
- **Files touched (15, = F-2):** the 14 F-1 surfaces + docs/release/current-release.json; plus the two anchor tests are among the 14. Verify: `git show --name-only --format= 49d598c13a | awk 'NF' | wc -l` → 15
- **FILES YOU OWN closure check:** `apps/desktop/src/testSupport/makeSurface.ts` and the six fixture-list files in `buildLabelFixtureFreshness.test.ts` (the test's own FIXTURE_FILES list) all bumped; `buildLabelFixtureFreshness.test.ts` STALE_LABEL left at `Codex 0.11.0-test` (its own convention, not in F-1; not changed).
- **Acceptance criterion (verbatim from epic-breakdown.md):** 0.19.0 on the 14 surfaces + current-release.json, with the F-1 grep empty and the pointer JSON assertion, resolver exit 0, and both version tests.
- **RED:** `apps/desktop/node_modules/.bin/tsx src/releaseChecks/buildVersionTriple.test.ts` after anchors moved to `0.19.` with files still at 0.18.0 → `Error: version "0.18.0" must keep major=0, tranche=19 on tranche-19` (exit 1). Same for `src/release/buildVersionTriple.test.ts`.
- **GREEN:** `buildVersionTriple.test.ts` ×2 → exit 0; `python3 tools/release/resolve_release_notes.py --repo-root .` → prints `docs/release/SD-39-ui-print-corrections/release-notes.md`, exit 0; pointer JSON assertion → `JSON_OK`; `cargo metadata --locked --offline --format-version 1 --no-deps` → `codex-desktop 0.19.0` (lock consistent).
- **Figures:** F-1 grep on the 14 surfaces → clean except Cargo.lock third-party lines (see below). `grep -c '0\.18' apps/desktop/src-tauri/Cargo.lock` → 27, all third-party crates (atk, gtk, gdk, glib, tray-icon, …); the codex-desktop entry is now 0.19.0.
- **Build scope verified:** Cargo.toml/Cargo.lock metadata only (no .rs changed, so no clippy stage). Frontend `npm run typecheck` / `npm run ui-smoke:doc` NOT run: worktree has no node_modules; the changed .ts files are test fixtures and the two version tests were run directly.
- **Clippy before first commit:** not applicable (no .rs file in the diff).
- **Identifier audit / Wired-integration audit:** `git diff --unified=0 2b14020bc7 49d598c13a -- apps crates src | awk '/^\+/ && !/^\+\+\+/' | grep -cE 'STUB|MOCK|Would '` → 0 (run against base 2b14020bc7 for the lane's own diff; the base for this cycle is b48bff2ee8, both have identical code paths for these files).
- **Seed deltas:** not applicable to C1 (no render output).
- **Rulings applied:** none beyond the package.
- **Does not cover:** the cargo test suite was not run (version-only change); the release workflow stamp was not executed; the `npm test` suite was not run.
- **Status:** blocked-escalated
- **Remainder:** the F-1 grep criterion `grep -rln '0\.18\.' <F-1 globs> | grep -v test_resolve_release_notes` → empty cannot hold literally: `apps/desktop/src-tauri/Cargo.lock` still matches on 27 third-party crate pins (`grep -c '0\.18' apps/desktop/src-tauri/Cargo.lock` → 27). The codex-desktop entry is correct. **Ruling needed:** scope the Cargo.lock check to the `codex-desktop` package entry (e.g. `awk '/^name = "codex-desktop"/{f=1} f&&/^version/{print;exit}' apps/desktop/src-tauri/Cargo.lock` → `version = "0.19.0"`), or confirm the literal grep is to be kept and the third-party pins are to be left at 0.18.x. Not changed here.
- **Retro events emitted:** none
- **Next-cycle plan:** after the ruling, mark C1 complete and proceed to E1.0 (depends on C1).

# Cycle C1 — Cycle 1 / 0.20.0 on the 14 surfaces + notes pointer

- **Card ID:** C1   **Model:** haiku   **RETRO_ACTOR:** sd40-c1
- **Commit SHA:** 3ceeb63b6622061d9dde1e0068df4b3d4065d626 (version commit, pushed to `tranche-20`)
- **Base SHA:** 39d505d8db (origin/tranche-20 before the push; 91d8753e2a is the package's last authored commit)
- **Files touched (15):** `.github/workflows/publish-tester-release.yml`, `apps/desktop/package.json`,
  `apps/desktop/src-tauri/Cargo.lock` (codex-desktop entry only), `apps/desktop/src-tauri/Cargo.toml`,
  `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src/operatorTriage/buildOperatorTriageDraft.test.ts`,
  `apps/desktop/src/release/buildVersionTriple.test.ts`, `apps/desktop/src/releaseChecks/buildVersionTriple.test.ts`,
  `apps/desktop/src/testSupport/makeSurface.ts`, `apps/desktop/src/testerWorkbench/feedback/bug/composeBugReport.test.ts`,
  `apps/desktop/src/testerWorkbench/feedback/enhancement/composeEnhancementRequest.test.ts`,
  `apps/desktop/src/testerWorkbench/feedback/evidence/captureFeedbackEvidence.test.ts`,
  `apps/desktop/src/testerWorkbench/loadTesterWorkbenchSurface.test.ts`,
  `apps/desktop/src/testerWorkbench/status/createWorkbenchStatus.test.ts`, `docs/release/current-release.json`.
- **FILES YOU OWN closure check:** `git show --name-only --format= 49d598c13a | awk 'NF' | grep -v current-release.json`
  → 14 paths, identical to the 14 non-pointer paths in the commit above. Second call sites: the two
  `buildVersionTriple.test.ts` files assert `startsWith('0.<tranche>.')` and `tranche=<n> on tranche-<n>`, so they moved
  with the bump (in the same commit, so no state exists where they fail).
- **Acceptance criterion (verbatim from epic-breakdown.md):** `tranche-20` on origin ⊇ `d4489b3557`; one commit sets
  `0.20.0` on the 14 surfaces SD-39's C1 bumped (F-1) and points `current-release.json` at this package's interim notes,
  which resolve.
- **RED (pre-change, run before any edit):**
  - `grep -rln '0\.19\.' --include=*.json --include=*.toml --include=*.ts --include=*.yml --include=*.py apps .github tools docs/release/current-release.json | grep -v node_modules | grep -v target | grep -v package-lock.json | awk 'END{print NR}'` → `13`
  - `awk '/^name = "codex-desktop"/{f=1} f&&/^version/{print;exit}' apps/desktop/src-tauri/Cargo.lock` → `version = "0.19.0"`
  - `python3 -c "...assert d=={'tranche':20,...}"` on `current-release.json` (still `tranche: 19`, SD-39 notes) → `AssertionError`
  - `python3 tools/release/resolve_release_notes.py --repo-root .` → exit 0 but resolves the SD-39 notes path (wrong tranche pointer)
- **GREEN (same commands, after the commit):**
  - `git merge-base --is-ancestor d4489b3557 origin/tranche-20` → yes
  - `git show --name-only --format= 3ceeb63b66 | awk 'NF' | wc -l` → `15`
  - residual `0.19.` grep → `0`
  - `codex-desktop` lock entry → `version = "0.20.0"`
  - `current-release.json` assert → `json ok`
  - `python3 tools/release/resolve_release_notes.py --repo-root .` → prints `docs/release/SD-40-pf-sheet-design/release-notes.md`, exit `0`
  - `cd apps/desktop && npx tsx src/release/buildVersionTriple.test.ts` → exit 0; `npx tsx src/releaseChecks/buildVersionTriple.test.ts` → exit 0
  - Six label tests (`operatorTriage/buildOperatorTriageDraft`, `testerWorkbench/status/createWorkbenchStatus`,
    `testerWorkbench/loadTesterWorkbenchSurface`, `…/feedback/enhancement/composeEnhancementRequest`,
    `…/feedback/evidence/captureFeedbackEvidence`, `…/feedback/bug/composeBugReport`) run with `npx tsx <file>` → exit 0 each
  - `cd apps/desktop/src-tauri && cargo metadata --locked --no-deps --format-version 1` → exit 0 (lockfile consistent with Cargo.toml)
- **Figures:** 15 files — `git show --name-only --format= 3ceeb63b66 | awk 'NF' | wc -l` → 15; 14 version surfaces + 1 pointer.
  Residual `0.19.` hits: 0 (command above). Red ids in SD-39 E2.4 (context only): `python3 -c "...status=='red'..."` → 14.
- **Build scope verified:** `cargo metadata --locked` (lockfile only; no compile). Frontend: the two buildVersionTriple tests
  and the six label tests run with `npx tsx`. No `cargo build` or `cargo test` was run (no `.rs` source changed).
- **Clippy before first commit:** not applicable (no `.rs` source file changed; `Cargo.toml` and `Cargo.lock` are metadata only).
- **Clippy stage + sd24 audit test:** not run (no `.rs` source changed).
- **Identifier audit:** not run as a grep pass (the version commit adds only version and build-label strings, no `sd<n>_`-style identifiers; stated, not measured).
  not run as a grep pass; stated as a non-run check).
- **Wired-integration audit:** not run (no source behaviour changed; the only added lines are version strings and build-label strings).
- **Seed deltas (G-1), golden deltas (G-2):** not applicable; no `print/**` or golden file touched.
- **Looked at:** not applicable (no look or layout change).
- **Rulings applied:** decisions.md §2 (version `0.20.0`, single commit, pointer to the SD-40 notes; the lockfile check reads only
  the `codex-desktop` entry; `apps/desktop/package-lock.json` (`version` 0.14.0) not touched).
- **Does not cover:**
  - the full `npm test`, `npm run typecheck` and `cargo test` suites (not run in this card; population of tests not counted);
  - CI on the `tranche-20` push (none; SD-j: the closure PR's `pr-tests` is the evidence);
  - the published `0.20.<run>` string (`VERSION="0.20.${GITHUB_RUN_NUMBER}"` is a template line, not run);
  - `docs/architecture/release-pipeline.md`'s `0.19.0` example line (not in the grep's file types; E5.4 restamps it per the card);
  - `apps/desktop/package-lock.json` top `version` 0.14.0 (decisions.md §2 notes it is not a release surface);
  - `resolve_release_notes.py` does not refuse the `SD-40 INTERIM NOTES` sentinel (only the placeholder markers); E5.5 owns that refusal.
- **Status:** complete
- **Remainder (partial only):** none.
- **Retro events emitted:** none.
- **Next-cycle plan:** E0.1 and E0.2 side lanes and E1.1 per epic-breakdown.md §0; not started by this card.

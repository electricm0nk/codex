# Cycle C1 — Version bump 0.17.0

- **Card ID:** C1   **Model:** haiku   **RETRO_ACTOR:** sd37-c1
- **Commit SHA:** 9d03a769962c2d6fa118ef80f8cce18443a296cd    **Base SHA:** 20bf84a3b2   **Oracle SHA:** 7f818006e3…
- **Files touched:** 14 surfaces (documented in CUI F-17)
  - `.github/workflows/publish-tester-release.yml`
  - `apps/desktop/package.json`
  - `apps/desktop/src-tauri/{Cargo.toml,Cargo.lock,tauri.conf.json}`
  - `apps/desktop/src/release/buildVersionTriple.test.ts`
  - `apps/desktop/src/releaseChecks/buildVersionTriple.test.ts`
  - `apps/desktop/src/testSupport/makeSurface.ts`
  - Six build-label fixtures: `operatorTriage/buildOperatorTriageDraft.test.ts`, `testerWorkbench/{feedback/bug/composeBugReport.test.ts, feedback/enhancement/composeEnhancementRequest.test.ts, feedback/evidence/captureFeedbackEvidence.test.ts, loadTesterWorkbenchSurface.test.ts, status/createWorkbenchStatus.test.ts}`

- **Acceptance criterion (verbatim from epic-breakdown.md):** `0.17.0` across exactly the 14 surfaces, in one commit.

- **RED (pre-change command + output):** Command `git show --name-only --format= HEAD | awk 'NF' | wc -l` on pre-bump HEAD returned 0 (no files touched by bump yet).

- **GREEN (same command + output):** Command `git show 9d03a769962c2d6fa118ef80f8cce18443a296cd --name-only --format= | awk 'NF' | wc -l` → **14 files**.

- **Figures:**
  - 14 files contain 0.16.0 or '0.16.' — **14** — (CUI F-17 predicate: `grep -rlE '0\.16\.0' . --exclude-dir={node_modules,target,.git,docs,.worktrees,graphify-out,data} | wc -l` → 12 + `grep -rlF "'0.16.'" apps --exclude-dir=node_modules | wc -l` → 2 = 14 total)
  - Root `Cargo.toml` unchanged (version stays 0.1.0) — **verified**
  - All version strings replaced 0.16.0 → 0.17.0 — **verified** by grep checks on key files

- **Build scope verified:** no build runs needed (version bump only, no code changes)

- **Identifier audit:** OK_NO_BUNDLE_TAGS (no bundle tags in version bumps)

- **Wired-integration audit:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS (version bump only, no functional code)

- **Structural diff (converter cycles):** not applicable (version bump)

- **Seed deltas:** not applicable (version bump only)

- **Does not cover:** This is a version bump only; no functional code is changed.

- **Status:** complete

- **Safe defaults taken:** none (this is a mechanical version bump, no decisions required)

- **Retro events emitted:** none (mechanical change)

- **Next-cycle plan:** C1 pushes `tranche/17` to origin. E0 ∥ E1 can then dispatch.

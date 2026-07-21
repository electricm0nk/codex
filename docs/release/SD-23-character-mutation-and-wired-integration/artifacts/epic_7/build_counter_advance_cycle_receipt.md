# Criterion 30 — Build counter advances on promotion (cycle 16)

## Correction first

`decisions.md` §3 originally targeted `0.6.0` (tranche-base 5 → 6) for this step — a repeat of the exact SD-22 tranche-version-bump mistake (see `[[sd22-tranche-version-bump-correction]]` and SD-22's own `31965e5` revert commit). `tranche/5-1` is a dash-release *within* tranche 5, not a new `tranche/N` cut, so the tranche-base digit does not advance. Caught by the operator on 2026-07-21 before this criterion executed (`decisions.md` §3 and the parallel `epic-breakdown.md`/`acceptance-and-verification.md` references corrected in commit `34f2756`, landed on `tranche/5-1` before PR #327 merged).

## Correct target: `0.5.96 → 0.5.97`

Followed the documented four-step process at `docs/release/SD-22/release-closure-checklist.md` (SD-22's own canonical process doc, reused as-is since it's bundle-agnostic):

1. **Bumped the three version files** to `0.5.97`: `apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`. Checked `.github/workflows/publish-tester-release.yml`'s stamp — already `VERSION="0.5.${GITHUB_RUN_NUMBER}"`, no change needed since the `<major>.<tranche-base>` prefix is unchanged.
2. **Build-label fixture refresh** (cosmetic housekeeping): 8 fixture files carrying the hard-coded `Codex 0.5.96-test` sample build label updated to `Codex 0.5.97-test` (`sd11/loadSd11TesterWorkbenchSurface.test.ts`, `sd11/status/createSd11WorkbenchStatus.test.ts`, `sd11/feedback/{bug,enhancement,evidence}/*.test.ts`, `sd15/buildSd15OperatorTriageDraft.test.ts`, `testSupport/makeSurface.ts`), plus `sd22/buildLabelFixtureFreshness.test.ts`'s own `STALE_LABEL` watch-value updated to the value being moved away from (`Codex 0.5.96-test`) so the test continues watching for the *next* stale bump, not this one. `sd21/buildVersionTriple.test.ts` and `sd22/buildVersionTriple.test.ts` only assert the `0.5.` prefix (no exact-build literal) — verified they needed no change.
3. **`cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`** refreshed `Cargo.lock`'s pinned `codex-desktop` version. `npm install --package-lock-only` refreshed `package-lock.json`'s own version field (2-line diff, no dependency-resolution changes).
4. **Committed and PR'd**: since `develop` requires PRs (branch protection: `required_pull_request_reviews` with 0 required approvals, no direct pushes), this landed via a small follow-up branch/PR rather than a direct commit — `tranche/5-1`'s own promotion (Criterion 29) had already merged, so this bump targets `develop` directly.

## Verification

- `cargo test --workspace` — 429 test binaries, 0 failures.
- `npm test` (apps/desktop) — 59/59 test files, including `buildVersionTriple.test.ts` (sd21 + sd22) and `buildLabelFixtureFreshness.test.ts` (the exact test whose watched literal was updated).
- **PR:** https://github.com/electricm0nk/codex/pull/328, CI green, merged `2026-07-21T12:55:20Z`, merge commit `b31258fc52b65c55d1845e7aa3a9f00410fd96f1`.

**Resulting version:** `0.5.97`. Tranche-base remains `5`.

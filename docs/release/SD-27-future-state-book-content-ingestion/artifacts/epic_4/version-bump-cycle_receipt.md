# Cycle version-bump — Epic 4 / Criterion 4.4

- **Cycle ID:** `version-bump`
- **Criterion:** 4.4
- **Owner:** Backend
- **Status:** complete
- **Route class:** Haiku
- **Started at:** 2026-07-28T12:15:00Z
- **Completed at:** 2026-07-28T12:22:00Z

## Inputs

- `apps/desktop/package.json`, `apps/desktop/package-lock.json`,
  `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`,
  `apps/desktop/src-tauri/Cargo.lock` — the version-anchor set.
- `acceptance-and-verification.md`'s requirement that all version-anchor files move together.
- `apps/desktop/src/releaseChecks/buildVersionTriple.test.ts:49` — the enforcing assertion
  (`pkg.startsWith('0.6.')`).

## Outputs

- 5 files bumped `0.6.0 → 0.6.1` (`major.tranche-base.build` scheme; this bundle is the 2nd
  build to land on the `tranche/7` cut).

## Operations

1. Confirmed `apps/desktop/package.json` / `package-lock.json` — flagged as pre-existing dirty
   modifications at session start, unrelated to SD-27 — were genuinely clean before touching them.
2. Bumped all 4 primary anchors via targeted `sed` (not a blanket rewrite, to avoid touching any
   unrelated `0.6.0` string elsewhere in `package-lock.json`'s dependency tree):
   - `apps/desktop/package.json` (root `"version"`)
   - `apps/desktop/package-lock.json` (line 3 root `"version"`, line 9
     `packages[""]."version"` — both confirmed as the root `codex` package entry, not a
     third-party dependency, via direct read of lines 1–12)
   - `apps/desktop/src-tauri/tauri.conf.json` (line 4)
   - `apps/desktop/src-tauri/Cargo.toml` (line 3)
3. Discovered `apps/desktop/src-tauri/Cargo.lock`'s own `codex-desktop` package entry still read
   `version = "0.6.0"` after the `Cargo.toml` bump — a 5th, previously-unlisted anchor. Synced it
   with `cargo check --offline` (not a blanket `cargo generate-lockfile`, to avoid touching
   dependency versions) from inside `apps/desktop/src-tauri/`; confirmed via `git diff --stat` that
   exactly 1 line changed (the local package's own version field).

## Verification

- `grep -n "0.6.1\|0.6.0"` across all 5 files (excluding `Cargo.lock`'s unrelated dependency-tree
  hits) → every version-anchor location now reads `0.6.1`.
- `apps/desktop/src/releaseChecks/buildVersionTriple.test.ts:49`'s `pkg.startsWith('0.6.')`
  assertion remains satisfied by `0.6.1`.
- `cargo check --offline` in `apps/desktop/src-tauri/` completed with exit 0 — only pre-existing,
  unrelated dead-code warnings (`ClassLevelDelta`, `RuleSystemAdapter` trait methods), no new
  errors introduced by the version bump.

## Notes

- `apps/desktop/src-tauri` is a separate Cargo project, not a member of the root workspace, so
  `cargo test --workspace` (the suite run at E1.1/E3.x) does not exercise it. A version-only bump
  carries negligible functional risk; the `cargo check --offline` run above is the appropriate,
  proportionate verification for this cycle rather than a full desktop build/test cycle.
- This is a genuinely new finding for this bundle: `Cargo.lock`'s own-package version field is not
  listed as a version anchor anywhere in the bundle's authoring docs. Recording it here so a future
  cycle's version bump doesn't miss it.

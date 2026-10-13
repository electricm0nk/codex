# Cycle C1 — Serial chain / Cut `tranche-18` + 0.18.0 (14 surfaces + `current-release.json`)

- **Card ID:** C1   **Model:** haiku   **RETRO_ACTOR:** sd38-c1
- **Commit SHA:** 3d8a18c784 (version bump, one commit, 15 files); this receipt and the kanban/progress rows are a second commit on `tranche-18` (see Notes)
- **Base SHA:** bdbbb6769c (`tranche-18` at start, fast-forwarded from origin before any read)
- **Files touched:** the 14 surfaces of `decisions.md §2` (`.github/workflows/publish-tester-release.yml`, `apps/desktop/package.json`, `apps/desktop/src-tauri/{Cargo.toml,Cargo.lock,tauri.conf.json}`, `apps/desktop/src/release/buildVersionTriple.test.ts`, `apps/desktop/src/releaseChecks/buildVersionTriple.test.ts`, `apps/desktop/src/testSupport/makeSurface.ts`, six build-label fixtures) + `docs/release/current-release.json`. Cargo.lock: the `codex-desktop` entry only (line 479). Every file is inside the §2 list; `tools/release/test_resolve_release_notes.py` is not edited.
- **Acceptance criterion (verbatim from epic-breakdown.md):** `tranche-18` exists on origin, cut from PR #398's head bb6107aee0 (which contains `origin/develop`), and one commit sets `0.18.0` on the 14 version surfaces and points `current-release.json` at this package's notes, which resolve (the interim body, `decisions.md §2`).
- **Dependency gate:** `git show origin/tranche-18:docs/release/SD-38-cyberpunk-red/kanban.md` → C0 `complete` (C0 is the only card in group [C0]).
- **Wrong-base control:** `test -d docs && test -d data && test -d scripts && test -f docs/release/SD-38-cyberpunk-red/kanban.md && [ "$(git rev-parse --show-toplevel)" != /home/ubuntu/workspace/repos/codex ]` → pass; tree `.worktrees/sd38-main`, clean before the first write (`git status --porcelain` empty).
- **Ancestry (acceptance):** `git merge-base --is-ancestor bb6107aee0 origin/tranche-18` → exit 0; `git merge-base --is-ancestor origin/develop origin/tranche-18` → exit 0.

## RED (pre-change, failing reason quoted)

- `python3 -c "import json;d=json.load(open('docs/release/current-release.json'));assert d=={'tranche':18,'notes_path':'docs/release/SD-38-cyberpunk-red/release-notes.md'}"` → `AssertionError`, exit 1.
- `python3 tools/release/resolve_release_notes.py --repo-root .` → prints `docs/release/SD-37-starfinder-1e/release-notes.md`, exit 0 (the pointer still names SD-37).

## GREEN (same commands, after the bump)

- JSON assertion → exit 0 (`JSON_GREEN`).
- `python3 tools/release/resolve_release_notes.py --repo-root .` → prints `docs/release/SD-38-cyberpunk-red/release-notes.md`, exit 0.
- `cd apps/desktop && npx tsx src/release/buildVersionTriple.test.ts` → exit 0; `npx tsx src/releaseChecks/buildVersionTriple.test.ts` → exit 0.
- `cd apps/desktop/src-tauri && cargo metadata --locked --format-version 1` → exit 0 (Cargo.lock matches the manifest).
- Frontend gate (`apps/desktop`): `npm run typecheck` → exit 0 (`artifacts/cycle_1/npm-typecheck.log`); `npm test` → `164/164 test files passed.` exit 0 (`artifacts/cycle_1/npm-test.log`); `npm run ui-smoke:doc` → exit 0 (`artifacts/cycle_1/npm-ui-smoke-doc.log`); `git status --porcelain` after the gates → unchanged (no regenerated tracked file).

## Figures + their re-derive commands

- 15 files in the bump: `git show --name-only --format= 3d8a18c784 | awk 'NF' | wc -l` → 15 (14 surfaces + `current-release.json`).
- The 14 surfaces agree with SD-37's bump: `git show --name-only --format= 9d03a76996 | awk 'NF' | sort` diffed against the 14 §2 paths → no difference (2 implementations: the diff and `wc -l` → 14).
- `Cargo.toml` root untouched: `git show 3d8a18c784 -- Cargo.toml | wc -l` → 0 (verified via `git show --name-only`, which lists no root `Cargo.toml`).
- A grep (decisions.md §2 A globs) over `0\.17\.`: before the bump 15 files; after the bump 2 files (`apps/desktop/src-tauri/Cargo.lock`, `tools/release/test_resolve_release_notes.py`). Command: `grep -rln '0\.17\.' --include=*.json --include=*.toml --include=*.lock --include=*.ts --include=*.yml --include=*.py apps .github tools docs/architecture docs/release/current-release.json | sort | awk 'END{print NR}'` → 2 (second implementation: `| sort` listing, same two files).
- `Cargo.lock` remaining `0.17.` lines: `grep -n '0\.17\.' apps/desktop/src-tauri/Cargo.lock` → lines 1440, 1591, 1721, 2483, 2769, 3470: third-party crate versions (`png 0.17.16`, `hashbrown 0.17.1`, and others). They must not change.
- **Correction (retro `sd38-c1-fdc117`):** the decisions.md §2 predicate "the A grep lists only `tools/release/test_resolve_release_notes.py`" is refuted after the bump: `Cargo.lock` still matches on third-party lines. The check that holds is the `codex-desktop` entry: `grep -n -A1 'name = "codex-desktop"' apps/desktop/src-tauri/Cargo.lock` → `version = "0.18.0"`.

## Build scope verified

- `apps/desktop` (frontend): typecheck, npm test, ui-smoke:doc (above).
- `apps/desktop/src-tauri` and root: `cargo metadata --locked` only. No cargo compile, no `cargo test`, no clippy: no `.rs` file changed (`git show --name-only --format= 3d8a18c784 | grep -c '\.rs$'` → 0), and the only Rust-side change is the package version string, which the card's acceptance does not exercise. `CARGO_TARGET_DIR` was claimed (`/home/ubuntu/cargo-target/sd38-main`) and no cargo build ran in it.

## Audits (on added lines, `CARD_BASE=bdbbb6769c`)

- Identifier audit: `OK_NO_BUNDLE_TAGS`.
- Wired-integration audit: `OK_NO_TOKENS`, `OK_NO_NOOP_HANDLERS`, `OK_NO_MOCK_LEAKS`, `OK_NO_WOULD_STRINGS`.
- Clippy + sd24 audit test: not run; no `.rs` file in the diff (R11 is triggered by a `.rs` diff).

## Seed deltas (R7)

- PF render-hash and SF seed lines: not rendered by this card. The bump touches no file under `src/`, `crates/`, or any `.rs`, and no sheet data. Unmeasured, not claimed unchanged.

## Rulings applied

- `decisions.md §2` (branch cut from PR #398's head; 14 surfaces + pointer; resolver contract). Alternative not taken: cutting from `origin/develop` (§2 "Revisit if the operator disagrees").

## Notes (deviation from workflow-instruction §6 step 9)

- The criterion says "one commit, 15 files" and step 9 says the receipt and the kanban/progress rows land in the same commit. Both cannot hold. The bump is one commit with exactly 15 files (`3d8a18c784`), and the receipt, kanban row and progress row are a second commit, following the SD-37 precedent (`9d03a76996` was 14 files, and its receipt was committed separately).

## Does not cover

- The published build triple `0.18.<run>`: stamped only at the first tester publish after the operator merges.
- Cargo compile of either workspace after the version bump (see Build scope).
- Whether `release-notes.md`'s interim body is shipped: E7.8 and E7.11 guard that (decisions.md §2, residual R-7).

- **Status:** complete
- **Retro events emitted:** correction 1 (`docs/retro/events/sd38-c1.jsonl`, id `1791927886689-sd38-c1-fdc117`)
- **Next-cycle plan:** E0.1 (`Depends on` C1, kanban row) is the next dispatchable card once this push lands on `origin/tranche-18`.

---
canonical: true
owner: operator
bundle_id: SD-39
slug: ui-print-corrections
canonical_branch: tranche-19
target_version: 0.19.0
date: 2026-11-01
status: planning-ready
---

# SD-39 Technical Requirements

## 1. Pre-launch prerequisites (each with its check; run by C1 and recorded in its receipt)

| ID | Prerequisite | Check | Closes at |
|---|---|---|---|
| P-1 | `origin/develop` is the cut base and contains PR #401 and #402 | `git merge-base --is-ancestor 5ef2f17b67 origin/develop && git merge-base --is-ancestor 2b14020bc7 origin/develop` exit 0 | C1 |
| P-2 | Working tree clean, no foreign worktree touched | `git status --porcelain \| wc -l` → 0; `git worktree list` recorded | C1 |
| P-3 | PCGen oracle on pin | `bash scripts/verify.sh --only preflight-oracle` PASS | E1.0 |
| P-4 | Crates.io reachable for `typst` | `cargo search typst --limit 1` prints a version (offline → SD-h) | E8.1 |
| P-5 | Doctrine present | `test -f docs/governance/no-stub-mvp-doctrine.md && test -f docs/doctrine-external/identifier-discipline.md` | C1 |
| P-6 | Disk ≥ 24 G free before any full sweep | `df -h /` | every cargo card |
| P-7 | `pdftotext` or a Rust extractor available for golden tests | `which pdftotext \|\| echo use-crate` (the test uses the crate; `pdftotext` is for the human) | E8.1 |
| P-8 | Fresh-install path available (`.deb`, AppImage) | `ls target/release/bundle/` after E9.1's build | E9.2 |
| P-9 | Release-notes pointer resolves after C1 | `python3 tools/release/resolve_release_notes.py --repo-root .` exit 0 | C1 |

## 2. Normative requirements

| ID | Requirement | Verified by |
|---|---|---|
| NR-1 | No live-code PCGen parsing: `src/`, `apps/desktop/**` contain no `.pcg`/`.lst` reader | `pcgen-residue-gate` stage; `grep -rn 'pcg' src apps/desktop/src-tauri/src \| grep -v sidecar` empty |
| NR-2 | Every sheet number printed comes from an explanations row or the saved DTO; no TSX or Typst re-derives a number | E8.MC review; `grep -n 'fn compute\|calc(' apps/desktop/src-tauri/src/print/` empty |
| NR-3 | PDF output is deterministic for identical input (fonts bundled, no timestamps) | golden test hashes two runs equal |
| NR-4 | A4 and US Letter both lay out without clipped text (Typst overflow warnings = 0) | E8.5 asserts Typst diagnostics empty |
| NR-5 | Feat add refuses at zero remaining for the consumed pool; error names the pool | `cargo test feat_budget` |
| NR-6 | Duplicate feat refused unless `stackable` with a different target | `cargo test feat_duplicate` |
| NR-7 | Market lists only the requested category; the Armor list contains zero `Ammunition`/`Weapon`/`Rod` records | `cargo test market_category_filter` (counts per category = 0 cross-type) |
| NR-8 | +N variant price = base + masterwork + N²×(1000 armor/shield \| 2000 weapon) gp | `cargo test enhancement_price_table` against the CRB table (5 armor + 5 weapon rows) |
| NR-9 | Worn applies, carried does not: AC with armor `SelectedInactive` = AC without armor; weight equal in both states | `cargo test worn_vs_carried` on Aldric |
| NR-10 | Slot conflict refused with `"<slot> slot is taken by <item>"`; two rings allowed | `cargo test slot_conflict` |
| NR-11 | Every new selection carries `taken_at_level = Some(level)`; a migrated legacy save has every row `assigned_on_upgrade = true` and the sum of assigned feats ≤ slots | `cargo test ledger_stamp`, `cargo test ledger_migration` |
| NR-12 | Each override flag is consulted by exactly the command(s) `decisions.md §10` names; admitted records carry `override = Some(flag)` | `cargo test overrides_*` (8) |
| NR-13 | `.pcg` import of every fixture produces a character that loads in the app without error; unknown keys counted in the report | `cargo test -p codex-ingest pcg_import_fixtures` |
| NR-14 | SF and CPR seed outputs byte-identical outside E3.5/E8.3; PF deltas explained field by field | G-1 |

## 3. Out of scope

DEF-1…DEF-6 (`forward-scope-register.md §4`); any book-content ingestion; any rules-engine change not
named by a card; the Hermes kanban (retired).

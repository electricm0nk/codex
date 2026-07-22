# Cycle 8-1-adversarial-verification — Epic 8 Closure Epilogue / Criterion 8.1 (adversarial re-check)

- **Card ID:** `t_0fd83dab` (done, board `codex-tranche-5`)
- **Commit SHA:** `27ff6c1` (receipt + progress.md); card-ID backfill in follow-up commit
- **Files touched:**
  - `docs/release/SD-25-ui-evaluation-defect-closure/artifacts/epic_8/8-1-adversarial-verification_cycle_receipt.md` (new — this file)
  - `docs/release/SD-25-ui-evaluation-defect-closure/progress.md` (status-matrix 8.1 row annotation only — adversarial-verification cross-reference)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (docs-only diff; no `apps/desktop/**`, `src/**`, `scripts/**` touched)
- **Wired-integration audit result:** OK_NO_TOKENS (docs-only diff; no shipping source touched)
- **Acceptance criterion (adversarial framing):** Independently verify criterion 8.1's closure-readiness scan — spot-check ≥8 criteria across epics against real receipts AND real repo state, hunt for any "complete" claim not backed by receipt+code, any DISCOVERED entry mis-triaged in either direction, and independently re-run `cargo test`. Either CONFIRM 8.1 with fresh evidence or list concrete discrepancies.
- **Status:** complete — **8.1's closure-readiness assessment CONFIRMED with independent evidence. Zero material discrepancies found.**

## Verdict

**CONFIRM.** Independent re-verification reproduces every load-bearing claim in `closure-readiness-report.md` and `final-scan-cycle_receipt.md`. No criterion was found falsely marked `complete`, no `## DISCOVERED` entry was found wrongly kept-open or wrongly closed, and `cargo test --workspace` is genuinely clean on a full-output run (not a truncated one). The two flagged paper-trail gaps (§3.1 criterion 2.4, §3.2 criterion 7.O) are real, correctly characterized, and correctly scoped as non-blocking.

## 1. Independent spot-check sample (10 criteria across 5 epics — my own sample, not 8.1's list)

Each row: read the receipt on disk AND verified the real repo state the receipt/matrix claims.

| # | Criterion | Independent check performed | Result |
|---|---|---|---|
| 1 | 3.2 Pf1Adapter extraction | `ls apps/desktop/src-tauri/src/pf1_adapter.rs` | Present (35 KB), receipt present — CONFIRMED |
| 2 | 3.3 StubAdapter | `ls .../stub_adapter.rs` (10 KB) + `.../rule_system_adapter.rs` (23 KB) present | CONFIRMED |
| 3 | 3.5 UI adapter-aware (+ its archived-Resolved DISCOVERED claim) | `grep -rn recomputeCharacter apps/desktop/src/ --include=*.tsx` → **real non-test call site** at `CharacterSheet.tsx:812` `await recomputeCharacter(request)` | CONFIRMED — the "Resolved by 3.5" archival is a genuine wired call site, not a wrongly-closed entry |
| 4 | 4.1 pcgen-run-character.sh | Present, executable (`-rwxrwxr-x`, 6.3 KB) | CONFIRMED |
| 5 | 4.2 pcgen-normalize-output.py | Present, executable (9.8 KB) | CONFIRMED |
| 6 | 4.3 pcgen_runner_smoke.rs | Present at **flat** `tests/pcgen_runner_smoke.rs` (12.8 KB), matching the receipt's documented path-correction away from the grant's nested path | CONFIRMED |
| 7 | 7.6 Bard (real bug fixed) | `grep BARD_RECOGNITION_ID src/rules_core/level_up/bard.rs` → `const BARD_RECOGNITION_ID: &str = "class_chassis.spell_baseline.bard";` (`bard.rs:79`), used in filter at `:230` | CONFIRMED — real code fix present |
| 8 | 7.9 Paladin (real bug fixed) | `grep PALADIN_HYBRID_BASELINE_RECOGNITION_ID paladin.rs` → present at `:145` (`class_chassis.hybrid_baseline.paladin`) | CONFIRMED |
| 9 | 7.10 Ranger (real bug fixed) | `grep RANGER_RECOGNITION_ID ranger.rs` → present at `:148` (`class_chassis.hybrid_baseline.ranger`) | CONFIRMED |
| 10 | 8.P registry-0002 exclusion (precursor) | `cargo test --test sd24_wired_integration_audit` run live → **`test result: ok. 5 passed; 0 failed`** | CONFIRMED |

**Receipt inventory:** all 34 cycle receipts + `closure-readiness-report.md` + `discovered-queue-triage` + `sd24-audit-registry-0002-exclusion` receipts are physically on disk under `artifacts/epic_*/`. No criterion the status matrix marks `complete` is missing its receipt.

**Commit existence:** independently `git cat-file -t`-verified 7 claimed SHAs (`3192075`, `97590aa`, `d74dee2`, `8a3e7ee`, `42f972b`, `84c46f8`, `1a5b61e`) — all resolve to their claimed commit subjects.

## 2. `## DISCOVERED` re-triage — hunting for mis-classification in both directions

All 5 spot-checkable "still genuinely open" claims independently reproduced against live repo state (I re-ran the checks, did not trust 8.1's or the triage receipt's text):

- **3.4** `#[allow(dead_code)]` in `pf1_adapter.rs` — present at lines **88, 91** (exact match). Still open. ✓
- **3.5** `CharacterSummaryDto` missing `revision_id` — read struct at `character_hub.rs:153-161`: exactly 7 fields (`character_id`, `display_label`, `game_system`, `schema_version`, `saved_at`, `race_id`, `class_summary`), **no** `revision_id`. (The `revision_id` grep hits at `:458/:522/:608/…` are inside function bodies, not the DTO struct — 8.1's claim is precise.) Still open. ✓
- **5.1** `MonsterId::ALL` constant — `grep "const ALL" beastiary1/mod.rs` → **no match**. Still open. ✓
- **5.1** version drift — `Cargo.toml` = `0.5.97`, `package.json` = `0.5.98`, `tauri.conf.json` = `0.5.98`. Still open; correctly expected to self-resolve at criterion 8.4 (`not-started`). ✓
- **7.N CRB** `equipmods.rs` duplicate keys — `grep -oP 'key:\s*"\K[^"]+' | sort` → **314 duplicates, 344 unique, 658 total** (exact match to the cited numbers). Still open. ✓

**Live DISCOVERED top-level entry count:** `awk`-counted **exactly 10** live entries under `## DISCOVERED` (before `## Resolved`) — matches the triage's 18→10 claim and respects `loop-instruction.md §8`'s 10-entry cap. Did NOT find any entry that a fresh read suggests is already resolved (none of the 5 code-state claims had been touched by a later criterion), nor any archived-Resolved entry that is actually still open (3.5's recomputeCharacter call site is genuinely wired — checked directly).

## 3. `cargo test` independent re-run — and a caveat about verification method

Independently re-ran `cargo test --workspace`, **capturing full output (6,839 lines) with cargo's real exit status** (not piped through `tail`, which would mask cargo's exit code behind `tail`'s):

- `CARGO_EXIT=0`
- **455** `test result:` lines, **every one reports `0 failed`**
- Zero lines matching `^error` / `error[` / `FAILED` / `panicked`

**Clean — independently confirmed, matching 8.1's claim.** Method note for the next cycle: an interim check of mine that piped `cargo test | tail -60` gave a misleadingly-narrow view (only 7 result lines, and the `$?` was `tail`'s exit, not cargo's). The full-capture re-run is the authoritative one. 8.1's own claim was verified live and holds against the full run.

(Frontend Vitest `buildVersionTriple.test.ts` remains the one known-tracked failure from the version drift — out of `cargo test` scope, already in `## DISCOVERED`, self-resolves at 8.4. Not a `cargo test` regression.)

## 4. Flagged-gap re-verification (did 8.1 characterize them honestly?)

- **§3.1 criterion 2.4** — read `artifacts/epic_2/tree-clean-cycle_receipt.md` directly: it **still literally reads** `Status: **BLOCKED** (not complete)` with `git status --porcelain | wc -l = 5 ≠ 0`, and kanban card `t_2da006d4` is **still `blocked`** on the board (confirmed via `hermes kanban list`). Meanwhile `progress.md`'s 2.4 row says `complete`. The three-way mismatch 8.1 flagged is **real and honestly described** — and the underlying substance is true today (working tree carries only the unrelated untracked `graphify-out/cache/`). Correctly scoped as a paper-trail gap, not functional. ✓
- **§3.2 criterion 7.O** — `## Open blockers` now carries the one-line 7.O cross-reference 8.1 added; the blocker's substance is genuinely deferred on Q5. Registration-only fix, correct. ✓
- **§3.4** — `git status --porcelain` shows exactly `?? graphify-out/cache/`, matching 8.1's note precisely. ✓

**Minor cosmetic nit (non-blocking, not a discrepancy in substance):** `closure-readiness-report.md §3.1` phrases the tree state as `git status --porcelain | wc -l = 1` (counting the untracked cache dir), while `progress.md`'s 2.4 annotation phrases it as `= 0 … modulo one unrelated untracked graphify-out/cache/`. Both are substantively correct (porcelain lists the untracked dir as one line; the tree is clean of everything 2.4 checks for). Flagged only for the record; no action taken.

## 5. Conclusion

8.1's closure-readiness scan is **independently CONFIRMED**. The bundle is closure-ready for criterion 8.2 to proceed, with the same two non-blocking operator-judgment items 8.1 already surfaced (2.4 kanban/receipt cleanup; Q5/Q6 open questions). This adversarial cycle found **no criterion falsely marked complete, no product-code gap, no mis-triaged DISCOVERED entry, and no dirty `cargo test`** — corroborating, not rubber-stamping, via freshly-run checks.

- **Notes:** Pure read-only verification cycle; introduces no code change and no new `## DISCOVERED` finding of its own. Only `progress.md`'s 8.1 row gets an adversarial-verification cross-reference (in-grant).
- **Discovery forwards:** none.
- **Next-cycle plan:** criterion 8.2 (architecture closure pipeline, Opus) may proceed. Per register C3, perform the architecture truth-up manually (known `architecture_truth_up.py` regex bug, still unfixed). The 2.4 kanban/receipt bookkeeping cleanup remains an operator judgment call, non-gating.

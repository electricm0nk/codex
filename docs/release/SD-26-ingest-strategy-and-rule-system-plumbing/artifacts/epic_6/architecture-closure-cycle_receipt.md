# Cycle architecture-closure — Epic 6: Closure Epilogue / Criterion 6.2 (Architecture closure pipeline)

- **Card ID:** (no kanban card minted this cycle — dispatch brief's landing procedure enumerated receipt + progress.md + commit/push only; hermes kanban daemon assignee hazard avoided)
- **Commit SHA:** (this cycle's commit)
- **Files touched:**
  - `docs/architecture/homebrew-and-oracle.md` (oracle_validation section rewritten)
  - `docs/architecture/status.md` (Oracle-parity row regraduated; JSON-cache + book_stub rows added)
  - `docs/architecture/overview.md` (prose + mermaid node + where-things-live table)
  - `docs/architecture/rules-data-tables.md` (new "JSON corpus cache" section; crb bullet)
  - `.gitignore` (added `/graphify-out`)
  - `docs/release/SD-26-.../progress.md` (6.2 row, TODO/DONE, Cycle log)
  - `docs/release/SD-26-.../artifacts/epic_6/architecture-closure-cycle_receipt.md` (this file)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 6 Criterion 6.2 — Architecture closure pipeline: run the architecture truth-up + graphify-update sub-steps of the Epic Closure pipeline (README §Maintenance contract sub-steps 2–3), and serve as the Opus adversarial-verify pass over 6.1's final criterion scan.
- **Status:** complete

## Part A — Architecture truth-up (manual)

The truth-up script `~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py`
has a documented bug: its `parse_source_dirs_index` regex expects literal `||`
cell delimiters, but `docs/architecture/README.md`'s index uses single-pipe
rows, so it always returns an empty path→doc map and falsely reports "no
architecture impact." Truth-up was therefore done by hand: mapped SD-26's real
changed paths to docs via the index "Source dirs" column, then edited each
impacted doc in place (current-state-only; obsolete statements removed, not
annotated).

**Docs updated and why:**

1. `homebrew-and-oracle.md` — **the load-bearing correction.** Its
   `oracle_validation/` section still quoted the pre-SD-26 `mod.rs` doc
   comment ("Comparator, normalization, parity-report, and PCGen-runner
   behavior are intentionally out of scope for this slice and live in later
   GE-05 slices") and carried a "### Deferred" section stating nothing under
   `oracle_validation/` could compare a Codex output against legacy evidence.
   SD-26 Epic 2 landed exactly those four modules. Replaced the stale intro
   with the real six-submodule doc comment and replaced the Deferred section
   with real function-level descriptions of `comparator.rs`
   (`compare` → `ComparisonResult`, exact-equality, typed `MismatchReason`),
   `normalization.rs` (`normalize` + the two default rules), `parity_report.rs`
   (`render_parity_report`/`write_parity_report`, PASS/FAIL), and
   `pcgen_runner.rs` (`run_pcgen_character` wrapping the two real scripts,
   typed `PcgenRunnerError`), plus a "pilot-case verification (2.5)" subsection
   naming the real CG-03 mismatch. Added the five new `tests/sd26_*.rs`
   coverage entries. `Last verified` refreshed.

2. `status.md` — **stub graduation.** The "Oracle-parity comparator" row said
   "the in-crate comparator and report-writer are absent" (now false).
   Rewrote it: the harness is real and tested; what remains deferred is a
   *passing* parity claim (the pilot run reports a real FAIL on the CG-03
   ability-modifier bug, and `ClaimTierFloor` still has no `OracleChecked`
   variant). Updated the "PCGen runner scaffolding" real-today row's stale
   cross-reference. Added a real-today row for the `data/corpus/` JSON cache
   and a stubbed-row for the 21 `data/stubs/` `book_stub` future-state
   placeholders. `Last verified` refreshed.

3. `overview.md` — the "What Codex is" paragraph said "that comparator does
   not exist yet"; corrected to describe the real in-crate comparator while
   keeping the honest "no passing verdict yet" truth. Updated the side-surface
   mermaid node and the "Where things live" table (oracle-parity harness;
   new `data/corpus/` + `data/stubs/` row). `Last verified` refreshed.

4. `rules-data-tables.md` — added a "JSON corpus cache (`data/corpus/`)"
   section documenting the SD-26 Epic 3 dump-not-reparse generation
   (`src/rules_core/cache_gen/`, the `src/bin/*` generator binaries,
   `crb/json_cache.rs`'s Shape-B record types), with real per-book record
   counts and the round-trip test files; added `json_cache.rs` to the `crb/`
   bullet. `Last verified` refreshed.

**Verification one-liners (README §Maintenance contract):** both clean —
cited-path existence check reports 0 MISSING, relative-link check reports 0
BROKEN LINK.

## Part A — Graphify update

The graphify-update wrapper (`.../graphify-update/scripts/update_graphify.py`)
requires a bundle `receipts.md` (this bundle keeps per-epic artifact receipts +
`progress.md`, not a single ledger) and a clean tree, so the underlying tool
was run directly:

```
$ graphify cluster-only /home/ubuntu/workspace/repos/codex --budget 1000 --exclude ...
Loading existing graph...
Graph: 12243 nodes, 29197 edges
Re-clustering...
[graphify] backed up semantic graph (6 files) -> 2026-07-23/
Skipped graph.html: Graph has 12243 nodes - too large for HTML viz (limit: 5000).
Done - 632 communities. GRAPH_REPORT.md and graph.json updated.
EXIT: 0
```

Graphify succeeded (exit 0). `graphify-out/` is regenerated build output and
was added to `.gitignore` (not committed as a source artifact).

## Part B — Adversarial verification of 6.1's final criterion scan

Independently re-verified 6.1's key substantive claims against the live repo
(not trusting its report prose):

- **21 E4 `book_stub` entries.** `ls data/stubs/*.json | wc -l` → 21;
  `grep -cE '^### [0-9]+ — \`book_stub\`:' docs/governance/wired-integration-stubs-registry.md`
  → 21. Match. Sample (`bestiary_4.json`) carries the honest future-state
  shape (`content_kind_counts: null`, `planned_resolution_bundle:
  "SD-27+ (unscheduled)"`). **CONFIRMED.**
- **4 E3 JSON caches with real record counts.** `data/corpus/` holds
  core_rulebook (3326: 2663 equipment + 652 spell + 11 class),
  advanced_players_guide (641), advanced_class_guide (423), beastiary (45) —
  real, non-trivial counts. Live tests green: `sd26_cache_core_rulebook` 6/6,
  `sd26_cache_apg` 7/7, `sd26_cache_acg` 9/9, `sd26_cache_beastiary` 7/7.
  **CONFIRMED.**
- **E2's 5 files + cited test counts (run live).**
  `src/oracle_validation/{comparator,normalization,parity_report,pcgen_runner}.rs`
  all present (+ the `sd26_pilot_case_verification` end-to-end test).
  `cargo test --locked`: `sd26_comparator` 4/4, `sd26_normalization` 7/7,
  `sd26_parity_report` 10/10, `sd26_pcgen_runner` 6/6 (incl. a real 32s PCGen
  engine invocation), `sd26_pilot_case_verification` 2/2 — including
  `full_pipeline_runs_end_to_end_and_finds_two_genuine_skill_mismatches`, the
  genuine CG-03 skill mismatch. **CONFIRMED.**

Opus did not rubber-stamp 6.1: each claim above was re-derived from the live
repo. **Verdict on 6.1's scan: CONFIRMED — no discrepancy found.** 6.1's own
`complete-with-flagged-gaps` verdict and its four flagged kanban/receipt
paper-trail gaps (criteria 1.1, 2.1, 4.6, 5.1) and two already-known real gaps
(CG-03 ability-modifier bug; `planned_resolution_bundle` doc-vs-data
discrepancy) all remain accurately characterized and correctly forwarded for
operator judgment.

## Dual-audit gate

`BASE_BRANCH=$(git merge-base HEAD origin/develop)` = `1af975b1...`

```
Identifier audit:        OK_NO_BUNDLE_TAGS
Wired-integration audit: OK_NO_TOKENS
```

- **Notes:** Criterion 6.2 is a documentation/verification criterion — no
  product code changed, so RED→GREEN TDD does not apply (per template, the
  "test" is the README verification one-liners + the adversarial re-check,
  both run above). Architecture docs under `docs/architecture/` are outside
  the dual-audit path filter; the gate was run over the full SD-26 diff
  anyway and passes, matching 6.1's result.
- **Discovery forwards:** none.
- **Next-cycle plan:** 6.3 (Release notes — Haiku), then 6.4 / 6.5.

---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
board: local-file (Hermes board retired 2026-08-01, SD-30 Decision 14a)
---

# SD-35 Kanban

One row per acceptance criterion. **33 rows covering 30 criteria** — row 29 carries both
remaining closure criteria; row 30 is AT-35-E2-005's disposition cycle, and rows 31, 32 and 33 are
the Epic 2, Epic 3 and Epic 4 wrap-up gates with their correction cycles
(`workflow-instruction.md §5`: one row per extra cycle; `§10` steps 0-3), none of them an
additional criterion. A cycle marks its row from inside the dispatched agent
(`workflow-instruction.md §6` step 8), from the mechanical receipt rows, never from effort.

**Status vocabulary:** `not-started` | `in-progress` | `complete` | `blocked-escalated`.

A cycle that closes part of its population and **names the rest by refused token type, with
counts that sum** leaves its row at `in-progress` and reports `partial`; the next cycle takes
the remainder — bundled back up to the 500-unit floor (`decisions.md §2`). **Needing more
cycles is never `blocked-escalated`.**

**There is no `returned-to-backlog` and no `deferred`.** A blocker on the Definition of Done is
cleared or escalated (`../../governance/blocker-closure-doctrine.md`).

**Row hygiene:** per-cycle narrative goes in `progress.md` and the cycle receipt, **not the
Notes column.** Notes here are a pointer, never a story.

| # | Card | Epic | Criterion | Status | Notes (pointer only) |
|---|---|---|---|---|---|
| 1 | `batch-floor-gate` | 1 | AT-35-E1-001 | complete | `artifacts/epic-1-tax-cut/AT-35-E1-001_cycle1_receipt.md` — `1d821cdc8d` |
| 2 | `content-anchored-citations` | 1 | AT-35-E1-002 | complete | `artifacts/epic-1-tax-cut/AT-35-E1-002_cycle1_receipt.md`; `artifacts/epic-1-tax-cut/citation-anchor-proofs.md`; cycle 2 re-verify + stale-pin repair: `artifacts/epic-1-tax-cut/AT-35-E1-002_cycle2_receipt.md` |
| 3 | `test-families-table-driven` | 1 | AT-35-E1-003 | complete | `artifacts/epic-1-tax-cut/AT-35-E1-003_cycle1_receipt.md`; `build-time.json`, `test-list-diff.txt` — `03072aea0c` |
| 4 | `ratio-row-and-gate-scope` | 1 | AT-35-E1-004 | complete | `artifacts/epic-1-tax-cut/AT-35-E1-004_cycle1_receipt.md` — `2bf452b038`; cycle 2 closes the last unscanned SD-35 `.md`: `artifacts/epic-1-tax-cut/AT-35-E1-004_cycle2_receipt.md` — `dd48f73d0f` |
| 5 | `pcgen-residue-gate` | 1 | AT-35-E1-005 | complete | `artifacts/epic-1-tax-cut/AT-35-E1-005_cycle1_receipt.md`; baseline `live_files=260 live_hits=12736` |
| 6 | `sd34-closure-folded` | 1 | AT-35-E1-006 | complete | `artifacts/epic-1-tax-cut/AT-35-E1-006_cycle1_receipt.md`; `docs/retro/sd34-book-completion-retrospective.md`; `artifacts/epic-1-tax-cut/sd34-open-row-map.json` (1,590 of 1,590), `sd34-deferral-dispositions.json` (29 of 29) |
| 7 | `sheet-rule-converter` | 2 | AT-35-E2-001 | complete | `artifacts/epic-2-sheet-rule/AT-35-E2-001_cycle1_receipt.md`; `data/sheet_rules/_refused.json` — `72ad0be010`; cycle 2 re-dispatch re-verifies at HEAD and corrects cycle 1's audit row: `artifacts/epic-2-sheet-rule/AT-35-E2-001_cycle2_receipt.md` |
| 8 | `live-evaluator-and-sheet-section` | 2 | AT-35-E2-002 | complete | `artifacts/epic-2-sheet-rule/AT-35-E2-002_cycle1_receipt.md`; `docs/retro/events/at-35-e2-002.jsonl` — `909bb0837c`; cycle 2 re-dispatch re-verifies at HEAD and corrects cycle 1's fixture line-count figure: `artifacts/epic-2-sheet-rule/AT-35-E2-002_cycle2_receipt.md` |
| 9 | `sheet-complete-status` | 2 | AT-35-E2-003 | complete | `artifacts/epic-2-sheet-rule/AT-35-E2-003_cycle1_receipt.md`; `docs/retro/events/at-35-e2-003.jsonl` — `a81c2a005c`; cycle 2 re-dispatch re-verifies at HEAD and corrects cycle 1's consumer-census after-count: `artifacts/epic-2-sheet-rule/AT-35-E2-003_cycle2_receipt.md` |
| 10 | `token-coverage-ledger` | 2 | AT-35-E2-004 | complete | `artifacts/epic-2-sheet-rule/AT-35-E2-004_cycle1_receipt.md`; `artifacts/epic-2-sheet-rule/token-coverage.json`; `data/sheet_rules/_tokens.json` — `344f18d1e1`; cycle 2 re-dispatch re-verifies at HEAD and corrects cycle 1's batch-floor figure (44 → 7 types ≥500 non-DONE): `artifacts/epic-2-sheet-rule/AT-35-E2-004_cycle2_receipt.md` |
| 11 | `first-corpus-wide-conversion` | 2 | AT-35-E2-005 | complete | Against the amended bar — `epic-breakdown.md` `### AT-35-E2-005` amendment 2026-09-08 + `### AT-35-E2-005-DISPOSITION` hand-off table; `decisions.md §16`; `artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_handoff.json` (1,404 of 1,404 owned); receipts `AT-35-E2-005_cycle1..4_receipt.md`; `oracle-parity/sheet-parity.json` — `cd3d64e578`; cycle 5 re-dispatch re-verifies the amended bar at HEAD and corrects the cycle-4 receipt's `cmp`-on-`ours.json` stability test: `artifacts/epic-2-sheet-rule/AT-35-E2-005_cycle5_receipt.md` — `77fa8a0d31` |
| 30 | `e2-005-disposition` | 2 | AT-35-E2-005-DISPOSITION (row 11's disposition cycle, `§5` one row per extra cycle) | complete | `artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_cycle1_receipt.md`; `AT-35-E2-005-DISPOSITION_handoff.py` / `.json`; `docs/retro/events/at-35-e2-005-disposition.jsonl`; cycle 2 re-dispatch re-derives the hand-off at HEAD and confirms all five obligations unchanged (`owned_sum=1404 unowned=0`, no discoveries): `artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_cycle2_receipt.md` |
| 31 | `epic-2-wrapup-gate` | 2 | Epic 2 wrap-up (`workflow-instruction.md §10` steps 0-3, `§5` one row per extra cycle) — not a criterion | complete | Gate run by the isolated read-only worker at `a542652c5e`: **FAIL, 45 of 48 PASS**, `artifacts/epic-2-sheet-rule/EPIC-2_wrapup_gate_report.md`; `docs/retro/events/at-35-e2-wrapup.jsonl`. Correction cycle fixed all three red stages and the `disk-full` incident-key defect, full gate re-run GREEN: `artifacts/epic-2-sheet-rule/EPIC-2_wrapup_fix_cycle_receipt.md`; `docs/retro/events/at-35-e2-wrapup-fix.jsonl`. **RE-GATE 2 at `f1f547a41e` came back RED: 48 of 49 PASS in 4,378 s, sole red stage `site-dashboard-check` (`artifacts/epic-2-sheet-rule/EPIC-2_wrapup_regate2_report.md`, `docs/retro/events/at-35-e2-regate2.jsonl` — both committed by correction cycle 2, the gate worker pushes nothing).** Correction cycle 2 DISPROVED the gate report's stated root cause (the missing `PF1E_DASHBOARD_STRICT_TIMEOUT=1` on the publish branch: measured, the non-strict producer returns the correct figure in 12.67 s, so no timeout and no stale-dump fallback occurred) and found the real one: `compute_wiring_class_summary()`'s cache is `~/swarm-observer/wiring-class-summary.json`, OUTSIDE the repo and shared by all 15 worktrees, and its freshness test was mtime + schema + a `publishable_document_path()`-normalised name — never the source document's CONTENT, so another tree's summary was served and published under this tree's pin. Control landed: the cache records `source_document_sha256` and the warm path requires it to equal the on-disk digest (`WIRING_SUMMARY_SCHEMA` 13 → 14); RED→GREEN in `scripts/tests/test_pf1e_dashboard_producer.py::ForeignContentCacheIsRejectedTest`, 27 → 30 cases. Artifact repaired, not re-pinned: the public feed's `by_doneness` moved from `done 23,650 / in-progress 17,563` to `done 46,965 / in-progress 160` of 49,438 units. Also fixed two `denominator-gate` violations the copied gate report brought in, and rescued an orphan retro shard from a worktree. Worktree sweep deferred a 5th time — proven prunable but blocked by this agent's permission classifier; now an operator/Epic 7 action: `artifacts/epic-2-sheet-rule/EPIC-2_wrapup_fix_cycle2_receipt.md`; `docs/retro/events/epic2-sheet-rule-fix2.jsonl` |
| 12 | `class-feature-b-zero` | 3 | AT-35-E3-001 | complete | `artifacts/epic-3-place-and-surface/AT-35-E3-001_cycle2_receipt.md` — `406003afc3`; `class_feature` B = 0 (`completion_atlas.py --by-kind`). Cycle 1's `blocked-escalated` superseded by the cycle-2 bundle |
| 13 | `other-kinds-b-zero` | 3 | AT-35-E3-002 | complete | `artifacts/epic-3-place-and-surface/AT-35-E3-002_cycle1_receipt.md` — `26bdfa8d5b`; bucket B 2 → 0 in every kind (`completion_atlas.py --by-kind`). Whole-remainder cycle: `cycle_scope_gate.py --min 500` → `scoped=786 verdict=PASS` |
| 14 | `bucket-c-zero` | 3 | AT-35-E3-003 | complete | `artifacts/epic-3-place-and-surface/AT-35-E3-001_cycle2_receipt.md` — `406003afc3`; bucket C 79 → 0, emptied by the same bundled cycle (`completion_atlas.py --check`). Own verification cycle at HEAD, C = 0 in all 19 kinds + SD-34 register C1.8 dispositioned: `artifacts/epic-3-place-and-surface/AT-35-E3-003_cycle1_receipt.md` — `0e0298d7fe` |
| 15 | `epic-3-rate-ledger` | 3 | AT-35-E3-004 | complete | `artifacts/epic-3-place-and-surface/AT-35-E3-004_cycle1_receipt.md` — `9a728d891f` (cycle start `697b7780ea`); `rate-ledger.json` all 5 Epic 3 cycles, one row each, re-verified against their receipts (0 discrepancies); `builds_recorded` 0/3/3/1/0, `pcgen_live_files` 260 throughout |
| 32 | `epic-3-wrapup-gate` | 3 | Epic 3 wrap-up (`workflow-instruction.md §10` steps 0-3, `§5` one row per extra cycle) — not a criterion | complete | Gate run by the isolated read-only worker at `07e29075b4`: **FAIL, 46 of 48 PASS**, `artifacts/epic-3-place-and-surface/EPIC-3_wrapup_gate_report.md`; `docs/retro/events/at-35-e3-wrapup.jsonl`, `docs/retro/events/epic-3-wrapup-gate.jsonl` (all three committed by this correction cycle — the gate worker pushes nothing). Correction cycle fixed both red stages (`site-dashboard-check` by `./scripts/publish-site-dashboard.sh`; `figure-provenance` by rewriting 16 unsourced figure lines — 14 named plus 2 that landed in `AT-35-E4-001_cycle1_receipt.md` after the gate ran) and raised `BASELINE_ROOT_FULL_TESTS` 8724 → 8727 on a derived attribution. Full gate re-run GREEN, **48 of 48 PASS**: `artifacts/epic-3-place-and-surface/EPIC-3_wrapup_fix_cycle_receipt.md` — `2dc322ae32` (cycle start `e91b1d8873`); `docs/retro/events/at-35-e3-wrapup-fix.jsonl` |
| 33 | `epic-4-wrapup-gate` | 4 | Epic 4 wrap-up (`workflow-instruction.md §10` steps 0-3, `§5` one row per extra cycle) — not a criterion | complete | Gate run by the isolated read-only worker at `5e2c0c8c5b`: **FAIL, 47 of 48 PASS** in 5,432 s, `artifacts/epic-4-resolve-and-verify/EPIC-4_wrapup_gate_report.md`; `docs/retro/events/at-35-e4-wrapup.jsonl`, `docs/retro/events/epic4-wrapup-gate.jsonl` (all three committed by this correction cycle — the gate worker pushes nothing). Sole red stage `figure-provenance`, `violations=2` of `figures_examined=228`, both in `AT-35-E4-003_cycle1_receipt.md` lines 129/131; fixed by moving the three inline figures into the table with a command each. Landed the named mechanical control for incident key `figure-provenance-command-on-next-line` (3rd firing): `§6` step 3 now runs `denominator_gate.py --check-provenance` alongside the different-flag `--check`. Raised `BASELINE_ROOT_TEST_BINARIES` 411 → 412 on an exhaustive attribution (`src/bin/sheet_rule_bucket_v_render.rs`, the only bin/test file added since `e0280a8fea`). Full gate re-run GREEN, **48 of 48 PASS**: `artifacts/epic-4-resolve-and-verify/EPIC-4_wrapup_correction_cycle_receipt.md`; `docs/retro/events/at-35-e4-wrapup-fix.jsonl` |
| 34 | `epic-5-wrapup-gate` | 5 | Epic 5 wrap-up (`workflow-instruction.md §10` steps 0-3, `§5` one row per extra cycle) — not a criterion | complete | Gate run by the isolated read-only worker at `c3500e7984`: **FAIL, 47 of 48 PASS** in 5,053 s, `artifacts/epic-5-residues/EPIC-5_wrapup_gate_report.md`; `docs/retro/events/at-35-e5-wrapup.jsonl` (both already folded to the branch in `5a7d476542` by an Epic 6 cycle's housekeeping — the gate worker pushes nothing). Sole red stage `site-dashboard-check`, `site/dashboard/PF1e-dashboard.json is STALE`; reproduced independently at `00e44eee02` (904 s, exit 1) and fixed by `./scripts/publish-site-dashboard.sh` — a generated artifact republished, not an assertion re-pinned. Landed the named mechanical control for incident key `site-dashboard-json-stale-after-inventory-move` (3rd firing, 7 failing runs of the stage, and the first mechanism it has ever had): a real publish records `sha256(docs/work-inventory.json)` into the new `site/dashboard/inventory-pin.json`, `--check-pin` re-hashes it in milliseconds, and that command is now BOTH `verify.sh` stage `site-dashboard-pin` (48 → 49 stages, both stage sets) and a push-blocking line in `§6` step 3 — moving detection from the 90-minute wrap-up to the cycle that creates the divergence. Named limit: the pin watches one input, so the full `--check` stays. RED→GREEN in `scripts/tests/test_publish_site_dashboard.sh`, 8 → 13 cases. Corrected the gate report's stale-baseline list — it named three, the certified re-run found **four** (it omitted `BASELINE_ROOT_TEST_BINARIES`, and three of its figures had already moved as HEAD advanced six commits past `c3500e7984`): `BASELINE_ROOT_LIB_TESTS` 3223 → 3261, `BASELINE_ROOT_FULL_TESTS` 8734 → 8772, `BASELINE_ROOT_TEST_BINARIES` 412 → 413 (attributed exhaustively to `src/bin/gen_record_vars.rs`, `ac38c5bf3c`), `BASELINE_DESKTOP_TESTS` 574 → 576; correction `1789024087133-at-35-e5-wrapup-fix-d6d8f0`. Full gate re-run GREEN, **49 of 49 PASS** in 3,916 s (logs `/tmp/codex-verify-00NKWe`): `artifacts/epic-5-residues/EPIC-5_wrapup_correction_cycle_receipt.md`; `docs/retro/events/at-35-e5-wrapup-fix.jsonl` |
| 16 | `bucket-m-zero` | 4 | AT-35-E4-001 | complete | `artifacts/epic-4-resolve-and-verify/AT-35-E4-001_cycle1_receipt.md` — `9bae2cfa1f` (cycle start `07e29075b4`); M = 0 and refused non-DONE = 0 carried from `AT-35-E3-002_cycle1_receipt.md` (`26bdfa8d5b`); this cycle closed the third Evidence clause — 24 mapping rows + 1 head alias took `token-coverage.json` `unmapped_token_types` 25 → 0, degraded records 974 → 603, oracle lines 42/41 → 146/145 with 0 new disagreements |
| 17 | `bucket-v-oracle-once` | 4 | AT-35-E4-002 | complete | `artifacts/epic-4-resolve-and-verify/AT-35-E4-002_cycle1_receipt.md` — `2645a3c85a` (cycle start `cdcfc897ea`); V = 0 carried from `AT-35-E3-002_cycle1_receipt.md` (`26bdfa8d5b`); this cycle paid the outstanding Evidence clause — `compared=392 oracle_agree=184 oracle_disagreement=10 of 392 oracle_unverifiable=198` at `PCGEN_ORACLE_SHA=7f818006e...`, export tier 286 of 392 over 21 carriers (0 failures), all 10 disagreements named and booked as Epic 6's parity baseline (`1788955474431-at-35-e4-002-0f136c`) |
| 18 | `epic-4-rate-ledger` | 4 | AT-35-E4-003 | complete | `artifacts/epic-4-resolve-and-verify/AT-35-E4-003_cycle1_receipt.md` — `ea9650ffc9` (cycle start `e7f66b1f80`); `artifacts/epic-4-resolve-and-verify/rate-ledger.json` — 3 rows, 0 transcription discrepancies, totals 3 cycles / 0 closed / 0 relabeled / 302 rust lines / 1 build, `pcgen_live_files` 260 → 260, `ratio_over_the_epic` null (Epic 4's population was already 0 at its first cycle); `docs/retro/events/at-35-e4-003.jsonl` |
| 19 | `bucket-a-two-tables` | 5 | AT-35-E5-001 | complete | Units emptied at `406003afc3` (`artifacts/epic-3-place-and-surface/AT-35-E3-001_cycle2_receipt.md`); `missing_engine_tables.py --check` → `population=0`. Cycle 1 pays the second Evidence clause (the refusal/success transcript pair, unpaid until then): `artifacts/epic-5-residues/AT-35-E5-001_cycle1_receipt.md`, `artifacts/epic-5-residues/table-proofs.md`, `docs/retro/events/at-35-e5-001.jsonl` — `7a0bf64bbf` |
| 20 | `bucket-d-zero` | 5 | AT-35-E5-002 | complete | Units emptied at `51f91bba11` (1,982 → 43) and `406003afc3` (43 → 0). Cycle 1 pays the second Evidence clause: `artifacts/epic-5-residues/AT-35-E5-002_cycle1_receipt.md`, `AT-35-E5-002_bucket_d_sub_causes.py` (14 families, 1,982 units, `not_sheet_complete_at_HEAD=0`), `docs/retro/events/at-35-e5-002.jsonl` |
| 21 | `buckets-u-z-zero` | 5 | AT-35-E5-003 | complete | Units emptied at `26bdfa8d5b` (`artifacts/epic-3-place-and-surface/AT-35-E3-002_cycle1_receipt.md`); U 202 → 0, Z 19 → 0. Cycle 1 pays the per-sub-cause and `beginner_box` clauses and fixes the defect that paying them exposed (166 package files printing PCGen's editorial not-implemented marker): `artifacts/epic-5-residues/AT-35-E5-003_cycle1_receipt.md`, `AT-35-E5-003_buckets_u_z.py` (4 sub-causes, 221 units, `not_sheet_complete_at_HEAD=0`), `docs/retro/events/at-35-e5-003.jsonl` |
| 22 | `bucket-x-choice-filter` | 5 | AT-35-E5-004 | complete | Units emptied at `26bdfa8d5b` (`artifacts/epic-3-place-and-surface/AT-35-E3-002_cycle1_receipt.md`); X 168 → 0. Cycle 1 pays the second Evidence clause and clears deferral `1788922132640-at-35-e3-002-ac4da5`: the filter is built (`src/rules_core/level_up_option_filter.rs`, served on `preview_level_up` as `featOptions`/`refusedFeatOptions`), plus the converter defect it exposed (354 files, `PreStatScore_<AB>` read 0) — `artifacts/epic-5-residues/AT-35-E5-004_cycle1_receipt.md` (`a56096b861`, `ef4dd0102a`) |
| 23 | `corpus-49438-of-49438` | 5 | AT-35-E5-005 | complete | `artifacts/epic-5-residues/AT-35-E5-005_cycle1_receipt.md` — `103693b365` (cycle start `ac2165b393`); atlas `DONE=49438 of 49438`, every other bucket 0; `completion-manifest.json` 49,438 rows, 0 non-DONE; `capability-register-rederived.json` 11 of 11 rows closed (5 built, 6 unnecessary-under-sheet-rule, 0 open) over 11,055 units. Residue named and gated, not exempted: `desc-without-prose.json`, 10 units, `AT-35-E5-005_desc_without_prose.py --check` exits 1 until 0 — converter-side, deferral `1788994100821-at-35-e5-005-5973cb`; 2 corrections in `docs/retro/events/at-35-e5-005.jsonl` |
| 24 | `formula-evaluator-leaves-live` | 6 | AT-35-E6-001 | done | `artifacts/epic-6-pcgen-exit/AT-35-E6-001_cycle4_receipt.md` — cycle 4; `PcgenFormulaEvaluator`, `bonus_stack_reader`, `pre_tokens` all `files=0 hits=0`; residue `253/12336 → 253/12256` |
| 25 | `generators-leave-rules-core` | 6 | AT-35-E6-002 | not-started | |
| 26 | `desktop-and-prose-leave-pcgen` | 6 | AT-35-E6-003 | not-started | |
| 27 | `pcgen-residue-zero` | 6 | AT-35-E6-004 | not-started | |
| 28 | `final-acceptance-scan` | 7 | AT-35-E7-001 | not-started | |
| 29 | `retro-sweep-archdocs-pr` | 7 | AT-35-E7-002 + AT-35-E7-003 | not-started | |

## Gating

```
Epic 1 — Tax cut (rows 1-6)                   cheaper builds; the two counters exist from cycle 1; SD-34's closure folded
   |   (rows 3, 6, and 1-2-5 in three worktrees; row 4 after all)
   v
Epic 2 — Sheet rule (rows 7-11)               converter + our schema; live evaluator; first corpus-wide conversion, oracle-checked
   |
   v
Epic 3 — Place and surface (rows 12-15)       B and C to zero
   |
   v
Epic 4 — Resolve and verify (rows 16-18)      M to zero by token family; V through the oracle once
   |
   v
Epic 5 — Residues (rows 19-23)                A, D, U, Z, X; corpus at 49,438 of 49,438
   |
   v
Epic 6 — PCGen exit (rows 24-27)              the old path comes out; residue gate reads zero; oracle agrees before and after
   |
   v
Epic 7 — Closure epilogue (rows 28-29)
```

**Epics 2–6 are strictly sequential** — every one of them writes `src/bin/v06_work_inventory.rs`,
`docs/work-inventory.json`, or `src/rules_core/` (`decisions.md §9` L12). Speed comes from
batch size, not lane count.

**Row 11 (`first-corpus-wide-conversion`) is the bundle's measurement**, and its oracle
comparison is the first proof that the rewrite is solid. **Row 27 (`pcgen-residue-zero`) is the
operator's boundary ruling made mechanical** (`decisions.md §11`). **Row 6
(`sd34-closure-folded`) is the debt SD-34 left at its merge**, cleared here rather than carried.

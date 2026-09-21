---
canonical: true
bundle_id: SD-36
---

# SD-36 Kanban

One row per epic. Status updated as each completes.

---

| Epic | Title | Acceptance | Status | Cycle count | Notes |
|---|---|---|---|---|---|
| B | Freeze status, retire producers (55,827 lines) | B1–B9 in epic-breakdown.md | **done** (corrected 2026-09-20 — was stale `open`/0; see retrospective "What did not work") | 7 (7 independent-verifier rounds) | Closed 2026-09-15/17; 46/46 `verify.sh` PASS logged at round 5 (`receipts/epic-b_receipt.md`). Unblocked A scope. |
| A | PCGen wall: crate codex-ingest (82k lines moved) | A1–A11 in epic-breakdown.md | done | 1 | Completed 2026-09-19. Unblocks C scope. A2's residue-gate `--closure` class has an open post-move regression — see release-notes.md "Known follow-ups". |
| E | SD-35 code-review correctness fixes (22 findings, folded in per 2026-09-15 operator ruling) | Disposition table in `receipts/epic-e_receipt.md`; not in original epic-breakdown.md | **done** (added to this board 2026-09-20 — had no row) | 2 (2 independent-verifier fix cycles) | 16/22 fixed whole, 3 partial (real mitigations, larger half escalated: FS-7, FS-9), 4 deferred with retro (CONV-06/07/08 grouped; PC4-1 since resolved). |
| C1 | Source refactor: split pilot_compute, consolidate paths | C1.1–C1.5 in epic-breakdown.md | done | 1 | Completed 2026-09-20. 42 submodules (largest 6,160 lines), path consolidation via `src/support/paths.rs`. Unblocks C2. |
| C2 | Test rewrite: table-driven sd18_widening/sd13_progression | C2.1–C2.6 in epic-breakdown.md | **done** (C2.1/C2.2 closed 2026-09-20 — this pass) | 2 (C2D gap-closure pass, then the C2.1/C2.2 rewrite pass) | Follows C1. C2.1/C2.2: `tests/sd18_widening/rows.rs` (182 rows) + `tests/sd13_progression/rows.rs` (143 rows) landed; `--list` byte-identical before/after both families; 891/1,136 passed, 0 failed; three-sabotage mutation gate confirms identical failing-name sets before/after (`receipts.md`'s Epic C2.1/C2.2 evidence section). C2.1/C2.2's own acceptance command as literally written in `epic-breakdown.md` does not reproduce (2,219 vs. the real 891+1,136) — see retrospective Finding 1; the corrected per-binary `--list` counts are what was actually gated on. `tests/support/paths.rs` tracked (C2.3); branch-promotion test moved (C2.4); oracle tests re-run 52/52 green against real PCGen corpus (C2.5, `receipts.md`); `scripts/verify-baselines.env` C2D block corrects 5 stale floors (C2.6). Uncommitted as of this docs pass — lands in the commit that follows. |
| D1 | Closure: architecture docs, full-set rewrite (§10 operator ruling 2026-09-20) | D1 in epic-breakdown.md | **done** | 1 | `docs/architecture/README.md` "Last verified" header now 2026-09-20 @ `b22ea9e113`. 9 docs updated, 1 deleted (`support-state-matrix.md`), 2 added (`getting-started.md`, `glossary.md`). |
| F0 | Class completion: permanent census instrument (RED first) | F0.1–F0.4 in epic-breakdown.md | **done** | 5 (F0a registry, F0b base sweep + stage, F0c prestige carrier, F0d mix panel, F0e generated table) | Closed 2026-09-21. Permanent instrument (`src/rules_core/class_census.rs` + `src/bin/class_census.rs`) measures, today: `ids=135 computed=42 blocked=93`; prestige `swept=74 alone_blocked=74 mix_computed=0`; multiclass mix panel `swept=185 computed=185 blocked=0`. `scripts/verify.sh`'s `class-census` stage runs the bin AND (F0e) `scripts/gen_class_status_table.py --check`, so `docs/architecture/status.md`'s "Class/level compute coverage" table (between `<!-- class-census:begin -->`/`<!-- class-census:end -->`) can never silently drift from these numbers again. Commits `b32178ac10`/`65d6d456fe`/`ecff9e8380`/`0cb4d975dd` + this cycle's F0e commit. |
| F1 | Class completion: converter link repair (Option A) + weapon proficiency reader | F1.1–F1.9 in epic-breakdown.md | **open** | 0 | Own worktree + own `CARGO_TARGET_DIR`; population run gated on F1b's n=1/n=5 classification. |
| F1b | Class completion: print-path reconciliation (new batch, required by Option A) | F1b.0–F1b.5 in epic-breakdown.md | **open** | 0 | Blast-radius measurement and `rule_for_explanation` join land before F1's population commit. |
| F2 | Class completion: gate arm, prestige-alone diagnostic | F2.1–F2.3 in epic-breakdown.md | **open** | 0 | Falsifiable acceptance: census `computed == 42` of 135 both before and after the gate-arm change. |
| F3 | Class completion: multiclass fold for every class with a chassis | F3.0–F3.4 in epic-breakdown.md | **open** | 0 | Least-certain sizing in the plan (17-26 agent-hours); the F0 mix-panel histogram must exist before code starts. |
| F4 | Class completion: desktop creation/level-up roster | F4.1–F4.4 in epic-breakdown.md | **open** | 0 | Roster rule: offered iff census Computed at every level; prestige level-up only; Ex-* census-only. |
| F5 | Class completion: closure deltas (docs, baselines, forward-scope register) | F5.1–F5.3 in epic-breakdown.md | **open** | 0 | Gates D2–D6 (see row below) — Epic F must close before graphify runs against the final tree. |
| D2–D6 | Closure: retrospective, release notes, graphify, PR, worktree sweep | D2–D6 in epic-breakdown.md | **blocked — now after Epic F** (D2–D4 were done against the pre-Epic-F tree; D5/D6 held) | 1 (paused) | D4's 2026-09-20 graphify run (exit=1, dedup-refusal guard, receipt in `receipts.md`) was against the pre-Epic-F tree and is superseded — graphify re-runs LAST against the tree Epic F leaves, per the standing "graphify runs against the FINAL repo state" rule. D5 (PR merge) and D6 (worktree sweep) do not proceed until F5 closes. |

---


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
| D2–D6 | Closure: retrospective, release notes, graphify, PR, worktree sweep | D2–D6 in epic-breakdown.md | in progress (D2–D4 done this pass; D5 PR open — operator merges; D6 read-only inventory recorded, sweep itself pending operator action) | 1 (in progress) | D4 graphify run 2026-09-20 against final tree — exit=1 (dedup-refusal guard), receipt filed in `receipts.md` per script's non-refusal policy. D6 `git worktree list`/`git branch` captured in `receipts.md` as POST-MERGE cleanup list for the operator; nothing deleted. D5: PR to be opened this cycle; operator merges. |

---


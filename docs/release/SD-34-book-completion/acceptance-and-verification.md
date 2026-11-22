---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-26
---

# SD-34 Acceptance and Verification

The closure gates, the command that proves each criterion, and the artifact each produces.
**A criterion is met by its command's output, never by a lane's account of it.**

## 1. Per-criterion verification map

| Criterion | Verifying command | Artifact |
|---|---|---|
| AT-34-E1-001 | `python3 scripts/completion_atlas.py --check` -> `population=49438 buckets=10 unclassified=0 overlap=0`, exit 0 | `artifacts/epic-1-atlas/completion-atlas.json` |
| AT-34-E1-002 | six RED->GREEN mutation transcripts, one per fail-closed condition (condition 6 = the `file:line` citation) | `artifacts/epic-1-atlas/fail-closed-proofs.md` |
| AT-34-E1-003 | per kind: units, core-rulebook slice, engine surface, exercised-by-core flag | `artifacts/epic-1-atlas/missing-engine-tables.json` |
| AT-34-E1-004 | the counts re-derived at HEAD + the promotion ladder quoted with its verified line number | `artifacts/epic-1-atlas/shape-engine-boundary.md` |
| AT-34-E1-005 | old-string count sweep across `tests/`, `src/`, `apps/`, `scripts/` returning 0 live uses | the rename diff + its RED->GREEN |
| AT-34-E1-006 | `scripts/verify.sh --only figure-provenance` exits 0; RED->GREEN on an unsourced figure AND on a wrong-command figure; `scripts/verify.sh --only denominator-gate` default run lists every SD-34 `.md` in `files_checked` | the stage, wired into `verify.sh`; the widened `denominator_gate.py` default scope |
| AT-34-E1-007 | `scripts/verify.sh --only corpus-trap-audit` exits 0 with its population printed; RED->GREEN on a planted trap; the stage bounds its own runtime | the stage, wired into `verify.sh` |
| AT-34-E1-008 | `scripts/verify.sh --only corpus-trap-audit` reports `wiring-class-mismatch=0`, other four trap kinds reported not absorbed; `corpus_literal_sweep` 0 findings with its examined-population moved by exactly the record delta | `artifacts/epic-1-atlas/wiring-class-remediation.json` |
| AT-34-E2-001 | per kind: the table holding a named record, OR the counts proving no table is needed | `artifacts/epic-2-tables/` build transcripts |
| AT-34-E2-002 | per table: a refusal transcript AND a success transcript | `artifacts/epic-2-tables/fail-closed-proofs.md` |
| AT-34-E2-003 | per table: wall time, lines changed, what dominated, and the spread across the eight | `artifacts/epic-2-tables/table-build-rate.json` |
| AT-34-E2-004 | `completion_atlas.py --book core_rulebook --check` AND `--book ultimate_campaign --check` -> bucket A at 0 | atlas output in the receipt |
| AT-34-E3-001 | atlas reports bucket B at 0 for `core_rulebook`; mechanisms named, not records | receipt + atlas |
| AT-34-E3-002 | atlas reports bucket C at 0; per unit, the display/explanation path carrying it | receipt + atlas |
| AT-34-E3-003 | atlas reports M, V, D, U, X at 0 for `core_rulebook`, movement in four buckets | receipt + atlas |
| AT-34-E3-004 | per bucket: units cleared, wall time, what dominated — **measured, not estimated** | `artifacts/epic-3-core-rulebook/step-cost-ledger.json` |
| AT-34-E3-005 | `python3 scripts/completion_atlas.py --book core_rulebook --check` -> `DONE=6701 of 6701`, exit 0 | `artifacts/epic-3-core-rulebook/core-rulebook-completion-manifest.json` |
| AT-34-E3-006 | the file exists; each entry carries its `correction` event and the atlas re-derivation | `artifacts/epic-3-core-rulebook/atlas-defects.md` |
| AT-34-E4-001 | per `U` unit: the instrument correction, or a proven statement that no verdict is possible; each `X` resolved | `artifacts/epic-4-ultimate-campaign/` receipts |
| AT-34-E4-002 | `completion_atlas.py --book ultimate_campaign --check` -> `DONE=265 of 265`, exit 0 | `artifacts/epic-4-ultimate-campaign/ultimate-campaign-completion-manifest.json` |
| AT-34-E4-003 | the second cost ledger + a stated comparison against Epic 3's rates | `artifacts/epic-4-ultimate-campaign/step-cost-ledger.json` |
| AT-34-E5-001 | per book, per bucket: units, mechanism, projected cost, **rate + sample size used** | `artifacts/epic-5-forward-plan/forward-plan.json` |
| AT-34-E5-002 | per capability: what, which buckets/books it unblocks, population, built-by-SD-34 flag | `artifacts/epic-5-forward-plan/capability-register.json` |
| AT-34-E5-003 | `power`'s projected cost, the rate it derives from, and what `ultimate_psionics` still needs after it | the forward plan |
| AT-34-E5-004 | the plan sorted by projected cost, ordering basis stated, single-bucket books flagged by name | the forward plan |
| AT-34-E6-001 | the full scan (§3 below) | `artifacts/epic-6-closure/AT-34-E6-001_cycle_receipt.md` |
| AT-34-E6-002 | retrospective written **and cited from `references/README.md` in the same cycle** | `docs/retro/sd34-book-completion-retrospective.md` |
| AT-34-E6-003 | sweep counts found vs removed; arch docs; graphify; PR; release notes | `receipts.md`, `release-notes.md` |

## 2. Standing gates — green at every cycle, not just at closure

```bash
python3 scripts/completion_atlas.py --check        # SD-34's own; exit 0, unclassified=0
python3 scripts/box_ledger.py --check              # inherited from SD-33; exit 0
python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'   # violations=0 — explicit path until AT-34-E1-006 widens the default (decisions.md §3)
scripts/verify.sh --only denominator-gate          # exit 0 — examines SD-34 only AFTER AT-34-E1-006
scripts/verify.sh --only figure-provenance         # exit 0 — exists only FROM AT-34-E1-006 onward; not a gate before that cycle
scripts/verify.sh --only corpus-trap-audit         # exit 0 — exists only FROM AT-34-E1-007 onward
cargo run --locked --bin corpus_literal_sweep      # 0 findings, exit 0
cargo test --locked --no-run                       # exit 0 — does everything COMPILE
cargo test --locked --lib
cd apps/desktop/src-tauri && cargo test --locked   # SEPARATE cargo workspace
```

**Do not narrow any gate's scope to make a cycle pass.** A gate weakened to pass is worse than
the red it replaced. Every gate prints the population it examined; a PASS without a stated
population is a vacuous pass.

## 3. The final-acceptance scan (AT-34-E6-001)

The scan checks **work**, never reports. Its obligations, each learned from a real SD-33
failure:

1. **Count rows and derive SETS, not sizes.** Subtract id-sets. A count can match while
   membership does not.
2. **Re-run every headline command yourself.** Subagent recaps quote stale figures.
3. **Read commit diffs.** Distinguish a real fix from an edited expectation.
4. **Verify any method change re-ran what it already judged**, coverage stated as rows-re-run
   of rows-in-affected-set, both with denominators.
5. **Re-derive failure attribution from `git`** against the branch-cut SHA. A lane's claim
   that a failure is pre-existing is a claim, not evidence.
6. **Grep the closure instruments for hardcoded exclusion lists.** Carve-outs hide in code.
7. **Verify at the widest build scope**, counting targets executed — not just the exit code.
8. **Read `## Open blockers`** — real heading, bounded at the next `## `, `<details>` archives
   ignored. Any active entry BLOCKS.
9. **Enumerate open deferrals.** None may defer DoD scope; all carry a revisit condition.
10. **Re-prove the gates still fail.** Plant a genuine violation, confirm the catch, remove the
    probe, confirm the baseline returns to zero. Leave no residue.
11. **Check every corpus change moved the sweep's population by exactly the records added**
    (`decisions.md §12` L8). Read the sweep-population row of every receipt that touched
    `data/corpus/`.
12. **Check every build-scope row names the SHA it ran at**, and that no later commit in the
    same cycle regenerated an inventory or fixture the asserted figures depend on
    (`decisions.md §12` L7).
13. **Read `forward-scope-register.md §E1` before the sweep.** The three branches there are
    deleted on SD-33's ruling, not re-diagnosed (`decisions.md §12` L6).

### 3a. Deliverable-integrity checks — specific to this bundle

SD-34's product is a map. These verify the map itself, and are as blocking as anything above:

- **`completion_atlas.py --check` re-run at HEAD returns `unclassified=0`.** An atlas that
  only balanced at authoring time is not a deliverable.
- **Both completion manifests' evidence pointers resolve**, on independently drawn samples.
  A unit "done" whose pointer does not resolve is a blocking shortfall.
- **`atlas-defects.md` exists.** Empty is excellent; absent is a failure. Each entry it does
  carry must have produced a `correction` event and an atlas re-derivation.
- **Every forward-plan projection names its measured rate and sample size.** A projection
  built on a rate measured from a thin sample must say so in the row.
- **All nine tables are accounted for** — eight built, `power` costed with the rate it used.
- **Bucket `U` has its sub-causes enumerated** with the probe (or absence of one) behind each,
  per SD-33's register C1.1. Same rule as `D`.
- **Bucket `D` has its sub-causes enumerated.** A holding pen without a census is a defect.

**If anything is short: STOP.** No retrospective, no sweep, **no PR**. Report what is short
with the command that shows it. That is a correct outcome — SD-33's scan halted nine times and
every halt was right.

**Do not manufacture a shortfall either.** If the work is genuinely done, PASS it.

## 4. Closure gate sequence

Order is load-bearing (`workflow-instruction.md §11`):

1. Final-acceptance scan PASSES
2. Retrospective written **and cited** in the same cycle
3. Full worktree/branch sweep, counts found vs removed
4. Architecture docs -> graphify -> PR -> merge-conflict resolution (`../template/template.md §6`)
5. Release notes + version bump

Steps 2 and 3 land **before** step 4 opens the PR.

**The operator merges `tranche/14` -> `develop`.** No dispatched agent merges.

## 5. What does NOT satisfy a criterion

- A lane's `status: complete` unsupported by a row count on its own artifact
- A percentage without its denominator in the same construct
- A filed `## Open blockers` entry
- An atlas with a non-zero `unclassified`, or a bucket without a named clearing mechanism
- A remaining step found in Epic 3 that the atlas missed and that was absorbed rather than
  recorded as an atlas defect
- A forward-plan projection with no stated rate or sample size
- A gate passing because its population was narrowed
- A count that dropped because measurement changed, reported as closure

---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
---

# SD-35 Technical Requirements

Pre-launch prerequisites and normative requirements. **§1 is unrun** — SD-35 is `planning`,
not `planning-ready`, until SD-34 closes and the `tranche/15` cut runs `workflow-instruction.md §1`.

## 1. Launch prerequisites

| # | Prerequisite | Tier |
|---|---|---|
| 1 | SD-34's closure PR **merged** to `develop`; SD-34's retrospective written and cited; its `## Open blockers` empty | **Tier 1 — blocking** |
| 2 | `tranche/15` cut from `develop` and pushed | **Tier 1 — blocking** |
| 3 | `0.15.0` stamped in `apps/desktop/package.json` and `apps/desktop/src-tauri/tauri.conf.json` | Tier 1 |
| 4 | `./kanban.md` present and readable | Tier 2 |
| 5 | Working tree clean on the bundle branch | Tier 2 |
| 6 | Oracle pin readable; repo-local slot used, **never** `~/workspace/repos/pcgen` | Tier 1 |
| 7 | Inherited gates green (§2), **including the two citation `--check`s that were stale before SD-34 wave 51** | Tier 1 |
| 8 | Widest-build-scope baseline re-derived at the cut (§3), **and cold build time recorded** | Tier 1 |
| 9 | Artifact directories exist, one per epic (`.gitkeep` in each) — done at authoring | Tier 2 |
| 10 | Denominator gate run against **this** package's glob, `violations=0` | Tier 1 |
| 11 | Every figure in this package re-measured at the cut (`content-unit-inventory.md §0`) | Tier 1 |

## 2. Inherited gates that must be green at launch

```bash
python3 scripts/completion_atlas.py --check
grep -rlE 'PcgenFormulaEvaluator|render_pcgen_desc|raw_tokens|bonus_stack_reader|pre_tokens' src/rules_core src/saved_character src/campaign src/homebrew_authoring apps/desktop | wc -l   # the coarse residue count at the cut; AT-35-E1-005 records the exact baseline
python3 scripts/shape_engine_boundary.py --check
python3 scripts/missing_engine_tables.py --check
python3 -m unittest scripts/tests/test_shape_engine_boundary.py
python3 scripts/box_ledger.py --check
python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md'
scripts/verify.sh --only denominator-gate
scripts/verify.sh --only figure-provenance
cargo run --locked --bin corpus_literal_sweep
ls scripts/oracle_harness/
grep -n 'len(open_deferrals)' scripts/retro.py
```

A red inherited gate at launch is a launch-gate failure, not an SD-35 defect to absorb. Report
it and stop.

## 3. Inherited test baseline, and the build-time baseline

SD-34 inherited **29 of 599** workspace suites carrying **46 of 8,034** failures, proven
pre-existing at the `tranche/13` cut. The fable review's post-fix sweep (2026-09-01) recorded
**14 of 40** `verify.sh` stages red at that time, all attributed to SD-34's own in-flight
instrument changes; SD-34's wave-end gates since report 40 of 40 green. **Re-derive the
failing-suite set at the `tranche/15` cut and record it as SD-35's baseline.** A failure
outside the recorded baseline is SD-35's, proven against the cut SHA with `git`.

```bash
cargo test --locked --no-run ; echo EXIT=$?                       # must be 0
/usr/bin/time -v cargo test --locked --no-run -j 6 2>&1 | grep -E 'Elapsed|Maximum resident'   # COLD, CARGO_INCREMENTAL=0, fresh CARGO_TARGET_DIR — AT-35-E1-003's "before"
cargo test --locked --no-fail-fast -j 6                            # record failing targets + counts + targets executed
cd apps/desktop/src-tauri && cargo test --locked
```

## 4. Normative requirements

**N1 — DONE is a rendered sheet line** (`decisions.md §1`): a final number, a dice expression,
or the rule's words. Computing every resolvable term is required; per-unit proof machinery is
forbidden.

**N2 — A cycle is one mechanism, corpus-wide, 500 units minimum** (`decisions.md §2`).
`scripts/cycle_scope_gate.py --min 500` passes before the cycle starts; its line is in the
receipt.

**N3 — One build per cycle; the full gate once per epic** (`decisions.md §3`). `builds_recorded`
in the receipt rows is 1.

**N4 — Lines per unit is reported** (`decisions.md §4`), and every cycle above 3.0 is named at
the epic wrap-up.

**N5 — Every figure states its denominator** in the same construct (`decisions.md §8`).

**N6 — A lane's status is a mechanical function of its receipt rows**, never a judgment about
effort.

**N7 — Measure before a population-scoped run.** AT-35-E2-005's timed corpus-wide pass is the
standing measurement; any cycle re-running the pass states projected wall time first.

**N8 — `data/corpus/**` is never hand-edited.** Guarded generator path only; never
`--allow-stamp-loss`; `corpus_literal_sweep` after; examined-count moves by exactly the record
delta.

**N9 — Verify at the widest build scope** the cycle touched; the desktop crate and frontend at
epic cadence unless touched.

**N10 — A blocker is cleared or escalated, never deferred.** "The engine cannot model X" is not
a blocker under N1.

**N11 — No stubs.** No stub, inline mock, placeholder, or `"Would …"` string in shipping code
(`../../governance/no-stub-mvp-doctrine.md`).

**N12 — A new status is wired into every consumer in the same cycle**, with the grep census
before and after (`decisions.md §9` L5).

**N13 — The remainder is named by token type**, and the token sums are checked
(`scripts/token_coverage.py --check`).

**N14 — Citation pins are content anchors**, and every `--check` instrument is a `verify.sh`
stage (`decisions.md §9` L3, L4).

**N15 — A field's name is not its meaning.** Read the code that writes a status before quoting
it.

**N16 — Recursive search, always.** State the search used.

**N17 — At most 3 concurrent building lanes, `-j 6` each** (`decisions.md §9` L11).

**N18 — No PCGen on the live side** (`decisions.md §11`). The converter (`src/pcgen_import/**`,
the `src/bin` generators) and the test oracle (`scripts/oracle_harness/`, `src/oracle_validation/`)
are the only readers of PCGen tokens, formulas, or variable names. `scripts/pcgen_residue_gate.py
--check` runs at the start and end of every cycle and never rises; `--closure` reads zero from
AT-35-E6-004 on. `data/sheet_rules/` carries none of the source format. A cycle that needs a
PCGen fact on the live side has a converter gap, not a live-side task.

## 5. Environment

Per dispatched agent:

```bash
export RETRO_ACTOR="<lane-role-name>"
export CARGO_TARGET_DIR="/tmp/cargo-sd35-<lane-role-name>"
export CARGO_INCREMENTAL=0
mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
```

**Disk:** after every parallel wave, `df -h /` and `git worktree list`. Prune merged worktrees
proactively. Never remove a `locked` worktree or one carrying unmerged commits.

## 6. Git

```bash
git fetch origin tranche/15 && git rebase origin/tranche/15 && git push origin HEAD:tranche/15
```

Retry up to 5 times on non-fast-forward. **Never force-push.** `git status --porcelain` before
**every** git write. **Never `git add -A`. Never `git stash`.**

The operator merges `tranche/15` → `develop`. No dispatched agent merges.

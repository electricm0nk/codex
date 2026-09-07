---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-26
---

# SD-34 Technical Requirements

Pre-launch prerequisites and normative requirements. **§1 is unrun** — SD-34 is
planning-ready, not launch-ready.

## 1. Launch prerequisites

Every item is a command, run for real with its output pasted into
`workflow-instruction.md §1` before launch. A prerequisite written from memory is not
verified.

| # | Prerequisite | Tier |
|---|---|---|
| 1 | SD-33's closure PR **merged** to `develop` — **satisfied** 2026-08-27, #377 → `ea2b3396f2`; re-confirm after `git fetch origin` | **Tier 1 — blocking** |
| 2 | `tranche/14` cut from `develop` and pushed | **Tier 1 — blocking** |
| 3 | `0.14.0` stamped in `apps/desktop/package.json` and `apps/desktop/src-tauri/tauri.conf.json` | Tier 1 |
| 4 | `./kanban.md` present and readable | Tier 2 |
| 5 | Working tree clean on the bundle branch | Tier 2 |
| 6 | Oracle pin readable; repo-local slot used, **never** `~/workspace/repos/pcgen` | Tier 1 |
| 7 | Inherited gates green (§2) | Tier 1 |
| 8 | Widest-build-scope baseline re-derived at the cut (§3) | Tier 1 |
| 9 | Artifact directories exist, one per epic (`.gitkeep` in each) | Tier 2 |
| 10 | Denominator gate run against **this** package's glob, `violations=0` | Tier 1 |

## 2. Inherited gates that must be green at launch

SD-34 depends on these and does not rebuild them:

```bash
python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'  # violations=0 (explicit path — default scope is SD-33's, decisions.md §3)
scripts/verify.sh --only denominator-gate      # exit 0
python3 scripts/box_ledger.py --check          # exit 0
cargo run --locked --bin corpus_literal_sweep  # 0 findings, exit 0
ls scripts/oracle_harness/                     # run.py, compare.py, oracle_export.py
grep -n 'len(open_deferrals)' scripts/retro.py # SD-32's fix present
```

A red inherited gate at launch is a launch-gate failure, not an SD-34 defect to absorb.
Report it and stop.

## 3. Inherited test baseline

SD-33 closed with **29 of 599** workspace suites carrying **46 of 8,034** failures, each
**proven pre-existing** at the `tranche/13` cut (`0` commits since the cut for all 29) and
registered forward. (31 / 49 of 8,026 through SD-33's attempt 10; the operator's fold fixed two
outright and added 8 executed cases — `docs/retro/sd33-computed-value-verification-retrospective.md §5`.)

**Re-derive that set at the `tranche/14` cut and record it as SD-34's baseline.** A failure
outside the recorded baseline is SD-34's, and "pre-existing" must be proven against the cut
SHA with `git`, never asserted (`decisions.md §10`).

```bash
cargo test --locked --no-run ; echo EXIT=$?     # must be 0
cargo test --locked --no-fail-fast              # record failing targets + counts
cd apps/desktop/src-tauri && cargo test --locked
```

## 4. Normative requirements

**N1 — Every unit lands in exactly one atlas bucket.** `decisions.md §2`. `unclassified` is a
hard error, not a residual category. A bucket must name the mechanism that clears it.

**N2 — Every figure states its denominator in the same construct.** Enforced by
`scripts/denominator_gate.py`, run with this package's explicit glob until AT-34-E1-006 widens
its default scope (`decisions.md §3`). It scans package markdown, not only receipts.

**N3 — A lane's status is a mechanical function of its artifact.** Every lane runs the count
on its own output and reports the literal command output. The scan derives the **set**.

**N4 — Every gate prints the population it examined.** A PASS without a stated population can
be vacuous. Enforced by AT-34-E1-006's `figure-provenance` stage from that cycle onward
(`risks-and-open-questions.md §10`); and a gate's population must **grow** by exactly the
records a corpus change added (`decisions.md §12` L8).

**N5 — `data/corpus/**` is never hand-edited.** Guarded generator path only. Never
`--allow-stamp-loss`. `corpus_literal_sweep` after every regeneration; license/PI and
`raw_tokens` verified surviving, per record.

**N6 — Recursive search, always.** A shallow glob under-reports in this repo by orders of
magnitude. State the search used.

**N7 — Measure before a population-scoped run.** Measured per-unit cost, population, and
projected wall time, stated **before** the full run starts. A method proven at n=1 is not
proven at n=8,330.

**N8 — A method change re-runs everything it already judged**, with coverage stated as
rows-re-run of rows-in-affected-set, both with denominators.

**N9 — Verify at the widest build scope.** `--no-run`, the full workspace run with targets
executed counted, and `apps/desktop/src-tauri` explicitly.

**N10 — A blocker is cleared or escalated, never deferred.** A fix that lives in another
subsystem is still a fix.

**N11 — No stubs.** No stub, inline mock, placeholder, or `"Would ..."` string in shipping
code. Code paths that ship do what they claim
(`../../governance/no-stub-mvp-doctrine.md`).

**N12 — A remaining step the atlas did not predict is an atlas defect.** It is logged as a
`correction`, the atlas is re-derived, and it is recorded in `atlas-defects.md`. Absorbing it
silently is the failure this bundle exists to end.

**N13 — A field's name is not its meaning.** Read the code that writes a status field before
quoting it. `not-ingested` cost this package a wrong headline reported to the operator.

## 5. Environment

Per dispatched agent:

```bash
export RETRO_ACTOR="<lane-role-name>"
export CARGO_TARGET_DIR="/tmp/cargo-sd34-<lane-role-name>"
export CARGO_INCREMENTAL=0
mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
```

`CARGO_INCREMENTAL=0` is not optional — a stale incremental cache served an SD-32 lane a
stale binary inside its own private target directory. The `.reclaim-claim` file is what
stops a sibling's reclaim sweep from deleting a live agent's target dir.

**Disk:** after every `parallel: yes` wave, `df -h /` and `git worktree list`. Prune merged
worktrees proactively. Never remove a `locked` worktree or one carrying unmerged commits.

## 6. Git

```bash
git fetch origin tranche/14 && git rebase origin/tranche/14 && git push origin HEAD:tranche/14
```

Retry up to 5 times on non-fast-forward. **Never force-push.** `git status --porcelain`
before **every** git write. **Never `git add -A`. Never `git stash`** — the bare form
stashes the whole shared checkout even from a subdirectory.

The operator merges `tranche/14` -> `develop`. No dispatched agent merges.

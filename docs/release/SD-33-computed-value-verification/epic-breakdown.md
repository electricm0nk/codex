---
canonical: true
owner: god-emporer
bundle_id: SD-33
date: 2026-08-24
---

# SD-33 Epic Breakdown — 6 epics, 21 criteria

Criterion IDs follow the program convention `AT-33-E<epic>-<nnn>`. Every criterion states its **evidence obligation** — the command or artifact that proves it, not a description of the work.

**Standing rule for every criterion in this document** (`decisions.md §2`): any percentage in any receipt states its denominator in the same construct. A criterion reported as a bare percentage is not met.

---

## Epic 1 — Instruments

**Gated on:** launch gates (`README.md §9`). **Gates:** every other epic.
**Why first:** SD-32's failure was not missing lessons but missing mechanisms (`decisions.md §4`). Epic 1 builds the mechanisms before any measurement leans on them.

### AT-33-E1-001 — `THE-BOX.md` exists as a living partition of the full inventory

The document partitions **all 49,438 inventory units** — not the not-done subset — into named groups, each carrying a count, a disposition, and a re-derive command. `uncovered == 0` and `overlap == 0`.

**Evidence:** `python3 scripts/box_ledger.py --check` exits 0 and prints `uncovered=0 overlap=0 population=49438`. A committed `THE-BOX.md` whose group counts sum to the stated whole.

### AT-33-E1-002 — `box_ledger.py` fails closed on all five conditions

The tool exits non-zero on: `uncovered != 0`; `overlap != 0`; oracle disagreement; an `unverifiable` unit dispositioned `done`; and a `derived_at` SHA that is not an ancestor of `HEAD` (**staleness gate**).

**Evidence:** five RED→GREEN mutation proofs, one per condition, in the cycle receipt. A tool that has never been observed to fail is not a gate.

### AT-33-E1-003 — the probe surface is enumerated for real

A committed enumeration of **every corpus kind**, stating for each whether a probe exists that can verify a computed magnitude, and naming the probe. **Derived by execution, not from memory or from prior prose** (`decisions.md §7`).

**Evidence:** `artifacts/epic-1-instruments/probe-surface-census.json` plus the command that generated it. The count of kinds with **no** probe is a bundle-level figure reported in `progress.md`, not a footnote.

### AT-33-E1-004 — the denominator gate is a real `scripts/verify.sh` stage

`scripts/verify.sh --only denominator-gate` runs, and **fails** on a percentage stated without its denominator in the same construct.

**Evidence:** RED→GREEN mutation proof — a deliberately-malformed receipt fails the stage; the corrected form passes. Wired into `verify.sh`'s stage list, not a standalone script (the gap `SD-31-.../forward-scope-register.md` C1.8 left open for `v06_corpus_trap_report`).

---

## Epic 2 — Oracle harness

**Gated on:** Epic 1. **Parallel-safe with:** Epics 3, 4.
**Timeboxed** per `decisions.md §5`. Its deliverable is a **ruling**, not an unbounded effort.

### AT-33-E2-001 — Path A feasibility is established by execution

The pinned PCGen (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`) either builds headless on this box or does not, **proven by running the build**, with the failure mode named if it fails.

**Evidence:** build transcript in `artifacts/epic-2-oracle-harness/`. The three named risks (`decisions.md §5`) are each resolved to a fact: Gradle-vs-Java-25, `pcgen.gui2.UIPropertyContext` coupling, `.pcg` input authoring.

### AT-33-E2-002 — a character round-trips through the oracle

One authored `.pcg` input exports through `BatchExporter` via a template that emits computed variables, and the output is machine-readable.

**Evidence:** the `.pcg`, the template, the emitted output, and the command — all committed. **Path A is not "established" until a real value comes out.**

### AT-33-E2-003 — the comparison harness answers the per-unit question

Given a unit, the harness returns `(ours, oracle, agree|disagree|unverifiable)`. `unverifiable` is a first-class return, never an error swallowed into `agree`.

**Evidence:** a committed fixture set exercising all three outcomes, including at least one **known-disagreeing** case. A harness that has never returned `disagree` has not been shown to be capable of it.

**Fixture discipline** (`stc-authoring`): a fixture's expected value is transcribed from bytes the harness's own read path does **not** touch. A fixture built from the file the harness reads is a mirror, not a check.

### AT-33-E2-004 — the Path A / Path B ruling is recorded and escalated

Epic 2's closing receipt states Path A or Path B explicitly. **If Path B, the consequence for Epic 5's throughput is escalated to the operator as a decision point** (`decisions.md §5`) — never absorbed silently, never allowed to reduce the bundle to "coverage only" by drift.

**Evidence:** the receipt, plus a `progress.md` entry naming the ruling and, if Path B, the escalation.

---

## Epic 3 — Engine-coverage closure

**Gated on:** Epic 1. **Parallel-safe with:** Epics 2, 4.
**Population:** the **6,854** formula-bearing units never run through an engine (E − F, `README.md §4`).

### AT-33-E3-001 — the cause of the 41% is diagnosed before anything is run

**41% coverage is a symptom.** A committed root-cause statement explains *why* 6,854 units were never run — per family, since the gap is uneven (F1 28%, F8 21%, F2 64%).

**Evidence:** `artifacts/epic-3-engine-coverage/coverage-gap-rootcause.md`, with the coordinates of sampled units traced concretely. **Generic pass, not per-object lanes** — the analysis is by mechanism.

### AT-33-E3-002 — F1's gap closes

F1 is the largest absolute gap (6,308 formula-bearing, 1,790 run). Its uncovered units run.

**Evidence:** a fresh corpus-wide run reporting F1 population == F1 formula-bearing count, both figures stated.

### AT-33-E3-003 — every remaining family closes to 100%

F2–F9 reach full population coverage.

**Evidence:** per-family table in the receipt, each row stating run-population and true-population.

### AT-33-E3-004 — the corpus-wide run reports 100% with its denominator

The regenerated `formula_interpreter.corpus-wide.json` covers **11,652 of 11,652**, and the receipt states both numbers.

**Output path (must, not the binary's default):** `cargo run --locked --bin formula_interpreter -- --corpus-wide --output docs/release/SD-33-computed-value-verification/artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json` (`--output` flag confirmed in `src/bin/formula_interpreter.rs`'s argument parser and `--help` text). The binary's default output, `artifacts/gate-2-engines/formula_interpreter.corpus-wide.json`, is SD-32's closed Gate 2 evidence file — **never overwrite it.**

**Evidence:** the artifact at the SD-33 path above, plus the comparison command from `README.md §4` row G returning `0`.

**Note:** recognition rate is a *separate* number from coverage. A refused unit is a named finding with its refusal reason, not a coverage failure — and not a silent exclusion either.

---

## Epic 4 — Unknown-status classification

**Gated on:** Epic 1. **Parallel-safe with:** Epics 2, 3.
**Population:** the **4,224** units at `status: unknown`, verdict `unmeasurable`.

### AT-33-E4-001 — the cause of `unknown` is established before reclassification

Whether these units are genuinely unmeasured, or measured by an instrument that could not express the result, is **answered before any count moves** (`instrument-correction-is-not-closure`: a count that drops because measurement changed is not closure).

**Evidence:** `artifacts/epic-4-unknown-classification/unknown-rootcause.md`.

### AT-33-E4-002 — the 4,224 reach zero

No inventory unit carries `status: unknown`.

**Evidence:** `jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json` → `0`. Movement is reported in **four buckets** — closure / reclassification / reachability / **instrument correction** — never as a single number.

### AT-33-E4-003 — nothing lands in a bucket meaning "we did not look"

Every reclassified unit carries a disposition that is a statement about the unit, not about our effort.

**Evidence:** `box_ledger.py --check` passes with the reclassified population; no group named for an absence of work.

---

## Epic 5 — Re-verification of the blessed

**Gated on:** Epic 2 (needs the harness). **Population:** the **8,330** units blessed by fixture (1,741) or literal check (6,589), never by the oracle.

### AT-33-E5-001 — the 1,741 `fixture-verified` units are re-examined against the oracle

**Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement counts both stated, with the denominator.

### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined

**Evidence:** as above.

### AT-33-E5-003 — every disagreement is a named defect, fixed or escalated

A disagreement is **never** closed by adjusting the expectation to match our output. Each is root-caused: either our computation is wrong (fix it) or the oracle comparison is wrong (fix the harness, and re-run everything it already judged).

**Evidence:** one entry per disagreement in `progress.md`, each resolved to a commit or an operator escalation. **A filed blocker does not satisfy this criterion** (`../../governance/blocker-closure-doctrine.md`).

---

## Epic 6 — Closure epilogue

**Gated on:** Epics 1–5 all `complete`. Fires **once**.

### AT-33-E6-001 — final-acceptance scan

Every criterion `AT-33-E1-001` … `AT-33-E5-003` is `complete`, and every `kanban.md` card is `complete`. **A card at `returned-to-backlog`, `in-progress`, or `complete`-with-a-deferred-half blocks closure.**

**If anything is short, the cycle stops here** — no retrospective, no sweep, **no PR**. Report what is short with the command that shows it. That is a correct outcome, not a failure.

### AT-33-E6-002 — retrospective written and cited

`docs/retro/sd33-computed-value-verification-retrospective.md`, grounded in `scripts/retro.py summary --since <launch> --json`, **and cited from `references/README.md` in the same cycle**.

**Binding correction from SD-32** (`decisions.md §2`): `retro.py`'s `deferrals.open` field is `deferrals[-limit:]`, not open deferrals. **Do not quote it as a closure figure.** If SD-32's fix has landed, use the corrected field and say so; if not, enumerate deferrals directly and state the total.

### AT-33-E6-003 — sweep, architecture docs, graphify, PR, release notes

Full worktree/branch sweep with counts found vs removed; architecture-docs refresh and graphify per `../template/template.md §6`; PR; release notes and version bump.

**Order is load-bearing:** retrospective and sweep happen **before** the PR opens.

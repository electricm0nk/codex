---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-26
---

# SD-34 Risks and Open Questions

## 1. Self-healable vs non-self-healable

**Self-healable** (resolve inline, continue): dirty tree; a single-token audit violation;
unrelated test-setup breakage; build-counter out of sync.

**Non-self-healable** (write `## Open blockers` in `progress.md`, stop): working tree diverged
needing manual rebase; two live cycles on conflicting files; a launch gate not actually met;
RED->GREEN not preserved in the receipt; a stub, inline mock, or `"Would ..."` string in
shipping code.

**A `## Open blockers` entry pauses the bundle** and is a request for an operator ruling —
never a disposition, never a closure path (`decisions.md §6`).

## 2. R1 — The atlas could be wrong, and that is the deliverable

**The bundle's central risk.** SD-34's product is a claim of completeness. A wrong atlas is
worse than no atlas, because it would be believed.

Three ways it fails:
- a unit lands in the wrong bucket (mis-clears, or blocks work that isn't needed)
- a bucket's "clearing mechanism" turns out not to clear it
- a **category** exists that the bucket list does not name — the operator's "three more
  things" in its purest form

**Mitigation.** Epic 3 is the test: completing the Core Rulebook exercises every bucket except
`Z`, so a wrong mechanism surfaces as real friction rather than a theory. AT-34-E3-006 makes
any unpredicted category a recorded **atlas defect** with a `correction` event and a forced
re-derivation, and AT-34-E6-001 re-runs the atlas at HEAD.

**Residual risk that remains after all of that:** buckets exercised only by the Core Rulebook
may behave differently in books with different content shapes. Epic 4's projections therefore
name their sample size, and a rate measured on one book is stated as such.

## 3. R2 — Nine tables is the largest unknown in the bundle

Bucket A is 8,463 units across 9 kinds with **no engine table at all** — `ability` (4,337),
`template` (2,248), `trait` (487), `deity` (459), `power` (421), `domain` (183), `skill`
(149), `language` (136), `companion` (43).

Building an engine table is not the same shape of work as placing a record in one. Nobody has
measured it in this program.

**Mitigation.** Epic 2 builds **eight** of the nine — the seven the Core Rulebook exercises
plus `trait` from Ultimate Campaign — which produces a **measured** build rate, and its spread
across eight kinds, before anything is projected. Only `power` is projected rather than
measured. AT-34-E2-001 also permits the
honest outcome that a kind needs no magnitude table at all — but requires it be *proven* by
counts, never assumed to save work.

**If a table turns out to be far larger than the others**, that is a finding for the forward
plan, not a reason to stop — and it is exactly why AT-34-E2-003 requires the spread across the
eight, not a single blended average. `power` is costed against that spread (AT-34-E5-003).

## 4. R3 — Bucket V is inherited, and large

**8,330 units** are `V` — verified by proxy, never by the oracle. That is exactly SD-33's own
population, and **2,582 of them are in the Core Rulebook** (38.5% of that book, its single
largest bucket).

SD-33 established that running these through the oracle is slow: its Epic 5 took multiple
remediation waves to reach 8,330 of 8,330.

**Mitigation.** The harness exists and its throughput lessons are recorded — amortise the JVM,
one character carrying many computed variables, measure per-unit cost before the full run
(`workflow-instruction.md §2.5`). Epic 3 clears only the Core Rulebook's 2,582; Ultimate Campaign has **no `V` units at all**,
which is part of why it is a useful second measurement. The rest is priced, not run.

**Honest note:** this is the bucket most likely to dominate Epic 3's wall time.

## 5. R4 — Renaming `not-ingested` touches every consumer

AT-34-E1-005 renames a status field that appears in `docs/work-inventory.json`, the work
inventory binary, the dashboard producer, ledgers, and any test asserting on it.

**Mitigation.** RED->GREEN plus a count sweep for the old string across `tests/`, `src/`,
`apps/`, `scripts/`. A string-valued rename compiles clean while leaving assertions red — the
same shape as a record-count change.

**Why it is worth the risk:** the name asserts the opposite of the fact and has already caused
a wrong headline to be reported to the operator. Leaving it costs more than changing it.

## 6. R5 — Corpus regeneration during Epics 2 and 3

Three hazards, all previously observed here: license/PI metadata and `raw_tokens` destroyed;
a record-count change compiling clean while leaving hardcoded assertions red; a shallow glob
under-reporting.

**Mitigation.** Guarded generator path only, never hand-edits; never `--allow-stamp-loss`;
`corpus_literal_sweep` after every regeneration; old **and** new counts grepped across
`tests/`, `src/`, `apps/`, `scripts/`.

**PI exposure is the serious tail risk** — a dropped `pi_field` or license stamp is a
distribution problem, not a test failure. Verify per record.

## 7. R6 — Epic 3 could exceed one bundle

The Core Rulebook has 5,551 non-DONE units across eight buckets. SD-33 spent ten waves on
8,330 units of *verification alone*, and completion is more work per unit than verification.

**Mitigation, and why this is shaped rather than open:** the bundle's primary deliverable is
the atlas (Epic 1) and the forward plan (Epic 5). Epic 4 is a separate, much smaller book that
does not depend on Epic 3 finishing. If Epic 3 runs long, it still produces
measured rates per bucket as it goes (AT-34-E3-004 is incremental by design), and those rates
are what Epic 4 needs. A Core Rulebook at 90% of 6,701 with a full step-cost ledger, plus a banked Ultimate Campaign, still
delivers S1 and S3.

**This is not a licence to under-deliver S2.** The Core Rulebook target is 6,701 of 6,701.
Falling short requires the per-bucket residual named with its mechanism — never a scope cut.

## 8. R7 — Mostly-sequential epics mean limited parallel throughput

Every epic feeds the next: atlas -> tables -> books -> rates -> plan.

**The one exception is Epics 3 and 4** — different books, disjoint corpus subtrees, both gated
only on Epic 2. Whether they run concurrently is decided **at launch** by
`workflow-instruction.md §4`'s per-file check, not assumed: both touch `src/rules_core/` and
`src/bin/`, so unless disjointness is proven they run sequentially, Core Rulebook first.

**Mitigation.** Every other boundary is a real dependency and inventing concurrency across it
would build work on a denominator that had not landed yet. Epic 3's one-bucket-per-cycle shape
is what keeps a long epic from delivering nothing until it finishes.

## 9. Open questions — no answer yet, and none invented

| # | Question | Answered by |
|---|---|---|
| Q1 | What does building one engine table actually cost? | AT-34-E2-001's measured build rate |
| Q2 | Does clearing bucket B need one mechanism or many? | AT-34-E3-001 |
| Q3 | Is bucket C a display-layer job or an engine job? | AT-34-E3-002 |
| Q4 | Do the atlas's ten buckets survive contact with a whole book? | AT-34-E3-006's defect file |
| Q5 | Which other books are genuinely single-bucket — the low-hanging fruit? | AT-34-E5-004 |
| Q6 | Beyond the `power` table, what else must be built? | AT-34-E5-002's capability register |

**Q4 is the one that decides whether the bundle succeeded.** Everything else is cost; Q4 is
correctness of the deliverable itself.

**Q6 is the operator's question, asked verbatim:** *"if we need to build something to process
the remaining work after the shape engine runs, sd34 must tell us that"*. The capability
register is that answer in machine-readable form. Bucket A already named nine such things before
the bundle starts — eight get built; the register is where `power` and anything Epics 3 or 4
discover are recorded with their populations and costs.

## 10. Lesson enforcement — no UNENFORCED row remains

`workflow-instruction.md §12` carries 26 standing lessons and **every one names an enforcing
command**. Under `decisions.md §12` an `UNENFORCED` marking is itself a defect, so this section
tracks the state rather than housing exceptions.

**Row 15 was UNENFORCED in this package's first draft and is now closed** by AT-34-E1-006 — a
`verify.sh --only figure-provenance` stage that fails on a figure with no reachable re-derive
command, and on a gate PASS line that names no population.

Its origin: SD-33's corpus sweep was green on records whose `raw_tokens` were empty. Its
population was "tokens the record claims", and a record claiming nothing cannot mismatch. The
green was real and meaningless.

**Rows 17 and 19–22 are this session's own lessons** (`decisions.md §12` L1–L5), each traced to
a real cost paid during SD-33's closure or this package's authoring:

| Lesson | What it cost | Enforcer |
|---|---|---|
| L1 — a field's name is not its meaning | a wrong ingestion headline reported to the operator, against a question already answered correctly | AT-34-E1-002 condition 6 + AT-34-E1-005 |
| L2 — never carry your own number forward | two counting errors, both from inherited figures; one hid the second vehicle book | AT-34-E1-006 |
| L3 — a script's return value is not a closure claim | a workflow reported `closed: true` with release notes unwritten; a lane reported `complete` over 103 of 494 units | AT-34-E6-001 |
| L4 — match structured fields, not substrings | a full wave halted on a **passing** scan | §2.4's dispatch contract |
| L5 — a repeated workaround means clear the obstacle | five waves; the fifth committed a 139-file revert titled "release notes" | §10 step 1 + AT-34-E6-001 |
| L6 — a stale branch's file count is not its value | SD-33's sweep nearly folded a 1,612-file superseded branch over a 45-record real one | `forward-scope-register.md §E1` + AT-34-E6-003's schema-against-HEAD diagnosis |
| L7 — run the suite after the last write that can move it | a true green receipt, a red tree one commit later, two extra scan attempts | §6 step 3 + §7's build-scope SHA |
| L8 — a gate's examined-population must grow when records are added | the only proof the fold's 65 records were swept was the count moving +65 | §6 step 3 + §7's sweep-population row + AT-34-E6-001 |

**The risk this section now carries is not that a lesson is unenforced — it is that an enforcer
is weaker than it looks.** Three specific ways that could happen here:

- **AT-34-E1-006's stage could be satisfiable by a command that does not actually reproduce the
  figure.** Its RED→GREEN proof must use a *wrong* command alongside a missing one.
- **AT-34-E1-002 condition 6 could pass on a citation that resolves to the wrong line** after a
  refactor moves code without changing line counts. The atlas should assert on the cited
  content, not only the path and line.
- **The 3-strike recurrence control depends on lanes emitting `incident` events at all.** A lane
  that works around a problem silently never trips it. `§2.3`'s emit-at-the-moment rule is the
  only thing behind that, and it is discipline, not a gate.

Those three are the honest residual, and AT-34-E6-001's scan is where they are checked.

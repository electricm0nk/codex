---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
---

# SD-35 Risks and Open Questions

## 1. Self-healable vs non-self-healable

**Self-healable** (resolve inline, continue): dirty tree; a single-token audit violation;
unrelated test-setup breakage; build-counter out of sync; a stage-count or binary-count
assertion moved by this cycle's own deliberate change.

**Non-self-healable** (write `## Open blockers` in `progress.md`, stop): working tree diverged
needing manual rebase; two live cycles on conflicting files; a launch gate not actually met;
RED→GREEN not preserved; a stub, inline mock, or `"Would …"` string in shipping code; a cycle
under the floor that is not the whole remainder.

**A `## Open blockers` entry pauses the bundle** and is a request for an operator ruling
(`decisions.md §6`).

## 2. R1 — The sheet rule could be read as "print anything and call it done"

**The bundle's central risk.** `decisions.md §1` says resolve every resolvable term. A lane
under throughput pressure could emit `Words` for a term the character does settle, and the unit
would read `sheet-complete` with a wrong-looking line ("DC 10 + spell level + Cha").

**Mitigation.** The converter's mapping rule is stated once (`technical-design.md §1`) and
tested per value form on real records; AT-35-E2-001's per-kind converter gates print the count
of `Number` / `Dice` / `Text` outcomes per kind, so a kind whose `Text` share climbs between
cycles is visible in `token-coverage.json`. The closure scan samples 200 `sheet-complete` units
and re-evaluates each from its `SheetRule`. The operator's example — *"a final number, not a number plus another
number"* — is the test's own assertion text.

**Residual.** A `Words` line that *should* have resolved is a correctness defect the oracle
harness does not see (it checks totals, not lines). AT-35-E7-002's retrospective must report
the `Words` share per kind at closure so SD-36 can decide whether to audit it.

## 3. R2 — The batch floor could be gamed by scope definition

A lane could define a 500-unit scope that mixes mechanisms it has no intention of closing, pass
the gate, close 40, and report `partial`.

**Mitigation.** `cycle_scope_gate.py --receipt` reports `closed` against the scoped population
in the same row; `partial` must name the refused-token remainder with counts that sum
(`workflow-instruction.md §6` step 8). The epic wrap-up names every cycle whose closed count is
under half its scoped population. **This is a reading, not a gate** — tracked in §10.

## 4. R3 — Test-suite consolidation (AT-35-E1-003) could lose coverage

Two families, ~184 files, ~80k lines, become two table-driven binaries.

**Mitigation.** Name-by-name diff of `cargo test -- --list` before and after; test count
unchanged; the 29-suite inherited baseline re-derived after the change; the change lands in its
own worktree and is the first thing the epic's full gate exercises. If the diff shows a lost
test, the cycle is `partial` and the missing test is restored before the merge.

**Why it is worth the risk.** Every later cycle pays the build. The fable review estimated
543 → ~360 binaries; the measured "before" and "after" in `build-time.json` are the honest
figure.

## 5. R4 — A new status breaks consumers (again)

SD-34 wave 22's `oracle-agree` / `oracle-unverifiable` left 14 of 40 stages red because
downstream instruments raised on unknown status (fable review §7).

**Mitigation.** AT-35-E2-003 uses the last-added status as a grep census and updates every site
in the same cycle; the epic's full gate runs before Epic 3 dispatches. `decisions.md §9` L5.

## 6. R5 — Bucket B's mechanism might not be one mechanism

11,589 units under three evidence families. SD-34 Q2 ("one mechanism or many?") was answered
for the Core Rulebook by nine mechanisms (`SD-34/decisions.md §14`).

**Mitigation.** Cycles scope by kind × evidence family, never by book; each is ≥ 500 units so a
mechanism that turns out to be several still moves a large population per build. The refused
residue is named by token type and bundled forward, never left as "the rest". SD-33's open
deferral 1 (1,128 unmatched pool-group prefixes) is the known sub-mechanism and closes here.

## 7. R6 — Bucket X's choice filter is a real capability, not a rendering

168 units. The operator's requirement (`SD-34/decisions.md §17`) is a backend query, not a
sheet line — the one Epic 5 criterion that builds product behavior.

**Mitigation.** The pieces exist (`list_class_feature_pool_options`,
`evaluate_feat_prerequisites`, `character_prereq_facts`); the join is the deliverable, on the
existing level-up IPC. Its test is one fixture character, one excluded option, one included.
It is scoped as one cycle bundled with U/Z to meet the floor.

## 8. R7 — The corpus-wide pass could be slow

AT-35-E2-005 converts 23,315 records and evaluates each against a probe character. SD-33's oracle runs were
slow because they spawned a JVM per unit; this pass is in-process Rust and should not be — but
that is an expectation, not a measurement.

**Mitigation.** The pass is timed and its wall time recorded before any later cycle re-runs it
(N7). If it exceeds one build's wall time, the converter runs per kind and the ledger merges.

## 8a. R8 — Two engines side by side until Epic 6

From Epic 2 to Epic 5 the live side carries both the old string-formula path (`PcgenFormulaEvaluator`
and its 14 callers) and the new `Expr` evaluator. A number the sheet shows could come from
either, and they could disagree.

**Mitigation.** The oracle harness compares **our** `Number` values against PCGen's on the
fixture roster at AT-35-E2-005 and on every cycle that adds a `Number` mapping; AT-35-E4-002's
run uses the new evaluator's values explicitly. The residue gate forbids any *new* live PCGen
read from cycle 1, so the old path only shrinks. Epic 6 removes it with parity artifacts before
and after.

**Residual.** A value the old path computed and the sheet already showed, which the converter
maps differently, is caught only if that value is on the fixture roster the oracle exports.
AT-35-E6-001 widens the roster to at least one character per class before the exit begins.

## 8b. R9 — The residue gate's allow-list is the carve-out this bundle would most want

The gate's live-path list is a Python list. Adding `src/rules_core/some_new_module.rs` to the
tool side is one line.

**Mitigation.** The path list is by top-level directory, not by file; the allow-list has exactly
the entries in `technical-design.md §0`; the closure scan greps independently of the gate
(`acceptance-and-verification.md §3a`) and reads the gate's own diff history for any addition.

## 9. Open questions — no answer yet, and none invented

| # | Question | Answered by |
|---|---|---|
| Q1 | How many of the 23,315 close on the first conversion with zero new mapping rows? | AT-35-E2-005 |
| Q2 | What is the build time after consolidation? | AT-35-E1-003's `build-time.json` |
| Q3 | Which token types refuse the most units after the first pass? | `token-coverage.json` |
| Q4 | What is the `Words` share per kind at closure? | AT-35-E7-002 |
| Q5 | Does any capability in SD-34's register survive the sheet rule as genuinely required? | AT-35-E5-005 |
| Q6 | What does one mechanism-cycle cost, measured? | AT-35-E3-004, AT-35-E4-003 |
| Q7 | What is the exact live-side PCGen baseline, and how much of it is `cache_gen` (relocate) vs real live readers (replace)? | AT-35-E1-005's first run |
| Q8 | Does our `Expr` vocabulary cover every `Number` the totals need, or does a PCGen variable exist that names a fact our character does not carry? | **Largely answered pre-launch** by the token-mapping synthesis (schema v2, `decisions.md §15`): 23,312 of 23,315 reachable; the residue is `blockers.md` B4 (character facts) and B8 (small shapes), owned by AT-35-E4-001 |
| Q9 | Rulings R1–R3 (`VISIBLE:DISPLAY`, PI term-hit bucket, closure scope) | The operator; AT-35-E2-001 builds to the recommendations until then |

**Q1 decides the shape of Epics 3–5.** A large first-pass yield means those epics are mostly
arm-adding cycles; a small one means the placement problem (bucket B) is real engine work
after all, and Epic 3's cycles will report it by evidence family.

## 10. Lesson enforcement — one partially-enforced row, tracked

`workflow-instruction.md §12` rows 27–34 each name an enforcer. **Row 29** (no per-unit proof
machinery) is mechanical in its ratio (`cycle_scope_gate.py --receipt`) and a reading in its
"what the lines bought" review. **R2 above** is likewise a reading. Both are checked at every
epic wrap-up and reported in the retrospective; neither is a gate with a nonzero exit. If
either fires more than twice across the bundle, `§10` step 1's recurrence rule requires a
mechanical control or a named escalation.

**Three ways an enforcer here could be weaker than it looks:**

- `cycle_scope_gate.py`'s whole-remainder exception could be satisfied by a scope filter that
  happens to match everything left in one bucket while other buckets still have work. The gate
  compares against the whole non-DONE population, not the bucket's — the selftest plants this
  case.
- A content anchor could match a moved *and changed* line if the anchor text is too short.
  Anchors are the exact multi-line condition, not a function name alone.
- The per-kind on-screen test could pass on a fixture that holds a record the converter never
  refuses. That is the point — it proves the section, not the mapping; the mappings are proven by the
  per-kind converter gates.

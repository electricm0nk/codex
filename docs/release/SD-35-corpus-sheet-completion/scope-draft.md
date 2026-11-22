---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
---

# SD-35 Scope Draft — Corpus Sheet Completion

## 1. The bundle in one sentence

Drive every remaining content unit in the corpus — **23,315 of 49,438, across all 37 books** —
to the state where the character sheet shows what the player would write on paper: a final
number, a dice expression, or the rule's words; in cycles of one mechanism at a time, never
fewer than 500 units, one build each.

## 2. The operator's requirement, verbatim

> *"This isn't a video game, it's a character sheet generator. Many rules just need to get printed
> out [...] Just write the rule out so we can print it on paper."*

> *"We should print out a final number. Not a number, plus another number, plus another number.
> Do the math. But sometimes a number is that very addition. A sword that does d8+2 damage - you
> just write that down. You don't need to write a dice rolling engine."*

> *"We need to tackle large swaths of items before we stop to do a build and test. [...] this is
> ok in theory until you start deciding to only put 2 items in a wave. That can't happen again.
> You have to think bigger."*

> *"Using it to convert and test is fine. But when we are done, there should be nothing left of
> pcgen."* *"it's not fine to include 1 line of java in our live code."*

Four requirements, ruled in `decisions.md §1`, `§2`, `§3`, `§11`:

1. **The finish line is the sheet**, not a pipeline proof.
2. **A cycle is a mechanism, corpus-wide, 500 units minimum**, enforced by a script.
3. **One build per cycle; the full gate once per epic.**
4. **PCGen is a converter input and a test oracle. None of it in live code at closure.**

## 3. Where we actually stand

Measured 2026-09-07 at `tranche/14` HEAD `5f6b18f4e3` and re-measured, identical, at the
`tranche/15` cut `4c6c57eb9f`; `docs/work-inventory.json` `generated_at: 2026-09-07T13:06:14Z`.
Re-derive commands in `content-unit-inventory.md`.

| Bucket | Meaning | Units of 49,438 |
|---|---|---:|
| DONE | grounded, text-complete, oracle-agree, oracle-unverifiable | 26,123 |
| B | table exists, record never placed in it | 11,589 |
| M | numbers ingested, never computed | 4,334 |
| C | computed, never shown | 4,180 |
| D | other engine gap, sub-causes enumerated | 1,982 |
| A | no engine table for this kind (`power` 421, `companion` 28) | 449 |
| V | verified by proxy, not by the oracle | 392 |
| U | instrument cannot express a verdict | 202 |
| X | deferred with reason | 168 |
| Z | not started (`beginner_box`, no compiled rule set) | 19 |
| **non-DONE** | | **23,315** |

By kind, `class_feature` alone is 12,856 of the 23,315 (7,866 in B, 4,180 in C). The five
common PCGen token types — `BONUS:`, the `PRE*` family, `DEFINE:`, `COST:`, `SAB:` — are the
only compute-bearing tokens on 13,002 of the 21,671 remaining units whose source line the
inventory lane could resolve. The fable review's TOKEN-MODEL measured the whole remainder's
vocabulary at 189 token types with 28 covering 90% of 181,274 instances. **This is a small
vocabulary, not 23 thousand separate problems.**

**The instruments exist.** `formula_interpreter` (F1..F9) and `bonus_stack_reader` already
evaluate the `BONUS:VAR` / `DEFINE` shapes present on 6,708 of the 22,369 non-vehicle-book
remainder units (fable review §1.b). `pre_tokens.rs` covers 48 of 69 `PRE*` types. The
`wiring_class` determinator classifies every record's token closure. SD-34's Completion Atlas
partitions every unit fail-closed. **SD-35 builds two small things** — the ingest-time converter
and our live evaluator — and runs the converter corpus-wide.

## 4. What SD-34 taught, and what SD-35 changes

SD-34 was a deliberate two-book proof (`SD-34/forward-scope-register.md` C1.4). It proved the
atlas, built eight engine tables, and measured rates. It also showed three things this bundle
must not repeat:

- **The DONE bar was simulation-shaped.** `grounded` meant a magnitude observed reaching a
  consumer through a fixture-executed pipeline. That bar built a `racial_sla` engine module with
  three hand-derived guards for a DC that is `10 + spell level + Cha`, and refused a correct Monk
  damage table because the engine has no size-Large character. `decisions.md §1` replaces it.
- **Cycles shrank to 2–45 units and paid a full build each** (waves 43–48), driven by a real
  wave-42 regression that made the full suite feel non-negotiable. Waves 49–51 batched 129, 343,
  217 with one build each. `decisions.md §2` makes the batch a floor with a nonzero exit.
- **Every edit paid a citation tax.** Three scripts pin line numbers in
  `v06_work_inventory.rs`; every code change shifts them, and two of three had silently drifted
  by wave 51. `AT-35-E1-002` anchors them to content.

The fable review (`SD-34/fable-review.md §1.b`, TOKEN-MODEL) supplied the method: **cost scales
with token vocabulary, not unit count.** SD-35 is that method run to the end.

## 5. The mechanism — one converter at ingest, one small evaluator live, run corpus-wide

`technical-design.md §0–§2` in one paragraph: a **converter** runs at ingest time over every
record's PCGen token closure and writes our own `SheetRule` — a label, a value that is either
an expression in our closed vocabulary (level, class level, ability modifier, size…), a dice
string, or text, plus the substituted prose. **No PCGen survives the conversion.** The live
engine loads `SheetRule` records and a forty-line **evaluator** does the arithmetic for the
character; one generic sheet section prints the line (`AT-35-E2-002`). A unit whose rule
converts and renders is `sheet-complete` (`AT-35-E2-003`). The converter grows by **token
type**, never by unit: a cycle adds a mapping row for `BONUS:SKILL`, or `DR:`, or `SPELLS:`,
and every unit carrying that token across all 37 books moves in the same build. **PCGen's own
computed totals are the test oracle** for every `Number` mapping (`AT-35-E2-005`,
`AT-35-E4-002`, `AT-35-E6-004`). When every unit is converted, **Epic 6 removes the old
run-time PCGen path from the live side** — 78 files by coarse grep at authoring — and a gate
that counted it down from cycle 1 reads zero.

Coverage is tracked per token type in `artifacts/epic-2-sheet-rule/token-coverage.json`
(`AT-35-E2-004`) — units carried, units rendered, units still refused — so "what is left" is
always named by mechanism (memory `name-a-remainder-by-mechanism`).

## 6. Epics, in dispatch order

| Epic | What | Population | Why this order |
|---|---|---:|---|
| 1 — Tax cut | batch-floor gate, content-anchored citations, test-suite consolidation, ratio row, **PCGen residue gate with its baseline**, **SD-34's closure folded** (`decisions.md §12`) | instruments | every later build pays less; both counters exist from cycle 1; no debt carried |
| 2 — Sheet rule | converter + our schema, live evaluator + on-screen section, `sheet-complete` status, token ledger, first corpus-wide conversion oracle-checked | all 23,315 | the finish line, made mechanical |
| 3 — Place and surface | bucket B to zero, bucket C to zero | 15,769 | the biggest homogeneous swaths |
| 4 — Resolve and verify | bucket M to zero by token family; bucket V through the oracle once | 4,726 | the compute-bearing tail |
| 5 — Residues | A (2 tables), D (generic class chassis, deity), U, Z, X (choice filter); corpus at 49,438 of 49,438 | 2,820 | named capabilities |
| 6 — PCGen exit | formula evaluator, token closure, generators, prose renderer, desktop catalogs off the live side; residue gate at zero; oracle parity before and after | 78 live files (coarse) | the boundary ruling, made mechanical |
| 7 — Closure | scan, retrospective, sweep, docs, PR | — | once |

Unit populations are at the 2026-09-07 measurement and sum to 23,315. They are re-measured at
the `tranche/15` cut (`content-unit-inventory.md §0`); Epic 2's first corpus-wide conversion is
expected to move units out of every bucket before Epics 3–5 start, and each epic re-derives its
own population at dispatch. Epic 6's population is a file count from `pcgen_residue_gate.py`,
re-derived every cycle.

## 7. Success — four things, in priority order

**S1 — Every unit is DONE under the sheet rule.** `python3 scripts/completion_atlas.py --check`
at closure reports `DONE=49438 of 49438`, every other bucket zero, `unclassified=0`. Every DONE
unit's evidence pointer resolves; a sample is re-derived independently (AT-35-E5-005,
AT-35-E6-001).

**S2 — The sheet shows it.** For each of the 19 kinds, one frontend test proves the generic
section renders a held record's sheet line on screen (AT-35-E2-002). SD-31 Decision 7
condition 3 — proven on screen, not inferred from a green gate — satisfied once per kind.

**S3 — Nothing left of PCGen on the live side.** `python3 scripts/pcgen_residue_gate.py --check
--closure` → `live_files=0 live_hits=0`, plus the scan's own independent grep
(`acceptance-and-verification.md §3a`); `data/sheet_rules/` carries no PCGen token, formula, or
variable name; the oracle parity artifacts before and after the exit agree (AT-35-E6-004).

**S4 — The process held.** No cycle under 500 units unless it was the last of its mechanism
(`cycle_scope_gate.py` output in every receipt); one build per cycle; full gate per epic;
`pcgen_live_files` never rose; build time at closure lower than at launch, measured
(AT-35-E1-003).

**No book count.** Books fall out of S1; the dashboard reports them; they are not the bar.

## 8. What SD-35 must tell us, explicitly

1. **Per token type, what renders and what does not** — `token-coverage.json`, re-derived
   every cycle (AT-35-E2-004).
2. **What the first corpus-wide pass closed with zero new per-unit code**, and the projection
   for the rest by token type (AT-35-E2-005).
3. **The measured rate per mechanism** — units, wall time, lines per unit (AT-35-E3-004,
   AT-35-E4-003).
4. **Build time before and after the test-suite consolidation** (AT-35-E1-003).
5. **Any capability that still must be built** — the register inherited from SD-34
   (`SD-34/artifacts/epic-5-forward-plan/capability-register.json`) re-derived at closure with
   every row either built or proven unnecessary under the sheet rule (AT-35-E5-005).
6. **The PCGen residue count per epic**, from the first-run baseline to zero (AT-35-E7-002).

## 9. Explicitly out of scope

- **A second PCGen-format system** (Starfinder) and the non-PCGen systems — `forward-scope-register.md §C2`.
- **Errata detection** for banked books — `§C3.3`, inherited from SD-34.
- **Oracle-agreement-as-correctness research** — `§C3.2`.
- **SD-34's own open cards.** They close in SD-34 before this bundle launches
  (`decisions.md §7`).
- **Keeping any PCGen reader on the live side "for now".** That is what SD-31 Decision 20 did
  and it grew to 78 files. `decisions.md §11`.
- **Deleting the converter, the PCGen parser, the generators, or the oracle harness.** They are
  the tool side and they are the asset Starfinder starts from (`decisions.md §11`, what is kept).
- **A character-size subsystem, a downtime/retraining subsystem, a multi-companion system.**
  Under the sheet rule these are not needed to close their units: the Monk damage table prints
  for the character's size; a retraining rule prints as words. If a later product bundle wants
  them, that is its scope.

## 10. Cross-references

- `decisions.md §1`–`§4`, `§11` — the five rulings this scope rests on.
- `epic-breakdown.md` — 30 criteria across 7 epics.
- `technical-design.md` — the boundary, the converter, the evaluator, the status, the gates, the exit.
- `content-unit-inventory.md` — every figure above with its re-derive command.
- `../SD-34-book-completion/fable-review.md §1.b`, `artifacts/fable-review/TOKEN-MODEL.md` — the
  method.
- `../SD-34-book-completion/forward-scope-register.md` C1.4, C1.6 — SD-34's own statement of
  what SD-35 is.

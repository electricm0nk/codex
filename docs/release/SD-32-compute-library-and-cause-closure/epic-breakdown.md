---
canonical: true
owner: god-emporer
status: finalised against SD-31 wave 31's measurement, 2026-08-22
source: SD-31/artifacts/MEASURE-TWICE.md (both operator questions answered, adversarially reviewed)
---

# SD-32 epic breakdown

Every number here comes from SD-31 wave 31's measurement and survived adversarial review. Where a
lane's filed figure was corrected, the **corrected** figure is used and the original noted — several
were wrong in both directions.

**Board at SD-31 close: 13,458 / 38,372 = 35.07%. Not-done: 24,914.**

## What the measurement actually established

**Plumbing beats rules complexity, ~3.3:1 to 4.4:1** — not the 4:1–5:1 first filed. The correction
came from measuring an overlap the original assumed: the largest "genuine complexity" bucket (2,453
units, classes outside the modelled roster) overlaps the largest "plumbing" bucket (8,243 units, a
class-name field read from the wrong place) by **1,354–2,124 measured units**, not the ~532 assumed.
Those overlap units — Psychic, Vigilante, Medium, Magus, Shifter, Kineticist, Spiritualist,
Occultist — genuinely need modelling, not relabelling. **The operator's suspicion survives; the
precision does not.**

**Ten compute families, ceiling 3,201 units (12.8%)** — not the 4,948 (19.9%) first filed. The
reason matters more than the number: **the single largest family is flat literal constants, 1,747
units, and it gets zero benefit from any shared function.** A library cannot help with values that
do not compute.

---

## Epic 1 — The compute library

**Ceiling: 3,201 of 24,914 units (12.8%). All nine formula-bearing families (F1-F9) are already
evaluable by grammar; one of them (F4) additionally needs a binding layer to resolve values.**

The canonical shape-family vocabulary lives in `scripts/shape_ledger.py`
(`artifacts/gate-1-shape-closure/family-vocabulary.md` is its reconciliation write-up, card
`family-vocabulary-reconciliation`, `decisions.md §12a`) — this section's F1/F2/F3 rows below are
work items, not a second family-count table; do not diff a family's unit count against them.

`formula_interpreter.rs` (authorised by ruling §20) evaluates all nine formula-bearing families
(F1..F9) directly by grammar today. **F4**'s producer-bound bare-identifier subset additionally
needs a binding layer to resolve what value the identifier holds before that grammar can run on
it, and one already exists: wave 26's `bonus_stack_reader.rs`, 329 lines, proving the "read the
producers of a named variable and sum them" pattern — generalizable to **77.2% (893/1,156)** of
the corpus's distinct custom identifiers (SD-31 wave 31 measurement, identifier-wide walk) or
**92.4% (390/422)** on the canonical F4-predicate-scoped population
(`family-vocabulary.md` §3, independently re-derived). F10 is a separate, 3-unit
level-threshold step-count family with no binding-layer need at all — earlier bundle drafts
conflated "the tenth family" (F10, by list position) with F4 (the actual binding-layer target);
that labelling defect is fixed by card `family-vocabulary-reconciliation`. One lane framed the
binding layer at 46.8% using a narrower mechanism; the broader already-proven one reaches the
figures above. The correction ran in both directions, which is why both were re-derived.

| F# | Work | Units |
|---|---|---:|
| F1 | Extract the general form of each family from the ~166 already-proven hand-modelled functions, rather than writing fresh. Each was verified byte-exact against the corpus when written. | — |
| F2 | Generalise `bonus_stack_reader.rs` to F4's binding pattern (the producer-bound bare-identifier subset) | up to 77.2%/92.4% of custom identifiers (two denominators — see `family-vocabulary.md` §3) |
| F3 | Wire the library behind the consumers, every value clearing `derived_evaluator_fixture_check` | 3,201 ceiling |

**Do not plan on 20%.** And do not spend effort on the 1,747-unit flat-constant family expecting
leverage — it is the biggest and the least helped.

---

## Epic 2 — Cause closure

Eleven named blocker shapes — ten with measured populations in the table below plus T10 (no unit
count) — closed **by class rather than by instance**. Epic 2's closure targets are the eight
measured shapes T2a, T2b, T9, T4, T12, T5, T1, T3 (T5 is credited via Epic 4 / card 4 and T3 via
Epic 5 / card 1; Epic 2 cites those receipts rather than re-closing them); T8/T7 are sub-20-unit
residuals closed opportunistically (`acceptance-and-verification.md` AT-32-E2-001, `kanban.md` #11).
Ranked by measured population.

| T# | Shape | Units | Note |
|---|---|---:|---|
| T2a | `data.class` read from the wrong place | **8,243** of 11,502 (71.7%) | **Only 2,360 are cleanly prefix-remappable.** The other 5,883 are a mix of category-label plumbing and genuine unmodelled-class content and must be re-examined before either bucket is credited. |
| T2b | Race-trait compound-key matcher | **2,472** | Found wave 19, named in six waves, never closed. Exact, reproduced. |
| T9 | Per-record onboarding backlog in registered books | **2,651** | spell 726, companion 726, feat 480, monster_ability 517, equipment 174, monster 28 |
| T4 | Built-but-unreachable render surface | up to **2,763** | L9's 471 had a **true reachable count of zero** — a lane filed on-screen evidence that was false. Verify with the real driver. |
| T12 | Genuine missing engine mechanism (contrast bucket) | **~3,000** | The honest "rules are actually hard" pile |
| T5 | Book-level gate: one missing enum variant kills a whole book | **422** | inner_sea_magic 335, temples 64, taverns 20, faiths 3 |
| T1 | Dispatch gap (Monk shape) | classes **0** (exhausted); **1 new live instance in equipment** | APG omits an `Equipmods` variant seven other books carry, against 35 real records. Race/monster **never fully checked** — 280 monster entries and 31 non-CRB races unexamined. |
| T3 | Self-erasing regeneration | **3 of 29** Rust binaries vulnerable; 17 never reached | `gen_advanced_race_guide` **live-reproduced wiping 93 spell + 15 equipment records** |
| T8 | Status stamp never re-examined once written | **12** | The producer's own doc comment names the missing check |
| T7 | Shallow single-hop traversal | **4** | 3 of 4 protected only by an incidental level mismatch, not by structure |

**T10 — unverified proxy measurement in the census process itself — has no unit count and is the
highest-leverage unstarted item in the ledger**, because every other number's reliability inherits
from it. Known instances: 431 filed vs 471 real; four blocked-row predicates unreproducible
(344/310, 0/2, 34/30, 0/12); five lanes' own proxies flagged unchecked.

---

## Epic 3 — Class reachability

The engine builds **eleven** classes. `class` is 28 done of 185, and it gates `class_feature` —
60% of everything remaining.

| | Count | Needs |
|---|---:|---|
| Prestige classes | **77** | Entry-requirement gating that **exists nowhere in the codebase**. Downstream `class_feature` population uncounted. |
| Real base classes, no table | 18 | Net-new table construction |
| Structurally not player classes | 48 | **Operator ruling B4** — monster HD progressions, Eidolon, power-list menus |
| Books with no compiled rule set | 28 | Epic 4 |
| CRB NPC classes | 5 | Real but untabled |
| **Ninja + Samurai** | **2** | **One missing weapon-proficiency table row.** Cheapest units in the program. |

Class chassis work: **2,453 units**, re-verified live.

---

## Epic 4 — Book onboarding

**422 units** behind a missing `RuleSetId` enum variant, across four books. The `adventurers_guide`
precedent shows the cost is **almost entirely compile-graph wiring, not content** — roughly 1.5–2h
per book, dominated by ~7 count-pinning files.

---

## Epic 5 — Automation, decided on evidence

Only candidates whose output can be **independently checked**. A tool that generates values needs a
fixture from bytes it does not read, or it manufactures plausible numbers faster than a human could.
Ruling §20 authorised the interpreter on exactly that condition.

**Already earned its place:** `scripts/coverage_ledger.py`. It proves inventory completeness
mechanically and **fails closed** on an empty predicate, so a placeholder group cannot manufacture
false 100% coverage.

**Finish first, before anything scales:** 17 of 29 Rust generators have never been checked for the
self-erasure shape, and 3 of the 12 checked are vulnerable. Scaling the interpreter over a generator
that silently empties its own fixtures is how thousands of banked units disappear with the suite
green.

---

## Sequencing

1. **T3 residual** — 17 unchecked generators. Protective; closes nothing; still first.
2. **Ninja + Samurai** — 2 units, one table row. Proves the S9 shape is real and cheap.
3. **Epic 4** — 422 units, calibrated cost, five kinds unblocked at once.
4. **Epic 1 F1/F2** — extract the library from proven code before writing anything fresh.
5. **Epic 2 T2a/T2b** — the two largest plumbing populations, once T10 makes their counts trustworthy.
6. **Epic 3** — the prestige-gating mechanism, the largest structural gap in the program.

## Operator rulings still open

**B1** `mod_only_rescue` (249 units) · **B2** per-race branch classification (gates race_trait) ·
**B4** the 48 non-PC-class units · **B5** the 5 `Ex-*` records.

B4 and B5 shrink the honest denominator without changing a line of code.

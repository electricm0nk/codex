---
canonical: true
owner: god-emporer
status: planning-ready pending §6 launch gates
bundle_id: SD-33
date: 2026-08-24
companion_to: ./README.md
---

# SD-33 Scope Draft — Computed-Value Verification

## 1. What changed, and why this bundle exists

SD-32 finished the ingestion program. Its four gates all closed:

- `no_record == 0` — every unit has a corpus record
- `unclassified == 0` — every unit in the ledger population carries a shape family
- Product Identity clean corpus-wide
- engines exist for every in-scope shape family (F1–F9)

**Every one of those gates asks the same kind of question: does the thing exist?** That question is cheap, binary, and self-evidencing. Crucially, **ingestion cannot silently lie about it** — a record is present or it is not, and a missing one is visible from the outside.

SD-33 asks a question with none of those properties: **is the computed number right?**

A wrong magnitude is indistinguishable from a right one by inspection. It renders. It has the right type. It sits in the right field. **The only way to catch it is to compare against something that already knows the answer.** That is what this bundle builds, and what it then applies.

## 2. What SD-33 promises

1. A **verification harness** capable of stating, per unit, whether our computed value **agrees with an oracle** — and, where it cannot, saying so as a first-class outcome rather than silence.
2. **Engine coverage closed to 100%** of the formula-bearing population: the 6,854 units never run through an engine, run.
3. The **4,224 `unknown`-status units** classified — no unit left in a bucket whose name means "we did not look".
4. The **8,330 units blessed by fixture or literal check** re-examined against the oracle.
5. **Two mechanisms, built as code with exit codes** — `THE-BOX.md` plus its enforcing tool, and the denominator gate. Not prose in a §12 lessons list.

## 3. What SD-33 does NOT promise

- **It does not promise that a running PCGen oracle is achievable.** Epic 2 is a timeboxed spike with a named fallback (`decisions.md §5`). If the spike fails, the fallback — reading PCGen's Java source per shape, the method that produced SD-32's `MaxCommand.java` correction — carries Epic 5 at reduced throughput, and **that becomes an explicit operator decision point, not a drift discovered at closure.**
- **It does not promise a percentage of units "verified correct" at authoring time.** Any such figure before Epic 2 returns would be exactly the provisional-headline defect `workflow-instruction.md §12` names (SD-31's retracted "1,049 formula shapes").
- **It does not promise to re-open SD-32's closed populations** beyond the 8,330 named in §2.4. If Epic 1's probe enumeration finds that a further population is unverifiable, that is a **finding with a named count**, escalated per `../../governance/blocker-closure-doctrine.md` — not a silent widening.

## 4. The population

Re-derived at `1d6ae1e72b`, 2026-08-24, `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`. Full table with per-figure commands: `README.md §4`.

```
49,438  inventory units
34,397    in the ledger population (not-done, non-excluded)
15,041    outside it  =  15,022 verdict==done  +  19 EXCLUDED_BOOKS

11,652  formula-bearing units (F1..F9)
 4,798    ever run through an engine        <- 41%
 6,854    NEVER run through an engine       <- Epic 3

   625  formula-bearing units carrying a magnitude
 8,330  units blessed by fixture/literal, never by oracle   <- Epic 5
 4,224  units at status `unknown`                            <- Epic 4
```

**The denominator statement this bundle exists to make honest:** SD-32's Gate 2 reports *97.9% recognised*. True — of the 41% it ran.

## 5. Epics

| Epic | Name | Gated on | Parallel-safe with |
|---|---|---|---|
| 1 | **Instruments** — `THE-BOX.md` + its enforcing tool, the denominator gate, and the probe-surface enumeration | launch gates | — |
| 2 | **Oracle harness** — timeboxed PCGen headless spike; fallback named up front | Epic 1 | Epic 3, Epic 4 |
| 3 | **Engine-coverage closure** — the 6,854 units, 41% → 100% | Epic 1 | Epic 2, Epic 4 |
| 4 | **Unknown-status classification** — the 4,224 | Epic 1 | Epic 2, Epic 3 |
| 5 | **Re-verification** — the 8,330 fixture/literal-blessed units against the oracle | Epic 2 | — |
| 6 | **Closure epilogue** | Epics 1–5 all `complete` | — |

**Epics 2, 3 and 4 are write-disjoint and run concurrently** — verified file-touch sets in `workflow-instruction.md §3`/`§4`. That is why the oracle spike being uncertain does not stall the bundle: coverage and classification advance regardless.

## 6. Launch gates

Open at authoring time. Full detail in `README.md §9` and `technical-requirements.md §1`:

1. SD-32's closure PR merged to `develop`.
2. **SD-32's own instrument debt closed inside SD-32** — not imported here.
3. `tranche/13` cut from `develop` and pushed.

## 7. The bundle's own standing hazard

SD-32's retrospective established that **the handoff loses mechanisms, not lessons** — SD-31's lessons were transcribed verbatim into SD-32's `workflow-instruction.md §9` and ignored anyway, while THE-BOX decayed from a 377-line living artifact plus `coverage_ledger.py` into a single past-tense anecdote.

**SD-33's countermeasure is structural, not editorial:** its two mandatory mechanisms (`decisions.md §1`, `§2`) are Epic 1 deliverables with exit codes, and `THE-BOX.md` carries a staleness gate that fails when the document has not been re-derived against the current HEAD. A document nobody must update is a document nobody updates.

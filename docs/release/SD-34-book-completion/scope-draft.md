---
canonical: true
owner: god-emporer
bundle_id: SD-34
status: planning-ready
date: 2026-08-26
---

# SD-34 Scope Draft — Completion Atlas, proven on two books

## 1. The bundle in one sentence

Produce **one exhaustive, mechanically-derived statement of every step that remains for every
one of the 49,438 units in the corpus** — and prove that statement is true by driving two
books of opposite shape to zero remaining steps: the **Core Rulebook** (deep, every bucket)
and **Ultimate Campaign** (shallow, effectively one bucket).

## 2. The operator's requirement, verbatim

> I need to know what is left. everything I think we are done, you surface 3 more things.
> that stops with sd-34.

> if we need to build something to process the remaining work after the shape engine runs,
> sd34 must tell us that

**That is the primary deliverable.** Not a book count. The two books are *vehicles* —
completing them is what proves the map is real rather than a spreadsheet nobody validated.

The recurring failure this bundle exists to end: a bundle reports done, and a further
category of work surfaces that nobody had counted. It has happened enough times to be a
pattern, not bad luck. The cure is not more diligence — it is a **fail-closed partition**
where every unit lands in exactly one named bucket, `unclassified` is a hard error, and the
count of buckets is itself a checked number.

## 3. Where we actually stand

Ingestion is **complete** and the shape engines **exist**. SD-33 delivered both.

- **All 49,438 units were read from source.** Every unit carries a real `source_file` and
  `source_line`. There are 51,505 JSON files under `data/corpus/`, 48,881 of them outside the two
  non-inventory directories (`content-unit-inventory.md §1` has the command and the 39→37 book collapse).
- **The `not-ingested` status field is a misnomer** and has caused real confusion, including
  during this package's own authoring. Every one of its evidence strings is about the
  *engine*, never the corpus: `ability_content_has_no_engine_table`,
  `class_feature_owner_matched_by_name_but_record_not_held_by_engine`,
  `race_trait_record_loaded_but_never_applies`. Renaming it is an SD-34 deliverable
  (AT-34-E1-005), because a field whose name asserts the opposite of its meaning will
  mislead again.
- **The shape engines work.** `formula_interpreter` covers F1..F9, recognising 10,626 of
  11,652 formulas and refusing 240 rather than guessing.

## 4. What a shape engine does, and what it does not

**A shape engine turns a formula string into a number.** That is its entire job, and it does
it well.

It does **not** put the record anywhere, attach it to a character, or show it to a player.

The proof: **26,396** units carry magnitude tokens — the shape engines' own input — and
**13,119 of those 26,396** are still not held by the engine. Half the feedstock is stuck
*after* the engine can already compute it.

The engine's own promotion ladder states the rest of the requirement in code
(`src/bin/v06_work_inventory.rs:9595`):

```rust
if has_real_description
    && is_display_wiring_class_for_promotion(wc_class)
    && !universal_sheet_modifier
    && facts.class_feature_pool_catalog_holds(&unit.source_book, &unit.key)
```

Computing a value is not one of the four conditions.

## 5. The Completion Atlas — measured, exhaustive, zero unclassified

Every unit, one bucket, derived by execution 2026-08-27 from `docs/work-inventory.json` at
`ea2b3396f2` (the merged tip of SD-33's closure PR #377):

| Bucket | What remains for this unit | Units of 49,438 |
|---|---|---:|
| **DONE** | nothing | 12,265 of 49,438 (24.8%) |
| **B** | table exists, record not in it | 11,921 of 49,438 (24.1%) |
| **A** | engine has **no table** for this kind | 8,463 of 49,438 (17.1%) |
| **V** | verified by proxy, never by the oracle | 8,330 of 49,438 (16.8%) |
| **C** | held by the engine, never surfaced to the player | 4,388 of 49,438 (8.9%) |
| **M** | magnitude ingested, never computed or applied | 2,455 of 49,438 (5.0%) |
| **D** | other engine gap (named sub-causes) | 1,230 of 49,438 (2.5%) |
| **U** | instrument cannot express a verdict (named sub-causes) | 321 of 49,438 (0.6%) |
| **X** | deferred with a stated reason | 46 of 49,438 (0.1%) |
| **Z** | not started | 19 of 49,438 (0.0%) |
| | **unclassified** | **0** |

The buckets sum to 49,438 exactly. **That zero is the deliverable's whole point** and
AT-34-E1-002 makes it fail closed.

### Bucket A — the nine tables that do not exist

This is the answer to *"do we need to build something after the shape engine runs?"*
**Yes: nine engine tables.**

| Kind | Units | of which Core Rulebook |
|---|---:|---:|
| ability | 4,337 | 471 |
| template | 2,248 | 262 |
| trait | 487 | 0 |
| deity | 459 | 21 |
| power | 421 | 0 |
| domain | 183 | 34 |
| skill | 149 | 110 |
| language | 136 | 22 |
| companion | 43 | 14 |
| **Total** | **8,463** | **934** |

A record of these kinds has nowhere to live. The shape engine can compute its number; there
is no shelf to put the record on. **Seven of the nine tables are exercised by the Core
Rulebook** — only `trait` and `power` sit outside it. An eighth, `trait`, is added by the
second vehicle book (§6a), leaving `power` as the single table SD-34 costs rather than builds.

## 6. Core Rulebook — the first vehicle

**6,701 units**, and the most-blocked book in the corpus (5,551 non-DONE, ahead of
Advanced Player's Guide at 3,004).

| Bucket | Units | % of 6,701 |
|---|---:|---:|
| V — verified by proxy only | 2,582 of 6,701 | 38.5% |
| DONE | 1,150 of 6,701 | 17.2% |
| B — record not in table | 970 of 6,701 | 14.5% |
| A — no table exists | 934 of 6,701 | 13.9% |
| M — never computed/applied | 512 of 6,701 | 7.6% |
| C — not surfaced | 370 of 6,701 | 5.5% |
| D — other engine gap | 119 of 6,701 | 1.8% |
| U — unmeasurable | 58 of 6,701 | 0.9% |
| X — deferred | 6 of 6,701 | 0.1% |

**Every bucket except Z is present.** Completing this one book therefore measures the real
cost of every step type in the atlas — which is precisely what prices the rest.

## 6a. Ultimate Campaign — the second vehicle

**265 units, and its entire profile is four rows:**

| Kind | Bucket | Units |
|---|---|---:|
| trait | A — no engine table | 154 |
| ability | A — no engine table | 88 |
| feat | U — instrument cannot express a verdict | 21 |
| feat | X — deferred with reason | 2 |

**No B, C, D, M or V at all.** It is the cleanest book in the corpus: 242 of 265 units clear on
two tables, one of which (`ability`) the Core Rulebook already requires. Adding `trait` — 487
units across five books — takes this book from 0 of 265 to roughly 242 of 265 (91%) in a single move, leaving a
named 23-unit tail.

It earns its place for two reasons beyond the cheap win:

1. **It supplies the eighth table.** `trait` is not in the Core Rulebook. With it, only `power`
   remains unbuilt.
2. **It is shaped the opposite way to the Core Rulebook.** Deep-and-many-bucketed versus
   shallow-and-single-bucketed. Two books of opposite shape give the forward plan a **range**
   rather than one blended rate — which is the difference between a projection and a guess.

**`power` stays costed, not built.** All 421 of its units sit inside `ultimate_psionics`, a
3,498-unit book with all eight non-DONE buckets occupied. Building the table would not close
that book, so there would be no banked book to prove the work.

## 7. Success — three things, in priority order

**S1 — The atlas is exhaustive and trustworthy.** Every one of 49,438 units carries exactly
one named remaining-step. `unclassified == 0`, enforced. Every bucket names the mechanism
that clears it. **This is the deliverable that must not be short.**

**S2 — Two books reach zero remaining steps**, each with a per-unit manifest a scan re-derives
by sample: the **Core Rulebook** (6,701 of 6,701, deep, every bucket) and **Ultimate Campaign**
(265 of 265, shallow, effectively one bucket).

**S3 — The remaining non-DONE units across the other 35 books are priced** using rates measured
in S2, per bucket per book — with the `power` table costed individually and every projection
naming the rate and sample size behind it.

**The 25%-of-37-books target is withdrawn as a success criterion** and replaced by S3. The
operator named it negotiable; a book count measures the wrong thing. A priced, exhaustive
forward plan is what makes the next bundle dispatchable — and banked books are a by-product
that Epic 5 reports, not the bar.

## 8. What SD-34 must tell us, explicitly

Named deliverables, each with a criterion:

1. **Every remaining step type**, with its unit count and denominator (AT-34-E1-001).
2. **Every engine table that must be built**, with its population **and which books it
   unblocks** (AT-34-E1-003). That book-coverage map is what found Ultimate Campaign.
3. **The measured cost of clearing each bucket**, from two real books of opposite shape
   (AT-34-E3-004, AT-34-E4-003), plus the measured cost of building a table (AT-34-E2-003).
4. **A per-book, per-bucket forward plan** for all 35 remaining books, cheapest-first, with
   single-bucket books flagged by name (AT-34-E5-001, AT-34-E5-004).
5. **Any capability that must be built and does not exist yet** — beyond the `power` table —
   discovered while completing the two books (AT-34-E5-002).

Item 5 is the "three more things" guard: anything Epic 3 discovers that the Epic 1 atlas did
not predict is a **defect in the atlas**, is recorded as one, and forces the atlas to be
re-derived. The atlas is not allowed to be quietly wrong.

## 9. Explicitly out of scope

- **A second game system.** No case for it while 37,173 of 49,438 units are open in the first.
- **Building all nine tables.** Eight are built — the seven the Core Rulebook exercises plus
  `trait` from Ultimate Campaign. Only **`power`** is costed rather than built, with its reason
  stated (`decisions.md §7`). Costing it is a deliverable, not a deferral.
- **Whether PCGen is right where it diverges from print.** SD-34 treats the pinned oracle as
  ground truth, as SD-33 did.

## 10. Cross-references

- `../SD-33-computed-value-verification/` — ingestion, shape engines, oracle harness.
- `./decisions.md` — the atlas rule, the bucket definitions, and the scope rulings above.
- `./epic-breakdown.md` — the acceptance criteria.
- `./content-unit-inventory.md` — the re-derive command for every figure in this document.

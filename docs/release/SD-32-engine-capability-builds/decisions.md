# SD-32 Decisions

## Decision 1 — Package created by split from SD-30; binding rules inherited verbatim (2026-08-14, operator ruling)

**Status:** New. Operator ruling, 2026-08-14, verbatim: "ok, let's split phase 3 and phase 4 into
their own SD's. SD-31 and SD-32. Take the existing SD-31 and rename it to SD-33."

**Number-reuse note.** This is not the same package as the deleted
`SD-32-instrument-coverage-and-consumer-wiring` (deleted 2026-08-14,
`SD-30-class-feature-archetype-bundle/decisions.md §50`, pre-deletion SHA
`b88b18fa3700125f992e67b0ae29e1d5b70de3c0`). The number was reused the same day by the same operator's
direction to name this package instead. See `README.md`'s disambiguation note.

**What moved here, from `SD-30-class-feature-archetype-bundle/`:** Epic 12 (race chassis) and Epic 13
(verdict-path capability), renumbered Epics 1 and 2 in this package's own `epic-breakdown.md`/
`kanban.md`. This package also carries its own scoped copy of SD-30's former Epic 14 (cloud fan-out
protocol), renumbered Epic 3, limited to this package's own capability-build lane shapes.
`SD-30-class-feature-archetype-bundle/decisions.md §51` records the split from the origin side; this
decision records it from here.

**What did not move:** SD-30's Epic 0 (instrument-apply), Epic 1 (identifier cleanup), Epic 2
(pre-launch), Epic 3 (PI-screening gate), Epic 7 (version numbering), Epic 8 (bundle code review), Epic
9 (closure). Also not moved here: `SD-31-corpus-closure-grind`'s ingest/onboarding epics — this package
builds capability, it does not run ingest cycles against it.

**Rules reproduced below, verbatim, load-bearing on every cycle in this package from here forward**
(copied rather than cited-by-reference — mirrors `SD-31-corpus-closure-grind/decisions.md` Decision 1's
identical reasoning):

### (a) The anti-gaming rule

Reproduced exactly as it stands in `SD-30-class-feature-archetype-bundle/decisions.md §50(a)`
(originally the deleted SD-32 package's Decision 1):

> **THE ONE RULE THAT OVERRIDES EVERYTHING ELSE: YOU MAY NOT MOVE A NUMBER BY LOWERING THE BAR.**
>
> The operator's directive is "improve our numbers, assuming the measuring systems are accurate." That
> second clause is a constraint, not a licence: the instruments are to be trusted and EXTENDED, never
> tuned to flatter the result. Every one of the following is forbidden, and doing any of them makes
> this work worse than not doing it:
>
> - Reclassifying a unit into an easier wiring_class so it clears a lower bar.
> - Loosening, skipping, #[ignore]-ing or special-casing a check so more units pass.
> - Marking a unit done on evidence weaker than its class actually requires.
> - Counting 'held' as done. SD-29 decisions.md §46.4 deliberately does NOT count it, and the
>   doneness_meaning text says so explicitly: "As done as the current instruments can prove, and
>   deliberately not counted as done."
> - Widening a bucket definition, or editing doneness_meaning, to make a bucket look better.
> - Ingesting fixture data, or hand-authoring rules data, to satisfy a check.
>
> This program has spent three days learning that a green instrument over an empty screen is worse
> than a red one. A number that moved because the bar moved is a lie told to the operator in the one
> artifact they use to judge progress. If a unit cannot legitimately reach its bar, LEAVE IT and say
> why. Reporting "fewer moved than hoped, honestly" is a success. If you ever find yourself editing a
> threshold, a classifier, or a definition to make a count rise, STOP and report it instead.

### (b) The classifier accuracy-not-movement rule (LOAD-BEARING, binds Epic 2 directly)

Reproduced exactly as it stands in `SD-30-class-feature-archetype-bundle/decisions.md §50(c)`
(originally the deleted SD-32 package's Decision 3):

> **Decision.** The classifier that resolves `ambiguous` (360 units) and re-examines
> `display`+`grounded` (1,416 units) is accepted or rejected on **agreement with a hand-labelled
> sample**, and on nothing else.
>
> 1. **The gate that runs first.** A sample of at least 100 units, stratified across the five
>    wiring classes and across at least four kinds, is hand-labelled from the corpus record — the whole
>    record, not a field-filtered grep — **before** the classifier is written. The labels are
>    committed. The labeller records the token evidence for each label.
> 2. The classifier's acceptance criterion is its **agreement rate against that held-out sample**,
>    reported per class and per kind, plus its full confusion matrix. There is no target count of
>    units moved anywhere in this epic's acceptance.
> 3. **Movement is reported in both directions.** A classifier that reclassifies 180 units into
>    `computed` and 400 units out of `computed` into `static` reports both, and its net effect on
>    `done` may be **negative**. That is a **passing** outcome. A classifier that only ever moves units
>    toward the two `done`-producing cells is presumptively wrong and must be re-examined before its
>    output is accepted.
> 4. If the labelling gate's sample shows the current classifier is substantially correct and any
>    `display`+`grounded` contradiction is real but rare, the reclassification pass is **not
>    dispatched**, this epic closes at the labelling gate, and the affected units are reported as
>    "examined, correctly classified, left alone." That is `COMPLETE`.
>
> **Rationale.** This lever is ranked #2 by ceiling and #1 by gaming risk. Under (a)'s first forbidden
> item — "reclassifying a unit into an easier wiring_class so it clears a lower bar" — a classifier is
> exactly the instrument that could do that at scale while looking principled. The defence is that the
> classifier is judged against ground truth established *before* anyone knows which way it moves the
> count.

### (c) The table-sheet doneness doctrine

Reproduced exactly as it stands in `SD-30-class-feature-archetype-bundle/decisions.md §49` — see
`SD-31-corpus-closure-grind/decisions.md` Decision 1(b) for the full verbatim text (not duplicated a
third time in this repo; cite either copy, they are byte-identical by construction).

**Operative consequence for this package:** Epic 2's verdict-path design question is "can the sheet
print the true end rule for this character," not "can the engine compute the outcome" — this is the
design bar for what counts as a "real, non-placeholder verdict," not merely a classifier-accuracy bar.

### (d) The concurrency/cloud fan-out protocol

Reproduced from `SD-30-class-feature-archetype-bundle/decisions.md §47` — same hardware baseline,
concurrency cap, and cloud-agent rules as `SD-31-corpus-closure-grind/decisions.md` Decision 1(d).
Governs this package's Epic 3.

**Authority:** operator ruling, 2026-08-14, transcribed in the dispatch brief for this split;
`SD-30-class-feature-archetype-bundle/decisions.md §43-§51` (the widened-charter, 100%-mandate, and
split decisions this package's scope descends from).

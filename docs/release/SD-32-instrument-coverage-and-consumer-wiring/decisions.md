---
canonical: true
owner: god-emporer
status: planning-ready (2026-08-13)
date: 2026-08-13
companion_to: ./scope-draft.md
---

# SD-32 Decisions

## Decision 1 — The anti-gaming rule (LOAD-BEARING, VERBATIM)

**Status:** Binding on every cycle in this bundle, from launch, without
exception. Reproduced verbatim from the execution brief that authorised this
package (operator directive, 2026-08-13). This is the constraint the bundle
exists under; every other decision in this file is subordinate to it.

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

**Operative consequences for this bundle:**

1. Every acceptance criterion in `epic-breakdown.md` and
   `acceptance-and-verification.md` is phrased as **"units legitimately reach
   their existing bar"** — never "the count rises." A criterion that names a
   target count is malformed and must be rewritten before the cycle proceeds.
2. A cycle receipt reports units moved **and** units examined-and-left-alone,
   with the reason for each class of the latter. The second number is evidence
   of compliance, not of failure.
3. A cycle that ends with fewer units moved than its epic's ceiling, and a
   correct account of why, is **COMPLETE**, not `BLOCKED`.
4. `held` is never reported as `done`, never aggregated with `done`, and never
   described as "effectively done" in any receipt, progress entry or release note.

## Decision 2 — The verdict table has no `done` rung for `static`/`derived` (MEASUREMENT GATE)

**Status:** OPEN — decision request to the operator / dashboard owner. Gates
epics E5 and E6. No cycle may act on it unilaterally.

**Finding, re-derived.** The producer's `doneness_verdict()` table maps
`static` and `derived` to `held` for **every** status it accepts
(`ingested-magnitude`, `grounded`, `text-complete`). Only two cells in the whole
table produce `done`: `display`+`text-complete` and `computed`+`grounded`.
7,479 held units are `static` or `derived`
(`artifacts/derive-movable-mass.py`, "static/derived held by kind").

**Consequence.** The corpus-literal byte-equality sweep and the
evaluator-vs-fixture check — the two instruments the dashboard's own
`doneness_meaning` names as the missing checks for `static` and `derived` — can
be built exactly to specification, run green over the whole corpus, and move
**zero** units on the board. There is no rung for their result to land on.

**What would be needed.** Two coordinated changes, neither of them inside this
bundle's write scope:

1. The generator (`src/bin/v06_work_inventory.rs`) emits a **new, strictly
   stronger status word** for a unit whose corpus literal was byte-compared, or
   whose evaluator was checked against a fixture, and passed — e.g.
   `literal-verified` / `fixture-verified`. It is a new word precisely so it
   cannot be confused with `grounded`, which means something else.
2. The producer's verdict table gains a rule mapping `static`/`derived` + that
   word to `done`, and `doneness_meaning` gains the sentence describing it.

**Why this is not a §1 violation, and why a cycle still may not do it.** Adding
a rung **above** `held`, reachable only by evidence that does not exist today,
is the opposite of lowering a bar: it raises the ceiling for units that clear a
*new and stricter* check. Nothing already `held` becomes `done` without that new
evidence. But it is still a change to the artifact the operator uses to judge
progress, made by the party being judged — so it is requested, in writing, with
this reasoning, and the operator or dashboard owner makes it. **A cycle that
edits the producer to do this has violated `§1` regardless of the reasoning
above.**

**Until this is answered:** E5 and E6 stay `BLOCKED` on the kanban. Their work
is real and their instruments are worth building on the merits — but this
package does not pretend they move the number.

## Decision 3 — The wiring-class classifier is accepted on accuracy, not on movement

**Status:** Binding on epic E4.

**Decision.** The classifier that resolves `ambiguous` (360 units) and
re-examines `display`+`grounded` (1,416 units) is accepted or rejected on
**agreement with a hand-labelled sample**, and on nothing else.

1. **E4-F1 runs first and is a gate.** A sample of at least 100 units, stratified
   across the five wiring classes and across at least four kinds, is
   hand-labelled from the corpus record — the whole record, not a field-filtered
   grep — **before** the classifier is written. The labels are committed. The
   labeller records the token evidence for each label.
2. The classifier's acceptance criterion is its **agreement rate against that
   held-out sample**, reported per class and per kind, plus its full confusion
   matrix. There is no target count of units moved anywhere in E4's acceptance.
3. **Movement is reported in both directions.** A classifier that reclassifies
   180 units into `computed` and 400 units out of `computed` into `static`
   reports both, and its net effect on `done` may be **negative**. That is a
   **passing** outcome. A classifier that only ever moves units toward the two
   `done`-producing cells is presumptively wrong and must be re-examined before
   its output is accepted.
4. If E4-F1's sample shows the current classifier is substantially correct and
   the `display`+`grounded` contradiction is real but rare, E4-F2 is **not
   dispatched**, E4 closes at F1, and the 1,776 units are reported as
   "examined, correctly classified, left alone." That is `COMPLETE`.

**Rationale.** This lever is ranked #2 by ceiling and #1 by gaming risk. Under
§1's first forbidden item — "reclassifying a unit into an easier wiring_class so
it clears a lower bar" — a classifier is exactly the instrument that could do
that at scale while looking principled. The defence is that the classifier is
judged against ground truth established *before* anyone knows which way it moves
the count.

## Decision 4 — Probe coverage extension is a coverage change, not a bar change

**Status:** Binding on epic E2.

**Decision.** `probe_equipment_effect_wiring()` currently builds its key set
from four compiled equipment tables (`crb`, `apg`, `acg`, `beastiary1`) and
loads corpus from six `OBSERVABLE_BOOK_DIRS`; eleven books have a compiled
`equipment_tables.rs`. E2 widens both to the full compiled set.

**`equipment_key_is_wired()` is not touched.** The predicate — equip this item
alone, against the real corpus, and observe at least one non-`None` mechanical
stat effect from `compute_equipment_effects` — stays byte-identical. Items the
widened probe examines and finds inert stay `ingested-magnitude`, correctly, and
E2's receipt reports how many did.

**Why this is §1-compliant.** The bar is unchanged; it is applied to units it
never previously examined. Widening the population a fixed test runs over is
coverage. Weakening the test so more of the existing population passes is
gaming. E2 does the first and its diff must show `equipment_key_is_wired`'s body
unmodified.

## Decision 5 — `spell` is reported as structurally blocked and is not worked for the numbers

**Status:** Binding on the whole bundle.

**Decision.** All 1,281 held `spell` units are reported as bucket C,
structurally unreachable, with the reason on the record. No epic in this bundle
attempts to move them.

**Evidence.** `spell` `grounded` corpus-wide is **0**, by construction:
`classify()`'s `Kind::Spell` arm cannot return `grounded` — "no currently-wired
consumer reads a spell's magnitude, so every resolved-level spell stays
`ingested-magnitude`." A spell's only `done` cell is `display`+`text-complete`,
occupied by exactly one spell.

Building a spell consumer-delta probe would move 178 units from `held` to
`in-progress` — a *worse*-looking bucket — and none to `done`, because reaching
`done` needs a real consumer that reads a spell magnitude, i.e. spellcasting as
product work. That is a product decision, recorded at
`forward-scope-register.md F2`, not a numbers lever.

## Decision 6 — `companion`'s `NO_GROUNDING_PROBE` listing is stale; report it, do not act on it

**Status:** Reported to the dashboard owner. No code change in this bundle.

**Finding.** The producer lists `companion` in `NO_GROUNDING_PROBE` on the
stated grounds that "`companion` and `spell` alone read `grounded: 0`." The
current payload carries **922 grounded companion units**, 416 of them
`computed`+`grounded` and already counted `done`. The justification is stale.

**But the cap moves zero companion units.** The cap only fires on units that
would otherwise be `in-progress` (`computed`/`display` + non-`grounded`), and
the corpus has none for `companion`: all 506 held companions are `derived` (270),
`display` (215), `static` (19) or `ambiguous` (2). Removing `companion` from the
tuple would change the board by 0.

**Decision.** Report it (`forward-scope-register.md F5`); change nothing.
Correcting a producer constant that moves 0 units is still a producer edit by
the party being measured, and this bundle does not make those. Recorded here so
that a later reader does not re-derive it as a new finding.

## Decision 7 — Scope is instrument coverage and consumer wiring, not content ingestion

**Status:** Binding.

`not-started` (21,303 units) is content that is not in the engine. Moving it is
book-ingestion work and belongs to the SD-29/SD-30 lanes, not here. This bundle
touches only units the engine already holds.

## Decision 8 — Every figure ships with the command that produced it

**Status:** Binding on every cycle receipt in this bundle.

Per `AGENTS.md` ("A number in a brief ships with the command that produced it,
or it does not ship — not the value, the invocation"), and because transcribed
figures are this program's rank-one recorded defect class: every count in a
receipt, a progress entry or a release note carries its invocation.
`artifacts/derive-movable-mass.py` is the canonical one for movable-mass
figures, and it self-validates against the live payload before printing.

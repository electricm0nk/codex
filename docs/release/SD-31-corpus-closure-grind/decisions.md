# SD-31 Decisions

## Decision 1 — Package created by split from SD-30; binding rules inherited verbatim (2026-08-14, operator ruling)

**Status:** New. Operator ruling, 2026-08-14, verbatim: "ok, let's split phase 3 and phase 4 into
their own SD's. SD-31 and SD-32. Take the existing SD-31 and rename it to SD-33."

**What moved here, from `SD-30-class-feature-archetype-bundle/`:** Epics 4 (per-class measurement), 5
(archetype mechanism), 6 (per-class chassis sweep), 10 (corpus-wide ingest lanes folded from SD-29),
11 (7-book onboarding), and the grind-lane half of Epic 14 (cloud fan-out protocol) — renumbered
Epics 1-6 in this package's own `epic-breakdown.md`/`kanban.md`. See `epic-breakdown.md`'s renumber
map for the exact `SD30-E# → SD31-E#` mapping. `SD-30-class-feature-archetype-bundle/decisions.md §51`
records the split from the origin side; this decision records it from here.

**What did not move:** SD-30's Epic 0 (instrument-apply), Epic 1 (identifier cleanup), Epic 2
(pre-launch), Epic 3 (PI-screening gate), Epic 7 (version numbering), Epic 8 (bundle code review),
Epic 9 (closure). SD-30 retains ownership of the PI-screening gate; this package's ingest epics
(3, 4, 5) are hard-gated on it as a cross-SD dependency, not a co-owned surface.

**Rules reproduced below, verbatim, load-bearing on every cycle in this package from here forward**
(copied rather than cited-by-reference, because a cross-SD reference that outlives the origin
package's own scope narrowing is a maintenance hazard this program has already been burned by once —
see `SD-30-.../decisions.md §50`'s own reasoning for absorbing SD-32's rules the same way):

### (a) The anti-gaming rule

Reproduced exactly as it stands in `SD-30-class-feature-archetype-bundle/decisions.md §50(a)`
(originally SD-32 `decisions.md` Decision 1):

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

### (b) The table-sheet doneness doctrine

Reproduced exactly as it stands in `SD-30-class-feature-archetype-bundle/decisions.md §49` (operator
ruling, verbatim):

> I would rather bring SD-32 into the SD-30 scope. I think the gist of what I was
> saying with "done rung for static and derived" was basically that some things do
> not require computation. If a fireball is 1d6 per spellcaster level - you don't
> need to compute 6d6 for a 6th level caster - you need to display that the
> fireball spell is 6d6 because the character in question is 6th level. That's
> just printed in the character sheet. The actual rolling of 6d6 happens on a
> table, with dice, and the additions are added by the player's brain. Our goal
> here is to print a character sheet that the user can use at the table - we are
> not making a video game. So in many cases we just need to expose the end rule -
> once we can do that it's done. If a spell says 1d6 per character level, you just
> need to be able to determine the character level and say the true value when the
> character sheet is created.

**Operative consequence for this package:** SD-31's Epic 1/2/3 (`class_feature`) acceptance is governed
by this bar directly — a feature is accepted once its rule is resolved to a true, character-specific
value and displayed, not once its mechanism is internally simulated. A resolved value must still be
**true**: `static` values pass byte-equality against the corpus literal, `derived` values pass
evaluator-vs-fixture verification, and the value must be **displayed** — reach/on-screen verification
still applies. This does not relax the anti-gaming rule at (a).

### (c) The PI-gate hard-block (cross-SD dependency, not a copy of the gate itself)

This package does not own or re-implement the PI-screening provenance gate — that stays
`SD-30-class-feature-archetype-bundle`'s Epic 3. What is binding here: **no cycle in this package's
Epic 3 (chassis sweep), Epic 4 (ingest lanes), or Epic 5 (book onboarding) may claim a book before that
book's declared-PI screen (SD30-E3-F2, the `NAMEISPI`/`DESCISPI` reader, and SD30-E3-F3, the corpus-wide
backfill sweep) shows `COMPLETE` for that book in SD-30's own `progress.md`.** This mirrors exactly how
SD-30's own Epic 6/Epic 10 were gated before the split (`SD-30-.../decisions.md §39`, `§44` lesson 4) —
the fold from SD-29 widened which kinds' ingest is subject to the gate, it never relaxed it, and moving
those epics to a sibling package does not create a bypass either.

### (d) The concurrency/cloud fan-out protocol

Reproduced from `SD-30-class-feature-archetype-bundle/decisions.md §47`: hardware baseline (8 cores /
45GB RAM / 968GB disk at 19% used, captured 2026-08-14), concurrency cap of three concurrent
build-capable agents (re-derived empirically per the existing budget discipline), and the cloud
fan-out rules — every cloud agent works its own branch, never two writers on one branch; the local
orchestrator owns all merges to `tranche/10`, verified by content not commit count; DoD-8 on-screen
verification and dashboard-producer work stay local. Governs this package's Epic 6.

**Authority:** operator ruling, 2026-08-14, transcribed in the dispatch brief for this split;
`SD-30-class-feature-archetype-bundle/decisions.md §43-§51` (the widened-charter, 100%-mandate, and
split decisions this package's scope descends from).

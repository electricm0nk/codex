---
canonical: true
owner: god-emporer
status: planning-ready (2026-08-13)
date: 2026-08-13
---

# SD-32 Forward Scope Register

Work this bundle deliberately does not take, with the reason and the successor
condition. An entry here is a decision, not an oversight.

---

## F1 — The 3,547 `unmeasurable` units

**Mass:** 3,218 `class_feature` + 329 `feat`. `status` came back `unknown`: the
record gives no evidence at all, so there is nothing to place on the ladder.

**Command:**
`python3 docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/derive-movable-mass.py`
→ final section.

**Why not here.** They are not `held` and not `in-progress`, so they are outside
this bundle's movable mass by definition. The dashboard's own `doneness_meaning`
calls them "a work item against the instruments, not a statement about the
content."

**Why it may be the better bundle.** 3,547 units is larger than everything E2,
E3 and E4 can move combined (ceiling 2,509). Every one of them is a unit the
instruments **failed to read**, not a unit the content failed to satisfy — which
makes it the purest instrument-coverage work on the board and the least exposed
to the gaming risk that dominates this bundle. Recorded as `risks-and-open-questions.md Q4`.

**Successor condition:** its own bundle, characterising the `unknown` reasons by
frequency before any fix is attempted.

---

## F2 — A spell consumer-delta probe

**Mass:** would move 178 `spell` units from `held` to `in-progress`, and 0 to
`done`.

**Why not here.** `decisions.md §5`. `classify()`'s `Kind::Spell` arm cannot
return `grounded` because no wired consumer reads a spell's magnitude. Building
the probe without building the consumer moves units into a *worse*-looking
bucket for no gain — and building the consumer is spellcasting as product work,
which is a product decision, not a numbers lever.

**Successor condition:** spellcasting becomes product scope. Then the probe is
built as part of it, and the 178 become genuinely `in-progress` on the way to
`grounded`.

---

## F3 — The `static` byte-equality sweep and the `derived` evaluator-vs-fixture check, if the gate is declined

**Mass:** 7,479 held units (4,805 `static`, 2,674 `derived`).

**Why conditionally not here.** E5 and E6 are in this bundle but `BLOCKED
(decision)` on `decisions.md §2`. If the operator declines the `done` rung, both
epics move to this register rather than being attempted: the instruments would
still be worth building on their engineering merits, but they would move zero
units on the board and this bundle must not pretend otherwise.

**Successor condition:** `decisions.md §2` answered yes, in writing, by the
operator or dashboard owner.

---

## F4 — Effect shapes E3 does not take

**Mass:** the remainder of the 375 bucket-A1 units after E3-F2 takes the largest
shapes.

**Why not here.** E3-F1 groups the inert items by effect shape; E3-F2 takes the
ones whose wiring is bounded. Shapes that need substantial new engine capability
are left, listed, with the reason. Leaving a shape is a **COMPLETE** outcome
(`epic-breakdown.md` E3-F2).

**Successor condition:** the shape becomes product scope in its own right.

---

## F5 — `companion`'s stale `NO_GROUNDING_PROBE` listing

**Mass:** 0 units. The cap fires on nothing for `companion`.

**Finding.** The producer lists `companion` in `NO_GROUNDING_PROBE` on the
grounds that it reads `grounded: 0`. The current payload carries **922 grounded
companion units**, 416 of them already `done`. The justification is stale.

**Why not here.** `decisions.md §6`. It is a producer edit, and this bundle does
not make producer edits — not even correct ones that move nothing. Reported to
the dashboard owner so a later reader does not re-derive it as new work.

**Successor condition:** the dashboard owner's call. If they act, expect the
board to be unchanged, which is itself the confirmation that the derivation was
right.

---

## F6 — `not-started` (21,303 units)

**Why not here.** `decisions.md §7`. That is content the engine does not hold —
book-ingestion work belonging to the SD-29/SD-30 lanes. This bundle touches only
units the engine already holds.

**Successor condition:** the existing ingestion lanes; no new successor needed.

---

## F7 — A second reviewer for E8

**Why recorded.** `risks-and-open-questions.md R1` residual: E8 reviews a diff
authored under the same pressure that E8 exists to catch. A reviewer that did
not implement E2/E3/E4 is materially stronger.

**Successor condition:** operator or orchestrator assigns one at E8 dispatch.

---
canonical: true
owner: god-emporer
status: planning-ready (2026-08-13)
date: 2026-08-13
companion_to: ./scope-draft.md, ./decisions.md
---

# SD-32 Epic Breakdown

Nine epics, dependency-ordered. Dispatch order is `scope-draft.md §5`:
E1 → E2 → E4-F1 → E3 → E4 rest → (E5/E6 only if E1 is answered yes) → E7 → E8 → E9.

**Every acceptance criterion below is phrased as "units legitimately reach their
existing bar."** None names a target count. `decisions.md §1.1` makes a
count-target criterion malformed by construction.

---

## Epic 1 (E1) — Measurement-Gate Decision Request

**Objective:** Put `decisions.md §2` in front of the operator with the derivation
attached, so the 7,479 `static`/`derived` held units are either unblocked or
formally recorded as unreachable. Writes no code.

**Ceiling:** 0 units directly. Gates 7,479.

### E1-F1 — Author and surface the request

Acceptance:

- `decisions.md §2` is complete: the finding, the two coordinated changes it
  would need, the reason it is not a `§1` violation, and the reason a cycle may
  not make it anyway.
- The receipt carries `artifacts/derive-movable-mass.py`'s output section
  "static/derived held by kind" verbatim, with its invocation.
- The request is delivered to the operator through the normal surface. **No file
  under `/home/ubuntu/swarm-observer/` or the producer skill directory is
  modified** — verified by `git status` on those paths being empty and by the
  cycle's own file-touch audit.
- Outcome recorded either way. "Operator declines; 7,479 units are permanently
  `held` under the current model" is a **COMPLETE** outcome for this epic.

---

## Epic 2 (E2) — Equipment-Effect Probe Coverage Extension

**Objective:** Apply the existing, unchanged grounding bar to the 358 units the
probe's key universe never examines.

**Ceiling:** 358 units. **Rank 1 (R = 358/epic), highest confidence in the bundle.**

**Derived from:** `decisions.md §4`; `scope-draft.md §4` bucket A2.

### E2-F1 — Widen the key universe and the corpus roots

Acceptance:

- `probe_equipment_effect_wiring()` builds its key set from every compiled
  `src/rules_core/rules_tables/<book>/equipment_tables.rs`, enumerated from the
  module tree rather than hand-listed, so a twelfth book cannot be silently
  omitted. `OBSERVABLE_BOOK_DIRS`' equipment role widens to match.
- **`equipment_key_is_wired()`'s body is unmodified.** Proven by diff, not
  asserted. The bar is byte-identical.
- A RED test first: a test that asserts the probe examines a key from a book
  outside the current four fails before the change and passes after.
- Every unit that becomes `grounded` does so because equipping it alone produced
  a real non-`None` mechanical stat effect — the same evidence `core_rulebook`'s
  37 already-`done` equipment units have. **Units that produce no effect stay
  `ingested-magnitude`**, and the receipt reports that count explicitly.
- Count-pin sweep run before commit: old and new counts grepped across `tests/`,
  `src/`, `apps/` (`docs/retro` memory: a count change compiles clean and leaves
  other files' hardcoded assertions red).
- `./scripts/verify.sh` FULL, exit code captured directly, not through a pipe.

### E2-F2 — Report the honest yield

Acceptance:

- Receipt states: units newly examined, units that legitimately reached
  `computed`+`grounded`, units examined and correctly left at
  `ingested-magnitude`, with the invocation for each.
- If the yield is low, that is the finding. `decisions.md §1.3`.

---

## Epic 3 (E3) — Equipment Effect Wiring for Examined-but-Inert Items

**Objective:** Make the 375 items the probe already examines actually do
something mechanically, so a real consumer delta exists to observe.

**Ceiling:** 375 units (233 `equipment_modifier` + 142 `equipment`; 216 of the
modifiers are `core_rulebook`). **Rank 3 (R ≈ 125/epic).**

**This is product work, not instrument work** — the most literal reading of "move
the underlying reality the dashboard measures."

### E3-F1 — Characterise the inert set

Acceptance:

- The 375 units are grouped by the **effect shape their corpus record carries**
  (enhancement bonus, resistance, situational skill bonus, …), with counts per
  shape and the command that produced them.
- The grouping distinguishes "the engine has no code for this shape" from "the
  engine has the code and this item is not routed to it." These need different
  work and conflating them has burned this program before.

### E3-F2 — Wire the largest shapes, RED first

Acceptance:

- For each shape taken: a failing test asserting the mechanical effect the corpus
  record specifies, then the smallest change that passes it.
- A unit reaches `grounded` **only** through `compute_equipment_effects`
  returning a real value for the real corpus record. No fixture data, no
  hand-authored rules rows (`decisions.md §1`, no-stub doctrine).
- Shapes deliberately not taken are listed with the reason. Leaving a shape is a
  **COMPLETE** outcome.

### E3-F3 — Verify no regression in the already-`done` set

Acceptance:

- The 37 `equipment` + 40 `equipment_modifier` units already at
  `computed`+`grounded` are still `grounded` afterwards, re-derived.

---

## Epic 4 (E4) — Wiring-Class Classifier over the Full Token Closure

**Objective:** Give 1,776 units a bar they do not currently have — 360
`ambiguous`, 1,416 `display`+`grounded`.

**Ceiling:** 1,776 units. **Rank 2 by ceiling (R ≈ 444/epic); highest gaming risk
in the bundle.** Governed end-to-end by `decisions.md §3`.

### E4-F1 — Hand-labelled calibration sample (GATE — runs before any classifier code)

Acceptance:

- ≥100 units hand-labelled from the **whole corpus record**, stratified across
  the five wiring classes and ≥4 kinds, labels committed with the token evidence
  for each.
- Labelling happens **before** the classifier is written, so no label is
  informed by which way it moves the count.
- The current classifier's agreement rate against the sample is reported per
  class and per kind, with the confusion matrix.
- **Gate:** if the current classifier substantially agrees with the sample, E4-F2
  is **not dispatched**, E4 closes here, and the 1,776 units are reported as
  "examined, correctly classified, left alone." That is **COMPLETE**
  (`decisions.md §3.4`).

### E4-F2 — Classifier over the full token closure (dispatched only if F1 opens the gate)

Acceptance:

- The classifier decides `wiring_class` from the full token closure GE-01
  defines — base row unioned with its mod rows — not a single-row magnitude
  check. `token_closure_rows()` in `src/bin/v06_work_inventory.rs` already exists
  and is the starting surface.
- Acceptance is **agreement with the held-out sample**, reported per class and
  per kind. No count of units moved appears anywhere in this criterion.
- Movement reported in **both** directions, with the net effect on `done` stated
  even when negative. A negative net is a **passing** outcome.
- A classifier whose reclassifications run only toward the two `done`-producing
  cells is presumptively wrong and is re-examined before its output is accepted
  (`decisions.md §3.3`).

### E4-F3 — Resolve the `display`+`grounded` contradiction on the evidence

Acceptance:

- For each of the 1,416, the receipt says which of three things is true: the unit
  is genuinely `computed`-shaped and the classifier was wrong; the unit is
  genuinely `display` and the `grounded` evidence is a probe artefact; or it is
  undecidable on the available evidence and stays `held`.
- The third answer is legitimate and expected for some fraction. Forcing a
  decision to move a unit violates `decisions.md §1`.

---

## Epic 5 (E5) — Static Corpus-Literal Byte-Equality Sweep

**Status:** `BLOCKED` on E1. **Do not dispatch until `decisions.md §2` is answered yes.**

**Objective:** Build the check the dashboard's `doneness_meaning` names as
`static`'s missing instrument: every `static` unit's engine value byte-equals its
corpus literal.

**Ceiling:** 4,805 `static` held units (4,511 of them `equipment`). R = 1,602/epic
**if** the gate opens; **0 today**.

### E5-F1 — The sweep

Acceptance:

- For each `static` unit, the engine's held value is compared byte-for-byte
  against the corpus literal, over the real corpus.
- Mismatches are **reported as defects**, not tolerated and not normalised away.
  A sweep that passes because it compares normalised forms has lowered the bar
  (`decisions.md §1`).
- Units the sweep cannot reach are named, with the reason, not silently skipped.

### E5-F2 — Emit the new status word (requires E1 = yes)

Acceptance:

- The generator emits the new strictly-stronger status word for units that pass,
  exactly as sanctioned in E1. A unit that did not pass does not receive it.
- The producer-side mapping is made **by the dashboard owner**, never by a cycle.

---

## Epic 6 (E6) — Derived Evaluator-vs-Fixture Check

**Status:** `BLOCKED` on E1. Same gate as E5.

**Objective:** Build `derived`'s missing instrument: the evaluator's output
checked against a fixture that encodes the rule the corpus states.

**Ceiling:** 2,674 `derived` held units (`monster` 1,229; `spell` 938 — E6 is the
**only** path that ever unblocks those spell units, since their bar is
evaluator-vs-fixture, not consumer-delta; `companion` 270). R = 535/epic if the
gate opens; **0 today**.

### E6-F1 — Fixtures encode the corpus rule, not the engine's output

Acceptance:

- Each fixture's expected value is derived from the **corpus record**, by a
  human or a documented derivation, and is committed **before** the evaluator is
  run against it.
- A fixture generated from the evaluator's own output is worthless and is the
  precise shape of `decisions.md §1`'s "green instrument over an empty screen."
  Any fixture whose provenance is not the corpus is rejected in review.

### E6-F2 — Emit the new status word (requires E1 = yes)

Acceptance: as E5-F2.

---

## Epic 7 (E7) — Structural-Block Report

**Objective:** Put the three structural findings on the permanent record so a
later reader does not re-derive them as new work.

**Ceiling:** 0 units, deliberately.

Acceptance:

- `spell`'s 1,281 held units are recorded as bucket C with the `classify()`
  citation (`decisions.md §5`).
- `companion`'s stale `NO_GROUNDING_PROBE` listing is reported to the dashboard
  owner, with the derivation showing it moves 0 units and the reason no cycle
  changes it (`decisions.md §6`).
- The 3,547 `unmeasurable` units are handed to `forward-scope-register.md F1`
  with their kind split.

---

## Epic 8 (E8) — Bundle Code Review

Acceptance:

- Full-bundle diff review against the branch point.
- The review's **first** check is `decisions.md §1` compliance: no threshold, no
  classifier definition, no bucket definition, and no check predicate was
  weakened anywhere in the diff. `equipment_key_is_wired()`'s body unmodified;
  no `#[ignore]` added; no assertion loosened; no file under
  `/home/ubuntu/swarm-observer/` or the producer skill directory touched.
- Four-check no-stub audit per `AGENTS.md` §6.

---

## Epic 9 (E9) — Closure

Acceptance:

- `progress.md` carries a receipt for every dispatched card.
- `release-notes.md` reports units moved **and** units examined-and-left-alone,
  with the invocation for each, and states the honest shortfall against
  `scope-draft.md §6`'s ceiling without rounding it away.
- Living architecture docs refreshed (template §6).
- Tranche promotion PR opened.

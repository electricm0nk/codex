---
canonical: true
owner: god-emporer
status: planning-ready (2026-08-13)
date: 2026-08-13
companion_to: ./epic-breakdown.md, ./decisions.md
---

# SD-32 Acceptance Tests

Given/When/Then, one per criterion, paired with `epic-breakdown.md`.

**Every criterion is phrased as "units legitimately reach their existing bar."**
None is phrased as "the count rises." `decisions.md §1.1` makes a count-target
criterion malformed; AT-32-001 is the check that enforces it across the package.

---

## AT-32-001 — No criterion in this bundle is a count target

**Given** any acceptance criterion in this package or in a cycle receipt.

**When** it is read.

**Then** it states a property units must satisfy to reach their **existing**
bar, and it does **not** state a number of units that must move. A criterion of
the form "at least N units reach `done`" fails this test and is rewritten before
the cycle proceeds.

**Evidence:** E8's review pass, first check.

---

## AT-32-002 — The bar is unmodified

**Given** the full bundle diff against the branch point.

**When** E8 reviews it.

**Then** all of the following hold:

- `equipment_key_is_wired()`'s body is byte-identical to the branch point.
- No `#[ignore]`, no `#[cfg(ignore)]`, no skipped suite, and no loosened
  assertion anywhere in the diff.
- No file under `/home/ubuntu/swarm-observer/` or
  `~/.hermes/profiles/god-emporer/skills/release-swarm-observer/` is modified.
- `doneness_verdict()`, `DONENESS_MEANING`, `DONENESS_VALUES`,
  `NO_GROUNDING_PROBE` and `EXCLUDED_BOOKS` are unmodified.
- No `wiring_class` value is assigned by a hand-written exception list.

**Evidence:** `git diff --stat` plus the named path checks, exit codes captured,
in the E8 receipt.

---

## AT-32-003 — E2: coverage widened, bar unchanged

**Given** the equipment-effect probe after E2-F1.

**When** the generator runs over the full corpus.

**Then**:

- The probe's key universe is enumerated from the compiled `equipment_tables.rs`
  module tree, not hand-listed, and covers every book that has one.
- A unit from a book outside the previous four is examined by the probe — proven
  by the RED test that failed before the change.
- Every unit that newly reads `grounded` did so because equipping it alone
  produced a non-`None` mechanical stat effect from `compute_equipment_effects`
  — the identical evidence the 37 `equipment` and 40 `equipment_modifier` units
  already at `computed`+`grounded` carry.
- Units the widened probe examines and finds inert **remain**
  `ingested-magnitude`, and their count is reported.

**Evidence:** the RED test name and its pre-change failure, the diff showing
`equipment_key_is_wired` untouched, and
`artifacts/derive-movable-mass.py` output before and after with both invocations.

---

## AT-32-004 — E3: a unit reaches `grounded` only through the real corpus record

**Given** an equipment or equipment-modifier unit that E3 wires.

**When** the probe equips it alone against the real corpus.

**Then** `compute_equipment_effects` returns a real mechanical value derived from
that unit's **own corpus record** — no fixture row, no hand-authored rules data,
no special-case branch keyed on the unit's name.

**And** the 77 units already at `computed`+`grounded` for these two kinds are
still `grounded` after the change (E3-F3).

**Evidence:** the failing-first test per effect shape; the re-derivation before
and after.

---

## AT-32-005 — E4: the classifier is accepted on accuracy, and only on accuracy

**Given** the hand-labelled sample committed by E4-F1.

**When** the classifier from E4-F2 runs against it.

**Then** the receipt reports the agreement rate per wiring class and per kind,
plus the full confusion matrix, and the accept/reject decision cites **only**
those figures.

**And** the labels were committed **before** the classifier was written — proven
by commit order in the git history, not asserted.

**And** reclassifications are reported in both directions, with the net effect on
`done` stated even when it is negative.

**Evidence:** commit timestamps for the label file versus the classifier file;
the confusion matrix in the receipt.

---

## AT-32-006 — E4: a classifier that only moves units toward `done` is rejected

**Given** E4-F2's reclassification report.

**When** every reclassification runs toward `computed`+`grounded` or
`display`+`text-complete` — the only two cells that produce `done` — and none
runs away from them.

**Then** the output is **not** accepted. The classifier is re-examined against
the sample before any of its output reaches the generator.

**Rationale:** `decisions.md §3.3`. A correct classifier corrects errors in both
directions; a one-directional one is indistinguishable from a bar-lowering
device.

---

## AT-32-007 — E4-F3: "undecidable" is an allowed and expected answer

**Given** a `display`+`grounded` unit E4-F3 examines.

**When** the available evidence does not settle whether it is `computed`-shaped
or the `grounded` reading is a probe artefact.

**Then** the unit stays `held`, the receipt records it as undecidable with the
evidence considered, and the cycle does **not** force a classification.

---

## AT-32-008 — E5: the sweep compares literals, not normalised forms

**Given** the `static` byte-equality sweep (gated: E1 must be answered yes).

**When** it compares an engine value to its corpus literal.

**Then** the comparison is byte-equality against the corpus literal as written.
A sweep that passes by normalising both sides has replaced the bar with a weaker
one and fails this test.

**And** mismatches are reported as defects; units the sweep cannot reach are
named with the reason, not silently skipped.

---

## AT-32-009 — E6: fixture provenance is the corpus, not the evaluator

**Given** any fixture E6 adds (gated: E1 must be answered yes).

**When** its expected value is traced.

**Then** that value came from the corpus record via a documented derivation, and
the fixture was committed **before** the evaluator was run against it — proven by
commit order.

**A fixture generated from the evaluator's own output fails this test outright**
and is removed, not adjusted.

---

## AT-32-010 — `held` is never reported as `done`

**Given** any receipt, progress entry or release note this bundle produces.

**When** it reports doneness.

**Then** `held` appears as its own figure, is never summed with `done`, and is
never described as "effectively done", "as good as done" or equivalent.

**Rationale:** `decisions.md §1`, SD-29 `decisions.md §46.4`, and the dashboard's
own `doneness_meaning`: "As done as the current instruments can prove, and
deliberately not counted as done."

---

## AT-32-011 — Every figure carries its invocation

**Given** any count in a receipt, progress entry or release note.

**When** it is read.

**Then** the command that produced it is printed alongside it — the invocation,
not the value. `artifacts/derive-movable-mass.py` is canonical for movable-mass
figures.

**Evidence:** `decisions.md §8`; `AGENTS.md` "A number in a brief ships with the
command that produced it, or it does not ship."

---

## AT-32-012 — A shortfall closes COMPLETE

**Given** a cycle that moved fewer units than its epic's ceiling.

**When** its receipt accounts for the gap — which units were examined, which were
left alone, and why each class of them could not legitimately reach its bar.

**Then** the card closes **COMPLETE**, not `BLOCKED` and not `partial`.

**Rationale:** `decisions.md §1.3`. "Reporting 'fewer moved than hoped,
honestly' is a success."

---

## AT-32-013 — Count-pin sweep before every commit that changes a count

**Given** a change that alters any record count.

**When** the cycle prepares to commit.

**Then** both the old and the new count have been grepped across `tests/`,
`src/` and `apps/`, and any hardcoded assertion carrying the old value has been
updated in the same commit.

**Rationale:** a count change compiles clean and leaves other files' hardcoded
assertions red.

---

## AT-32-014 — Full verification, exit code captured directly

**Given** any code-bearing cycle in this bundle.

**When** it verifies.

**Then** `./scripts/verify.sh` FULL ran, its exit code was captured directly and
not through a pipe, and the receipt carries that exit code. Every
`test result: FAILED` line is attributed back to its `Running` line and named.
"The N known environmental failures" is not an attribution.

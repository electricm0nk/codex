---
canonical: true
owner: god-emporer
status: planning-ready (2026-08-13)
date: 2026-08-13
---

# SD-32 Risks and Open Questions

## Risks

### R1 — The bundle games its own numbers (SEVERITY: highest)

**Shape.** Every lever here is one small edit away from being a bar-lowering
device. A classifier that "fixes" `display` into `computed` moves units straight
into a `done` cell. A probe predicate loosened by one field admits hundreds. A
fixture generated from the evaluator's own output makes an entire epic green
over nothing.

**Why it is the top risk.** The pressure is structural: the operator asked for
the numbers to improve, the honest ceiling is a few hundred units, and the
dishonest ceiling is several thousand. Nobody has to decide to cheat; it is
enough to take the interpretation that flatters the result at three separate
junctions.

**Mitigations, all already in the package:** `decisions.md §1` verbatim and
binding; `§3` accepts the classifier on pre-committed ground truth only;
`§4` freezes `equipment_key_is_wired()`; AT-32-002 checks the diff for a
weakened bar; AT-32-006 rejects a one-directional classifier; AT-32-009 rejects
evaluator-derived fixtures; AT-32-012 makes an honest shortfall a **passing**
outcome, which removes the incentive at its root.

**Residual.** Non-zero. E8's review is the last line, and it reviews a diff
authored under the same pressure. Consider dispatching E8 to a reviewer that did
not implement E2/E3/E4.

### R2 — The measurement gate is declined, and 73% of the mass stays frozen

**Shape.** If `decisions.md §2` is answered no, 7,479 units are permanently
`held` under the current model and the bundle's realistic ceiling drops to
734 + yield × 1,776.

**Mitigation.** E1 is card #1 precisely so this is known before E5/E6 effort is
spent. A "no" is a **COMPLETE** outcome for E1, recorded, not a failure.

### R3 — E4's yield is unknown and could be near zero

**Shape.** 1,776 units get a bar; how many land in a `done`-producing cell is
unmeasured. If most `display`+`grounded` units turn out to be genuinely `static`
or `derived`, they land in `held` again and E4 moves almost nothing after four
epics.

**Mitigation.** E4-F1 is a cheap gate that measures the yield before the four
epics are spent, and `decisions.md §3.4` lets E4 close at F1.

### R4 — E2's widened probe finds the new books' items inert

**Shape.** The 358 units become examined rather than grounded. Ceiling 358,
realistic yield possibly far lower.

**Mitigation.** None needed — this is the honest outcome and E2-F2 reports it as
such. It is also cheap (one epic), which is why it ranks first: low cost, high
information, and the information is useful either way.

### R5 — A count change leaves other files' assertions red

**Shape.** Recorded defect class in this program. A record-count change compiles
clean and breaks hardcoded assertions elsewhere.

**Mitigation.** AT-32-013; grep old **and** new counts across `tests/`, `src/`,
`apps/` before every such commit.

### R6 — Shared-checkout collisions

**Shape.** The largest incident class of tranche/7 — 10 of 34 — and nothing
caught any of them prospectively.

**Mitigation.** `technical-design.md §4`: one writer per tree, per-agent
`CARGO_TARGET_DIR` per source tree, `git status` before every git write, never
`git add -A`, never `git stash` here at all.

### R7 — The dashboard's verdict table changes underneath the bundle

**Shape.** Every figure in this package is a split of the producer's current
table. If the table changes mid-bundle, the split is stale and the receipts are
wrong.

**Mitigation.** `artifacts/derive-movable-mass.py` asserts its transcription
against the live payload before printing and exits non-zero on divergence. A
cycle whose derivation exits non-zero treats **every** count in its receipt as
void until re-derived.

## Open questions

### Q1 — Does the operator sanction a `done` rung for `static`/`derived`?

`decisions.md §2`. Gates 7,479 units. E1 asks it. **Open.**

### Q2 — What is the classifier's real yield?

Unmeasured until E4-F1. The package deliberately does **not** guess a multiplier
(`scope-draft.md §5`, ranking rule 2). **Open.**

### Q3 — Are the 375 inert equipment items inert for one reason or many?

E3-F1 answers it. If they share a small number of effect shapes, E3's R rises
sharply; if each needs its own wiring, E3 is 375 units of individual work and
should be re-ranked below E4. **Open, and it changes the dispatch order.**

### Q4 — Should the `unmeasurable` 3,547 be this program's next bundle?

3,218 `class_feature` + 329 `feat` whose `status` came back `unknown` — the
largest single instrument defect on the board, and larger than everything E2,
E3 and E4 can move combined. Out of scope here by `scope-draft.md §7`; recorded
at `forward-scope-register.md F1`. **Open, and arguably the better bundle.**

### Q5 — Who reviews E8?

R1's residual argues for a reviewer that did not implement the bundle. **Open.**

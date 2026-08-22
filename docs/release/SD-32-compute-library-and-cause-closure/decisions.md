---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22 from SD-31 session)
date: 2026-08-22
predecessor: SD-31-corpus-closure-grind
---

# SD-32 Decisions

**Status:** Planning-ready. Authored 2026-08-22 during SD-31 wave 31's measurement, before SD-31
closes, so the reasoning is captured while it is fresh.

## Decision 1 — Branch, build version, board (operator ruling 2026-08-22)

**Status:** Operator-pinned.

* **Branch:** `tranche/12` (cut from `tranche/11`'s tip after SD-31's closure PR is merged). The
  `SD-31-corpus-closure-grind/decisions.md §6` rationale — "tranche/N+1 carries tranche/N's full
  history" — applies symmetrically; SD-31's work will be present in `tranche/12`'s history.
* **Build version:** `<major>.<tranche-base>.<build>` per the 2026-07-17 amendment. **`tranche-base`
  is the numeral in the branch name** (per SD-31 decisions §6, which is the canonical pattern this
  bundle inherits). For `tranche/12` that is `12`, so SD-32's first concrete value is
  `0.12.<build_at_launch>`. The `0.11.x` line in `apps/desktop/package.json` /
  `apps/desktop/src-tauri/tauri.conf.json` is the SD-31 close-out value; SD-32's first cycle
  increments `build` to `0.12.<next>`. Major stays `0` until first publish to `main`.
* **Board:** local-file `kanban.md` paired with `progress.md` — no Hermes kanban board. The Hermes
  board was retired 2026-08-01 (`SD-30-class-feature-archetype-bundle/decisions.md` Decision 14a);
  SD-30 and its successors use the local-file pattern. Cycle dispatch reads `kanban.md` at the top
  of each tick and updates the card's `Status` row in place.

## Decision 1a — Anti-gaming doctrine (carried unchanged from SD-31 decisions §50)

**Status:** Doctrine-of-record. **Non-negotiable.**

A gate that cannot fail is worse than no gate. Every gate in this bundle — Gate 0 (census
closure), Gate 1 (shape closure), Gate 2 (engines), Gate 3 (closure invariant) — must be able to
fail closed. A placeholder predicate, an empty match set, or a default-to-true verifier cannot
manufacture false coverage; the verifier itself must refuse the empty case the way
`scripts/coverage_ledger.py` does (its empty-predicates fall through to uncovered = N, not 0).

The four gates' verification commands (see `acceptance-and-verification.md`) are the live
enforcement; this decision is the principle behind why every one of them is written so the empty
case is a red light.

## Decision 2 — The four gates are a definition of done, not a wave budget

**Status:** Operator-pinned.

SD-32 is scoped to reach a **definition of done**, not to a wave budget. The operator was
explicit: measurement waves are acceptable, and the board may barely move for several of them.
That cost is accepted deliberately in exchange for never running the same shape engine twice.

```
census closure   →   shape closure   →   engines
```

This is **program-level idempotence**, the same discipline SD-31 applied to generators one level up.
An engine's correctness claim is *"this shape is now handled."* That claim is false if the shape's
population can still grow. So engines cannot run until the population is closed.

**Consequence: book onboarding is a precondition, not an epic.** Every unbuilt book is an open
hole in the census, and running engines with holes open guarantees the rerun the operator does
not want. The Epic 4 "Book Onboarding" line in `epic-breakdown.md` is sequenced behind Gate 0 by
construction — every book lands before the shape that includes it can be declared closed.

The four gates (full text in `scope-draft.md §"Definition of done — four gates"`):

* **Gate 0** — Census closure: every object in every in-scope book is enumerated as a unit,
  proven by an independent walker (not the one being audited).
* **Gate 1** — Shape closure: every unit maps to a known shape; unclassified count is zero.
* **Gate 2** — Engines: one engine per shape, run once, every emitted value fixture-checked.
* **Gate 3** — Closure invariant: standing test that goes red if any object appears that no shape
  covers.

## Decision 3 — Fixture discipline is non-negotiable (operator ruling §20, restated)

**Status:** Operator-pinned, originally authorised for `formula_interpreter.rs` and generalised
here.

Every interpreted value clears `derived_evaluator_fixture_check`, whose expected value is
transcribed from bytes the evaluator never reads. **An interpreted value with no fixture is not
done.** This is the condition operator ruling §20 rests on; it is restated in this bundle's
scope, not because it changes, but because the temptation to skip it under compute-library
pressure will recur. It is the difference between "the engine emits 4" and "the engine emits 4
because the corpus says 4, and the verifier would catch a fabrication."

## Decision 4 — Shape analyser is a method, never a vocabulary

**Status:** Operator-pinned.

The ten semantic families identified in SD-31 wave 31's measurement are **d20 families**. They
assume levels exist and modifiers are numbers. Traveller has no levels at all — it is career and
skill based, where "per level" is a category error. World of Darkness uses dice pools: the shape
is not "+2 damage" but "add 2 dice." A shape analyser carrying PF1e's families would actively
mislead on system two.

**Consequence.** The shape analyser is built as a **portable procedure**, not a portable
vocabulary:

* Extract value-bearing expressions.
* Normalise away the nouns.
* Cluster.
* Count the units behind each cluster.
* Report coverage.

That procedure ports to any system. The ten families do not. SD-32 ships the procedure plus the
PF1e families as a binding; future systems re-derive their own families against the same
procedure. The shape analyser does not ship with a hard-coded family list.

## Decision 5 — Tooling scope: built, not built, and the third category

**Status:** Operator-pinned.

Three things are scoped; three are not.

**Built.**

* **Census tool** — framework plus a pluggable reader. The LST reader exists in substance already
  (every PCGen-derived walker in `scripts/`). Structure it as reader / analyser / reporter so the
  seam is present and reusable.
* **Shape analyser** — built per Decision 4 above.

**Deliberately not built.**

* **No PDF reader in SD-32.** The orchestrator argued for one on the premise that PF1e PDFs
  existed and could calibrate the extractor against PCGen ground truth. **The operator corrected
  that: there are no PF1e PDFs. The nearest source is scraping d20pfsrd.com.** The argument does
  not survive the correction — HTML scraping and PDF extraction have different failure modes
  (columns, ligatures and reconstructed tables versus templating and navigation cruft), so
  calibrating one against the other measures the wrong thing.
* **No speculative generalisation of the reader seam.** One worked example. An abstraction drawn
  from a single instance reliably comes out shaped like that instance. The seam is built;
  generalising it waits for a second real reader to test it against.

**Genuinely unresolved, and belongs in its own scoping.**

* **How to ingest a system whose rules exist only as prose.** This is a research question, not an
  engineering one, and assuming it into SD-32's tool design would be the same error as the PDF
  reader. Two things are known and worth carrying forward:
  - The pipeline splits in two. **Getting text out of a source is source-specific and does not
    transfer.** Turning prose into structured objects and shapes is shared regardless of source —
    and it is the harder, riskier half, so it is where calibration effort belongs if any is
    available.
  - **The whole anti-gaming apparatus rests on a checkable source.** Every "re-derive it yourself"
    instruction, every mutation proof, every GAMED verdict bottoms out in *the corpus says X,
    verifiably against a pinned SHA*. A prose source removes that foundation: the extraction
    **is** the corpus, and the extraction is the thing most likely to be wrong. The discipline
    does not port unchanged, and a replacement for pinned ground truth must be designed before a
    prose-sourced system is attempted.

  This is filed in `forward-scope-register.md` (C3.x) as research-grade forward scope, not as a
  bundle item.

**On d20pfsrd specifically**, if it is ever used: it is Open Game Content, which is legally cleaner
than a commercial PDF, but it is a **subset of the books by construction** — Product Identity is
excluded by design, so deity names and setting content are simply absent. Gaps found against it
are real in the source, not extraction errors. And for PF1e it is largely redundant: PCGen is
better structured, pinned, and already the ground truth every gate depends on. Adding a second
PF1e source creates a conflict-resolution problem that does not currently exist. If tested at all,
test it on one of the four uncompiled books first.

## Decision 6 — File-disjointness is structural, not a preference

**Status:** Standing doctrine.

Each gate's work touches a different surface and runs in a different order:

| Phase | Surface | Why it must come first |
|---|---|---|
| Gate 0 | `scripts/census_*.py` (new) + `data/corpus/*/...lst` enumeration | The population must be closed before shape claims can hold |
| Gate 1 | Something `coverage_ledger.py` already does for groups, extended to shapes | The vocabulary must close before engines run |
| Gate 2 | `src/rules_core/pilot_compute/formula_interpreter.rs` + `src/rules_core/pilot_compute/bonus_stack_reader.rs` generalisation | One engine per shape; each fixture-checked |
| Gate 3 | A standing `verify.sh` stage, similar in shape to `coverage_ledger.py`'s own closed-on-empty posture | The closure invariant must fail the way the gate it's enforcing does |

Epic 1 (compute library) and Epic 2 (cause closure) are sequenced behind Gates 1+2 by
construction. Epic 3 (class reachability) and Epic 4 (book onboarding) are sequenced behind Gate 0
by construction. Epic 5 (automation, decided on evidence) runs throughout — its first deliverable
is the *protective* self-erasure sweep across all 30 Rust generators, which fires before Gate 0
because scaling engines over a generator that silently empties its own fixtures is how thousands
of banked units disappear with the suite green.

The file-disjointness claim is verified at `loop-instruction.md §3` (per-epic parallel/sequential
map), not assumed.

## Decision 7 — Open operator rulings (B1/B2/B4/B5 from SD-31 `todo/blocked.md`)

**Status:** Operator rulings pending. These four unresolved items from SD-31 carry into SD-32
because they would shrink the honest denominator without changing a line of code:

* **B1** — `mod_only_rescue`: a 249-unit cross-kind phantom-duplicate population that would shrink
  both the `feat` kind and the denominator. Proposed, never ruled.
* **B2** — per-race branch 1/2/3 classification. Race attribution stays frozen until this is
  answered.
* **B4** — do the 48 structurally-non-PC-class `class` units belong under the class doneness gate
  at all? Monster hit-dice progressions, Eidolon, psionic power-list menus.
* **B5** — are the 5 `Ex-*` records real classes, or PCGen alignment-violation bookkeeping?

Rulings on these belong in `risks-and-open-questions.md` (not here) so they stay visible as live
operator questions, not bundled into doctrinal closure. B4 and B5 are the most leveraged: a single
ruling would lower the denominator by 48+5 = 53 units and the closure threshold by the same.

## Decision 8 — SD-31 → SD-32 session handoff is load-bearing

**Status:** Standing.

The SD-31 session ran for days across 31 waves and is being retired for stability. The
`artifacts/HANDOFF.md` document (captured 2026-08-22) holds context that is true but not written
down anywhere else. The five operator-pattern footguns it lists (wrong-base worktrees, `find
-newermt` lies, omitted `model` on `agent()` calls, `git stash` taking the whole shared checkout,
rulings not in force until committed) are not the bundle's scope, but they are the load-bearing
**operator-pattern** knowledge the next session will need. They are mirrored into
`loop-instruction.md §6/§7` and into `risks-and-open-questions.md §"Five footguns from the SD-31
session"`. The HANDOFF itself stays as the canonical source of record.

## Decision 9 — Build counter resolution at first cycle

**Status:** Authoring-time rule.

**`build` is a monotonic counter across all builds across all branches — never resets**, per the
2026-07-17 amendment. `apps/desktop/package.json` and `apps/desktop/src-tauri/tauri.conf.json` are
the live version source of truth (`Cargo.toml` stays pinned at `0.1.0` and is not authoritative).
SD-32's first concrete build value is whatever `cargo run --locked --bin v06_work_inventory` or
the equivalent version-derivation tool returns at cycle-0, written into the cycle-0 receipt —
**not** left as `0.12.<build_at_launch>` in any shipped file. The `0.12.<build_at_launch>` form
appears only in `README.md` "Bundle at a glance" and in `loop-instruction.md §1.7` as the
template-time placeholder; both are replaced by the literal value at the first cycle.

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

* **Branch:** `tranche/12` (cut from `tranche/11`'s tip). SD-31's content reached `develop` via
  PR #374 (merged 2026-08-22T19:53Z), verified **by content** (`git diff origin/develop b1b7f4290 --
  src scripts data docs/retro docs/release/SD-31-corpus-closure-grind` is empty), not by ancestry —
  merge-commit topology makes `--is-ancestor` report false even when every byte is present. The
  `SD-31-corpus-closure-grind/decisions.md §6` rationale — "tranche/N+1 carries tranche/N's full
  history" — applies symmetrically; SD-31's work is present in `tranche/12`'s history.
* **Build version:** `<major>.<tranche-base>.<build>` per the 2026-07-17 amendment. **`tranche-base`
  is the numeral in the branch name** (per SD-31 decisions §6, which is the canonical pattern this
  bundle inherits). For `tranche/12` that is `12`. The tranche digit bumps **once, at the tranche
  cut** (SD-31 precedent commit `147f1c2b7`, `0.11.0` for `tranche/11`): SD-32's bump to **`0.12.0`**
  in `apps/desktop/package.json` / `apps/desktop/src-tauri/tauri.conf.json` landed on `tranche/12`
  at launch-readiness remediation (2026-08-22). Published builds stamp `0.12.<build>` at publish
  time. Major stays `0` until first publish to `main`. Derivation command (§9):
  `grep -h '"version"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | head -2`.
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
**Reaffirmed 2026-08-22 at launch-readiness review: SD-32 closes when the content achieves the
Definition of Done — all four gates' AT-32-* criteria met — and on no other trigger** (not a wave
count, a date, or a token budget; `workflow-instruction.md §13`).

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
is the *protective* self-erasure sweep across all 29 Rust generators
(`ls src/bin/{gen_,ingest_,enrich_}*.rs | wc -l`), which fires before Gate 0
because scaling engines over a generator that silently empties its own fixtures is how thousands
of banked units disappear with the suite green.

The file-disjointness claim is verified at `workflow-instruction.md §3` (per-phase
parallel/sequential map) and `§4` (file-touch verification), not assumed.

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

(There is no **B3** here: the identifiers are SD-31 `todo/blocked.md`'s own numbering, carried
unchanged so cross-references to SD-31 stay valid. B3 — "prerequisites in open pools" — was CLOSED
in SD-31 wave 29 with a corpus-wide count (`SD-31-corpus-closure-grind/todo/blocked.md` B3;
`is_archetype_locked()` in `src/rules_core/class_feature_pool_catalog.rs`) and does not carry
forward.)

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
`workflow-instruction.md §9` (with §6 step 1 as the mechanical control for footgun 1) and into
`risks-and-open-questions.md §"Five footguns from the SD-31 session"`. The HANDOFF itself stays as
the canonical source of record and is not edited; where its figures have since been re-derived
(29 generators, not ~30; the stale local branches are `site-deploy` / `fix/site-deploy-page-workflow`,
not a `site-publish/*` glob), the citing documents carry the corrected value and its command.

## Decision 9 — Build counter resolution (resolved 2026-08-22)

**Status:** Resolved at launch-readiness remediation.

**`build` is a monotonic counter across all builds across all branches — never resets**, per the
2026-07-17 amendment. `apps/desktop/package.json` and `apps/desktop/src-tauri/tauri.conf.json` are
the live version source of truth (`Cargo.toml` stays pinned at `0.1.0` and is not authoritative).
The tranche digit bumps once, at the tranche cut (SD-31 precedent `147f1c2b7`). SD-32's literal
first concrete value is **`0.12.0`**, landed on `tranche/12` 2026-08-22 by a dispatched housekeeping
agent (shipping-code edit, per `workflow-instruction.md §2.2`). Published builds stamp
`0.12.<build>` at publish time. One derivation command, quoted wherever the value is quoted:

```bash
grep -h '"version"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | head -2
```

The literal is written in `README.md` (frontmatter `build_version_target` + "Bundle at a glance"),
this file's §1, `workflow-instruction.md §0` and `§1` item 7 and `§11`, `progress.md` "Pre-launch
receipt", and `risks-and-open-questions.md` risk 7. The old build-at-launch template marker no
longer appears anywhere in the bundle (`workflow-instruction.md §10`).

## Decision 10 — Closure requires every Epic card `complete`; "filed under Open blockers" is not a closure path (operator ruling 2026-08-22, post-run)

**Status:** Operator-pinned. **Supersedes** the "complete **or** filed under `## Open blockers`
with a named owner" clause in `acceptance-and-verification.md` AT-32-CLOSE-001 item 1 and
`workflow-instruction.md §13` step 1, as those read at the first dispatch run.

The first SD-32 dispatch run (`wf_efd6f5fc-a9c`, 2026-08-22) closed all four gates and cards
1-10 and 12, then closed the bundle and opened PR #375 with card 11 (`epic-2-cause-closure`) at
`returned-to-backlog` — its remaining eight blocker shapes deferred to a successor bundle via
`forward-scope-register.md` C2.5. The closure cycle did this correctly under the criterion as
written. **The operator ruled the criterion itself wrong:**

> "if card 11 is returned to the backlog, then sd-32 isn't ready for a pr, nor a merge."

**In force from now on:**

1. The Definition of Done is **all four gates met AND every Epic 1-5 kanban card at `complete`**.
   A card at `returned-to-backlog`, `in-progress`, or `DISCOVERED-forked` blocks closure.
2. An `## Open blockers` filing is a **request for a ruling**, not a disposition. It pauses the
   bundle and surfaces the blocker to the operator; it never authorises closure past the card.
   Only an operator ruling may move scope out of a card and into `forward-scope-register.md`.
3. **No PR opens while any Epic card is short of `complete`.** PR #375 was opened prematurely
   under the old wording and was closed 2026-08-22 pending card 11's real closure.
4. This applies to a card's *deferred halves* too, not only whole cards: card 12
   (`epic-3-class-reachability`) was marked `complete` with its 18-untabled-base-class half
   explicitly deferred. Under this ruling that half is reopened and must land before closure.

**Why:** a bundle that ships with its largest content epic deferred has moved work, not done it.
The gates measure that the *method* is sound; the cards measure that the *content* is closed.
Closing on gates alone lets a green board describe an unfinished bundle — the exact
shape `Decision 1a`'s anti-gaming doctrine exists to refuse.

## Decision 11 — T8 write scope granted for `scripts/observer/pf1e_dashboard_producer.py` (operator ruling 2026-08-22)

**Status:** Operator-pinned. Resolves the scope-boundary block recorded in card 11's cycle-1
receipt (`artifacts/gate-3-closure-invariant/epic-2-cause-closure_cycle-1_cycle_receipt.md`,
"T8 (D13, 12 units) — scope-boundary block").

Card 11's cycle 1 identified T8's fix site as `scripts/observer/pf1e_dashboard_producer.py` — the
`wiring_class`-vs-`status` classifier blind spot, 12 units — and correctly refused to touch it,
because that file is SD-30's Epic 0 surface and read-only from SD-32 absent a ruling.

**Ruling: SD-32 is granted write scope to `scripts/observer/pf1e_dashboard_producer.py`, bounded to
the T8 classifier fix.** The grant is for that defect only; it is not a general licence over SD-30's
Epic 0 surface, and no other file under `scripts/observer/` is in scope.

**Conditions:**

1. The change is TDD'd like any other cycle — RED proved for the intended reason, GREEN, the
   population re-derived with a committed command, and the fix proved by class over all 12 units,
   not by instance.
2. `scripts/observer/pf1e_dashboard_producer.py` is the **producer** in the
   producer → JSON → static-viewer pipeline; a separate one-minute renderer writes a different
   file. The cycle must confirm which artifact its change actually lands in and prove the corrected
   classification reaches the dashboard's consumed JSON, not only the producer's own output.
3. Any dashboard figure the fix moves is re-derived and stated with its command — a count change
   compiles clean but can leave other files' pinned assertions red, so the cycle greps the old and
   new counts across `tests/`, `src/`, `scripts/`, and `apps/` before committing.
4. T8 closing removes the last non-`complete` condition on card 11 under `decisions.md §10`.

## Decision 12 — One family vocabulary, and the 27,847 kind-unenumerable objects are in scope (operator ruling 2026-08-22)

**Status:** Operator-pinned. Adds kanban cards 14 and 15. Both are Epic-scope cards and therefore
bind closure under `decisions.md §10`.

Two defects surfaced when the operator asked, after the first dispatch run, whether SD-32 had
classified 100% of all objects. The honest answer was no, for two independent reasons.

### 12a — The shape-family vocabulary forked; reconcile it to one

`scripts/shape_ledger.py` ships eleven families (F0-F10) whose definitions and counts are its own:
F0 20,113 / F1 1,790 / F2 1,490 / F4 570 / F5 361 / F3 303 / F6 211 / F8 41 / F9 27 / F7 5 /
F10 3, summing to 24,914 with `unclassified_count` 0. SD-31's `MEASURE-TWICE.md §3` carries a
*different* ten-family vocabulary with different counts (1747/1140/804/563/368/211/54/37/17/7).
AT-32-G1-001's "vocabulary extension allowed with measured units" permits the ledger to define its
own — but nothing reconciled the two, and three consequences followed:

1. **AT-32-G1-003's own cross-check command names a table that does not exist.** Card 5 logged this
   as a `scripts/retro.py correction` (`docs/retro/events/gate-1-shape.jsonl`, id
   `1787437987996-gate-1-shape-0ae65f`): the criterion tells a cycle to diff the ledger's printed
   family counts against "the F1..F10 table in `epic-breakdown.md`", and `epic-breakdown.md`
   Epic 1's F1/F2/F3 entries are three *work items*, not semantic families with counts.
2. **The same label means two things.** `kanban.md` card 7 is titled "Generalise
   `bonus_stack_reader.rs` for **F10** binding-layer family", while the shipped ledger's own F4
   proof-width text states F4 is "the shape `bonus_stack_reader.rs`'s binding-layer pattern
   targets" and defines F10 as an unrelated step-count heuristic covering 3 units.
3. **Engine coverage claims are not checkable.** Card 7's "77.2% of custom identifiers (893/1,156)"
   and the ledger's F4/F10 counts are three different denominators; no committed command reconciles
   them.

**Ruling: SD-32 ships exactly one shape-family vocabulary.** Card 14 picks it, defines it in one
committed, re-derivable place, and propagates it to the ledger, every engine's module
documentation, every AT-32-* criterion that names a family, `kanban.md`'s card titles, and
`epic-breakdown.md`. Where the two vocabularies genuinely disagree about what a shape *is*, the
reconciliation states which is correct and why, with counts — it does not silently pick one.

### 12b — The 27,847 kind-unenumerable objects are in scope, not a footnote

Gate 0's own `artifacts/gate-0-census-closure/diff.json` reports `unexplained: 0` while also
reporting `total_kind_unenumerable_units: 27,847` across 44 buckets. That filing is *honest* — it
names and counts them rather than pretending they are zero, which is what AT-32-G0-002 asked for —
but naming an object is not enumerating it, and an object outside the ledger's population is an
object no shape covers and no engine reaches. The buckets:

| Bucket | Units | Note |
|---|---|---|
| `class_feature` | 18,231 | `docs/work-inventory.json` tracks 15,439 — a **2,792-unit** disagreement, direction unknown |
| `ability_category:*` | 5,886 | ~10 categories (`Special Ability` 3,436, `Internal` 839, `Words of Power` 369, …), no tracked kind |
| everything else | 3,551 | `template_row` 2,343, `deity` 460, `power` 421, `domain` 183, `language` 143, … |
| `unclassified:<file>` | 179 | files the walker could not type at all |

**Ruling: these are in scope for SD-32.** Card 15 closes them. Closure means each object is either
(a) enumerated as a unit in a tracked kind, classified into a shape family, and covered by Gate 3's
standing gate, or (b) proven **not** to be an object — a continuation row, a facet of a unit already
counted, or a non-object file — by class, with the committed command that proves it and the count
it accounts for. The `mod_continuation: 23,625` and `copy_derivation: 2,338` figures in the same
`diff.json` are the leading hypothesis for the `class_feature` disagreement and must be tested, not
assumed.

**"Sum the piles" is the acceptance bar** (standing lesson 5). Card 15 is complete when the census
population, the inventory population, and the shape-ledger population reconcile to each other with
one committed command, and every unit in the reconciled total carries a family.

### 12c — Consequence for the three denominators

The bundle currently quotes three unreconciled totals: the ledger's 24,914, the inventory's 38,391,
and the census's 28,037-plus-27,847. The 24,914-vs-38,391 gap is legitimate and already explained
(the ledger's population is *not-done* units; 13,477 are done). It must nonetheless be **stated**
wherever a total is quoted, with its command — no bundle document may quote a bare total again
without naming which population it is.

## Decision 13 — Card 11's five open sub-populations are closed by doing the work (operator ruling 2026-08-22)

**Status:** Operator-pinned. Answers the ruling request filed under `progress.md` `## Open blockers`
by the reclosure-epilogue correction cycle (`e47f641b9`), which correctly refused to rule on it
itself per `decisions.md §10` item 2.

### What was asked

SD-32's continuation run closed T1, T3, T5, T7, T8, T4-L8, and T2a's structural cause, and left
five sub-populations open. Two lanes (T2b, T9) filed explicit "ruling needed" requests; a
consolidation cycle answered them on its own authority; the Opus adversarial verifier caught that
(`NOT_READY`, finding 3) and the reclosure cycle reverted it and escalated properly.

| Shape | Open units | What it actually is |
|---|---:|---|
| T2b | 2,472 | Named cause **disproven** — the compound-key matcher only runs on already-ingested records and none of these were ever ingested. Real cause: 1,754 in books never registered in `race_catalog.rs`'s `RACE_CORPUS_BOOKS`, 718 in registered books never transcribed from the pinned oracle. |
| T9 | 2,712 | Per-record onboarding backlog across `spell`/`companion`/`feat`/`monster_ability`/`equipment`/`monster`. Same shape as T2b: the records exist in the oracle and were never ingested. |
| T12 | 2,453 | `class_feature`s belonging to classes the engine does not model. ~47 are suspected false positives (archetype features attributed to a phantom PCGen "class") and must be confirmed, not assumed. |
| T2a residual | ~2,775 | Records carrying a category label (`Domain Power`, `Wild Talent`, `Ki Power`, …) rather than a class name. 4,936 records were already corrected at the cause this run. |
| T4-L9 | 471 | Needs a **feat-held** reachability gate; today's gate is class-held. |

### The ruling

**All five are closed by doing the work.** None moves to `forward-scope-register.md`. Card 11
reaches `complete` only when every one of these five is genuinely closed by class.

> *"do the work on in all cases as suggested. If you want to do measurements first, i support this.
> No matter what though, i want the work done"*

**Measurement is explicitly authorised as a first step, and does not substitute for the work.** The
cost on this repo is per-book and per-file, not per-record (`docs/retro/` E13 calibration), so the
book count — not the ~10,800 unit count — determines the real size. A measurement cycle that banks
zero units but produces a real, re-derivable book/file census is a legitimate closed cycle
(standing lesson 6). It is a **precursor to** the work, never a report **instead of** it.

### Consequences

1. **A disproven cause is not a closed shape.** T2b's and T9's lanes did real forensic work and
   correctly disproved their *named* causes — that finding stands and is valuable. But
   AT-32-E2-001 requires each shape closed corpus-wide, and 2,472 and 2,712 units respectively are
   still open. Reclassifying them as "a separate book-onboarding project" is moving work.
2. **T12 is not closed by T2a's fix.** Its population is literally unchanged at 2,453; the
   `data.class` correction improved data quality without touching T12's own defect (the class is
   unmodelled). A relabelled shape is not a closed shape (`decisions.md §1a`).
3. **T4-L9 blocks card 11 exactly as a whole card would** (`decisions.md §10` item 4) — a card at
   `complete` with a named, uncleared sub-population is the half-deferral defect card 12 was
   reopened for, reproduced inside card 11.
4. **Card 15's own acceptance bar is unchanged** (`§12b`): the single committed reconciliation
   command, and every unit in the reconciled total carrying a family.

## Decision 14 — Gates 1 and 3 are REOPENED: the closure invariant cannot fail, and 41.8% of the "covered" population has no corpus record (2026-08-22)

**Status:** Finding of record, verified twice — by card 15's Opus adversarial verifier and independently
re-run by the orchestrating session against the **repo-local pinned oracle**
(`7f818006e371188e5717fd18d74d18a420747fc6`, identical SHA to the path the verifier used, so its
findings are not an artefact of corpus choice). **Gate 1 and Gate 3 no longer count as met.**

### 14a — Gate 3's standing gate cannot go red for a real object of any kind

`scripts/shape_coverage_standing_gate.py` was accepted as Gate 3 on a red-proof that **fabricates a
row with `family: None`** by `mock.patch`-ing `SL.build_ledger`. That path cannot occur in reality:
`shape_ledger.classify_unit()` **always** returns a family, falling through to F0 or F8 rather than
ever returning `None`. `scripts/tests/test_shape_coverage_standing_gate.py` states this in its own
docstring. So `unclassified_count` can never organically go non-zero, and the gate can never fail.

Reproduced by the orchestrator — 80 fabricated objects across the eight kinds card 15 has pending,
every one pointing at a nonexistent corpus file, zero shape evidence:

```bash
export PCGEN_CORPUS_ROOT="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data"
python3 -c "
import sys; sys.path.insert(0,'scripts')
import shape_coverage_standing_gate as G
u=[{'id':f'b:{k}:{i}','kind':k,'book':'b','status':'not-started','wiring_class':'static','source_file':'totally_fake_file.lst','source_line':i} for k in ('ability','skill','template','deity','power','domain','language','kit') for i in range(1,11)]
print(G.run_gate({'units':u}, corpus_root='/nonexistent'))"
```
→ `(0, {'population': 80, 'unclassified_count': 0, 'piles_reconcile': True, 'families': {'F0': 80}})`
— **exit 0, PASS.**

This is `decisions.md §1a` verbatim: *a gate that cannot fail is worse than no gate*, because it
reports safety it does not provide. It also invalidates card 15's stated remaining path — *"once
units land correctly, family classification and Gate 3 coverage follow for free"*. Landing the
9,008 pending units produces 9,008 more F0 rows and an unchanged `PASS`.

**Required:** Gate 3 must carry an invariant that goes red on a **real** object with no shape
evidence, proven by mutating real data — never by patching the ledger builder. See 14b: the
`no_record` join status is the natural candidate, because a unit whose corpus record cannot be
found is precisely an object no shape covers.

### 14b — 41.8% of the ledger's "100% covered" population has no corpus record at all

`unclassified_count = 0` over 24,914 units has been quoted as Gate 1's closure throughout this
bundle. The join behind it:

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l.json
python3 -c "
import json,collections
r=json.load(open('/tmp/l.json'))['rows']
print(collections.Counter(x['join_status'] for x in r))"
```
→ `{'no_record': 10419, 'matched': 4801, 'no_formula_tokens': 9694}`

**Only 4,801 of 24,914 units (19.3%) rest on a matched corpus record.** F0 (20,113 units, 81% of the
population) conflates two different things its own proof-width text distinguishes — `no_record`
("the join found nothing") and `no_formula_tokens` ("found a record, it carries no DEFINE/BONUS").
The first is not evidence of no formula content; it is absence of evidence.

The disclosure exists in F0's proof-width prose but appears in **no** gate output, in **no**
AT-32-G* criterion, and not in card 14's canonical family table headline — and card 14 blessed this
vocabulary as canonical without flagging it. Under `decisions.md §12c` (no bare totals), every
statement of Gate 1 coverage must from now on carry its join-status split.

### 14c — Consequences

1. **Gates 1 and 3 are reopened.** Neither may be quoted as met until 14a's invariant exists and
   14b's split is surfaced in the gate output and the criteria. The Definition of Done
   (`decisions.md §10`) is unchanged; the gates simply are not met yet.
2. **This is a blocker, not a deferral** (`docs/governance/blocker-closure-doctrine.md`). It was in
   the Definition of Done at launch. It gets cleared.
3. **Do not "fix" this by deleting or renaming F0.** The honest split is the deliverable: state how
   many units have a matched record, how many have a record with no formula tokens, and how many
   have no record — then make the third category behave like the gap it is.
4. **Card 15's 2,614-row `CATEGORY:Internal` disposition is unresolved, not settled.** Card 15's
   class_feature lane disposed all 2,614 as (B) "not an object"; the sibling `ability_category`
   lane's own per-row classifier found **81.6% (685/839)** of the same marker is (A) "is an object".
   The verifier found 910 of the 2,614 resolve to no already-counted unit, and 2,420 (92.6%) carry
   independent mechanical content (`SPELLKNOWN:` 1,185, `BONUS:` 675, `ABILITY:` 512, `DEFINE:` 151,
   `TEMPBONUS:` 70, `AUTO:` 38). Two lanes reached opposite conclusions on the same rows and neither
   cross-checked. `remaining_undisposed: 0` is arithmetic, not substance. Settle it by evidence
   before any of it is enumerated or excluded.

## Decision 15 — T9's Product-Identity exposure is audited before the blacklist is signed off (operator ruling 2026-08-22)

**Status:** Operator-pinned. Answers the escalation raised by T9's census lane and relayed by the
orchestrating session. **T9's onboarding work is paused pending this audit; T2b, T12 and
T2a-residual are unaffected and proceed.**

### The blocker

`docs/governance/ogl-pi-blacklist.md` carries `status: DRAFT — operator-reviewable, not
unilaterally binding` and `review_state: pending_operator_sign_off`, `last_reviewed_at: 2026-07-27`.
Transcribing a record that carries Product Identity is not a judgment a cycle may make on its own
authority, and the blacklist that would settle it is not in force.

T9's census sampled the monster kind and found **21 of 28 units** PI-blocked. Critically, only
**114 of 2,712 units (4.2%)** of T9 have had any PI pass at all — so the exposure across `spell`,
`feat`, `equipment`, `monster_ability` and `companion` is **unknown**, and the 96% rate observed in
the monster kind is a sample, not a measurement.

### The ruling

> **Operator: option 2 — audit first.**

**A dedicated audit lane sweeps all 2,712 T9 units against `ogl-pi-blacklist.md` and reports the
real blocked count, per kind and per book, naming the records.** The operator signs off (or amends)
the blacklist knowing the actual exposure, rather than approving a July draft blind.

**The audit is read-only.** It transcribes nothing, ingests nothing, and changes no corpus data. It
does not amend `ogl-pi-blacklist.md` — its status stays `DRAFT` until the operator acts on the
audit's findings. Its deliverable is evidence, not a decision.

### Standing rule this establishes

**A cycle that reaches a record it believes carries Product Identity stops on that record.** It
does not transcribe it, and it does not silently skip it: it lands everything else in scope, then
reports the record by name and count so the orchestrator can escalate. This is `AGENTS.md`
Blocker Discipline disposition 2 — raise your hand — and it applies to every shape in this bundle,
not only T9. A silent skip is the worst outcome available: it looks like completed work and leaves
no trace that a licensing decision was made by an agent.

### Why an audit and not a sign-off

Both options cleared T9's blocker. The audit was chosen because nobody currently knows whether the
PI-blocked share of T9 is 5% or 90%, and the two cases call for different rulings — a small blocked
set is a named exclusion, while a large one changes what T9's closure can mean at all. Signing off
a draft to unblock a lane, without knowing what the draft is being applied to, is the shape of
decision this bundle has already had to reverse twice.

## Decision 16 — T2b's population is largely classifier noise; the work is the classifier, not 26 book-onboarding cycles (2026-08-22)

**Status:** Finding of record. Does **not** narrow `decisions.md §13` — the operator's ruling stands and
every one of card 11's five shapes is still closed by doing the work. What changed is **what the work
is**, established by evidence rather than by estimate.

### The finding — four independent corroborations, same day

T2b wave 1 dispatched four lanes across nine books. All four independently reached the same
conclusion by different methods, in different books:

| Lane | Books | Nominal units | Real T2b work found |
|---|---|---:|---|
| w1-a | bestiary_2, monster_codex, bestiary_6 | 268 | **18 closed**; ~235 are monster stat-block content; 8 not-work |
| w1-b | bestiary_5, inner_sea_races, horror_adventures | 199 | **12 closed** (a stale regen, not new content); 4 template `.MOD`/`.COPY=` rows; 181 need new chassis/mechanisms |
| w1-c | core_rulebook, advanced_players_guide, advanced_race_guide | 104 | **0**; 51/51 confirmed non-race PCGen plumbing; APG's 37 are Favored-Class-Bonus engine rows naming no race at all |
| w1-d | bestiary_3 | 819 | **0**; the book declares **zero** playable races — every one of its ~261 `b3_races.lst` entries carries a `CR:` token (monster) |

**Root cause:** `src/bin/v06_work_inventory.rs::refine_kind` matches on the TYPE token's **first
segment only**, so monster and creature-template special-ability rows are typed `kind: race_trait`.
The population T2b was sized against is therefore substantially not race content. This is the same
defect class already recorded for `file_kind()` typing `.lst` files by filename.

**30 units closed in wave 1** — 18 (Dhampir chassis + Ratfolk alternates) and 12 (`inner_sea_races`,
which turned out to be a **stale regeneration**: `IN_SCOPE_RACES` widened 18→34 across three SD-31
waves and the book was never re-run; the unmodified binary against the pinned oracle closed 12).

### Two real defects found and fixed on the way

1. `race_creation.rs::vision_reading()` read only the first `VISION:` value when PCGen states
   multiple senses as one `|`-joined field (Dhampir's shape). Both shapes now resolve.
2. A sibling lane's `inner_sea_races` regen landed 9 alternates without engine wiring and left
   `cargo test --locked --lib` **red on `origin/tranche/12` itself**. The w1-a lane caught it on
   rebase and fixed it, per §5's own instruction to verify tests before pushing. The shared-file
   contention warning in the dispatch earned its place.

### The corrected plan — three cycles, not twenty-six

Dispatching sixteen more per-book onboarding waves would build race chassis for monster stat blocks.
That is fabricating content to close a counter, which `decisions.md §1a` forbids. The real work:

1. **Fix `refine_kind`.** Blast radius is corpus-wide and a naive fix is **known unsafe** — the w1-d
   lane verified that every real race's `Favored Enemy ~ Humanoid (<Race>)` row shares an inner
   `SpecialAttack` dot-segment with the monster-only facet vocabulary, so a first-segment-only
   widening would wrongly reclassify genuine content in **every** book. This needs a tested,
   adversarially-checked cycle.
2. **Build the `Adoptive Parentage` / "Adopted Race" selector once.** It spans **five** books —
   bestiary_2 (7), bestiary_3 (5), advanced_race_guide (7), bestiary_5 (1), bestiary_6 (1) — and
   every lane that met it correctly refused to build a per-book shim.
3. **Then re-measure T2b.** The residual after (1) and (2) is the real per-book work, and it is a
   fraction of 2,325.

### The guard rail on this decision

Reclassification makes work **appear** to vanish, which is exactly the shape `decisions.md §1a`
exists to refuse. This finding is accepted because four lanes reached it independently, by different
methods, with committed commands — not because it is convenient.

**The classifier cycle must therefore prove, by test, that it does not reclassify genuine race
content**, and its result goes through adversarial verification before any unit is credited as
"not work". A unit moved out of T2b is not a unit closed: it is a unit that belonged to a different
kind all along, and the receipt must say which kind and prove it.

## Decision 17 — Stop treating every object as a snowflake: the generic ingest already exists (operator correction 2026-08-23)

**Status:** Operator correction of the orchestrating session. Supersedes the per-book dispatch shape
used in T2b wave 1 and the 98-cycle estimate derived from it. `decisions.md §13` (do the work) and
`§16` (fix the classifier) both stand.

### What the operator said

> *"havent we had a generic ingest since we first started? ingest everything. analyze the shapes.
> quit trying to treat every object as a snowflake. you seem to have forgotten all the lessons from
> sd-31"*

Correct on every count.

### The measurement that settles it

| Layer | Size | Generic? |
|---|---:|---|
| `scripts/census_independent.py` | 564 lines | yes — walks 186 book dirs, finds 55,884 objects |
| `scripts/shape_ledger.py` | 510 lines | yes — classifies all 24,914 not-done units, 0 unclassified |
| `src/bin/v06_work_inventory.rs` | 15,821 lines | partly — **174** hardcoded book references |
| `src/rules_core/rules_tables/**` | **137,002 lines**, 204 files | no — hand-authored per book/race/class |

**A generic ingest has existed the whole time.** "Ingest everything and analyse the shapes" is
~1,000 lines of already-working tooling. The 137,002-line hand-authored table layer is the snowflake
treatment, and it is where every per-book estimate came from.

The same shape repeats in `src/bin/`: **seven** spell-ingest binaries, 3,367 lines, one per book,
sharing the same thirteen functions around a common parser (`src/pcgen_import/lst_parser/`, 4,161
lines, 8 modules). Only two lines per binary are genuinely book-specific. The copies have **drifted**
— `pi_screen` has three distinct implementations across the seven, and `ultimate_combat`'s is six
lines shorter than the others. That is a live licensing-correctness defect, not just duplication,
and it undermines the T9 PI audit's own `clear` bucket.

### The SD-31 lessons this violated

`docs/retro/sd31-retrospective.md` had already written both failures down:

1. **"Lane scoping that guaranteed zero yield.** Six consecutive `race_trait` lanes were scoped to
   'tables and matchers only, no chassis work' and every one shipped nothing. Units finally moved
   when an integration cycle lifted the restriction. **The bottleneck was my brief, not the lanes.**"

   T2b wave 1's brief said *"ingest-tool extension only, ~3 files each"* and scoped chassis work out.
   Four lanes, nine books, **30 units**. The identical failure, one bundle later, from the same cause.

2. **"The wall was never the rules.** PCGen has solved Pathfinder for twenty years; the logic exists,
   tested, in Java. Wave 31's taxonomy put the split at roughly **3.3:1 to 4.4:1 in favour of our own
   plumbing** over genuine rules complexity — a dispatch that did not exist, a matcher requiring an
   exact string, a class name read from the wrong field."

   Every one of card 11's five shapes is that plumbing. T2b's named cause was a matcher that turned
   out not to run at all; T2a's was a class name read from the wrong field; §16 found T2b's
   population is largely a classifier typing monsters as races. Estimating them per-book accepted
   the snowflake premise instead of attacking the plumbing.

### The ruling

**Enumeration and shape analysis are generic passes, not per-object work.** No further per-book,
per-race, or per-class onboarding lanes are dispatched for card 11 or card 15. The work is:

1. **Make `v06_work_inventory.rs` enumerate every kind the census already finds**, driven by the
   walker's own object-definition rules rather than 174 hardcoded book references and one
   hand-added `Kind::` variant per cycle. Adding a kind must not cost a cycle — a full cycle to add
   `Kind::Skill` alone is the symptom.
2. **Collapse the seven per-book spell-ingest binaries into one config-driven pass**, and fix the
   three-way `pi_screen` drift to a single screen in a single place while doing it.
3. **Then re-run the shape ledger over everything** and report what is genuinely left.

The residual after those three is real per-object work, and it will be a fraction of the 98 cycles
estimated from the snowflake premise. **That estimate is withdrawn** — it measured the cost of the
wrong approach.

### The standing control

**Before any lane is scoped to "extension only" or "no <X> work", check whether that restriction is
the bottleneck.** SD-31's six zero-yield lanes and SD-32's T2b wave 1 are the same error, and the
common signature is a brief that forbids the change the units actually need. A lane that returns
"blocked, needs chassis work" three times over is reporting a scoping defect, not a content problem.

## Decision 18 — Per-record review of T9's 1,344 uncertain units before any further PI sign-off (operator ruling 2026-08-23)

**Status:** Operator-pinned. Answers the question posed by the T9 PI exposure audit
(`artifacts/gate-3-closure-invariant/t9-pi-exposure-audit.md` §9). Second ruling in this chain:
`decisions.md §15` ordered the audit; this one acts on its result.

### What the audit established

All 2,712 T9 units classified against the DRAFT `docs/governance/ogl-pi-blacklist.md`:

| Bucket | Units | Share |
|---|---:|---:|
| Blocked — clearly Product Identity | 261 | 9.6% |
| Clear — safe under the draft | 1,107 | 40.8% |
| **Uncertain — the draft cannot resolve it** | **1,344** | **49.6%** |

Signing off the draft as-is would have unblocked only 40.8% and left half of T9 in a bucket the
document cannot decide either way.

### The ruling

> **Operator: option 2 — per-record review of the 1,344 uncertain units before any further sign-off.**

The blacklist stays `DRAFT` / `pending_operator_sign_off`. The review **proposes**; it does not amend
the blacklist and does not transcribe anything.

### What the review must resolve

Two of the audit's three recorded gaps are the direct cause of most of the uncertainty and must be
answered by the review's output:

1. **`companion` and `monster_ability` have no §2.3 field entry at all** — 443 and 359 units
   respectively, **802 units, 59.7% of the whole uncertain bucket**. The draft's per-record-judgment
   table names only `SpellCacheData` / `EquipmentCacheData` / `FeatTableEntry.description` /
   `RaceTraitEntry.detail`. Note that `companion` shows **0 blocked** — that is not "companion is
   safe", it is "no rule exists for companion".
2. **The 57-term scan is exact-substring, with no case-folding and no OCR normalization.** The
   blacklist's own §4 records a real incident (`Cayden CaiLean`, `lrori`) where two records shipped
   **un-redacted** because of exactly this, caught later by adversarial review rather than by the
   scan. The `clear` bucket of 1,107 inherits that limitation, so the review's remit includes
   re-checking `clear`, not only `uncertain`.

The third gap — `.MOD`/`.COPY` rows classified by their own line without tracing a referenced
target's PI status — is a smaller population but the same shape of hole, and gets an answer too.

### Standing constraints, unchanged

- **`decisions.md §15`'s rule stays in force for every shape, not only T9:** a cycle reaching a
  suspected Product Identity record stops on that record, lands everything else, and reports it by
  name. Never transcribe, never silently skip — a silent skip looks like completed work and leaves
  no trace that an agent made a licensing decision.
- The blacklist's own DRAFT banner governs the review: *"treat every classification below as a
  starting hypothesis, not a verified legal fact. When a real field's content doesn't obviously fit
  a bucket, stop and ask the operator rather than guessing."* A residual "still cannot decide"
  bucket is therefore a **legitimate and expected** output. Forcing 1,344 records into blocked/clear
  to produce a tidy number would be the worst available outcome.
- T9's onboarding stays paused until the operator acts on the review.

## Decision 17a — Correction: `pi_screen` had NOT drifted into three behaviours (2026-08-23)

**Status:** Correction of record, issued by the orchestrating session against its own claim in
`decisions.md §17`. `§17`'s ruling stands unchanged; one of its supporting facts was wrong.

### What §17 claimed

> *"The copies have **drifted** — `pi_screen` has three distinct implementations across the seven,
> and `ultimate_combat`'s is six lines shorter than the others. That is a live
> licensing-correctness defect, not just duplication, and it undermines the T9 PI audit's own
> `clear` bucket."*

### What is actually true

The three copies were **byte**-distinct and **logically identical**. Normalising whitespace and
comments, the only differences are line-wrapping and two inline comments:

```bash
diff <(git show 6ae4a364b:src/bin/ingest_ultimate_combat_spells.rs | awk '/^fn pi_screen/,/^}/' | grep -vE '^\s*//') \
     <(git show 6ae4a364b:src/bin/ingest_ultimate_magic_spells.rs  | awk '/^fn pi_screen/,/^}/' | grep -vE '^\s*//')
# only: one call wrapped across two lines; one struct literal expanded with `// filled by caller`
```

The "six lines shorter" figure was a **line count of a differently-wrapped copy**, not a behavioural
difference. The orchestrator's own hashing method produced three distinct digests because it
stripped only leading `//` comments, not trailing inline ones — a bad instrument, trusted without
validating it against a known case.

**There was no live licensing defect here, and the T9 PI audit's `clear` bucket was never
undermined by this.** `decisions.md §18`'s per-record review proceeded on a sound basis.

### What the collapse cycle did find, which is real

1. **A proof-coverage gap.** Deleting `|| name_blacklisted` from `pi_screen`'s guard left **every
   existing test green** across all seven binaries — none of them exercised the blacklist-only path
   (a record whose *name* is blacklisted with no declared PI token). The screen's most important
   branch was untested everywhere. Now covered by
   `pi_screen_drops_a_record_whose_name_is_blacklisted_with_no_declared_pi_token_at_all`,
   mutation-proved RED then GREEN.
2. **`min_level` had genuinely diverged.** `occult_adventures` and `ultimate_combat` lacked
   `DOMAINS:` support and PRESKILL/PREDEITY bracket-stripping. Re-derived against the pinned oracle:
   neither book's corpus exercises the gap (`grep -c 'DOMAINS:'` and `PRESKILL` are both 0), so
   unifying to the general form is output-neutral — but the divergence was real.

### The lesson

**Validate the instrument against a known case before trusting a confident claim it produces.** A
hash-comparison that reports "three distinct implementations" is a proxy; the claim it was used to
support — *behaviours differ* — needed one `diff` to check, and the `diff` refutes it. This is the
same failure shape as `decisions.md §14a`'s mock-based red-proof: a check that could not have
produced the answer it was read as producing.

Escalating an overstated defect costs real work: this one was written into a committed decision,
relayed to the operator as urgent, and used to justify a lane's priority.

## Decision 19 — T9 PI sign-off: all four blacklist amendments approved; the two open questions ruled (operator ruling 2026-08-23)

**Status:** Operator-pinned. Closes the chain `§15` (order the audit) → `§18` (order the per-record
review) → this. **`docs/governance/ogl-pi-blacklist.md` is signed off as amended** and stops being
`DRAFT` / `pending_operator_sign_off`. T9's onboarding pause is lifted for everything the rulings
below place in `clear`.

### 19a — Amendments 3a-3d, all approved

Verbatim text in `artifacts/gate-3-closure-invariant/t9-pi-signoff-package.md §3`. Applied to
`ogl-pi-blacklist.md`:

- **3a — new §2.3 entries for `companion` and `monster_ability`.** Neither kind had *any* field rule
  before, which is why 802 units could not be resolved either way. `companion`'s 443 uncertain rows
  were read corpus-wide and found to be entirely generic game mechanic; `monster_ability`'s rule ties
  judgment to the owning creature's PI status rather than the row's content in isolation.
- **3b — normalization rule.** Case-fold plus a bounded OCR-confusion table (`l`/`I`/`1`/`!` → one
  canonical character, `0`/`o`, `rn`→`m`), with **word-boundary matching, not bare substring**. Two of
  three lanes independently hit the `Nex`/`next` false positive and fixed it the same way. The PCGen
  field delimiter `|` must **never** enter the OCR table — folding it produces a false *negative* on
  the `Cayden CaiLean` incident itself, confirmed by direct test.
- **3c — `.COPY=`/`.MOD` inheritance.** A derivative row inherits its base item's declared
  `NAMEISPI:YES`/`DESCISPI:YES`. Both lanes that examined it reached this independently. Resolves 5
  units (Hellknight/Gray Maiden equipment, `clear` → `blocked`); identifies 6 more whose targets were
  never traced.
- **3d — term-list additions.** `Aldori` and `Magaambya`/`Magaambyan` added. The reviewing lane left
  this undecided because the terms appear in mechanical `PREABILITY` prerequisite fields rather than
  a record's own name or flavour, and judged that a legal call. **The operator has now made it.**

### 19b — The 954 `monster_ability` embedded-creature-name units: the row's own declaration governs

Ruled **clear**. A `monster_ability` row carrying no PI declaration and no term-list hit is not
Product Identity merely because its text names a Paizo-original creature.

**Recorded caveat, once, so it is not lost:** the review found PCGen's own data inconsistent here —
`Summon Monster IX (Cthulhu)` is declared `NAMEISPI:YES` as a **spell**, while the three
`Star-Spawn of Cthulhu` **monster_ability** rows for the identical creature carry no declaration.
Under this ruling "no declaration" is decisive, so that inconsistency now resolves in favour of
`clear`. It remains a **data-quality finding against the pinned oracle**, independent of the policy
question, and is not re-litigated by any later cycle without a new operator ruling.

### 19c — The ~360 generic-token units: widen the allowlist and re-run

Ruled **widen**. Rows flagged only for a generic token outside the classifier's allowlist
(`reflex`, `eidolon`, `swim`, `Adamantine`, `Mithral`, …) are stuck because the vocabulary is
incomplete, not because content was found — the consolidating lane's own read, which the operator
accepts.

**Binding condition:** the widening cycle **names every token it adds and why**, in its receipt. A
too-broad allowlist is precisely how a silent miss happens, and §4's recorded incident is the
program's own proof of that. A token added without a stated reason is a defect, not a shortcut.

### 19d — Consequences

1. `ogl-pi-blacklist.md` frontmatter moves to signed-off, dated 2026-08-23, citing this decision.
   Its DRAFT banner's standing instruction — *"when a real field's content doesn't obviously fit a
   bucket, stop and ask the operator rather than guessing"* — **survives sign-off unchanged** and
   continues to bind every cycle.
2. **`§15`'s standing rule stays in force for every shape, not only T9:** a cycle reaching a
   suspected Product Identity record stops on that record, lands everything else, and reports it by
   name. Never transcribe, never silently skip.
3. T9's real disposition is re-derived after 19a-19c are applied. The pre-ruling figures (266
   blocked / 1,988 clear / 1,319 undecidable) are superseded and must not be quoted as final.

## Decision 20 — `no_record` must reach ZERO. The budget is a ratchet, not a finish line (operator correction 2026-08-23)

**Status:** Operator correction of the orchestrating session. Corrects how `§14`'s Gate 3 budget has
been read since repin 1.

### What the operator said

> *"definition of done says we need to have every shape measured. if it isn't ingested, the shape
> cant be measured. it sounds to me like you have 20k items to ingest before you can you then need
> to measure those shapes. sd-32 needs to keep running until it reaches 100% the definition of done.
> please dont make me keep reminding you of that requirement"*

Correct, and the orchestrating session had been reporting "budget not exceeded" as if it were green.

### The chain that makes this dispositive

1. **`no_record` means the object was never ingested.** `scripts/shape_ledger.py` joins each
   inventory unit to `data/corpus/**/*.json` on `(book, source_basename, source_line)`. Three
   outcomes: `matched` (record found, carries `DEFINE:`/`BONUS:` tokens), `no_formula_tokens`
   (record found, genuinely no formula — a race description, a flavour feat), and **`no_record`
   (no record found at all)**. Only the third is a gap in our work rather than a fact about the object.
2. **An un-ingested object's shape cannot be measured.** There is nothing to classify. It is parked
   in family F0 by default, which is not a measurement.
3. **Gate 1's Definition of Done is that every unit's shape is measured.** So every `no_record` unit
   is an unmet Gate 1 criterion, whatever the budget says.

**Therefore: Gate 3's closure condition is `no_record == 0`, not "budget not exceeded."**

### What the budget actually is

The evidence-gated `NO_RECORD_BUDGET_COUNT`/`POPULATION` mechanism (`§14`, repins 1-4) stays — it
does a real job, catching drift and forcing every rise to carry committed evidence. But it is a
**transitional ratchet measuring progress toward zero**, not a terminal state. A passing budget with
a non-zero `no_record` means *"the backlog did not get worse"*, never *"the backlog is closed."*

**No cycle, receipt, or closure scan may report Gate 3 as met while `no_record` is non-zero.**

### The real remaining work — 20,889 objects, 18 kinds

Re-derived at `16300bde7` against the pinned oracle
(`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`, then count rows with
`join_status == "no_record"`):

| Kind | Units | | Kind | Units |
|---|---:|---|---|---:|
| `class_feature` | 5,604 | | `deity` | 459 |
| `ability` | 4,824 | | `power` | 421 |
| `template` | 2,248 | | `equipment` | 313 |
| `race_trait` | 1,913 | | `equipment_modifier` | 237 |
| `monster_ability` | 1,210 | | `domain` | 183 |
| `feat` | 1,202 | | `class` | 157 |
| `spell` | 860 | | `skill` | 149 |
| `companion` | 773 | | `monster` | 141 |
| | | | `language` | 136 |
| | | | `race` | 59 |

**Enumeration was half the job.** Card 15 made these objects *visible*; ingesting them is what makes
their shapes measurable. Treating card 15 as nearly complete because `remaining_undisposed: 0` was
the same error in a different place — that figure means every unit has a *disposition*, not that
every unit has been *ingested*.

### The standing instruction

**SD-32 runs until 100% of the Definition of Done.** The orchestrating session does not stop to ask
whether to continue, does not report a wave's completion as a stopping point, and does not treat a
passing budget or a green suite as closure. `decisions.md §10` already states the Definition of Done;
this decision states that reaching it is unconditional and that the operator should not have to
restate it.

The cost model is unchanged from `§17`: **per-mechanism, not per-object.** The `monster_ability`
cycle ingested 190 records by finding an existing config-driven pipeline rather than building one,
and the facet widening then landed 442 more by extending a vocabulary. Twenty thousand objects is
not twenty thousand cycles — it is however many distinct ingest mechanisms these eighteen kinds need,
and several already exist.

## Decision 21 — Duplicate-chooser-picker groups are ruled on as a CLASS, not id by id (operator ruling 2026-08-23)

**Status:** Operator-pinned. Answers the escalation in
`artifacts/gate-0-census-closure/15-card-15-duplicate-identity-review-memo.md`. **Amends SD-31
`decisions.md` Decision 17's "case by case" posture** for this specific shape — see 21c on why that
is not a reversal.

### The ruling

> **Operator: A — rule on the class.**

**The rule, in force:**

> Every fallback-key `class_feature` collision group whose members **all** carry a `TYPE:*Choice`
> facet **and** whose granted targets pairwise coincide is a **duplicate-chooser-picker group, not
> distinct objects**.

Such groups collapse: the picker rows are the same game concept as the feature they select, and are
removed from the unit ledger rather than counted as separate objects.

### 21a — What this closes

**39 collision groups, 113 rows, 74 residual ids**, across `advanced_class_guide` (27),
`ultimate_magic` (7), `advanced_race_guide` (2), `occult_adventures` (2), `monster_codex` (1).

The evidence, traced per group by the review cycle — **grant targets, not adjacency**:
- ACG's 27 groups: every member's `ABILITY:AUTOMATIC` target is either the Sorcerer chassis feature
  or the Bloodrager one, **never a third independent target**. The extra rows are Arcanist / Blood
  Arcanist / Crossblooded archetype pickers for the same feature.
- The 5 single-chassis groups: all **five** member rows (Sorcerer, Arcanist, Blood Arcanist,
  Crossblooded, Eldritch Heritage) converge on one target — those races and that monster only ever
  had a Sorcerer-chassis bloodline. One real feature, five duplicate pickers.
- `ultimate_magic`'s 7: both rows grant an identical target set, and **all seven surviving rows are
  already on `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS`**. The residual siblings are the other half of
  pairs the operator already ruled on in Decision 17 — invisible to that audit only because it worked
  from ids and these rows have none.

**Zero exceptions were found across all 39 groups.**

### 21b — Binding conditions on the implementation

The rule is broader than an id list, so it carries obligations an id list did not need:

1. **The predicate is exactly as stated.** All members carry a `TYPE:*Choice` facet **AND** granted
   targets pairwise coincide. Both conditions, every member. A group failing either is **not**
   covered and stays counted.
2. **Every collapse is logged, never silent.** The implementation emits a committed artifact naming
   every group it collapses and every row it removes, with book/file/line. Decision 17's real concern
   was *unreviewed* sweeps; a sweep whose every action is enumerated in a committed file is
   reviewable after the fact even though it was not reviewed before.
3. **A test proves the predicate cannot over-reach** — specifically, that a group whose members grant
   *different* targets is left alone. Prove it goes red by loosening the predicate to adjacency (the
   rule Decision 17 rejected), then revert.
4. **This is a denominator change** (`§1a`, "sum the piles"): report units before and after, prove
   nothing is lost beyond the named collapses, and re-run `scripts/card15_reconcile.py`.
5. **Scope is `class_feature` fallback-key collisions.** It does not extend to other kinds, to keyed
   collisions, or to any other duplicate shape without a further ruling.

### 21c — Why this is not a reversal of Decision 17

Decision 17 rejected *"a generic 'same name, adjacent line' rule"*, on the grounds that adjacency is
a heuristic rather than proof and would sweep in future collisions nobody reviewed. **That objection
stands, and this rule is not that rule.**

Adjacency asks *"are these two rows near each other?"*. This predicate asks *"do these rows hand the
player the same thing?"* — a semantic test on `ABILITY:AUTOMATIC` grant targets, which is the
evidence Decision 17 itself used to confirm its original 33. The class rule generalises Decision 17's
**reasoning**, not its heuristic.

The residual risk the operator accepted is real and named here: a future group matching this
predicate collapses without a human reading it. Condition 21b-2 is the mitigation — such a collapse
is enumerated in a committed artifact, so it is visible in review even though it was automatic.

## Decision 22 — Upstream data bugs are inherited, not perpetuated (operator ruling 2026-08-23)

**Status:** Operator-pinned. General doctrine, arising from the Decision 21 chooser-collapse but
**not limited to it**.

> *"If they are identically named, they are bugged already in pcgen. If those few things cause a
> breakage, I would rather carry a new bug into Codex that we can fix than perpetuate bad data."*

### The rule

**Where the PCGen corpus is internally inconsistent, Codex resolves the inconsistency rather than
faithfully reproducing it.** A defect we introduce in our own code is one we can find, test, and
fix. A defect we inherit by mirroring bad upstream data is permanent, invisible, and indistinguishable
from correct behaviour.

**A downstream breakage caused by resolving an upstream defect is an acceptable, expected cost**, and
is treated as an ordinary bug in Codex — not as evidence the resolution was wrong.

### What this settles

1. **Decision 21's collapse proceeds even where a consumer depends on the duplicate rows.** Two
   identically-named `class_feature` rows granting the same target are a PCGen data defect. If
   collapsing them breaks a picker, a count, or a test, **fix the consumer** — do not preserve the
   duplicate to keep the consumer quiet.
2. **The oracle's own PI inconsistency is a data-quality finding, not a policy question.** The T9 PI
   review found `Summon Monster IX (Cthulhu)` declared `NAMEISPI:YES` as a *spell* while three
   `Star-Spawn of Cthulhu` *monster_ability* rows for the identical creature carry no declaration
   (`§19b`). That is the same shape: an upstream inconsistency. `§19b`'s ruling stands on the policy
   question; this decision records that the inconsistency itself is Codex's to resolve where it
   affects our output, not to mirror.
3. **Corpus typos are fixed at ingest, and named.** A monster-ability cycle found 2 corpus typos and a
   comma-delimiter anomaly among its 86 unmodelled units. Under this decision those are candidates
   for correction at the ingest boundary — **provided the correction is recorded**, so a reader can
   see Codex diverging from the oracle and why.

### The binding condition — divergence must be visible

**Every deliberate divergence from the oracle is recorded where a reader will find it**: in the
ingesting code's own comment, in the cycle receipt, and — where it changes a shipped record — in a
committed artifact naming the records. Silent divergence is worse than faithful reproduction, because
it makes the corpus untrustworthy in an undetectable way.

This does **not** license loosening a gate, fabricating content, or "fixing" data that is merely
unfamiliar. `§1a` is unchanged: under-include rather than invent. The rule is about *inconsistency*
the corpus cannot itself resolve — two rows that cannot both be right — not about data we find
inconvenient.

### Relationship to fixture discipline

`§3` is unchanged and still governs: an emitted value's fixture is transcribed from bytes the
evaluator never reads. Where Codex deliberately diverges from the oracle, **the fixture records the
Codex value and the comment records the oracle value and the reason** — so the divergence is pinned,
not drifting.

## Decision 23 — `Domain Power` closes by reading the upstream class link; `Demonic Obedience` is re-typed (operator ruling 2026-08-23)

**Status:** Operator-pinned. Closes the two labels a T2a-residual cycle verified and deliberately
left unmapped (`artifacts/gate-3-closure-invariant/epic-2-t2a-residual-alias-tier_cycle-1_cycle_receipt.md`).
**Sets the precedent for the remaining ~525 unverified labels** — see 23c.

### 23a — `Domain Power` (172 units): option (a), extend the generator's inputs

The label is **genuinely multi-owner** and the corpus says so, verified corpus-wide rather than
sampled: 158 of 172 records' `DESC` names no class at all; the 14 that do split 13 Cleric-only and 1
Cleric+Druid; and the `PRE:`/`TYPE:` tokens (`DomainLawLVL`, `SpecialQuality.DomainPower`) are
generic across every class with domain access — Cleric, Inquisitor's Inquisition, Warpriest's
Blessing-domain hybrid, Paladin's Sacred Servant archetype.

**Ruled: close it by reading the source the generator does not currently read** — which specific
class build each domain-power-granting deity/domain entry attaches to in the PCGen source tree.

The operator rejected the cheaper option (b), "rule that *shared across domain-access classes* is
itself an acceptable disposition", because it closes the counter without learning which class grants
what — a gap that surfaces again the moment an engine tries to compute these values. **This is a real
mechanism extension, deliberately chosen over a definitional shortcut.**

`CATEGORY_LABEL_ALIASES`' single-label-to-single-class shape does **not** fit here and must not be
forced (`§1a`: that is a relabelled shape, not a closed one). The standing test
`category_label_alias_owner_refuses_the_known_multi_owner_and_not_class_owned_labels` pins that
refusal; it may only be amended by a cycle that has actually built the upstream link, and the
amendment must be visible in the diff.

### 23b — `Demonic Obedience` (42 units): re-type out of `class_feature`

**Not class-owned at all**, and the evidence is one-directional with no exceptions: every one of the
42 records' `PRE:` tokens names a **demon lord** (`Shivaska`, `Jubilex`, …), never a class or a
class-shaped variable. It is a deity-obedience feat line, structurally outside any PC class chassis —
comparable to a boon feat.

**Ruled: re-type it.** These units sit under `class_feature` because that is where the category label
landed, not because they belong there. The closure is a `kind` correction, not a class mapping.

This is a `§16` movement, so it binds: **a unit re-typed out of `class_feature` is not a unit closed.**
Name the kind it moves to, prove the move, and report closure and reclassification as separate
numbers. If the correct target kind does not exist, say so rather than forcing it into the nearest
one.

### 23c — The precedent this sets

**1,612 units across roughly 525 more category labels remain individually unverified.** The two
labels ruled here are the two shapes that population will keep producing, and this decision is how
each is handled:

| Shape | Disposition |
|---|---|
| **Multi-owner** — a real class feature whose owning class is not determinable from the generator's current inputs | Extend the inputs and read the real link (23a). Do **not** pick one owner, and do **not** define the ambiguity away. |
| **Not class-owned** — the units are not class features at all | Re-type them (23b), reporting the movement per `§16`. |
| **Single-owner** | The existing `CATEGORY_LABEL_ALIASES` path, verified across all a label's records (the 21 labels / 814 units already closed that way). |

A cycle working the remaining labels applies this table rather than re-escalating each one. Only a
label that fits **none** of the three shapes is a new escalation.

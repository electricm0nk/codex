---
canonical: true
owner: god-emporer
status: planning-ready (operator directive 2026-08-10 -- re-scoped to class_feature/archetype)
date: 2026-08-10
canonical_branch: tranche/10 (operator directive 2026-08-01, unchanged)
build_version_target: 0.10.<build>
companion_to: ./scope-draft.md, ./decisions.md
---

# SD-30 Epic Breakdown

**Re-cut 2026-08-10 (`decisions.md §33-38`).** The sixteen per-book content epics this file
previously carried (Epics 3-18 under the old numbering, matching `kanban.md`'s 21 cards) are
retired — that book list dissolved into SD-29's corpus-wide scope. SD-30's epics now follow the
**dependency chain `decisions.md §37` establishes**, not a book list: measurement gates mechanism
gates chassis sweep. 9 epics total, matching `kanban.md`'s 9 cards.

Epic 1 fires FIRST. Closure fires LAST. Epic 4 (measurement) gates Epic 5 (mechanism) and Epic 6
(chassis sweep) **per class**, not bundle-wide — see `decisions.md §37`.

## Epic 1 (SD30-E1) — Code-Side Identifier Cleanup

**Objective:** Establish identifier discipline across all code this bundle introduces.

**Derived from:** `decisions.md §7` (Identifier discipline, unchanged by the re-scope).

### Feature seeds

#### SD30-E1-F1 — Identifier-disclosure audit pass

Acceptance:

- No `sd30_*`, `SD30_*`, `Sd30*`, `sd30-*` patterns in the surface code (`src/rules_core/rules_tables/<book>/`
  for the 23 `class_feature`-bearing books, `archetype_resolver.rs`, `pilot_compute.rs`).
- No `t_<hex>` kanban tokens.
- Identifier-discipline audit script returns 0 findings.

## Epic 2 (SD30-E2) — Operator Pre-Launch

**Objective:** Pre-launch checklist verification + cycle-0 trap-report + work-inventory validation,
re-scoped to `class_feature` across all 23 books this kind touches (not the old sixteen).

**Derived from:** `loop-instruction.md §"Pre-launch checklist"` + `decisions.md §33`.

### Feature seeds

#### SD30-E2-F1 — Local-file dispatch readiness

Acceptance:

- `kanban.md` lists the 9 re-cut epics as ready/gated cards, matching this file.
- `progress.md` carries a re-scope receipt for 2026-08-10.
- Working tree clean.

#### SD30-E2-F2 — Branch state + cycle-0 trap-report + work-inventory, corpus-wide for `class_feature`

Acceptance:

- Branch `tranche/10` pushed to origin (unchanged from the old scope).
- `cargo run --locked --bin v06_work_inventory` regenerated `docs/work-inventory.json`; the 23
  `class_feature`-bearing books' entries confirm per-book shape (re-derive the book list at cycle-0 —
  `decisions.md §33`'s 23-book table is a snapshot, not a hand-maintained constant).
- `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>` ran for every book Epic 4/6
  actually targets that cycle; output recorded in `artifacts/<book>-cycle0-trap-report.md`. Not all
  23 books need a trap-report before Epic 1/2 close — only the ones Epic 4's next measurement pass
  or Epic 6's next chassis-sweep cycle claims.

## Epic 3 (SD30-E3) — PI-Screening Provenance Gate

**Objective:** Resolve the blocking provenance question before any `class_feature` content lands a
record in `src/rules_core/rules_tables/<book>/`, mirroring SD-29 Epic 3 exactly — same pipeline,
same starting state.

**Derived from:** `SD-29-corpus-wide-catch-up-lanes/epic-breakdown.md` "Epic 3 (SD29-E3) — Provenance
Gate: PI-Screening for Kind-Lane Ingestion" (same shape, applied to `class_feature` instead of
SD-29's kinds); `docs/governance/license-matrix.md` (commit `314a7ad9`, all 37 books) for OGL/
attribution/publisher provenance, cited not re-derived.

**Status:** Gates Epics 5 and 6.

### What is already resolved, cited not re-done

OGL / attribution / publisher provenance for every book in `class_feature`'s 23-book population is
established by `docs/governance/license-matrix.md`'s per-book table. A cycle cites the matrix row for
the book it is touching; it does not re-derive OGL status per cycle.

### What this epic actually gates

The same finding SD-29 Epic 3 made applies unchanged here, because it is a Pipeline-B finding, not a
kind-specific one: `rules_tables/*.rs` has **zero PI-screening anywhere in the repo**, and a direct
sweep already found three real, unredacted Product-Identity leaks in other bundles' Pipeline B
tables. `class_feature` records are exactly the shape of content those leaks came from (named,
copyrighted class-feature text, ability names, and flavor text carried through from the corpus).

### Feature seeds

#### SD30-E3-F1 — Per-class PI-blacklist sweep wired in

Acceptance:

- Every Epic 6 chassis-sweep cycle calls `pi_screening::classify_field` (or runs the 55-term
  blacklist sweep as a pre-commit check) against that cycle's newly-generated `class_feature` content
  before it lands in `rules_tables/`, for whichever book the cycle is touching.
- The sweep's output (clean, or hits found) is recorded in the cycle's first receipt per book, in
  `progress.md`.
- A hit is a hard stop for that record (`loop-instruction.md` "Stop vs. press on"), not routed around.

#### SD30-E3-F2 — Declared-PI reader wired into the `class_feature` ingest/transcription path (2026-08-13, `decisions.md §39`)

**Blocks Epic 6 entirely — no per-book chassis-sweep cycle may claim its class until this feature
seed is `COMPLETE`.** `decisions.md §39.2` re-derived 464 source rows across 6 of the 23 in-scope
books (`adventurers_guide` 276, `inner_sea_magic` 67, `inner_sea_world_guide` 49, `inner_sea_intrigue`
45, `book_of_the_damned_volume_2` 18, `inner_sea_combat` 9) that declare `NAMEISPI:YES` or
`DESCISPI:YES` in PCGen source and would ship past the 55-term blacklist the same way SD-29's
race-trait lane's 8 undeclared-term rows did before its own fix (`SD-29-corpus-wide-catch-up-lanes/decisions.md §53`).

Acceptance:

- Whichever ingest binary or Python transcriber Epic 6 builds for `class_feature` calls
  `pi_screening::{declared_product_identity, classify_optional_field_declared}` (already implemented,
  7 unit tests, `src/rules_core/pi_screening.rs`) — the same reader `ingest_race_traits` already
  uses, not a new implementation.
- `NAMEISPI:YES` rows are dropped **before** the scope filter, named by file:line in the run receipt
  (mirrors `ingest_race_traits`'s `dropped, NAMEISPI:YES : N` line).
- `DESCISPI:YES` descriptions are redacted through the shared reader and counted in the receipt
  (mirrors `descriptions redacted by DESCISPI:YES : N`).
- This is a **production-path** change to the ingest/transcription binary itself — not a fixture, not
  a script run by hand outside the pipeline that generates `data/corpus/`.
- Reclassifying a specific declared-PI row as shippable is `ogl-pi-blacklist.md` §3's per-book
  override — an operator decision a cycle may request but not make unilaterally.

#### SD30-E3-F3 — Corpus-wide declared-PI backfill sweep, every already-shipped kind

`decisions.md §53.7`'s own scope finding named this as "the successor's first move": the shared
reader is placed in `pi_screening` but only `ingest_race_traits` calls it; every other Pipeline A
writer and the entire Pipeline B transcription pipeline (`transcribe_monster_tables.py` — drops
`NAMEISPI:YES`, per lines 780/818, but has never read `DESCISPI:YES`; `transcribe_companion_tables.py`
— reads neither token) still screen on the term list alone or not at all.

Acceptance:

- Run `decisions.md §39.2`'s corpus-wide sweep command (`raw_tokens` scan over
  `data/corpus/*/*/*/*.json`, all kinds) at the start of this feature seed's cycle, not transcribed
  from `§39.2`'s 2026-08-13 result (currently zero hits outside the already-fixed `race_trait`, but
  this is a point-in-time fact that must be re-checked at time of use, per this program's own
  standing-defect pattern).
- Any hit found is resolved the same way `§53` resolved race-trait's: `DESCISPI:YES` → redact and
  regenerate that book's corpus files; `NAMEISPI:YES` → drop the row, re-derive every pinned count the
  drop touches (mirrors `§53.3`'s nine-file re-pin), do not decrement by hand.
- `transcribe_monster_tables.py` gains `DESCISPI:YES` handling (redaction) alongside its existing
  `NAMEISPI:YES` drop; `transcribe_companion_tables.py` gains both, scoped to whatever ISPI exposure
  its own source sweep finds (1 `NAMEISPI:YES` row measured in `decisions.md §39` at
  `dtt_races_companion.lst`, re-derive at time of use).

#### SD30-E3-F4 — Regression gate: a future ingest cannot reintroduce a declared-PI leak

`scripts/verify.sh`'s `pi-sweep` stage and `docs/governance/ogl-pi-blacklist.md` are both
term-blacklist mechanisms; neither reads `NAMEISPI`/`DESCISPI`. Reading a declaration and scanning for
undeclared terms are different questions (`decisions.md §53.1` — "the two are now a union") and need
a sibling check, not a merge into either existing mechanism.

Acceptance:

- A corpus-level test, following `tests/sd29_declared_product_identity_in_shipped_race_traits.rs`'s
  shape exactly (reads shipped files, not source rows, so both ends read the same bytes), extended to
  walk `*/class_feature/*` once that path exists under `data/corpus/`.
- The test is wired into `scripts/verify.sh` (either as its own stage or folded into an existing
  full-corpus stage — cycle's choice, but it must run in `verify.sh full`, mirroring `pi-sweep`'s
  "cheap enough to also run in `quick`" reasoning if the walk stays cheap).
- The gate fails RED, not warns, if a declared-PI row is found unredacted/undropped in shipped output.

## Epic 4 (SD30-E4) — Per-Class Archetype Measurement (GATES Epics 5 and 6)

**Objective:** Extend SD-28 `§63`/`§64`'s hand-verification method — find each named archetype slot's
base computation in `pilot_compute.rs`, confirm it is unconditional (level-gated only), confirm the
slot name maps to a real id, report per class, never blended, no automated proxy — to every
`class_feature`-bearing class this bundle has not yet measured. Also owns characterizing the
2,958-unit `unknown` bucket (`decisions.md §38`) and designing the chooser-interaction primitive for
the 3 classes `§64` excluded (Oracle, Arcanist, Sorcerer).

**Derived from:** `decisions.md §34, §37, §38`; `SD-28-ultimate-book-content-ingestion/decisions.md
§60, §63, §64`; `corpus-work-channels.md §9.1` (the FUNDED effort this epic is the SD assignment for).

**Status:** Gates Epic 5 (mechanism) and Epic 6 (chassis sweep) **per class** — a specific class's
Epic 6 cycle cannot be scheduled until this epic has produced that class's `wired-able / named`
figure by direct evidence.

### Inherited starting state — verify before extending, do not re-measure

- **25 of 28 archetype-bearing classes already hand-verified** (Fighter, Alchemist, Ranger, Cleric,
  Druid, Rogue, Monk, Paladin, Bard, Cavalier, Witch, Shaman, Warpriest, Summoner, Skald, Barbarian,
  Inquisitor, Hunter, Bloodrager, Wizard, Investigator, Brawler, Swashbuckler, Familiar, Companion) —
  263 wired-able / 475 named slots, 175 mechanisms after collapsing duplicate slot-tier names.
- **3 classes excluded, own wiring shape:** Oracle, Arcanist, Sorcerer — choice-based, need the
  chooser-interaction primitive, not `archetype_claims_slot`.
- **11 unmodelled base-class features found incidentally** (`decisions.md §34` lists all eleven) —
  recorded so this epic does not rediscover them as new findings.

### Feature seeds

#### SD30-E4-F1 — Class inventory: which classes remain unmeasured

Acceptance:

- A direct enumeration (grepped, not estimated) of every class with `class_feature` records across
  the 23 in-scope books, cross-referenced against SD-28 `§64`'s 28-class list, producing a named
  remainder (expected: Occultist, Spiritualist, Medium, Mesmerist from Occult Adventures at minimum;
  Mythic Adventures path-tier features if any resolve to `class_feature`; any Inner Sea archetype
  content not already covered as a tier-1/tier-2 archetype swap).
- Report is per-class, cites the corpus files grepped, and does not assume book-of-origin implies
  class-of-origin (the class-grant boundary with SD-28, `decisions.md §5`, is a name/identity join,
  not a book join).

#### SD30-E4-F2 — Per-class hand-verification, extended

Acceptance:

- Each newly-measured class reports `wired-able / named` by direct evidence in `pilot_compute.rs` (or
  the class's own compute module), the same method as `§63`/`§64` — no id-string proxy, no
  extrapolation from another class's ratio.
- No blended percentage reported across classes, ever (per `§64`'s standing discipline).
- A near-miss slot name is not counted as evidence without direct confirmation.

#### SD30-E4-F3 — Chooser-interaction primitive design (Oracle, Arcanist, Sorcerer)

Acceptance:

- A design decision (not yet a measurement) on what a "which options remain choosable, and does the
  substitute grant compute" primitive looks like, distinct from `archetype_claims_slot`'s supersession
  shape.
- Once designed, Oracle/Arcanist/Sorcerer are measured by the same no-proxy, per-class standard as the
  supersession-shape classes, producing their own `wired-able / named` figures.

#### SD30-E4-F4 — `unknown`-bucket characterization, per class

Acceptance:

- For each class this epic measures, its share of the 2,958-unit `unknown` bucket is characterized
  using SD-28 `§52`/`§53`'s already-proven distinction: option-pool sub-choice content (real chooser,
  canonical-narrowing is the deliberate design, not a gap) vs. genuinely-unreachable content (no
  chooser code at all, needs net-new engine work) vs. residual unclustered content (not yet
  characterized).
- The 303-unit genuinely-unreachable subset (Vigilante, Ultimate Psionics disciplines) and the
  1,772-unit unclustered remainder SD-28 left open are tracked as named backlog items, not dropped.
- No `unknown` unit's status is changed by this feature seed alone — characterization, not
  reclassification, per SD-28 `§52`'s standing constraint (a classifier-taxonomy change is a
  separate, explicit decision if the operator authorizes it).

## Epic 5 (SD30-E5) — Archetype Mechanism (supersession shape now, chooser shape when Epic 4 funds it)

**Objective:** Wire the 175-mechanism / ~5,775-line supersession shape (`archetype_claims_slot`) for
each of the 25 measured classes as Epic 4 clears them for scheduling; design and wire the
chooser-interaction shape for Oracle/Arcanist/Sorcerer once Epic 4-F3 resolves its primitive.

**Derived from:** `decisions.md §34`; `SD-28-ultimate-book-content-ingestion/decisions.md §59, §60,
§63, §64`.

**Status:** Gated on Epic 4 clearing the specific class(es) a cycle targets. Gates Epic 6 for the same
class (ingestion is sequenced after wiring per class, `decisions.md §37`).

### Feature seeds

#### SD30-E5-F1 — Supersession-shape wiring, per measured class

Acceptance:

- For each of the 25 measured classes, the `if let`/`else` supersession branch replacing the prior
  unconditional feature-grant block lands in `pilot_compute.rs`, one mechanism at a time (per `§60`'s
  proven pattern on Alchemist/Fighter).
- Reachability proven per SD-28 `§43`'s standard: a headless pilot receipt test through
  `build_pilot_headless_receipt`, not a unit test on the resolver alone — bare-class baseline,
  archetype supersedes with the archetype named in the explanation, a non-matching archetype leaves
  the base grounding unchanged.
- No `§59` vacuity-audit citation touched without being addressed (`§59`'s backlog is a prerequisite
  check, not a blocker to route around).

#### SD30-E5-F2 — Chooser-interaction primitive, once designed

Acceptance:

- `archetype_resolver.rs` (or a sibling module) gains the chooser-interaction primitive Epic 4-F3
  designs.
- Oracle/Arcanist/Sorcerer's measured wireable mechanisms wire through it, proven reachable the same
  way as the supersession shape.

**Instrument-coverage note (2026-08-13, `decisions.md §41`).** Sibling bundle SD-32 landed corpus-wide
`static`/`derived` gates on `tranche/9` today (`scripts/verify.sh` `corpus-sweep`/
`corpus-sweep-selftest`; `tests/derived_evaluator_fixture_check.rs`). Neither this epic nor Epic 6 was
ever scoped to build its own such gate — re-read of this file and `acceptance-and-verification.md`
found no prior criterion naming one — so there is no stale acceptance criterion to rewrite here.
Recorded so a future cycle does not duplicate that instrument: once Epic 6 ships `class_feature`
records, running `./scripts/verify.sh` (already `AT-30-002`/Decision §18's standing requirement)
exercises both gates against them for free. The `computed` bucket (4,178 of 15,472 `class_feature`
units) has no equivalent gate — SD-32's spell consumer-delta probe is a precedent for the shape, not
a `class_feature` instrument — and building one is flagged for the operator, not scheduled here.

## Epic 6 (SD30-E6) — Per-Class Chassis Sweep (the `class_feature` ingest, gated per class on Epics 4/5)

**Objective:** The actual per-book, per-class `class_feature` ingest cycles across all 23 in-scope
books — the direct successor to the old per-book epics 3-18, but scoped by class and gated by
measurement rather than dispatched book-parallel from cycle one.

**Derived from:** `decisions.md §33, §35, §37`.

### Feature seeds

#### SD30-E6-F1 — Per-class ingest, scheduled by Epic 4's clearance order

Acceptance:

- A cycle-batch targets one class (or a small set of related classes in one book) only after Epic 4
  has produced that class's `wired-able / named` figure and Epic 5 has wired its supersession (or
  chooser-interaction) mechanisms.
- Canonical `class_feature` records land in `src/rules_core/rules_tables/<book>/`, one per real
  feature, matching the class's measured wireable set — not the book's full named-slot list.
- Reach-gate claim executes the real IPC builder for each record (`apps/desktop/src-tauri/src/reach_gate.rs`).

#### SD30-E6-F2 — Occultist/Spiritualist/Medium/Mesmerist canonical definitions (class-grant boundary with SD-28)

Acceptance:

- SD-30 owns the canonical class definition for these four shared classes per `decisions.md §5`
  (unchanged by the re-scope) — SD-28 references, does not redefine.
- Cross-book conflict rule (`decisions.md §16`) applies if Ultimate Intrigue and Occult Adventures
  wording diverges on the same class.

#### SD30-E6-F3 — `unknown`-bucket disposal, per Epic 4's characterization

Acceptance:

- Units Epic 4-F4 characterizes as option-pool content with a real chooser get their representative
  option grounded (canonical narrowing), the rest deferred with a named diagnostic — not bulk-worked
  to zero.
- Units characterized as genuinely unreachable are NOT silently ingested as if grounded; they are
  named findings routed to a successor (net-new engine work, out of an ingest cycle's own scope
  unless the operator explicitly funds the engine change inside this bundle).

## Epic 7 (SD30-E7) — Build Version Numbering

**Objective:** unchanged from the old scope.

**Derived from:** `decisions.md §15`.

### Feature seeds

#### SD30-E7-F1 — Version patch

Acceptance:

- First concrete value: `0.10.<build>` (read from current build counter at cycle close).
- Closing-PR iteration on Closure increments per the 2026-07-17 build-version amendment.

## Epic 8 (SD30-E8) — Bundle Code Review

**Objective:** A full code review of the bundle's entire diff against its branch point, run after
Epics 5, 6, and 7 close — not in parallel with them, and not scoped to only the final cycle.
`./scripts/verify.sh` passing is a precondition, not the review itself.

**Derived from:** operator directive 2026-08-01 + `decisions.md §26` (unchanged by the re-scope).

### Feature seeds

#### SD30-E8-F1 — Whole-bundle diff review

Acceptance:

- The reviewed diff scope is the bundle's full change set against its branch point
  (`git diff origin/develop...HEAD`), not the closing cycle's slice alone.
- `./scripts/verify.sh` has a recorded green run for that diff.
- `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh` re-run at bundle
  scope.

#### SD30-E8-F2 — Correctness, no-stub, reach, test-quality sweep

Acceptance:

- A sample of this bundle's `class_feature` rules logic is checked against the source corpus across
  the 23 in-scope books; disagreements are recorded as findings.
- No stub, fixture-only, or mock data in a production path per `docs/governance/no-stub-mvp-doctrine.md`.
- A sample of records claiming to reach a player surface is spot-checked against `reach_gate.rs`'s
  `OPEN_FINDINGS` mechanism and the live IPC/UI path.
- Test quality checked per `docs/governance/book-ingestion-playbook.md §7.4` — a sample of new
  gates/tests actually fails when the thing it protects is broken.
- No hand-authored rules data under `apps/desktop/src/`.

#### SD30-E8-F3 — Findings triage

Acceptance:

- Every finding records a severity and a disposition: `fixed-in-bundle` or `deferred`.
- A `deferred` finding names an owner and is entered in `forward-scope-register.md`.
- Real defects found are fixed in-bundle before Closure fires.
- A `scripts/retro.py` event is emitted per finding, carrying `--verified-by`.

## Epic 9 (SD30-E9) — Closure Epilogue

### Feature seeds

#### Closure-F1 — Closure cycle

Acceptance:

- Epic 4 has either covered every `class_feature`-bearing class or named its own successor for what
  remains (measurement is not required to reach 100% before closure — the operator may fund a
  successor for the remainder, per the same pattern `§63`/`§64` used).
- Epics 5-8 `complete` in `progress.md`.
- `release-notes.md` populated.
- Tranche promotion PR fires: `tranche/10 → develop`; `0.10.<last_build>` remains the post-closure
  value.

#### Closure-F2 — Workspace-tree removal (move-not-copy) — LANDED under the old slug, verify at closure

Acceptance:

- The source-of-record workspace directory was removed on the original 2026-08-01 publish commit; the
  2026-08-10 rename (`git mv` to `SD-30-class-feature-archetype-bundle`) does not re-trigger or
  require a second removal.
- The canonical repo-resident home is `docs/release/SD-30-class-feature-archetype-bundle/`.

## Recommended sequencing (dependency order, not exclusive scope)

```
E1 -> E2 -> E3 -> E4 (per-class measurement, ongoing) -> E5 (per-class mechanism, gated per class
   on E4) -> E6 (per-class chassis sweep, gated per class on E4+E5) -> E7 (Version) -> E8 (Bundle
   Code Review) -> E9 (Closure)
```

Unlike the old book-parallel diagram, **E4/E5/E6 do not run bundle-wide in one pass** — they cycle
per class, with E4 always leading E5 and E5 always leading E6 for any given class. Different classes'
E4/E5/E6 triples can run concurrently (file-disjoint by class and by `src/rules_core/rules_tables/<book>/`
path) under operator-pinned concurrency, the same way the old per-book epics were file-disjoint by
book. E3 (PI-screening) is a standing gate re-invoked by every E6 cycle, not a one-time epic.

## Completion gate

SD-30 closes when:

- Epic 4 has measured every `class_feature`-bearing class (or the operator has funded/named a
  successor for the remainder) and characterized the `unknown` bucket per class.
- Epic 5 has landed the 175-mechanism supersession shape for the 25 originally-measured classes plus
  whatever Epic 4 adds, and has either wired or explicitly deferred the chooser-interaction shape for
  Oracle/Arcanist/Sorcerer.
- Epic 6's chassis sweep has ingested and reach-gated every class Epic 4/5 cleared.
- Epic 3's PI-screening gate ran clean (or recorded and resolved hits) on every book touched,
  including the declared-PI reader (SD30-E3-F2), the corpus-wide backfill sweep (SD30-E3-F3), and the
  regression gate (SD30-E3-F4) — not the 55-term blacklist sweep alone (`decisions.md §39`).
- Epic 8 (Bundle Code Review) closed, all findings triaged with named owners for deferrals.
- Epic 9 (Closure) fires.
- `progress.md` carries the closure receipt.
- `release-notes.md` is populated.
- The tranche-promotion PR `tranche/10 → develop` is opened and merged.
- `docs/release/SD-30-class-feature-archetype-bundle/` carries the canonical file chassis.

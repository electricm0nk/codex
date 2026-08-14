---
canonical: true
owner: god-emporer
status: planning-ready (split from SD-30, operator ruling 2026-08-14)
date: 2026-08-14
canonical_branch: tranche/10
build_version_target: 0.10.<build>
companion_to: ./scope-draft.md, ./decisions.md
---

# SD-31 Epic Breakdown

**Moved from `SD-30-class-feature-archetype-bundle/epic-breakdown.md` Epics 4, 5, 6, 10, 11, and 14
(operator ruling 2026-08-14, "split phase 3 and phase 4 into their own SD's").** Feature-seed content
below is reproduced from the origin epics with IDs renumbered `SD30-E{4,5,6,10,11,14}-F*` →
`SD31-E{1..6}-F*`; each section's opening line records its SD-30 origin. Acceptance criteria are
otherwise unchanged from what SD-30 had measured/decided as of the split.

Epic 1 fires first (measurement gates everything downstream, per-class). Epic 4 (ingest lanes) and
Epic 5 (book onboarding) are independent of the Epic 1/2/3 `class_feature` chain — different kinds/
books — but share the same cross-SD PI-gate dependency on `SD-30-class-feature-archetype-bundle`'s
Epic 3.

## Epic 1 (SD31-E1) — Per-Class Archetype Measurement (moved from SD30-E4; GATES Epics 2 and 3)

**Objective:** Extend SD-28 `§63`/`§64`'s hand-verification method — find each named archetype slot's
base computation in `pilot_compute.rs`, confirm it is unconditional (level-gated only), confirm the
slot name maps to a real id, report per class, never blended, no automated proxy — to every
`class_feature`-bearing class this bundle has not yet measured. Also owns characterizing the
`unknown` bucket (`SD-30-.../decisions.md §38`) and designing the chooser-interaction primitive for
the 3 classes excluded so far (Oracle, Arcanist, Sorcerer).

**Derived from:** `SD-30-class-feature-archetype-bundle/decisions.md §34, §37, §38`;
`SD-28-ultimate-book-content-ingestion/decisions.md §60, §63, §64`; `corpus-work-channels.md §9.1`.

**Status:** Gates Epic 2 (mechanism) and Epic 3 (chassis sweep) **per class** — a specific class's
Epic 3 cycle cannot be scheduled until this epic has produced that class's `wired-able / named` figure
by direct evidence.

### Inherited starting state — verify before extending, do not re-measure

- 25 of 28 archetype-bearing classes already hand-verified at split time (Fighter, Alchemist, Ranger,
  Cleric, Druid, Rogue, Monk, Paladin, Bard, Cavalier, Witch, Shaman, Warpriest, Summoner, Skald,
  Barbarian, Inquisitor, Hunter, Bloodrager, Wizard, Investigator, Brawler, Swashbuckler, Familiar,
  Companion) — 263 wired-able / 475 named slots, 175 mechanisms after collapsing duplicate slot-tier
  names.
- 3 classes excluded, own wiring shape: Oracle, Arcanist, Sorcerer — choice-based, need the
  chooser-interaction primitive, not `archetype_claims_slot`.
- 11 unmodelled base-class features found incidentally (`SD-30-.../decisions.md §34` lists all eleven)
  — recorded so this epic does not rediscover them as new findings.

### Feature seeds

#### SD31-E1-F1 (was SD30-E4-F1) — Class inventory: which classes remain unmeasured

Acceptance:

- A direct enumeration (grepped, not estimated) of every class with `class_feature` records across
  the 23 in-scope books, cross-referenced against SD-28 `§64`'s 28-class list, producing a named
  remainder (expected: Occultist, Spiritualist, Medium, Mesmerist from Occult Adventures at minimum;
  Mythic Adventures path-tier features if any resolve to `class_feature`; any Inner Sea archetype
  content not already covered as a tier-1/tier-2 archetype swap).
- Report is per-class, cites the corpus files grepped, and does not assume book-of-origin implies
  class-of-origin (the class-grant boundary with SD-28, `decisions.md §5`, is a name/identity join,
  not a book join).

#### SD31-E1-F2 (was SD30-E4-F2) — Per-class hand-verification, extended

Acceptance:

- Each newly-measured class reports `wired-able / named` by direct evidence in `pilot_compute.rs` (or
  the class's own compute module), the same method as `§63`/`§64` — no id-string proxy, no
  extrapolation from another class's ratio.
- No blended percentage reported across classes, ever.
- A near-miss slot name is not counted as evidence without direct confirmation.

#### SD31-E1-F3 (was SD30-E4-F3) — Chooser-interaction primitive design (Oracle, Arcanist, Sorcerer)

Acceptance:

- A design decision (not yet a measurement) on what a "which options remain choosable, and does the
  substitute grant compute" primitive looks like, distinct from `archetype_claims_slot`'s supersession
  shape.
- Once designed, Oracle/Arcanist/Sorcerer are measured by the same no-proxy, per-class standard as the
  supersession-shape classes, producing their own `wired-able / named` figures.

#### SD31-E1-F4 (was SD30-E4-F4) — `unknown`-bucket characterization, per class

Acceptance:

- For each class this epic measures, its share of the `unknown` bucket is characterized using SD-28
  `§52`/`§53`'s already-proven distinction: option-pool sub-choice content (real chooser,
  canonical-narrowing is the deliberate design, not a gap) vs. genuinely-unreachable content (no
  chooser code at all, needs net-new engine work — routed to `SD-32-engine-capability-builds/`'s
  verdict-path epic if it needs a new capability) vs. residual unclustered content (not yet
  characterized).
- No `unknown` unit's status is changed by this feature seed alone — characterization, not
  reclassification, per SD-28 `§52`'s standing constraint.

## Epic 2 (SD31-E2) — Archetype Mechanism (moved from SD30-E5)

**Objective:** Wire the 175-mechanism / ~5,775-line supersession shape (`archetype_claims_slot`) for
each measured class as Epic 1 clears them for scheduling; design and wire the chooser-interaction
shape for Oracle/Arcanist/Sorcerer once Epic 1-F3 resolves its primitive.

**Derived from:** `SD-30-class-feature-archetype-bundle/decisions.md §34`;
`SD-28-ultimate-book-content-ingestion/decisions.md §59, §60, §63, §64`.

**Status:** Gated on Epic 1 clearing the specific class(es) a cycle targets. Gates Epic 3 for the same
class (ingestion is sequenced after wiring per class).

### Feature seeds

#### SD31-E2-F1 (was SD30-E5-F1) — Supersession-shape wiring, per measured class

Acceptance:

- For each measured class, the `if let`/`else` supersession branch replacing the prior unconditional
  feature-grant block lands in `pilot_compute.rs`, one mechanism at a time.
- Reachability proven per SD-28 `§43`'s standard: a headless pilot receipt test through
  `build_pilot_headless_receipt`, not a unit test on the resolver alone.
- No `§59` vacuity-audit citation touched without being addressed.

#### SD31-E2-F2 (was SD30-E5-F2) — Chooser-interaction primitive, once designed

Acceptance:

- `archetype_resolver.rs` (or a sibling module) gains the chooser-interaction primitive Epic 1-F3
  designs.
- Oracle/Arcanist/Sorcerer's measured wireable mechanisms wire through it, proven reachable the same
  way as the supersession shape.

## Epic 3 (SD31-E3) — Per-Class Chassis Sweep (moved from SD30-E6; the `class_feature` ingest, gated per class on Epics 1/2)

**Objective:** The actual per-book, per-class `class_feature` ingest cycles across all 23 in-scope
books, scoped by class and gated by measurement.

**Derived from:** `SD-30-class-feature-archetype-bundle/decisions.md §33, §35, §37`.

**Cross-SD gate:** hard-blocked on `SD-30-class-feature-archetype-bundle`'s Epic 3 (PI-Screening
Provenance Gate), specifically SD30-E3-F2 (declared-PI reader). No cycle here may claim a book before
that book's declared-PI screen is `COMPLETE` in SD-30. SD-31 does not re-run or duplicate the gate.

### Feature seeds

#### SD31-E3-F1 (was SD30-E6-F1) — Per-class ingest, scheduled by Epic 1's clearance order

Acceptance:

- A cycle-batch targets one class (or a small set of related classes in one book) only after Epic 1
  has produced that class's `wired-able / named` figure and Epic 2 has wired its supersession (or
  chooser-interaction) mechanisms.
- Canonical `class_feature` records land in `src/rules_core/rules_tables/<book>/`, one per real
  feature, matching the class's measured wireable set.
- Reach-gate claim executes the real IPC builder for each record (`apps/desktop/src-tauri/src/reach_gate.rs`).

#### SD31-E3-F2 (was SD30-E6-F2) — Occultist/Spiritualist/Medium/Mesmerist canonical definitions (class-grant boundary with SD-28)

Acceptance:

- This package owns the canonical class definition for these four shared classes per
  `SD-30-.../decisions.md §5` — SD-28 references, does not redefine.
- Cross-book conflict rule (`decisions.md §16`) applies if Ultimate Intrigue and Occult Adventures
  wording diverges on the same class.

#### SD31-E3-F3 (was SD30-E6-F3) — `unknown`-bucket disposal, per Epic 1's characterization

Acceptance:

- Units Epic 1-F4 characterizes as option-pool content with a real chooser get their representative
  option grounded (canonical narrowing), the rest deferred with a named diagnostic.
- Units characterized as genuinely unreachable are NOT silently ingested as if grounded; named findings
  routed to `SD-32-engine-capability-builds/`'s verdict-path epic if the operator funds the engine
  change.

## Epic 4 (SD31-E4) — Corpus-Wide Ingest Lanes, folded from SD-29 (moved from SD30-E10)

**Objective:** the real per-book ingest that SD-30's instrument-application epic (Epic 0) cannot
substitute for. SD-29 closed with its corpus-wide kind lanes at a *measured* ceiling, not an exhausted
one; SD-30 inherited them (`SD-30-.../decisions.md §44`), and this package now carries them forward.

**Cross-SD gate:** hard-gated behind `SD-30-class-feature-archetype-bundle`'s Epic 3 (PI-Screening
Provenance Gate), exactly like Epic 3 above. Also gated behind SD-30's Epic 1/Epic 2 (identifier
cleanup, pre-launch trap-report).

**Per-kind cards**, each running the raw-vs-workable split and the pre-cycle row-classifier screen
before planning cycles:

#### SD31-E4-F1 (was SD30-E10-F1) — `monster` ingest lane (1,242 grounded / 7 done, 0.6%)

- Pilot book selection runs `scripts/screen_pcc_load_gates.py` and a monster-count check
  (`cargo run --locked --bin v06_work_inventory` per-book breakdown) before committing a round —
  `bestiary_5`/`bestiary_6` are confirmed zero-monster books, hard stop, not a candidate.
- Splits `not-started`/`not-ingested` `monster` units into workable vs. structurally blocked
  (negated-PCC-gate exclusions) before scheduling any cycle.
- Mirrors SD-29 Epic 3's monster-ingest pipeline (same `rules_tables/*.rs` shape).

#### SD31-E4-F2 (was SD30-E10-F2) — `spell` ingest lane (623 grounded / 47 done, 1.7%)

- `spell`'s `computed` bucket has no consumer-delta probe corpus-wide (owned by SD-30's Epic 0); this
  card's ingest work raises `grounded`/`held`, not `done`, until that probe lands.
- Runs the same pre-cycle screen as F1 before selecting a book.

#### SD31-E4-F3 (was SD30-E10-F3) — `race` ingest lane (7 grounded / 0 done, 0.0%)

- Runs `scripts/classify_race_trait_rows.py` before selecting any book or committing a round.
- **Cross-SD dependency:** the ~2,894-unit chassis-blocked remainder is out of this card's reach until
  `SD-32-engine-capability-builds/`'s race-chassis epic lands; this card runs its own chassis-blind
  ceiling now and re-runs once SD-32 delivers.

#### SD31-E4-F4 (was SD30-E10-F4) — `race_trait` ingest lane (513 grounded / 264 done, 7.7%)

- Raw remainder is not workload: of 3,447 corpus `race_trait` units, only 553 carry a
  `TYPE:<Race> Racial Trait` component naming one of the 18 races the engine models. This card's
  workable pool is 553 minus whatever F3's `race`-chassis work has not yet landed.
- Runs `scripts/classify_race_trait_rows.py` and `scripts/screen_pcc_load_gates.py` before selecting a
  book.
- Same cross-SD dependency on SD-32's race chassis as F3, for the remainder beyond 553.

**Acceptance (per card):** the raw-vs-workable split is recorded with its command before any cycle
claims; the pre-cycle classifier/screen ran against the candidate book before the round was committed;
PI screen clean for the book (cross-SD gate, SD-30 Epic 3) before any record is written; reach-gate-
satisfied per record ingested; units found structurally unreachable are named findings routed to a
successor, not silently ingested as if grounded.

**Not in this epic:** `class_feature` ingest (Epic 3, above). `equipment`/`equipment_modifier`/
`companion`/`feat`/`monster_ability` — not named in the operator's cited frustration list; a future
pass may open cards for them under this same epic without a new operator ruling.

## Epic 5 (SD31-E5) — Book Onboarding, 100% mandate (moved from SD30-E11)

**Objective:** onboard the 7 `future_state` books — `occult_adventures`, `adventurers_guide`,
`mythic_adventures`, `inner_sea_magic`, `inner_sea_temples`, `inner_sea_taverns`,
`inner_sea_faiths` — the population these books add is not yet in the engine at all; closing to 100%
requires bringing it in.

**Derived from:** `SD-30-class-feature-archetype-bundle/decisions.md §45` (the 100%-mandate ruling,
item 3).

**Cross-SD gate:** hard-gated on `SD-30-class-feature-archetype-bundle`'s Epic 3 (PI-screening), same
as Epic 3/Epic 4 above — no book's records land before its PI screen is clean.

**Not in this epic:** any book already in-scope under Epic 4's four kind-lanes or Epic 3's
`class_feature` chassis sweep — this epic covers only the 7 books named above.

## Epic 6 (SD31-E6) — Cloud Fan-Out Protocol (moved from SD30-E14, scoped to grind lanes)

**Objective:** the local-proof-then-cloud-scale protocol that lets build-heavy, self-contained lanes
(Epic 4's per-kind ingest, Epic 5's book onboarding) scale to cloud agents after one local proof cycle
per lane shape.

**Derived from:** `SD-30-class-feature-archetype-bundle/decisions.md §47` (hardware re-derivation and
cloud fan-out ruling).

**Rules carried into every cycle dispatched under this epic** (unchanged from SD-30's original):

1. Every cloud agent works its own branch — never two writers on one branch.
2. The local orchestrator owns all merges to `tranche/10`, verified by content, not commit count.
3. DoD-8 on-screen verification and dashboard-producer work stay local — no cloud agent runs either.

**Not in this epic:** the ingest/onboarding work itself (Epic 4, Epic 5 own that) — this epic is the
dispatch protocol enabling it at cloud scale. `SD-32-engine-capability-builds/` carries its own copy of
this protocol scoped to its own lane shapes (race chassis, verdict paths) — the two are siblings, not a
shared epic, because their file scopes never overlap.

## Recommended sequencing

```
E1 (per-class measurement, ongoing) -> E2 (per-class mechanism, gated per class on E1) -> E3
   (per-class chassis sweep, gated per class on E1+E2)
E4 (ingest lanes) and E5 (book onboarding) run independently of E1/E2/E3 — different kinds/books —
   sharing only the cross-SD PI-gate dependency on SD-30's Epic 3.
E6 (cloud fan-out) is a dispatch protocol available to E4/E5 once each lane shape has one local proof
   cycle.
```

## Completion gate

SD-31 closes when:

- Epic 1 has measured every remaining `class_feature`-bearing class (or named a successor).
- Epic 2 has landed the supersession shape for every class Epic 1 clears, and resolved or explicitly
  deferred the chooser-interaction shape for Oracle/Arcanist/Sorcerer.
- Epic 3's chassis sweep has ingested and reach-gated every class Epic 1/2 cleared.
- Epic 4's four per-kind ingest cards have each reached their measured workable-pool ceiling (noting
  the race/race_trait cross-SD dependency on SD-32) or named a successor.
- Epic 5 has onboarded all 7 `future_state` books, PI-clean.
- Epic 6 has run at least one local-proof-then-cloud-scale cycle per lane shape it claims a role in.
- `progress.md` carries the closure receipt; this package's contribution to the joint
  SD-30→SD-31→SD-32 100% mandate is stated explicitly, not assumed.

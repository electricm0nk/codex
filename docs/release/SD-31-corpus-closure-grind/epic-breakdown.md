---
canonical: true
owner: god-emporer
status: planning-ready (SD-32 absorbed and epics re-sequenced, operator ruling 2026-08-15)
date: 2026-08-15
canonical_branch: tranche/10
build_version_target: 0.10.<build>
companion_to: ./scope-draft.md, ./decisions.md
---

# SD-31 Epic Breakdown

**Origin.** Epics 3-8 were moved from `SD-30-class-feature-archetype-bundle/epic-breakdown.md`
(Epics 4, 5, 6, 10, 11, 14) by operator ruling 2026-08-14, `SD-30 decisions.md §51`. Epics 1 and 2
were moved from `SD-32-engine-capability-builds/epic-breakdown.md` (its Epics 1 and 2, themselves
SD-30's Epics 12 and 13) by operator ruling 2026-08-15, `decisions.md §2`, which absorbed that package
and deleted it. Epics 0 and 9 are new in that same ruling. Feature-seed acceptance content is
reproduced from the origin epics unchanged except where this file states otherwise; each section
records its origin.

**Why this order.** The prior arrangement scheduled the capability builds (race chassis, verdict
paths) *after* the grind lanes that cannot reach `done` without them — 8,524 units, 22.1 % of the
board, with a ceiling of 77.9 % reachable without them. `decisions.md §2` carries the derivation.
**Capability now comes first, and the dependencies that used to be cross-package handoffs are internal
hard gates.**

**What runs concurrently.** Epics 1 and 2 (engine capability: race-chassis data model, wiring-class
classifier) are file-disjoint from Epics 3-5 (the `class_feature` measurement→mechanism→sweep chain,
which touches `pilot_compute.rs`, `archetype_resolver.rs` and `rules_tables/<book>/`). Those two
tracks run in parallel from the start. What may **not** start early is any lane whose units are
capability-blocked: Epic 6-F3/F4 and the `unknown`-bucket work in Epic 3-F4 / Epic 5-F3.

## Epic 0 (SD31-E0) — Reachability Audit (NEW, `decisions.md §4`)

**Objective:** answer, mechanically and for every unit on the board, *given current engine capability,
does a path to `done` exist?* Publish the **reachable ceiling** and give every gap a name.

**Why it exists:** the `ambiguous` dead-end — 2,109 units with no path to `done` at any status — was
present in the engine before either successor package was authored, is detectable by a five-line
query, and was found by neither authoring pass. It surfaced only when the operator asked whether the
sequencing could strand the mandate.

**Standing, not one-shot.** Runs before this package's first cycle, at every epic closure, and before
any closure receipt is written.

### Feature seeds

#### SD31-E0-F1 — `scripts/reachability_audit.py`

Acceptance:

- For each `(wiring_class, status, kind)` cell present on the board, the audit reports whether
  `doneness_verdict()` can return `done`, by **importing the dashboard producer's own function** —
  never by reimplementing its table, which would drift from the thing it audits.
- Output: per-kind and corpus-wide **reachable ceiling** (share of units for which a `done`-producing
  cell is reachable), plus a named list of every dead-end cell and its unit count.
- The audit is proven able to fail before it is trusted: its own tests feed it a fabricated dead-end
  and confirm it is reported. `SD-30 state-goals-and-lessons.md §3.1` — this repo has shipped three
  gates that could not fail, each caught only by running it against a known-answer case.
- Wired into `scripts/verify.sh` as its own stage, or run as a required step in the cycle procedure
  with its output in the receipt — cycle's choice, recorded either way.

#### SD31-E0-F2 — Baseline run and gap ownership

Acceptance:

- A baseline run is committed as an artifact under `artifacts/`, with the commit and the command.
- Every dead-end the baseline reports is either **assigned to an epic in this file** or **proposed to
  the Structural Exclusion Register** (`decisions.md §3`) — never left unowned.
- The known dead-ends at authoring time, to be re-derived not transcribed: `wiring_class ==
  ambiguous` reaches `done` from no status (2,109 units); `unmeasurable`/`status == unknown` totals
  3,989 (`class_feature` 3,622 + `feat` 367); `race` 103 units at 0 % and `race_trait` 3,284 not-done.

## Epic 1 (SD31-E1) — Race Chassis, 100 % mandate (moved from SD32-E1, orig. SD30-E12)

**Objective:** build the missing race chassis that `SD-30 decisions.md §44` (citing SD-29
`§44.4`/`§45.1`/`§49.2`) found absent for ~2,894 of the corpus's 3,447 `race_trait` units, plus the
`race` kind itself (103 units, 0 % done). That absence was previously ruled structurally unreachable;
this epic reverses that ruling by building the capability rather than accepting the ceiling.

**Derived from:** `SD-30 decisions.md §45` item 1; `SD-30 decisions.md §44` (the original
chassis-absence finding, `RaceCorpus::resolve` returning `None` without a chassis).

**Gates:** Epic 6-F3 (`race` lane) and Epic 6-F4 (`race_trait` lane) beyond their chassis-blind
ceiling. This was a cross-package handoff before `decisions.md §2`; it is now an internal hard gate.

**Verification:** DoD-8 on-screen verification is mandatory for this epic — a chassis claim is not
accepted from static/derived instrument output alone, and this is not weakened by the merge.

### Feature seeds

#### SD31-E1-F1 (was SD32-E1-F1) — Chassis design: what makes a race "modeled"

Acceptance:

- A direct enumeration (grepped, not estimated) of the chassis-blind population's source races — which
  named races the corpus's `race_trait` rows reference that the engine's 18 modeled races do not cover.
- A design decision on the chassis shape: does each new race need a full `RaceCorpus` entry (ability
  score modifiers, size, speed, languages, the works) or a narrower "recognized name, traits resolve"
  shim — recorded with its tradeoffs, not assumed.
- The design cites `RaceCorpus::resolve`'s current signature and what changes to accommodate the new
  races without breaking the 18 already-modeled races' resolution.

#### SD31-E1-F2 (was SD32-E1-F2) — Chassis build, per race (or race batch)

Acceptance:

- Each new race's chassis entry lands with the same rigor as the 18 existing ones — no placeholder
  ability scores, no stub trait list.
- `RaceCorpus::resolve` returns a real value (not `None`) for every race this feature seed adds.
- DoD-8 on-screen verification: a character sheet built with the new race shows its real ability score
  modifiers, size and speed — not a default or blank value silently substituted.

#### SD31-E1-F3 (was SD32-E1-F3, rewritten by `decisions.md §2`) — Ceiling release to Epic 6

Acceptance:

- Each landed race batch is recorded in `progress.md` with the races it covers, and Epic 6-F3/F4's
  card gate in `kanban.md` is updated to name the batch — the gate opens per race batch, not
  all-or-nothing, so ingest starts as soon as any chassis is real.
- No `race`/`race_trait` unit is marked `done` by this epic. `done` is Epic 6's ingest claim, made once
  the chassis this epic built lets a real record ground.
- **Changed by the merge:** this was a cross-package notification to a separate SD. It is now an
  in-package gate update, so the "notified via receipt, cited by the other package's next cycle"
  choreography is gone; the gate state lives in this package's own `kanban.md`.

## Epic 2 (SD31-E2) — Verdict-Path Capability, 100 % mandate (moved from SD32-E2, orig. SD30-E13)

**Objective:** give every currently-unmeasurable unit a real, non-placeholder verdict.

**Scope, corrected 2026-08-15 (`decisions.md §2`).** The origin charter described the target as "the
~3,547 unmeasurable units incl. the 2,109-unit `ambiguous` bucket." Those are **two nearly disjoint
populations, not a nested one**: 3,989 `unmeasurable` (all `status == unknown`) and 2,109 `ambiguous`
(1,590 `not-started`, 400 `held`, 119 `unmeasurable`), overlapping by 119. The real target is their
union, ~5,979 units. Re-derive before planning; do not transcribe these.

**Derived from:** `SD-30 decisions.md §45` item 2.

**Gates:** Epic 3-F4 (`unknown`-bucket characterization) and Epic 5-F3 (`unknown`-bucket disposal).
Cross-package handoff before `decisions.md §2`; internal hard gate now.

**Constraint:** classifier/instrument work here is bound by `decisions.md` Decision 1(e) — the
wiring-class classifier is accepted on **accuracy, not on movement**. A verdict path is validated
against known-correct cases before it is trusted to move counts.

### Feature seeds

#### SD31-E2-F1 (was SD32-E2-F1) — Hand-labelled ground-truth sample (the gate that runs first)

Acceptance:

- At least 100 units, stratified across the five wiring classes and at least four kinds, hand-labelled
  from the corpus record — the whole record, not a field-filtered grep.
- Labels committed, with the token evidence for each label recorded by the labeller.
- No classifier code is written before this sample is committed.

#### SD31-E2-F2 (was SD32-E2-F2) — Classifier build and acceptance

Acceptance:

- The classifier's acceptance criterion is its agreement rate against the F1 sample, reported per class
  and per kind, plus its full confusion matrix.
- Movement is reported in both directions — units moved toward AND away from `done`-producing cells.
- If F1's sample shows the current classifier substantially correct and any contradiction rare, F2 is
  **not dispatched**: this feature seed closes with the affected units reported "examined, correctly
  classified, left alone," per Decision 1(e) item 4.

#### SD31-E2-F3 (was SD32-E2-F3, rewritten by `decisions.md §2`) — `ambiguous` dead-end closure

Acceptance:

- **The `ambiguous` wiring class must end this epic with a path to `done`, or an entry in the
  Structural Exclusion Register bearing operator sign-off.** Re-run Epic 0's audit to prove which.
  This is the single largest structural gap on the board and the merge exists because of it: closing
  this epic while `ambiguous` still reaches `done` from no status leaves 2,109 units permanently
  outside the 100 % bar.
- Units the classifier resolves to a real wiring class get their new verdict recorded and are picked
  up by Epic 3-F4 / Epic 5-F3's next disposition cycle — an in-package gate now, not a handoff.
- Units confirmed genuinely unreachable are **proposed** to the Structural Exclusion Register with the
  four items `decisions.md §3` requires. A cycle may propose; only the operator grants.
- **Widened 2026-08-15 (`acceptance-and-verification.md` AT-31-010, launch-readiness remediation Step
  2, blocker B4):** the same ground-truth-sample classifier is also applied to the 1,243-unit
  `wiring_class == display, status == grounded` population (re-derive) — the other half of Decision
  1(e)'s own named scope, previously bound nowhere in this epic's own acceptance text. Passing
  outcome per Decision 1(e) item 4 may be "examined, correctly classified, left alone"; what fails
  this bullet is closing Epic 2 having never run the sample against this population at all.

## Epic 3 (SD31-E3) — Per-Class Archetype Measurement (was SD31-E1; orig. SD30-E4; GATES Epics 4 and 5)

**Objective:** Extend SD-28 `§63`/`§64`'s hand-verification method — find each named archetype slot's
base computation in `pilot_compute.rs`, confirm it is unconditional (level-gated only), confirm the
slot name maps to a real id, report per class, never blended, no automated proxy — to every
`class_feature`-bearing class not yet measured. Also owns characterizing the `unknown` bucket
(`SD-30 decisions.md §38`) and designing the chooser-interaction primitive for the 3 classes excluded
so far (Oracle, Arcanist, Sorcerer).

**Derived from:** `SD-30 decisions.md §34, §37, §38`; `SD-28-ultimate-book-content-ingestion/decisions.md
§60, §63, §64`; `corpus-work-channels.md §9.1`.

**Status:** Gates Epic 4 (mechanism) and Epic 5 (chassis sweep) **per class** — a class's Epic 5 cycle
cannot be scheduled until this epic has produced that class's `wired-able / named` figure by direct
evidence. Runs concurrently with Epics 1 and 2 (different files), except F4 below.

### Inherited starting state — verify before extending, do not re-measure

- 25 of 28 archetype-bearing classes already hand-verified at split time (Fighter, Alchemist, Ranger,
  Cleric, Druid, Rogue, Monk, Paladin, Bard, Cavalier, Witch, Shaman, Warpriest, Summoner, Skald,
  Barbarian, Inquisitor, Hunter, Bloodrager, Wizard, Investigator, Brawler, Swashbuckler, Familiar,
  Companion) — 263 wired-able / 475 named slots, 175 mechanisms after collapsing duplicate slot-tier
  names.
- 3 classes excluded, own wiring shape: Oracle, Arcanist, Sorcerer — choice-based, need the
  chooser-interaction primitive, not `archetype_claims_slot`.
- 11 unmodelled base-class features found incidentally (`SD-30 decisions.md §34` lists all eleven) —
  recorded so this epic does not rediscover them as new findings.

### Feature seeds

#### SD31-E3-F1 (was SD31-E1-F1 / SD30-E4-F1) — Class inventory: which classes remain unmeasured

Acceptance:

- A direct enumeration (grepped, not estimated) of every class with `class_feature` records across the
  23 in-scope books, cross-referenced against SD-28 `§64`'s 28-class list, producing a named remainder
  (expected: Occultist, Spiritualist, Medium, Mesmerist from Occult Adventures at minimum; Mythic
  Adventures path-tier features if any resolve to `class_feature`; any Inner Sea archetype content not
  already covered as a tier-1/tier-2 archetype swap).
- Report is per-class, cites the corpus files grepped, and does not assume book-of-origin implies
  class-of-origin (the class-grant boundary with SD-28, `SD-30 decisions.md §5`, is a name/identity
  join, not a book join).

#### SD31-E3-F2 (was SD31-E1-F2 / SD30-E4-F2) — Per-class hand-verification, extended

Acceptance:

- Each newly-measured class reports `wired-able / named` by direct evidence in `pilot_compute.rs` (or
  the class's own compute module), the same method as `§63`/`§64` — no id-string proxy, no
  extrapolation from another class's ratio.
- No blended percentage reported across classes, ever.
- A near-miss slot name is not counted as evidence without direct confirmation.

#### SD31-E3-F3 (was SD31-E1-F3 / SD30-E4-F3) — Chooser-interaction primitive design (Oracle, Arcanist, Sorcerer)

Acceptance:

- A design decision (not yet a measurement) on what a "which options remain choosable, and does the
  substitute grant compute" primitive looks like, distinct from `archetype_claims_slot`'s supersession
  shape.
- Once designed, Oracle/Arcanist/Sorcerer are measured by the same no-proxy, per-class standard as the
  supersession-shape classes, producing their own `wired-able / named` figures.

#### SD31-E3-F4 (was SD31-E1-F4 / SD30-E4-F4) — `unknown`-bucket characterization, per class

**HARD-GATED ON EPIC 2.** Changed by `decisions.md §2`: this seed previously routed
genuinely-unreachable content to a separate package's verdict-path epic. That epic is now Epic 2 of
this package and runs **before** this seed, so the routing is a gate, not a handoff. A cycle claiming
this seed before Epic 2 is `COMPLETE` is out of protocol.

Acceptance:

- For each class this epic measures, its share of the `unknown` bucket is characterized using SD-28
  `§52`/`§53`'s proven distinction: option-pool sub-choice content (real chooser, canonical-narrowing
  is the deliberate design, not a gap) vs. genuinely-unreachable content (no chooser code at all —
  now resolved by Epic 2's verdict path, or proposed to the Structural Exclusion Register) vs.
  residual unclustered content.
- No `unknown` unit's status is changed by this feature seed alone — characterization, not
  reclassification, per SD-28 `§52`'s standing constraint.

## Epic 4 (SD31-E4) — Archetype Mechanism (was SD31-E2; orig. SD30-E5)

**Objective:** Wire the 175-mechanism / ~5,775-line supersession shape (`archetype_claims_slot`) for
each measured class as Epic 3 clears them; design and wire the chooser-interaction shape for
Oracle/Arcanist/Sorcerer once Epic 3-F3 resolves its primitive.

**Derived from:** `SD-30 decisions.md §34`; `SD-28-ultimate-book-content-ingestion/decisions.md §59,
§60, §63, §64`.

**Status:** Gated on Epic 3 clearing the specific class(es) a cycle targets. Gates Epic 5 for the same
class.

### Feature seeds

#### SD31-E4-F1 (was SD31-E2-F1 / SD30-E5-F1) — Supersession-shape wiring, per measured class

Acceptance:

- For each measured class, the `if let`/`else` supersession branch replacing the prior unconditional
  feature-grant block lands in `pilot_compute.rs`, one mechanism at a time.
- Reachability proven per SD-28 `§43`'s standard: a headless pilot receipt test through
  `build_pilot_headless_receipt`, not a unit test on the resolver alone.
- No `§59` vacuity-audit citation touched without being addressed.

#### SD31-E4-F2 (was SD31-E2-F2 / SD30-E5-F2) — Chooser-interaction primitive, once designed

Acceptance:

- `archetype_resolver.rs` (or a sibling module) gains the chooser-interaction primitive Epic 3-F3
  designs.
- Oracle/Arcanist/Sorcerer's measured wireable mechanisms wire through it, proven reachable the same
  way as the supersession shape.

## Epic 5 (SD31-E5) — Per-Class Chassis Sweep (was SD31-E3; orig. SD30-E6)

**Objective:** The actual per-book, per-class `class_feature` ingest cycles across all 23 in-scope
books, scoped by class and gated by measurement.

**Derived from:** `SD-30 decisions.md §33, §35, §37`.

**Cross-SD gate — SATISFIED.** Hard-blocked on `SD-30-class-feature-archetype-bundle`'s Epic 3
(PI-Screening Provenance Gate), specifically SD30-E3-F2 (declared-PI reader). **SD-30 closed
2026-08-14 with `epic-3-pi-gate` COMPLETE (all of F1-F4) and PR #363 open**, so this gate is
discharged — but a cycle still cites the SD-30 receipt for the book it touches, and calls the
documented invocation contract (`SD-30 decisions.md §52.3` blacklist sweep, `§53.5` declared-PI
reader) before writing any generated record. SD-31 does not re-run or duplicate the gate.

### Feature seeds

#### SD31-E5-F1 (was SD31-E3-F1 / SD30-E6-F1) — Per-class ingest, scheduled by Epic 3's clearance order

Acceptance:

- A cycle-batch targets one class (or a small set of related classes in one book) only after Epic 3 has
  produced that class's `wired-able / named` figure and Epic 4 has wired its supersession (or
  chooser-interaction) mechanisms.
- Canonical `class_feature` records land in `src/rules_core/rules_tables/<book>/`, one per real
  feature, matching the class's measured wireable set.
- Reach-gate claim executes the real IPC builder for each record
  (`apps/desktop/src-tauri/src/reach_gate.rs`).

#### SD31-E5-F2 (was SD31-E3-F2 / SD30-E6-F2) — Occultist/Spiritualist/Medium/Mesmerist canonical definitions

Acceptance:

- This package owns the canonical class definition for these four shared classes per `SD-30
  decisions.md §5` — SD-28 references, does not redefine.
- Cross-book conflict rule (`SD-30 decisions.md §16`) applies if Ultimate Intrigue and Occult
  Adventures wording diverges on the same class.

#### SD31-E5-F3 (was SD31-E3-F3 / SD30-E6-F3) — `unknown`-bucket disposal, per Epic 3's characterization

**HARD-GATED ON EPIC 2 AND EPIC 3-F4.** Same change as Epic 3-F4: the successor package this seed used
to route to is now Epic 2 of this package and runs first.

Acceptance:

- Units Epic 3-F4 characterizes as option-pool content with a real chooser get their representative
  option grounded (canonical narrowing), the rest deferred with a named diagnostic.
- Units characterized as genuinely unreachable are NOT silently ingested as if grounded. After Epic 2,
  "genuinely unreachable" requires an Epic 0 audit run confirming no path exists, and it is a
  **proposal** to the Structural Exclusion Register — not a disposal a cycle performs on its own
  authority.

#### SD31-E5-F4 — The 36 `deferred-with-reason` units (added 2026-08-15, launch-readiness remediation Step 2, blocker B2)

**Re-derived this cycle** (not transcribed):

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d = json.load(open('docs/work-inventory.json'))
U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
deferred = [u for u in U if u.get('status')=='deferred-with-reason']
print(len(deferred))
print(collections.Counter((u.get('kind'), u.get('book')) for u in deferred))
"
```
→ **36 units**, matching the plan's expected figure exactly: **34 `class_feature`** (17
`advanced_class_guide`, 7 `core_rulebook`, 4 `advanced_players_guide`, 2 `advanced_race_guide`, 2
`ultimate_magic`, 1 `pathfinder_unchained`, 1 `ultimate_psionics`) and **2 `feat`** (both
`ultimate_campaign`). Sits in the mandate denominator (`decisions.md §5`) with zero sign-off and,
before this seed, no card.

**Full list, by `id` — each already carries a per-unit `reason` in `docs/work-inventory.json`, not
reproduced verbatim here for space, but grep-able by `id`:**

| id | kind | book | wiring_class | reason theme (see full text in `docs/work-inventory.json`) |
|---|---|---|---|---|
| `advanced_class_guide:class_feature:arcanist` | class_feature | advanced_class_guide | computed | Arcane Reservoir / prepared spellbook beyond chassis pillar |
| `advanced_class_guide:class_feature:bloodrager` | class_feature | advanced_class_guide | computed | Bloodrage / spells-per-day beyond chassis pillar |
| `advanced_class_guide:class_feature:bloodrager_bloodrage` | class_feature | advanced_class_guide | computed | same as above (Bloodrage sub-feature) |
| `advanced_class_guide:class_feature:brawler` | class_feature | advanced_class_guide | computed | AC bonus lost while helpless/immobilized — no transient combat-state representation |
| `advanced_class_guide:class_feature:hunter` | class_feature | advanced_class_guide | computed | full class-table grounding note, no remaining gap named beyond diagnostic |
| `advanced_class_guide:class_feature:hunter_animal_companion` | class_feature | advanced_class_guide | static | animal-companion advancement, columns with no engine consumer |
| `advanced_class_guide:class_feature:investigator` | class_feature | advanced_class_guide | computed | Trapfinding/Trap Sense/Inspiration beyond chassis pillar |
| `advanced_class_guide:class_feature:shaman` | class_feature | advanced_class_guide | computed | Orisons/prepared spellcasting beyond chassis pillar |
| `advanced_class_guide:class_feature:shaman_spirit_life` | class_feature | advanced_class_guide | computed | Spirit power content beyond Life's Channel |
| `advanced_class_guide:class_feature:shaman_wandering_spirit_life` | class_feature | advanced_class_guide | computed | same, Wandering Spirit variant |
| `advanced_class_guide:class_feature:shaman_spirit` | class_feature | advanced_class_guide | computed | same, base Spirit |
| `advanced_class_guide:class_feature:skald` | class_feature | advanced_class_guide | computed | full class-table grounding note |
| `advanced_class_guide:class_feature:slayer` | class_feature | advanced_class_guide | computed | full class-table grounding note |
| `advanced_class_guide:class_feature:slayer_talent_feat` | class_feature | advanced_class_guide | computed | same, talent-feat sub-feature |
| `advanced_class_guide:class_feature:spirit_summoner_eidolon` | class_feature | advanced_class_guide | display | Quadruped Eidolon stat block, one built evolution set |
| `advanced_class_guide:class_feature:swashbuckler` | class_feature | advanced_class_guide | computed | full class-table grounding note |
| `advanced_class_guide:class_feature:warpriest` | class_feature | advanced_class_guide | computed | Blessing power content beyond Destruction/Strength |
| `advanced_players_guide:class_feature:cavalier_mount` | class_feature | advanced_players_guide | computed | mount advancement, columns with no engine consumer |
| `advanced_players_guide:class_feature:summoner` | class_feature | advanced_players_guide | computed | Quadruped Eidolon stat block, one built evolution set |
| `advanced_players_guide:class_feature:summoner_eidolon` | class_feature | advanced_players_guide | computed | same, eidolon sub-feature |
| `advanced_players_guide:class_feature:witch_hex` | class_feature | advanced_players_guide | computed | only selected hex + shared save DC (51/53 records) modeled |
| `advanced_race_guide:class_feature:reincarnated_oracle_revelations` | class_feature | advanced_race_guide | computed | Mystery revelations beyond grounded Tier-1 set |
| `advanced_race_guide:class_feature:sky_druid_animal_companion` | class_feature | advanced_race_guide | display | animal-companion advancement, columns with no engine consumer |
| `core_rulebook:class_feature:bard` | class_feature | core_rulebook | computed | only Inspire Courage modeled among bardic performances |
| `core_rulebook:class_feature:bard_bardic_performance` | class_feature | core_rulebook | computed | same, performance sub-feature |
| `core_rulebook:class_feature:cleric` | class_feature | core_rulebook | computed | domain spell-list contents (which spell fills the slot) unmodeled |
| `core_rulebook:class_feature:druid` | class_feature | core_rulebook | computed | animal-companion advancement, columns with no engine consumer |
| `core_rulebook:class_feature:druid_domain_animal` | class_feature | core_rulebook | computed | same, domain-animal sub-feature |
| `core_rulebook:class_feature:sorcerer` | class_feature | core_rulebook | computed | Arcane bloodline progression at level 10, three items ungrounded |
| `core_rulebook:class_feature:sorcerer_bloodline_arcane` | class_feature | core_rulebook | computed | same, bloodline sub-feature |
| `pathfinder_unchained:class_feature:unchained_summoner_eidolon` | class_feature | pathfinder_unchained | computed | Quadruped Eidolon stat block, one built evolution set |
| `ultimate_campaign:feat:fearless_zeal` | feat | ultimate_campaign | display | `.MOD BENEFIT:` row splices verbatim mid-sentence into Damned's own text |
| `ultimate_campaign:feat:magnum_opus` | feat | ultimate_campaign | display | `.MOD BENEFIT:` row's sentence grammatically truncated |
| `ultimate_magic:class_feature:planar_oracle_revelations` | class_feature | ultimate_magic | computed | Mystery revelations beyond grounded Tier-1 set |
| `ultimate_magic:class_feature:storm_druid_animal_companion` | class_feature | ultimate_magic | static | animal-companion advancement, columns with no engine consumer |
| `ultimate_psionics:class_feature:phrenic_slayer_as` | class_feature | ultimate_psionics | computed | full class-table grounding note |

**Disposition — none silently carried, per this seed's own charter:**

- **The 6 animal/eidolon/mount "columns with no engine consumer" units**
  (`hunter_animal_companion`, `sky_druid_animal_companion`, `druid`, `druid_domain_animal`,
  `cavalier_mount`, `storm_druid_animal_companion`) name a real,
  structural gap: the engine has no consumer for certain advancement columns at all. **Real path:**
  a future cycle under this seed builds the missing consumer (in scope, ordinary engine work, not
  impossible) — or, if genuinely out of charter, proposes each to the Structural Exclusion Register
  with `decisions.md §3`'s four items. Not yet proposed as of this receipt — flagged for the next
  cycle claiming this seed, not silently carried.
- **The "beyond grounded Tier-1/chassis-pillar" units** (Oracle Mystery revelations ×2, Shaman Spirit
  power ×3, Bardic Performance, Warpriest Blessing, Sorcerer bloodline, Witch hex, Eidolon evolutions
  ×4, Cleric domain spell) name option-pool content: a representative option is already grounded, the
  remainder deliberately deferred — this is `decisions.md #38`'s standing "ground one representative,
  defer the rest with a named diagnostic" ruling, **already satisfied** by the existing `reason` text.
  **Real path:** these are correctly `deferred-with-reason` today and need no register entry; they
  remain in the denominator (per Decision 5) and are candidates for Epic 3/Epic 5's per-class
  measurement chain to widen the grounded set, not for exclusion.
- **The 1 combat-state unit** (`brawler`, AC bonus lost while helpless/immobilized) names a genuine
  engine-capability gap (no transient combat-state representation). **Real path:** proposed here as a
  **PROPOSED** Structural Exclusion Register entry (not yet operator-signed — a cycle may only
  propose):

  | Missing capability | Why impossible/out-of-charter (not merely expensive) | Proving command | Epic 0 run | Sign-off |
  |---|---|---|---|---|
  | Transient combat-state representation (helpless/immobilized flags feeding AC-bonus suppression) | Out-of-charter for this package: no other `class_feature` or `feat` record in the corpus needs transient combat state, so building a whole new state axis for one Brawler feature is a scope expansion this package's charter (`class_feature`, corpus-wide ingest, book onboarding) does not cover — **not proposed as a cost objection**, which `decisions.md §3` forbids as a reason. | `grep -n "helpless\|immobilized" src/rules_core/*.rs src/rules_core/**/*.rs` (no transient-state field found at authoring) | Not yet run — Epic 0 does not exist yet at this cycle; **this entry is provisional pending Epic 0's own audit run**, per `AT-31-100` item 3 | **NONE — proposed only, per this seed's launch-readiness remediation instruction; the operator has not signed this entry** |

  This entry is **PROPOSED, not granted** — the unit stays in the denominator until the operator
  signs `AT-31-100`'s register (`acceptance-and-verification.md`) or a real engine consumer lands.
- **The 2 PCGen `.lst` transcription-quality `feat` units** (`fearless_zeal`, `magnum_opus`) name a
  source-data legibility issue (`.MOD BENEFIT:` splicing/truncation), not a capability gap. **Real
  path:** a future ingest cycle re-reads the raw `.lst` line and hand-corrects the transcription
  (ordinary ingest-quality work, not exclusion-eligible under `decisions.md §3` item 2's "genuinely
  impossible" bar).

**Acceptance:** every one of the 36 units above has either (a) a real forward path stated (consumer
build, Epic 3/5 measurement widening, transcription fix) or (b) a Structural Exclusion Register entry
— proposed here for the 1 `brawler` unit, pending operator sign-off. None of the 36 is left with
neither.

## Epic 6 (SD31-E6) — Corpus-Wide Ingest Lanes, folded from SD-29 (was SD31-E4; orig. SD30-E10)

**Objective:** the real per-book ingest that instrument application cannot substitute for. SD-29 closed
with its corpus-wide kind lanes at a *measured* ceiling, not an exhausted one; SD-30 inherited them
(`SD-30 decisions.md §44`), and this package carries them forward.

**Gates:** SD-30's Epic 3 PI gate (satisfied, cite per book, as Epic 5 above) and SD-30's Epic 1/Epic 2
(both COMPLETE). **F3 and F4 are additionally hard-gated on Epic 1 of this package**, per race batch.

**Per-kind cards**, each running the raw-vs-workable split and the pre-cycle row-classifier screen
before planning cycles. Every figure below is a split-time snapshot — re-derive at time of use.

#### SD31-E6-F1 (was SD31-E4-F1 / SD30-E10-F1) — `monster`: **rewritten 2026-08-15 as a fixture-coverage lane, not an ingest lane** (launch-readiness remediation Step 2, blocker B4)

**Correction (re-derived, launch-readiness remediation Step 2).** This seed's header previously read
"1,242 grounded / 7 done, 0.6 %" and was framed as ingest work (raw-vs-workable split, pre-cycle
screen, mirroring SD-29's ingest pipeline). Re-derived this cycle by importing the dashboard
producer's own `doneness_verdict()` over `docs/work-inventory.json`:

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d = json.load(open('docs/work-inventory.json'))
U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS and u.get('kind')=='monster']
c = collections.Counter(P.doneness_verdict(u.get('wiring_class'),u.get('status'),'monster') for u in U)
print(dict(c), len(U))
"
```
→ `{'held': 1235, 'done': 7, 'not-started': 28}`, total 1,270. Of the 1,235 `held`, **1,229 are the
single cell `derived|grounded`** — the whole monster lane's not-done mass is *one* cell, not the
raw-ingest residue the old framing implied. `retro.py correction` emitted (claimed "1,242 grounded /
7 done" framed as ingest ceiling → actual 1,229 `derived|grounded` held units, a fixture-coverage
gap, not an ingest gap).

**Why it is a fixture lane, not an ingest lane.** `monster` is `wiring_class == derived` almost
entirely; a `derived` unit reaches `done` only via the `fixture-verified` rung
(`v06_work_inventory.rs` ~4574-4592), stamped when a unit's `id` is in
`derived_evaluator_fixture_check`'s verified set. That set is `tests/fixtures/rules_core/derived-
evaluator-fixtures.json`, **94 entries corpus-wide** — see Epic 6-F11 below, which owns growing it.
Ingesting more monster records does not move this lane's `done` count; only fixture coverage does.

- **Instrument that moves this cell:** `tests/fixtures/rules_core/derived-evaluator-fixtures.json` +
  `derived_evaluator_fixture_check` (owner: Epic 6-F11).
- **What remains a real ingest concern for `monster`:** the 28 `not-started` units (24
  `derived|not-ingested`, 3 `static|not-ingested`, 1 `ambiguous|not-started`) and the pre-cycle screen
  discipline SD-29's pipeline established — `bestiary_5`/`bestiary_6` are confirmed zero-monster
  books, hard stop, not a candidate; `scripts/screen_pcc_load_gates.py` and a monster-count check
  (`cargo run --locked --bin v06_work_inventory` per-book breakdown) run before any of those 28 are
  claimed.
- **Acceptance:** a cycle claiming this seed either (a) grows `derived-evaluator-fixtures.json`
  coverage for `monster` `id`s and re-runs `derived_evaluator_fixture_check` to confirm the stamp
  loop moves them to `fixture-verified`/`done`, or (b) ingests one of the 28 genuinely `not-started`
  units per the pre-cycle screen above. Citing a rising `grounded`/`held` count as progress on this
  seed, without a fixture-coverage or true-ingest delta, is not acceptance evidence.

#### SD31-E6-F2 (was SD31-E4-F2 / SD30-E10-F2) — `spell` ingest lane (623 grounded / 47 done, 1.7 %)

- `spell`'s `computed` bucket consumer-delta probe was owned by SD-30's Epic 0, which closed
  2026-08-14 — **verify by content what that epic actually landed for `spell`** before assuming this
  lane's ingest work can raise `done` rather than only `grounded`/`held`.
- Runs the same pre-cycle screen as F1 before selecting a book.

#### SD31-E6-F3 (was SD31-E4-F3 / SD30-E10-F3) — `race` ingest lane (7 grounded / 0 done, 0.0 %)

- **HARD-GATED ON EPIC 1**, per race batch: no cycle claims a book before Epic 1 has landed a chassis
  covering the races that book's rows reference. This was a cross-SD dependency on a package scheduled
  *after* this lane; `decisions.md §2` inverted the order and made it an internal gate.
- Runs `scripts/classify_race_trait_rows.py` before selecting any book or committing a round.

#### SD31-E6-F4 (was SD31-E4-F4 / SD30-E10-F4) — `race_trait` ingest lane (513 grounded / 264 done, 7.7 %)

- **HARD-GATED ON EPIC 1**, per race batch, same as F3.
- Raw remainder is not workload: of 3,447 corpus `race_trait` units, only 553 carried a
  `TYPE:<Race> Racial Trait` component naming one of the 18 races the engine modeled at split time.
  That 553 is a **function of Epic 1's output**, not a constant — re-derive it after each chassis
  batch rather than treating it as this lane's ceiling.
- Runs `scripts/classify_race_trait_rows.py` and `scripts/screen_pcc_load_gates.py` before selecting a
  book.

### F5-F10 — cards for the six previously-unowned kinds (added 2026-08-15, launch-readiness remediation Step 2, blocker B2)

**Why these six.** `forward-scope-register.md` `G1.3` and `risks-and-open-questions.md` open question
1 both named `equipment`/`equipment_modifier`/`companion`/`feat`/`monster_ability` as real,
un-carded `not-started` residue in the 100 % denominator; the readiness review (`~/.claude/plans/
conduct-a-launch-readines-zesty-ripple.md`, blocker B2) found a sixth, **`class`** (158 not-done
units), named nowhere in the package. All six are re-derived fresh below — not transcribed from the
plan — by the same replay every other figure in this file uses:

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d = json.load(open('docs/work-inventory.json'))
U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
for k in ('equipment','monster_ability','feat','companion','equipment_modifier','class'):
    units = [u for u in U if u.get('kind')==k]
    verdict = lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),k)
    c = collections.Counter(verdict(u) for u in units)
    print(k, 'total', len(units), 'done', c.get('done',0), 'not_done', len(units)-c.get('done',0), dict(c))
"
```
→ `equipment 6,208 total, 2,626 done, 3,582 not_done` / `monster_ability 3,107, 334, 2,773` /
`feat 2,610, 1,178, 1,432` / `companion 1,696, 416, 1,280` / `equipment_modifier 1,580, 911, 669` /
`class 185, 27, 158`. **Sum of not-done: 9,894** — matches the readiness review's figure exactly.

#### SD31-E6-F5 — `equipment` ingest/instrument lane (3,582 not-done)

**Shape** (not-done cells, largest first): `static|ingested-magnitude` 2,175 held, `static|not-
ingested` 449 not-started, `static|not-started` 340 not-started, `computed|ingested-magnitude` 291
in-progress, `derived|ingested-magnitude` 73 held, `display|not-ingested` 72 not-started,
`display|not-started` 43 not-started, `static|text-complete` 36 held, `computed|not-ingested` 33
not-started, `ambiguous|text-complete` 23 held, `ambiguous|ingested-magnitude` 20 held, plus a small
tail. **2,327 of the 3,582 (65 %) are `held`**, almost entirely `static`/`derived` — the same
literal-sweep/fixture-check coverage gap Epic 6-F11 owns corpus-wide; the remaining 293
`computed|ingested-magnitude` are `in-progress` toward `grounded` under an existing probe, and ~962
are genuine `not-started` ingest residue.

- **Instrument/lane that moves each cell:** `held` `static`/`derived` cells move via `corpus_literal_
  sweep` coverage and `derived_evaluator_fixture_check` fixture coverage (Epic 6-F11 extends both for
  `equipment`); `computed|ingested-magnitude` in-progress units move via whatever consumer-delta probe
  exists for `equipment` magnitudes (verify by content before assuming coverage, per this epic's
  standing discipline); the `not-started` residue is real per-book ingest, same shape as F1/F2.
- **Acceptance:** raw-vs-workable split recorded with command before any ingest cycle claims a book
  (same as F1-F4); a `held` cell moved is cited against `corpus_literal_sweep`/fixture-check coverage
  deltas, never against a raw ingest count; PI screen cited clean per book; reach-gate satisfied per
  record.

#### SD31-E6-F6 — `equipment_modifier` ingest/instrument lane (669 not-done)

**Shape:** `computed|ingested-magnitude` 422 in-progress (the dominant cell — 63 % of the not-done
population, already past ingest and sitting on a `computed` probe), `display|not-started` 119
not-started, `computed|not-started` 79 not-started, `display|not-ingested` 15 not-started,
`static|ingested-magnitude` 14 held, `static|not-started` 10 not-started, `derived|ingested-
magnitude` 5 held, `computed|not-ingested` 5 not-started. Only 19 units are `held`; this kind's gap
is mostly `not-started` ingest plus a large `in-progress` `computed` population one probe-coverage
step from `grounded`.

- **Instrument/lane:** the `computed` consumer-delta probe already exercising 422 units (verify by
  content which probe and its coverage ceiling before claiming it can be widened) moves the
  in-progress mass; the `not-started` residue (213 units) is real per-book ingest.
- **Acceptance:** same per-card acceptance as F1-F4/F5; a cycle claiming the `computed` in-progress
  mass cites the probe's own coverage figures before and after, never a raw status-count delta.

#### SD31-E6-F7 — `companion` ingest/instrument lane (1,280 not-done)

**Shape:** `computed|not-ingested` 377 not-started, `derived|grounded` 303 held, `display|not-
ingested` 298 not-started, `display|grounded` 182 held (part of the corpus-wide 1,243 `display|
grounded` population Epic 2/AT-31-010 now binds — see below), `derived|not-ingested` 42 not-started,
`static|not-ingested` 34 not-started, `ambiguous|not-ingested` 23 not-started, `static|grounded` 19
held, `ambiguous|grounded` 2 held. Split roughly evenly between real `not-started` ingest (774 units)
and `held` fixture/verdict-path gaps (506 units, of which 182 are the `display|grounded` population
Epic 2 owns and 303+19 are `derived`/`static` fixture-coverage gaps Epic 6-F11 owns).

- **Instrument/lane:** `display|grounded` 182 moves via Epic 2's verdict-path work (AT-31-010,
  widened below); `derived|grounded` 303 and `static|grounded` 19 move via Epic 6-F11's fixture/sweep
  coverage growth; the 774 `not-started` units are real per-book ingest (`companion` PI-gate contract
  `forward-scope-register.md G1.6`, dormant today — 17 registered companion books carry zero
  declared-PI source tokens).
- **Acceptance:** same per-card discipline as F1-F4; a cycle cites which of the three levers (verdict
  path / fixture coverage / ingest) it is pulling and does not credit one lever's movement to another.

#### SD31-E6-F8 — `feat` ingest/instrument lane (1,432 not-done) — **routes the SD-30 E0-F3 217-unit probe-fixture residue**

**Shape:** `display|not-started` 556 not-started, `computed|unknown` 306 unmeasurable,
`display|not-ingested` 189 not-started, `computed|not-started` 120 not-started, `ambiguous|text-
complete` 69 held, `derived|unknown` 43 unmeasurable, `derived|not-started` 33 not-started,
`derived|not-ingested` 25 not-started, `computed|not-ingested` 24 not-started, `static|unknown` 18
unmeasurable, plus a small tail (`367` total `unknown`, matching SD30-E0-F3's own re-derivation
exactly).

**The 217-unit routing.** `docs/release/SD-30-class-feature-archetype-bundle/artifacts/sd30-e0-f3-
unknown-residue/` (README.md + `feat_unknown_characterization.json`) already diagnosed `feat`'s
367-unit `unknown` residue per-unit against each record's own PCGen `.lst` source line, not the
stored `reason` text, and found:

| bucket | units | remedy per the artifact's own §4 |
|---|---:|---|
| chooser-pre-selection-gap (positive `PREABILITY` naming a prior chooser selection the probe's synthetic character never makes) | 194 | widen `probe_feat_effect_wiring`'s fixture to pre-select a representative chooser option (a Rage Power, a Discovery, …) per swept class before checking for a computed delta |
| prereq-stat-or-skill-gap (`PRESTAT`/`PRESKILL` floor the probe's fixed per-class stat block does not satisfy) | 23 | either a richer per-feat stat floor in the fixture, or an honest per-feat "structurally needs a purpose-built character" acknowledgment |
| **genuinely-unreachable total (probe-fixture gap, both above)** | **217** | — |
| option-pool (mechanism real, specific pool-slot ungrounded — `BONUS:ABILITYPOOL`/`Extra <X>`, 68; inline `CHOOSE:`, 16; named sub-choice `KEY:... ~ ...`, 16) | 100 | ground one representative option per chooser family, defer the rest with a named diagnostic — mirrors `decisions.md #38`'s standing `class_feature` ruling; no new ingest |
| unclustered remainder (no structural signal on the record's own `.lst` line) | 50 | read each unit's full `.lst` record individually — no pattern found by the artifact's structural-signal pass |

**Concretely, in this repo:** `PROBE_CLASSES` (`src/bin/v06_work_inventory.rs:128`, 5 classes) and
`PROBE_SELECTIONS` (`:138`, 4 generic choices) drive `probe_feat_effect_wiring` (`:1574`) via
`feat_probe_input` (`:1560`), which **strips** the fixture's pre-selected feats/choices rather than
seeding a chooser pre-selection or a prerequisite-satisfying stat floor — exactly the gap the 194+23
units name. Widening the fixture (not the corpus, not the classifier) is the remedy; this is
probe-capability work, the same shape `decisions.md §2` moved into this package as Epic 2.

- **Owner/instrument:** the probe-fixture widening above (Epic 2's verdict-path capability track, or
  a dedicated Epic 6-F8 cycle using the same fixture — either is in-package now that `decisions.md
  §2` absorbed the capability track; no cycle may claim this seed by re-running the classifier or
  loosening the probe's pass bar, per Decision 1(a)).
- **Acceptance:** a cycle claiming the 217-unit bucket cites the artifact's per-unit `id` list and,
  after widening the fixture, re-runs `probe_feat_effect_wiring` (via `v06_work_inventory`) and shows
  the specific `id`s' status leaving `unknown`; the 100-unit option-pool bucket is accepted at
  "grounded one representative option, deferred the rest with a named diagnostic," no per-option
  computation attempted; the 50-unit remainder is read individually, not pattern-guessed.

#### SD31-E6-F9 — `monster_ability` ingest/instrument lane (2,773 not-done)

**Shape:** `display|grounded` 981 held (**an Epic 2 dependency** — part of the corpus-wide 1,243
`display|grounded` population AT-31-010 now binds, below), `display|not-ingested` 653 not-started,
`computed|not-ingested` 335 not-started, `derived|grounded` 219 held, `derived|not-ingested` 214
not-started, `static|not-ingested` 213 not-started, `static|grounded` 85 held, `ambiguous|not-
ingested` 39 not-started, `display|not-started` 23 not-started, `ambiguous|grounded` 10 held,
`derived|not-started` 1 not-started. **1,295 held** (981 of which is the Epic-2-owned `display|
grounded` cell) + **1,478 not-started** = 2,773.

- **Instrument/lane:** the 981 `display|grounded` cell moves only when Epic 2's verdict-path work
  lands (it is capability-blocked exactly like `ambiguous`, not an ingest gap); the remaining 314
  `static`/`derived` `grounded`-held units move via Epic 6-F11's sweep/fixture coverage; the 1,478
  not-started units are real per-book ingest, PI-gate contract `forward-scope-register.md G1.6` (6
  registered monster books, zero declared-PI tokens today, dormant).
- **Acceptance:** same three-lever discipline as F7; a receipt citing "monster_ability held count
  dropped" without naming which lever (Epic 2 / Epic 6-F11 / ingest) moved it is not acceptance
  evidence.

#### SD31-E6-F10 — `class` ingest/instrument lane (158 not-done)

**Shape:** `computed|not-ingested` 114 not-started, `computed|not-started` 35 not-started,
`display|not-ingested` 7 not-started, `ambiguous|not-started` 1 not-started, `derived|not-started` 1
not-started. **158 of 158 not-done units are `not-started`; zero are `held`.** Dominated by
`computed|not-ingested` (114, 72 %) — this is real ingest work, not a fixture/verdict-path gap, and
overlaps Epic 3's per-class measurement chain (a `class` unit and its `class_feature` units share a
class-of-origin but are separately-tracked kinds; ingesting a class's own `class` record is not the
same act as measuring or wiring its `class_feature` records).

- **Instrument/lane:** real per-book ingest; no probe or fixture blocks this kind.
- **Acceptance:** raw-vs-workable split recorded before a cycle claims a book; reach-gate satisfied
  per record; PI screen cited clean per book (same discipline as F1-F4).

### F11 — held static/derived residual, corpus-wide (added 2026-08-15, launch-readiness remediation Step 2, blocker B4)

**Re-derived this cycle**, corpus-wide, both wiring classes:

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d = json.load(open('docs/work-inventory.json'))
U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
verdict = lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
c = collections.Counter()
for u in U:
    if verdict(u)=='held' and u.get('wiring_class') in ('static','derived'):
        c[u.get('kind')] += 1
print('total', sum(c.values())); print(dict(c.most_common()))
"
```
→ **5,273 held `static`/`derived` units total** (matches the readiness review's B4 figure exactly):
`equipment 2,284`, `monster 1,232`, `spell 1,061`, `companion 322`, `monster_ability 304`,
`class_feature 33`, `equipment_modifier 19`, `feat 17`, `race_trait 1`.

**The two instruments, and their coverage gap:**

- `corpus_literal_sweep` (byte-equality against the corpus literal) covers the `static` rung — wired
  into `verify.sh`'s `corpus-sweep` stage, already run every full gate.
- `derived_evaluator_fixture_check` (evaluator-vs-fixture) covers the `derived` rung, gated on
  `tests/fixtures/rules_core/derived-evaluator-fixtures.json`, whose own doc comment
  (`src/bin/derived_evaluator_fixture_check.rs:14`) states its coverage as "94 of 2,879." **Re-derived
  this cycle, that trailing figure is stale:**
  ```
  python3 -c "
  import json, sys, collections
  sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
  d = json.load(open('docs/work-inventory.json'))
  U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
  elig = [u for u in U if u.get('wiring_class')=='derived' and u.get('status') in
          ('ingested-magnitude','grounded','text-complete')]
  print(len(elig))
  "
  ```
  → **2,792**, not 2,879 (corpus drift since the comment was authored; the fixture file itself
  confirms **94 entries**: `python3 -c "import json; print(len(json.load(open('tests/fixtures/
  rules_core/derived-evaluator-fixtures.json'))['entries']))"` → `94`). `retro.py correction`
  emitted (claimed 2,879, `src/bin/v06_work_inventory.rs:4585` and `derived_evaluator_fixture_check.
  rs:14`, → actual 2,792, verified by the two commands above).

**Owner and lane.** This seed extends both instruments' coverage — it does not build a third one.
`monster`'s F1 above (1,229 of the 5,273) and F5/F7/F9's `held` cells above are this lane's direct
consumers; a cycle here grows either `corpus_literal_sweep`'s cited-file coverage or `derived-
evaluator-fixtures.json`'s entry set and re-runs the relevant per-kind figure above to show the
delta, never claims a `held`→`done` move without a coverage-instrument citation.

- **Acceptance:** every unit moved off `held` cites the specific `corpus_literal_sweep` file or
  `derived-evaluator-fixtures.json` entry that verified it; the two re-derivation commands above are
  re-run and their deltas quoted in the cycle's receipt; no `held` unit is marked `done` by widening
  `doneness_verdict()`'s table itself (Decision 1(a) forbids widening a bucket definition to move a
  count) — only by adding real, checkable coverage.

**Acceptance (per card, F1-F10):** the raw-vs-workable split is recorded with its command before any
cycle claims; the pre-cycle classifier/screen ran against the candidate book before the round was
committed; PI screen cited clean for the book before any record is written; reach-gate satisfied per
record ingested; units found structurally unreachable are proposed to the Structural Exclusion
Register with `decisions.md §3`'s four items, never silently ingested as if grounded and never
deferred by cycle fiat.

**Not in this epic:** `class_feature` ingest (Epic 5, per-class chassis sweep) — `class_feature` is
never an Epic 6 kind, even though `class` (F10) now is. **Superseded 2026-08-15 (launch-readiness
remediation Step 2, blocker B2):** the prior text here read "`equipment`/`equipment_modifier`/
`companion`/`feat`/`monster_ability` — not named in the operator's cited frustration list; a future
pass may open cards for them ... without a new operator ruling," carried forward, dated, per this
program's doc convention. **All six previously-unowned kinds (those five plus `class`) now have
cards — F5-F10 above — opened by this remediation step, not by a new operator ruling** (the operator's
2026-08-15 ruling directed "open cards for the six unowned kinds," `decisions.md §5`). Nothing in
this epic's kind roster is unowned as of this step.

## Epic 7 (SD31-E7) — Book Onboarding, 100 % mandate (was SD31-E5; orig. SD30-E11)

**Objective:** onboard the 7 `future_state` books — `occult_adventures`, `adventurers_guide`,
`mythic_adventures`, `inner_sea_magic`, `inner_sea_temples`, `inner_sea_taverns`, `inner_sea_faiths`.
The population these books add is not yet in the engine at all; closing to 100 % requires bringing it in.

**Derived from:** `SD-30 decisions.md §45` item 3.

**Gate:** SD-30's Epic 3 PI screen cited clean per book before any record lands (satisfied at package
level; cited per book).

**Cost note, not a scope note:** the recorded calibration is ~1.5-2 h per book of real work, dominated
by fixed per-file cost (~7 count-pinning files), with content nearly free after that. Do not
extrapolate a blended per-record rate.

**Not in this epic:** any book already in-scope under Epic 6's kind lanes or Epic 5's chassis sweep.

## Epic 8 (SD31-E8) — Cloud Fan-Out Protocol (merged: was SD31-E6 + SD32-E3; orig. SD30-E14)

**Objective:** the local-proof-then-cloud-scale protocol that lets build-heavy, self-contained lanes
scale to cloud agents after one local proof cycle per lane shape. Covers **both** lane families now:
the grind lanes (Epic 6 ingest, Epic 7 onboarding) and the capability-build lanes (Epic 1's per-race
rollout once its design is proven on one race).

**Merged by `decisions.md §2`:** SD-31 and SD-32 each carried an independent copy of this protocol with
identical rules, on the reasoning that their lane shapes never overlap. One package cannot drift from
itself, so the copies are collapsed into this single epic.

**Derived from:** `SD-30 decisions.md §47`.

**Rules carried into every cycle dispatched under this epic:**

1. Every cloud agent works its own branch — never two writers on one branch.
2. The local orchestrator owns all merges to `tranche/10`, verified by content, not commit count.
3. DoD-8 on-screen verification and dashboard-producer work stay local — no cloud agent runs either.
   Load-bearing for Epic 1 specifically: its DoD-8 mandate cannot be satisfied by a cloud agent.
4. **Every cloud runner bootstraps the PCGen oracle before its first `verify.sh`.** Run
   `scripts/fetch-pcgen-oracle.sh` (network + npm/crates.io egress required), export
   `PCGEN_CORPUS_ROOT`/`PCGEN_REPO_DIR` from its printed output, and quote the pin SHA
   (`scripts/pcgen-oracle-pin.env`) in the cycle receipt. A runner that cannot fetch the oracle does
   not run any corpus-touching lane (Epic 6 ingest, Epic 7 onboarding, `corpus-sweep`) — it is not a
   local-machine-path problem, it is a hard precondition checked by `verify.sh --only
   preflight-oracle` before anything else.

**Not in this epic:** the ingest, onboarding, or capability design work itself — this epic is the
dispatch protocol for scaling work already proven locally.

## Epic 9 (SD31-E9) — Closure and the 100 % Exit Gate (NEW, `decisions.md §2`/`§3`)

**Objective:** close this package against a mechanically checkable bar, so that "SD-31 closed but the
mandate silently did not" is a state that cannot be reached without an operator having signed for it.

### Feature seeds

#### SD31-E9-F1 — The exit gate

Acceptance:

- Epic 0's reachability audit re-run at the closing tip, with its output in the receipt.
- **`reachable ceiling == 100 %`, OR every shortfall unit carries a Structural Exclusion Register entry
  with operator sign-off** (`decisions.md §3`). No third option exists. In particular the phrase "or
  named a successor for the remainder," struck from this package by `decisions.md §2`, is not
  reintroduced.
- **The doneness bar (`decisions.md §5`, operator ruling 2026-08-15), alongside — not instead of — the
  reachable-ceiling bar above:** `done / denominator == 100 %`, where `denominator` is every unit in
  `docs/work-inventory.json` except `EXCLUDED_BOOKS` (today 38,521), OR every shortfall unit carries a
  signed `AT-31-100` register entry. A reachable ceiling of 100 % does not by itself satisfy this bar —
  reachability is a capability-gap check ("could a unit get to `done`"), doneness is the actual count
  ("did it"). Both bars must pass; a receipt quoting only the reachable ceiling is not a valid closure.
  Re-derive with the command in `AT-31-103`.
- Board position stated per kind — `done`/total with the command — and the delta from this package's
  opening baseline, computed by replaying the dashboard producer's own `doneness_verdict()` over
  `git show <ref>:docs/work-inventory.json` at both ends, never by comparing status counts.
- `./scripts/verify.sh` green at the closing tip, exit code captured directly.
- `release-notes.md` populated; `docs/architecture/` refreshed.

#### SD31-E9-F2 — Honest-closure precedent

Acceptance:

- SD-29's closure history is the standard: its first attempt was premature and the operator reopened
  it, its second refused to close with 63 workable units outstanding, its third closed honestly. A
  closure cycle that cannot meet F1 writes a closure-blocked receipt naming exactly what is
  outstanding — it does not close and it does not open a promotion PR.

## Recommended sequencing

```
E0 (reachability audit) runs FIRST and re-runs at every epic closure.

Track A - capability, must lead the lanes that consume it:
  E1 (race chassis)  -> opens E6-F3 / E6-F4 per race batch
  E2 (verdict paths) -> opens E3-F4 and E5-F3; must resolve the `ambiguous` dead-end or register it

Track B - class_feature chain, file-disjoint from Track A, runs concurrently from the start:
  E3 (measurement, per class) -> E4 (mechanism, per class) -> E5 (chassis sweep, per class)
  ...except E3-F4 and E5-F3, which wait on E2.

Track C - ingest and onboarding, after their gates:
  E6-F1 (monster) and E6-F2 (spell) may start immediately - not capability-blocked.
  E6-F3 / E6-F4 (race, race_trait) start per race batch as E1 delivers.
  E7 (book onboarding) may start immediately - PI gate already satisfied.

E8 (cloud fan-out) becomes available to any lane shape with one local proof cycle.
E9 (closure) fires LAST, against the E0 audit, with no deferral hatch.
```

## Completion gate

SD-31 closes when:

- **Epic 0's audit reports a reachable ceiling of 100 %**, or every shortfall unit carries an
  operator-signed Structural Exclusion Register entry (`decisions.md §3`).
- **The doneness bar is satisfied** (`decisions.md §5`, `AT-31-103`, operator ruling 2026-08-15):
  `done / denominator == 100 %` against the full 38,521-unit mandate denominator (every unit in
  `docs/work-inventory.json` except `EXCLUDED_BOOKS`), or every shortfall unit carries a signed
  `AT-31-100` entry. This bar is **separate from and additional to** the reachable-ceiling bar
  immediately above — a 100 % reachable ceiling with the board still at 15 % `done` does not close
  the package. `AT-31-005`'s per-kind `done+held` figures are progress floors that inform whether a
  kind is moving; they are not, on their own, a closure criterion for this bar or any other.
- Epic 1 has landed a race chassis for the chassis-blind population, DoD-8 verified per race added.
- Epic 2 has landed the ground-truth sample and either a validated classifier or a documented close-at-F1,
  **and the `ambiguous` wiring class has a path to `done` or a signed register entry, and the
  1,243-unit `display|grounded` population has been examined by the same classifier** (`AT-31-010`,
  widened 2026-08-15).
- Epic 3 has measured every remaining `class_feature`-bearing class.
- Epic 4 has landed the supersession shape for every class Epic 3 clears, and resolved the
  chooser-interaction shape for Oracle/Arcanist/Sorcerer.
- Epic 5's chassis sweep has ingested and reach-gated every class Epics 3/4 cleared, **and its F4 (the
  36 `deferred-with-reason` units, added 2026-08-15) has a real path or a signed register entry for
  each unit — none silently carried past closure**.
- Epic 6's per-kind ingest/instrument cards — F1 (`monster`, fixture-coverage), F2 (`spell`), F3/F4
  (`race`/`race_trait`), F5-F10 (`equipment`, `equipment_modifier`, `companion`, `feat`,
  `monster_ability`, `class`, added 2026-08-15), F11 (held static/derived residual, added
  2026-08-15) — have each reached a ceiling that is **a capability fact, not a scheduling artifact**
  — F3/F4 re-derived after Epic 1's final chassis batch; **no kind in the 38,521-unit denominator is
  outside this epic's card roster as of this closure**.
- Epic 7 has onboarded all 7 `future_state` books, PI-clean.
- Epic 8 has run at least one local-proof-then-cloud-scale cycle per lane shape it claims a role in.
- Epic 9's exit gate passes; `progress.md` carries the closure receipt; the promotion PR is opened, not
  merged — the operator holds sole merge authority.

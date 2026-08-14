# Forward-Scope Register — SD-30

**Re-scoped 2026-08-10 (`decisions.md §33-38`).** SD-30 is no longer the
sixteen-book content bundle this register was written for — it is now the
`class_feature`/archetype bundle, corpus-wide across 23 books. The book-list
entries below (Class 2's C2.1-C2.4, Class 3's C3.1-C3.3) are **retired as
moot**, not merely stale: they deferred specific *books* from a book-scoped
bundle that no longer has a book list. Retained inline, marked RETIRED, so
the reasoning is not lost. A new C1.3 records SD-30's own inheritance as the
`class_feature`/archetype successor `corpus-work-channels.md §9.1` funded.

This register captures work downstream of SD-30. SD-30's successor bundles
would consume SD-30's outputs, but the next post-tranche bundle isn't yet
named; it's recorded here as **Class 1**. SD-30-specific retrofits land in
**Class 3**.

## Class 0 — Doctrinal anchors (always-on)

| Anchor | Path | Note |
|--------|------|------|
| Per-book ingest pipeline | `docs/governance/book-ingestion-playbook.md` | Doctrine-of-record; pre-cycle trap-report is mandatory |
| Reach gate | `apps/desktop/src-tauri/src/reach_gate.rs` | Definition-of-done per `decisions.md §18` (prime rule); gate's `OPEN_FINDINGS` carries missing-surface prerequisites |
| Identifier discipline | `docs/doctrine-external/identifier-discipline.md` | SD-30 inherits; Epic 1 enforces |
| Build-version scheme | `<major>.<tranche-base>.<build>` (2026-07-17 amendment) | SD-30 first concrete value `0.10.<build>` |
| Source STC chassis | `spec-domain-bundle-authoring` skill | 13-file shape per the modern chassis |
| Move-not-copy publish | `release-package-promotion` skill | Workspace tree removed on publish commit |
| Hermes board retirement | SD-28 `decisions.md §15a` (2026-08-01) | All post-2026-08-01 bundles are local-file only |
| "Recently published takes precident" | SD-30 `decisions.md §16` (operator directive 2026-08-01) | Cross-bundle precedence for SD-28/SD-29's already-published surfaces |

## Class 1 — Predecessor-deferred (named successor owns)

### C1.1 — Post-tranche consumer

**Owner:** not yet named.

**What depends on SD-30:** SD-30 is the third post-2026-08-01 bundle
(SD-28, SD-29, SD-30). The next bundle would consume SD-30's
content-source-ingest outputs (occult + mythic + Monster Codex + Inner Sea
+ Book of the Damned cycles). Operator-pinned when the next bundle
opens.

### C1.2 — Class-grant overlap with SD-28

**Owner:** SD-30. **Unchanged by the 2026-08-10 re-scope** — this is a class-identity join, not a
book-list item.

**What depends on SD-30:** SD-30 owns canonical class definitions for the
four shared classes (Occultist, Spiritualist, Medium, Mesmerist) that
appear in both Ultimate Intrigue (SD-28's territory) and Occult
Adventures (SD-30's territory). SD-28's Epic 6 references the canonical
class id from SD-30's progress; SD-30 does not redefine.

### C1.3 — `class_feature`/archetype: SD-30 is the named successor `corpus-work-channels.md §9.1` funded (2026-08-10)

**Owner:** SD-30 itself (this bundle) — recorded for cross-reference, mirroring
`SD-29-corpus-wide-catch-up-lanes/successor-forward-scope-register.md C1.3`, which named
`class_feature`'s owner as "whichever bundle executes `corpus-work-channels.md §9.1`", SD number
unassigned at the time it was written (SD-29's `decisions.md §38.4`, same wording).

**What SD-30 inherits:** the full `class_feature` population (15,472 units, 23 books,
`decisions.md §33`) and SD-28's already-landed measurement (`§60`/`§63`/`§64` — 25/28 classes
hand-verified, 175 mechanisms, two wiring shapes; `decisions.md §34` verifies this directly against
the SD-28 decisions, not the commit message alone). Not a cold start.

**Cross-bundle doc:** `SD-29-corpus-wide-catch-up-lanes/successor-forward-scope-register.md C1.3`
should be read alongside this entry; both name the same funded effort from opposite sides.

### C1.4 — Per-class PI-blacklist sweep: mechanism delivered, invocation contract for the ingest lane (2026-08-14, `SD30-E3-F1-001`)

**Owner:** `SD-31-corpus-closure-grind` (its Epic 3, ex-SD-30 Epic 6 — `decisions.md §51`).

**What SD-30 delivers:** `SD30-E3-F1` closed (`decisions.md §52`) without a `class_feature` ingest
lane to wire into, because that lane (Epic 6) moved to SD-31 before this cycle fired. The PI-blacklist
sweep mechanism itself (`codex::rules_core::pi_table_sweep::screen_generated_table`, the shared
`pi_screening::PI_BLACKLIST_TERMS`) is already built, already production-wired (two live non-test
callers, `gen_feat_gap_tables.rs`/`gen_equipment_gap_tables.rs`), and proven this cycle against real
`class_feature`-shaped content (`tests/pi_table_sweep.rs`'s two new tests, plus two pre-existing
`real-leak` baseline rows already found inside shipped `archetype_tables.rs` files). `decisions.md
§52.3` states the exact six-step invocation contract SD-31's Epic 3 generator/transcriber must follow.

**What SD-31 must do:** call `screen_generated_table` before writing any generated `class_feature`
table text, hard-stop on a non-empty result, and record the outcome in that book's cycle's first
receipt — the contract, not a re-derivation of it, per `decisions.md §52.3`.

## Class 2 — RETIRED 2026-08-10 (book-list deferrals, moot under the `class_feature` re-scope)

**The four book-specific deferrals below (C2.1-C2.4) are retired, not merely stale.** They deferred
whole *books* from a book-scoped bundle. SD-30 no longer has a book list — see `decisions.md §35`.
NPC Codex and Planar Adventures were already confirmed absent from the whole corpus (Decision §32's
re-verification), so nothing changes for them regardless of scope shape. Occult Origins and Haunted
Heroes Handbook are real corpus directories; if either ever carries `class_feature` units, SD-30
picks them up automatically as part of its corpus-wide `class_feature` scope — no separate deferral
decision is needed, and none is recorded here. If either carries only non-`class_feature` content,
SD-29's corpus-wide lanes own it. Retained below, verbatim, for the audit trail.

### C2.1 — NPC Codex (RETIRED — moot, book-list scope no longer exists)

The NPC Codex is a real Paizo product; the corpus directory is not
under `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/`
as of 2026-08-01 (re-confirmed 2026-08-02, `decisions.md §32`). Per the 2026-08-01 absent-book rule,
NPC Codex dropped from the old book-scoped scope. Under the `class_feature` re-scope this is moot —
SD-30 has no book list to defer it from; if the book is ever acquired and carries `class_feature`
units, it is in scope automatically.

### C2.2 — Planar Adventures (RETIRED — moot, book-list scope no longer exists)

Same disposition as C2.1 — Planar Adventures is a real Paizo product
without a `planar_adventures/` corpus directory. Retired as moot under the re-scope.

### C2.3 — Occult Origins (RETIRED — moot, book-list scope no longer exists)

**Present in the corpus** at
`~/workspace/repos/pcgen/data/pathfinder/paizo/player_companion/occult_origins/`
(7 `.lst`, `_occult_origins.pcc`; verified 2026-08-01). Previously deferred from SD-30's old
sixteen-book scope by explicit operator choice. Retired as moot under the re-scope — SD-30 now picks
up whatever `class_feature` content this book carries automatically, as one of the 23 in-scope books
(if any; not confirmed present in the 23-book table at `decisions.md §33`, re-check at Epic 4-F1's
class inventory). Any non-`class_feature` content is SD-29's corpus-wide territory.

### C2.4 — Haunted Heroes Handbook (RETIRED — moot, book-list scope no longer exists)

**Present in the corpus** at
`~/workspace/repos/pcgen/data/pathfinder/paizo/player_companion/haunted_heroes_handbook/`
(14 `.lst`, `_haunted_heroes_handbook.pcc`; verified 2026-08-01). Same disposition as C2.3 — retired
as moot under the re-scope.

## Class 3 — Retrofit (operator-on-request)

### C3.1 — Mythic Adventures reach-surface prerequisite (RETIRED as book-wide; narrowed to `class_feature` tier features only)

**Narrowed, not fully retired**, unlike C2.1-C2.4 — Mythic Adventures' *mythic-path tier features* are
`class_feature`-shaped and stay in SD-30's scope; its monster stat blocks and non-`class_feature`
mechanics do not (SD-29's territory now). Per `decisions.md §18` reach-gate = DoD, Epic 6 cycles pause
on `decision-blocked` if no consumer surface reaches the gate for a mythic-path tier feature.

The remedy is either (a) a campaign-tool consumer epic inside SD-30, or (b) a separate bundle. The
operator decides per cycle — when attended.

**Unattended safe default (2026-08-01, per `loop-instruction.md` UNATTENDED MODE):** never invent a
surface and never add an epic on the cycle's own authority. Classify each record family into
`RECORD_TYPE_KINDS` or `SUPPORTING_RECORD_TYPES` only where honestly justifiable; otherwise record an
`OPEN_FINDINGS` entry naming the remedy, count it as a cycle shortfall, record `decision-blocked` in
`progress.md`, and continue to the next ready card.

### C3.2 — Occult Adventures psychic-discipline consumer surface (RETIRED as book-wide; narrowed to `class_feature` scope)

**Narrowed, not fully retired** — Occult Adventures' psychic-discipline mechanics are
`class_feature`-shaped (per-class chooser content, same family as archetype/discovery/mystery pools
per `decisions.md §38`) and stay in scope for Epic 4/6's characterization. Non-`class_feature` Occult
Adventures content (spells, equipment) is SD-29's.

**Unattended safe default (2026-08-01):** same rule as C3.1 — classify into an existing family via
`SUPPORTING_RECORD_TYPES` only where the discipline genuinely surfaces as a class feature the sheet
already renders; otherwise `OPEN_FINDINGS` + recorded shortfall + `decision-blocked`, and move on.

### C3.3 — Inner Sea series campaign-tool surface (RETIRED — moot, no non-`class_feature` Inner Sea content is SD-30's)

The Inner Sea series (×9 modules) is primarily campaign-setting data (traits, regions, factions) —
none of that is `class_feature`-shaped. **Fully retired under the re-scope**: SD-29's corpus-wide
lanes own every Inner Sea kind except `class_feature`, and Inner Sea's `class_feature` population is
small (`inner_sea_combat` 314, `inner_sea_magic` 218, `inner_sea_world_guide` 171, `inner_sea_intrigue`
169, per `decisions.md §33`'s table — likely per-class archetype content, tracked inside Epic 4/6 like
any other book, not a standalone campaign-tool-surface risk).

## Class 4 — Measured inheritance from tranche/7 (SD-30-specific, derived 2026-08-01; re-scope caveat 2026-08-10)

**Caveat added 2026-08-10:** the four findings below (C4.1-C4.5) were derived against the old
sixteen-book scope. Under the re-scope, SD-30 only ingests `class_feature` content from these books;
where a finding is about a different kind (a spell list, a monster count), it is now informational —
SD-29 owns the ingest that would trigger it. Where a finding is about `class_feature` content
specifically (C4.2's Shaman Spirits, keyed to class features), it remains live and unchanged.

Findings that are **about this bundle specifically**. Zero tranche/7 deferrals route to SD-30 — the
register says so rather than filling the table. Its real inheritance was found instead by grepping
**shipped source for SD-30's own book names**, which surfaced three live constraints that no deferral
recorded. Sources: `docs/retro/tranche-7-retrospective.md`.

**Method worth repeating before cycle 1:** `command grep -rn "<book_slug>" --include=*.rs src/` for each
book in scope. Shipped code carries correctness constraints keyed to books that are *not yet ingested*,
and those constraints become false the moment the book lands. A deferral register cannot find them
because nobody deferred them — they were written as facts that silently expire.

### C4.1 — Ingesting `horror_adventures` invalidates a shipped constant, by its own doc comment

`src/rules_core/durability.rs:333` ships `FAMILIAR_TOAD_MAX_HP_BONUS: i16 = 3`. Its doc comment
(`:325–332`) states the negative `FamiliarGrantedBonus_N|-1/-2/-3/-4` setters that would cancel it are
*"provably vacuous here"* because **every one lives in `player_companion/familiar_folio` or
`horror_adventures`, neither ingested** — and closes with:

> *"Re-verify if the ingested book set ever widens."*

**SD-30 ingests `horror_adventures`. That widening is this bundle.** The constant does not become wrong
automatically, but its stated justification expires on the day the book lands, and the comment names the
exact re-verification: trace all twelve setters to their files.

**Readiness:** schedule the re-verification in the same cycle that ingests `horror_adventures`, not
after. This is a correctness constraint with a trigger, not a deferral.

### C4.2 — The Shaman later-book Spirits split across SD-30 and SD-28; neither closes it alone

`src/rules_core/pilot_compute.rs:19364` and `:19586` carry a claim-blocking diagnostic naming *"the two
later-book Spirits (Mammoth, Wood)"* this codebase does not recognise.

**Mammoth is SD-30's; Wood is SD-28's.** Whichever bundle lands first will find the diagnostic still
firing on the other's Spirit and must resist closing it. **The finding for both registers: this is a
two-bundle claim and closing it requires both** — a cycle that flips it on one Spirit has made the
diagnostic lie.

### C4.3 — A measurement-shape trap already recorded in shipped source, keyed to SD-30's books

`src/rules_core/rules_tables/acg/bloodrager_spell_list.rs` documents that a tree-wide count of its
spells returns 220, sweeping in `monster_codex`, `inner_sea_races`, `adventurers_guide` and
`aquatic_adventures` — books the repo does not ingest — and names that *"measurement-shape error"* as
the cause of a stale figure the task had been carrying.

**Every spell/feat list in this repo is scoped to a single book's `.lst` on purpose.** SD-30 widens the
ingested book set, so any tree-wide count taken today and re-taken after ingest will move for reasons
that have nothing to do with the cycle's work. **Scope every count to its source file and state the
file**, per SD-27 `decisions.md §27.1`.

### C4.4 — Two scope hazards derived, neither recorded in the bundle's own docs

- **Occult Adventures is a subsystem, not a content drop.** Nine classes' worth of psychic magic with
  **472 spell keys not defined in any currently-ingested book**, plus its own casting mechanics.
  `C3.2` already flags the consumer surface; the *ingest* side is the larger half and is unsized.
- **Mythic Adventures is predominantly a `.MOD` graft layer.** `ma_spells.lst` carries 279 rows and
  only **10 distinct non-`.MOD` keys** — the rest modify records defined elsewhere. Shape B has no
  precedent for a record that exists only as a delta on another book's record, and SD-27 hit the same
  shape with ARG's races (`decisions.md §25.2`: 37 `.MOD` lines declaring nothing). **Resolve the
  schema question before the first Mythic cycle dispatches**, or it will be resolved per-record by
  whoever hits it first.

### C4.5 — Shared with SD-28 and SD-29: pay the pipeline debt once

There is no single ingestion pipeline. Four binaries (`ingest_races.rs`, `ingest_race_traits_arg.rs`,
`ingest_pu_classes.rs`, `cache_gen/apg.rs`) carry three private partial copies of the PCGen description
treatment; only `codex::rules_core::pcgen_desc::render_pcgen_desc` is sanctioned. SD-27 paid this defect
three times in three places.

Likewise the **magnitude predicate**: four reasonable variants of "does this record carry a computed
magnitude" returned 48/49/51/52 on one unchanged tree, so any coverage ratio SD-30 publishes will not be
comparable to SD-28's, SD-29's or SD-27's until an optional `source_record` lands on
`ComputationExplanation`.

**Ownership rule agreed across all three registers: whichever bundle dispatches first pays it; the
others re-verify rather than re-implement.** See `../SD-29-corpus-wide-catch-up-lanes/forward-scope-register.md §7.4`
(directory renamed 2026-08-10 from `SD-29-bestiary-line-book-ingestion` when SD-29 was re-scoped
corpus-wide, `decisions.md §38` in that package)
and `§7.6`, and `../SD-28-ultimate-book-content-ingestion/forward-scope-register.md §C4.3`–`§C4.4`.

## Review trigger

Reopen SD-30's forward-scope register when:

- A successor bundle reaches into SD-30's outputs.
- A class-grant resolution fires for the four shared classes.
- A missing-surface gap is recorded in `reach_gate.rs OPEN_FINDINGS` for a `class_feature` record
  (Mythic tier features, Occult psychic disciplines, or any other class's chooser content).
- Epic 4 names a successor for the remainder of the per-class measurement, or funds the
  chooser-interaction primitive design for Oracle/Arcanist/Sorcerer.
- The 303-unit genuinely-unreachable `unknown` subset or the 1,772-unit unclustered remainder
  (`decisions.md §38`) gets an operator ruling on net-new engine work.
- The post-`tranche/10` consumer is operator-named.

Closed-form: the bundle closes when the Closure Epilogue fires.

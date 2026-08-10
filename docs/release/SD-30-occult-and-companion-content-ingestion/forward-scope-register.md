# Forward-Scope Register — SD-30

This register captures work downstream of SD-30. SD-30 ships the final
content-ingest slot on the post-2026-08-01 trio (SD-28, SD-29, SD-30).
SD-30's successor bundles would consume SD-30's outputs, but the next
post-tranche bundle isn't yet named; it's recorded here as **Class 1**.
Bundles that depend on the four deferred books (NPC Codex and Planar
Adventures, absent from the corpus; Occult Origins and Haunted Heroes
Handbook, present but deferred by operator choice 2026-08-01) live in
**Class 2** as deferred.
SD-30-specific retrofits land in **Class 3**.

## Class 0 — Doctrinal anchors (always-on)

| Anchor | Path | Note |
|--------|------|------|
| Per-book ingest pipeline | `docs/governance/book-ingestion-playbook.md` | Doctrine-of-record; pre-cycle trap-report is mandatory |
| Reach gate | `apps/desktop/src-tauri/src/reach_gate.rs` | Definition-of-done per `decisions.md §18` (prime rule); gate's `OPEN_FINDINGS` carries missing-surface prerequisites |
| Identifier discipline | `~/workspace/governance/identifier-discipline.md` | SD-30 inherits; Epic 1 enforces |
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

**Owner:** SD-30.

**What depends on SD-30:** SD-30 owns canonical class definitions for the
four shared classes (Occultist, Spiritualist, Medium, Mesmerist) that
appear in both Ultimate Intrigue (SD-28's territory) and Occult
Adventures (SD-30's territory). SD-28's Epic 6 references the canonical
class id from SD-30's progress; SD-30 does not redefine.

## Class 2 — Future-acquired (deferred)

### C2.1 — NPC Codex

The NPC Codex is a real Paizo product; the corpus directory is not
under `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/`
as of 2026-08-01. Per the 2026-08-01 absent-book rule, NPC Codex drops
from scope. A future bundle (or runtime operator directive) may acquire
the LST data and bring it in.

### C2.2 — Planar Adventures

Same disposition as C2.1 — Planar Adventures is a real Paizo product
without a `planar_adventures/` corpus directory. Deferred.

### C2.3 — Occult Origins

**Present in the corpus** at
`~/workspace/repos/pcgen/data/pathfinder/paizo/player_companion/occult_origins/`
(7 `.lst`, `_occult_origins.pcc`; verified 2026-08-01). The 07-30 stub's
"not present" finding searched only `roleplaying_game/`. Deferred from SD-30
by explicit operator choice 2026-08-01 — no acquisition needed; a future
bundle picks it up directly.

### C2.4 — Haunted Heroes Handbook

**Present in the corpus** at
`~/workspace/repos/pcgen/data/pathfinder/paizo/player_companion/haunted_heroes_handbook/`
(14 `.lst`, `_haunted_heroes_handbook.pcc`; verified 2026-08-01). The 07-30
stub searched the bare stem `haunted_heroes` and the wrong subtree. Deferred
from SD-30 by explicit operator choice 2026-08-01 — no acquisition needed; a
future bundle picks it up directly.

## Class 3 — Retrofit (operator-on-request)

### C3.1 — Mythic Adventures reach-surface prerequisite

Mythic Adventures' reach surfaces are existential (the mythic path
mechanics + tier features + monster stat blocks all require consumer
integration). Per `decisions.md §18` reach-gate = DoD, cycles pause on
`decision-blocked` if no consumer surface reaches the gate.

The remedy is either (a) a campaign-tool consumer epic inside SD-30, or
(b) a separate bundle that consumes Mythic Adventures' records. The
operator decides per cycle — when attended.

**Unattended safe default (2026-08-01, per `loop-instruction.md` UNATTENDED
MODE):** never invent a surface and never add an epic on the cycle's own
authority. Classify each record family into `RECORD_TYPE_KINDS` (with the
surface that really renders it) or `SUPPORTING_RECORD_TYPES` (with why it is
a facet of an existing family) only where honestly justifiable; otherwise
record an `OPEN_FINDINGS` entry naming the remedy, count it as a cycle
shortfall (Definition-of-done items 2 and 6), record `decision-blocked` in
`progress.md` with the reason, and continue to the next ready card. The
operator rules on (a)-vs-(b) after return.

### C3.2 — Occult Adventures psychic-discipline consumer surface

Occult Adventures' psychic-discipline mechanics (`psychic_discipline_*`
records) require a class-feature consumer surface to satisfy reach.
`reach_gate.rs OPEN_FINDINGS` flags missing surfaces per the per-cycle
audit.

**Unattended safe default (2026-08-01):** same rule as C3.1 — classify into
an existing family via `SUPPORTING_RECORD_TYPES` only where the discipline
genuinely surfaces as a class feature the sheet already renders; otherwise
`OPEN_FINDINGS` + recorded shortfall + `decision-blocked`, and move on. Do
not build a new consumer surface on the cycle's own authority.

### C3.3 — Inner Sea series campaign-tool surface

The Inner Sea series (×9 modules) is primarily campaign-setting data
(traits, regions, factions). Per-book ingest produces canonical records;
the cycle's reach gate may flag missing consumer integration (e.g.,
a campaign-setup wizard surface). Per-cycle gap filing.

**Unattended safe default (2026-08-01):** same rule as C3.1 — ingest the
record families whose surfaces exist (traits and feats the sheet renders);
for campaign-tool-only families, `OPEN_FINDINGS` + recorded shortfall +
`decision-blocked`, and move on. No campaign-setup wizard is built on the
cycle's own authority.

## Class 4 — Measured inheritance from tranche/7 (SD-30-specific, derived 2026-08-01)

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
- Operator authorizes NPC Codex / Planar Adventures / Occult Origins /
  Haunted Heroes retrofit.
- A missing-surface gap is recorded in `reach_gate.rs OPEN_FINDINGS`
  for Occult / Mythic / Inner Sea records.
- The post-`tranche/10` consumer is operator-named.

Closed-form: the bundle closes when the Closure Epilogue fires.

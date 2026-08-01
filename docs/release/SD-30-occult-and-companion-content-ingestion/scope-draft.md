# SD-30 — Occult + Companion Content Ingestion

**Status:** Planning-ready (operator directives 2026-08-01; canonical chassis landing this cycle).
**Operator pin:** 2026-08-01 (refines 2026-07-28 stub)
**Branch:** `tranche/10` (operator-pinned 2026-08-01; SD-30 takes its own tranche parallel to SD-28's `tranche/8` and SD-29's `tranche/9`, deliberately off the `tranche/6` family used by SD-22's Bestiary 1 baseline).
**Board:** Local-file only. The Hermes board is retired per operator directive 2026-08-01. The work-queue artifact is `kanban.md` paired with `progress.md` inside this directory. There is no `codex-tranche-10` Hermes board; the slug is reserved-as-form, not as-instance.
**Build version target:** `0.10.<build>` first concrete value. tranche-base = 10 per `<major>.<tranche-base>.<build>` scheme (tranche-base is the base digit of the active working tranche, per the 2026-07-17 build-version amendment). Major stays `0` until first main-publish.
**Owner:** Todd Hintzmann
**Scope:** universal

> ⚠️ **OPERATING METHOD — REQUIRED FOR THIS BUNDLE**
> This bundle is operated via `/loop 60m /batch /goal ~/workspace/programs/codex/requirements/SD-30-occult-and-companion-content-ingestion/loop-instruction.md`, NOT via ad-hoc single-task invocations. The `/loop` cadence + `/batch` concurrency primitive is the supervisor's file-touch partition that enforces 1-cycle-at-a-time per file. One launch runs to closure.
>
> **Pre-launch checklist (must be true before any cycle fires):**
> 1. `kanban.md` exists at this directory and lists the ready queue (local-file dispatch).

## Book list — operator-pinned CONFIRMED 2026-08-01

Sixteen books in scope (plus NPC Codex and Planar Adventures deferred per the 2026-08-01 absent-book rule).

| Slot | Book | Publisher | Ingest subtype | Path | Corpus dir | Per-entity count |
|------|------|-----------|----------------|------|-----------|------------------|
| 1    | Occult Adventures | Paizo (hardcover) | Per-class cycles + per-monster-block cycles + per-psychic-discipline cycles | `src/rules_core/rules_tables/occult_adventures/` | `roleplaying_game/occult_adventures` ✅ | derived — see §"Book shape" |
| 2    | Horror Adventures | Paizo (hardcover) | Per-monster-block cycles + per-haunt-block cycles + per-corruption-mechanic cycles | `src/rules_core/rules_tables/horror_adventures/` | `roleplaying_game/horror_adventures` ✅ | derived — see §"Book shape" |
| 3    | Mythic Adventures | Paizo (hardcover) | Per-mythic-path cycles + per-monster-block cycles | `src/rules_core/rules_tables/mythic_adventures/` | `roleplaying_game/mythic_adventures` ✅ | derived — see §"Book shape" |
| 4    | Monster Codex | Paizo (hardcover) | Per-monster-block cycles | `src/rules_core/rules_tables/monster_codex/` | `roleplaying_game/monster_codex` ✅ | derived — see §"Book shape" |
| 5    | Book of the Damned Vol. 1 | Paizo (campaign_setting) | Per-archetype cycles + per-monster-block cycles + per-tactic cycles | `src/rules_core/rules_tables/book_of_the_damned_volume_1/` | `campaign_setting/book_of_the_damned_volume_1` ✅ | derived — see §"Book shape" |
| 6    | Book of the Damned Vol. 2 | Paizo (campaign_setting) | Per-archetype cycles + per-monster-block cycles + per-tactic cycles | `src/rules_core/rules_tables/book_of_the_damned_volume_2/` | `campaign_setting/book_of_the_damned_volume_2` ✅ | derived — see §"Book shape" |
| 7    | Inner Sea World Guide | Paizo (campaign_setting) | Per-trait cycles + per-feat cycles + per-region cycles | `src/rules_core/rules_tables/inner_sea_world_guide/` | `campaign_setting/inner_sea_world_guide` ✅ | derived — see §"Book shape" |
| 8    | Inner Sea Combat | Paizo (campaign_setting) | Per-trait cycles + per-option cycles | `src/rules_core/rules_tables/inner_sea_combat/` | `campaign_setting/inner_sea_combat` ✅ | derived — see §"Book shape" |
| 9    | Inner Sea Faiths | Paizo (campaign_setting) | Per-deity cycles + per-trait cycles + per-option cycles | `src/rules_core/rules_tables/inner_sea_faiths/` | `campaign_setting/inner_sea_faiths` ✅ | derived — see §"Book shape" |
| 10   | Inner Sea Gods | Paizo (campaign_setting) | Per-deity cycles + per-domain cycles | `src/rules_core/rules_tables/inner_sea_gods/` | `campaign_setting/inner_sea_gods` ✅ | derived — see §"Book shape" |
| 11   | Inner Sea Magic | Paizo (campaign_setting) | Per-spell cycles + per-magic-trait cycles | `src/rules_core/rules_tables/inner_sea_magic/` | `campaign_setting/inner_sea_magic` ✅ | derived — see §"Book shape" |
| 12   | Inner Sea Races | Paizo (campaign_setting) | Per-race cycles + per-archetype cycles | `src/rules_core/rules_tables/inner_sea_races/` | `campaign_setting/inner_sea_races` ✅ | derived — see §"Book shape" |
| 13   | Inner Sea Temples | Paizo (campaign_setting) | Per-temple cycles + per-trait cycles | `src/rules_core/rules_tables/inner_sea_temples/` | `campaign_setting/inner_sea_temples` ✅ | derived — see §"Book shape" |
| 14   | Inner Sea Taverns | Paizo (campaign_setting) | Per-tavern cycles + per-event cycles | `src/rules_core/rules_tables/inner_sea_taverns/` | `campaign_setting/inner_sea_taverns` ✅ | derived — see §"Book shape" |
| 15   | Inner Sea Bestiary | Paizo (campaign_setting) | Per-monster-block cycles | `src/rules_core/rules_tables/inner_sea_bestiary/` | `campaign_setting/inner_sea_bestiary` ✅ | derived — see §"Book shape" |
| 16   | Inner Sea Intrigue | Paizo (campaign_setting) | Per-trait cycles + per-faction cycles + per-rule cycles | `src/rules_core/rules_tables/inner_sea_intrigue/` | `campaign_setting/inner_sea_intrigue` ✅ | derived — see §"Book shape" |

**Deferred (NOT in scope this turn).** Per the 2026-08-01 absent-book rule ("if content is not in pcgen, remove from scope entirely"):

- **NPC Codex** — `npc_codex` not found in the PCGen corpus. Recorded in `forward-scope-register.md C2.x` as a future-acquisition candidate. A future bundle (or runtime operator directive) may acquire the LST data and bring it in.
- **Planar Adventures** — `planar_adventures` not found in the PCGen corpus. Recorded in `forward-scope-register.md C2.x`.
- **Occult Origins** — `occult_origins` not found in the PCGen corpus. Carried over from the 07-30 stub; recorded in `forward-scope-register.md C2.x` as future-acquisition.
- **Haunted Heroes Handbook** — `haunted_heroes` not found in the PCGen corpus. Carried over from the 07-30 stub; recorded in `forward-scope-register.md C2.x` as future-acquisition.

**Cycle-0 trap-report + work-inventory gating.** Epic 2's pre-flight runs against all 16 in-scope books; for each book, the inventory surfaces the per-book shape (kinds, files_not_enumerated, trap_hits). Per-book cycles dispatch per the shape finding; cycles on the 4 absent slots are NOT in scope.

Verified 2026-08-01 by directory listing of `~/workspace/repos/pcgen/data/pathfinder/paizo/`.


## Book shape — derived, never hand-maintained

**Per-entity counts are not recorded in this package.** They are generated:

```sh
cargo run --locked --bin v06_work_inventory     # regenerates docs/work-inventory.json
```

Read `books[]` for the book in question: `kinds` (units per kind, each with a
`by_status` map), `files_not_enumerated` (files the generator deliberately
skipped — a kind you expected and did not find may be sitting in one),
`trap_hits` (how many times each corpus trap fires in this book), and
`reconciliation` (per-kind corpus-vs-engine delta once ingest starts).

The generator is idempotent by contract: two runs over an unchanged corpus and
engine differ only in `generated_at`. If a rerun moves a number, the corpus or
the engine moved — that is a finding, not a re-baseline.

This replaces the hand-maintained per-entity count. Every figure this project
hand-maintained has drifted and then actively misled; the counting record is in
`docs/governance/book-ingestion-playbook.md` §6.

### Shape finding that affects this bundle's book list

**Two of the four candidate books have no PCGen corpus directory.** The
generator enumerates every directory under
`~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/` — 25 of them,
verified 2026-07-30 by both the generator's book enumeration and a plain
directory listing. `occult_adventures` and `horror_adventures` are present.
**`occult_origins` and `haunted_heroes` are not**, under those names or any
recognisable variant.

Both books are real Paizo products; the finding is about this corpus checkout,
not about the books' existence. Three readings, in decreasing likelihood:

1. The PCGen dataset does not package them separately and their content is
   folded into `occult_adventures` / `horror_adventures`.
2. They live outside `pathfinder/paizo/roleplaying_game` in the PCGen tree
   (e.g. under a companion/player-companion path).
3. The checkout is incomplete.

**This is flagged, not resolved.** Determining which reading holds is a
pre-launch task, and it changes the answer to operator question 1 below. **The
recorded book list is an operator decision and is not changed here.**

Also present in the corpus and unclaimed by SD-28 / SD-29 / SD-30:
`ultimate_wilderness` and `ultimate_campaign` (SD-27 `decisions.md §9` Tier-2),
and `mythic_adventures`, `pathfinder_unchained`, `monster_codex`, `bestiary_6`,
`bonus_bestiary` (SD-27 §9 Tier-1). All seven are recorded there as deferred to
"SD-28+" without a named successor bundle, so none currently has a bundle of
record beyond that deferral. (`advanced_race_guide` and `adventurers_guide` are
SD-27's own two in-scope books and are **not** in this list.)

**Operator questions to confirm when reviewing on a real computer:**

1. Are the three core books (Occult Adventures + Occult Origins + Horror Adventures) in scope, or is the bundle a different cut? The "some others" in the operator message is ambiguous between (a) Horror Adventures (the canonical Paizo companion), (b) Haunted Heroes Handbook (a softcover), and (c) something else. The candidate above reflects the canonical Paizo Occult Adventures product line. **See the shape finding above first: two of the four candidates have no corpus directory to ingest from, which may settle this question by itself.**
2. Per-book ingest pattern: Occult Adventures is multi-subtype (classes + monsters + psychic disciplines). Confirm whether the Epic structure subdivides per subtype, or whether one epic handles all of Occult Adventures with one cycle per subtype.
3. Cross-book class overlap with SD-28 (Ultimate Intrigue): Occultist, Spiritualist, Medium, Mesmerist appear in both Ultimate Intrigue and Occult Adventures. Canonical class definition lives in SD-30 (Occult Adventures is the primary hardcover defining those classes). SD-28 references the canonical class id from SD-30's progress file but does not redefine.
4. Cross-book class overlap with SD-29 (Bestiary 2-5): Some occult monsters appear in both Bestiary books and Horror Adventures. Canonical monster definition lives in whichever book first introduces the monster.

## Scope

- **In scope:** End-to-end content-source ingest for the three (or four) Occult / Horror books enumerated above. Per-class / per-monster-block / per-discipline cycles produce canonical entries that match the SD-22 corpus-source-inventory doctrine-of-record. **"End-to-end" now includes the player surface** — see §"Ingest and surfacing are one unit of work" below and `decisions.md` Decision 11.
- **Out of scope:** Bestiary 1 (closed in SD-22). Ultimate books (separate bundle, SD-28). Bestiary 2-5 (separate bundle, SD-29). Update-UI bug remediation (lifecycle-routed from SD-16, separate).
- **Boundary with SD-28:** Classes that appear in both Ultimate Intrigue and Occult Adventures (Occultist, Spiritualist, Medium, Mesmerist) live canonically in SD-30. SD-28 references the canonical class id only.

## Ingest and surfacing are one unit of work

Content that is ingested but reaches no player surface is the dominant defect
of this project — six instances in a single session, each found by accident and
patched individually while the next appeared.
`apps/desktop/src-tauri/src/reach_gate.rs` now makes it structural: it builds
its inventory from the app's live ingest diagnostic **and** from a filesystem
scan of every `pub const <NAME>: &[<RecordType>]` slice under
`src/rules_core/rules_tables/`. Because SD-30's ingest target **is**
`src/rules_core/rules_tables/<book>/`, the gate applies directly to every cycle
in this bundle.

Two of this bundle's declared ingest subtypes hit known gaps:

- **Per-monster-block cycles.** No monster record in this codebase reaches a
  player today; Bestiary 1's 41 stat blocks are pinned in the gate's
  `OPEN_FINDINGS` with the remedy "a monster catalog command and browser,
  mirroring `spell_catalog.rs` + SpellCatalogScreen.tsx". Occult Adventures and
  Horror Adventures both carry monster content.
- **Per-class-options / per-archetype / per-discipline cycles.** This is
  chooser-shaped content, and chooser-shaped content is where this project's
  scoping most often overreaches. Apply canonical narrowing — ground one
  representative option's real magnitude end to end, then name and defer the
  rest with a derived count. Precedents are the `*-canonical-narrowing-scoping.md`
  documents under `docs/release/v0.6/`.

**Open operator question this package cannot decide for itself.** SD-30's epic
structure contains no surface-building epic. **The operator decides whether the
missing surfaces land inside SD-30 or as named prerequisites outside it; this
package does not add an epic on its own authority.** Skipping them is not
available — the gate fails the cycle either way.

## Per-cycle repo tooling (process, not scope)

Every cycle in this bundle uses the repo's tooling rather than a
bundle-specific process. The full procedure is
`docs/governance/book-ingestion-playbook.md`; the load-bearing points:

| Step | Command | When |
|---|---|---|
| Book shape | `cargo run --locked --bin v06_work_inventory` | Before planning the book's cycles |
| Pre-ingest trap report | `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>` | Before writing any ingest code for that book |
| Verification | `./scripts/verify.sh` | Every cycle, before commit |
| Reach gate | `./scripts/verify.sh --only reach` | Definition-of-done condition |
| Citation audit | `cargo run --locked --bin v06_corpus_trap_report -- --audit` | Definition-of-done condition |

Note for this bundle specifically: the cross-bundle class-overlap rule
(Occultist, Spiritualist, Medium, Mesmerist are canonical to SD-30) is a join
on record **identity**, and the corpus trap catalogue is explicit that a shared
display name never implies a shared record — `KEY:Bard ~ Lore Master` and
`KEY:Skald ~ Lore Master` are the recorded worked example. Join on `KEY:`; the
trap report's per-book `KEY:` namespace listing tells you the prefix to search
under, which is why a grep for a bare leaf name returns zero and reads, wrongly,
as absence.

## Epic structure (proposed)

| Epic | Title | Fires | Notes |
|------|-------|-------|-------|
| 1 | Code-Side Identifier Cleanup | FIRST | Governance base requirement. Per SD-22 Epic 1 pattern. |
| 2 | Operator Pre-Launch | Gating | Pre-launch checklist verification. |
| 3 | Occult Adventures content-source ingest | After Epic 2 | Per-class cycles + per-monster-block cycles + per-psychic-discipline cycles. |
| 4 | Occult Origins content-source ingest | After Epic 2 | Per-class-options cycles + per-archetype cycles. |
| 5 | Horror Adventures content-source ingest | After Epic 2 | Per-monster-block cycles + per-haunt-block cycles + per-corruption-mechanic cycles. |
| 6 | (optional) Haunted Heroes Handbook content-source ingest | After Epic 2 | Per-class-options cycles. |
| 7 | Closure Epilogue | LAST | Tranche promotion version increment. |
| 8 | Build Version Numbering | After Epic 1, before Epic 7 | First concrete value `0.6.<build>`. |

**Acceptance criteria stub:** 30 criteria, 9 epics (one per in-scope book group: occult group, mythic group, monster codex group, Inner Sea World Guide, Inner Sea modules ×9, Book of the Damned ×2 — fires in Epic 3-N+; pre-cycle trap-report + work-inventory output gates each). Mirrors SD-22's epic shape with the seventeen-book expansion.

## What is operator-pinned vs. doctrine

- **Operator-pinned (CONFIRMED 2026-08-01):** Book list (sixteen books: occult + mythic + Monster Codex + Inner Sea series + Book of the Damned ×2; NPC Codex, Planar Adventures, Occult Origins, Haunted Heroes deferred to forward-scope-register). Per-book path locations confirmed. Per-book ingest subtype confirmed. Epic structure (one epic per book group) confirmed. Branch `tranche/10`. No Hermes board; local-file dispatch. Build version `0.10.<build>`. Cross-book conflict rule. Reach-gate DoD doctrine.
- **Doctrine-of-record (already established):** Epic 1 = Code-Side Identifier Cleanup. Operator Pre-Launch gates. Identifier discipline. Build-version scheme (`<major>.<tranche-base>.<build>`). `/loop /batch /goal` operating form. Per-bundle progress file. Reach-gate = definition of done. Engine policy (real-time forbidden; rules-data in scope only when strictly necessary).

## Next step (operator-pinned CONFIRMED 2026-08-01)

All six operator-pinned items are now confirmed:

1. **Book list confirmed** — sixteen books in scope (occult + mythic + Monster Codex + Inner Sea series + Book of the Damned ×2); four books deferred (NPC Codex, Planar Adventures, Occult Origins, Haunted Heroes).
2. **Per-book path locations confirmed** — `src/rules_core/rules_tables/<book>/` for each.
3. **Branch name + board name** — `tranche/10` branch; Hermes board retired in favor of local-file `kanban.md` + `progress.md`.
4. **Build version target** — `0.10.<build>` per `<major>.<tranche-base>.<build>` scheme.
5. **Book ingest subtype per book** — per-class / per-monster-block / per-psychic-discipline / per-haunt / per-corruption-mechanic / per-mythic-path / per-tactic / per-trait / per-feat / per-region / per-deity / per-domain / per-spell / per-race / per-temple / per-event / per-rule / per-faction.
6. **Packaging decision** — promote this source-of-record to `docs/release/SD-30-occult-and-companion-content-ingestion/` in the repo as a planning-ready publication, with the canonical chassis (15 files) landing this cycle.

The bundle is no longer a stub; it is a planning-ready package. Pre-launch checklist remains: `kanban.md`, branch `tranche/10` pushed, OAuth valid, working tree clean, cycle-0 trap-report + work-inventory validation run against all sixteen books in scope.

# SD-30 — `class_feature` / Archetype Bundle

**Re-scoped 2026-08-10** (`decisions.md §33-38`, operator directive). This document's "Book list" and
"Scope" sections below describe the retired sixteen-book-content era; both are superseded by the
"Re-scoped `class_feature` scope" section that follows them. Everything else in this file (branch,
board, build version, operating method, per-cycle tooling) is unchanged by the re-scope and still
governs.

**Status:** Planning-ready (operator directives 2026-08-01; re-scoped 2026-08-10, canonical chassis unchanged).
**Operator pin:** 2026-08-01 (refines 2026-07-28 stub)
**Branch:** `tranche/10` (operator-pinned 2026-08-01; SD-30 takes its own tranche parallel to SD-28's `tranche/8` and SD-29's `tranche/9`, deliberately off the `tranche/6` family used by SD-22's Bestiary 1 baseline).
**Board:** Local-file only. The Hermes board is retired per operator directive 2026-08-01. The work-queue artifact is `kanban.md` paired with `progress.md` inside this directory. There is no `codex-tranche-10` Hermes board; the slug is reserved-as-form, not as-instance.
**Build version target:** `0.10.<build>` first concrete value. tranche-base = 10 per `<major>.<tranche-base>.<build>` scheme (tranche-base is the base digit of the active working tranche, per the 2026-07-17 build-version amendment). Major stays `0` until first main-publish.
**Owner:** Todd Hintzmann
**Scope:** universal

> ⚠️ **OPERATING METHOD — REQUIRED FOR THIS BUNDLE**
> This bundle is operated via the in-harness `Workflow` tool driven from a live session, NOT via `/loop`/`/batch` and NOT via ad-hoc single-task invocations — see `loop-instruction.md`'s OPERATING METHOD callout and `decisions.md §22` for the full mechanism (supersedes Decision §8 above).
>
> **Pre-launch checklist (must be true before any cycle fires):**
> 1. `kanban.md` exists at this directory and lists the ready queue (local-file dispatch).

## Book list — operator-pinned CONFIRMED 2026-08-01

Sixteen books in scope. Four books deferred: NPC Codex and Planar Adventures per the 2026-08-01 absent-book rule (genuinely absent from the corpus), Occult Origins and Haunted Heroes Handbook by explicit operator choice 2026-08-01 (present in the corpus under `player_companion/`; see the resolved shape finding below).

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
- **Occult Origins** — **present** in the PCGen corpus at `player_companion/occult_origins` (7 `.lst`, real `.pcc`; verified 2026-08-01). Deferred by explicit operator choice 2026-08-01, NOT by the absent-book rule — the 07-30 "not found" finding was a bad check (see the resolved shape finding below). Recorded in `forward-scope-register.md C2.3` as a future-bundle candidate.
- **Haunted Heroes Handbook** — **present** in the PCGen corpus at `player_companion/haunted_heroes_handbook` (14 `.lst`, real `.pcc`; verified 2026-08-01 — the 07-30 check grepped the bare stem `haunted_heroes` and missed the real directory name). Deferred by explicit operator choice 2026-08-01, NOT by the absent-book rule. Recorded in `forward-scope-register.md C2.4` as a future-bundle candidate.

**Cycle-0 trap-report + work-inventory gating.** Epic 2's pre-flight runs against all 16 in-scope books; for each book, the inventory surfaces the per-book shape (kinds, files_not_enumerated, trap_hits). Per-book cycles dispatch per the shape finding; cycles on the 4 deferred slots are NOT in scope.

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

### Shape finding that affected the 07-30 book list — RESOLVED 2026-08-01

The 07-30 stub reported that "two of the four candidate books have no PCGen
corpus directory" (`occult_origins`, `haunted_heroes`). **That finding was
wrong on both counts, and the operator has since re-cut the bundle to the
sixteen-book list above.** For the record, so the failure mode is not
repeated:

- The 07-30 check enumerated only
  `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/`. Both
  books are player-companion products and live under
  `pathfinder/paizo/player_companion/` — reading #2 of the three readings the
  stub itself listed, written down and then never checked. The claimed
  "verified by both the generator's book enumeration and a plain directory
  listing" was not two independent checks: both walked the same wrong root.
- The check also searched the identifier `haunted_heroes`; the directory is
  `haunted_heroes_handbook`. A grep for a bare stem that returns zero reads,
  wrongly, as absence — the exact trap §"Per-cycle repo tooling" below warns
  about.

Re-derived 2026-08-01: `ls ~/workspace/repos/pcgen/data/pathfinder/paizo/player_companion/`
shows `occult_origins` (7 `.lst`, `_occult_origins.pcc`) and
`haunted_heroes_handbook` (14 `.lst`, `_haunted_heroes_handbook.pcc`). Both
are deferred from SD-30 **by operator choice** (2026-08-01), not by absence.
`npc_codex` and `planar_adventures` remain genuinely absent from the whole
corpus (`find` across all publishers, 2026-08-01) — their deferrals stand on
the absent-book rule.

The former "operator questions to confirm" are all resolved and pinned above:
the bundle is the sixteen-book cut (question 1); epic structure is one epic
per book with cycles per subtype (question 2, `epic-breakdown.md`);
cross-bundle class canon lives in SD-30 for Occultist / Spiritualist /
Medium / Mesmerist (question 3, `decisions.md §5`); monster canon lives in
the first-introducing book (question 4, `decisions.md §6`).

## Scope (RETIRED SECTION — superseded by "Re-scoped `class_feature` scope" below)

- **In scope:** End-to-end content-source ingest for the sixteen books enumerated above. Per-class / per-monster-block / per-discipline cycles produce canonical entries that match the SD-22 corpus-source-inventory doctrine-of-record. **"End-to-end" now includes the player surface** — see §"Ingest and surfacing are one unit of work" below and `decisions.md` Decision 11.
- **Out of scope:** Bestiary 1 (closed in SD-22). Ultimate books (separate bundle, SD-28). Bestiary 2-5 (separate bundle, SD-29). Update-UI bug remediation (lifecycle-routed from SD-16, separate).
- **Boundary with SD-28:** Classes that appear in both Ultimate Intrigue and Occult Adventures (Occultist, Spiritualist, Medium, Mesmerist) live canonically in SD-30. SD-28 references the canonical class id only.

## Re-scoped `class_feature` scope (operator directive 2026-08-10, `decisions.md §33-38`)

**This section is the operative scope statement; the "Book list" and "Scope" sections above are
retired history.**

- **In scope:** `class_feature`, corpus-wide — 15,472 units across 23 books, 40.2% of the 38,536-unit
  corpus, 109 grounded (0.7%). Books: `advanced_class_guide` (2,396), `advanced_players_guide`
  (2,055), `ultimate_psionics` (1,422), `ultimate_combat` (1,412), `ultimate_magic` (1,070),
  `occult_adventures` (979), `core_rulebook` (959), `ultimate_wilderness` (866), `ultimate_intrigue`
  (777), `adventurers_guide` (700), `advanced_race_guide` (645), `pathfinder_unchained` (577),
  `horror_adventures` (419), `inner_sea_combat` (314), `inner_sea_magic` (218),
  `book_of_the_damned_volume_2` (212), `inner_sea_world_guide` (171), `inner_sea_intrigue` (169),
  `monster_codex` (68), `bestiary_6` (18), `inner_sea_taverns` (11), `book_of_the_damned_volume_1`
  (10), `bestiary_4` (4). Re-derive at each cycle-0 per `technical-requirements.md TR-30-012` — this
  table is a snapshot (`decisions.md §33`), not a hand-maintained constant.
- **Also in scope:** the archetype-swap primitive (`archetype_resolver::archetype_claims_slot`) and
  both its wiring shapes, inherited whole from SD-28 `§60`/`§63`/`§64`; the ~~2,958-unit~~
  **superseded 2026-08-13 (`decisions.md §40`): 3,218-unit** `unknown` bucket's per-class
  characterization; the class-grant boundary with SD-28 for Occultist/Spiritualist/Medium/Mesmerist
  (unchanged, `decisions.md §5`).
- **Out of scope:** every other kind, in every book — SD-29's corpus-wide lanes own them
  (`SD-29-corpus-wide-catch-up-lanes/decisions.md §38`). The old sixteen-book list's non-`class_feature`
  content. Epic 14's harness widening (`spell`/`equipment`-shaped, `decisions.md §36`). Bestiary 1
  (closed in SD-22). Update-UI bug remediation (lifecycle-routed from SD-16, separate). SD-32's
  instrument/gate tooling — `corpus_literal_sweep`, the derived-evaluator check, `wiring_class.rs`,
  and every `probe_*` function in `v06_work_inventory.rs` — SD-30 consumes these, does not build or
  modify them (`decisions.md §41-§42`, 2026-08-13).
- **Gating relationship, not just sequencing:** Epic 4 (per-class measurement) must clear a specific
  class before Epic 5 (mechanism) or Epic 6 (chassis sweep) can schedule a cycle against that class —
  `decisions.md §37`. This replaces the old scope's "sixteen books, dispatch in any order post-Epic 2"
  shape entirely.
- **Instrument coverage (2026-08-13, `decisions.md §41`).** Re-derived `wiring_class` split for the
  15,472 `class_feature` units: `display` 7,227, `computed` 4,178, `derived` 1,792, `static` 1,191,
  `ambiguous` 1,084. SD-32 (sibling bundle, landed on `tranche/9` 2026-08-13) built two corpus-wide
  gates that already cover this bundle's future `static` and `derived` shipments for free:
  `scripts/verify.sh`'s `corpus-sweep`/`corpus-sweep-selftest` stages (`corpus_literal_sweep`, all
  shipped records, all kinds) and `tests/derived_evaluator_fixture_check.rs`. Together these cover
  2,983 of 15,472 units once ingested. The `computed` bucket (4,178 units, the largest) has **no**
  dedicated gate — SD-32's spell consumer-delta probe (`probe_spell_effect_wiring`) is a precedent
  for the shape such a probe would take, not a `class_feature` gate itself; none exists in
  `src/bin/v06_work_inventory.rs` today. Building one is flagged for the operator (SD-30 Epic 4/5 vs.
  SD-32), not decided by this doc pass — see `decisions.md §41`'s closing paragraph.
- **`unknown`-bucket residue (2026-08-13, `decisions.md §40`, §38 method unchanged).** Re-derived
  `by_status` split: `not-ingested` 10,203, `not-started` 1,908, `unknown` 3,218, `grounded` 109,
  `deferred-with-reason` 34 — superseding Decision §33's 2026-08-10 snapshot (9,078 / 3,293 / 2,958 /
  109 / 34), moved by a sibling bundle's classifier/generator fixes, not by any `class_feature` work.
  `unknown` means *unmeasurable by any instrument regardless of `wiring_class`* per
  `status_vocabulary` — it is not "not yet worked," it is "could not be classified, `reason` says
  why." Decision §38's method for splitting it (option-pool sub-choice content vs. genuinely-
  unreachable content needing new engine code vs. residual unclustered content) is unchanged and is
  the method Epic 4 must apply at time of use — **this doc pass does not re-run that classification**,
  it only re-confirms the bucket's current size and restates the required method so a future cycle
  does not treat `unknown` as workload without first applying it. The two named non-workable subsets
  Decision §38 already carries forward as backlog (303-unit genuinely-unreachable, 1,772-unit
  unclustered remainder, both at SD-28's last snapshot) are unchanged by this pass; Epic 4 re-derives
  both at time of use per the "generated, never hand-maintained" rule (`decisions.md §12`).

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

## Epic structure

**Re-cut 2026-08-10.** The authoritative epic structure lives in `epic-breakdown.md` and `kanban.md`
(9 cards, dependency-ordered: Epic 1 Identifier Cleanup, Epic 2 Operator Pre-Launch, Epic 3
PI-Screening Provenance Gate, Epic 4 Per-Class Archetype Measurement (gates 5/6 per class), Epic 5
Archetype Mechanism, Epic 6 Per-Class Chassis Sweep, Epic 7 Build Version Numbering (`0.10.<build>`
per `decisions.md §15`), Epic 8 Bundle Code Review (`decisions.md §26`), Epic 9 Closure Epilogue,
fires last). The prior 21-card, sixteen-per-book-epic structure this section described is retired —
`kanban.md`'s "Retired cards" table resolves old references. An earlier revision of this section also
carried the superseded 07-30 four-book epic table with a `0.6.<build>` version target; neither that
nor the 21-card structure is current — do not cite either for epic structure.

## What is operator-pinned vs. doctrine

- **Operator-pinned (CONFIRMED 2026-08-10, supersedes the 2026-08-01 book-list pin):** Scope is
  `class_feature` corpus-wide, 23 books (`decisions.md §33`). Epic structure is the 9-epic
  dependency chain above (`decisions.md §37`). Branch `tranche/10` unchanged. No Hermes board;
  local-file dispatch, unchanged. Build version `0.10.<build>`, unchanged. Cross-book conflict rule,
  unchanged. Reach-gate DoD doctrine, unchanged. The PI-screening provenance gate is new (Epic 3,
  mirrors SD-29 Epic 3).
- **Doctrine-of-record (already established, unaffected by the re-scope):** Epic 1 = Code-Side
  Identifier Cleanup. Operator Pre-Launch gates. Identifier discipline. Build-version scheme
  (`<major>.<tranche-base>.<build>`). `Workflow`-tool operating form (`decisions.md §22`, supersedes
  the prior `/loop /batch /goal` form at §8). Per-bundle progress file. Reach-gate = definition of
  done. Engine policy (real-time forbidden; rules-data in scope only when strictly necessary).

## Next step (operator-pinned CONFIRMED 2026-08-10, supersedes the 2026-08-01 book-list confirmation)

Six items were confirmed 2026-08-01 under the old book-list scope; the re-scope confirms a new set:

1. **Scope confirmed** — `class_feature`, corpus-wide, 23 books, 15,472 units, 40.2% of the corpus,
   109 grounded (`decisions.md §33`). The old sixteen-book list is retired (`decisions.md §35`).
2. **Per-book path locations** — `src/rules_core/rules_tables/<book>/` for each of the 23 books,
   same convention as before, wider set.
3. **Branch name + board name** — unchanged: `tranche/10` branch; Hermes board retired in favor of
   local-file `kanban.md` + `progress.md`.
4. **Build version target** — unchanged: `0.10.<build>` per `<major>.<tranche-base>.<build>` scheme.
5. **Epic/dependency structure confirmed** — measurement (Epic 4) gates mechanism (Epic 5) and
   chassis sweep (Epic 6), per class, not per book (`decisions.md §37`).
6. **Packaging decision** — this source-of-record was renamed `docs/release/SD-30-class-feature-archetype-bundle/`
   (`git mv`, 2026-08-10) in the repo; the move-not-copy publish landed under the old name 2026-08-01
   and is not re-run by the rename.

The bundle remains a planning-ready package, re-scoped rather than restarted. Pre-launch checklist
remains: `kanban.md`, branch `tranche/10` pushed, OAuth valid, working tree clean, cycle-0
trap-report + work-inventory validation re-run against the 23 `class_feature`-bearing books (not the
old sixteen).

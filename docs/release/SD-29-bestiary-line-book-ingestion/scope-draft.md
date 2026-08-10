# SD-29 — Bestiary 2-3-4-5 Content Ingestion

**Status:** Planning-ready (operator directives 2026-08-01; canonical chassis landing this cycle).
**Operator pin:** 2026-08-01 (refines 2026-07-28 stub)
**Branch:** `tranche/9` (operator-pinned 2026-08-01; SD-29 takes its own tranche parallel to SD-28's `tranche/8`, off the `tranche/6` family used by SD-22's Bestiary 1 baseline).
**Board:** Local-file only. The Hermes board is retired per operator directive 2026-08-01 (SD-28's no-Hermes-board amendment propagates to all post-2026-08-01 bundles). The work-queue artifact is `kanban.md` paired with `progress.md` inside this directory. There is no `codex-tranche-9` Hermes board; the slug is reserved-as-form, not as-instance.
**Build version target:** `0.9.<build>` first concrete value. tranche-base = 9 per `<major>.<tranche-base>.<build>` scheme (tranche-base is the base digit of the active working tranche, per the 2026-07-17 build-version amendment). Major stays `0` until first main-publish.
**Owner:** Todd Hintzmann
**Scope:** universal

> **Re-cut 2026-08-10 (`decisions.md §37`, executing §36).** SD-29 dispatches by **kind lane**
> (Epics 4-7: Monster/Monster-Ability chassis, Race-Trait, Companion, Residual proven-path), not
> by per-book epic. The seven-book list below is unchanged — this section still names the correct
> books, publishers, and physical write paths — but the epic/cycle-batch structure it points to is
> `epic-breakdown.md`'s 11-epic lane cut, not the retired 13-epic per-book cut. See
> `epic-breakdown.md` and `decisions.md §37` for the lane sizes and sequencing.

> ⚠️ **OPERATING METHOD — REQUIRED FOR THIS BUNDLE**
> This bundle is operated via the in-harness `Workflow` tool driven from a live session, NOT via `/loop`/`/batch` and NOT via ad-hoc single-task invocations — see `loop-instruction.md`'s OPERATING METHOD callout and `decisions.md §23` for the full mechanism (supersedes Decision §7 above).
>
> **Pre-launch checklist (must be true before any cycle fires):**
> 1. `kanban.md` exists at this directory and lists the ready queue (local-file dispatch).
> 2. Branch `tranche/9` is pushed to origin.
> 3. OAuth credentials are valid for the active harness.
> 4. Working tree is clean (no uncommitted work-in-progress from prior bundles).

## Book list — operator-pinned, confirmed 2026-08-02 (Decision §34)

| Slot | Book | Publisher | Ingest subtype | Path | Corpus dir | Per-entity count |
|------|------|-----------|----------------|------|-----------|------------------|
| 1 | Bestiary 2 | Paizo (hardcover) | Per-monster-block cycles | `src/rules_core/rules_tables/beastiary2/` | `bestiary_2` | derived — see §"Book shape" |
| 2 | Bestiary 3 | Paizo (hardcover) | Per-monster-block cycles | `src/rules_core/rules_tables/beastiary3/` | `bestiary_3` | derived — see §"Book shape" |
| 3 | Bestiary 4 | Paizo (hardcover) | Per-monster-block cycles | `src/rules_core/rules_tables/beastiary4/` | `bestiary_4` | derived — see §"Book shape" |
| 4 | Bestiary 5 | Paizo (hardcover) | **See shape finding below — not a monster book in this corpus** | `src/rules_core/rules_tables/beastiary5/` | `bestiary_5` | derived — see §"Book shape" |
| 5 | Bestiary 6 | Paizo (hardcover) | Per-race-trait / per-class-feature / per-companion cycles (player-options, same shape as Bestiary 5) | `src/rules_core/rules_tables/beastiary6/` | `bestiary_6` | 0 monsters; 63 units total (22 class_feature, 13 race_trait, 2 spell, 26 companion) |
| 6 | Bonus Bestiary | Paizo (softcover) | Per-monster-block cycles (smallest book) | `src/rules_core/rules_tables/bonus_bestiary/` | `bonus_bestiary` | 14 monsters; 34 units total (3 class, 17 race_trait, 14 monster) |
| 7 | Monster Codex | Paizo (softcover) | Per-record-family cycles, **not** per-monster-block | `src/rules_core/rules_tables/monster_codex/` | `monster_codex` | 2 monsters; 213 units total (72 class_feature, 32 feat, 24 spell, 45 equipment, 4 equipment_modifier, 19 race_trait, 15 companion, 2 monster) |

All seven corpus directories exist under
`~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/` (verified
2026-07-30 by directory listing and by `v06_work_inventory`'s book enumeration;
re-verified 2026-08-01/02 for `bestiary_6`, `bonus_bestiary`, and
`monster_codex` per `forward-scope-register.md §1.1/§1.3`). **All seven are
claimed by this bundle** per `decisions.md §34` (operator directive
2026-08-02) — `bestiary_6`, `bonus_bestiary`, and `monster_codex` are no
longer excluded; the "not claimed by this bundle" statement this section
previously carried is superseded.

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

### Shape finding that affects this bundle's cycle plan

**The recorded estimate "~250-300 monsters each; total ~1,000-1,200" does not
survive contact with the generator, and the discrepancy is not evenly spread.**
Read from `docs/work-inventory.json` generated 2026-07-30, and cross-checked
against a directory listing of the corpus:

- **Bestiary 2, 3 and 4 are broadly in the expected band** and their
  per-monster-block cycle plan is sound, though each also carries a large
  `race_trait` population (four figures per book) that the plan does not
  currently name.
- **Bestiary 5 has no monster file at all.** `bestiary_5/` contains
  `b5_races_pc.lst`, `b5_races_companion.lst`, `b5_abilities_race*.lst`,
  `b5_feats.lst`, `b5_companionmods.lst` and a `_bestiary_5_for_players.pcc`
  — it is a **player-options dataset**, not a monster dataset. The generator
  reports zero units of kind `monster` for it. A per-monster-block epic for
  Bestiary 5 would produce zero cycles.
- **Bestiary 6 is the same shape** as Bestiary 5 in this corpus.

**CORRECTED at package consolidation (2026-08-01) — old value → new value:**
this section's own "broadly in the expected band" language for Bestiary 2/3/4
was never given exact numbers here; the sibling package
(`forward-scope-register.md §1.3`, consolidated into this directory) supplies
the verified base `races.lst` row counts, re-derived against the PCGen
checkout on 2026-08-01: bestiary_2 **322**, bestiary_3 **261**, bestiary_4
**220** (803 combined — note 322 and 220 actually fall just outside the
original "~250-300" band, at the high and low ends respectively), bestiary_5
**0** (confirmed, matching this section's own finding above). The register
also confirms `bestiary_6` and a seventh book, `monster_codex`, are outside
this package's four-book scope — see that register and the merged `README.md`
for the scope discrepancy between the two source packages.

**This is flagged, not resolved.** The recorded book list and the recorded
ingest subtype are operator-pinned decisions; whether Bestiary 5 stays in scope
under a *different* ingest subtype (races/companions rather than monster
blocks), or is dropped in favour of Bestiary 6 or Bonus Bestiary, is the
operator's call. Re-derive before deciding — the corpus checkout can move.

**Operator questions to confirm when reviewing on a real computer:**

1. Are all four Bestiary books (2, 3, 4, 5) in scope, or is the bundle a subset (e.g., 2-3 only, or 2-4 only)? Bestiary 5 is the most recent hardcover in the line and is the candidate I want to confirm. **See the shape finding above before answering: Bestiary 5's PCGen dataset carries no monsters.**
2. Per-monster-block cycle pattern: confirm the SD-22 Bestiary 1 ingest pattern translates 1:1 to the larger books. SD-22's Bestiary 1 ingest produced one canonical entry per monster block (CR, stat block, special abilities, ecology notes). Re-derive the per-book monster count from `docs/work-inventory.json` rather than from the estimate this line used to carry.
3. Cross-book monster overlap (e.g., reprints of famous monsters in later bestiaries): canonical class definition lives in whichever book first introduces the monster (typically the lowest-numbered Bestiary). Later bestiaries reference the canonical id.
4. The DM toolkit (`src/rules_core/encounters.rs` + `src/rules_core/party_cr.rs`) consumes Bestiary 1 from SD-22 Epic 6. After SD-29 lands, the DM toolkit needs an extension epic to consume Bestiary 2-5 as well. Should that extension be in scope for SD-29 or split into a separate bundle?

## Scope

- **In scope:** End-to-end content-source ingest for the seven Bestiary-line books enumerated above (per `decisions.md §34`). Per-monster-block cycles produce canonical monster entries that match the SD-22 corpus-source-inventory doctrine-of-record for the monster-bearing books; Bestiary 5 and Bestiary 6 use player-options cycles, and Monster Codex uses per-record-family cycles — see §"Book shape" and §"Epic structure" below. **"End-to-end" now includes the player surface** — see §"Ingest and surfacing are one unit of work" below and `decisions.md` Decision 10.
- **Out of scope:** Bestiary 1 (closed in SD-22 procedurally; **4.1% proven, 42/1,027 units**, 41/326 declared monsters ingested — measured 2026-08-02, `decisions.md §35`). Mythic monster appendices (separate treatment). NPC codex (separate, not in any current SD). Update-UI bug remediation (lifecycle-routed from SD-16, separate).
- **Boundary with SD-22:** Bestiary 1 lives canonically in SD-22. SD-29 references Bestiary 1's canonical id only; does not redefine.

## Ingest and surfacing are one unit of work

Content that is ingested but reaches no player surface is the dominant defect
of this project — six instances in a single session, each found by accident and
patched individually while the next appeared.
`apps/desktop/src-tauri/src/reach_gate.rs` now makes it structural: it builds
its inventory from the app's live ingest diagnostic **and** from a filesystem
scan of every `pub const <NAME>: &[<RecordType>]` slice under
`src/rules_core/rules_tables/`. Because SD-29's ingest target **is**
`src/rules_core/rules_tables/beastiary<N>/`, the gate applies directly to every
cycle in this bundle.

**This bundle was the one most exposed by that gate, and the exposure was
recorded, not hypothetical.** From `reach_gate.rs`'s `OPEN_FINDINGS`, as it
read prior to 2026-08-01 (historical record, preserved below — see the
supersession note that follows it):

> Bestiary 1's 41 ingested monster stat blocks reach no surface. The only
> consumers are `corpus_ingest_diagnostic` (a count) and `cache_gen::beastiary1`
> (a build-time JSON generator); the React app contains no monster reference at
> all. The Pets tab does NOT count — its companion stat block is computed by
> `pilot_compute`'s own `ground_*_companion_stat_block`, not read from these
> tables. Remedy: a monster catalog command and browser, mirroring
> `spell_catalog.rs` + SpellCatalogScreen.tsx.

Bestiary 1's four ingested equipment records were in the same list, for the
same reason.

**Superseded, 2026-08-01.** The monster catalog command and browser this
finding called for have shipped: `reach_gate.rs:840` now carries an executed
reach claim for `("beastiary1", "monsters")` in place of the old
`OPEN_FINDINGS` entry (comment at `:836` records the replacement);
`apps/desktop/src-tauri/src/monster_catalog.rs`'s `list_monster_catalog`
command is registered (`main.rs:57,197`); `MonsterCatalogScreen.tsx` is
routed via `CharacterHubPage.tsx:104-105`, reachable from a "Browse Monster
Catalog" button at `LandingScreen.tsx:353`. **Monster records now reach a
player.** The gate's sole surviving finding is unrelated to the quote above:
`beastiary1/race_traits` — the Duergar `Spell-Like Ability ~ Invisibility`
record, upstream-blocked on `monster_codex` (`forward-scope-register.md
§1.2`). `monster_codex` is in scope for this bundle per `decisions.md §34`,
and Epic 5's Monster Codex cycle-batch (Race-Trait Lane; was Epic 13 under the
retired per-book numbering, `epic-breakdown.md`) is expected to retire that finding as part
of its bounded work.

**The "open operator question" below is resolved and preserved as historical
record.** SD-29's epic structure at the time contained no surface-building
epic; the operator resolved this by the monster catalog shipping outside a
dedicated SD-29 epic (above), which independently satisfies the gate's
Bestiary-1-monster-surface prerequisite regardless of whether Epic 8 (DM
Toolkit extension; was Epic 7) lands in-bundle. Original text: SD-29's epic structure
contains no surface-building epic. Either the monster catalog command and
browser land inside SD-29, or they are a named prerequisite outside it —
possibly folded into the proposed Epic 8 (DM Toolkit extension; was Epic 7), which is
the nearest existing consumer of monster data. **The operator picks; this
package does not add an epic on its own authority.** Skipping it is not
available.

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

Note for this bundle specifically: the cross-book monster-overlap rule
(Decision 5 — canonical definition lives in the lowest-numbered Bestiary) is a
join on record **identity**, and the corpus trap catalogue is explicit that a
shared display name never implies a shared record. Join on `KEY:`; the trap
report's per-book `KEY:` namespace listing is what tells you the right prefix
to search under.

## Epic structure — SUPERSEDED, see `epic-breakdown.md`

The 13-epic per-book table this section previously pointed to is itself superseded, 2026-08-10
(`decisions.md §37`, executing §36): SD-29 dispatches by **kind lane**, not by book. Do not maintain
a duplicate epic table here; `epic-breakdown.md` is the canonical source. Summary: Epic 1
(Identifier Cleanup, fires first) → Epic 2 (Operator Pre-Launch, corpus-wide gating) → Epic 3
(Provenance Gate — PI-screening wired into each lane, blocking per `../corpus-work-channels.md §6`)
→ Epics 4-7 (kind lanes: Monster/Monster-Ability chassis [2,159 units, pilot-then-extend], Race-Trait
[1,124 units, defect-fix-alongside], Companion [275 units, new mechanism], Residual proven-path
content [203 units — spell/equipment/feat/race/equipment_modifier/class; `class_feature`'s 90 units
excluded, Channel D, see `decisions.md §37.4`]) → Epic 8 (DM Toolkit extension, gated on Epic 4's
pilot) → Epic 9 (Build Version Numbering, after Epic 1, before Epic 11) → Epic 10 (Bundle Code
Review, after Epic 9 and Epics 4-7, plus Epic 8 if in scope, before Epic 11) → Epic 11 (Closure
Epilogue, fires last). See `epic-breakdown.md`'s "Recommended sequencing" for the full dependency
diagram.

**Acceptance criteria:** ~40 criteria across 11 epics (~3-4 per epic) —
see `epic-breakdown.md`. The ~46-criteria/13-epic figure this section
previously carried was against the retired per-book cut and is retired along with it.

## What is operator-pinned vs. doctrine

- **Operator-pinned, confirmed 2026-08-01/02, re-cut 2026-08-10:** Book list (7 books, per `decisions.md §34`), kind-lane structure and lane sizes (`decisions.md §37`), Epic 8 in-scope vs. separate-bundle decision, branch name, board name, build version target. See `decisions.md §§13, 14, 15, 34, 37` and the "Next step" section below.
- **Doctrine-of-record (already established):** Epic 1 = Code-Side Identifier Cleanup. Operator Pre-Launch gates. Identifier discipline. Build-version scheme. `Workflow`-tool operating form (`decisions.md §23`, supersedes the prior `/loop /batch /goal` form at §7). Per-bundle progress file.

## Next step (operator-pinned, confirmed 2026-08-01/02)

All operator-pinned items are now confirmed:

1. **Book list confirmed** — seven books (Bestiary 2, 3, 4, 5, 6, Bonus Bestiary, Monster Codex), 2026-08-02 per `decisions.md §34`.
2. **Per-book path locations confirmed** — `src/rules_core/rules_tables/<book>/` for each of the seven books, per the §"Book list" table above.
3. **Epic 8 (DM Toolkit extension; was Epic 7)** remains operator-pinned whether in scope, per-cycle at Epic 4's pilot cycle-batch closure (`decisions.md §19`, `§37`); its safe default absent an explicit call is the Class 3 (C3.1) retrofit per `successor-forward-scope-register.md C3.1`.
4. **Branch name + board name confirmed** — `tranche/9` branch (cut from the post-SD-28 tip per `decisions.md §34`); Hermes board retired in favor of local-file `kanban.md` + `progress.md`, 2026-08-01 per `decisions.md §13`/§14a.
5. **Build version target confirmed** — `0.9.<build>` per `<major>.<tranche-base>.<build>` scheme, 2026-08-01 per `decisions.md §14`.
6. **Packaging decision confirmed** — the canonical repo-resident home is `docs/release/SD-29-bestiary-line-book-ingestion/`, with the chassis planning-ready at seven-book width per `decisions.md §34`.

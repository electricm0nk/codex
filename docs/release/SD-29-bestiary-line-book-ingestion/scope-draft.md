# SD-29 — Bestiary 2-3-4-5 Content Ingestion

**Status:** Planning-ready (operator directives 2026-08-01; canonical chassis landing this cycle).
**Operator pin:** 2026-08-01 (refines 2026-07-28 stub)
**Branch:** `tranche/9` (operator-pinned 2026-08-01; SD-29 takes its own tranche parallel to SD-28's `tranche/8`, off the `tranche/6` family used by SD-22's Bestiary 1 baseline).
**Board:** Local-file only. The Hermes board is retired per operator directive 2026-08-01 (SD-28's no-Hermes-board amendment propagates to all post-2026-08-01 bundles). The work-queue artifact is `kanban.md` paired with `progress.md` inside this directory. There is no `codex-tranche-9` Hermes board; the slug is reserved-as-form, not as-instance.
**Build version target:** `0.9.<build>` first concrete value. tranche-base = 9 per `<major>.<tranche-base>.<build>` scheme (tranche-base is the base digit of the active working tranche, per the 2026-07-17 build-version amendment). Major stays `0` until first main-publish.
**Owner:** Todd Hintzmann
**Scope:** universal

> ⚠️ **OPERATING METHOD — REQUIRED FOR THIS BUNDLE**
> This bundle is operated via the in-harness `Workflow` tool driven from a live session, NOT via `/loop`/`/batch` and NOT via ad-hoc single-task invocations — see `loop-instruction.md`'s OPERATING METHOD callout and `decisions.md §23` for the full mechanism (supersedes Decision §7 above).
>
> **Pre-launch checklist (must be true before any cycle fires):**
> 1. `kanban.md` exists at this directory and lists the ready queue (local-file dispatch).
> 2. Branch `tranche/6-1` is pushed to origin.
> 3. OAuth credentials are valid for the active harness.
> 4. Working tree is clean (no uncommitted work-in-progress from prior bundles).

## Book list — operator-pinned pending

| Slot | Book | Publisher | Ingest subtype | Path | Corpus dir | Per-entity count |
|------|------|-----------|----------------|------|-----------|------------------|
| 1 | Bestiary 2 | Paizo (hardcover) | Per-monster-block cycles | `src/rules_core/rules_tables/beastiary2/` | `bestiary_2` | derived — see §"Book shape" |
| 2 | Bestiary 3 | Paizo (hardcover) | Per-monster-block cycles | `src/rules_core/rules_tables/beastiary3/` | `bestiary_3` | derived — see §"Book shape" |
| 3 | Bestiary 4 | Paizo (hardcover) | Per-monster-block cycles | `src/rules_core/rules_tables/beastiary4/` | `bestiary_4` | derived — see §"Book shape" |
| 4 | Bestiary 5 | Paizo (hardcover) | **See shape finding below — not a monster book in this corpus** | `src/rules_core/rules_tables/beastiary5/` | `bestiary_5` | derived — see §"Book shape" |

All four corpus directories exist under
`~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/` (verified
2026-07-30 by directory listing and by `v06_work_inventory`'s book enumeration).
`bestiary_6` and `bonus_bestiary` also exist and are not claimed by this bundle
or by SD-28 / SD-30; SD-27 `decisions.md §9` lists both as Tier-1 deferrals.

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

- **In scope:** End-to-end content-source ingest for the four Bestiary books enumerated above. Per-monster-block cycles produce canonical monster entries that match the SD-22 corpus-source-inventory doctrine-of-record. **"End-to-end" now includes the player surface** — see §"Ingest and surfacing are one unit of work" below and `decisions.md` Decision 10.
- **Out of scope:** Bestiary 1 (closed in SD-22). Mythic monster appendices (separate treatment). NPC codex (separate, not in any current SD). Update-UI bug remediation (lifecycle-routed from SD-16, separate).
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

**This bundle is the one most exposed by that gate, and the exposure is
recorded, not hypothetical.** From `reach_gate.rs`'s `OPEN_FINDINGS`:

> Bestiary 1's 41 ingested monster stat blocks reach no surface. The only
> consumers are `corpus_ingest_diagnostic` (a count) and `cache_gen::beastiary1`
> (a build-time JSON generator); the React app contains no monster reference at
> all. The Pets tab does NOT count — its companion stat block is computed by
> `pilot_compute`'s own `ground_*_companion_stat_block`, not read from these
> tables. Remedy: a monster catalog command and browser, mirroring
> `spell_catalog.rs` + SpellCatalogScreen.tsx.

Bestiary 1's four ingested equipment records are in the same list, for the same
reason.

**So: no monster record in this codebase reaches a player today.** Ingesting
three more bestiaries without the monster surface reproduces the defect at
several times the scale, and the gate will fail the cycles rather than let it
happen quietly.

**Open operator question this package cannot decide for itself.** SD-29's epic
structure contains no surface-building epic. Either the monster catalog command
and browser land inside SD-29, or they are a named prerequisite outside it —
possibly folded into the proposed Epic 7 (DM Toolkit extension), which is the
nearest existing consumer of monster data. **The operator picks; this package
does not add an epic on its own authority.** Skipping it is not available.

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

## Epic structure (proposed)

| Epic | Title | Fires | Notes |
|------|-------|-------|-------|
| 1 | Code-Side Identifier Cleanup | FIRST | Governance base requirement. Per SD-22 Epic 1 pattern. |
| 2 | Operator Pre-Launch | Gating | Pre-launch checklist verification. |
| 3 | Bestiary 2 content-source ingest | After Epic 2 | Per-monster-block cycles. |
| 4 | Bestiary 3 content-source ingest | After Epic 2 | Per-monster-block cycles. |
| 5 | Bestiary 4 content-source ingest | After Epic 2 | Per-monster-block cycles. |
| 6 | Bestiary 5 content-source ingest | After Epic 2 | Per-monster-block cycles. |
| 7 | DM Toolkit extension (consume Bestiary 2-5) | After Epics 3-6 | Optional-but-proposed. Operator-pinned whether in scope. |
| 8 | Closure Epilogue | LAST | Tranche promotion version increment. |
| 9 | Build Version Numbering | After Epic 1, before Epic 8 | First concrete value `0.6.<build>`. |

**Acceptance criteria stub:** 30 criteria, 9 epics (matches SD-22 shape; +1 over SD-22 because the optional Epic 7 is proposed). Per-criterion detail deferred until book list is operator-pinned.

## What is operator-pinned vs. doctrine

- **Operator-pinned (NOT yet confirmed):** Book list (4 books), epics 3-6 per-book paths, per-book entity count, Epic 7 in-scope vs. separate-bundle decision, branch name, board name, build version target.
- **Doctrine-of-record (already established):** Epic 1 = Code-Side Identifier Cleanup. Operator Pre-Launch gates. Identifier discipline. Build-version scheme. `Workflow`-tool operating form (`decisions.md §23`, supersedes the prior `/loop /batch /goal` form at §7). Per-bundle progress file.

## Next step

When the operator is back at a real computer, this stub needs:

1. Operator confirms the book list (4 books).
2. Operator confirms the per-book path locations.
3. Operator decides whether Epic 7 (DM Toolkit extension to consume Bestiary 2-5) is in scope.
4. Operator confirms the branch name + board name.
5. Operator confirms the build version target.
6. Operator decides whether the package promotes to `docs/release/SD-29-...-.../` in the repo now (planning-ready publish) or waits for first-cycle launch.

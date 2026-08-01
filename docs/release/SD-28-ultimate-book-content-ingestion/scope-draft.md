# SD-28 — Ultimate Book Content Ingestion

**Status:** Planning-ready (operator directives 2026-08-01; canonical chassis landing this cycle).
**Operator pin:** 2026-08-01 (refines 2026-07-28 stub)
**Branch:** `tranche/8` (operator-pinned 2026-08-01; off the `tranche/6` family so SD-29 keeps `tranche/6-1` and SD-30 keeps `tranche/6-2`).
**Board:** Local-file only. The Hermes board is retired per operator directive 2026-08-01. The work-queue artifact is `kanban.md` paired with `progress.md` inside this directory. There is no `codex-tranche-8` Hermes board; the slug is reserved-as-form, not as-instance.
**Build version target:** `0.8.<build>` first concrete value. tranche-base = 8 per `<major>.<tranche-base>.<build>` scheme (tranche-base is the base digit of the active working tranche). Major stays `0` until first main-publish.
**Owner:** Todd Hintzmann
**Scope:** universal

> ⚠️ **OPERATING METHOD — REQUIRED FOR THIS BUNDLE**
> This bundle is operated via the in-harness `Workflow` tool driven from a live session, NOT via `/loop`/`/batch` and NOT via ad-hoc single-task invocations — see `loop-instruction.md`'s OPERATING METHOD callout and `decisions.md §22` for the full mechanism (supersedes Decision §7 above).
>
> **Pre-launch checklist (must be true before any cycle fires):**
> 1. `kanban.md` exists at this directory and lists the ready queue (local-file dispatch).
> 2. Branch `tranche/8` is pushed to origin.
> 3. Local OAuth credentials are valid for the active harness (for `git push` to origin).
> 4. Working tree is clean (no uncommitted work-in-progress from prior bundles).
> 5. The Dreamscarred Press licensing pre-cycle verification per `decisions.md §17` has been run against `dreamscarred_press/ultimate_psionics/`.

## Book list — operator-pinned CONFIRMED 2026-08-01

Seven books in scope (six Paizo hardcover + one Dreamscarred Press hardcover):

| Slot | Book | Publisher | Ingest subtype | Path | Corpus dir | Per-entity count |
|------|------|-----------|----------------|------|-----------|------------------|
| 1 | Ultimate Combat | Paizo (hardcover, 2011-08-01) | Combat maneuvers + new classes + martial rules | `src/rules_core/rules_tables/ultimate_combat/` | `ultimate_combat` ✅ | derived — see §"Book shape" |
| 2 | Ultimate Magic | Paizo (hardcover, 2011) | Spell subsystems + new classes + casting variants | `src/rules_core/rules_tables/ultimate_magic/` | `ultimate_magic` ✅ | derived — see §"Book shape" |
| 3 | Ultimate Equipment | Paizo (hardcover, 2012) | Equipment catalog + crafting rules | `src/rules_core/rules_tables/ultimate_equipment/` | `ultimate_equipment` ✅ | derived — see §"Book shape" |
| 4 | Ultimate Intrigue | Paizo (hardcover, date TBD) | Social combat + intrigue subsystems + new classes | `src/rules_core/rules_tables/ultimate_intrigue/` | `ultimate_intrigue` ✅ | derived — see §"Book shape" |
| 5 | Ultimate Campaign | Paizo (hardcover, 2013) | Player-options subsystems (downtime, kingdom-building, traits, retraining) — class-feature-dominant chooser-shaped content | `src/rules_core/rules_tables/ultimate_campaign/` | `ultimate_campaign` ✅ | derived — see §"Book shape" |
| 6 | Ultimate Wilderness | Paizo (hardcover, date TBD) | Per-class + per-Companion-rules cycles | `src/rules_core/rules_tables/ultimate_wilderness/` | `ultimate_wilderness` ✅ | derived — see §"Book shape" |
| 7 | Ultimate Psionics | Dreamscarred Press (third-party, 2014; PSPF/OGL-compatible per `decisions.md §17`) | Per-class + per-power cycles; tier-cross license gate at cycle 0 | `src/rules_core/rules_tables/ultimate_psionics/` | `dreamscarred_press/ultimate_psionics` ✅ | derived — see §"Book shape" |

All seven corpus directories exist:

- Six Paizo dirs under `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/` (verified 2026-07-30 by directory listing and by `v06_work_inventory`'s book enumeration).
- One Dreamscarred Press dir at `~/workspace/repos/pcgen/data/pathfinder/dreamscarred_press/ultimate_psionics/` (verified 2026-08-01; pre-cycle licensing verification per `decisions.md §17` confirms the third-party tier is in-bounds).

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

**Shape note relevant to this bundle's cycle plan (2026-07-30 generation;
re-derive before use):** `ultimate_equipment` is overwhelmingly one kind —
equipment plus equipment modifiers, with a single spell record and nothing
else — so its "per-equipment-entry cycles" plan is well matched. `ultimate_combat`,
`ultimate_magic` and `ultimate_intrigue` are class-feature-dominant, which is
chooser-shaped content; see the playbook §7.5 on canonical narrowing before
committing to per-class cycles that attempt a whole option family.

**Operator questions to confirm when reviewing on a real computer:**

1. Are Ultimate Combat + Ultimate Magic + Ultimate Equipment + Ultimate Intrigue the four books in scope, or is the bundle a different cut of the Ultimate line (e.g., drop something, add Ultimate Wilderness)? For reference: `ultimate_wilderness` and `ultimate_campaign` both exist as corpus directories and both currently sit at `not-started` in the generated inventory; neither is claimed by SD-29 or SD-30.
2. Per-book ingest pattern is "per-class cycles" for Ultimate Combat (gunslinger, ninja, samurai, etc.), Ultimate Magic (new casting variants), Ultimate Intrigue (class options like the vigilante, the medium, the occultist if not in SD-30); "per-monster-block cycles" if any of the Ultimate books have monster appendices; "per-equipment-entry cycles" for Ultimate Equipment. Confirm the cycle pattern per book.
3. Cross-book class overlap (e.g., the Occultist appears in both Ultimate Intrigue and Occult Adventures): how should the SD-28 / SD-30 boundary handle classes that appear in both? Doctrine proposal: the class grant + level-up mechanics live in whichever bundle owns the book's full class definition, and the other bundle only references the canonical class id.

## Scope

- **In scope:** End-to-end content-source ingest for the four Ultimate books enumerated above. Per-book ingest cycles produce per-class / per-monster-block / per-equipment-entry artifacts that match the SD-22 corpus-source-inventory doctrine-of-record (one canonical type per book, one ingest path per type, one cycle per entity). **"End-to-end" now includes the player surface** — see §"Ingest and surfacing are one unit of work" below and `decisions.md` Decision 10.
- **Out of scope:** SD-17 PCGen LST parser work (separate, on `codex-tranche-2-7`). SD-22 closure-epilogue work (already closed). SD-29 bestiary (separate bundle). SD-30 occult (separate bundle). Code-side identifier cleanup is in scope (Epic 1 governance base requirement) but only for new code this bundle introduces, not for retroactive renames in adjacent bundles.
- **Boundary with SD-30:** Classes that appear in both Ultimate Intrigue and Occult Adventures (Occultist, Spiritualist, Medium, Mesmerist) live canonically in SD-30 (Occult Adventures is the primary hardcover defining those classes). SD-28 may reference those class ids but does not redefine them.

## Ingest and surfacing are one unit of work

Content that is ingested but reaches no player surface is the dominant defect
of this project — six instances in a single session, each found by accident and
patched individually while the next appeared. `apps/desktop/src-tauri/src/reach_gate.rs`
now makes it structural: it builds its inventory from the app's live ingest
diagnostic **and** from a filesystem scan of every `pub const <NAME>: &[<RecordType>]`
slice under `src/rules_core/rules_tables/`. A new book has to defeat both to
slip through, and an unrecognized record type is a hard failure by design.

Because SD-28's ingest target **is** `src/rules_core/rules_tables/<book>/`, the
gate applies directly to every cycle in this bundle. A cycle that lands a
book's records without a reach claim fails.

**Prerequisite the operator should be aware of, flagged not assumed.** The
equipment catalog (`apps/desktop/src-tauri/src/equipment_catalog.rs`) reads the
CRB table alone, so APG's and ACG's already-ingested equipment records reach no
surface today; both are pinned in the gate's `OPEN_FINDINGS` with the remedy
("widen `build_equipment_catalog` across all books and tag each DTO with its
book, exactly the way `spell_catalog.rs` and `feat_catalog.rs` were already
widened"). Ultimate Equipment is the largest equipment book in the corpus.
**Whether that widening lands inside SD-28 or as a prerequisite outside it is
an operator decision this package does not make** — but it cannot simply be
skipped, because the gate will fail the cycle either way.

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

## Epic structure (proposed)

| Epic | Title | Fires | Notes |
|------|-------|-------|-------|
| 1 | Code-Side Identifier Cleanup | FIRST | Governance base requirement. Per SD-22 Epic 1 pattern. |
| 2 | Operator Pre-Launch | Gating | Pre-launch checklist verification. |
| 3 | Ultimate Combat content-source ingest | After Epic 2 | Per-class cycles. |
| 4 | Ultimate Magic content-source ingest | After Epic 2 | Per-class + per-spell-subsystem cycles. |
| 5 | Ultimate Equipment content-source ingest | After Epic 2 | Per-equipment-entry cycles. |
| 6 | Ultimate Intrigue content-source ingest | After Epic 2 | Per-class + per-social-rule cycles. |
| 7 | Closure Epilogue | LAST | Tranche promotion version increment (`0.6.<build>` → `0.7.0` if SD-29 ships on `tranche/6-1`; otherwise `0.6.<last_build>`). |
| 8 | Build Version Numbering | After Epic 1, before Epic 7 | First concrete value `0.6.<build>`. |

**Acceptance criteria stub:** 30 criteria, 8 epics (matches SD-22 shape). Per-criterion detail deferred until book list is operator-pinned.

## What is operator-pinned vs. doctrine

- **Operator-pinned (NOT yet confirmed):** Book list (4 books), per-book path locations, per-book ingest subtype, epic count, criterion count, branch name, board name, build version target.
- **Doctrine-of-record (already established):** Epic 1 = Code-Side Identifier Cleanup. Operator Pre-Launch gates. Identifier discipline (PascalCase / camelCase, no `sd<N>_*` patterns). Build-version scheme `<major>.<tranche-base>.<build>` (major stays at 0 until first main-publish; tranche-base is the active working tranche's base digit; build is monotonic never-resets). `Workflow`-tool operating form (`decisions.md §22`, supersedes the prior `/loop /batch /goal` form at §7). Per-bundle progress file at `~/workspace/programs/codex/requirements/SD-28-ultimate-book-content-ingestion/progress.md`.

## Next step (operator-pinned CONFIRMED 2026-08-01)

All five operator-pinned items are now confirmed:

1. **Book list confirmed** — seven books (six Paizo + one Dreamscarred Press), per the §"Book list" table above.
2. **Per-book path locations confirmed** — `src/rules_core/rules_tables/<book>/` for each of the seven books.
3. **Branch name + board name** — `tranche/8` branch; Hermes board retired in favor of local-file `kanban.md` + `progress.md`.
4. **Build version target** — `0.8.<build>` per `<major>.<tranche-base>.<build>` scheme.
5. **Packaging decision** — promote this source-of-record to `docs/release/SD-28-ultimate-book-content-ingestion/` in the repo as a planning-ready publication, with the canonical chassis (12 files) landing in this cycle.

The bundle is no longer a stub; it is a planning-ready package. Pre-launch checklist remains: `kanban.md`, branch `tranche/8` pushed, OAuth valid, working tree clean, and the §17 licensing pre-cycle verification (Dreamscarred Press) run.

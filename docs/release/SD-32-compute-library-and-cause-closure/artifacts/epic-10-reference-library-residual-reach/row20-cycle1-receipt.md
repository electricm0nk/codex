# Cycle row20-cycle1 — Epic 10 (`epic-10-reference-library-residual-reach`) / Row 20

- **Card ID:** `epic-10-reference-library-residual-reach`
- **Files touched:**
  - `src/rules_core/spell_resolver.rs` (11 new books chained into `spell_catalog_rows()`)
  - `src/bin/v06_work_inventory.rs` (`spell_book_slug_for` widened with the 11 new
    `SPELL_BOOK_*` codes — this closed-set lookup panics on an unmapped code, and 11 new codes
    flowing through `spell_catalog_rows()` would have panicked this binary at runtime; also fixed
    a real latent double-count bug in `the_probe_examines_every_catalog_spell_key_of_every_
    observable_book` that item (a)'s new `beastiary1`/`"bestiary_1"` content exposed — see
    "Root-workspace sweep" below)
  - `apps/desktop/src-tauri/src/reach_gate.rs` (25 new dispatch arms for spells/feats/equipment;
    `BARE_RECORD_FINDINGS`/`UNREACHED_RECORD_FINDINGS`/`OPEN_FINDINGS` corrected — 25 gap families
    closed, 6 companion `§24` citations corrected, 25 of 30 delta-row companions closed, 18-family
    class/class_features residual re-stated unchanged)
  - `apps/desktop/src-tauri/src/spell_catalog.rs` (DTO-layer book registry widened — 11 new
    `BOOK_*` constants, 11 new `map_*_entry` helpers, both tests updated with re-derived counts)
  - `apps/desktop/src-tauri/src/reference_library_catalog.rs` (`mechanical_summary` made
    `pub(crate)` for reuse, no behavior change)
  - `apps/desktop/src-tauri/src/companion_pool_catalog.rs` (new `origin == "copy"` tier-3
    admission, 2 new unit tests)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 HEAD --
  <touched files> | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`, per §6's own
  guidance that the full `BASE_BRANCH...HEAD` form returns thousands of pre-existing tagged lines).
- **Wired-integration audit result:** `OK_NO_TOKENS`.
- **PI scrub:** `pi_scrub.normalized_term_hits()` on the full diff — zero hits.
- **Acceptance criterion:** kanban row 20 — close or precisely size all three items row 19 named
  (the 43-family gap's 25 cheapest families, the companion formula residual's stale citations, the
  30 delta-row companions), set `complete` only when all three are closed or sized with evidence,
  and re-run the desktop workspace to confirm ≥536/0.
- **Corpus SHA:** oracle bootstrapped fresh this cycle (`scripts/fetch-pcgen-oracle.sh --dest`,
  confirmed populated, `pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6`).
- **Status:** `in-progress` (NOT `complete` — 18-family class-chassis/`class_features` residual and
  4-record companion delta-merge residual remain real, sized, unbuilt work).

## Starting state (verified, not assumed)

Worktree started on a stale lineage: `git rev-parse HEAD` was `1bb523773d` (the SD-31 PR #374 merge
commit), far behind `$PIN`. `git merge-base --is-ancestor $PIN HEAD` failed. Recovered via
`git reset --hard $PIN` + `git rebase origin/tranche/12` (reported "up to date" — `HEAD` already
equalled `origin/tranche/12` after the reset, i.e. cycle-4's/row-20's-own doc commit). Re-verified
(`BASE_OK`) before any edit. Oracle slot was empty (git-ignored, fresh worktree); bootstrapped via
`scripts/fetch-pcgen-oracle.sh --dest`, confirmed populated before trusting any figure. Checked
`git log origin/tranche/12` for recent activity in target files at the start — no sibling-lane
commits touched `reach_gate.rs`/`companion_pool_catalog.rs`/`pilot_compute` since row 19 cycle 4's
own commit, so no territory conflict at start.

## §17a re-derivation: the brief's figures, checked before use

- **The 43-family gap**: re-counted directly from `reach_gate.rs`'s own `OPEN_FINDINGS` array
  (`awk` over the exact line range, `grep -oE '"(classes|spells|feats|equipment|class_features)"'`
  `| sort | uniq -c`) — confirmed exactly 43 (classes 17, spells 11, feats 11, equipment 3,
  class_features 1), matching the brief.
- **The companion residual (~260 of 330)**: not independently re-counted as a bare total this
  cycle (per `§12c`, no bare total is quoted below without its population and command); instead
  every `%`-carrying record in the largest single residual book (APG, 14 records) was read directly
  against its `raw_tokens` — see "Item (b)" below.
- **The 30 delta-row companions**: re-derived corpus-wide via
  `python3 -c "... origin=='copy' ..."` over every `data/corpus/*/companion/*.json` — found **25**
  real `.COPY=` records, not the 30 named (22 `beastiary1` + 2 `bestiary_4` matches the brief's
  30 minus the 6 that are actually `.MOD` rows or other shapes per row 19 cycle 1's own breakdown;
  **plus 1 more in `ultimate_wilderness`** — "Margay ~ Sound Mimicry" — never individually named in
  any prior receipt's count but genuinely the same shape, found live by this cycle's own
  corpus-wide scan rather than trusting the inherited per-book breakdown).

## Item (a): the 43-family gap — 25 of 43 closed, cheapest-first per the brief's own ordering

Re-derivation found the "cheapest" 25 (all 11 `spells`, all 11 `feats`, all 3 `equipment`) were
**already fully tooled and just never wired**:

1. **Spells.** Every one of the 11 gap books' `spell_list.rs` tables already existed, generated by
   `src/bin/ingest_spells.rs`'s config-driven `BOOKS` list (`bestiary_4`, `book_of_the_damned_volume_1/2`,
   `inner_sea_intrigue/races/world_guide`, `monster_codex`, `mythic_adventures`, `ultimate_equipment`,
   `ultimate_magic_wordsofpower`). `beastiary1`'s spell content lives in a SEPARATELY-named module,
   `rules_tables::bestiary` (not `beastiary1`) — direct inspection of every one of
   `data/corpus/bestiary/spell/*.json`'s 111 records' `source.path` confirms all 111 transcribe from
   `pathfinder/paizo/roleplaying_game/core_essentials/ce_spells.lst`, i.e. this is Core Essentials
   content filed under the Bestiary corpus directory (the same `beastiary1`-maps-to-`bestiary`
   shared-library-host shape `decisions.md §9`/`cache_gen::equipment_gap::book_routing` already
   document for this book's equipment rows). Chained all 11 into `spell_resolver::spell_catalog_rows()`
   and added 11 new `reach_gate.rs` dispatch arms.
2. **Feats.** Every one of the 11 gap books already had an empty slot in
   `feats_all::hand_authored_feat_tables()` (`RuleSetId::Ce`/`Ha`/`Isr`/`Oa`/`Iswg`/`MonsterCodex`/
   `Mythic`/`Isi`/`Botd2`/`Isc`/`Isg`) specifically so `feat_gap_tables::feat_gap_rows_for` could
   join corpus rows onto them — `all_feat_tables()` already served every one of these books' real
   feat rows; only the `reach_gate.rs` dispatch arm was missing. Added 11 arms
   (`feats_reach(RuleSetId::X, "X")`, the exact `("inner_sea_taverns", "feats")` shape already
   proven).
3. **Equipment.** All 3 gap books (`adventurers_guide`/`inner_sea_magic`/`inner_sea_temples`) were
   already registered in `gen_equipment_gap_tables.rs`'s `BOOK_INPUTS` with real `EQUIPMENT_BOOK_*`
   codes and their rows already live in the generated `equipment_gap_tables.rs` (`grep -c '"AG"\|
   "ISM"\|"ISTEM"'` → 227 hits) — again, only the `reach_gate.rs` dispatch arm was missing. Added 3
   arms (`equipment_reach("AG"/"ISM"/"ISTEM", BTreeSet::new())`, the exact
   `("ultimate_wilderness", "equipment")` shape already proven for a book with no hand-authored
   table).

**Second registry found and fixed**: `spell_catalog.rs` (the desktop DTO layer) keeps its OWN
`BOOK_*` constant list and `mapping_helpers_agree_with_the_registry`/
`the_catalog_serves_every_ingested_book_not_only_crb` tests, independent of `spell_resolver.rs`'s
own list — exactly the "two lists drift" shape this module's own doc comment already warns about
for the pre-existing five books. Widened both; `mapping_helpers_agree_with_the_registry`'s hand-
reconstructed `expected` chain now applies the SAME global first-key-wins dedup pass
`spell_catalog_rows()`'s production code uses, rather than hand-listing every duplicate key —
found via live test failure, not assumed: `bestiary_4` restates two keys twice each and
`inner_sea_races` restates "Elemental Mastery" five times, genuine WITHIN-book corpus duplicates
distinct from the 8 genuine CROSS-book verbatim reprints also found and excluded
(`Quickened Lightning Bolt`, `Agonize`, `Vision of Hell`, `Disfiguring Touch`, `Vermin Shape I/II`,
`Brightest Light`, plus `inner_sea_world_guide`'s 8-way collision with UW/UM/AG/B4).

**Re-derived total, live-computed**: `apps/desktop/src-tauri`'s spell catalog grew 2197 → 2481
records (`cargo test spell_catalog::tests::the_catalog_serves_every_ingested_book_not_only_crb`).

**Not closed this item**: the 17 `classes` families (needs a `ClassId`-shaped chassis + character-
creation/level-up picker wiring per book — the same shape `ClassId`/`ApgClassId`/`AcgClassId`/
`PuClassId` already use, genuinely new per-book engineering, not registration) and
`ultimate_psionics`'s 1,573 `class_features` records (`epic-4-mechanism` scope). Both are unchanged
in kind from row 19 cycle 4's own sizing — sized, not built, this cycle's own scope did not extend
to building a class chassis.

## Item (b): the companion formula residual — stale citations corrected, zero new closures (honest null)

Read `src/rules_core/pilot_compute/formula_interpreter.rs` (1,345 lines, confirmed real and
Gate-2-proven) per the brief. Sampled every `%`-carrying companion record in Advanced Players
Guide (the largest single residual book, 14 of 220 records) directly against their corpus
`raw_tokens`:

- `Evolution ~ Breath Weapon (Cone of Fire)` and its 5 siblings: `%2`/`%4` resolve through
  `BONUS:VAR|BreathWeaponDice|HD` and `BONUS:VAR|BreathWeaponDC|10+(HD/2)+CON` — `HD` and `CON` are
  live-character stats.
- `Evolution ~ Poison Con`/`Poison Str`: `%1` resolves through a `PREABILITY`-gated `BONUS:VAR`
  (whether the character possesses Ability Focus) — feat possession, character-scoped.
- `Evolution ~ Reach`/`Temp Evolution ~ Reach`: `%1|%LIST` — `%LIST` is a PCGen `CHOOSE`-selection
  token, a player's own pick, not a formula at all.

**Every sampled record needs a live character, not merely a working interpreter.** The interpreter
itself would evaluate these formulas correctly in isolation (Gate 2 already proved its grammar
reaches all nine in-scope shape families); what is missing is a consumer surface that hands it a
character — `list_companion_catalog` is a browse-only catalog with none. This is a DIFFERENT,
real remaining gap from "no interpreter exists" (the stale `§24` citation), not a re-filing of the
same wrong reason.

**Corrected, not re-filed**: all 6 `OPEN_FINDINGS` entries citing "a real formula interpreter, out
of scope per `decisions.md §24`" (`advanced_race_guide`, `apg`, `crb`, `ultimate_magic`,
`book_of_the_damned_volume_1`, `ultimate_wilderness`) now name the real blocker (a character-scoped
companion-ability consumer surface, none exists) and cite the real ruling (SD-31 Decision 20,
overturned 2026-08-21, corrected here 2026-08-23) instead of the wrong one. A block comment above
the `OPEN_FINDINGS` array records the correction and its evidence for the next reader, per
`docs/governance/deferral-revisit-doctrine.md`'s "when inheriting a package, evaluate the
condition, don't relitigate it silently" discipline.

**Zero companion records closed by this correction** — an honest null result. No fixture-check
work was needed or performed (`decisions.md §3`): nothing was interpreted, because there is still
no character to interpret against.

## Item (c): the 30 delta-row companions — 25 of 30 closed, 4 sized, engine-need withdrawn

Read `companion_pool_catalog.rs` (this bundle's own row 19 cycle 3 prior art) before building
anything, per the brief. Row 19 cycle 1's sizing ("needs a Celestial/Fiendish creature-template
application engine") was **re-derived and found overstated**: every real `.COPY=` companion record
corpus-wide (25, re-derived via `python3`, not the inherited "28+2=30" breakdown) carries
`description: null` plus a real, self-contained mechanical token —
`TEMPLATE`/`KIT` for a creature-template header (`Cat (Fiendish)`: `TEMPLATE: Fiendish Creature`,
`KIT: 1|NE`) or `ASPECT` for an ability variant (`Pooka ~ Change Shape`:
`ASPECT: Ability Benefit|(2 of the following forms: cat, goat, rabbit ...)`). None is a dangling
fragment of another record's prose — genuinely different from `origin: "mod_only"` rows (confirmed:
`Universal Monster Rule ~ Fast Healing`'s description, "Works only in gusty and windy areas.", has
no antecedent without its base row).

Built a generic `origin == "copy"` tier-3 admission in `companion_pool_catalog.rs::load_raw_pool_entries`,
reusing `reference_library_catalog.rs`'s own `mechanical_summary()` (made `pub(crate)`) rather than
reinventing it or building a bespoke creature-template applicator. `origin: "mod_only"` remains
structurally refused, unchanged — the two `origin` values are deliberately NOT treated the same way
(see the module's own updated doc comment for the reasoning).

**Closed 25 of the 30 for real**: 22 `beastiary1` Celestial/Fiendish rows, 2 `bestiary_4` ability-
variant rows, and 1 `ultimate_wilderness` row ("Margay ~ Sound Mimicry") found live during the
corpus-wide re-derivation, not in any prior receipt's named 30. `bestiary_4`'s companions family is
now fully closed (0 residual) — its `OPEN_FINDINGS` entry is deleted per this table's own
discipline. `beastiary1`'s companion residual falls from 28 to **6**;
`ultimate_wilderness`'s from 43 to **42**.

**Not closed**: the 4 remaining `beastiary1` `Universal Monster Rule ~ Change Shape`/`Disease
(Extraordinary)`/`Fast Healing`/`Poison (Extraordinary)` rows are genuine `origin: "mod_only"`
dangling-conditional-clause fragments — need a real base-record delta-merge mechanism (smaller in
scope than a creature-template applicator, since no template-application math is involved, just
resolving which base record a `.MOD` row's conditional clause attaches to). Sized, not built.

2 new unit tests added (`a_copy_template_row_is_served_as_a_mechanical_summary`,
`a_copy_ability_variant_row_is_served_from_its_aspect_token`), both mutation-proved: the admission
arm was disabled (`if origin == Some("copy") && false`), both tests failed for the intended reason,
then reverted and re-confirmed green.

## Full-sweep re-run

`apps/desktop/src-tauri`: `cargo test --locked --bin codex-desktop` → **538 passed, 0 failed** (up
from row 19 cycle 4's exit state, 536 passed / 0 failed — net +2, both new tests, whole workspace
still green). Root workspace not re-run in full this cycle (`§13`, scoped per the brief's own "do
not run the full unscoped suite" instruction) — only `apps/desktop/src-tauri` is this row's own
write scope and every touched file lives there or in `src/rules_core/spell_resolver.rs`, which
`cargo build --locked --lib` (root) confirmed compiles clean before the desktop build.

## Root-workspace sweep (`decisions.md` cross-file pinned-count discipline)

`spell_resolver::spell_catalog_rows()` also feeds `src/bin/v06_work_inventory.rs`, which is not
this row's own file scope but breaks silently if left unswept (the exact "count change compiles
clean but leaves other files' pinned assertions red" failure mode this bundle has hit three times).

1. `spell_book_slug_for` is a closed-set lookup that **panics** on an unmapped `SPELL_BOOK_*` code
   (its own dedicated test, `spell_book_slug_for_covers_every_catalog_book`). Added the 11 new
   codes. `"UMWP"` maps to `"ultimate_magic"` (its own book's slug) rather than a new slug of its
   own — it has no `RuleSetId`, being a second source file inside the already-compiled
   `ultimate_magic` book, not a new book.
2. `cargo test --locked --bin v06_work_inventory spell` (23 tests, scoped) found and fixed a real,
   pre-existing latent bug this cycle's content exposed rather than introduced:
   `the_probe_examines_every_catalog_spell_key_of_every_observable_book`'s `expected` computation
   iterated `OBSERVABLE_BOOK_DIRS` without deduplicating by resolved `engine_book` — harmless while
   `"beastiary1"`'s `"bestiary_1"` engine-book bucket carried zero spell keys (its two aliased
   directory entries, `"beastiary"` and `"bestiary"`, both resolved to it and both contributed
   zero), now a real double-count once `beastiary1`'s 111 real spell keys populate that bucket.
   Fixed by deduplicating on `engine_book` before accumulating `expected`, matching the production
   `outcomes: BTreeMap<(book,key),_>`'s own natural dedup. Mutation-proved implicitly: the fix was
   proved correct by the failure itself (RED before, GREEN after), the standard "does the failure
   name the intended defect" bar rather than a separate injected mutation.
3. Full `cargo test --locked --bin v06_work_inventory` (357 tests, scoped to this one binary, not
   the unscoped root suite): **357 passed, 2 failed** — both PRE-EXISTING, standing, already
   documented regressions unrelated to this cycle's diff (confirmed via `grep` across
   `docs/release/SD-32-.../progress.md`/`OPEN-ISSUES.md`/prior cycle receipts, e.g.
   `t12-class-feature-pool-population_cycle-2_cycle_receipt.md` already names both by name as
   "present in the live suite"): `e14_harness_tests::a_key_two_books_share_grounds_only_the_book_
   whose_corpus_was_read` (equipment probe, unrelated to spells) and
   `race_trait_grounding_tests::the_t2b_residual_population_is_never_ingested_not_a_matcher_miss`
   (race_trait T2b residual, a standing regression card 11's own T2b cycle recorded deliberately).
   Neither test references `spell_resolver`, `spell_book_slug_for`, or any file this cycle touched.
   `cargo test --locked --lib rules_core::spell_resolver::` (root lib, 12 tests): all green.

## Mutation proofs (RED → GREEN, then reverted)

1. `("adventurers_guide", "equipment")` dispatch arm commented out →
   `every_ingested_family_is_accounted_for` failed with
   `"ingested content with no declared consumer and no recorded finding: adventurers_guide/equipment"`
   → reverted → green.
2. `companion_pool_catalog.rs`'s `origin == "copy"` admission disabled (`&& false`) →
   `a_copy_template_row_is_served_as_a_mechanical_summary` failed with
   `"Cat (Fiendish) must be served via the .COPY= tier-3 admission"` → reverted → green.

## Territory

`git status --porcelain` confirmed clean before every commit: touched only
`src/rules_core/spell_resolver.rs`, `apps/desktop/src-tauri/src/{reach_gate.rs,spell_catalog.rs,
reference_library_catalog.rs,companion_pool_catalog.rs}`, and this row's own kanban/progress cells
— none overlapping row 18's pool-magnitude files (`pilot_compute/mod.rs`,
`class_feature_pool_catalog.rs`, `class_feature_grant_consumer.rs`), which were read for the item
(b) investigation but never written. Rebased on `origin/tranche/12` immediately before push and
re-ran the targeted tests after (§5/§6 requirement).

## Next-cycle plan

1. **The 17-family `classes` chassis + `ultimate_psionics`'s 1,573 `class_features`.** The largest
   remaining item — a `ClassId`-shaped enum plus character-creation/level-up picker wiring per
   book, the same shape `ClassId`/`ApgClassId`/`AcgClassId`/`PuClassId` already use. Genuinely new
   per-book chassis engineering, not registration; size per book before committing to all 17 in one
   cycle.
2. **The 4 remaining `beastiary1` `Universal Monster Rule ~ ...` `mod_only` rows** need a real
   base-record delta-merge mechanism — read how `ingest_spells.rs`'s `build_global_base_index`
   resolves a `.COPY=` variant's base by name across every book's file, and adapt the same
   name-indexed-lookup pattern for a `.MOD` row's own base (a different corpus dir walk, same
   underlying idea).
3. Row 20 stays `in-progress` under `decisions.md §10` until both residuals above reach zero.

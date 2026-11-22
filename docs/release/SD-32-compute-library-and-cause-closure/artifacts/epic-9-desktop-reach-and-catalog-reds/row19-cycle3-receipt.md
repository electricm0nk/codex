# Cycle row19-cycle3 — Epic 9 (`epic-9-desktop-reach-and-catalog-reds`) / Row 19

- **Card ID:** `epic-9-desktop-reach-and-catalog-reds`
- **Files touched:**
  - `apps/desktop/src-tauri/src/companion_pool_catalog.rs` (new)
  - `apps/desktop/src-tauri/src/companion_catalog.rs`
  - `apps/desktop/src-tauri/src/reach_gate.rs`
  - `apps/desktop/src-tauri/src/main.rs` (module registration only)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, per §6's own guidance that the full
  `BASE_BRANCH...HEAD` form returns tens of thousands of pre-existing tagged lines).
- **Wired-integration audit result:** `OK_NO_TOKENS`.
- **PI scrub:** `pi_scrub.normalized_term_hits()` on the full diff plus the new file — zero hits.
- **Acceptance criterion:** kanban row 19 — build the generic "referenced pool" mechanism cycle 1
  and cycle 2 both named, re-derive the companion residual, close as much as the mechanism
  genuinely reaches, and set `complete` only when the whole desktop workspace is green.
- **Corpus SHA:** oracle bootstrapped fresh this cycle (`scripts/fetch-pcgen-oracle.sh --dest`,
  confirmed populated, `pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6`).
- **Status:** `in-progress` (NOT `complete` — 2 of `apps/desktop/src-tauri`'s tests remain red,
  both outside this mechanism's scope; see below).

## Starting state (verified, not assumed)

Worktree started on a stale `tranche/11`-lineage tip (footgun 1, fired a SIXTH time across this
bundle). `git merge-base --is-ancestor $PIN HEAD` failed; recovered via `git reset --hard $PIN` +
`git rebase origin/tranche/12` (reported "up to date" — `HEAD` already equalled
`origin/tranche/12` after the reset). Re-verified (`git merge-base --is-ancestor $PIN HEAD` →
`BASE_OK`) before any edit. Oracle slot was empty (git-ignored, fresh worktree); bootstrapped via
`scripts/fetch-pcgen-oracle.sh --dest`, confirmed populated before trusting any figure.

Reproduced cycle 2's exit state first: `apps/desktop/src-tauri` → `cargo test --locked --bin
codex-desktop` (own `CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0`) → **512 passed, 7 failed** at cycle
1's baseline / **514 passed, 5 failed** at cycle 2's exit, matching the brief exactly.

## The mechanism built: `companion_pool_catalog.rs`

Read `class_feature_pool_catalog.rs` and `class_feature_grant_consumer.rs` first, per the brief.
Built the SAME "member of a referenced pool" shape for `companion`, not a second module reinventing
it: a `companion/*.json` record with `owners: []` and `origin: "declared"` is a shared
reference-library entry (`Animal Trick ~ Aid`, `Aberrant Companion ~ Aberrant Sight`, an eidolon
`Evolution ~ ...`, ...), not a creature's own ability — the same shape `reach_gate.rs`'s existing
`OPEN_FINDINGS` already documents for `monster_ability`'s `owners: &[]` orphans, generalized to a
second kind for the first time.

**Structural admission gates, all real and mutation-tested, mirroring `class_feature_pool_catalog
.rs`'s own render-and-refuse discipline:**

1. `owners` empty (present-and-empty, or absent). A creature stat-block record (`gen_book_cache`-
   written) carries no `owners` field at all, which `is_none_or` alone would read as vacuously
   empty — the real near-miss found and fixed this cycle (test:
   `a_creature_stat_block_record_is_never_admitted_as_a_pool_member`).
2. `origin == "declared"`. This is what ACTUALLY protects against creature records (which carry no
   `origin` field either) and against PCGen `.MOD`/`.COPY=` delta rows (`origin: "mod_only"` /
   `"copy"`) — a delta row can render perfectly clean PCGen syntax while still being a meaningless
   fragment without its base record (confirmed real: `beastiary/companion/
   universal_monster_rule_fast_healing.json`, `origin: "mod_only"`, description "Works only in
   gusty and windy areas." — a dangling conditional clause). Test:
   `a_mod_only_delta_row_is_refused_even_though_it_renders_clean`.
3. Render-and-refuse: `render_pcgen_desc` with no character to resolve against; any unresolved
   `%N` or leaked syntax refuses the record. Test:
   `render_and_refuse_gate_is_provably_live` (mutation-proves-RED, per the universal requirement).
4. A real description value (not null/empty/`.CLEAR`/the PI marker).

A `" ~ "` group qualifier in the key is common but NOT required — two real Advanced Player's Guide
records (`Companion Bonus Skill`, `Eidolon Bonus Skill`) are genuine, ungrouped, clean-rendering
standalone content, served as their own singleton pools rather than excluded on a syntax
technicality (test: `an_ungrouped_clean_record_is_served_as_its_own_singleton_pool`). Found and
fixed mid-cycle after the first full sweep showed these 2 records wrongly unaccounted-for despite
carrying zero defect.

**9 unit tests, all green**, covering every gate above plus the owned-ability non-duplication case
and book/group aggregation.

## Wired into both consumers

- `companion_catalog.rs`'s `CompanionCatalogResponse` gained `pool_groups:
  Vec<CompanionPoolGroupDto>`, populated by `build_companion_catalog()` — a real, additional field on
  the existing Tauri command's response, not a parallel unconsumed struct.
- `reach_gate.rs`'s `companions_reach` now folds every pool-group ability into `with_payload` (every
  entry that reaches `pool_groups` at all already carries a real rendered description by
  construction) — inserted by `corpus_key` (the record's raw, un-slugged `data.key`), not the
  slugged wire `key`, because `corpus_record_keys`'s denominator reads `data.key` verbatim and this
  ingest path (`scripts/ingest_companion.py`, unlike every other kind and unlike
  `gen_book_cache`-written companion creature/ability records) never slugs it. This was the first
  real near-miss found this cycle: without the `corpus_key` field, every served pool entry would
  silently fail to match its own denominator, closing the mechanism as a no-op.

## Re-derived, not assumed (`§17a`) — the new numbers

Before this cycle (cycle 2's own figures): `companion_catalog::every_served_key_matches_a_corpus
_record_file` named **434 records across 4 books** unaccounted for (`ultimate_wilderness` 248,
`ultimate_magic` 139, `advanced_race_guide` 18, `book_of_the_damned_volume_1` 29), plus
`beastiary1`'s pre-existing 28 and `bestiary_4`'s 2. `reach_gate`'s `every_ingested_family_is_
accounted_for` named **~170** unaccounted `(book, kind)` families across 12 newly-classified
corpus kinds.

After this cycle, live-computed via `cargo test`, never hand-counted:

- **Reachability** (records that genuinely became reachable through the new mechanism — a real
  root-cause fix, not an instrument correction): the companion residual fell from **434+28+2 =
  464** records across 6 books to **330** records across **8** books (2 more books —
  `advanced_players_guide`/`core_rulebook` — surfaced their own residuals the moment a generic
  pass looked at every book the transcribed table registers, not the 4 the brief's estimate
  named). **134 records now reach a player for real**: `ultimate_wilderness` 248→43,
  `ultimate_magic` 139→106, `advanced_race_guide` 18→9, `book_of_the_damned_volume_1` 29→4, plus
  every clean `" ~ "`-qualified orphan across every other registered book that previously reached
  no surface at all. Verified per `§16`: these are records that moved from "reaches no surface" to
  "genuinely serves a real rendered description on the wire", not a reclassification.
- **Instrument correction** (the test's OWN assertion mechanism changed, no record's reach
  changed): `companion_catalog::every_served_key_matches_a_corpus_record_file` no longer needs a
  static, hand-maintained exception list for the residual — it now re-derives, per residual
  record, whether one of the pool catalog's own three refusal reasons applies (empty description /
  non-`"declared"` origin / unresolved formula), structurally, the same way the catalog itself
  decided not to serve it. This is `§16`-clean: no record's reach changed by this refactor, only
  how the test proves the residual is real rather than gamed.
- **The residual, named by book, not gamed:** `advanced_race_guide` 9, `apg` 137 (2 fewer than the
  naive 139 estimate — `Companion Bonus Skill`/`Eidolon Bonus Skill` are genuinely clean, ungrouped
  records the mid-cycle widening now serves), `beastiary1` 28 (unchanged — needs a Celestial/
  Fiendish creature-template application engine, cycle 1's original assessment), `bestiary_4` 2
  (unchanged, same shape), `book_of_the_damned_volume_1` 4, `core_rulebook` 31, `ultimate_magic`
  106 (1 fewer — `Black Blade ~ Ego` shifted, same reason), `ultimate_wilderness` 43. Every one of
  these 330 records is now pinned by exact key in `UNREACHED_RECORD_FINDINGS` and named by book in
  `OPEN_FINDINGS`, copied verbatim from a live `cargo test` failure output, never retyped from
  memory.

## Closed this cycle

- `companion_catalog::tests::every_served_key_matches_a_corpus_record_file` — **GREEN**.
- `reach_gate::tests::every_declared_claim_actually_carries_the_records` — **GREEN**.
- `reach_gate::tests::unreached_records_are_exactly_the_recorded_findings` — **GREEN**.
- `reach_gate::tests::unsurfaced_families_are_exactly_the_recorded_findings` — **GREEN** for
  `companion`; still red, unrelated to this cycle's scope (see below).

## NOT closed — named with fresh evidence, not gamed

`reach_gate::tests::every_ingested_family_is_accounted_for` and `reach_gate::tests::
unsurfaced_families_are_exactly_the_recorded_findings` both remain red, on the **same ~170
`(book, kind)` families** cycle 2 named — `abilities`, `domains`, `templates`, `languages`,
`skills`, `deities`, `generic_feats`, `race_variants`, `class_variants`, `monster_variants`,
`named_traits`, `powers`, recurring across roughly 30 books. **None of these families is
`companion`** — this cycle's mechanism was scoped to the eidolon-evolution / companion
shared-reference-library shape specifically, as the brief directed, and does not touch these 12
unrelated kinds. Re-confirmed live this cycle (not carried forward from cycle 2's count): the
family list is unchanged in content and count from cycle 2's dump, module for module — this
mechanism neither helped nor hurt that residual, which needs its own generic pass (a browsable
catalog per kind, or a shared multi-kind "declared, not consumer-reached yet" surface) as its own
cycle's scope.

## Full-sweep re-run

`apps/desktop/src-tauri`: `cargo test --locked --bin codex-desktop` → **526 passed, 2 failed** (up
from 514 passed / 5 failed at cycle 2's exit — net +12 passed [9 new unit tests plus 3 previously-
red tests closed], -3 failed). Full failing list: `reach_gate::tests::every_ingested_family_is_
accounted_for`, `reach_gate::tests::unsurfaced_families_are_exactly_the_recorded_findings`.

## Territory

`git status --porcelain` confirmed clean before every commit: touched only `companion_catalog.rs`,
`companion_pool_catalog.rs` (new), `reach_gate.rs`, `main.rs` (module registration line only) — none
overlapping row 18's pool-magnitude files (`pilot_compute/mod.rs`, `class_feature_pool_catalog.rs`,
`class_feature_grant_consumer.rs`), row 11's `kanban.md`/`progress.md` (this cycle edits only row
19's own cells), or the corpus-literal-sweep/monster_chassis lanes. Rebased on `origin/tranche/12`
immediately before push and re-ran the targeted tests after.

## Next-cycle plan

1. **The ~170-family residual is the whole remaining scope for row 19.** It is 12 corpus kinds ×
   ~30 books, none of them `companion` — the next cycle should build ONE generic browsable-catalog
   mechanism per kind (or a shared "declared content, no consumer yet" surface spanning all 12),
   the same `§17` discipline this cycle applied to `companion`, batched by KIND as cycle 2's own
   plan already named, not by book.
2. **`beastiary1`'s 28 and `bestiary_4`'s 2 `.COPY=`/`.MOD` delta rows remain genuinely
   unresolved** — still need a real Celestial/Fiendish creature-template application engine, sized
   as its own epic, unchanged from cycle 1's original assessment.
3. **The formula-scaled residual (roughly 260 of the 330 companion records left red) needs a real
   formula interpreter** (`MasterLevel`, eidolon-level-scaled DR/resistance/breath-weapon
   magnitudes, arcane-pool-scaled Black Blade values) — out of scope per `decisions.md §24`
   ("there is no formula interpreter") unless a future decision revisits that scoping.

# Cycle t2b-w1-a/2 — Gate 3 closure invariant / Epic 2, shape T2b, book `monster_codex`

- **Card ID:** `epic-2-cause-closure` (row 11)
- **Actor:** `t2b-w1-a`
- **Commit SHA:** (see push receipt)
- **Files touched:**
  - `src/bin/ingest_race_traits.rs` — pinned per-book count (`monster_codex` 5 -> 11) and total
    (544 -> 550) in `no_committed_trait_description_leaks_pcgen_syntax_in_any_declared_book`.
    **No parser/scope change was needed**: Ratfolk was already in `IN_SCOPE_RACES` (widened by
    SD-31-E6-F4-002/003), so `cargo run --bin ingest_race_traits -- monster_codex` already emits
    its 6 real alternate-trait rows on a plain re-run — the 5-record state on disk was stale
    output from before that widening, not a code gap.
  - `src/rules_core/race_resolver.rs` — added the 4 new Ratfolk alternates' replace-flag entries
    to `ALTERNATE_TRAIT_REPLACE_FLAGS` (`Cheek Pouches`/`Cleanliness`/`Lab Rat`/
    `Surface Sprinter`, transcribed from `mc_abilities_race.lst`'s own `FACT:` tokens) — this IS
    the real fix: without it the picker offers the 4 alternates and `pilot_compute` refuses every
    one with a claim-blocking `race.alternate_trait.unknown`
    (`every_alternate_the_app_offers_is_one_the_engine_can_place`, proven RED before the fix).
    Updated 8 pinned-count assertions in the same file that this table addition and the re-run
    moved (`Default`/`Alternate`/`FlagGranted`/total-role counts, `RACE_SIZES` unaffected here).
  - `data/corpus/monster_codex/race_trait/ratfolk/*.json` (new, 6 files) — real re-derived output
    of the plain re-run above, fixture-checked against the pinned oracle (transcription is
    mechanical off `mc_abilities_race.lst`'s own tokens, not hand-authored).
  - `apps/desktop/src-tauri/src/race_catalog.rs`, `race_trait_picker.rs`,
    `corpus_ingest_diagnostic.rs` — pinned-count sweep (separate cargo workspace, tested
    explicitly) for the same 4-alternate/8-Monster-Codex-total shift.
  - `tests/duergar_invisibility_sla_reaches_a_player_via_monster_codex.rs` — the
    `the_monster_codex_race_trait_records_are_the_in_scope_ones` test's own comment asserted
    "Ratfolk has no ingested race chassis" — stale since SD-31-E6-F4-002 (2026-08-16). Corrected
    the expected key set to include the 6 real Ratfolk rows and restated why `Standard Goblin`
    stays absent (see finding below, not the old "no chassis" reason).
  - `tests/sd27_alternate_racial_trait_reachability.rs`,
    `tests/sd27_aasimar_globalvar_gate_closes_the_dead_affordance.rs` — pinned-count sweep (4 more
    357->361 sites across the two files).
  - `docs/retro/events/t2b-w1-a.jsonl` — correction logged.

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim):** AT-32-E2-001 — T2b closed corpus-wide, by class.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6`
- **Status:** complete (7 of 8 real-work units closed; 1 remains open, named below)

- **RED -> GREEN evidence:**
  1. Before the `ALTERNATE_TRAIT_REPLACE_FLAGS` addition, re-running `ingest_race_traits` alone
     (no engine-table change) made `every_alternate_the_app_offers_is_one_the_engine_can_place`
     fail for exactly the 4 new Ratfolk alternates — confirmed the intended reason (picker offers
     content the engine cannot place), not a build error.
  2. Mutated `("Ratfolk ~ Cheek Pouches", &["Ratfolk_ReplaceSwarming"])` to a wrong flag name post
     -fix; `the_alternate_trait_flag_table_matches_the_corpus_for_every_alternate` failed with
     `left: ["Ratfolk_ReplaceWRONGFLAG"] right: ["Ratfolk_ReplaceSwarming"]` — confirmed the
     intended reason, reverted, re-ran green.

- **Notes / findings:**
  1. **`Standard Goblin` (`mc_abilities_race.lst:30`) is correctly excluded, not open work.**
     Its row carries `VISIBLE:DISPLAY`, `TYPE:Goblin Variant`, `SOURCEPAGE:p.104` and nothing
     else — no `DESC:`, `BONUS:`, or `ABILITY:` token at all. It is a UI bookkeeping marker for
     "took no Goblin variant" (the pool-empty counterpart to `Oversized Goblin`, which DOES carry
     real content and is already ingested). Every committed race_trait record in this corpus
     carries a real description (`no_committed_trait_description_leaks_pcgen_syntax_in_any_
     declared_book`'s own `with_description == checked` assertion, corpus-wide); inventing prose
     to satisfy that would violate `decisions.md §3`. Logged as a `scripts/retro.py correction`
     against the census memo's "8 real-work units" figure for this book (the row's `TYPE:` also
     matches none of the census script's header patterns, so it fell into "other" by default, not
     because it was checked against the raw oracle text).
  2. **`Bat (Sootwing) ~ Paralysis` (`mc_abilities_race.lst:72`) remains open.** It sits under the
     file's `### MONSTER ABILITIES` section (a Sootwing Bat stat-block special ability, `.COPY`'d
     from `Universal Monster Rule ~ Paralysis (Supernatural)`), not the `### RACE ABILITIES`
     section the 7 closed units above belong to. This is genuine content, but it is a *monster*
     special ability, not a *playable-race* trait — the same shape `decisions.md §13`'s T9 row
     names ("monster_ability... per-record onboarding backlog"), filed under `race_trait` kind
     only because the walker types every row in a `*_abilities_race.lst` file by filename,
     regardless of whether the row's own content is a player race option or a monster stat block.
     Closing it correctly needs either T9's own monster-ability ingestion mechanism reaching this
     file, or an explicit ruling that `ingest_race_traits.rs`'s per-race-chassis pipeline is also
     the right home for monster-stat-block content with no playable-race chassis at all. **Not
     attempted this cycle** — escalated, not silently dropped or hand-modelled as an instance.
  3. `monster_codex`'s "8 real-work units" cited by the dispatch brief (matching the census memo)
     is itself off by the `Standard Goblin` correction above: real transcribable work was 7
     (6 Ratfolk + `Bat (Sootwing) ~ Paralysis`), of which 6 are now closed.

- **Discovery forwards:** `Bat (Sootwing) ~ Paralysis` — monster-ability content in a race_trait
  -typed file; needs T9's mechanism or a ruling, not T2b's per-race chassis pipeline.
- **Next-cycle plan:** none for this book beyond the one named residual above; a T9-scoped or
  ruling-driven cycle picks it up.

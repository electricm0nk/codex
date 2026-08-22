# Cycle 3 — Gate 0 census closure / Criterion AT-32-G0-003

- **Card ID:** `gate-0-book-onboarding-precondition`
- **Commit SHA:** `a50b7da04` (implementation); `21b348ed9` (retro-log append after the
  `verify.sh --only reach` run)
- **Files touched:**
  - `src/rules_core/rules_tables/mod.rs` (new `RuleSetId` variants: `InnerSeaFaiths`,
    `InnerSeaMagic`, `InnerSeaTaverns`, `InnerSeaTemples`; three new `pub mod` registrations)
  - `src/rules_core/rules_tables/inner_sea_faiths/{mod.rs,spell_list.rs}` (new)
  - `src/rules_core/rules_tables/inner_sea_magic/{mod.rs,spell_list.rs}` (new)
  - `src/rules_core/rules_tables/inner_sea_temples/{mod.rs,spell_list.rs}` (new)
  - `src/bin/ingest_inner_sea_setting_spells.rs` (new — codegen for the three spell modules above)
  - `src/bin/gen_feat_gap_tables.rs` (new `BOOK_INPUTS` row: `inner_sea_taverns`)
  - `src/rules_core/rules_tables/feat_gap_tables.rs` (regenerated — new
    `INNER_SEA_TAVERNS_FEAT_GAP_ROWS`, 9 rows)
  - `src/rules_core/rules_tables/feats_all.rs` (new empty `hand_authored_feat_tables()` entry for
    `RuleSetId::InnerSeaTaverns`; pinned-count test updates)
  - `src/rules_core/spell_resolver.rs` (three new `SPELL_BOOK_*` constants, chained into
    `spell_catalog_rows()`)
  - `src/bin/v06_work_inventory.rs` (`COMPILED_RULE_SETS`, `corpus_dir_for`, `rule_set_id`,
    `spell_book_slug_for` — four new book registrations)
  - `src/bin/v06_content_state_dump.rs` (exhaustive-match arms for the four new `RuleSetId`
    variants)
  - `apps/desktop/src-tauri/src/spell_catalog.rs` (three new `map_*_entry` functions + `BOOK_*`
    constants, chained into `build_spell_catalog()`; test updates)
  - `apps/desktop/src-tauri/src/reach_gate.rs` (four new reach claims: `inner_sea_faiths`/
    `inner_sea_magic`/`inner_sea_temples` spells, `inner_sea_taverns` feats)
  - `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs` (three new `book_status` rows +
    counts functions; pinned-order test update)
  - `apps/desktop/src-tauri/src/feat_catalog.rs`, `src/rules_core/feat_prereqs.rs`,
    `src/rules_core/feat_identity.rs`, `apps/desktop/src-tauri/src/character_hub.rs`,
    `tests/sd27_feat_prerequisite_enforcement.rs`,
    `tests/sd27_known_spells_must_be_on_the_class_spell_list.rs`,
    `tests/v06_apg_acg_feat_catalog.rs` (pinned-count sweep: every hardcoded catalog total/
    per-book/per-category count that moved with the new records, re-derived by running the
    affected test and reading its actual/expected mismatch, never guessed)
  - `docs/retro/events/gate-0-book-onboarding.jsonl` (new — rework event for the mid-cycle
    intra-book dedup fix)

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff against
  `git merge-base HEAD origin/develop`, this cycle's file set; the two `sd[0-9]+_` hits the
  unscoped grep would show are `diff --git`/`---`/`+++` header lines naming the pre-existing
  `tests/sd27_*.rs` filenames I edited, not new content — self-explained, not force-scrubbed)
- **Wired-integration audit result:** `OK_NO_TOKENS`

- **Acceptance criterion (verbatim, `acceptance-and-verification.md`):**
  **AT-32-G0-003.** "The four unbuilt books (Epic 4 scope) land their compiled rule sets before
  Gate 0 is declared closed. The decision to demote book onboarding from epic to precondition
  (`scope-draft.md`, 'The gap that makes Gate 0 necessary') is binding: running Gate 1 against an
  open hole in the census guarantees the rerun the operator does not want." — **MET.**

- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`; `scripts/verify.sh --only preflight-oracle` → PASS at cycle start).

- **Status:** complete

- **Notes / judgment calls:**
  1. **Which four books.** `progress.md`'s Cycle 2 (card 3) receipt already named them:
     `inner_sea_faiths`, `inner_sea_magic`, `inner_sea_taverns`, `inner_sea_temples` — the four
     `docs/work-inventory.json` books with `scope: "future_state"` and `engine_rule_set: null`.
     Cross-checked against `epic-breakdown.md` Epic 4's "422 units" figure:
     `python3 -c "..."` summing each book's `kinds[*].units` over `docs/work-inventory.json`
     gives faiths 3 + magic 335 (3 class + 218 class_feature + 7 feat + 39 spell + 6 equipment +
     62 equipment_modifier) + taverns 20 (11 class_feature + 9 feat) + temples 64 (21 spell + 43
     equipment) = **422**, matching the epic figure exactly — confirms this is the right book set,
     not merely an assumption.
  2. **`adventurers_guide` is already onboarded** (SD-31 wave-29, `RuleSetId::AdventurersGuide`) —
     it is NOT one of the four; the earlier confusion in that book's own commit message ("the
     other three THE-BOX #3 books... remain future work") undercounted by one (it omitted
     `inner_sea_faiths`, which THE-BOX's own #3 finding hadn't separately named). Re-derived from
     `docs/work-inventory.json`, not carried forward from that commit message.
  3. **Shape per book, following the established "first compiled rule set" precedent** (`Oa`,
     `Mythic`, `AdventurersGuide`): three books (`inner_sea_faiths`, `inner_sea_magic`,
     `inner_sea_temples`) have a dedicated `*_spells.lst` corpus file, so their first family is
     `spell` — a dedicated `rules_tables::<book>::spell_list` module, ingested by a shared codegen
     binary (`ingest_inner_sea_setting_spells.rs`) reusing the tested general-purpose LST spell
     parser and both SD-30 PI-screening contracts, exactly as
     `ingest_adventurers_guide_spells.rs` does. `inner_sea_taverns` has no `*_spells.lst` at all;
     its first family is `feat`, via the existing `gen_feat_gap_tables.rs` generalised generator
     (the same mechanism `RuleSetId::Mythic` uses), which needed no new dedicated module directory.
  4. **A genuine intra-book duplicate, found and fixed mid-cycle.** `isf_spells.lst` restates
     "Curse of Disgust (Besmaran)" as a second, non-`.MOD` base declaration later in the same file
     (a fuller reprint of the same spell). The first ingest pass shipped it as 3 raw entries;
     `spell_resolver::spell_catalog_rows()`'s own cross-book dedup then silently collapsed it to 2
     in the *served* catalog, which `mapping_helpers_agree_with_the_registry`
     (`apps/desktop/src-tauri/src/spell_catalog.rs`) caught as a length mismatch (the test's naive
     re-chain of the raw tables has no dedup). Fixed at the source: the ingest binary now dedups
     within a book (first-declaration-wins, mirroring the resolver's own cross-book policy) before
     writing the module, so the raw table never asserts a count the resolver would silently
     shrink. Logged: `scripts/retro.py rework` (`docs/retro/events/gate-0-book-onboarding.jsonl`).
  5. **Corpus-JSON-only families are not double-registered.** `inner_sea_magic`'s 218
     `class_feature` records (and `inner_sea_taverns`'s 11) were already ingested corpus-wide as
     `data/corpus/<book>/class_feature/*.json` by an earlier SD-31 cache-gen lane — landed with no
     compiled `RuleSetId` to unlock the book-level gate for them. This cycle does not re-ingest or
     re-count them; it only registers each book's `RuleSetId`, which is what makes
     `v06_work_inventory::classify`'s book-level gate stop short-circuiting every one of that
     book's units to `not-started`/`no_compiled_rule_set_for_book` before any per-kind arm runs —
     the same "registering the rule set via one family unblocks the book-level gate for ALL of its
     kinds" mechanism `RuleSetId::AdventurersGuide`'s own landing commit documented and measured.
     `corpus_ingest_diagnostic.rs`'s new `inner_sea_magic_counts()` deliberately does NOT sum these
     218 corpus-JSON-only records into its `content_kind_counts` row, matching the established
     precedent `advanced_race_guide`'s own corpus-JSON-only `class_feature`/`race_trait` records
     already set (that panel counts compiled `rules_tables` families only; a separate
     reconciliation test accounts for the corpus-JSON-only gap where one exists).
  6. **`class_feature`/`feat` reach for `inner_sea_magic`/`inner_sea_taverns` beyond this cycle's
     one representative family is explicitly NOT this cycle's scope** — `reach_gate.rs` already
     carries `OPEN_FINDINGS` entries for both books' `class_feature` gap ("no per-class mechanism
     wiring has landed for this book's classes yet"), unchanged by this cycle; `inner_sea_taverns`
     feats now DO reach a player (`feats_reach(RuleSetId::InnerSeaTaverns, "InnerSeaTaverns")`),
     genuinely, not just registered — verified live against `feat_catalog::build_feat_catalog()`'s
     real response, not asserted.
  7. **`docs/work-inventory.json` was NOT regenerated this cycle** — `cargo run --locked --bin
     v06_work_inventory` fail-closed-refuses without `CORPUS_LITERAL_SWEEP_REPORT`/
     `DERIVED_FIXTURE_CHECK_REPORT` set ("this run would drop 8246 of the 8246 verification
     stamp(s)"), the identical guard Cycle 2 (card 3) hit and correctly did not force past. Those
     two reports belong to a different pipeline (the literal-sweep + fixture-check cycle), and
     `--allow-stamp-loss` is exactly the shortcut AGENTS.md rule 5/8 forbids taking on a prompt's
     authority. AT-32-G0-003's own text is about compiled rule sets landing, not about the
     inventory file's regeneration — met independently via the reach-gate/test evidence above; the
     inventory regeneration is deferred to whichever cycle next runs the full sweep+fixture-check
     pipeline (flagged forward, not silently skipped).
  8. **Verification run for real, this cycle:**
     - `cargo build --locked --lib` and `--bins` (root crate): clean, zero errors, one
       pre-existing unrelated warning (`EMPOWER_SPELL_METAMAGIC_SELECTION` dead code, not touched
       here).
     - `cargo build --locked --bins` (`apps/desktop/src-tauri`): clean, zero errors.
     - `cargo test --locked --lib` (root crate): **2341 passed, 0 failed, 13 ignored** (unrelated
       corpus-gated skips).
     - `cargo test --locked --bins` (`apps/desktop/src-tauri`): **514 passed at first full run**;
       one failure (`corpus_ingest_diagnostic::tests::last_ingested_at_is_a_real_git_derived_
       timestamp_when_available`) because the new book directories had no git history yet before
       this cycle's own commit — resolves once committed, per that test's own doc comment ("this
       test runs inside the real repo checkout, so git history... must be reachable"); not
       force-skipped, just correctly gated on the commit this receipt accompanies.
     - `scripts/verify.sh --only reach`: PASS (30/30 `reach_gate` tests).
     - Targeted integration tests directly exercising the moved figures — all green after the
       pinned-count sweep: `tests/sd27_feat_prerequisite_enforcement.rs` (9 passed, 3 ignored —
       corpus-gated), `tests/sd27_known_spells_must_be_on_the_class_spell_list.rs` (6 passed),
       `tests/v06_apg_acg_feat_catalog.rs` (9 passed).
  9. **RED → GREEN, concretely.** Every one of the pinned-count assertions above started RED for
     the intended reason (a real count moved: feat gap total 531→540, spell catalog total
     2056→2113, feat catalog total 2109→2118, `PRE`-kind census deltas, category-count deltas —
     each confirmed by running the specific failing test and reading its own `left`/`right`
     mismatch before editing the assertion, never guessed from the diff alone) and went GREEN
     after the fix, never edited pre-emptively.

- **Discovery forwards:** none new. Both `## DISCOVERED` items already open from Cycle 2 (the
  ten-kind `class_feature` gap; the "158-book"/"38,372-unit" stale figures) are untouched by this
  cycle and remain open.

- **Next-cycle plan:** Gate 0 is now closed (AT-32-G0-001/002/003 all met). Card 5
  (`gate-1-shape-closure`) opens next per `kanban.md`'s gated order.

## Gate wrap-up (this cycle closes Gate 0 — census closure), `workflow-instruction.md §12`

1. **Retro summary, `scripts/retro.py summary --since 2026-08-22 --json`, read not just run.**
   Over the gate's work window (Pre-G0 + Gate 0, all same-day): **9 corrections** (7 already
   present from Pre-G0/earlier SD-32 cycles + 2 new this cycle — none against a claim this cycle
   made itself; the census-card corrections about the "158-book"/"38,372-unit" figures are
   Cycle 2's, cited not re-derived), **2 open deferrals** (unchanged by this cycle), **7
   incidents** (all Pre-G0/SD-31-era; none from this cycle), **1 rework** (this cycle's own
   intra-book spell dedup, note 4 above). **Recurrence key firing more than once:** `disk-full`
   ×3 (all Pre-G0-era, unrelated to this cycle's work — flagged per standing lesson 1, not
   actioned here since it is outside this card's file grant and already a known SD-31-era
   pattern the operator has separately tracked).
2. **Worktree sweep for this gate's worktrees.** `git worktree list` shows two entries: the
   primary checkout (this cycle ran here, in the primary checkout, matching card 3's own
   precedent — no isolation requested for the serial Gate 0 phase per
   `workflow-instruction.md §2.4`) and `worktree-wf_efd6f5fc-a9c-1`, an unrelated pre-existing
   worktree not owned by any Gate 0 card. Nothing to remove: Gate 0's own cards (3, 4) used no
   dedicated worktree.
3. **Open rulings check, `decisions.md §7` B1/B2/B4/B5.** None of the four books this cycle
   onboarded touch race attribution (B1/B2), the class doneness gate (B4), or `Ex-*` records
   (B5) — `inner_sea_faiths`/`inner_sea_magic`/`inner_sea_temples`/`inner_sea_taverns` contribute
   no `class`, `race`, or `race_trait` units in this cycle's own scope (spell/feat families
   only). No new finding against any of the four; standing lesson 7 satisfied by explicit check,
   not by omission.
4. **No PR here** — per §12 step 4, the bundle's single `tranche/12 → develop` PR is card 13's
   job.

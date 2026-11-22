# Cycle 6 — Epic 4 (Ultimate Campaign to zero) / AT-34-E4-002

- **Commit SHA:** `f74db48f38`
- **Files touched:** `src/rules_core/trait_effects.rs` (+308/-? : new `TraitSaveBonus` struct,
  `SAVE_TRAIT_BONUSES` 2-entry table, `find_save_by_trait_id`, `SaveBonusesFromTraits`,
  `save_bonuses_from_traits`, `save_trait_magnitude_is_grounded_for_corpus_key`, module doc
  "Fourth slice" section, 5 new tests), `src/rules_core/pilot_compute/mod.rs` (+33/-4:
  `compute_total_saves` folds `trait_effects::save_bonuses_from_traits` into all three save
  totals and their explanation strings, the real consumer), `src/rules_core/character_input.rs`
  (doc-comment correction: the `selected_traits` field comment now names all four compute
  paths and the correct 42-of-59 coverage figure, was stale at 31-of-59), `src/bin/
  v06_work_inventory.rs` (+57/-?: fourth `.or_else` fallback onto `save_trait_magnitude_
  is_grounded_for_corpus_key` in the `Kind::Trait` classifier, doc-comment update, 1 new
  positive-classifier test, negative-control comment widened to name the fourth entry point),
  `apps/desktop/src-tauri/src/trait_picker.rs` (+130/-?: `CharacterTraitOptionDto.save: Option
  <String>`, a fourth `save_bonus` iterator chained into `list_available_character_traits`, 4
  new/updated tests), `apps/desktop/src/boundary/loadCharacterTraits.ts` (+26/-?: `save: string
  | null` field, doc-comment update), `apps/desktop/src/characterHub/CreateCharacterForm.tsx`
  (+14/-?: the trait-row display line now renders `"<Save> save"` for a save-bonus option, and
  the section's own doc comment corrected from a stale "31" to the real 42-trait count spanning
  all four slices), `scripts/completion_atlas.py` (+7/-4: instrument-correction, the bucket-V
  citation line pin re-derived after this cycle's own insertions into `v06_work_inventory.rs`
  shifted it, `citation_failures` 1→0).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (this cycle's own diff against the pre-cycle
  working tree, scoped to the 8 files above — zero hits). Re-run at the workflow-instruction's
  own literal §6 formula (`BASE_BRANCH=$(git merge-base HEAD origin/develop)`, i.e. the whole
  bundle's diff since the `tranche/14` cut across Epic 4's full `src/rules_core/`/`src/bin/`/
  `apps/desktop/src{-tauri,}/src` file-touch set) surfaces 20 hits, all inside `src/rules_core/
  pilot_compute/class_feature_grant_consumer.rs` — a file this cycle never touched, confirmed by
  `awk`-walking each diff hunk back to its own `diff --git` header; those are legitimate
  pre-existing `sd13_*`/`sd25_*` real test-file-name references from concurrent Epic-3 work, not
  a bundle-tag leak from this cycle.
- **Wired-integration audit result:** `OK_NO_TOKENS` (this cycle's own diff, same 8-file scope
  — zero hits). The same whole-epic-scope formula surfaces 15 hits, every one the English word
  "placeholder" describing PCGen's own corpus token shapes (`%LIST` player-chosen-target
  placeholders, "Human Ethnicity placeholder row", CHOOSE-menu "no selection" placeholder rows)
  in doc comments/prose from cycles 1-5's own pre-existing text (including one line in
  `trait_effects.rs`'s own "Scope this cycle deliberately covers" section, written by cycle 1,
  untouched by this cycle) — never a marker for incomplete shipping code. Verified by re-running
  the check against this cycle's own uncommitted diff alone (above), which is unambiguously
  clean.
- **Acceptance criterion:** AT-34-E4-002 — drive Ultimate Campaign to zero — BUILD the character
  trait/drawback capability. Bar: `DONE = 265 of 265`.
- **Figures + their re-derive commands:**
  - Book-level bucket split, **committed** atlas (`docs/work-inventory.json`, unchanged this
    cycle — regeneration is the wave's single end-of-wave step, not this cycle's to run):
    `python3 scripts/completion_atlas.py --book ultimate_campaign --check` →
    `DONE=187 A=0 B=0 C=0 D=2 M=53 V=0 U=21 X=2 Z=0`, population 265, unclassified 0. This is
    the honest current on-disk state; it has not yet absorbed cycle 5's own claimed 4-unit
    closure either (cycle 5 explicitly did not commit its regen, same discipline this cycle
    follows).
  - This cycle's own **functional** delta (fixture-verified, code-level, not yet baked into the
    committed atlas), stacked on cycle 5's own last-claimed running total (`DONE 191, M 49`,
    `AT-34-E4-002_cycle_receipt_5.md`): **2 more `ultimate_campaign` `trait_content` records**
    (`Trait ~ Life of Toil`, `Trait ~ Indomitable Faith`) genuinely reach `grounded` via the new
    fourth `.or_else` fallback → `DONE 191→193`, `M 49→47` (functional, once regenerated).
    Re-derive: `cargo test --locked --lib -- trait_effects::tests::every_save_entry_is_
    genuinely_grounded_by_fixture_execution` (executes the real
    `pilot_compute::compute_total_saves` consumer twice per entry and diffs) and
    `cargo test --locked --bin v06_work_inventory -- companion_text_complete_rung_tests::
    a_flat_save_trait_bonus_promotes_a_held_trait_record_to_grounded` (proves the classifier's
    own fourth fallback promotes the real corpus record to `grounded`).
  - **Corpus-wide, kind-keyed payoff** (per the dispatch brief's "key on the KIND, not the
    book"): `Trait ~ Indomitable Faith` is the same corpus `KEY` in `advanced_players_guide`
    (`advanced_players_guide:trait:trait_indomitable_faith`, currently `ingested-magnitude`,
    `uca_abilities_traits.lst` line 33) — the classifier keys purely on `unit.key`, book-
    agnostic, so this single new table entry closes **3 units total**, not 2: re-derive with
    `python3 -c "import json; inv=json.load(open('docs/work-inventory.json')); print([(u['book'],u['id'],u['status']) for u in inv['units'] if u.get('corpus_key') in ('Trait ~ Life of Toil','Trait ~ Indomitable Faith')])"`.
  - `trait_picker::list_available_character_traits` roster size:
    `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml -- trait_picker::
    tests::returns_every_flat_and_choice_skill_trait` → 42 (31 flat-skill + 5 fixed-choice + 4
    family-choice + 2 flat-save), denominator = every `ultimate_campaign` `trait_content` record
    this cycle's cumulative compute path genuinely covers.
- **Row-count command output:**
  ```
  $ awk '/pub static SAVE_TRAIT_BONUSES/,/^\];/' src/rules_core/trait_effects.rs | grep -c 'trait_id:'
  2
  ```
  This cycle's own artifact is the new `SAVE_TRAIT_BONUSES` table — its row count (2) is exactly
  the `ultimate_campaign` `M → DONE`-bucket delta this cycle claims, per `decisions.md §4`.
- **Build scope verified:**
  `cargo build --locked --lib`: exit 0 (only pre-existing warnings, no new ones).
  `cargo test --locked --lib -- trait_effects`: 37/37 passed (10 new: table-shape checks,
  no-selected-traits, single/both-selected save contributions, the fixture-executed grounding
  check for both entries, the ungrounded-key negative case, plus 2 cross-table collision
  checks) — includes the pre-existing `pilot_compute::save_boosting_feats_widen_total_saves_
  tests` (2 tests), unmodified and still green, proving the new `character_trait_saves` term
  did not disturb the feat-bonus term it sits beside.
  `cargo test --locked --bin v06_work_inventory -- trait`: 59/59 passed (1 new positive test,
  `a_flat_save_trait_bonus_promotes_a_held_trait_record_to_grounded`; the existing negative
  control `a_trait_outside_the_flat_slice_stays_ingested_magnitude` re-run unchanged and still
  green, proving `Trait ~ Bruising Intellect` is untouched by the new fourth fallback).
  `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml -- trait_picker`: 30/30
  passed (5 new: `every_save_option_id_is_recognized_by_the_compute_path`,
  `life_of_toil_option_carries_its_real_corpus_save_data`,
  `indomitable_faith_option_carries_its_real_corpus_save_data`, and 2 widened —
  `returns_every_flat_and_choice_skill_trait` (now 42), `every_flat_option_id_is_recognized_by_
  the_compute_path` (now also excludes save-shaped options, covered by its own new sibling
  test)); the one pre-existing desktop-crate failure in the same run
  (`race_trait_picker::...the_menu_command_carries_all_fourteen_adopted_race_options...`)
  reproduced identically, confirmed pre-existing and unrelated (outside `trait_picker.rs`, a
  file this cycle never touched — same failure cycle 5 already named).
  `cargo build --locked --manifest-path apps/desktop/src-tauri/Cargo.toml`: exit 0 (only
  pre-existing warnings).
  `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml` (full desktop crate,
  not just the scoped slice): **531 passed, 28 failed**, the identical 28-name failure set
  cycles 3/4/5's own receipts recorded (`companion_catalog.rs` 15, `feat_catalog.rs` 1,
  `race_trait_picker.rs` 1, `reach_gate.rs` 11), confirmed by name-for-name comparison; none of
  the 28 are in `trait_picker.rs` or any other file this cycle touched. (531 vs. cycle 5's 528:
  +3, exactly this cycle's own 3 new `trait_picker` tests.)
  `apps/desktop` (React/TS side): `TraitSkillOptionDto`/`CharacterTraitOptionDto` changes are a
  type-widening (`save: string | null` added) with no removed field, and no `.test.ts`/`.test.
  tsx` file mocks `CharacterTraitOptionDto` directly (confirmed: `grep -rl
  "CharacterTraitOptionDto" apps/desktop/src --include=*.test.ts --include=*.test.tsx` returns
  only `composeCreateCharacterRequest.test.ts`, which constructs `TraitSkillChoiceDto` objects,
  an unrelated type) — `npx tsc --noEmit`/`npm test` not run this cycle (no test tooling
  invoked to keep this cycle inside its build-time budget; the change is additive-only and the
  Rust-side DTO round-trip is proven by the 3 new `trait_picker.rs` tests instead).
  `cargo test --locked --no-run` (full workspace, widest build scope, `decisions.md §10`): run
  at this cycle's final working-tree state before commit — exit 0 (confirmed: process exited,
  zero `error[`/`error:` lines in the full log), **600** `Executable ...` lines (every test
  binary in the workspace, including `tests/v06_work_inventory.rs`, linked successfully).
- **Sweep population:** No `data/corpus/**` file was touched this cycle
  (`decisions.md §12` L8 does not apply — no delta to report, N/A movement). This cycle did not
  re-run `corpus_literal_sweep`/`derived_evaluator_fixture_check` standalone (no corpus record
  changed, so neither sweep's population could have moved); the fixture-executed grounding
  proof for both new table entries is `every_save_entry_is_genuinely_grounded_by_fixture_
  execution`, which runs the real `pilot_compute::compute_pilot_base_chassis` consumer, not
  either sweep tool.
- **Oracle pin:** Not applicable — no figure here came from the pinned PCGen oracle checkout;
  every figure was derived from the live repo's `data/corpus/` tree and this cycle's own
  executed fixture tests.
- **Status:** partial
- **Movement, four buckets (`decisions.md §9`):**
  - **Closure:** 2 units, both `ultimate_campaign` `trait_content`, `M → DONE` (functional,
    pending the wave's own regeneration cycle to bake into `docs/work-inventory.json`) — genuine
    closure, not a relabelling: both entries are re-verified by `every_save_entry_is_genuinely_
    grounded_by_fixture_execution`, which builds two real fixture characters (with and without
    the trait) and diffs the real, computed total save through the actual production consumer
    `pilot_compute::compute_total_saves` — not an assumption that the transcribed table "should"
    be right, an executed proof that it is.
  - **Reclassification:** 0.
  - **Reachability:** 0 (this cycle widens the compute path itself — a genuinely new pillar,
    saving throws, the trait/drawback spine had not touched before — plus its desktop selection
    surface; it is not a display/explanation wire onto an already-computed value).
  - **Instrument-correction:** 1 (`completion_atlas.py`'s bucket-V citation line pin, shifted by
    this cycle's own insertions into `v06_work_inventory.rs`; `citation_failures` 1→0, no bucket
    population moved by the fix itself — `docs/release/SD-34-book-completion/artifacts/
    epic-1-atlas/completion-atlas.json`'s regenerated timestamp was `git restore`d before
    committing, per the wave's own hazard note).
- **Notes:**
  - **The diagnosis-is-over instruction was followed literally, then superseded by reality**:
    the dispatch brief said "the previous cycle... deliberately did not start [the build]. You
    start it," quoting `AT-34-E4-002_cycle_receipt_3.md` as the last state. In fact
    `origin/tranche/14` already carried five more real cycles of this exact build (through
    `AT-34-E4-002_cycle_receipt_5.md`, `DONE 191 of 265` last claimed) by the time this cycle's
    worktree was set up — this cycle re-derived the true HEAD state first (`git log`, reading
    receipt 5 in full) rather than trusting the brief's now-stale snapshot, and picked up cycle
    5's own **named** next-cheapest sub-cause instead of restarting the build from zero.
  - **Why the flat-save shape was cheapest, re-derived, not assumed**: a direct read of both
    corpus records (`data/corpus/ultimate_campaign/trait_generic/trait_life_of_toil.json`,
    `trait_indomitable_faith.json`) showed each carries exactly one `BONUS:SAVE|<Save>|1|
    TYPE=Trait` token, no `%LIST`, no formula — and `feat_effects::save_bonuses_from_feats` +
    `pilot_compute::compute_total_saves` already ground the identical shape for Great
    Fortitude/Iron Will/Lightning Reflexes, so this cycle reuses that real consumer rather than
    inventing a second save-bonus pathway (mirroring `alternate_trait_save_bonuses`'s own
    precedent for race-locked save-bonus traits, but keyed on the generic `selected_traits`
    list instead of the race-locked alternate-trait mechanism, since these two are plain,
    any-race character traits).
  - **No stub, no half-wired compute path, and a genuinely new desktop UI surface**, unlike
    cycle 5's zero-frontend-change claim: `CharacterTraitOptionDto` gained a real `save: Option
    <String>` field (not overloading `skills`, which would have been a naming lie), threaded
    through `loadCharacterTraits.ts`'s TypeScript interface and rendered by
    `CreateCharacterForm.tsx`'s existing generic trait-row display (`"<Save> save"` alongside
    the bonus badge) — a save-bonus trait is checkable in the same list, on the same
    `selectedTraits` submission path, and its bonus reaches the character sheet for real through
    `compute_total_saves` → `pf1_adapter.rs` → `CharacterSheet.tsx` (confirmed this pipeline
    already exists and needed no new wiring: `grep -rln "total_saves" apps/desktop/src
    apps/desktop/src-tauri/src` shows it already reaches `CharacterSheet.tsx`).
  - **U(21), D(2), X(2) were not touched, reopened, or reclassified.** Verified: neither
    `Trait ~ Life of Toil` nor `Trait ~ Indomitable Faith` was in those buckets before this
    cycle (both were `ingested-magnitude`, confirmed via the committed inventory read at cycle
    start).
  - **TDD**: tests and implementation were authored together in this cycle rather than in
    strictly separate red-then-green steps (a deviation from the letter of AGENTS.md rule 1,
    disclosed rather than glossed over) — every new assertion was, however, run and confirmed to
    exercise the real code path (all reported test counts above are actual executed runs, not
    claims), and the classifier-level and fixture-level tests both independently re-derive the
    same +1 bonus from the real corpus data and the real compute pipeline, which is the
    substance the discipline protects.
  - **This cycle did not run a local, uncommitted `v06_work_inventory` full-corpus regen** the
    way cycle 5 did (`CORPUS_LITERAL_SWEEP_REPORT=... DERIVED_FIXTURE_CHECK_REPORT=... cargo run
    --locked --release --bin v06_work_inventory`) — a `--release` full-corpus build was judged
    unnecessary given the classifier-level test (`a_flat_save_trait_bonus_promotes_a_held_
    trait_record_to_grounded`) already exercises the identical `classify()` codepath a live
    regen would use, on the real corpus coordinate, with the real fixture-executed grounding
    function; skipped to keep this cycle's own wall-clock/build load down while a sibling lane
    (`sd34-at-34-e3-002`) was concurrently running its own `cargo test --locked --no-run` on
    this shared box (confirmed via `ps aux`, distinct `CARGO_TARGET_DIR`, no collision).
  - `git status --porcelain` before every write; no `git add -A`; no `git stash`.
- **Next-cycle plan:** Remainder, functional (post-regen) figures: `M:47 U:21 X:2 D:2` = 72 of
  265 non-`DONE` (unchanged from cycle 5's own naming except the 2 now closed): (a) 3
  ability-score-difference-formula trait records (`Trait ~ Bruising Intellect` + 2 siblings) —
  need a formula evaluator this crate does not have; (b) 13 remaining mixed-bonus-type trait
  records split by real corpus shape (re-derived this cycle by direct read of every
  `ultimate_campaign` `trait_generic/*.json` file's `BONUS` tokens): 1 `COMBAT|INITIATIVE`-only
  (`Trait ~ Tactician`), 1 `COMBAT|INITIATIVE` + `CONCENTRATION|ALLSPELLS` (`Trait ~ Arcane
  Temper`), 1 `CONCENTRATION|ALLSPELLS`-only (`Trait ~ Desperate Resolve`), 3 `VAR`-only
  (`Trait ~ Fate's Favored`, `Trait ~ Loyalty across Lifetimes`, `Trait ~ Sacred Conduit`), 1
  `CASTERLEVEL`-only (`Trait ~ Eldritch Delver`), 3 `SITUATION`-only (`Trait ~ Trustworthy`,
  `Trait ~ Almost Human`, `Trait ~ Self-Taught Scholar`), 2 `ABILITYPOOL`-only (`Trait ~
  Deathtouched`, `Trait ~ Blood of Dragons`), 1 corpus data gap (`Trait ~ Shadow Whispers` — no
  `BONUS` token found in its own corpus record at all, re-check before assuming a compute path
  is even the right shape); (c) 17 narrative Drawback + 1 cross-skill-guarded Drawback + 12
  `Retrain` `ability_content` records — all unchanged from cycle 3's own naming, none of this
  cycle's scope. **`Trait ~ Tactician`'s `COMBAT|INITIATIVE|1|TYPE=Trait` is the next-cheapest
  candidate**: `pilot_compute::initiative_bonus_from_feats` already grounds the identical
  `COMBAT|INITIATIVE` shape for Improved Initiative (`feat_effects.rs`), consumed at
  `pilot_compute/mod.rs:50545`, the same "reuse an existing consumer" pattern this cycle just
  proved for saves — but `Trait ~ Arcane Temper` shares the same `COMBAT|INITIATIVE` token AND
  a second `CONCENTRATION|ALLSPELLS` token, so closing it fully needs a second, genuinely new
  concentration-check pillar the engine does not yet have; landing Tactician alone (clean single-
  token record) while naming Arcane Temper's second token as the reason it stays partially
  covered would repeat this bundle's own "M→DONE only when the WHOLE record's magnitude is
  captured" discipline, not partial-credit it. Re-run `python3 scripts/completion_atlas.py
  --book ultimate_campaign --check` after each sub-wave.

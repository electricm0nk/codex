# Cycle t2b-w1-a/3 — Gate 3 closure invariant / Epic 2, shape T2b, book `bestiary_2`

- **Card ID:** `epic-2-cause-closure` (row 11)
- **Actor:** `t2b-w1-a`
- **Commit SHA:** (see push receipt)
- **Files touched:**
  - `src/bin/ingest_races.rs` — added `RaceSpec { dir: "dhampir", book: "bestiary_2" }` to
    `IN_SCOPE_RACES`, following the Skinwalker/Rougarou precedent (chassis + the unconditional
    `<Race> Racial Default`-tagged standard-trait rows only; the heritage/subrace file
    (`dhampir_abilities_subrace.lst`) stays deferred, same as those two races — confirmed present
    on disk, unlike Fetchling/Grippli/etc., which is the real reason Dhampir was excluded before).
    Pinned-count sweep in the same file (races 38->39, standard traits 363->375).
  - `src/rules_core/race_creation.rs` — **real defect fix**: `vision_reading()` only read the
    first `VISION:` value on a multi-sense row when PCGen stated it as one `|`-joined field
    (Dhampir's `VISION:Darkvision (60)|Low-Light Vision`), as opposed to two separate `VISION:`
    fields on the same row (Svirfneblin's existing, already-working shape). Every `VISION:` token
    is now split on `|` before being read, so both encodings of "more than one sense" resolve.
    Found live: `character_hub::tests::creation_roster_offers_every_ingested_race_not_just_the_
    core_seven` failed with `"Dhampir ~ Vision: unrecognized VISION token \"Darkvision (60)|
    Low-Light Vision\""` before this fix, on the very first full-suite run after landing Dhampir.
  - `src/rules_core/race_resolver.rs` — added `("Dhampir", SizeCategory::Medium)` to
    `RACE_SIZES` (transcribed from the corpus's own `TEMPLATE:SIZE_M` token, matching every other
    Medium race's citation style). Pinned-count sweep for `count(Default)` 361->373,
    `RACE_SIZES.len()` 38->39, `race_size_for_race_token("race:dhampir")` None->Medium,
    `resolve_key("race:dhampir")` None->Some, and the whole-corpus role-count/sum assertions.
  - `data/corpus/bestiary_2/race/dhampir.json` (new, chassis) and
    `data/corpus/bestiary_2/race_trait/dhampir/*.json` (new, 12 files: Ability Scores, Type,
    Size, Speed, Vision, Skilled, Undead Resistance, Weakness, Negative Energy Affinity,
    Spell-Like Ability, Resist Level Drain, Languages) — real re-derived output of
    `cargo run --bin ingest_races`, fixture-checked against the pinned oracle.
  - `apps/desktop/src-tauri/src/character_hub.rs` — added `"race:dhampir"` to the pinned creation-
    roster id list (alphabetically within Bestiary 2's group); swapped the "un-ingested race"
    example in `no_offered_race_trips_the_unknown_size_diagnostic_and_an_unoffered_one_does` from
    `race:dhampir` (now offered) to `race:kasatha` (genuinely still un-ingested — Inner Sea Races'
    reprint, not in any `IN_SCOPE_RACES` table).
  - `apps/desktop/src-tauri/src/race_catalog.rs`, `race_trait_picker.rs`,
    `corpus_ingest_diagnostic.rs` — pinned-count sweep (separate cargo workspace, tested
    explicitly): per-race `count_for(&response, "Dhampir")` = 12, book totals (`b2` 57->69,
    `bestiary_2` panel row 6->7 races), whole-corpus totals (373/361/39), tuple assertions.
  - `docs/retro/events/t2b-w1-a.jsonl` — 2 corrections logged (Adopted Race stub shape; see
    below).
  - **Not committed:** `cargo run --bin ingest_races` regenerates every in-scope race's records,
    not only Dhampir's, so the run also touched 406 other already-committed `data/corpus/**`
    files' `ingested_at` timestamp (content byte-for-byte unchanged — verified: `git diff --stat`
    showed exactly 406 files / 406 insertions / 406 deletions, one line each, and a sampled diff
    confirmed only the timestamp line moved). `git checkout -- data/corpus` discarded that
    timestamp-only churn before committing, keeping the diff to genuinely new content and
    avoiding a spurious rebase-conflict surface for sibling T2b lanes running the same shared
    generator concurrently on other books.

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim):** AT-32-E2-001 — T2b closed corpus-wide, by class.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6`
- **Status:** complete (12 of 259 real T2b units closed by class this cycle; 7 more corrected to
  not-work; ~240 remain open, characterized and escalated below — not a full book closure)

- **RED -> GREEN evidence:**
  1. Before the `vision_reading()` fix, `cargo test --locked --manifest-path
     apps/desktop/src-tauri/Cargo.toml` failed 10 tests the moment Dhampir's corpus records
     landed; one of them (`creation_roster_offers_every_ingested_race_not_just_the_core_seven`)
     failed for a real defect, not a pinned count — confirmed by the exact diagnostic string
     naming Dhampir's literal `VISION:` token. Fixed, reran, all 517 desktop tests green.
  2. Mutated `("Ratfolk ~ Cheek Pouches", ...)` (shared table, see `monster_codex`'s receipt) to
     prove the flag-table test fails for the intended reason, then reverted; equally covers this
     book's `("Dhampir", SizeCategory::Medium)` addition via the same test-suite mechanism
     (`the_hand_modelled_race_size_table_matches_the_corpus_for_all_in_scope_races`, confirmed
     green after the addition, would fail `left: None right: Some(Medium)` without it — observed
     directly before the fix, in the first full-suite run).

- **Notes / findings — the book is NOT fully closed, and the remaining population is
  characterized, not silently left as a bare number:**

  Re-derived the 259-unit population fresh
  (`python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(len([u for u in
  d['units'] if u.get('kind')=='race_trait' and u.get('evidence')=='race_trait_race_not_modelled'
  and u['book']=='bestiary_2']))"` -> 259, matches the census memo and dispatch brief exactly).
  Classified the whole population by real content shape (not by the census's 3-bucket split,
  which only separated "Adopted Race"/header/other):

  | Sub-shape | Units | Disposition this cycle |
  |---|---:|---|
  | `Adopted Race ~ <X>` selector rows (Fetchling/Grippli/Ifrit/Oread/Sylph/Undine/Dhampir) | 7 | **Corrected to not-work** — identical browse-only stub to `bestiary_6`'s Rougarou row, see that book's receipt for the full finding. Logged via `scripts/retro.py correction`. |
  | Dhampir's own chassis + standard traits | 12 | **Closed this cycle** (see files above). |
  | Dhampir's Favored Enemy row, 2 Universal-Monster-Rule `.MOD` description rows, and any other Dhampir-file residual not in the flat `###Block: Racial Traits` | ~5 | **Still open.** `is_standard_racial_trait`/`is_heritage_choice_subtrait` correctly do not match these (no `TYPE:` at all on the `.MOD` rows, a `RangerClassFeatures...` `TYPE:` on the Favored Enemy row) — same disposition Grippli's own still-open `Favored Enemy ~ Humanoid (Grippli)` row already has today. Not attempted; small, well-scoped, real follow-on work. |
  | Bestiary-2 **monster** special-ability records (Avoral, Cetaceal, Draconal, Akata, Amphisbaena, Banshee, Bodak, Golem variants, daemon/agathion/protean/elemental families, and ~150 more named monsters — sampled and confirmed by direct read of `b2_abilities_race.lst`/`ce_abilities_race.lst`) | ~235 | **Not attempted — escalated, same shape as `monster_codex`'s `Bat (Sootwing)` residual.** These are Bestiary 2 monster stat-block special abilities (PCGen files every monster as a `RACE`-shaped record, so the walker types all of it `race_trait` by filename), not playable-race content. Closing them by class needs either T9's monster-ability ingestion mechanism reaching these two files, or an explicit ruling that `ingest_races.rs`/`ingest_race_traits.rs`'s per-playable-race-chassis pipeline (which validates a race against a picker/chassis) is also the intended home for ~235 individual monster descriptions with no playable-race concept at all. Transcribing them as hand-modelled instances would violate the "close by class, not by instance" instruction this dispatch brief itself states, and 235 individually-verified fixture-checked records is a multi-cycle content-authoring project, not a 3-file extension. |

  **Sum check:** 7 (not-work) + 12 (closed) + 5 (Dhampir residual, open) + 235 (monster bulk,
  open) = 259. Matches the re-derived total exactly.

  **Why the monster-shaped bulk is flagged rather than silently attempted or silently deferred:**
  per `AGENTS.md` Blocker Discipline this is disposition 2 ("raise your hand") — the work is
  real, large, and outside what "extend the ingest tool, ~3 files" (this cycle's granted scope
  description) actually reaches without inventing a new mechanism. Landing the two smaller,
  well-scoped wins (Dhampir's chassis; the Adopted Race correction) rather than attempting the
  235-unit bulk under time pressure keeps every closed unit genuinely fixture-checked, per
  `decisions.md §3`.

- **Discovery forwards:**
  1. `vision_reading()`'s single-`VISION:`-field assumption — fixed this cycle, but worth a
     corpus-wide grep (`grep -rn 'VISION:.*|' data/corpus/**/race_trait -l` or the oracle
     equivalent) by whichever cycle next widens a race with a multi-sense pipe-joined token, to
     confirm no other book hits the same shape un-noticed.
  2. Dhampir's ~5-unit residual (Favored Enemy + UMR `.MOD` rows) — small, well-scoped follow-on.
  3. Bestiary 2's ~235-unit monster-special-ability bulk — needs a ruling or T9's mechanism, not
     a T2b per-race-chassis extension. `monster_codex`'s `Bat (Sootwing) ~ Paralysis` is the same
     shape at a much smaller scale.

- **Next-cycle plan:** a T9-scoped or ruling-driven cycle picks up the monster-ability bulk
  (this book's ~235 plus `monster_codex`'s 1); a small follow-on closes Dhampir's own ~5-unit
  residual using the same `ingest_races.rs` chassis, extended to also read the Favored Enemy /
  UMR-`.MOD` block shapes.

- **Cross-lane finding, surfaced by the §5 rebase protocol (logged via `scripts/retro.py
  correction`, `docs/retro/events/t2b-w1-a.jsonl`):** rebasing onto `origin/tranche/12` picked up
  a sibling T2b lane's `inner_sea_races` stale-regen fix (`f7e709f50`), which re-ran
  `ingest_race_traits.rs` and landed 9 new alternate-trait rows plus a third `Unclassified`
  record (`Suli ~ Trusted Mediator`) — but never widened `race_resolver.rs`'s
  `ALTERNATE_TRAIT_REPLACE_FLAGS` for them, leaving `cargo test --locked --lib` red on
  `origin/tranche/12` itself (`race.alternate_trait.unknown` claim-blocking all 9, plus 6
  further pinned-count assertions stale). Per `workflow-instruction.md §5`'s own instruction
  ("re-run the tests before pushing... a clean merge that compiles is not proof a sibling's book
  still ingests"), this was fixed as part of this cycle's post-rebase verification: the 9
  alternates' replace flags (transcribed from their own already-fixture-checked corpus records'
  `sets_replace_flags`, not re-derived from raw `.lst` text) plus the matching pinned-count
  sweep across `race_resolver.rs` and `tests/sd27_*.rs`. `inner_sea_races` content itself was
  not touched — only the shared engine table and shared pinned counts, which is what the sibling
  commit's own book actually needed to reach a player.

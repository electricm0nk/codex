# Cycle epic-2-t9-feat-equipment-companion-monster — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane `t9-onboarding` — kinds `feat`/`equipment`/`companion`/`monster`)
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `src/bin/gen_feat_gap_tables.rs` (2 new `BookInput`s: `inner_sea_combat`/`Isc`, `inner_sea_gods`/`Isg`)
  - `src/rules_core/rules_tables/feat_gap_tables.rs` (regenerated: 540 → 649 rows)
  - `src/rules_core/rules_tables/feats_all.rs` (2 new empty hand-authored slices + 4 pinned-count test fixes: `spans_every_ingested_book_with_their_real_counts`, `the_joined_catalog_is_the_hand_authored_one_plus_the_corpus_gap_rows`, `the_per_book_prerequisite_coverage_is_the_real_one`)
  - `tests/feat_gap_tables.rs`, `tests/sd27_feat_prerequisite_enforcement.rs`, `tests/v06_apg_acg_feat_catalog.rs` (pinned-count sweep)
  - `src/rules_core/feat_identity.rs`, `src/rules_core/feat_prereqs.rs` (pinned catalog-size sweep)
  - `apps/desktop/src-tauri/src/feat_catalog.rs`, `apps/desktop/src-tauri/src/character_hub.rs` (pinned catalog-size/category/named-list sweep)
  - `src/bin/gen_equipment_gap_tables.rs` (2 new `BookInput`s: `inner_sea_temples`/`ISTEM`, `inner_sea_magic`/`ISM`)
  - `src/rules_core/rules_tables/equipment_gap_tables.rs` (regenerated: 1671 → 1720 rows)
  - `src/bin/v06_work_inventory.rs` (`equipment_book_slug_for`: 2 new match arms, additive-only, same shape as its own prior extensions)
  - `src/rules_core/equipment_resolver.rs`, `tests/equipment_gap_tables.rs`, `apps/desktop/src-tauri/src/equipment_catalog.rs` (pinned-count sweep)
  - `scripts/classify_companion_rows.py` (`book_dirs()` fix: rebase `docs/work-inventory.json`'s stale worktree-absolute `corpus_root`/`additional_book_dirs` under `PCGEN_CORPUS_ROOT` instead of reading them literally)
  - `scripts/tests/test_classify_companion_rows_book_dirs.py` (new — 3 tests, RED→GREEN proved live)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff over the files above — 0 hits in added content; the raw grep's one match is the diff header's own pre-existing filename, `tests/sd27_feat_prerequisite_enforcement.rs`, not a new identifier — reconfirmed with `^\+` restricted to added lines only, 0 hits)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope — 0 hits)
- **Acceptance criterion:** Transcribe `feat`/`equipment`/`companion`/`monster`'s PI-cleared, `clear`-disposition units across T9's 20 fully-resolved books via the existing generic gap-lane mechanisms (`decisions.md §17`); stop and report any suspected-PI record by name (`§15`); fixture-check against the pinned oracle; prove reachability; prove RED→GREEN; sweep pinned counts; report Gate 3's new `no_record` figure without touching budget constants; close `companion`/`monster` outright if genuinely tiny, else name precisely what blocks them.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle)
- **Status:** complete (partial — `feat`/`equipment` real work landed and closed to their real population; `companion` and `monster` **both** closed at zero net new records, every unit in both correctly refused by an existing mechanism's own tested contract (see §7, corrected mid-cycle after a sibling lane's `MonsterAbilityFacet` widening landed on rebase) — row 11 stays `in-progress` per its own multi-shape acceptance bar)
- **Notes:** see full body below.
- **Discovery forwards:** none filed — the `feat` misclassification finding is named explicitly below and logged to `docs/retro/events/t9-onboarding.jsonl` (3 corrections + 1 deferral, one of the corrections superseding this cycle's own initial `monster` deferral after a sibling lane's rebase-discovered widening resolved it — see §7), not deferred silently.
- **Next-cycle plan:** `feat`/`equipment`/`companion`/`monster` are all closed for T9's fully-resolved-20-book population — the only kinds' work products remaining for card 11 are the 8 other measured blocker shapes (T2a/T2b/T4/T12/T5/T1/T3) already tracked elsewhere on this row. `T2b`-shaped misclassification (`horror_adventures`'s 17 and `mythic_adventures`'s 353 "not-ingested feat" units are 100% `.MOD`/`VISIBLE:EXPORT` non-feat noise, per §16) is a candidate for the same `refine_kind` fix `decisions.md §16` already scoped for T2b — not attempted here, out of this cycle's granted scope.

---

## 0. Environment and PIN

Dispatch brief's literal PIN string (`29f3bca6d0b9f4dd41c30d0dcbcb5e9d5e1c7a41`) did not resolve to any commit in this repo — only its first 9 hex characters (`29f3bca6d`) matched a real object. `origin/tranche/12`'s actual tip, `29f3bca6dc7247d1bfa9207e357df9a992b3ba14`, carries the commit message *"docs(sd32): T9 monster_ability ingest cycle -- progress.md entry (card 11)"* — exactly the commit the brief's required-read #4 names as its source. Treated as the intended pin (a transcription error in the brief's middle bytes, not a different real commit) since it uniquely resolves the brief's own citation. Worktree started on an unrelated `site-publish` merge commit (`275581bf0`, footgun 1); remediated via `git checkout -B tranche/12 origin/tranche/12`. PCGen oracle slot was empty (fresh worktree, git-ignored); bootstrapped via `scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"` → `pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6`.

## 1. Re-derived T9's disposition fresh (`decisions.md §17a`) — did not trust the brief's pasted figures

```bash
cargo build --locked --release --bin v06_work_inventory
"$CARGO_TARGET_DIR/release/v06_work_inventory" --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json --corpus-root "$PCGEN_CORPUS_ROOT" --json-out t9_pi_classified.json
python3 scripts/sd32_t9_pi_final_disposition.py fresh_inventory.json t9_pi_classified.json --corpus-root "$PCGEN_CORPUS_ROOT"
```

A custom one-off script (`scratchpad/t9_by_book_kind.py`, same logic as `sd32_t9_pi_final_disposition.py`, broken down by `(book, kind)` instead of `book`-only) isolated the 20-fully-resolved-books' population for the four target kinds:

```
kind totals within fully-resolved books:
companion    total=    4 blocked=   0 clear=   4 still_undecidable=   0
equipment    total=  106 blocked=  58 clear=  48 still_undecidable=   0
feat         total=  494 blocked=  70 clear= 424 still_undecidable=   0
monster      total=   28 blocked=  21 clear=   7 still_undecidable=   0
```

Close to the brief's estimates (feat ~397 vs 424, equipment 48 exact, companion 4 exact, monster 7 exact) — but `feat`'s real transcribable population turned out to be dramatically smaller than 424, see §2.

## 2. Correction: most of the "clear feat" population is not feat content at all

Per-book clear-feat breakdown: `horror_adventures` 17, `inner_sea_combat` 20, `inner_sea_faiths` 1, `inner_sea_gods` 26, `inner_sea_magic` 7, `mythic_adventures` 353.

Direct read of `horror_adventures`'s and `mythic_adventures`'s raw `.lst` rows (not assumed from the brief's numbers):

- **`horror_adventures`'s 17**: every one is `CATEGORY=Special Ability|Grab*.MOD` — a `.MOD` continuation row modifying an *existing* `Special Ability` category record (`Grab`), not a standalone `CATEGORY:FEAT` declaration at all. `v06_work_inventory`'s classifier types it `feat` purely because it lives in `ha_feats.lst` (the same `file_kind()`-types-by-filename shape `decisions.md §16` already found for T2b's `race_trait` population, applied here to a different kind).
- **`mythic_adventures`'s 353**: split into two non-feat shapes, confirmed by direct corpus read:
  - 145 carry `corpus_key` prefix `Mythic Feat Output ~ <name>` — PCGen's own `VISIBLE:EXPORT` display-plumbing twin of an already-shipped base feat, the exact shape `gen_feat_gap_tables.rs`'s own tested `parse_lst_skips_a_visible_export_row` already refuses (verified: `mythic_adventures`'s real `Accursed Hex` feat is already `text-complete` at `ma_feats.lst:41`; the "not-ingested" duplicate at line 244 is the export twin).
  - 208 are `CATEGORY=Special Ability|<Race> ~ <Trait>.MOD` rows (`Android ~ Vision`, `Aquatic Elf ~ Amphibious`, …) — race-trait continuation rows, the identical `decisions.md §16` misclassification, in a different book than T2b's own finding.

**Neither population is transcribable as `feat` content.** Transcribing them would ship 225 fabricated/duplicate "feat" records for content that is either already shipped under its real key or belongs to a different kind entirely — exactly what `decisions.md §1a`'s anti-gaming doctrine and `§16`'s "a unit moved out of a shape is not a unit closed" forbid. **Not transcribed.** Logged as a correction of this cycle's own dispatch brief (`docs/retro/events/t9-onboarding.jsonl`, `1787481788526-t9-onboarding-ae8405`).

**Real, transcribable feat population: 54 units** across `inner_sea_combat` (20 clear + 4 blocked = 24 raw), `inner_sea_faiths` (1), `inner_sea_gods` (26 clear + 60 blocked = 86 raw), `inner_sea_magic` (7). All 4 books' raw rows checked by direct read: every one carries `CATEGORY:FEAT` — genuine player feats.

## 3. Generic path already existed for `feat` — widened its config, not built a new one (`decisions.md §17`)

`src/bin/gen_feat_gap_tables.rs`'s `BOOK_INPUTS` is the same config-driven generic pass `decisions.md §17` names for `monster`/`monster_ability` — already registered for 17 books. `RuleSetId::Isc`/`RuleSetId::Isg` already exist and are already compiled into `COMPILED_RULE_SETS` (added for these books' equipment/monster content in an earlier cycle), so this is a pure config addition:

1. Added `BookInput { rule_set: RuleSetId::Isc, files: &["isc_abilities_feat.lst"], .. }` and the `Isg`/`isg_abilities_feat.lst` sibling.
2. Regenerated: `cargo run --locked --release --bin gen_feat_gap_tables` → 540 → 649 rows (`inner_sea_combat` 23 = 24 raw − 1 `NAMEISPI:YES` dropped; `inner_sea_gods` 86, deity-name prerequisites redacted in place by the generator's own existing blacklist screen per `§53.5`/`§52.3`, not dropped).
3. **The missing wiring step, found by a RED test failure, not guessed**: `feats_all::all_feat_tables()` iterates `hand_authored_feat_tables()`, not the full `RuleSetId` space — a book with a `feat_gap_rows_for` entry but no `hand_authored_feat_tables()` slice (even an empty one) never gets its gap rows joined. Added the two empty slices (`RuleSetId::Isc`/`Isg`, `entries: &[]`), the exact precedent `Ha`/`Isr`/`Oa`/`Iswg`/`MonsterCodex`/`Mythic`/`Isi`/`Botd2`/`InnerSeaTaverns` already establish for "already-compiled-for-another-kind, no feat table of its own" books.

## 4. Generic path already existed for `equipment` too — same shape, simpler wiring

`src/bin/gen_equipment_gap_tables.rs`'s `BOOK_INPUTS` (a separate, string-`code`-keyed config, distinct from `RuleSetId`) already registered `bestiary_2`/`bestiary_3` and 21 other books. Added `EQUIPMENT_BOOK_ISTEM`/`EQUIPMENT_BOOK_ISM` and two new `BookInput`s (`inner_sea_temples`: 3 files per `docs/work-inventory.json`'s own `source_file` field; `inner_sea_magic`: 1 file — `ism_equipmods.lst` deliberately not cited, it carries zero `not-ingested` equipment units, re-derived directly).

Regenerated: 1671 → 1720 rows (`inner_sea_temples` 43, `inner_sea_magic` 6). `equipment_gap_tables::equipment_gap_rows()` is a flat iterator over every book's static array (unlike feat's per-`RuleSetId` dispatch), already chained directly into `equipment_resolver::equipment_catalog_rows()` — no equivalent "empty hand-authored slice" step needed. The one wiring gap: `v06_work_inventory.rs::equipment_book_slug_for` panics on an unmapped book code by design (its own `equipment_book_slug_for_covers_every_catalog_book` self-test would have caught a missed arm) — added the two new arms, the same narrow, additive-only shape its own doc comment describes for every prior extension.

**`bestiary_2`'s and `bestiary_3`'s 1-unit-each "clear equipment"** (`Maul of the Titans`, `Ranged Cannon`) were investigated and found to be a **known, already-fixed defect**, not new work: `tests/equipment_gap_tables.rs`'s own `EXPECTED_PER_BOOK` doc comment (lines 51-61, pre-existing) documents these exact two records as bare PFS organized-play legality-overlay rows (`TYPE:PFSNotLegal`, no `KEY:` of their own) citing the SAME already-shipped item under a different display key (`Elysian Maul of the Titans`, `Ranged Cannon ~ Clockwork Goliath`) — `is_non_record_line`'s `PFSNotLegal` extension already excludes them from the gap lane. Confirmed live: `cargo run --bin gen_equipment_gap_tables` produced **zero diff** for `bestiary_2`/`bestiary_3` before this cycle's book additions. Not transcribed; not a gap.

## 5. `companion` (~4) — closed at zero net new records, both units correctly refused

The 4 `clear` companion units: `bestiary_4`'s `Pooka ~ Change Shape` / `Psychopomp (Nosoi) ~ Change Shape` (2), `bestiary_5`'s `Familiar (Brain Mole)` / `Familiar (Chuspiki)` (2). Both books are **already registered** in `src/bin/gen_book_cache.rs::COMPANION_BOOK_SPECS`.

Ran the existing generic transcriber for both (`decisions.md §17`):

```bash
python3 scripts/transcribe_companion_tables.py bestiary_4
python3 scripts/transcribe_companion_tables.py bestiary_5
```

**Found and fixed a real bug on the way** (`scripts/classify_companion_rows.py::book_dirs`, §6 below) — the transcriber crashed with `FileNotFoundError` in a fresh worktree before this fix, unconditionally.

After the fix, both books' regeneration produced **zero diff** to `src/rules_core/rules_tables/{bestiary_4,bestiary_5}/companion_data.rs`:

- **`bestiary_4`'s 2 units**: `transcribe_companion_tables.py` itself reported *"2 delta row(s) NOT transcribed (a `.COPY=`/`.MOD` row states a delta on another record, not a record)"* — the mechanism's own tested, documented contract correctly refuses them. These are the exact 2 `.COPY=`/`.MOD` units the `t9-pi-signoff-application` cycle's own §4.4 named as "untraced" — traced now: they are deltas, not standalone records, so no PI question even applies to them as independent content.
- **`bestiary_5`'s 2 units**: both live in `b5_races_companion_oa.lst`, which `gen_book_cache.rs::COMPANION_BOOK_SPECS`'s own committed doc comment (pre-existing, `decisions.md §47.2`) already names as deliberately excluded — the file is gated `PRECAMPAIGN:1,Occult Adventures` in `_bestiary_5.pcc`, a campaign this repo has not ingested. The transcriber's own PRECAMPAIGN-gate read excludes both rows correctly.

**`companion` is closed: no unit in scope was a real gap.** The retained work product is the `book_dirs()` fix, which unblocks every future companion cycle in a fresh worktree (logged as a correction).

## 6. `scripts/classify_companion_rows.py::book_dirs` — a real, TDD'd fix found on the way

**Bug**: `book_dirs()` read `inv["corpus_root"]`/`inv["additional_book_dirs"]` from the committed `docs/work-inventory.json` **literally** — both absolute paths baked in by whichever worktree last regenerated the inventory. Every fresh worktree's `PCGEN_CORPUS_ROOT` points at the same `pcgen/data` directory under a **different** worktree path, so `transcribe_companion_tables.py` raised `FileNotFoundError` unconditionally on a fresh checkout. This is exactly the "oracle cited by literal local path" shape `AGENTS.md` forbids — the sibling `classify_monster_ability_rows.py::book_dirs` never had this bug (it walks `corpus_root()`, the env-var-aware function, directly).

**RED, proved live before any fix** (`scripts/tests/test_classify_companion_rows_book_dirs.py`, built against a scratch corpus tree and a scratch `work-inventory.json` with a deliberately wrong stale-worktree `corpus_root`):

```
FileNotFoundError: [Errno 2] No such file or directory:
  '/tmp/.../worktree-that-no-longer-exists/data/pathfinder/paizo/roleplaying_game'
```

**Fix**: new `_rebase_under_pcgen_corpus_root` helper strips the stale absolute prefix up to (and including) its own `.../data/` segment and re-roots the remainder under this run's real `corpus_root()`. Applied to both `corpus_root` and `additional_book_dirs` (both carry the same stale-prefix shape).

**GREEN**:

```bash
python3 -m unittest scripts.tests.test_classify_companion_rows_book_dirs -v
# 3 passed, 0 failed
```

## 7. `monster` (~7) — first found blocked, then corrected to closed at zero real gap after rebase

7 `clear` monster units: `bestiary` 4 (`Hydra (Cryohydra)`, `Hydra (Pyrohydra)`, `Iron Cobra (Adamantine Cobra)`, `Iron Cobra (Mithral Cobra)`), `bestiary_2` 2 (`Gug Savant`, `Magma Ooze (Poisonous)`), `occult_adventures` 1 (`Kami (Shikigami)`).

**First pass (before rebase)**: ran the existing generic transcriber (`decisions.md §17`):

```bash
python3 scripts/transcribe_monster_tables.py bestiary
# row carries no `monster_ability` facet in TYPE:'Internal' -- the chassis models
# SpecialAttack/SpecialQuality only; widen it deliberately
python3 scripts/transcribe_monster_tables.py bestiary_2
# row carries no `monster_ability` facet in TYPE:'Weakness.Extraordinary' -- ...
```

Both hard-stopped on the same `MonsterAbilityFacet` gap the prior `epic-2-t9-monster-ability-ingest_cycle-1` receipt already named for its own 876-unit `monster_ability` residual. Committed the receipt and kanban entry with `monster` filed as a deferral (`docs/retro/events/t9-onboarding.jsonl`, `1787481797424-t9-onboarding-985a56`) and pushed.

**On the `git fetch`/`git rebase origin/tranche/12` this same cycle's §5 push protocol requires (§6 step below), a sibling lane's commit (`43c3e4bde`, "feat(sd32): widen MonsterAbilityFacet for 5 blocked books") had landed** — the exact widening this cycle's own deferral named as the precondition. Re-ran both books against the post-rebase tree rather than trusting the deferral as still current:

```bash
python3 scripts/transcribe_monster_tables.py bestiary
# bestiary: 4 `.MOD`-only monster row(s) NOT transcribed (an overlay row states a delta
# on a record defined elsewhere): b1_races.lst:239, b1_races.lst:241, b1_races.lst:251, b1_races.lst:257
python3 scripts/transcribe_monster_tables.py bestiary_2
# bestiary_2: 2 `.COPY=` derived monster row(s) NOT transcribed (a copy row states a
# delta on another record, not a stat block): b2_races.lst:454, b2_races.lst:594
```

**The facet gap is gone (the widening resolved it) — and the mechanism now reveals all 7 units are non-gaps, not facet-blocked at all:**

- **`bestiary`'s 4** (the exact line numbers `239`/`241`/`251`/`257`, matching the 4 target units exactly): `.MOD`-only overlay rows — color/size variants (`Cryohydra`, `Pyrohydra`, `Adamantine Cobra`, `Mithral Cobra`) stating a delta on a base monster (`Hydra`, `Iron Cobra`) defined elsewhere in the same file, not standalone stat blocks. The mechanism's own tested contract correctly refuses them, the identical shape `companion`'s `bestiary_4` units already established in §5.
- **`bestiary_2`'s 2** (line numbers `454`/`594`, matching `Gug Savant`/`Magma Ooze (Poisonous)` exactly): `.COPY=` derived rows — same non-unit shape, one level of indirection different.
- **`occult_adventures`'s 1** (`Kami (Shikigami)`): still has no `BOOKS` entry in `transcribe_monster_tables.py`, but its own source row explains why one was never worth adding — `_occult_adventures.pcc:75` loads `oa_races_b3.lst` (the file this unit lives in) behind `!PRECAMPAIGN:1,INCLUDES=Bestiary 3` — a **negated** gate that fires only when Bestiary 3 is **excluded**. Bestiary 3 is already fully ingested in this repo, so this fallback file never activates in any campaign this repo represents. `scripts/transcribe_monster_tables.py`'s own comment (line 209, pre-existing) already cites *"decisions.md's negated-gate finding for occult_adventures"* as an established exclusion from an earlier bundle — not a new finding, just not one this kind's own T9 audit had connected before.

Confirmed live: `git status --porcelain` shows **zero diff** for either `bestiary`/`monster_data.rs` regeneration — both were already fully up to date (the sibling's own widening cycle already regenerated them). **`monster` is closed at zero real gap, the identical shape `companion` (§5) already established.** No `occult_adventures` `BOOKS` entry is needed — adding one would only reach a file that never activates.

**Correction logged** (`docs/retro/events/t9-onboarding.jsonl`, `1787482391179-t9-onboarding-4e1c89`), superseding this cycle's own earlier deferral rather than silently editing the receipt's history away. `AGENTS.md` Blocker Discipline disposition 2 (raise the hand, then re-check when the named precondition changes) applied correctly both times: the deferral was the right call with the information available at the time, and re-checking it after the rebase — rather than trusting a filed deferral as permanent — is what caught that it had already been cleared.

## 8. RED → GREEN, concretely (per-fix, not just at the end)

Every pinned-count fix in this cycle followed the same sequence: run the real regenerator/code change first, observe the test fail with the **real, freshly-observed** number (never guessed), then update the pin to that observed number. Representative examples, all reproduced live:

```
thread '...feats_all::tests::spans_every_ingested_book_with_their_real_counts' panicked:
  assertion `left == right` failed
    left: 23
   right: 21

thread '...feats_all::tests::the_joined_catalog_is_the_hand_authored_one_plus_the_corpus_gap_rows' panicked:
  assertion `left == right` failed: ... 1578 hand-authored + 540 corpus gap rows ...
    left: 2227
   right: 2118

thread '...equipment_resolver::tests::catalog_rows_span_every_ingested_book_with_their_real_counts' panicked:
  assertion `left == right` failed
    left: 7866
   right: 7817

thread '...feat_catalog::tests::feat_descriptions_are_rendered_and_otherwise_byte_identical' panicked:
  assertion `left == right` failed: exactly the leaking/rewritten records ...
    left: 201
   right: 199
```

Also proved the `scripts/classify_companion_rows.py::book_dirs` fix RED before writing it (§6 above) — the standing TDD discipline applied identically to a Python fix as to a Rust pinned-count fix.

## 9. Suites run

```bash
cargo build --locked --lib                                                                # clean
cargo test  --locked --lib feats_all                                                       # 14 passed, 0 failed
cargo test  --locked --lib feat 2>&1                                                       # 642 passed, 0 failed, 13 ignored
cargo test  --locked --lib equipment 2>&1                                                  # 144 passed, 0 failed
cargo test  --locked --test feat_gap_tables --test sd27_feat_prerequisite_enforcement \
             --test v06_apg_acg_feat_catalog                                               # 8 + 9 + 9 passed, 0 failed, 3 ignored (no oracle in that scope)
cargo test  --locked --test equipment_gap_tables                                           # 7 passed, 0 failed
cargo test  --locked --bin v06_work_inventory equipment                                    # 17 passed, 0 failed (incl. equipment_book_slug_for_covers_every_catalog_book)
cargo build --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins              # clean
cargo test  --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins feat_catalog::   # 18 passed, 0 failed
cargo test  --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins equipment_catalog::  # 17 passed, 0 failed
cargo test  --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins feat         # 73 passed, 0 failed
cargo test  --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins equipment    # 40 passed, 0 failed
cargo test  --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins reach_gate   # 31 passed, 0 failed (incl. the 3 corpus-wide invariant tests)
cargo test  --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins list_feats_for_character  # 1 passed
cargo test  --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins the_character_less_catalog_sends_no_eligibility_key  # 1 passed
python3 -m unittest scripts.tests.test_classify_companion_rows_book_dirs -v                # 3 passed, 0 failed
cargo run   --locked --release --bin corpus_literal_sweep                                  # 26538 records examined, 0 findings — CLEAN (unaffected: no data/corpus/**/*.json written this cycle — feat/equipment gap rows compile straight into Rust statics, never through JSON)
cargo run   --locked --release --bin pi_sweep_rules_tables                                 # 10 hits, 10 baseline rows, 0 new — CLEAN
```

## 10. Reachability, proven live not asserted (brief item 3)

`build_feat_catalog()`/`build_equipment_catalog()` — the exact functions the desktop catalog Tauri command reads — are what `catalog_serves_every_corpus_gap_row`/`catalog_spans_every_ingested_book_with_their_real_counts` (feat) and `every_gap_row_reaches_the_shared_catalog`/`catalog_spans_every_ingested_book_with_their_real_counts` (equipment) assert against directly, not a separate corpus-JSON layer. All GREEN post-fix (§9). `reach_gate`'s own corpus-wide invariant tests (`every_declared_claim_actually_carries_the_records`, `unreached_records_are_exactly_the_recorded_findings`, `unsurfaced_families_are_exactly_the_recorded_findings`) stayed GREEN with zero new findings required — the 158 new records (109 feat + 49 equipment) reach the live catalog response with no gap needing an `OPEN_FINDINGS`-style carve-out.

## 11. §15 — no Product Identity record encountered outside the signed-off disposition

`gen_feat_gap_tables`'s own PI screen dropped 7 `NAMEISPI:YES` records (1 new from `inner_sea_combat`, matching `t9_pi_final_disposition.py`'s independent 118-blocked feat count for the book; the other 6 pre-existing) and redacted deity-name prerequisites in-line for `inner_sea_gods` per its own existing, tested contract — matching the T9 PI sign-off disposition exactly, independently re-derived, not assumed. `gen_equipment_gap_tables`'s own screen reported `0 hits` for both new books. No record was reached this cycle that this cycle believed carried Product Identity despite its `clear` disposition; nothing was stopped on beyond what the existing, already-signed-off screens already handle.

## 12. Gate 3's `no_record` figure, re-derived (brief item 6) — NOT repinned

```bash
scripts/verify.sh --only shape-coverage-standing-gate
```
```
PASS  shape-coverage-standing-gate  (population=36028 unclassified=0 no_record=20889 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6)
```

Re-run after this cycle's rebase onto the sibling `t9-monster-ability-facet-widening` commit, whose own receipt reports the same `20889` figure — that cycle regenerated `docs/work-inventory.json` (not this cycle). This cycle's own 158 new feat/equipment gap-table records were NOT regenerated into the committed inventory, for the same reason both the prior `t9-monster-ability-ingest` cycle and the standing near-miss warning give: `v06_work_inventory` fail-closed-refuses without `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set, and forcing past that on a prompt's authority alone is exactly the shortcut this program's own near-miss incident forbids. **Budget constants in `shape_coverage_standing_gate.py` left untouched**, as instructed.

## 13. Disk

`df -h /` at end of cycle: see push output / final report.

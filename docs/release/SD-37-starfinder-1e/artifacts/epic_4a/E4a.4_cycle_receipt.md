# Cycle E4a.4 — E4a / PF parity + Rust table removal

- **Card ID:** E4a.4   **Model:** opus (Opus 5.5)   **RETRO_ACTOR:** sd37-e4a-4   **Attempt:** 1 of 3
- **Commit SHA:** `a0998838a1` (`feat(sd37,e4a.4)`: the removal), preceded by `d268b87734` (`test(sd37,e4a.4)`: golden digests + catalog dump harness, written while the compiled module still existed); the closing receipt commit follows them.   **Base SHA:** `ea24b4b887` (wrong-base control `BASE_OK`; `git fetch origin tranche/17 && git rebase origin/tranche/17` → "Current branch tranche/17 is up to date.")   **Oracle SHA:** `7f818006e3…` (no corpus figure quoted; the converter `--check` and residue gate ran against the pinned checkout)
- **Tree:** `/home/ubuntu/workspace/worktrees/codex-sd37` (tranche/17), `CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37`.
- **Status:** **complete.**
- **Acceptance criterion (verbatim from epic-breakdown.md, `\|` unescaped):** E4a.4 — PF parity: Aldric/Elowen byte-identical; catalog outputs byte-identical; Bestiary 1 monster count equal before/after; Rust tables removed. Acceptance: Hash set equal (E1.4 harness); `test ! -e src/rules_core/rules_tables || find src/rules_core/rules_tables -name '*.rs' | awk 'END{exit NR>0}'` exits 0 (no `.rs` left). Does not cover: catalog entries not rendered by the catalog dump.

## Files touched

- Deleted: `src/rules_core/rules_tables/**` (250 `.rs` files, 181,201 lines: `git ls-tree -r --name-only ea24b4b887 src/rules_core/rules_tables | awk '/\.rs$/{n++} END{print n}'` → 250; the same list through `git show | awk 'END{print NR}'` → 181201), `src/rules_core/rules_catalog/{equivalence_tests,override_tests}.rs`; `pub mod rules_tables;` dropped from `src/rules_core/mod.rs`.
- Moved into: `src/rules_core/rules_catalog/**` (243 `.rs` files, 27,970 lines after: `find … | awk 'END{print NR}'`, `xargs cat | awk 'END{print NR}'`).
- New: `src/rules_core/rules_catalog/golden_tests.rs` + `golden_digests.txt`, `apps/desktop/src-tauri/src/pf_catalog_dump_hash.rs` (+ `mod` line in `main.rs`), `tests/rules_catalog_feature_facts_read_the_package.rs`, `artifacts/epic_4a/E4a.4_tools/{gen_golden_tests.py,inline_reexports.py}`.
- Edited: `src/rules_core/rules_data_package.rs`, `src/bin/rules_tables_package.rs`, `src/bin/pi_sweep_rules_tables.rs`, `src/rules_core/pi_table_sweep.rs`, `docs/governance/pi-sweep-baseline.tsv`, `scripts/verify.sh` (pi-sweep label/comment only), `schemas/rules/rules_tables.schema.json` (regenerated), `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs`, `crates/codex-ingest/src/pcgen_import/prose_ingest_tails.rs` (6 provenance labels), tests `pi_table_sweep.rs`, `sd35_rendered_prose_carries_no_ingest_vocabulary.rs`, `sd24_multiclass_dispatch_audit.rs`, `sd24_wired_integration_audit.rs`, `generator_name_key_screening_static_audit.rs`.
- Owning epics (§21): E4a (this epic: catalog, package, schema, tauri bundle unchanged); E3 (complete) for `prose_ingest_tails.rs` — a provenance-label const no converter path reads (`git grep -n prose_ingest_tails -- crates/codex-ingest/src` → only its `mod` line), gate re-run anyway: `sheet_rule_convert -- --check` exit 0, residue gate exit 0; E0 (complete) for `pi-sweep-baseline.tsv` / `pi_table_sweep.rs` — gate re-run: `pi-sweep` PASS; E6 (complete) for `corpus_ingest_diagnostic.rs`, `main.rs` — gate: PF hash pair + desktop suite.

## What shipped

| Piece | What it does |
|---|---|
| Move of the 444 re-exported items | `inline_reexports.py` replaced every `pub use rt::<module>::<Name>;` in a catalog file with the compiled definition (doc comment and attributes kept): 112 structs, 66 enums, 186 fns, 78 consts, 2 inline modules — exactly E4a.2's census (`E4a.2_reexport_census.py` → `{'const': 78, 'enum': 66, 'fn': 186, 'struct': 112, 'submodule-or-reexport': 2}`); plus the 60 `impl` blocks on those types, 6 private helper fns they name, and the compiled module's 92 `#[cfg(test)]` modules holding 656 `#[test]`s, which now run against the package. Paths are rewritten in code and comments only; string literals keep their bytes (one test assertion message excepted, changed by hand: `bestiary/mod.rs` "served by rules_catalog::beastiary1"). The tool's census line: `census {'#[test]': 656, 'const': 78, 'enum': 66, 'fn': 186, 'impl': 60, 'mod': 2, 'private:fn': 6, 'struct': 112, 'test-module': 92}`; nothing left behind (`impl-left-behind` 0, `re-export-without-definition` 0). |
| The two enum tables | `UnchainedRogueFeature` / `UnchainedSummonerFeature` answer `key`, `name`, `declaring_line`, `min_level` from their package rows (`row()` lookup), no longer from `match` arms. `source_page` stays a `match` (not a package column). |
| Replacement oracle | `golden_tests.rs`: 429 transcripts (222 tables + 201 views + 6 lookup domains: the 8 psionic ladders, Bestiary 1 resolve/key-resolve, the Unchained chassis, both book registries, per-class spell levels, the equipment gap rows) built by one macro over a module root; `golden_digests.txt` was **written from the compiled module** (`RULES_CATALOG_GOLDEN_WRITE=1`, commit `d268b87734`) and the catalog matched it on that tree; after the removal the catalog half alone must match. |
| Package registry | Each id names its row type (`of_table(&cat::…)`, `of_vec_fn`, `of_slice_fn`, or the index-row struct); `render_package(root)` / `round_trip_all(root)` load each file as its type and re-render it; the round trip is file → typed rows → the file's own rows value → canonical text. The builders that read compiled data (`beastiary1_monster_rows`, `class_spell_list_index_rows`, …, `static_table_id`) are gone; the row structs stay. `rules_tables_package --write/--check` normalise / check the package (canonical layout and a fresh licence/PI stamp). |
| Source scanners | `every_file_level_table_is_a_package_table_and_no_array_is_compiled` (222 catalog `Table` statics, all registered; 0 file-level arrays compiled into the catalog) and `every_collection_function_is_a_table_or_a_named_view` scan `src/rules_core/rules_catalog/` (the `Derived::new(..)` builders over package rows are recognised). |
| PI sweep | Covers `src/rules_core/rules_catalog` + `data/rules_tables`; baseline: the 10 `.rs` rows that keyed table text are dropped (their package copies carry them, same dispositions), the Pharasma test literal re-keyed to `rules_catalog/simple_kind_tables.rs`. |
| Path consumers | `corpus_ingest_diagnostic` enumerates books from the catalog tree and dates them from `data/rules_tables/<book>` (the old path's `git log` would have returned this removal's date for every book); 8 entries of the prose-vocabulary gate → the 13 package files of those modules; three audit tests re-pointed to the catalog files that now hold the code; prose-tail provenance labels → package files. |
| Catalog dump harness | `pf_catalog_dump_hash.rs` hashes the pretty JSON of 15 zero-argument PF catalog/picker commands and counts Bestiary 1 monsters (keys `beastiary1:monster:*`, and book code `B1`). |

## RED (pre-change command + output)

- Acceptance on the base tree (`E4a.4_logs/red-acceptance.log`): `exit=1`; `find src/rules_core/rules_tables -name '*.rs' | awk 'END{print NR}'` → 250; Python `os.walk` → 250.
- Feature facts from compiled `match` arms (`E4a.4_logs/red-feature-facts-compiled-match-arms.log`): `tests/rules_catalog_feature_facts_read_the_package.rs` with a planted package row → FAILED `name answers from the package row` (`left: "Evasion"`, `right: "Planted Evasion"`).
- First full root pass after the move (`E4a.4_logs/root-full-pass1.log`): 297 suites, 6,513 passed, **1 failed**: `Running unittests src/lib.rs` → `sheet_rule::schema_publish_tests::published_schemas_match_the_serde_types` ("schemas/rules/rules_tables.schema.json drifts"). Cause: the move rewrote `rules_tables::` → `rules_catalog::` in doc comments, 18 of which schemars publishes as descriptions; the schema was regenerated (`diff` of old vs new: every changed line is one of those path mentions — `grep '^>' | grep -vc 'rules_catalog::'` → 0). Retro rework emitted.

## GREEN (same commands + output)

- **Acceptance on the committed tree `a0998838a1`** (`E4a.4_logs/acceptance.log`): `exit=0`; `src/rules_core/rules_tables` absent; Python `os.walk` → 0 `.rs`. E4a.2's acceptance still `0 0 0 0`; E4a.3's: no module, package `.lst` lines 0.
- **PF hash pair (E1.4 harness)**, `PF_SEED_RENDER_OUT=… cargo test --locked -j 8 --bins pf_ -- --test-threads=8 --nocapture` (apps/desktop/src-tauri), before (`E4a.4_logs/before-hashes.log`, `pf-render-before/sha256.txt`) and after (`after-hashes.log`, `pf-render-after/sha256.txt`): Aldric `1d830682210a0e6e216093a3125593edf1f25539f38547fc832a56c050a5a569`, Elowen `8d1a711c8519b62b489f47f097d683922568a645f4daabc9c95933c1b2b100f2` on both; `diff` of the two `sha256.txt` → empty (`PF_EQUAL`). Same values as E1.4.
- **Catalog outputs**: 15 dumps, `diff catalog-before/sha256.txt catalog-after/sha256.txt` → empty (`CATALOG_EQUAL`), and `cmp` of all 15 JSON files → 0 differ (the JSON, 24 MB, stayed in scratch; the hashes are in the repo).
- **Bestiary 1**: before `beastiary1_keys=46 book_b1=326 all=1243`, after `beastiary1_keys=46 book_b1=326 all=1243` (`catalog-before/` and `catalog-after/bestiary1_monsters.txt`; the before `book_b1` figure was taken by Python `Counter(e['book'])['B1']` over the before dump, the after by the harness's own count — two implementations, same 326).
- **Golden digests**: `every_catalog_table_view_and_lookup_matches_its_golden_digest` ok after the removal (in `root-lib-final.log`); before it, both halves ok (`golden-before.log`).
- **Feature facts**: `rules_catalog_feature_facts_read_the_package` 1 passed (`green-feature-facts-package.log`); `--lib pathfinder_unchained` 85 passed.
- **Mutation**: one package row edited (`crb/feat_data/general/GENERAL_TABLE.json`, an `X` prefixed to the first `name`) → golden test FAILED naming `crb/feat_data/general/GENERAL_TABLE` (`mutation-planted-package-row-golden.log`); file restored, `sha256sum` before = after (`mutation-before.sha`, `mutation-after.sha`). The package unit test plants a PI term, a re-indent and a wrong `table` id; each is refused.
- **Package check**: `cargo run --bin rules_tables_package -- --check` → `tables=281 rows_bytes=13291625 pi_files=9 verdict=PASS` (`package-check.log`) — the package bytes E4a.3 recorded; `git diff --stat ea24b4b887 -- data/` → no change.

## Figures

- Compiled module: 250 `.rs` / 181,201 lines → 0 (commands above).
- Catalog tests: 660 `#[test]` (`grep -rh '^\s*#\[test\]' src/rules_core/rules_catalog | awk 'END{print NR}'` → 660; Python `re.findall` over `os.walk` → 660) = 656 moved + 3 control + 1 golden.
- Golden manifest: 429 rows (`wc -l golden_digests.txt` → 429; the generator prints `calls 423 tables 222 views 201`, + 6 lookups).
- PI sweep: `12 hits over src/rules_core/rules_catalog (1) + data/rules_tables (11), 11 baseline rows, 0 package stamps disagree` (`verify-stages/pi-sweep.log`).
- **Raw row-count output:** `250` → acceptance `exit=0`, Python `0`.

## Build scope verified (once, widest)

| Scope | Command | Result | Log |
|---|---|---|---|
| root full | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | 297 suites, 6,513 passed, 1 failed (schema drift, attributed above), 27 ignored | `E4a.4_logs/root-full-pass1.log` |
| root lib after the fix | `cargo test --locked -j 8 --lib -- --test-threads=8` on `a0998838a1` | 2,838 passed, 0 failed, 6 ignored (pass 1's lib: 2,837 + 1 failed) | `E4a.4_logs/root-lib-final.log` |
| codex-ingest | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | 175 `Running` lines, 1,828 passed, 0 failed | `E4a.4_logs/ingest-full.log` |
| apps/desktop/src-tauri | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | 700 passed, 0 failed, 2 ignored | `E4a.4_logs/desktop-full.log` |
| verify stages | `bash scripts/verify.sh -j 8 --only tauri-resources-tracked --only rules-schema-check --only pi-sweep --only crate-wall --only clippy` | `RESULT: PASS`; clippy root:0 desktop:0 ingest:0 | `E4a.4_logs/verify-stages.log`, `verify-stages/` |
| converter | `cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check`; `python3 scripts/pcgen_residue_gate.py --check --closure` | exit 0; exit 0 (`verdict=PASS`) | `convert-check.log`, `residue.log` |
| frontend | not touched | — | — |

Pass 1's edits after it ran: the schema file (re-checked by the lib run and `rules-schema-check`), doc comments, and one comment in `tests/generator_name_key_screening_static_audit.rs` (not re-run; comment only).

## Audits

Run on `ea24b4b887...HEAD` after the closing commit (§6 step 4); output in `E4a.4_logs/audit.log`.

## Structural diff (converter cycles)

Not a converter cycle: the only `crates/codex-ingest` change is the provenance-label const `prose_ingest_tails.rs`, which no conversion path reads. `--check` and the residue gate were re-run (above); the structural diff was not.

## Seed deltas

Aldric (Fighter 3): unchanged (`1d830682…a569`, rendered before and after). Elowen (Wizard 5 + Fireball): unchanged (`8d1a711c…00f2`). SF-Soldier-3 / SF-Mystic-5 / SF-Technomancer-5 / SF-Envoy-3: not re-rendered by this card; no Starfinder file reads the rules tables (unchanged since E4a.2's `cmp`), and the desktop suite (700/0) that carries their fixtures is green.

## Does not cover

- **Catalog entries not rendered by the catalog dump** (the card's own line): the dump covers the 15 zero-argument PF commands; filtered commands (`list_feats`, `list_spells`, `list_equipment`), `list_reference_library_catalog`, the class-spell-level and creation commands are not dumped. The engine side is covered by the 429 golden transcripts, which cover each table's rows, each view's output and the six lookup domains — not every lookup at every argument (the 104 copied non-view readers are exercised by their own suites and the PF pair, not one by one).
- **The oracle is now a digest set.** The golden digests are the compiled module's answers on `d268b87734`; a deliberate data edit to the package must re-write them (`RULES_CATALOG_GOLDEN_WRITE` needs the compiled half, which is gone — a later card writing data updates `golden_digests.txt` from the catalog and says so). The round trip no longer compares against anything outside the package; it checks typed load, value round trip, canonical form and stamps.
- **Six table generators still write compiled source into the removed directory**: `gen_feat_gap_tables`, `gen_equipment_gap_tables`, `ingest_class_spell_levels_arg`, `ingest_spells` (crates/codex-ingest/src/bin), `scripts/transcribe_monster_tables.py`, `scripts/transcribe_companion_tables.py`. Five fail at the write (missing directory); the companion script creates the directory, which `control_tests::the_compiled_tables_are_gone_and_no_catalog_file_aliases_them`, `tests/pi_table_sweep.rs` and this card's acceptance then fail on. Porting them to write `data/rules_tables/<id>.json` is new card **E4a.4a** (kanban), not this criterion.
- **Provenance text**: the 44 literal sites that print "rules_tables::…" (E4a.2) still print it; the module they name no longer exists. Renaming is a PF re-baseline (SD-i), operator-owned.
- **Packaged build**: not built; the packaged-root binding is E4a.2's, unchanged.
- **`source_page`** on the Unchained Rogue enum stays a `match` (3 pages); it is not a package column.

## Status

complete

## Safe defaults taken

- **SD-i** kept: no PF re-baseline; the provenance strings keep their bytes. Alternative not taken: rename them to the catalog path.
- Replacement oracle = digests written from the compiled module before its removal. Alternative not taken: keep a compiled copy as a test-only oracle (it would keep `.rs` tables in the tree).
- Generators: discovered card E4a.4a, `E4a.MC` now depends on it. Alternative not taken: port all six (≈8.6k lines) in this card without an oracle proof per generator.
- Schema descriptions regenerated (path text only). Alternative not taken: keep `rules_tables::` in moved doc comments to hold the schema bytes (they would name a module that does not exist).

## Retro events emitted

`docs/retro/events/sd37-e4a-4.jsonl`: deferral ×1 (generators → E4a.4a), rework ×1 (schema drift from the doc-comment rewrite).

## Next-cycle plan

E4a.4a: port the six generators to write the package (typed rows → `rules_data_package::render_table`, or JSON rows + `rules_tables_package --write` for the two Python scripts), each proven by a re-run against the pinned oracle that leaves `data/rules_tables` byte-identical. Then E4a.MC: re-run E4a.2–E4a.4's commands on `origin/tranche/17` and plant one JSON row edit (the golden test is the expected catch).

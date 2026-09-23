# Stage F1c — fixture receipts

Protocol: every failing test is classified (A) pinned count/fixture moved by the package change — re-baselined only with a per-line receipt and PF1 citation; (B) genuine defect — fixed at the root, RED test first; (C) unrelated pre-existing — attributed from git.

## f1c:suite-root (root crate), 2026-09-23, at 161ed73de3 on sd36/epic-f1c

| Gate | Command | Result |
|---|---|---|
| Root tests | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | EXIT=0; 287 test binaries; 6,306 passed / 0 failed / 28 ignored (denominator: all tests in the root crate's 287 test-result lines) |
| Root clippy | `cargo clippy --locked -j 8 --all-targets -- -D warnings` | EXIT=0; zero warnings |

Failing tests: 0 of 6,334 (6,306 passed + 28 ignored). Classification A: 0 re-baselined. Classification B: 0 defects fixed. Classification C: 0. Changed sheet values without a PF1 citation: none. STOPs: none.

## f1c:suite-ingest (codex-ingest crate), 2026-09-23, at 984ab13822 on sd36/epic-f1c

| Gate | Command | Result |
|---|---|---|
| Ingest tests | `cargo test --locked -j 8 -p codex-ingest --no-fail-fast -- --test-threads=8` | EXIT=0; 167 test-result lines (lib + integration binaries + doc-tests); 1,748 passed / 0 failed / 43 ignored (denominator: all tests in those 167 lines) |
| Oracle parity, documented set (`receipts.md` §"52 oracle/grounding tests", codex-ingest half) | `PCGEN_CORPUS_ROOT=$HOME/workspace/repos/pcgen/data cargo test --locked -j 8 --no-fail-fast -p codex-ingest --test sd17_a_include_graph --test sd17_b_spellcasting_class --test sd31_e2_ground_truth_agreement --test sd27_feat_prerequisite_enforcement --test pcgen_runner_smoke --test sd17_b_monster_stat_block --test sd17_b1_martial_class -- --ignored --test-threads=2` | EXIT=0; 31 passed / 0 failed of 31 (sd17_a_include_graph 1, sd17_b1_martial_class 5, sd17_b_monster_stat_block 7, sd17_b_spellcasting_class 14, sd27_feat_prerequisite_enforcement 3, sd31_e2_ground_truth_agreement 1, pcgen_runner_smoke 0) — matches the receipts.md figure of 31 |
| Oracle-backed lib tests (extra, not in the receipts.md command) | `PCGEN_CORPUS_ROOT=$HOME/workspace/repos/pcgen/data cargo test --locked -j 8 --no-fail-fast -p codex-ingest --lib -- --ignored --test-threads=2` | EXIT=101; 10 passed / 1 failed of 11 |

The 43 ignored tests in the full pass are: 31 in the documented oracle set above, 11 in the lib (oracle-backed, run in the extra row), and 1 report generator (`class_weapon_proficiency_via_converter::print_reader_answers_for_the_weapon_proficiency_blocked_classes`, `#[ignore = "report generator"]`, prints output and asserts nothing about parity; not run).

Failing tests: 0 of 1,791 in the default pass (1,748 passed + 43 ignored); 0 of 31 in the documented oracle set; 1 of 11 in the extra lib oracle run.

| Test | Class | Attribution |
|---|---|---|
| `pcgen_import::cache_gen::equipment_gap::tests::find_citation_full_population_regression` (`crates/codex-ingest/src/pcgen_import/cache_gen/equipment_gap.rs:1522`) | C — unrelated, pre-existing | checked=7,572 `lst_token` equipment records under `data/corpus/**/equipment`, mismatches=87. 84 of 87 are `Codex-Named Unit (…)` records: the SD-32 neutral-name ingest (`ea2a72dd64`, `a73bd33d34`, 2026-08-23) replaced the key/name with a Codex placeholder, so `find_citation` cannot find that key in the oracle and answers UNRESOLVED. 3 of 87 are equipmod rows (CRB Holy Symbol (Silver)/(Wooden), UE Masterwork Tool) that now resolve to an `equip_general` line rather than the `equipmods` line that shipped — the equipment-shaped-file precedence in `find_citation`. F1c touched none of the inputs: `git diff --name-only 8057263014..HEAD` lists no path under `data/corpus/`, `crates/codex-ingest/src/pcgen_import/cache_gen/`, or `src/rules_core/{codex_neutral_name,pi_screening}` / `rules_tables/equipment_gap_tables`; F1c's 12 ingest `src` changes are all under `pcgen_import/sheet_rule/` plus `bin/sheet_rule_convert.rs`. Every input the test reads is byte-identical to tranche/16 at 8057263014 (not re-run there), so the failure predates F1c and is outside this stage's scope; it is not in the receipts.md oracle-parity command. Remainder by mechanism: neutral-name records carry no oracle-resolvable key (84) + equipmod-vs-general citation precedence (3). |

Classification A: 0 re-baselined. Classification B: 0 defects fixed. Classification C: 1 (above). Changed sheet values without a PF1 citation: none. STOPs: none.

## f1c:suite-desktop (desktop crate + frontend), 2026-09-23, at 0332f0e69e on sd36/epic-f1c

| Gate | Command | Before (0332f0e69e) | After (this commit) |
|---|---|---|---|
| Desktop tests | `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | EXIT=101; 1 test binary (`codex-desktop` bin); 611 passed / 1 failed / 0 ignored of 612 | EXIT=0; 613 passed / 0 failed / 0 ignored of 613 (612 + 1 new) |
| Frontend | `cd apps/desktop && npm ci && npm run typecheck && npm test` | not run before the fix (no `node_modules` in the worktree) | EXIT=0; typecheck clean; 125 of 125 test files passed |
| Root tests (re-run because the fix touches the root crate) | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | 6,306 passed / 0 failed / 28 ignored (suite-root row above) | first run with the fix: 287 test binaries; 6,307 passed / 1 failed / 28 ignored (the sd14 row below); after the sd14 update, `--test sd14_saved_character_envelope` 13 of 13 passed, so 6,308 passed / 0 failed / 28 ignored (6,306 + 2 new lib tests) |
| Root clippy | `cargo clippy --locked -j 8 --all-targets -- -D warnings` | EXIT=0 | EXIT=0 |

| Test | Class | Receipt |
|---|---|---|
| `character_hub::tests::create_character_at_root_grants_no_wealth_when_the_build_is_blocked` (`apps/desktop/src-tauri/src/character_hub.rs`) | B: a real defect the new package exposes, fixed at the root | Error: `selected choice choice_set_id 'advanced_players_guide:class_feature:summoner' must have exactly two colon-segments to round-trip through the fixture grammar`. F1c records two Path-A picks under the converted rule's own id, `book:kind:slug` (3 colon-segments). The first is the Summoner Class Selection (`class_seeds::SUMMONER_CLASS_SELECTION_CHOICE_ID`, 641691e283). The second is the Commoner's one Simple weapon (`COMMONER_WEAPON_CHOICE_ID`, e61473e9c9). `pf1_adapter::compose_character_input` pushes both into `selected_choices`. The saved-character store's `choice=<set>:<selection>` line re-splits after the second colon, so `validate_character_input` refused the save. A Human Summoner or Commoner was never created from the desktop: the create call errored before anything was written. **Root fix (one rule, no class named):** a choice set id with exactly two colon-segments keeps the `choice=` line, and any other set id goes on a new `rule_choice=<set>\|<selection>` line (`src/saved_character/local_store.rs` writer and validation; loader `apply_rule_choice` in `src/rules_core/character_input.rs`). An id that contains `\|` or is empty is refused at save. **RED first:** `saved_character::local_store::tests::save_and_load_round_trips_a_choice_recorded_under_a_converted_rule_id` failed with the same message before the fix and passes after it. Also added: `a_rule_choice_id_containing_the_separator_is_refused` and the desktop test `create_character_at_root_persists_a_pick_recorded_under_a_converted_rule_id`, which creates Summoner and Commoner through the real create path; both save and reload their pick. No sheet value changed: the fix changes only how a pick is persisted. |
| `sd14_save_rejects_selected_choice_that_cannot_round_trip` (`tests/sd14_saved_character_envelope.rs`) | Consequence of the B fix (the test's premise changed by design) | Its first case assumed that a 3-segment choice set (`choice:fighter:bonus_feat`) cannot round-trip, so save must refuse it. After the fix it round-trips on `rule_choice=`. The case now asserts it reloads as the SAME choice. The test's invariant (never reload as a different choice) is unchanged. The refusal branch is still covered by an id carrying the `\|` separator (`choice:fighter:bonus\|feat`, error names `choice_set_id`). The second case (`selection_id` with too few segments on a `choice=` line) is unchanged. No sheet value is involved. |

Other checks:
- **Desktop clippy (not a gate of this step).** `cargo clippy --locked -j 8 --all-targets --manifest-path apps/desktop/src-tauri/Cargo.toml -- -D warnings` fails with 67 errors, and all 67 are `dead_code`/`unused_import` in the non-test bin target. By file: `spell_catalog.rs` 52, `corpus_full.rs` 4, `character_hub.rs` 3 (`SavedCharacterMutationOp*`, lines 1314-1349), `rule_system_adapter.rs` 2, and 1 each in `recomputeCharacter.rs`, `class_catalog_generic.rs`, `corpus_ingest_diagnostic.rs`, `equipment_catalog.rs`, `feat_catalog.rs` and `reference_library_catalog.rs`. **C (unrelated, pre-existing):** none of the flagged items is in F1c's desktop diff. `git diff --stat 8057263014..HEAD -- apps/desktop/src-tauri/src` names `character_hub.rs` (tests), `class_feature_feat_bridge.rs`, `class_spell_levels.rs` and `pf1_adapter.rs`, and this step adds only a test.
- **Ingest crate.** It uses `load_character_input_fixture` in 10 test files plus `oracle_validation/selected_parity_dimensions.rs`, and was not re-run. The loader change only adds a match arm for a key that no ingest fixture or package file contains (`grep -rn rule_choice crates data/sheet_rules`: 0 hits). Every existing input parses exactly as before.

Classification A: 0 re-baselined. Classification B: 1 defect fixed (converted-rule-id picks could not be saved). Test premise updated by the fix: 1 (sd14). Classification C: 1 (desktop clippy dead code, not a gate here). Changed sheet values without a PF1 citation: none. STOPs: none.

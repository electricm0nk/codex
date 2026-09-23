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

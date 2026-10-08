# Cycle E4a.3 — E4a / `.lst` citation burn-down

- **Card ID:** E4a.3   **Model:** sonnet   **RETRO_ACTOR:** sd37-e4a-3
- **Commit SHA:** the commit that adds this file (`git log -1 --format=%H -- docs/release/SD-37-starfinder-1e/artifacts/epic_4a/E4a.3_cycle_receipt.md`)   **Base SHA:** `615ce41338e4cc2fa0705ea98b5f58ff15239629` (tranche/17 at start; `git rev-parse HEAD` after the §5 rebase said "up to date")   **Oracle SHA:** 7f818006e3…
- **Tree:** `/home/ubuntu/workspace/worktrees/codex-sd37` (tranche/17), `CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37`.
- **Acceptance criterion (verbatim from epic-breakdown.md):** Receipt shows the D6 status; `test ! -e src/rules_core/rules_tables || grep -rcF '.lst' src/rules_core/rules_tables | awk -F: '{s+=$2} END{print s+0}'` → 0 or no output (12,529 before, CUI F-11) and the same predicate over every bundled data-package file → 0, each cross-checked in Python. Out of D6 scope and only reported: the 1,098 `.lst` lines elsewhere in `src/rules_core` … residue gate PASS.

## D6 status, re-derived first

| Half | Command | Result at base |
|---|---|---|
| Identifier half (`lst_file` in the residue gate) | `grep -n 'lst_file' scripts/pcgen_residue_gate.py` | lines 223 and 229 present (`"lst_file": r"\blst_file\b"`); `python3 scripts/pcgen_residue_gate.py --check --closure` exit 0 before and after |
| Burn-down half (citations in tables) | `grep -rcF '.lst' src/rules_core/rules_tables \| awk -F: '{s+=$2} END{print s+0}'` | **12,529** compiled lines, and **6,164** lines in the package (`grep -rcF '.lst' data/rules_tables \| awk …`), 85 package files |

The burn-down half was open and is closed here. SD-36's "8,592" is a retired figure (predicate never stated, `content-unit-inventory.md` F-11).

## RED → GREEN

- **RED (pre-change):** the new suite `tests/rules_tables_citation_burn_down.rs` on the pre-change tree: `cargo test --locked -j 8 --test rules_tables_citation_burn_down` → 1 passed (the planted-line control), **2 failed** with `left: 12529 right: 0` (compiled tables) and `left: 6164 right: 0` (package) — the intended reason, and the two numbers equal the acceptance command's `E4a.3_logs/red.log`.
- **GREEN:** same command after the change → `3 passed; 0 failed` (`E4a.3_logs/root-targeted-rerun.log`), and the acceptance command, verbatim, prints **0** (`E4a.3_logs/acceptance.log`).
- **Mutation that must fail:** one package row edited to carry the literal (`"Fighter"` → `"Fighter.lst"` in `data/rules_tables/crb/class_tables/class_tables.json`) → the package test fails with `left: 1`; file restored, `sha256sum -c` OK (`E4a.3_logs/planted-mutation-package.log`). A second control inside the suite plants a line in a scratch tree and asserts the scan counts exactly 1.

## What shipped

**Decision (safe default, logged in `progress.md`):** the citation keeps its book stem and its line and loses only the list-file extension (`b1_races.lst` → `b1_races`). Nothing is lost: the extension is a constant, and it is spelled in exactly one place, `crates/codex-ingest/src/pcgen_import/cache_gen/mod.rs` (`cited_stem`, `cited_file`, `cited_coordinate`), so the generators that join a citation back to a corpus file still do. All 90 distinct stems in the package resolve, with `.lst` appended, to a file in the pinned oracle checkout (`comm -23` of the package stems against the oracle's basenames → 0 missing; checked by basename, not by directory).

| Part | What changed | Command → figure |
|---|---|---|
| Compiled tables | extension removed from 12,474 citations in 236 files by `E4a.3_strip_citations.py` (regex: an extension directly after a name character), then 72 files of prose that named the extension itself (`` `.lst` `` → "source file"), plus 2 hand-edited `ends_with(".lst")` checks (now "is a stem: not empty, no `.`") | `grep -rcF '.lst' src/rules_core/rules_tables \| awk -F: '{s+=$2} END{print s+0}'` 12529 → **0**; Python `os.walk` → 0 |
| Package | regenerated from the compiled tables by `cargo run --locked -j 8 --bin rules_tables_package -- --write`; still 281 tables, bytes 13,316,285 → 13,291,625 | `--check` → `tables=281 rows_bytes=13291625 pi_files=9 verdict=PASS` (`E4a.3_logs/package-check.log`); `grep -rcF '.lst' data/rules_tables \| awk …` 6164 → **0**, 85 files changed, `+6164 −6164` |
| Package schema | `schemas/rules/rules_tables.schema.json` regenerated (its descriptions are the row types' doc comments): 41 lines changed | `RULES_SCHEMA_OUT=<dir> cargo test --lib sheet_rule::schema_publish_tests::…`; `grep -cF '.lst' schemas/rules/rules_tables.schema.json` → 0 |
| Tests that named the literal | 3 edited: `simple_kind_tables.rs` (reads the coordinate back from the table's index instead of spelling the corpus's own string), `tests/v06_beastiary1_natural_attack_grounding.rs` (path of the row is now a stem), `crates/codex-ingest/tests/rules_core_feats_all_via_converter.rs` (`uca_feats:`) | each was RED in the first sweep and is green in the rerun |
| Generators that join a citation to a corpus file | `gen_book_cache.rs` keys its file maps by stem, loads by `cited_file`, writes the corpus coordinate through `cited_coordinate`; `cache_gen/equipment_gap.rs` resolves the cited file through `cited_file`; `gen_equipment_gap_tables.rs` writes the stem; both `scripts/transcribe_*_tables.py` strip the extension at write time | compiled, clippy 0, ingest sweep green; **not run end to end** (see "Does not cover") |
| Control | `tests/rules_tables_citation_burn_down.rs`: compiled tables (skipped when the directory is gone, E4a.4), package, package schema, planted-line control | 3 passed |

**Files touched (explicit paths):** 237 files under `src/rules_core/rules_tables/` (`git diff --numstat -- src/rules_core/rules_tables` → 237 files, +12542 −12529), 85 files under `data/rules_tables/`, `schemas/rules/rules_tables.schema.json`, `tests/rules_tables_citation_burn_down.rs` (new), `tests/v06_beastiary1_natural_attack_grounding.rs`, `crates/codex-ingest/src/pcgen_import/cache_gen/{mod,equipment_gap}.rs`, `crates/codex-ingest/src/bin/{gen_book_cache,gen_equipment_gap_tables}.rs`, `crates/codex-ingest/tests/rules_core_feats_all_via_converter.rs`, `scripts/transcribe_{monster,companion}_tables.py`, `docs/release/SD-37-starfinder-1e/{kanban,progress}.md`, this receipt and `artifacts/epic_4a/E4a.3_*`, `docs/retro/events/sd37-e4a-3.jsonl`. Files owned by completed epics (decisions.md §21): the E4a.1/E4a.2 package, schema and catalog tests (this epic), and E3's `crates/codex-ingest` generators; the E3 gate (structural diff) is for a converter or mapping change and this is neither, but `sheet_rule_convert --check` and the residue gate were re-run anyway (below).

## Figures

- **Burn-down:** compiled 12,529 → 0, package 6,164 → 0, schema 41 → 0 — predicate "lines containing the literal `.lst`" — `E4a.3_logs/acceptance.log` (before from `git grep -cF '.lst' HEAD -- <path>`; after from `grep -rcF` and from Python `os.walk`, equal).
- **Reported, not burned (out of D6 scope):** `grep -rcF '.lst' src/rules_core --exclude-dir=rules_tables | awk -F: '{s+=$2} END{print s+0}'` → **1117** (decisions.md says 1098 on 2026-10-02; 1110 at `5cbee570e1~1`, 7 of the rest in `src/rules_core/rules_catalog`; retro correction emitted). Other bundled data holds the literal too: `data/sheet_rules` 53,020 lines, `data/starfinder-1e/sheet_rules` 9,094 lines — corpus and sheet-rule provenance, not table citations.
- **Package:** 281 tables, 24,583 rows, unchanged by this card (`rules_tables_package --check`, above).

## Build scope verified

| Scope | Command | Result | Log |
|---|---|---|---|
| root full | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | 296 suites run, **6,522 passed, 0 failed** (awk over `test result:` lines; 297 `ok` result lines, doc-tests included) | `E4a.3_logs/root-full.log` |
| root first pass (before the 3 test fixes) | same | 3 failed, each attributed to its `Running` line: `unittests src/lib.rs` (2: `simple_kind_tables::…domain_table_resolves_a_pi_renamed_record_by_coordinate…`, `sheet_rule::schema_publish_tests::published_schemas_match_the_serde_types`), `tests/v06_beastiary1_natural_attack_grounding.rs` (1) | `E4a.3_logs/root-full-first-pass.log` |
| codex-ingest | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` (crates/codex-ingest) | 174 suites, 1,824 passed in the sweep + the one failing suite (`rules_core_feats_all_via_converter`, `uca_feats.lst:` literal) rerun alone 4 passed 0 failed = **1,828 passed, 0 failed** | `E4a.3_logs/ingest-full.log`, `ingest-feats-all-rerun.log` |
| apps/desktop/src-tauri | `PF_SEED_RENDER_OUT=… cargo test --locked -j 8 -- --test-threads=8 --nocapture` | **699 passed, 0 failed, 2 ignored** | `E4a.3_logs/desktop-full.log` |
| verify stages | `bash scripts/verify.sh -j 8 --only tauri-resources-tracked --only rules-schema-check --only pi-sweep --only crate-wall --only clippy` | PASS ×5: resources 9 checked; 3 schemas regenerate byte-equal; pi-sweep 22 hits (11 Rust + 11 package), 21 baseline rows, 0 stamps disagree; crate wall clean; clippy root 0 / desktop 0 / ingest 0 | `E4a.3_logs/verify-stages.log` |
| converter / residue | `cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check`; `python3 scripts/pcgen_residue_gate.py --check --closure` | exit 0; exit 0 | `E4a.3_logs/python-residue-convert.log` |
| frontend | not touched | — | — |

The first ingest sweep was started before the generators were fixed and stopped at suite 51 of 174 (rework event); the sweep cited above is the second, on the tree that was committed. The root sweep was run twice for the same reason (3 test fixes in between); only the second counts.

## PF hash pair (decisions.md §21: `src/`, `crates/` and desktop engine files are edited)

`PF_SEED_RENDER_OUT=<dir> cargo test --locked -j 8 -- --test-threads=8 --nocapture` (apps/desktop/src-tauri, inside the desktop sweep): Aldric `1d830682210a0e6e216093a3125593edf1f25539f38547fc832a56c050a5a569`, Elowen `8d1a711c8519b62b489f47f097d683922568a645f4daabc9c95933c1b2b100f2` — `diff` of `E4a.3_logs/pf-render/sha256.txt` against `E4a.2_logs/pf-render/sha256.txt` empty, and `cmp` of both JSON files byte-equal. The render reads the package (E4a.2's planted row moved Elowen's hash), so this is a render over the rewritten package.

## Audits

Run after the local commit (§6 step 4), scoped to `src tests crates apps data schemas scripts` (`E4a.3_logs/audit.log`): see the Status section's final line for the printed `OK_*` tokens.

## Seed deltas

Aldric (Fighter 3): unchanged (`1d830682…a569` = E1.4 = E4a.2). Elowen (Wizard 5 + Fireball): unchanged (`8d1a711c…00f2` = E1.4 = E4a.2). SF-Soldier-3 / SF-Mystic-5 / SF-Technomancer-5 / SF-Envoy-3: unchanged — printed lines `cmp`-identical to E6.MC's after-lines (51 / 72 / 63 / 42 lines; `SF_SEED_LINES_OUT=<dir> cargo test --locked -j 8 --bin codex-desktop -- sf_ starfinder` → `56 passed; 0 failed; 1 ignored`, `E4a.3_logs/sf-seeds.log`, `E4a.3_logs/sf-seed-lines/`); no Starfinder file reads the rules tables.

## Does not cover

- **The literal only.** The scan matches the extension. A citation is still a book stem plus a line (90 distinct stems, 6,025 citation rows), and a stem names a PCGen file. Removing the stems would remove the join key the generators check each row against. If the operator reads "burn down" as "no source-file name at all", that is a larger change (drop the field, re-point every generator) and a ruling.
- **The extension still exists in tooling.** It is spelled once in `codex-ingest` (`cited_file`), which is compiled into the ingest tools behind the crate wall and is neither shipped nor bundled; `decisions.md §19` asked for provenance "in a file that is neither compiled nor bundled", and this does not literally meet "not compiled". Alternative not taken: a sidecar file mapping each stem to its extension, which would carry the same constant 90 times.
- **"Every bundled data-package file" is read as `data/rules_tables/` (plus its schema).** `tauri.conf.json` also bundles `data/sheet_rules/` (53,020 lines with the literal), `data/starfinder-1e/sheet_rules/` (9,094) and the corpus; those are corpus and sheet-rule provenance, not `rules_tables` citations, and burning them is a converter change this card does not own. `decisions.md §19`'s parenthesis ("every path the `tauri.conf.json` resources bundle") can be read the other way; that reading would reopen the card.
- **The 1,117 lines elsewhere in `src/rules_core`** are reported only, as the criterion says. `scripts/pcgen_residue_gate.py` line 227 still carries a comment that the table citations "burn down" later; the gate has no `.lst` pattern and was left alone.
- **Generators were compiled, linted and unit-tested, not re-run against the corpus.** `gen_book_cache`, `gen_equipment_gap_tables` and the two transcribers were not executed end to end; a regenerated corpus record or table is the proof that would close that, and E4a.4 deletes the tables they read. The transcribers were checked by `python3 -m unittest` from the repo root: 88 tests, 1 failure, `test_an_ability_no_bundle_names_stays_an_orphan_and_is_not_shipped`, which fails identically on `git archive HEAD scripts` (40 tests, 1 failure; test file unchanged) — pre-existing, not caused here, not fixed (out of scope).
- **Tests rewritten to stems prove the stems, not the corpus.** The three edited tests assert the new spelling; the corpus spelling is exercised by `simple_kind_tables`' coordinate lookup (which now reads the corpus's own string) and by the ingest sweep.
- **Provenance reconstruction was checked by basename** against the oracle checkout (90 of 90), not by directory, so two files with one basename in different books would not be told apart by that check.
- **The CI and packaged build** were not run; the first tester publish after the operator merges is the first packaged build to read the rewritten package.

## Status

**complete** — acceptance command prints 0 on the pushed tree (compiled), 0 over the package and its schema; PF pair equal; root / ingest / desktop green; 5 verify stages PASS.

- **Safe defaults taken:** SD-b-shaped scope call recorded in `progress.md`: strip the extension, keep stem and line (alternatives not taken: remove the citation fields; move citations to a sidecar). `decisions.md §21` used for `crates/codex-ingest` and the E4a.1/E4a.2 package and schema.
- **Retro events emitted:** `docs/retro/events/sd37-e4a-3.jsonl`: correction ×1 (the 1098 figure → 1117), rework ×1 (the early ingest sweep).
- **Next-cycle plan:** E4a.4 (PF parity + Rust table removal) now deletes `src/rules_core/rules_tables/**/*.rs`; `tests/rules_tables_citation_burn_down.rs` already skips its compiled-tables check when the directory is gone, and keeps checking the package. E4a.4 must also carry the generators that read compiled tables (`gen_book_cache`, `gen_equipment_gap_tables`, the two transcribers), which the E4a.2 path-string note already lists.

# Cycle E4a.4a — E4a / Six table generators write the data package, not compiled source

- **Card ID:** E4a.4a   **Model:** opus   **RETRO_ACTOR:** sd37-e4a-4a
- **Commit SHA:** the `feat(sd37,e4a.4a)` commit (`git log -1 --format=%H -- docs/release/SD-37-starfinder-1e/artifacts/epic_4a/E4a.4a_cycle_receipt.md`)    **Base SHA:** `118a07c7c0` (wrong-base control `BASE_OK`; `git fetch origin tranche/17 && git rebase origin/tranche/17` → "Current branch tranche/17 is up to date.")   **Oracle SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`git -C $PCGEN_REPO_DIR rev-parse HEAD`; `scripts/fetch-pcgen-oracle.sh --check` ok)
- **Files touched:**
  - New: `crates/codex-ingest/src/rules_package_out.rs` (+ `pub mod` in `lib.rs`), `scripts/rules_tables_package_out.py`, `crates/codex-ingest/tests/table_generators_write_the_data_package.rs`, `scripts/tests/test_transcribers_write_the_data_package.py`.
  - Ported: `crates/codex-ingest/src/bin/{gen_feat_gap_tables,gen_equipment_gap_tables,ingest_class_spell_levels_arg,ingest_spells}.rs`, `scripts/transcribe_{monster,companion}_tables.py`; test rewrite `scripts/tests/test_transcribe_monster_tables.py`.
  - Not changed: anything under `data/`, `src/`, `apps/` (the proof is that the package did not move).
  - Owning epics (§21): E4a (this epic) for the generators; E3 (complete) for `crates/codex-ingest/**` — gate re-run: `sheet_rule_convert --check` exit 0, residue gate exit 0, structural diff `verdict=PASS` (records 49,450 → 49,450).
- **Acceptance criterion (verbatim from epic-breakdown.md):** The six generators that wrote compiled `.rs` into the removed `src/rules_core/rules_tables` now write `data/rules_tables/<id>.json`. A re-run of each against the pinned oracle leaves the package byte-identical. Command: `grep -rnE '"src/rules_core/rules_tables/|f"src/rules_core/rules_tables/' crates/codex-ingest/src/bin scripts --include='*.rs' --include='*.py' | awk -F: '{print $1}' | sort -u | awk '!/scripts\/tests\//' | awk 'END{print NR}'` → 0 (was 6); each generator re-run → `git status --porcelain data/rules_tables` empty; golden digest test green; PF hash pair equal to E1.4. Does not cover: generators outside the six.

## What shipped

| Generator | Now writes | How |
|---|---|---|
| `gen_feat_gap_tables` | `feat_gap_tables/<SLUG>_FEAT_GAP_ROWS` × 19 | typed `FeatCatalogRecord` rows → `rules_package_out::write_table` (= `rules_data_package::render_table`, the package's own canonical form + stamp). PI hard stop kept: every table screened (`screen_table`) before any file is written. The converter-side `feat_gap_prereq_tokens.rs` path was also broken (`src/pcgen_import/…`, relative to the repo root since the SD-36 crate split) and its header had drifted from the shipped file; both fixed, file byte-identical on re-run. |
| `gen_equipment_gap_tables` | `equipment_gap_tables/<SLUG>_GAP_ROWS` × 28 | typed `EquipmentGapRow` rows; same screen-all-then-write. |
| `ingest_class_spell_levels_arg --emit` | `advanced_race_guide/class_spell_levels/ARG_CLASS_SPELL_LEVELS` | `(class id, [(key, level)])` rows. `--verify` still reproduces all 12 shipped CRB/APG/ACG per-class tables. |
| `ingest_spells` | `<book>/spell_list/SPELL_LIST` × 21 | a serde mirror of the per-book `SpellListEntry` (`--check` loads each file back as the book's own type). Two source fixes, below. |
| `transcribe_monster_tables.py` | `<book>/monster_data/{MONSTERS,MONSTER_ABILITIES}` × 22 books | rows built as dicts, `.lst` stripped from every string (E4a.3 rule, now applied to rows), written per file atomically, then `rules_tables_package --write` normalises and re-stamps from the Rust screen; `check_written` fails if the normaliser removed a file no table owns. Provenance header → printed notes. One source fix, below. |
| `transcribe_companion_tables.py` | `<module dir>/companion_data/{COMPANIONS,COMPANION_ABILITIES[,COMPANION_CLASSES]}` × 16 books | same path; `COMPANION_CLASSES` only for a book with class rows (the 3 the package registers). Guard typing, below. |

**Source fixes found by the re-run (each was a post-hoc hand edit to generated `.rs` that the generator never learned; a re-run would have reverted it).** Each is now done in the generator, with the commit that did it by hand cited in code:

1. `ingest_spells`: SD-32 `c27375ee1d` added `.COPY=` variant ingest for every book but regenerated only `bestiary` and `book_of_the_damned_volume_1`. First re-run (`E4a.4a_logs/rerun1-spell-drift.stat`): 8 files, +2,878/−1 lines. New per-book `copy_variants` flag (true for exactly those two books) reproduces the package; 411 variant rows in 7 books are not ingested and counted per book in the run log (see Does not cover).
2. `ingest_spells`: SD-34 `9d2e7d9e28` replaced U+00AD soft hyphens with `-` in `monster_codex` Spellsteal by hand (`clippy::invisible_characters`) — `soft_hyphens_as_hyphens` (`rerun1-monster-codex-soft-hyphen.diff`).
3. `transcribe_monster_tables.py`: SD-35 cycle 11 `208ebf1e21` (`%CHOICE`/`%LIST` → "the chosen option", `TYPE=Base` → `Base`, leaked ` DESC:` token name) — `in_the_rules_words`. First re-run drifted 3 files (bestiary, bestiary_3, inner_sea_world_guide; 23+/23−).
4. `transcribe_companion_tables.py`: SD-35 cycles 9 `f4583db504` and 13 `88b4490e16` (guard strings → typed `EffectCondition` in description variants, `CompanionAbilityGrant`, `NaturalAttackDamageBonus.conditions`, `ExternalAbilityRefCondition`) — `typed_guard` (refuses a token that does not rebuild verbatim), `ability_grant`, `natural_attack_damage_bonus`, `split_external_ref_guards`.

## RED (pre-change command + output)

- Acceptance on the base tree (`E4a.4a_logs/acceptance-red.log`): `6`; Python `os.walk` over the same roots → 6 (`gen_equipment_gap_tables.rs`, `gen_feat_gap_tables.rs`, `ingest_class_spell_levels_arg.rs`, `ingest_spells.rs`, `transcribe_companion_tables.py`, `transcribe_monster_tables.py`).
- New test before the writer existed (`E4a.4a_logs/tdd-red.log`): `error[E0432]: unresolved import codex_ingest::rules_package_out` — the intended reason. After the Rust ports and before the Python ports: `no_generator_writes_into_the_removed_compiled_tables_directory` FAILED (the two scripts), the other two passed.
- Byte-identity RED per mechanism: the first re-runs above (spell lists 8 files; monster 3 files).

## GREEN (same commands + output) — one pass, `E4a.4a_verify.sh`

- **Acceptance** (`acceptance.log`): `0`, exit 0.
- **Every generator re-run against the pinned oracle** (`proof_rerun.log`, exit 0): 42 invocations = 4 Rust bins + 22 monster books + 16 companion books; then `git status --porcelain data/rules_tables crates/codex-ingest/src/pcgen_import` → `status_lines=0` (`proof_status.log`).
- **Package check** (`package_check.log`): `rules_tables_package: tables=281 rows_bytes=13291625 pi_files=9 verdict=PASS` — the same figures E4a.4 recorded.
- **Plants** (`plants.log`): P1 a `name` edited in `feat_gap_tables/CORE_RULEBOOK_FEAT_GAP_ROWS.json` (sha `82d7f56b…` → `0c237b41…`, git shows `M`) → `gen_feat_gap_tables` restores `82d7f56b…`; P2 the same for `bestiary_2/monster_data/MONSTERS.json` (`fb73ed92…` → `60c94389…` → `fb73ed92…`) via the monster transcriber. The proof can fail.
- **Golden digests** (`golden.log`): `every_catalog_table_view_and_lookup_matches_its_golden_digest` ok.
- **PF hash pair** (`pf_seed_render.log`, `pf-render-sha256.txt`): Aldric `1d830682210a0e6e216093a3125593edf1f25539f38547fc832a56c050a5a569`, Elowen `8d1a711c8519b62b489f47f097d683922568a645f4daabc9c95933c1b2b100f2` = E1.4.
- **Python** (`python_tests.log`): 50 tests OK (`test_transcribe_monster_tables.py` 40 + new 10). Mutations (`python-test-mutations.log`): rules-words made identity → 1 failure; external-ref guards left unsplit → 1 failure.
- **Rust test** `table_generators_write_the_data_package`: 3 passed (in `verify_root.log`).

## Figures

- Generators writing into the removed directory: 6 → 0 (acceptance command above; Python agrees).
- `.COPY=` spell variants not ingested: 411 across 7 books — `awk '/not ingested/{s+=$NF} END{print s}' E4a.4a_logs/proof_rerun.log` → 411; a Python regex sum over the same log → 411. Per book: AG 7, HA 4, ISM 1, ISWG 2, OA 369, UM 19, UW 9. Of these, 388 (OA 369, UM 19) are `origin: mod_only` spell units in `docs/work-inventory.json`, 23 are not inventory units at all (Python over `units` keyed `(book, name)`; predicate: same book, kind `spell`).
- Python-suite baseline: on the base tree `discover -s scripts/tests` → 1,087 run, 12 failing (`python-suite-base-failures.txt`); with the ports before the test rewrite, 13 more, all in `test_transcribe_monster_tables.py` (`python-suite-pre-test-rewrite-failures.txt`); after the rewrite that file is 40/40 (it includes `test_an_ability_no_bundle_names_stays_an_orphan_and_is_not_shipped`, failing on the base tree too — it asserted the orphan's absence, contradicting `decisions.md §20`; now asserts it ships with no owner). The other 11 base failures are in unrelated scripts and unchanged.
- **Raw row-count output:** RED `6`, GREEN `0`.

## Build scope verified (once, widest)

| Scope | Command | Result | Log |
|---|---|---|---|
| root workspace (root + codex-ingest) | `cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8` | exit 0; 474 `test result` lines, 8,346 passed, 0 failed, 70 ignored (awk over the log; Python `re.findall` agrees: 474 / 8,346 / 0 / 70) | `verify_root.log` |
| desktop | PF hash pair only (`--bins pf_seed_render_hash`); no desktop file changed and the desktop does not depend on codex-ingest | 1 passed | `pf_seed_render.log` |
| clippy | `bash scripts/verify.sh --only clippy` | PASS root 0 / desktop 0 / ingest 0 | `clippy.log` |
| pi-sweep | `bash scripts/verify.sh --only pi-sweep` | PASS (12 hits, 11 baseline rows, 0 stamps disagree) | `pi_sweep.log` |

No `test result: FAILED` line in the root workspace run, so there is nothing to attribute.

- **Identifier audit:** OK_NO_BUNDLE_TAGS (scoped to `crates/codex-ingest` and `scripts`, added lines only)   **Wired-integration audit:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS — run on the committed diff `118a07c7c0...HEAD` after the commit (`E4a.4a_audit.sh ...HEAD`; the committed `E4a.4a_logs/audit.log` is the same script on the staged tree, identical output)
- **Structural diff (converter cycles):** `verdict=PASS`, records 49,450 → 49,450, removed `granted_by` 0, removed grants 0 (`pf_structural_diff.log`; `diff -rq` differs only by the dump's own `var_names.json`, which the tracked baseline does not carry). Planted mutation (one `_defects/` file removed from a copy of the dump) → `verdict=FAIL`, exit 1 (`structural_plant.log`): 1 of 1 plants failed. `sheet_rule_convert --check` exit 0 (`pf_check.log`), `pcgen_residue_gate.py --check --closure` exit 0 (`residue.log`). No converter code path changed (only the six generators, a new writer module and its `mod` line).
- **Seed deltas:** Aldric unchanged (`1d830682…a569`, rendered); Elowen unchanged (`8d1a711c…00f2`, rendered); SF-Soldier-3 / SF-Mystic-5 / SF-Technomancer-5 / SF-Envoy-3 not re-rendered — no engine, data or desktop file changed, and no SF file reads the rules tables (E4a.4/E4a.MC).
- **Does not cover:**
  - Generators outside the six (the card's own line), e.g. `gen_book_cache` (reads the catalog, writes `data/corpus`).
  - Whether the 411 `.COPY=` spell variants should ship. The generator reproduces the shipped package, and the census agrees for 388 of them (`mod_only`); the 23 bare `.COPY=` rows in AG/HA/ISM/ISWG/UW are in neither the package nor the inventory. Shipping any of them changes PF content (tables + golden digests) and the census predicate — outside SD-37. Retro deferral emitted.
  - The byte-identity proof covers the pinned oracle and today's `docs/work-inventory.json` (the transcribers' unit set comes from it); a regenerated inventory can move the transcribers' output legitimately.
  - The normaliser drops JSON fields no row type declares, so a transcriber field the Rust type lacks would vanish silently rather than fail; every field the types declare is required and was checked by the byte comparison.
  - E4a.MC findings: F1 (Fighter saves read from a second source) is engine-side, not generator output — not touched. F2 (the Crocodile `groundingNote` `.lst` strip) is E4a.3's mandated strip; the monster transcriber now applies that same strip to its rows, so a re-run keeps it.
- **Status:** complete
- **Safe defaults taken:** none from §12.1 applies. Process choices logged in `progress.md`: per-book `copy_variants` flag reproducing the package (alternative not taken: ship the 411 rows); hand edits folded into the generators as named functions (alternative: re-apply the old one-off SD-35 scripts after each run).
- **Retro events emitted:** `docs/retro/events/sd37-e4a-4a.jsonl`: correction ×3 (ingest_spells vs shipped spell lists; monster transcriber vs SD-35 cycle 11; companion transcriber vs SD-35 cycles 9/13), deferral ×1 (the 411 `.COPY=` rows).
- **Next-cycle plan:** E4a.MC re-run: the generator grep → 0, the six re-runs leave `data/rules_tables` unchanged (`E4a.4a_verify.sh` stages `proof_rerun`/`proof_status`), golden test, PF pair, one planted JSON row.
